"""Independent implementation of spec/definition-v1.md and spec/api-v1.md.

Standard library only. Written from the spec text, and for `exceptions.history`
from the accepted 0.2.0 design (ADR-0032), without reference to the fiscalclock
sources, so that its answers can serve as expected results.
Dates are handled as proleptic Gregorian ordinals (date.toordinal()), which
lets boundary arithmetic run past 0001-01-01 and 9999-12-31 without overflow.

A calendar is a list of regimes: the top-level `year` and `subdivision`, then one
per history entry. `Calendar.seq` lists every fiscal year in the supported range
in date order: each regime's years, with each entry's transition year between
the previous regime's last year and the entry's first. Every operation works on
positions in that list.
"""

from __future__ import annotations

import bisect
import calendar as _cal
import copy
import hashlib
import re
from dataclasses import dataclass
from datetime import date

WEEKDAYS = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"]
LEVELS = ["year", "quarter", "period", "week"]
CYCLE_DAYS = 146097
CYCLE_YEARS = 400
MIN_ORD = date(1, 1, 2).toordinal()
MAX_ORD = date(9999, 12, 30).toordinal()


class FiscalError(Exception):
    def __init__(self, code: str, path: str, details=None):
        super().__init__(f"{code} at {path!r}")
        self.code = code
        self.path = path
        self.details = details

    def expect_error(self) -> dict:
        out = {"code": self.code, "path": self.path}
        if self.details is not None:
            out["details"] = self.details
        return out


def weekday_of(o: int) -> int:
    return (o - 1) % 7


def _shift_into_range(y: int) -> tuple[int, int]:
    k = 0
    while y < 1:
        y += CYCLE_YEARS
        k -= 1
    while y > 9999:
        y -= CYCLE_YEARS
        k += 1
    return y, k


def days_in_month(y: int, m: int) -> int:
    y, _ = _shift_into_range(y)
    return _cal.monthrange(y, m)[1]


def ymd_ord(y: int, m: int, d: int) -> int:
    y2, k = _shift_into_range(y)
    return date(y2, m, d).toordinal() + k * CYCLE_DAYS


def civil_year(o: int) -> int:
    k = 0
    while o < 1:
        o += CYCLE_DAYS
        k -= 1
    while o > date.max.toordinal():
        o -= CYCLE_DAYS
        k += 1
    return date.fromordinal(o).year + k * CYCLE_YEARS


def iso(o: int) -> str:
    return date.fromordinal(o).isoformat()


def parse_iso(s, path: str = "date") -> int:
    if not isinstance(s, str) or not re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", s, re.ASCII):
        raise FiscalError("INVALID_DATE", path)
    try:
        return date(int(s[:4]), int(s[5:7]), int(s[8:])).toordinal()
    except ValueError:
        raise FiscalError("INVALID_DATE", path) from None


@dataclass(frozen=True)
class Span:
    level: str
    index: int | None
    start: int
    end: int
    weeks: int | None
    extra: bool


