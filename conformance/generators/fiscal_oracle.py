"""Independent implementation of spec/definition-v1.md and spec/api-v1.md.

Standard library only. Written from the spec text without reference to the
fiscalclock sources, so that its answers can serve as expected results.
Dates are handled as proleptic Gregorian ordinals (date.toordinal()), which
lets boundary arithmetic run past 0001-01-01 and 9999-12-31 without overflow.
"""

from __future__ import annotations

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


class Calendar:
    def __init__(self, definition: dict):
        self.definition = definition
        y = definition["year"]
        self.week_start = WEEKDAYS.index(definition["week"]["starts_on"])
        self.kind = y["type"]
        sub = definition["subdivision"]
        self.quarters = list(sub["quarters"])
        if self.kind == "week_based":
            a = y["anchor"]
            self.edge = a["edge"]
            self.month = a["month"]
            self.day = a["day"]
            self.rule = a["rule"]
            self.qual = self.week_start if self.edge == "start" else (self.week_start - 1) % 7
            self.period_weeks = list(sub["periods"])
            self.extra_period = sub["extra_week"]["period"]
            assert sub["type"] == "week_pattern" and sum(self.period_weeks) == 52
        else:
            self.month = y["start"]["month"]
            self.day = y["start"]["day"]
            self.short_month = y.get("short_month")
            assert sub["type"] == "calendar_months" and sub["month_start_day"] == self.day
        assert sum(self.quarters) == self.n_periods()
        naming = definition["naming"]
        self.default_scheme = naming["default"]
        self.schemes = naming["schemes"]
        self._bounds: dict[int, tuple[int, int]] = {}

    # Year bounds -------------------------------------------------------

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

    def _adjusted(self, y: int, m: int, d: int) -> int:
        dim = days_in_month(y, m)
        if d <= dim:
            return ymd_ord(y, m, d)
        if self.short_month == "clamp":
            return ymd_ord(y, m, dim)
        assert self.short_month == "next"
        return ymd_ord(y, m, dim) + 1

    def key_bounds(self, k: int) -> tuple[int, int]:
        if k not in self._bounds:
            if self.kind == "fixed_date":
                b = self._adjusted(k, self.month, self.day), self._adjusted(k + 1, self.month, self.day)
            elif self.edge == "end":
                b = self._boundary(k - 1) + 1, self._boundary(k) + 1
            else:
                b = self._boundary(k), self._boundary(k + 1)
            self._bounds[k] = b
        return self._bounds[k]

    def key_of(self, o: int) -> int:
        y = civil_year(o)
        for k in (y - 1, y, y + 1, y - 2, y + 2):
            s, e = self.key_bounds(k)
            if s <= o < e:
                return k
        raise AssertionError("no fiscal year contains ordinal %d" % o)

    def in_range(self, k: int) -> bool:
        s, e = self.key_bounds(k)
        return s >= MIN_ORD and e - 1 <= MAX_ORD

    def scheme_number(self, k: int, scheme: str) -> int | None:
        """The year number of fiscal year k, or None under "majority" when the year is split evenly."""
        sc = self.schemes[scheme]
        s, e = self.key_bounds(k)
        rule = sc["year_number"]
        if rule == "start":
            y = civil_year(s)
        elif rule == "end":
            y = civil_year(e - 1)
        else:
            assert rule == "majority"
            first = civil_year(s)
            in_first = min(e, ymd_ord(first + 1, 1, 1)) - s
            if 2 * in_first == e - s:
                return None
            y = first if 2 * in_first > e - s else first + 1
        return y + sc.get("offset", 0)

    def number(self, k: int) -> int:
        return self.scheme_number(k, self.default_scheme)

    def key_for_scheme_number(self, n: int, scheme: str) -> int | None:
        found = None
        approx = n - self.schemes[scheme].get("offset", 0)
        for k in range(approx - 2, approx + 3):
            if self.scheme_number(k, scheme) == n:
                assert found is None, "year number %d names two fiscal years" % n
                found = k
        return found

    def naming_errors(self) -> list[FiscalError]:
        """MAJORITY_TIE or YEAR_NUMBER_COLLISION per scheme, over every fiscal year in the supported range."""
        keys = [k for k in range(0, 10001) if self.in_range(k)]
        errors = []
        for name in sorted(self.schemes):
            path = f"naming.schemes.{name}.year_number"
            numbers = [self.scheme_number(k, name) for k in keys]
            if None in numbers:
                errors.append(FiscalError("MAJORITY_TIE", path))
            elif any(b != a + 1 for a, b in zip(numbers, numbers[1:])):
                errors.append(FiscalError("YEAR_NUMBER_COLLISION", path))
        return sorted(errors, key=lambda err: (err.path, err.code))

    # Structure ---------------------------------------------------------

    def spans(self, k: int, level: str) -> list[Span]:
        s, e = self.key_bounds(k)
        if level == "year":
            if self.kind == "week_based":
                w = (e - s) // 7
                return [Span("year", None, s, e, w, w == 53)]
            return [Span("year", None, s, e, None, False)]
        if level == "week":
            return self._week_spans(k, s, e)
        periods = self._period_spans(k, s, e)
        if level == "period":
            return periods
        out, i = [], 0
        for q, n in enumerate(self.quarters, start=1):
            group = periods[i : i + n]
            i += n
            weeks = sum(p.weeks for p in group) if self.kind == "week_based" else None
            out.append(Span("quarter", q, group[0].start, group[-1].end, weeks, any(p.extra for p in group)))
        return out

    def _period_spans(self, k: int, s: int, e: int) -> list[Span]:
        if self.kind == "week_based":
            long_year = (e - s) // 7 == 53
            out, cur = [], s
            for i, w in enumerate(self.period_weeks, start=1):
                extra = long_year and i == self.extra_period
                w += 1 if extra else 0
                out.append(Span("period", i, cur, cur + 7 * w, w, extra))
                cur += 7 * w
            assert cur == e
            return out
        starts = []
        for i in range(12):
            m0 = self.month - 1 + i
            starts.append(self._adjusted(k + m0 // 12, m0 % 12 + 1, self.day))
        starts.append(e)
        assert starts[0] == s
        return [Span("period", i + 1, starts[i], starts[i + 1], None, False) for i in range(12)]

    def _week_spans(self, k: int, s: int, e: int) -> list[Span]:
        if self.kind == "week_based":
            n = (e - s) // 7
            extra_idx = sum(self.period_weeks[: self.extra_period]) + 1 if n == 53 else None
            return [Span("week", i, s + 7 * (i - 1), s + 7 * i, 1, i == extra_idx) for i in range(1, n + 1)]
        out, cur, i = [], s, 1
        while cur < e:
            step = (self.week_start - weekday_of(cur)) % 7 or 7
            nxt = min(cur + step, e)
            out.append(Span("week", i, cur, nxt, None, False))
            cur, i = nxt, i + 1
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
        if k is None or not self.in_range(k):
            raise FiscalError("OUT_OF_RANGE", path_year)
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
        k = self.key_of(o)
        if not self.in_range(k):
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
        k, sp = self._arg_period(p)
        if not isinstance(n, int) or isinstance(n, bool):
            raise FiscalError("INVALID_ARGUMENT", "n")
        year = self.number(k)
        if sp.level == "year":
            target = self.key_for_scheme_number(year + n, self.default_scheme)
            if target is None or not self.in_range(target):
                raise FiscalError("OUT_OF_RANGE", "n")
            return self.period_value(target, self.spans(target, "year")[0])
        idx = sp.index + n
        while idx > self.count(k, sp.level):
            idx -= self.count(k, sp.level)
            k += 1
        while idx < 1:
            k -= 1
            idx += self.count(k, sp.level)
        if not self.in_range(k):
            raise FiscalError("OUT_OF_RANGE", "n")
        return self.period_value(k, self.spans(k, sp.level)[idx - 1])

    def parent(self, p: dict) -> dict:
        k, sp = self._arg_period(p)
        up = {
            "quarter": "year",
            "period": "quarter",
            "week": "period" if self.kind == "week_based" else "year",
        }.get(sp.level)
        if up is None:
            raise FiscalError("INVALID_ARGUMENT", "period")
        (par,) = [x for x in self.spans(k, up) if x.start <= sp.start < x.end]
        return self.period_value(k, par)

    def children(self, p: dict, level: str) -> list[dict]:
        k, sp = self._arg_period(p)
        if level not in LEVELS:
            raise FiscalError("INVALID_ARGUMENT", "level")
        if self.kind == "fixed_date" and level == "week" and sp.level in ("quarter", "period"):
            raise FiscalError("LEVEL_NOT_NESTED", "level")
        assert LEVELS.index(level) > LEVELS.index(sp.level)
        return [self.period_value(k, x) for x in self.spans(k, level) if sp.start <= x.start < sp.end]

    def years(self, first: int, last: int) -> list[dict]:
        return [self.period("year", y) for y in range(first, last + 1)]

    # Labels --------------------------------------------------------------

    def _fill(self, template: str, number: int, sp: Span) -> str:
        if "{yyyy}" in template and not 1 <= number <= 9999:
            raise FiscalError("OUT_OF_RANGE", "period")
        values = {
            "yyyy": "%04d" % number,
            "yy": "%02d" % (number % 100),
            "yy_next": "%02d" % ((number + 1) % 100),
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
        return {"label": self._fill(template, self.scheme_number(k, name), sp), "scheme": name}

    @staticmethod
    def _compile(template: str) -> re.Pattern:
        widths = {"yyyy": "[0-9]{4}", "yy": "[0-9]{2}", "yy_next": "[0-9]{2}", "q": "[1-9][0-9]?", "pp": "[0-9]{2}", "ww": "[0-9]{2}"}
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
                if k is None or not self.in_range(k):
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
        if not self.in_range(self.key_of(s)):
            raise FiscalError("OUT_OF_RANGE", "start")
        if not self.in_range(self.key_of(e - 1)):
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
    d = copy.deepcopy(definition)
    d.pop("$schema", None)
    if d["year"]["type"] == "fixed_date":
        d["year"].setdefault("short_month", None)
    for sc in d["naming"]["schemes"].values():
        sc.setdefault("offset", 0)
        sc.setdefault("century_pivot", None)
    return _serialise(d)


def calendar_id(definition: dict) -> str:
    return hashlib.sha256(canonical(definition).encode("utf-8")).hexdigest()[:16]
