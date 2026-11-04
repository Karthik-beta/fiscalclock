"""Oracle vectors: expected results from fiscal_oracle.py and from Python's ISO calendar."""

from __future__ import annotations

import copy
import hashlib
from datetime import date, timedelta

from fiscal_oracle import WEEKDAYS, Calendar, iso
from vectorlib import (
    ORACLE_CITATION,
    case,
    coords,
    fixed_date,
    naming,
    run,
    scheme,
    source,
    validate_case,
    vector,
    week_based,
    year_args,
)

VERIFY = "research/verification/verify_claims.py (run 2026-10-31)"

NRF_TEMPLATES = {"year": "FY{yyyy}", "quarter": "FY{yyyy} Q{q}", "period": "FY{yyyy} P{pp}", "week": "FY{yyyy} W{ww}"}


def nrf_definition() -> dict:
    return week_based("end", 1, "last", "nearest", "sunday", [4, 5, 4] * 4, [3, 3, 3, 3], 12, naming({"nrf": scheme("start", NRF_TEMPLATES)}))


def apple_definition() -> dict:
    templates = {"year": "FY{yy}", "quarter": "Q{q} FY{yy}"}
    return week_based("end", 9, "last", "last", "sunday", [4, 4, 5] * 4, [3, 3, 3, 3], 1, naming({"fy": scheme("end", templates, pivot=2069)}))


def oracle_source(checked_by: str) -> dict:
    return source("oracle", ORACLE_CITATION, checked_by)


def year_cases(cal: Calendar, years) -> list[dict]:
    return [case(cal, "period", year_args(y)) for y in years]


def structure_cases(cal: Calendar, years, levels=("quarter", "period")) -> list[dict]:
    out = []
    for y in years:
        k = cal.key_for_scheme_number(y, cal.default_scheme)
        for level in levels:
            for i in range(1, cal.count(k, level) + 1):
                out.append(case(cal, "period", {"level": level, "year": y, "index": i}))
    return out


def boundary_dates(cal: Calendar, y: int) -> list[str]:
    k = cal.key_for_scheme_number(y, cal.default_scheme)
    s, e = cal.key_bounds(k)
    return [iso(s - 1), iso(s), iso(e - 1), iso(e)]


def locate_cases(cal: Calendar, dates) -> list[dict]:
    seen, out = set(), []
    for d in dates:
        if d not in seen:
            seen.add(d)
            out.append(case(cal, "locate", {"date": d}))
    return out


def label_cases(cal: Calendar, periods, schemes=(None,)) -> list[dict]:
    out, parsed = [], set()
    for p in periods:
        for sc in schemes:
            if p["level"] not in cal.schemes[sc or cal.default_scheme]["templates"]:
                continue
            args = {"period": p} if sc is None else {"period": p, "scheme": sc}
            fmt = case(cal, "format", args)
            out.append(fmt)
            label = fmt.get("expect", {}).get("label")
            if label is not None and label not in parsed:
                parsed.add(label)
                out.append(case(cal, "parse", {"label": label}))
    return out


# 1. Week-based spread ----------------------------------------------------

ANCHORS = [(1, "last"), (2, 14), (3, "last"), (4, 1), (5, "last"), (6, 15), (7, "last"), (8, 1), (9, "last"), (10, 28), (11, "last"), (12, 15), (12, "last"), (1, 1)]
PATTERNS = [
    ([4, 4, 5] * 4, [3, 3, 3, 3], 12),
    ([4, 5, 4] * 4, [3, 3, 3, 3], 3),
    ([5, 4, 4] * 4, [3, 3, 3, 3], 1),
    ([4] * 13, [3, 3, 3, 4], 13),
    ([4] * 13, [4, 3, 3, 3], 7),
]