class Rule:
    """One year rule with its subdivision: the top-level `year` and `subdivision`, or those of one history entry."""

    def __init__(self, year: dict, sub: dict, week_start: int):
        self.kind = year["type"]
        self.quarters = list(sub["quarters"])
        if self.kind == "week_based":
            a = year["anchor"]
            self.edge = a["edge"]
            self.month = a["month"]
            self.day = a["day"]
            self.rule = a["rule"]
            self.qual = week_start if self.edge == "start" else (week_start - 1) % 7
            self.period_weeks = list(sub["periods"])
            self.extra_period = sub["extra_week"]["period"]
            assert sub["type"] == "week_pattern" and sum(self.period_weeks) == 52
        else:
            self.month = year["start"]["month"]
            self.day = year["start"]["day"]
            self.short_month = year.get("short_month")
            assert sub["type"] == "calendar_months" and sub["month_start_day"] == self.day
        assert sum(self.quarters) == self.n_periods()
        self._bounds: dict[int, tuple[int, int]] = {}

    def n_periods(self) -> int:
        return len(self.period_weeks) if self.kind == "week_based" else 12

    def _boundary(self, y: int) -> int:
        d = days_in_month(y, self.month) if self.day == "last" else self.day
        t = ymd_ord(y, self.month, d)
        before = t - (weekday_of(t) - self.qual) % 7
        if self.rule == "last":
            return before
        after = t + (self.qual - weekday_of(t)) % 7
        if self.rule == "first":
            return after
        return before if t - before <= 3 else after

    def adjusted(self, y: int, m: int, d: int) -> int:
        dim = days_in_month(y, m)
        if d <= dim:
            return ymd_ord(y, m, d)
        if self.short_month == "clamp":
            return ymd_ord(y, m, dim)
        assert self.short_month == "next"
        return ymd_ord(y, m, dim) + 1

    def bounds(self, k: int) -> tuple[int, int]:
        """First day and end_exclusive of the year with key k: its start year (fixed_date) or anchor year (week_based)."""
        if k not in self._bounds:
            if self.kind == "fixed_date":
                b = self.adjusted(k, self.month, self.day), self.adjusted(k + 1, self.month, self.day)
            elif self.edge == "end":
                b = self._boundary(k - 1) + 1, self._boundary(k) + 1
            else:
                b = self._boundary(k), self._boundary(k + 1)
            self._bounds[k] = b
        return self._bounds[k]

    def key_of(self, o: int) -> int:
        y = civil_year(o)
        for k in (y - 1, y, y + 1, y - 2, y + 2):
            s, e = self.bounds(k)
            if s <= o < e:
                return k
        raise AssertionError("no fiscal year contains ordinal %d" % o)

    def starts_year(self, o: int) -> bool:
        return self.bounds(self.key_of(o))[0] == o


@dataclass(frozen=True)
class Change:
    """One exceptions.history entry: the new regime starts on `frm`, after an optional transition year from `start`."""

    frm: int
    start: int | None
    sub: dict | None


@dataclass(frozen=True)
class FiscalYear:
    """One fiscal year of the timeline: a year of regime `regime` (its rule's key `key`), or a transition (regime None)."""

    start: int
    end: int
    regime: int | None
    key: int | None
    transition: dict | None


