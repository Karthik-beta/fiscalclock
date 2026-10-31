"""Error vectors: invalid definitions (expected codes read from the spec) and runtime errors."""

from __future__ import annotations

import copy
from datetime import date

from fiscal_oracle import Calendar, iso
from oracle_vectors import iso_definition, nrf_definition
from vectorlib import case, coords, fixed_date, naming, scheme, source, validate_case, vector, week_based, year_args

SPEC = "spec/definition-v1.md"
CHECKED = "Manual reading of spec/definition-v1.md and spec/api-v1.md, 2026-10-31"


def fixed_base() -> dict:
    return fixed_date(1, 1, "monday", naming({"fy": scheme("start", {"year": "FY{yyyy}"})}))


def edit(base: dict, fn) -> dict:
    d = copy.deepcopy(base)
    fn(d)
    return d


def setp(path: str, value):
    def fn(d):
        *head, last = path.split(".")
        for key in head:
            d = d[key]
        d[last] = value
    return fn


def delp(path: str):
    def fn(d):
        *head, last = path.split(".")
        for key in head:
            d = d[key]
        del d[last]
    return fn


def both(*fns):
    def fn(d):
        for f in fns:
            f(d)
    return fn


nrf = nrf_definition()
fixed = fixed_base()
calendar_months_in_week_based = {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]}
week_pattern_in_fixed = {"type": "week_pattern", "periods": [4, 5, 4] * 4, "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}}

# (name, definition, expected error(s), rule the expectation comes from)
INVALID = [
    ("invalid-json-not-object", [1, 2], {"code": "INVALID_JSON", "path": ""}, "api-v1 catalogue: INVALID_JSON when the definition is not valid JSON or not an object; path '' means the whole input"),
    ("unknown-field", edit(nrf, setp("week.color", "blue")), {"code": "UNKNOWN_FIELD", "path": "week.color"}, "Unknown fields at any level are rejected with UNKNOWN_FIELD"),
    ("unknown-template-level", edit(nrf, setp("naming.schemes.nrf.templates.month", "{yyyy}")), {"code": "UNKNOWN_FIELD", "path": "naming.schemes.nrf.templates.month"}, "templates: Unknown keys are UNKNOWN_FIELD"),
    ("missing-field", edit(nrf, delp("week")), {"code": "MISSING_FIELD", "path": "week"}, "A missing required field is rejected with MISSING_FIELD"),
    ("invalid-type", edit(nrf, setp("subdivision.extra_week.period", "12")), {"code": "INVALID_TYPE", "path": "subdivision.extra_week.period"}, "A value of the wrong JSON type is rejected with INVALID_TYPE"),
    ("invalid-value-anchor-day-29", edit(nrf, setp("year.anchor.day", 29)), {"code": "INVALID_VALUE", "path": "year.anchor.day"}, "anchor.day above 28 is rejected with INVALID_VALUE"),
    ("invalid-value-anchor-month-13", edit(nrf, setp("year.anchor.month", 13)), {"code": "INVALID_VALUE", "path": "year.anchor.month"}, "Month values are integers 1-12"),
    ("invalid-value-weekday-case", edit(nrf, setp("week.starts_on", "Sunday")), {"code": "INVALID_VALUE", "path": "week.starts_on"}, "Weekday values are the lower-case English names"),
    ("invalid-value-short-month-unused", edit(fixed, setp("year.short_month", "clamp")), {"code": "INVALID_VALUE", "path": "year.short_month"}, "short_month when start.day <= 28 is rejected with INVALID_VALUE"),
    ("invalid-value-periods-length", edit(nrf, both(setp("subdivision.periods", [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 9]), setp("subdivision.quarters", [3, 3, 3, 2]), setp("subdivision.extra_week.period", 11))), {"code": "INVALID_VALUE", "path": "subdivision.periods"}, "periods: Any other length (than 12 or 13) is INVALID_VALUE"),
    ("invalid-value-scheme-name", edit(fixed, setp("naming", {"default": "FY", "schemes": {"FY": {"year_number": "start", "templates": {"year": "FY{yyyy}"}}}})), {"code": "INVALID_VALUE", "path": "naming.schemes.FY"}, "Scheme names match ^[a-z][a-z0-9_]{0,31}$, otherwise INVALID_VALUE"),
    ("invalid-value-empty-schemes", edit(fixed, setp("naming.schemes", {})), [{"code": "DEFAULT_SCHEME_MISSING", "path": "naming.default"}, {"code": "INVALID_VALUE", "path": "naming.schemes"}], "schemes must not be empty (INVALID_VALUE) and default must be a key of schemes (DEFAULT_SCHEME_MISSING); validation reports every error sorted by path then code"),
    ("invalid-value-century-pivot", edit(fixed, setp("naming.schemes.fy.century_pivot", 99)), {"code": "INVALID_VALUE", "path": "naming.schemes.fy.century_pivot"}, "century_pivot is an integer 100-9999"),
    ("unsupported-spec-version", edit(nrf, setp("spec_version", 2)), {"code": "UNSUPPORTED_SPEC_VERSION", "path": "spec_version"}, "A higher value is rejected with UNSUPPORTED_SPEC_VERSION"),
    ("unsupported-feature-first-full", edit(nrf, setp("week.numbering", "first_full")), {"code": "UNSUPPORTED_FEATURE", "path": "week.numbering"}, "first_full is reserved and rejected with UNSUPPORTED_FEATURE"),
    ("unsupported-feature-exceptions", edit(nrf, setp("exceptions", {})), {"code": "UNSUPPORTED_FEATURE", "path": "exceptions"}, "exceptions: Any value is rejected with UNSUPPORTED_FEATURE"),
    ("unsupported-feature-year-number-table", edit(nrf, setp("naming.schemes.nrf.year_number", "table")), {"code": "UNSUPPORTED_FEATURE", "path": "naming.schemes.nrf.year_number"}, "year_number table is reserved (UNSUPPORTED_FEATURE)"),
    ("unsupported-feature-eras", edit(nrf, setp("naming.schemes.nrf.eras", [])), {"code": "UNSUPPORTED_FEATURE", "path": "naming.schemes.nrf.eras"}, "eras: Reserved (UNSUPPORTED_FEATURE)"),
    ("unsupported-feature-standalone", edit(nrf, setp("subdivision.extra_week.standalone", True)), {"code": "UNSUPPORTED_FEATURE", "path": "subdivision.extra_week.standalone"}, "extra_week.standalone is reserved for 0.2"),
    ("unsupported-combination-calendar-months", edit(nrf, setp("subdivision", calendar_months_in_week_based)), {"code": "UNSUPPORTED_COMBINATION", "path": "subdivision.type"}, "calendar_months inside a week_based year is rejected with UNSUPPORTED_COMBINATION"),
    ("unsupported-combination-week-pattern", edit(fixed, setp("subdivision", week_pattern_in_fixed)), {"code": "UNSUPPORTED_COMBINATION", "path": "subdivision.type"}, "week_pattern inside a fixed_date year is rejected with UNSUPPORTED_COMBINATION"),
    ("weeks-sum-mismatch", edit(nrf, setp("subdivision.periods", [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 4, 4])), {"code": "WEEKS_SUM_MISMATCH", "path": "subdivision.periods"}, "periods must sum to 52, otherwise WEEKS_SUM_MISMATCH"),
    ("quarters-sum-mismatch-week-pattern", edit(nrf, setp("subdivision.quarters", [3, 3, 3, 4])), {"code": "QUARTERS_SUM_MISMATCH", "path": "subdivision.quarters"}, "quarters must sum to the length of periods, otherwise QUARTERS_SUM_MISMATCH"),
    ("quarters-sum-mismatch-calendar-months", edit(fixed, setp("subdivision.quarters", [4, 4, 3])), {"code": "QUARTERS_SUM_MISMATCH", "path": "subdivision.quarters"}, "calendar_months quarters must sum to 12, otherwise QUARTERS_SUM_MISMATCH"),
    ("extra-week-missing", edit(nrf, delp("subdivision.extra_week")), {"code": "EXTRA_WEEK_MISSING", "path": "subdivision.extra_week"}, "extra_week missing is EXTRA_WEEK_MISSING"),
    ("extra-week-out-of-range-high", edit(nrf, setp("subdivision.extra_week.period", 13)), {"code": "EXTRA_WEEK_OUT_OF_RANGE", "path": "subdivision.extra_week.period"}, "extra_week.period outside 1..length of periods is EXTRA_WEEK_OUT_OF_RANGE"),
    ("extra-week-out-of-range-zero", edit(nrf, setp("subdivision.extra_week.period", 0)), {"code": "EXTRA_WEEK_OUT_OF_RANGE", "path": "subdivision.extra_week.period"}, "extra_week.period outside 1..length of periods is EXTRA_WEEK_OUT_OF_RANGE"),
    ("short-month-policy-missing", edit(fixed, both(setp("year.start.day", 31), setp("subdivision.month_start_day", 31))), {"code": "SHORT_MONTH_POLICY_MISSING", "path": "year.short_month"}, "start.day >= 29 without short_month is rejected with SHORT_MONTH_POLICY_MISSING"),
    ("subdivision-year-mismatch", edit(fixed, setp("subdivision.month_start_day", 6)), {"code": "SUBDIVISION_YEAR_MISMATCH", "path": "subdivision.month_start_day"}, "month_start_day must equal year.start.day, otherwise SUBDIVISION_YEAR_MISMATCH"),
    ("default-scheme-missing", edit(fixed, setp("naming.default", "ay")), {"code": "DEFAULT_SCHEME_MISSING", "path": "naming.default"}, "default must be a key of schemes, otherwise DEFAULT_SCHEME_MISSING"),
    ("invalid-template-no-year", edit(fixed, setp("naming.schemes.fy.templates.year", "FY")), {"code": "INVALID_TEMPLATE", "path": "naming.schemes.fy.templates.year"}, "Every template must contain {yyyy} or {yy}, otherwise INVALID_TEMPLATE"),
    ("invalid-template-yy-next-alone", edit(fixed, setp("naming.schemes.fy.templates.year", "FY{yy_next}")), {"code": "INVALID_TEMPLATE", "path": "naming.schemes.fy.templates.year"}, "Every template must contain {yyyy} or {yy}; {yy_next} alone is not enough, otherwise INVALID_TEMPLATE"),
    ("invalid-template-unknown-placeholder", edit(fixed, setp("naming.schemes.fy.templates.year", "FY{yyyy}{mm}")), {"code": "INVALID_TEMPLATE", "path": "naming.schemes.fy.templates.year"}, "An unknown placeholder is INVALID_TEMPLATE"),
    ("invalid-template-unbalanced", edit(fixed, setp("naming.schemes.fy.templates.year", "FY{yyyy")), {"code": "INVALID_TEMPLATE", "path": "naming.schemes.fy.templates.year"}, "An unbalanced brace is INVALID_TEMPLATE"),
    ("invalid-template-misplaced", edit(fixed, setp("naming.schemes.fy.templates.period", "FY{yyyy} Q{q}")), {"code": "INVALID_TEMPLATE", "path": "naming.schemes.fy.templates.period"}, "A placeholder used outside the levels shown ({q} only in quarter) is INVALID_TEMPLATE"),
    ("century-pivot-missing", edit(fixed, setp("naming.schemes.fy.templates.year", "FY{yy}")), {"code": "CENTURY_PIVOT_MISSING", "path": "naming.schemes.fy.century_pivot"}, "A template that contains {yy} but not {yyyy} needs century_pivot, otherwise CENTURY_PIVOT_MISSING"),
    ("century-pivot-missing-yy-next", edit(fixed, setp("naming.schemes.fy.templates.year", "FY{yy}-{yy_next}")), {"code": "CENTURY_PIVOT_MISSING", "path": "naming.schemes.fy.century_pivot"}, "A template that contains {yy} but not {yyyy} needs century_pivot, otherwise CENTURY_PIVOT_MISSING"),
]


def invalid_definition_vectors() -> list[dict]:
    out = []
    for name, defn, expected, rule in INVALID:
        desc = f"Invalid definition. Rule: {rule}. Only code and path are asserted."
        src = source("errors", f"{SPEC} / spec/api-v1.md error catalogue", CHECKED)
        out.append(vector(f"definition-{name}", desc, src, defn, [{"op": "validate", "args": {}, "expect_error": expected}]))
    return out


def naming_error_vectors() -> list[dict]:
    """YEAR_NUMBER_COLLISION and MAJORITY_TIE: the code comes from the oracle's scan of the supported range, the path from api-v1.md."""
    iso_2018, iso_2019 = date.fromisocalendar(2018, 1, 1), date.fromisocalendar(2019, 1, 1)
    assert (iso_2018, iso_2019) == (date(2018, 1, 1), date(2018, 12, 31))
    assert (date.fromisocalendar(2017, 1, 1), date.fromisocalendar(2018, 1, 1)) == (date(2017, 1, 2), date(2018, 1, 1))
    rows = [
        ("year-number-collision-iso-start", iso_definition("start"), "YEAR_NUMBER_COLLISION", "naming.schemes.iso.year_number",
         "ISO 8601 week years numbered by 'start'. date.fromisocalendar() gives ISO 2018 starting 2018-01-01 and ISO 2019 starting "
         "2018-12-31, so both get the number 2018 (the spec's own example)."),
        ("year-number-collision-iso-end", iso_definition("end"), "YEAR_NUMBER_COLLISION", "naming.schemes.iso.year_number",
         "ISO 8601 week years numbered by 'end'. ISO 2016 ends 2017-01-01 and ISO 2017 ends 2017-12-31, so both get the number 2017."),
        ("majority-tie-fixed-jul-2", fixed_date(7, 2, "monday", naming({"fy": scheme("majority", {"year": "FY{yyyy}"})})), "MAJORITY_TIE", "naming.schemes.fy.year_number",
         "Fixed-date year starting 2 July, numbered by 'majority'. A year that contains 29 February has 366 days, 183 from 2 July to "
         "31 December and 183 after, so no calendar year holds more than half."),
        ("majority-tie-week-based-jul-1", week_based("start", 7, 1, "nearest", "sunday", [4] * 13, [3, 3, 3, 4], 13, naming({"fy": scheme("majority", {"year": "FY{yyyy}"})})), "MAJORITY_TIE", "naming.schemes.fy.year_number",
         "Week-based year starting on the Sunday nearest 1 July, numbered by 'majority'. A 364-day year that starts on 3 July has "
         "182 days in each calendar year."),
    ]
    src = source("errors", "spec/definition-v1.md, Year numbers must be unique; path from spec/api-v1.md, Paths for common errors",
                 "conformance/generators/fiscal_oracle.py Calendar.naming_errors scans every fiscal year in the supported range; code and path asserted by the generator")
    out = []
    for name, defn, code, path, why in rows:
        c = case(Calendar(defn), "validate", {})
        assert c.get("expect_error") == {"code": code, "path": path}, (name, c)
        desc = f"Invalid definition. {why} Only code and path are asserted; details.years is not, because the spec only says it lists the first affected years."
        out.append(vector(f"definition-{name}", desc, src, defn, [c]))
    return out


def runtime(cal: Calendar, op: str, args: dict, code: str) -> dict:
    c = case(cal, op, args)
    assert "expect_error" in c and c["expect_error"]["code"] == code, (op, args, c)
    return c


def runtime_vectors() -> list[dict]:
    src = source("errors", "spec/api-v1.md error catalogue and conventions; expected values computed by conformance/generators/fiscal_oracle.py", CHECKED)
    out = []

    cal = Calendar(nrf)
    cases = [validate_case(nrf)]
    for d in ("2023-02-29", "2023-2-01", "2023-01-05T00:00:00", "20230105", "0000-06-15", "2023-13-01", " 2023-01-05", "2023-01-05Z"):
        cases.append(runtime(cal, "locate", {"date": d}, "INVALID_DATE"))
    cases += [
        runtime(cal, "period", {"level": "quarter", "year": 2023, "index": 5}, "PERIOD_NOT_FOUND"),
        runtime(cal, "period", {"level": "period", "year": 2023, "index": 13}, "PERIOD_NOT_FOUND"),
        runtime(cal, "period", {"level": "week", "year": 2024, "index": 53}, "PERIOD_NOT_FOUND"),
        runtime(cal, "period", {"level": "week", "year": 2024, "index": 0}, "PERIOD_NOT_FOUND"),
        runtime(cal, "shift", {"period": coords("week", 2024, 53), "n": 1}, "PERIOD_NOT_FOUND"),
        runtime(cal, "period", {"level": "month", "year": 2023, "index": 1}, "INVALID_ARGUMENT"),
        runtime(cal, "parent", {"period": coords("year", 2023)}, "INVALID_ARGUMENT"),
        runtime(cal, "shift", {"period": coords("week", 2023, 1), "n": 1.5}, "INVALID_ARGUMENT"),
        runtime(cal, "children", {"period": coords("year", 2023), "level": "day"}, "INVALID_ARGUMENT"),
        runtime(cal, "format", {"period": coords("year", 2023), "scheme": "nope"}, "SCHEME_NOT_FOUND"),
        runtime(cal, "parse", {"label": "FY2023 X1"}, "LABEL_NOT_RECOGNISED"),
        runtime(cal, "parse", {"label": ""}, "LABEL_NOT_RECOGNISED"),
        runtime(cal, "dimension", {"start": "2024-13-01", "end_exclusive": "2024-02-07"}, "INVALID_DATE"),
        runtime(cal, "dimension", {"start": "2024-02-07", "end_exclusive": "2024-02-06"}, "INVALID_ARGUMENT"),
        runtime(cal, "dimension", {"start": "2000-01-01", "end_exclusive": iso(date(2000, 1, 1).toordinal() + 146_098)}, "INVALID_ARGUMENT"),
        runtime(cal, "dimension_csv", {"start": "2024-02-07", "end_exclusive": "2024-02-06"}, "INVALID_ARGUMENT"),
    ]
    desc = (
        "Runtime errors on the NRF 4-5-4 definition: malformed dates (including year 0000, a time suffix and a zone suffix), missing periods, "
        "bad arguments, unknown scheme, unrecognised labels, and dimension ranges with start after end_exclusive or longer than 146,097 days "
        "(INVALID_ARGUMENT; the spec does not pin which argument the path names, read here as end_exclusive)."
    )
    out.append(vector("runtime-nrf", desc, src, nrf, cases))

    jan = fixed_date(1, 1, "monday", naming({"cal": scheme("start", {"year": "{yyyy}", "quarter": "{yyyy}-Q{q}"}), "be": scheme("start", {"year": "BE {yyyy}"}, offset=543)}))
    cal = Calendar(jan)
    cases = [validate_case(jan)]
    cases += [
        runtime(cal, "children", {"period": coords("quarter", 2024, 1), "level": "week"}, "LEVEL_NOT_NESTED"),
        runtime(cal, "children", {"period": coords("period", 2024, 1), "level": "week"}, "LEVEL_NOT_NESTED"),
        runtime(cal, "format", {"period": coords("week", 2024, 1)}, "TEMPLATE_MISSING"),
        runtime(cal, "format", {"period": coords("quarter", 2024, 1), "scheme": "be"}, "TEMPLATE_MISSING"),
        runtime(cal, "format", {"period": coords("year", 9500), "scheme": "be"}, "OUT_OF_RANGE"),
        runtime(cal, "locate", {"date": "0001-01-01"}, "OUT_OF_RANGE"),
        runtime(cal, "locate", {"date": "0001-06-01"}, "OUT_OF_RANGE"),
        runtime(cal, "locate", {"date": "0001-12-31"}, "OUT_OF_RANGE"),
        runtime(cal, "locate", {"date": "9999-01-01"}, "OUT_OF_RANGE"),
        runtime(cal, "locate", {"date": "9999-12-31"}, "OUT_OF_RANGE"),
        runtime(cal, "period", year_args(1), "OUT_OF_RANGE"),
        runtime(cal, "period", year_args(9999), "OUT_OF_RANGE"),
        runtime(cal, "period", year_args(12000), "OUT_OF_RANGE"),
        runtime(cal, "shift", {"period": coords("year", 9998), "n": 1}, "OUT_OF_RANGE"),
        runtime(cal, "shift", {"period": coords("quarter", 2, 1), "n": -1}, "OUT_OF_RANGE"),
    ]
    desc = (
        "Runtime errors on a 1 January fixed-date calendar. Fiscal year 1 starts 0001-01-01 and fiscal year 9999 ends 9999-12-31, "
        "so neither lies wholly in 0001-01-02..9999-12-30 and both are OUT_OF_RANGE. Formatting {yyyy} for a year number above 9999 "
        "after offset (9500 + 543) is OUT_OF_RANGE. Weeks do not nest in quarters or periods of a fixed-date year."
    )
    out.append(vector("runtime-fixed-jan-1", desc, src, jan, cases))

    cal = Calendar(nrf)
    first = next(k for k in range(1, 10) if cal.in_range(k))
    last = next(k for k in range(10000, 9990, -1) if cal.in_range(k))
    s_first, _ = cal.key_bounds(first)
    _, e_last = cal.key_bounds(last)
    cases = [validate_case(nrf)]
    cases += [
        runtime(cal, "locate", {"date": iso(s_first - 1)}, "OUT_OF_RANGE"),
        runtime(cal, "locate", {"date": iso(e_last)}, "OUT_OF_RANGE"),
        runtime(cal, "period", year_args(cal.number(first) - 1), "OUT_OF_RANGE"),
        runtime(cal, "period", year_args(cal.number(last) + 1), "OUT_OF_RANGE"),
        runtime(cal, "shift", {"period": coords("week", cal.number(last), 1), "n": 60}, "OUT_OF_RANGE"),
        runtime(cal, "dimension", {"start": iso(s_first - 1), "end_exclusive": iso(s_first + 1)}, "OUT_OF_RANGE"),
        runtime(cal, "dimension", {"start": iso(e_last - 1), "end_exclusive": iso(e_last + 1)}, "OUT_OF_RANGE"),
        runtime(cal, "dimension", {"start": iso(e_last), "end_exclusive": iso(e_last + 1)}, "OUT_OF_RANGE"),
    ]
    assert [c["expect_error"]["path"] for c in cases[-3:]] == ["start", "end_exclusive", "start"]
    desc = (
        f"The days just outside the first ({cal.number(first)}) and last ({cal.number(last)}) NRF fiscal years that lie wholly in the supported range. "
        "dimension reports OUT_OF_RANGE at start when the fiscal year of start is out of range, otherwise at end_exclusive."
    )
    out.append(vector("runtime-out-of-range-nrf", desc, src, nrf, cases))
    return out


def build() -> list[dict]:
    return invalid_definition_vectors() + naming_error_vectors() + runtime_vectors()