def valid_naming(defn: dict, preferred: str) -> tuple[dict, str, list[str]]:
    """The first of preferred, the other edge, then majority that passes YEAR_NUMBER_COLLISION and MAJORITY_TIE."""
    rejected = []
    for year_number in (preferred, "start" if preferred == "end" else "end", "majority"):
        d = copy.deepcopy(defn)
        d["naming"]["schemes"]["fy"]["year_number"] = year_number
        errors = Calendar(d).naming_errors()
        if not errors:
            return d, year_number, rejected
        rejected.append(f"{year_number} ({errors[0].code})")
    raise AssertionError(f"no valid year_number for {defn['year']}")


def week_based_spread() -> list[dict]:
    out = []
    i = 0
    for edge in ("end", "start"):
        for rule in ("last", "first", "nearest"):
            for starts_on in WEEKDAYS:
                month, day = ANCHORS[i % len(ANCHORS)]
                periods, quarters, extra = PATTERNS[i % len(PATTERNS)]
                i += 1
                base = week_based(edge, month, day, rule, starts_on, periods, quarters, extra, naming({"fy": scheme("end", {"year": "FY{yyyy}"})}))
                defn, year_number, rejected = valid_naming(base, "end" if edge == "end" else "start")
                cal = Calendar(defn)
                cases = [validate_case(defn)] + year_cases(cal, range(2015, 2031))
                dates = boundary_dates(cal, 2023) + boundary_dates(cal, 2028)
                for y in range(2015, 2031):
                    k = cal.key_for_scheme_number(y, "fy")
                    weeks = cal.spans(k, "week")
                    extra_weeks = [w for w in weeks if w.extra]
                    if extra_weeks:
                        dates += [iso(extra_weeks[0].start - 1), iso(extra_weeks[0].start), iso(extra_weeks[0].end - 1)]
                        break
                cases += locate_cases(cal, dates)
                name = f"week-based-{edge}-{rule}-{starts_on}"
                why = f" ({', '.join(rejected)} rejected for this rule)" if rejected else ""
                desc = (
                    f"Week-based year, anchor edge {edge}, rule {rule}, month {month}, day {day}, weeks start {starts_on}; "
                    f"periods {periods}, quarters {quarters}, extra week in period {extra}; years named by year_number '{year_number}'{why}. "
                    f"The generator checked that this scheme numbers every fiscal year in the supported range consecutively, with no majority tie. "
                    f"Year bounds 2015-2030 and full locations on boundary days and around the first extra week."
                )
                out.append(vector(name, desc, oracle_source("fiscal_oracle.Calendar.naming_errors; boundary arithmetic shared with verify_claims.py C01"), defn, cases))
    return out


# 2. NRF ------------------------------------------------------------------