class Calendar:
    def __init__(self, definition: dict):
        self.definition = definition
        self.week_start = WEEKDAYS.index(definition["week"]["starts_on"])
        # Regime 0 is the top-level year and subdivision; history entry i is regime i + 1.
        self.rules = [Rule(definition["year"], definition["subdivision"], self.week_start)]
        self.history: list[Change] = []
        exceptions = definition.get("exceptions")
        if exceptions is not None:
            for entry in exceptions["history"]:
                self.rules.append(Rule(entry["year"], entry["subdivision"], self.week_start))
                t = entry.get("transition")
                self.history.append(Change(parse_iso(entry["from"]), None if t is None else parse_iso(t["start"]), None if t is None else t["subdivision"]))
        naming = definition["naming"]
        self.default_scheme = naming["default"]
        self.schemes = naming["schemes"]
        self._seq: list[FiscalYear] | None = None
        self._starts: list[int] | None = None
        self._numbers: dict[str, dict[int, list[int]]] = {}
        self._spans: dict[tuple[int, str], list[Span]] = {}

    # Validation that needs dates ------------------------------------------

    def change_errors(self) -> list[FiscalError]:
        """The cross-block checks on exceptions.history, which the spec runs once every block is otherwise valid."""
        errors = []
        for i, ch in enumerate(self.history):
            base = f"exceptions.history[{i}]"
            prev, own = self.rules[i], self.rules[i + 1]
            c, c_path = (ch.frm, base + ".from") if ch.start is None else (ch.start, base + ".transition.start")
            if i >= 1 and c <= self.history[i - 1].frm:
                errors.append(FiscalError("EXCEPTIONS_OVERLAP", c_path))
            if not prev.starts_year(c):
                errors.append(FiscalError("CHANGE_NOT_ON_BOUNDARY", c_path))
            if not own.starts_year(ch.frm):
                errors.append(FiscalError("CHANGE_NOT_ON_BOUNDARY", base + ".from"))
            if ch.sub is None:
                continue
            sub_path = base + ".transition.subdivision"
            first, after = date.fromordinal(ch.start), date.fromordinal(ch.frm)
            if ch.sub["type"] == "calendar_months":
                if ch.sub["month_start_day"] != first.day or ch.sub["month_start_day"] != after.day:
                    errors.append(FiscalError("SUBDIVISION_YEAR_MISMATCH", sub_path + ".month_start_day"))
                elif sum(ch.sub["quarters"]) != (after.year - first.year) * 12 + after.month - first.month:
                    errors.append(FiscalError("QUARTERS_SUM_MISMATCH", sub_path + ".quarters"))
            else:
                assert ch.sub["type"] == "week_pattern"
                whole_weeks = (ch.frm - ch.start) % 7 == 0
                if weekday_of(ch.start) != self.week_start or not whole_weeks:
                    errors.append(FiscalError("SUBDIVISION_YEAR_MISMATCH", sub_path + ".type"))
                if whole_weeks and sum(ch.sub["periods"]) != (ch.frm - ch.start) // 7:
                    errors.append(FiscalError("WEEKS_SUM_MISMATCH", sub_path + ".periods"))
        # A stable sort: two CHANGE_NOT_ON_BOUNDARY errors at one `from` keep the previous regime's first.
        return sorted(errors, key=lambda err: (err.path, err.code))

    def validation_errors(self) -> list[FiscalError]:
        """Cross-block errors if there are any, otherwise the year-number checks (which run only after them)."""
        return self.change_errors() or self.naming_errors()

    # The sequence of fiscal years ------------------------------------------

    @property
    def seq(self) -> list[FiscalYear]:
        """Every fiscal year that lies wholly in the supported range, in date order."""
        if self._seq is None:
            assert not self.change_errors(), self.change_errors()
            years: list[FiscalYear] = []
            lower = None
            for r, rule in enumerate(self.rules):
                ch = self.history[r] if r < len(self.history) else None
                upper = None if ch is None else (ch.frm if ch.start is None else ch.start)
                a = MIN_ORD if lower is None else max(lower, MIN_ORD)
                b = MAX_ORD if upper is None else min(upper - 1, MAX_ORD)
                if a <= b:
                    for k in range(rule.key_of(a), rule.key_of(b) + 1):
                        s, e = rule.bounds(k)
                        if (lower is None or s >= lower) and (upper is None or e <= upper):
                            years.append(FiscalYear(s, e, r, k, None))
                if ch is not None:
                    if ch.start is not None:
                        years.append(FiscalYear(ch.start, ch.frm, None, None, ch.sub))
                    lower = ch.frm
            self._seq = [y for y in years if y.start >= MIN_ORD and y.end - 1 <= MAX_ORD]
            assert all(a.end == b.start for a, b in zip(self._seq, self._seq[1:])), "fiscal years must tile"
        return self._seq

    def key_bounds(self, k: int) -> tuple[int, int]:
        fy = self.seq[k]
        return fy.start, fy.end

    def in_range(self, k: int) -> bool:
        return 0 <= k < len(self.seq)

    def last_key(self) -> int:
        return len(self.seq) - 1

    def find(self, o: int) -> int | None:
        """The key of the supported fiscal year that contains ordinal o, or None."""
        if self._starts is None:
            self._starts = [fy.start for fy in self.seq]
        k = bisect.bisect_right(self._starts, o) - 1
        if k < 0 or o >= self.seq[k].end:
            return None
        return k

    def week_pattern(self, k: int) -> bool:
        """Whether fiscal year k is divided into whole weeks (a week_based regime year or a week_pattern transition)."""
        fy = self.seq[k]
        if fy.regime is None:
            return fy.transition["type"] == "week_pattern"
        return self.rules[fy.regime].kind == "week_based"

    # Year numbers -------------------------------------------------------

    def scheme_number(self, k: int, scheme: str) -> int | None:
        """The year number of fiscal year k, or None under "majority" when no calendar year holds more than half its days."""
        sc = self.schemes[scheme]
        s, e = self.key_bounds(k)
        rule = sc["year_number"]
        if rule == "start":
            y = civil_year(s)
        elif rule == "end":
            y = civil_year(e - 1)
        else:
            assert rule == "majority"
            winners = []
            for cy in range(civil_year(s), civil_year(e - 1) + 1):
                days = min(e, ymd_ord(cy + 1, 1, 1)) - max(s, ymd_ord(cy, 1, 1))
                if 2 * days > e - s:
                    winners.append(cy)
            if not winners:
                return None
            (y,) = winners
        return y + sc.get("offset", 0)

    def number(self, k: int) -> int:
        return self.scheme_number(k, self.default_scheme)

    def key_for_scheme_number(self, n: int, scheme: str) -> int | None:
        if scheme not in self._numbers:
            table: dict[int, list[int]] = {}
            for k in range(len(self.seq)):
                table.setdefault(self.scheme_number(k, scheme), []).append(k)
            self._numbers[scheme] = table
        keys = self._numbers[scheme].get(n)
        if keys is None:
            return None
        assert len(keys) == 1, "year number %d names two fiscal years" % n
        return keys[0]

    def naming_errors(self) -> list[FiscalError]:
        """MAJORITY_TIE or YEAR_NUMBER_COLLISION per scheme, over every fiscal year in the supported range.

        Within a run of years made by one regime's rule the numbers must be consecutive. Across a change
        (a regime's last year, the transition if any, the next regime's first year) each number must be
        greater than the one before, so a number may be skipped there and nowhere else.
        """
        errors = []
        for name in sorted(self.schemes):
            path = f"naming.schemes.{name}.year_number"
            numbers = [self.scheme_number(k, name) for k in range(len(self.seq))]
            if None in numbers:
                errors.append(FiscalError("MAJORITY_TIE", path))
                continue
            for k in range(len(self.seq) - 1):
                a, b = numbers[k], numbers[k + 1]
                same_run = self.seq[k].regime is not None and self.seq[k].regime == self.seq[k + 1].regime
                if (b != a + 1) if same_run else (b <= a):
                    errors.append(FiscalError("YEAR_NUMBER_COLLISION", path))
                    break
        return sorted(errors, key=lambda err: (err.path, err.code))

    # Structure ---------------------------------------------------------

    def spans(self, k: int, level: str) -> list[Span]:
        if (k, level) not in self._spans:
            self._spans[(k, level)] = self._make_spans(k, level)
        return self._spans[(k, level)]

    def _make_spans(self, k: int, level: str) -> list[Span]:
        fy = self.seq[k]
        s, e = fy.start, fy.end
        pattern = self.week_pattern(k)
        if level == "year":
            long_year = fy.regime is not None and self.rules[fy.regime].kind == "week_based" and (e - s) // 7 == 53
            return [Span("year", None, s, e, (e - s) // 7 if pattern else None, long_year)]
        if level == "week":
            return self._week_spans(k)
        periods = self._period_spans(k)
        if level == "period":
            return periods
        quarters = self.rules[fy.regime].quarters if fy.regime is not None else fy.transition["quarters"]
        out, i = [], 0
        for q, n in enumerate(quarters, start=1):
            group = periods[i : i + n]
            i += n
            weeks = sum(p.weeks for p in group) if pattern else None
            out.append(Span("quarter", q, group[0].start, group[-1].end, weeks, any(p.extra for p in group)))
        assert i == len(periods)
        return out

    def _period_spans(self, k: int) -> list[Span]:
        fy = self.seq[k]
        s, e = fy.start, fy.end
        if fy.regime is None:
            t = fy.transition
            if t["type"] == "week_pattern":
                out, cur = [], s
                for i, w in enumerate(t["periods"], start=1):
                    out.append(Span("period", i, cur, cur + 7 * w, w, False))
                    cur += 7 * w
                assert cur == e
                return out
            first, after = date.fromordinal(s), date.fromordinal(e)
            months = (after.year - first.year) * 12 + after.month - first.month
            starts = []
            for i in range(months):
                m0 = first.month - 1 + i
                starts.append(ymd_ord(first.year + m0 // 12, m0 % 12 + 1, t["month_start_day"]))
            starts.append(e)
            assert starts[0] == s
            return [Span("period", i + 1, starts[i], starts[i + 1], None, False) for i in range(months)]
        rule = self.rules[fy.regime]
        if rule.kind == "week_based":
            long_year = (e - s) // 7 == 53
            out, cur = [], s
            for i, w in enumerate(rule.period_weeks, start=1):
                extra = long_year and i == rule.extra_period
                w += 1 if extra else 0
                out.append(Span("period", i, cur, cur + 7 * w, w, extra))
                cur += 7 * w
            assert cur == e
            return out
        starts = []
        for i in range(12):
            m0 = rule.month - 1 + i
            starts.append(rule.adjusted(fy.key + m0 // 12, m0 % 12 + 1, rule.day))
        starts.append(e)
        assert starts[0] == s
        return [Span("period", i + 1, starts[i], starts[i + 1], None, False) for i in range(12)]

    def _week_spans(self, k: int) -> list[Span]:
        """Week 1 runs from the year's first day to the day before the next week start; the last week ends with the year."""
        fy = self.seq[k]
        s, e = fy.start, fy.end
        pattern = self.week_pattern(k)
        rule = self.rules[fy.regime] if fy.regime is not None else None
        extra_idx = None
        if rule is not None and rule.kind == "week_based" and (e - s) // 7 == 53:
            extra_idx = sum(rule.period_weeks[: rule.extra_period]) + 1
        out, cur, i = [], s, 1
        while cur < e:
            step = (self.week_start - weekday_of(cur)) % 7 or 7
            nxt = min(cur + step, e)
            out.append(Span("week", i, cur, nxt, (nxt - cur) // 7 if pattern else None, i == extra_idx))
            cur, i = nxt, i + 1
        if pattern:
            assert all(w.end - w.start == 7 for w in out)
        if rule is not None and rule.kind == "fixed_date":
            assert 53 <= len(out) <= 54
        return out

    def period_value(self, k: int, sp: Span) -> dict:
        return {
            "level": sp.level,
            "year": self.number(k),
            "index": sp.index,
            "start": iso(sp.start),
            "end_exclusive": iso(sp.end),
            "last_day": iso(sp.end - 1),
            "days": sp.end - sp.start,
            "weeks": sp.weeks,
            "has_extra_week": sp.extra,
        }

    # Operations --------------------------------------------------------

    def _resolve(self, level: str, year: int, index, path_year="year", path_index="index") -> tuple[int, Span]:
        if level not in LEVELS:
            raise FiscalError("INVALID_ARGUMENT", "level")
        k = self.key_for_scheme_number(year, self.default_scheme)
        if k is None:
            skipped = self.number(0) <= year <= self.number(self.last_key())
            raise FiscalError("PERIOD_NOT_FOUND" if skipped else "OUT_OF_RANGE", path_year)
        spans = self.spans(k, level)
        if level == "year":
            return k, spans[0]
        if not isinstance(index, int) or not 1 <= index <= len(spans):
            raise FiscalError("PERIOD_NOT_FOUND", path_index)
        return k, spans[index - 1]

    def period(self, level: str, year: int, index=None) -> dict:
        k, sp = self._resolve(level, year, index)
        return self.period_value(k, sp)

    def _arg_period(self, p: dict) -> tuple[int, Span]:
        try:
            return self._resolve(p["level"], p["year"], p.get("index"), "period", "period")
        except FiscalError as err:
            raise FiscalError(err.code, "period") from None

    def locate(self, d: str) -> dict:
        o = parse_iso(d)
        k = self.find(o)
        if k is None:
            raise FiscalError("OUT_OF_RANGE", "date")
        found = {}
        for level in LEVELS:
            (sp,) = [x for x in self.spans(k, level) if x.start <= o < x.end]
            found[level] = sp
        out = {"date": d}
        for level in LEVELS:
            out[level] = self.period_value(k, found[level])
        out["day_of_year"] = o - found["year"].start + 1
        out["day_of_quarter"] = o - found["quarter"].start + 1
        out["day_of_period"] = o - found["period"].start + 1
        out["day_of_week"] = (weekday_of(o) - self.week_start) % 7 + 1
        return out

    def count(self, k: int, level: str) -> int:
        return len(self.spans(k, level))

    def shift(self, p: dict, n: int) -> dict:
        """Steps through the sequence of fiscal years, and through each year's own quarters, periods or weeks."""
        k, sp = self._arg_period(p)
        if not isinstance(n, int) or isinstance(n, bool):
            raise FiscalError("INVALID_ARGUMENT", "n")
        if sp.level == "year":
            if not self.in_range(k + n):
                raise FiscalError("OUT_OF_RANGE", "n")
            return self.period_value(k + n, self.spans(k + n, "year")[0])
        idx = sp.index + n
        while idx > self.count(k, sp.level):
            idx -= self.count(k, sp.level)
            k += 1
            if not self.in_range(k):
                raise FiscalError("OUT_OF_RANGE", "n")
        while idx < 1:
            k -= 1
            if not self.in_range(k):
                raise FiscalError("OUT_OF_RANGE", "n")
            idx += self.count(k, sp.level)
        return self.period_value(k, self.spans(k, sp.level)[idx - 1])

    def parent(self, p: dict) -> dict:
        k, sp = self._arg_period(p)
        up = {
            "quarter": "year",
            "period": "quarter",
            "week": "period" if self.week_pattern(k) else "year",
        }.get(sp.level)
        if up is None:
            raise FiscalError("INVALID_ARGUMENT", "period")
        (par,) = [x for x in self.spans(k, up) if x.start <= sp.start < x.end]
        return self.period_value(k, par)

    def children(self, p: dict, level: str) -> list[dict]:
        k, sp = self._arg_period(p)
        if level not in LEVELS:
            raise FiscalError("INVALID_ARGUMENT", "level")
        if not self.week_pattern(k) and level == "week" and sp.level in ("quarter", "period"):
            raise FiscalError("LEVEL_NOT_NESTED", "level")
        assert LEVELS.index(level) > LEVELS.index(sp.level)
        return [self.period_value(k, x) for x in self.spans(k, level) if sp.start <= x.start < sp.end]

    def years(self, first: int, last: int) -> list[dict]:
        """Every fiscal year whose number lies in [first, last]; empty only where numbers are skipped."""
        if last < first or last - first + 1 > 10000:
            raise FiscalError("INVALID_ARGUMENT", "to")
        lowest, highest = self.number(0), self.number(self.last_key())
        if not lowest <= first <= highest:
            raise FiscalError("OUT_OF_RANGE", "from")
        if not lowest <= last <= highest:
            raise FiscalError("OUT_OF_RANGE", "to")
        keys = [self.key_for_scheme_number(n, self.default_scheme) for n in range(first, last + 1)]
        return [self.period_value(k, self.spans(k, "year")[0]) for k in keys if k is not None]

    # Labels --------------------------------------------------------------

    def _fill(self, template: str, number: int, k: int, sp: Span) -> str:
        if "{yyyy}" in template and not 1 <= number <= 9999:
            raise FiscalError("OUT_OF_RANGE", "period")
        values = {
            "yyyy": "%04d" % number,
            "yy": "%02d" % (number % 100),
            "yy_next": "%02d" % ((number + 1) % 100),
            "yy_end": "%02d" % (civil_year(self.seq[k].end - 1) % 100),
            "q": str(sp.index),
            "pp": "%02d" % (sp.index or 0),
            "ww": "%02d" % (sp.index or 0),
        }
        return re.sub(r"\{([a-z_]+)\}", lambda m: values[m.group(1)], template)

    def format(self, p: dict, scheme: str | None = None) -> dict:
        k, sp = self._arg_period(p)
        name = self.default_scheme if scheme is None else scheme
        if name not in self.schemes:
            raise FiscalError("SCHEME_NOT_FOUND", "scheme")
        template = self.schemes[name]["templates"].get(sp.level)
        if template is None:
            raise FiscalError("TEMPLATE_MISSING", "period")
        return {"label": self._fill(template, self.scheme_number(k, name), k, sp), "scheme": name}

    @staticmethod
    def _compile(template: str) -> re.Pattern:
        widths = {"yyyy": "[0-9]{4}", "yy": "[0-9]{2}", "yy_next": "[0-9]{2}", "yy_end": "[0-9]{2}", "q": "[1-9][0-9]?", "pp": "[0-9]{2}", "ww": "[0-9]{2}"}
        out, pos = [], 0
        for m in re.finditer(r"\{([a-z_]+)\}", template):
            out.append(re.escape(template[pos : m.start()]))
            out.append("(?P<%s>%s)" % (m.group(1), widths[m.group(1)]))
            pos = m.end()
        out.append(re.escape(template[pos:]))
        return re.compile("".join(out))

    def parse(self, label: str) -> dict:
        found = []
        for name in sorted(self.schemes):
            sc = self.schemes[name]
            for level in LEVELS:
                template = sc["templates"].get(level)
                if template is None:
                    continue
                m = self._compile(template).fullmatch(label)
                if not m:
                    continue
                g = m.groupdict()
                if "yyyy" in g:
                    number = int(g["yyyy"])
                else:
                    pivot = sc["century_pivot"]
                    number = next(y for y in range(pivot - 99, pivot + 1) if y % 100 == int(g["yy"]))
                if "yy" in g and int(g["yy"]) != number % 100:
                    continue
                if "yy_next" in g and int(g["yy_next"]) != (number + 1) % 100:
                    continue
                k = self.key_for_scheme_number(number, name)
                if k is None:
                    continue
                if "yy_end" in g and int(g["yy_end"]) != civil_year(self.seq[k].end - 1) % 100:
                    continue
                spans = self.spans(k, level)
                if level == "year":
                    sp = spans[0]
                else:
                    idx = int(g.get("q") or g.get("pp") or g.get("ww"))
                    if not 1 <= idx <= len(spans):
                        continue
                    sp = spans[idx - 1]
                found.append((sp.start, name, level, self.period_value(k, sp)))
        merged = {}
        for start, name, level, value in found:
            merged.setdefault((level, start), (start, name, level, value))
        cands = sorted(merged.values(), key=lambda c: (c[0], c[1]))
        if not cands:
            raise FiscalError("LABEL_NOT_RECOGNISED", "label")
        if len(cands) > 1:
            details = {"candidates": [{"period": c[3], "scheme": c[1], "level": c[2]} for c in cands]}
            raise FiscalError("AMBIGUOUS_LABEL", "label", details)
        return cands[0][3]

    # Dimension -----------------------------------------------------------

    def label_columns(self) -> list[tuple[str, str]]:
        cols = []
        for name in sorted(self.schemes):
            for level in LEVELS:
                if level in self.schemes[name]["templates"]:
                    cols.append((name, level))
        return cols

    def dimension(self, start: str, end_exclusive: str) -> list[dict]:
        s, e = parse_iso(start, "start"), parse_iso(end_exclusive, "end_exclusive")
        if e < s or e - s > CYCLE_DAYS:
            raise FiscalError("INVALID_ARGUMENT", "end_exclusive")
        if s == e:
            return []
        if self.find(s) is None:
            raise FiscalError("OUT_OF_RANGE", "start")
        if self.find(e - 1) is None:
            raise FiscalError("OUT_OF_RANGE", "end_exclusive")
        cid = calendar_id(self.definition)
        rows = []
        for o in range(s, e):
            loc = self.locate(iso(o))
            y, q, p, w = loc["year"], loc["quarter"], loc["period"], loc["week"]
            row = {
                "date": iso(o),
                "calendar_id": cid,
                "fiscal_year": y["year"],
                "quarter": q["index"],
                "period": p["index"],
                "week": w["index"],
                "day_of_year": loc["day_of_year"],
                "day_of_quarter": loc["day_of_quarter"],
                "day_of_period": loc["day_of_period"],
                "day_of_week": loc["day_of_week"],
                "year_start": y["start"],
                "year_last_day": y["last_day"],
                "quarter_start": q["start"],
                "quarter_last_day": q["last_day"],
                "period_start": p["start"],
                "period_last_day": p["last_day"],
                "week_start": w["start"],
                "week_last_day": w["last_day"],
                "year_has_extra_week": y["has_extra_week"],
                "period_has_extra_week": p["has_extra_week"],
                "week_is_extra_week": w["has_extra_week"],
            }
            for name, level in self.label_columns():
                row[f"label_{name}_{level}"] = self.format(loc[level], name)["label"]
            rows.append(row)
        return rows

    def dimension_columns(self) -> list[str]:
        fixed = (
            "date calendar_id fiscal_year quarter period week day_of_year day_of_quarter day_of_period day_of_week "
            "year_start year_last_day quarter_start quarter_last_day period_start period_last_day week_start week_last_day "
            "year_has_extra_week period_has_extra_week week_is_extra_week"
        ).split()
        return fixed + [f"label_{name}_{level}" for name, level in self.label_columns()]

    def dimension_csv(self, start: str, end_exclusive: str) -> str:
        rows = self.dimension(start, end_exclusive)

        def cell(v) -> str:
            if isinstance(v, bool):
                return "true" if v else "false"
            s = str(v)
            if any(c in s for c in ',"\r\n'):
                return '"' + s.replace('"', '""') + '"'
            return s

        header = self.dimension_columns()
        assert all(list(r) == header for r in rows)
        lines = [",".join(header)] + [",".join(cell(r[h]) for h in header) for r in rows]
        return "\n".join(lines) + "\n"


# Canonical form ----------------------------------------------------------


_ESCAPES = {'"': '\\"', "\\": "\\\\", "\b": "\\b", "\f": "\\f", "\n": "\\n", "\r": "\\r", "\t": "\\t"}


def _json_string(s: str) -> str:
    out = []
    for c in s:
        if c in _ESCAPES:
            out.append(_ESCAPES[c])
        elif ord(c) < 0x20:
            out.append("\\u%04x" % ord(c))
        else:
            out.append(c)
    return '"' + "".join(out) + '"'


def _serialise(v) -> str:
    if v is None:
        return "null"
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return str(v)
    if isinstance(v, str):
        return _json_string(v)
    if isinstance(v, list):
        return "[" + ",".join(_serialise(x) for x in v) + "]"
    assert isinstance(v, dict), type(v)
    items = sorted(v.items(), key=lambda kv: kv[0].encode("utf-8"))
    return "{" + ",".join(_json_string(k) + ":" + _serialise(x) for k, x in items) + "}"


def canonical(definition: dict) -> str:
    """Absent or null `exceptions` is left out, so a definition without exceptions keeps its 0.1.0 canonical form."""
    d = copy.deepcopy(definition)
    d.pop("$schema", None)
    if d["year"]["type"] == "fixed_date":
        d["year"].setdefault("short_month", None)
    if d.get("exceptions") is None:
        d.pop("exceptions", None)
    else:
        for entry in d["exceptions"]["history"]:
            if entry["year"]["type"] == "fixed_date":
                entry["year"].setdefault("short_month", None)
            entry.setdefault("transition", None)
    for sc in d["naming"]["schemes"].values():
        sc.setdefault("offset", 0)
        sc.setdefault("century_pivot", None)
    return _serialise(d)


def calendar_id(definition: dict) -> str:
    return hashlib.sha256(canonical(definition).encode("utf-8")).hexdigest()[:16]