def nrf_vector() -> dict:
    defn = nrf_definition()
    cal = Calendar(defn)
    cases = [validate_case(defn)]
    cases += year_cases(cal, range(2016, 2031))
    cases.append(case(cal, "years", {"from": 2016, "to": 2030}))
    cases += structure_cases(cal, (2017, 2023, 2024))
    cases.append(case(cal, "children", {"period": coords("year", 2023), "level": "week"}))
    cases += locate_cases(cal, ["2023-01-29", "2024-02-03", "2024-02-04", "2024-01-28", "2023-12-31", "2017-01-28", "2017-01-29", "2018-02-03"])
    stale = cal.period("week", 2023, 53)
    stale["start"] = "1999-01-01"
    stale["days"] = 1
    nav = [
        ("shift", {"period": coords("week", 2022, 52), "n": 1}),
        ("shift", {"period": coords("week", 2023, 52), "n": 1}),
        ("shift", {"period": coords("week", 2023, 53), "n": 1}),
        ("shift", {"period": coords("week", 2024, 1), "n": -1}),
        ("shift", {"period": coords("week", 2023, 1), "n": -1}),
        ("shift", {"period": coords("week", 2017, 1), "n": 53}),
        ("shift", {"period": coords("week", 2022, 10), "n": 104}),
        ("shift", {"period": coords("week", 2024, 10), "n": -104}),
        ("shift", {"period": stale, "n": 1}),
        ("shift", {"period": coords("period", 2023, 12), "n": 1}),
        ("shift", {"period": coords("period", 2023, 1), "n": -13}),
        ("shift", {"period": coords("quarter", 2016, 4), "n": 1}),
        ("shift", {"period": coords("quarter", 2023, 4), "n": -9}),
        ("shift", {"period": coords("year", 2023), "n": 5}),
        ("shift", {"period": coords("year", 2023), "n": -6}),
        ("shift", {"period": coords("week", 2023, 20), "n": 0}),
        ("parent", {"period": coords("week", 2023, 53)}),
        ("parent", {"period": coords("week", 2023, 1)}),
        ("parent", {"period": coords("period", 2023, 12)}),
        ("parent", {"period": coords("quarter", 2023, 4)}),
        ("children", {"period": coords("quarter", 2023, 4), "level": "period"}),
        ("children", {"period": coords("quarter", 2023, 4), "level": "week"}),
        ("children", {"period": coords("period", 2023, 12), "level": "week"}),
        ("children", {"period": coords("year", 2024), "level": "quarter"}),
        ("children", {"period": coords("year", 2024), "level": "period"}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", 2023), coords("quarter", 2023, 4), coords("period", 2023, 12), coords("week", 2023, 53), coords("week", 2024, 1)])
    cases.append(case(cal, "parse", {"label": "FY2024 W53"}))
    cases.append(case(cal, "parse", {"label": "FY2023 Q5"}))
    cases.append(case(cal, "parse", {"label": "FY2023 Q01"}))
    desc = (
        "NRF 4-5-4 retail calendar: year ends on the Saturday nearest 31 January, weeks start Sunday, periods 4-5-4 per quarter, "
        "extra week in period 12, years named by start year. Year bounds 2016-2030; quarters and periods of 2017 and 2023 (53-week years) "
        "and 2024; all weeks of 2023; navigation across year boundaries into and out of 53-week years; labels ('FY2023 Q01' is "
        "LABEL_NOT_RECOGNISED because {q} matches [1-9][0-9]?, with no leading zero). A shift case passes a Period "
        "whose start and days were edited, which the core must ignore."
    )
    return vector("nrf-4-5-4", desc, oracle_source(f"{VERIFY} C03 (53-week years 2017, 2023, 2028), C04, C05, C08 agree"), defn, cases)


# 3. Apple-style ----------------------------------------------------------


def apple_vector() -> dict:
    defn = apple_definition()
    cal = Calendar(defn)
    cases = [validate_case(defn)]
    cases += year_cases(cal, range(2016, 2031))
    cases += structure_cases(cal, (2023, 2024), ("quarter",))
    cases += structure_cases(cal, (2023,), ("period",))
    cases.append(case(cal, "children", {"period": coords("period", 2023, 1), "level": "week"}))
    cases += locate_cases(cal, ["2022-09-24", "2022-09-25", "2022-10-29", "2022-10-30", "2023-09-30", "2023-10-01", "2023-12-30"])
    nav = [
        ("shift", {"period": coords("quarter", 2024, 1), "n": -4}),
        ("shift", {"period": coords("week", 2022, 52), "n": 1}),
        ("shift", {"period": coords("week", 2023, 53), "n": 1}),
        ("parent", {"period": coords("week", 2023, 5)}),
        ("parent", {"period": coords("week", 2023, 6)}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", 2023), coords("year", 2000), coords("year", 2069), coords("quarter", 2024, 1)])
    for label in ("FY69", "FY70", "FY99", "FY00", "Q1 FY24"):
        cases.append(case(cal, "parse", {"label": label}))
    desc = (
        "Apple-style year: ends on the last Saturday of September, weeks start Sunday, named by end year. The quarter pattern "
        "(13-week quarters, periods 4-4-5, extra week in period 1 so that the first quarter has 14 weeks in a 53-week year) is "
        "illustrative of Apple's first-quarter placement, not Apple's actual internal month pattern. Two-digit year labels resolve "
        "through century_pivot 2069, so FY69 is 2069 and FY70 is 1970."
    )
    return vector("apple-style", desc, oracle_source(f"{VERIFY} C09 (53-week years 2017, 2023, 2028) agrees"), defn, cases)


# 4. ISO week year --------------------------------------------------------


def iso_definition(year_number: str) -> dict:
    return week_based("start", 1, 1, "nearest", "monday", [4] * 13, [3, 3, 3, 4], 13, naming({"iso": scheme(year_number, {"year": "{yyyy}", "week": "{yyyy}-W{ww}"})}))


def matches_subset(got, want) -> bool:
    if isinstance(want, dict):
        return isinstance(got, dict) and all(k in got and matches_subset(got[k], v) for k, v in want.items())
    return got == want


def iso_vector() -> dict:
    defn = iso_definition("majority")
    cal = Calendar(defn)
    cases = [validate_case(defn)]

    def checked(op: str, args: dict, want: dict, exact: bool = False) -> dict:
        got = run(cal, op, args)
        assert got == want if exact else matches_subset(got, want), (op, args, want, got)
        return {"op": op, "args": args, "expect" if exact else "expect_subset": want}

    dates = [d for y in range(1600, 2400) for d in (date(y, 12, 28), date(y, 1, 4))]
    dates += [d for y in range(1970, 2070) for d in (date(y, 1, 1), date(y, 12, 31))]
    for d in dates:
        iy, iw, idow = d.isocalendar()
        ystart = date.fromisocalendar(iy, 1, 1)
        ynext = date.fromisocalendar(iy + 1, 1, 1)
        nweeks = date(iy, 12, 28).isocalendar()[1]
        wstart = date.fromisocalendar(iy, iw, 1)
        subset = {
            "date": d.isoformat(),
            "year": {
                "year": iy,
                "start": ystart.isoformat(),
                "end_exclusive": ynext.isoformat(),
                "last_day": (ynext - timedelta(days=1)).isoformat(),
                "days": (ynext - ystart).days,
                "weeks": nweeks,
                "has_extra_week": nweeks == 53,
            },
            "week": {
                "year": iy,
                "index": iw,
                "start": wstart.isoformat(),
                "last_day": (wstart + timedelta(days=6)).isoformat(),
                "has_extra_week": iw == 53,
            },
            "day_of_year": (d - ystart).days + 1,
            "day_of_week": idow,
        }
        cases.append(checked("locate", {"date": d.isoformat()}, subset))
    for y in range(2015, 2031):
        for d in (date(y, 1, 1), date(y, 12, 31)):
            iy, iw, _ = d.isocalendar()
            week_label = "%04d-W%02d" % (iy, iw)
            cases.append(checked("format", {"period": coords("week", iy, iw)}, {"label": week_label, "scheme": "iso"}, exact=True))
            cases.append(checked("parse", {"label": week_label}, {"level": "week", "year": iy, "index": iw, "start": date.fromisocalendar(iy, iw, 1).isoformat()}))
        cases.append(checked("format", {"period": coords("year", y)}, {"label": "%04d" % y, "scheme": "iso"}, exact=True))
        cases.append(checked("parse", {"label": "%04d" % y}, {"level": "year", "year": y, "start": date.fromisocalendar(y, 1, 1).isoformat()}))
    desc = (
        "ISO 8601 week-numbering year expressed as a week-based definition: year starts on the Monday nearest 1 January, "
        "numbered with year_number 'majority'. Locate on 28 December and 4 January for every year 1600-2399, and on 1 January "
        "and 31 December (which can fall in the neighbouring ISO year) for 1970-2069. Week and year labels and their parse "
        "for 2015-2030. Expected year numbers, week index, week bounds, year bounds, year length, day of week and labels come "
        "from Python's date.isocalendar() and date.fromisocalendar(), not from the fiscalclock arithmetic. expect_subset is "
        "applied recursively: a nested object lists only the keys it asserts."
    )
    return vector("iso-week-year", desc, source("oracle", "Python standard library date.isocalendar() and date.fromisocalendar()", f"{VERIFY} C15 agrees; the generator also asserts fiscal_oracle.py matches on every case"), defn, cases)


# 5. Fixed-date calendars -------------------------------------------------

FIXED_YEARS = (1999, 2000, 2023, 2024, 2099, 2100)


def fixed_vector(name: str, desc: str, defn: dict, structure_years, extra_dates=(), label_periods=(), label_schemes=(None,), extra_cases=()) -> dict:
    cal = Calendar(defn)
    cases = [validate_case(defn)]
    cases += year_cases(cal, FIXED_YEARS)
    cases += structure_cases(cal, structure_years)
    dates = []
    for y in structure_years:
        dates += boundary_dates(cal, y)
    cases += locate_cases(cal, dates + list(extra_dates))
    cases.append(case(cal, "children", {"period": coords("year", 2024), "level": "week"}))
    cases += label_cases(cal, label_periods, label_schemes)
    cases += [case(cal, op, args) for op, args in extra_cases]
    full = desc + (
        " Weeks use contains_start: week 1 runs from the year's first day to the day before the next week start "
        "(7 days when the year starts on the week-start weekday), and day_of_week counts from the weekday, so a partial "
        "week can begin on a day_of_week other than 1."
    )
    return vector(name, full, oracle_source("fiscal_oracle.py; leap-year bounds hand-checked 2026-10-31"), defn, cases)


def fixed_vectors() -> list[dict]:
    out = []
    jan = fixed_date(1, 1, "monday", naming({
        "cal": scheme("start", {"year": "{yyyy}", "quarter": "{yyyy}-Q{q}", "period": "{yyyy}-M{pp}", "week": "{yyyy}-W{ww}"}),
        "be": scheme("start", {"year": "BE {yyyy}"}, offset=543),
    }))
    out.append(fixed_vector(
        "fixed-jan-1", "Calendar-year fiscal year starting 1 January, weeks start Monday; a second scheme 'be' adds offset 543 (Buddhist Era style).", jan, (2023, 2024),
        extra_dates=("2024-02-28", "2024-02-29", "2024-03-01", "2100-02-28", "2100-03-01", "2023-01-02", "0002-01-01", "9998-12-31"),
        label_periods=(coords("year", 2024), coords("quarter", 2024, 3), coords("period", 2024, 2), coords("week", 2024, 1), coords("week", 2024, 53), coords("week", 2023, 53)),
        label_schemes=(None, "be"),
        extra_cases=[
            ("parent", {"period": coords("week", 2024, 5)}),
            ("parent", {"period": coords("period", 2024, 5)}),
            ("children", {"period": coords("quarter", 2024, 1), "level": "period"}),
            ("children", {"period": coords("year", 2023), "level": "quarter"}),
            ("shift", {"period": coords("period", 2023, 12), "n": 1}),
            ("shift", {"period": coords("week", 2023, 53), "n": 1}),
            ("shift", {"period": coords("week", 2024, 1), "n": -1}),
            ("shift", {"period": coords("quarter", 2024, 1), "n": -5}),
            ("period", year_args(2)),
            ("period", year_args(9998)),
        ],
    ))
    india = fixed_date(4, 1, "monday", naming({
        "fy": scheme("start", {"year": "FY {yyyy}-{yy_next}", "quarter": "Q{q} FY {yyyy}-{yy_next}", "period": "P{pp} FY {yyyy}-{yy_next}"}),
        "ay": scheme("start", {"year": "AY {yyyy}–{yy_next}"}, offset=1),
        "short": scheme("start", {"year": "{yy}-{yy_next}"}, pivot=2069),
    }))
    out.append(fixed_vector(
        "fixed-apr-1-india", "India-style fiscal year starting 1 April, named by start year ('FY 2024-25'); scheme 'ay' is the assessment year, offset 1, with an en dash in its template to exercise non-ASCII labels. Scheme 'short' uses {yy}-{yy_next} with century_pivot 2069, so its label for 2099 is '99-00', which parses back as 1999 (the pivot window is 1970-2069), and {yy_next} is checked against the resolved year.", india, (2023, 2024, 2099),
        extra_dates=("2024-02-29", "2024-03-31", "2024-04-01"),
        label_periods=(coords("year", 2024), coords("year", 2099), coords("quarter", 2024, 4), coords("period", 2024, 12)),
        label_schemes=(None, "ay", "short"),
        extra_cases=[("parse", {"label": "24-26"}), ("parse", {"label": "69-70"}), ("parse", {"label": "70-71"})],
    ))
    jul = fixed_date(7, 1, "sunday", naming({"fy": scheme("end", {"year": "FY{yy}", "quarter": "Q{q} FY{yy}"}, pivot=2069)}))
    out.append(fixed_vector(
        "fixed-jul-1", "Fiscal year starting 1 July, named by end year, two-digit labels with century_pivot 2069, weeks start Sunday.", jul, (2024, 2100),
        label_periods=(coords("year", 2024), coords("year", 2000), coords("year", 2070), coords("quarter", 2024, 1)),
        extra_cases=[("parse", {"label": "FY69"}), ("parse", {"label": "FY70"})],
    ))
    oct_ = fixed_date(10, 1, "sunday", naming({"fy": scheme("end", {"year": "FY{yyyy}", "quarter": "FY{yyyy} Q{q}"})}))
    out.append(fixed_vector(
        "fixed-oct-1-us-federal", "US federal style fiscal year starting 1 October, named by end year (FY2024 is 1 Oct 2023 to 30 Sep 2024), weeks start Sunday.", oct_, (2024, 2025),
        label_periods=(coords("year", 2024), coords("quarter", 2024, 1), coords("quarter", 2024, 4)),
        extra_cases=[("period", year_args(2)), ("period", year_args(9999)), ("locate", {"date": "0001-10-01"}), ("locate", {"date": "9999-09-30"})],
    ))
    uk = fixed_date(4, 6, "monday", naming({"tax": scheme("start", {"year": "{yyyy}-{yy_next}", "period": "{yyyy}-{yy_next} M{pp}"})}))
    out.append(fixed_vector(
        "fixed-apr-6-uk-tax", "UK tax year style: starts 6 April, fiscal months start on the 6th (month_start_day 6), named by start year ('2024-25').", uk, (2023, 2024),
        extra_dates=("2024-04-05", "2024-04-06", "2025-04-05"),
        label_periods=(coords("year", 2024), coords("year", 2099), coords("period", 2024, 11)),
    ))
    for month, day, ws in ((2, 30, "monday"), (1, 31, "sunday")):
        for policy in ("clamp", "next"):
            defn = fixed_date(month, day, ws, naming({"fy": scheme("start", {"year": "FY{yyyy}", "period": "FY{yyyy} P{pp}"})}), short_month=policy)
            mname = "feb" if month == 2 else "jan"
            note = {"clamp": "a missing day becomes the month's last day", "next": "a missing day becomes the first day of the following month"}[policy]
            out.append(fixed_vector(
                f"fixed-{mname}-{day}-{policy}",
                f"Fixed-date year starting on day {day} of month {month} with short_month '{policy}': {note}. Covers leap and non-leap years (2024, 2100) and periods that start in short months.",
                defn, (2023, 2024, 2099, 2100),
                extra_dates=("2024-02-28", "2024-02-29", "2024-03-01", "2100-02-28", "2100-03-01"),
                label_periods=(coords("year", 2024), coords("period", 2024, 2)),
            ))
    return out


# 6. Dimension ------------------------------------------------------------


def dimension_vector() -> dict:
    defn = nrf_definition()
    defn["naming"]["schemes"]["retail"] = scheme("start", {"quarter": 'Q{q}, "{yyyy}"'})
    cal = Calendar(defn)
    cases = [
        validate_case(defn),
        case(cal, "dimension", {"start": "2024-01-28", "end_exclusive": "2024-02-07"}),
        case(cal, "dimension_csv", {"start": "2024-01-28", "end_exclusive": "2024-02-07"}),
        case(cal, "dimension", {"start": "2024-02-04", "end_exclusive": "2024-02-04"}),
        case(cal, "dimension_csv", {"start": "2024-02-04", "end_exclusive": "2024-02-04"}),
    ]
    desc = (
        "Date dimension rows for 10 days across the end of NRF fiscal 2023 (a 53-week year) into fiscal 2024, as JSON rows and "
        "as CSV. A second scheme 'retail' has a quarter template with a comma and double quotes, so the CSV must quote and "
        "double them. An empty range (start equal to end_exclusive) gives [] as JSON, and as CSV the header row alone "
        "(reading: the header row is always written)."
    )
    return vector("dimension-nrf", desc, oracle_source("fiscal_oracle.py; CSV quoting per RFC 4180 as specified"), defn, cases)


# 8. Labels ---------------------------------------------------------------


def ambiguous_label_vector() -> dict:
    defn = fixed_date(4, 1, "monday", naming({
        "fy": scheme("start", {"year": "{yyyy}-{yy_next}", "quarter": "Q{q} {yyyy}-{yy_next}"}),
        "ay": scheme("start", {"year": "{yyyy}-{yy_next}"}, offset=1),
    }, default="fy"))
    cal = Calendar(defn)
    cases = [validate_case(defn)]
    cases += label_cases(cal, [coords("year", 2025), coords("quarter", 2025, 1)], (None, "ay"))
    for label in ("2025-27", "Q5 2025-26", "FY2025", "2025-26 ", "2099-00"):
        cases.append(case(cal, "parse", {"label": label}))
    desc = (
        "Two schemes with the same year template: 'fy' (start year) and 'ay' (assessment year, offset 1). '2025-26' names "
        "fiscal 2025 under fy and fiscal 2024 under ay, so parse returns AMBIGUOUS_LABEL with both candidates ordered by start date "
        "then scheme name. A {yy_next} that disagrees with {yyyy}, an out-of-range quarter, and trailing text are LABEL_NOT_RECOGNISED. "
        "expect_error carries 'details' here; it is compared exactly."
    )
    return vector("labels-ambiguous-india", desc, oracle_source("fiscal_oracle.py parse"), defn, cases)


def calendar_id_vectors() -> list[dict]:
    a = {
        "spec_version": 1,
        "year": {"type": "fixed_date", "start": {"month": 1, "day": 1}},
        "week": {"starts_on": "monday", "numbering": "contains_start"},
        "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
        "naming": {"default": "cal", "schemes": {"cal": {"year_number": "start", "templates": {"year": "{yyyy}"}}}},
    }
    b = {
        "naming": {"schemes": {"cal": {"templates": {"year": "{yyyy}"}, "offset": 0, "year_number": "start"}}, "default": "cal"},
        "subdivision": {"quarters": [3, 3, 3, 3], "month_start_day": 1, "type": "calendar_months"},
        "week": {"numbering": "contains_start", "starts_on": "monday"},
        "$schema": "https://example.invalid/fiscalclock/definition-v1.json",
        "year": {"start": {"day": 1, "month": 1}, "type": "fixed_date"},
        "spec_version": 1,
    }
    ca, cb = validate_case(a), validate_case(b)
    assert ca["expect"] == cb["expect"]
    desc = "{} of a pair that differ only in key order, $schema and a written-out default (offset 0); both must have calendar_id {}. Whitespace differences cannot be expressed in a parsed JSON definition."
    cid = ca["expect"]["calendar_id"]
    return [
        vector("calendar-id-pair-a", desc.format("First", cid), oracle_source("hashlib.sha256 over the canonical form built in fiscal_oracle.canonical"), a, [ca]),
        vector("calendar-id-pair-b", desc.format("Second", cid), oracle_source("hashlib.sha256 over the canonical form built in fiscal_oracle.canonical"), b, [cb]),
    ]


CANONICAL_NON_ASCII = (
    '{"naming":{"default":"jp","schemes":{"jp":{"century_pivot":null,"offset":0,"templates":'
    '{"quarter":"{yyyy}年度 第{q}四半期","year":"{yyyy}年度"},"year_number":"start"},'
    '"tab":{"century_pivot":null,"offset":0,"templates":{"year":"FY\\t{yyyy}\\u001f\\"\\\\"},"year_number":"start"}}},'
    '"spec_version":1,"subdivision":{"month_start_day":1,"quarters":[3,3,3,3],"type":"calendar_months"},'
    '"week":{"numbering":"contains_start","starts_on":"monday"},'
    '"year":{"short_month":null,"start":{"day":1,"month":4},"type":"fixed_date"}}'
)


def canonical_non_ascii_vector() -> dict:
    defn = fixed_date(4, 1, "monday", naming({
        "jp": scheme("start", {"year": "{yyyy}年度", "quarter": "{yyyy}年度 第{q}四半期"}),
        "tab": scheme("start", {"year": 'FY\t{yyyy}\x1f"\\'}),
    }))
    c = validate_case(defn)
    assert c["expect"]["canonical"] == CANONICAL_NON_ASCII, c["expect"]["canonical"]
    assert c["expect"]["calendar_id"] == hashlib.sha256(CANONICAL_NON_ASCII.encode("utf-8")).hexdigest()[:16]
    cal = Calendar(defn)
    cases = [c, case(cal, "format", {"period": coords("year", 2024)}), case(cal, "format", {"period": coords("quarter", 2024, 2)}),
             case(cal, "parse", {"label": "2024年度 第2四半期"}), case(cal, "format", {"period": coords("year", 2024), "scheme": "tab"})]
    desc = (
        "Canonical form with non-ASCII and control characters. The Japanese templates ('年度', fiscal year) are written as raw "
        "UTF-8; the tab is written as \\t, U+001F as \\u001f with lower-case hex digits, and the quote and backslash as \\\" and \\\\. "
        "Keys are sorted by their UTF-8 bytes, offset 0 and null century_pivot and short_month are written out. The expected "
        "canonical string is typed into the generator by hand and calendar_id is SHA-256 of its UTF-8 bytes, first 16 hex digits."
    )
    return vector("canonical-non-ascii", desc, oracle_source("Canonical string typed by hand in oracle_vectors.CANONICAL_NON_ASCII; fiscal_oracle.canonical agrees (asserted)"), defn, cases)


def range_edge_vectors() -> list[dict]:
    out = []
    defs = [("nrf", nrf_definition()), ("jan-1", fixed_date(1, 1, "monday", naming({"cal": scheme("start", {"year": "{yyyy}"})}))), ("apple", apple_definition())]
    for label, defn in defs:
        cal = Calendar(defn)
        first = next(k for k in range(1, 10) if cal.in_range(k))
        last = next(k for k in range(10000, 9990, -1) if cal.in_range(k))
        cases = [validate_case(defn)]
        for k in (first, last):
            s, e = cal.key_bounds(k)
            cases.append(case(cal, "period", year_args(cal.number(k))))
            cases += locate_cases(cal, [iso(s), iso(e - 1)])
        cases.append(case(cal, "shift", {"period": coords("year", cal.number(first)), "n": cal.number(last) - cal.number(first)}))
        desc = "First and last fiscal years that lie wholly in 0001-01-02..9999-12-30, with locate on their first and last days."
        out.append(vector(f"range-edges-{label}", desc, oracle_source("fiscal_oracle.py with ordinal arithmetic beyond the datetime range"), defn, cases))
    return out


def build() -> list[dict]:
    vectors = week_based_spread() + [nrf_vector(), apple_vector(), iso_vector(), dimension_vector(), ambiguous_label_vector()]
    vectors += fixed_vectors() + calendar_id_vectors() + [canonical_non_ascii_vector()] + range_edge_vectors()
    return vectors
