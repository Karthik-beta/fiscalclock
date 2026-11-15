"""Error vectors: invalid definitions (expected codes read from the spec) and runtime errors."""

from __future__ import annotations

import copy
from datetime import date

import history_vectors as hv
from fiscal_oracle import Calendar, iso
from oracle_vectors import iso_definition, nrf_definition
from vectorlib import case, coords, fixed_date, naming, scheme, source, validate_case, vector, week_based, year_args

SPEC = "spec/definition-v1.md"
SPEC_BASE = "https://github.com/Karthik-beta/fiscalclock/blob/main/spec/"
DEFINITION_URL = SPEC_BASE + "definition-v1.md"
CATALOGUE_URL = SPEC_BASE + "api-v1.md#error-catalogue"
YEAR_NUMBERS_URL = SPEC_BASE + "definition-v1.md#year-numbers-must-be-unique"
CHECKED = "Manual reading of spec/definition-v1.md and spec/api-v1.md, 2026-10-31"
RUNTIME_CHECKED = (
    "Manual reading of spec/api-v1.md, 2026-10-31; conformance/generators/fiscal_oracle.py computes each case "
    "and the generator asserts that its error code equals the one read from the spec"
)


def fixed_base() -> dict:
    return fixed_date(1, 1, "monday", naming({"fy": scheme("start", {"year": "FY{yyyy}"})}))


def edit(base: dict, fn) -> dict:
    d = copy.deepcopy(base)
    fn(d)
    return d


def _key(container, key: str):
    """A dotted-path segment: a list index when the container is a list, otherwise an object key."""
    return int(key) if isinstance(container, list) else key


def setp(path: str, value):
    def fn(d):
        *head, last = path.split(".")
        for key in head:
            d = d[_key(d, key)]
        d[_key(d, last)] = value
    return fn


def delp(path: str):
    def fn(d):
        *head, last = path.split(".")
        for key in head:
            d = d[_key(d, key)]
        del d[_key(d, last)]
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

# (name, definition, expected error(s), paraphrase of the spec rule the expectation comes from)
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


# Rules that only the API catalogue states; the definition spec does not mention them.
CATALOGUE_ONLY = {"invalid-json-not-object"}


def invalid_definition_vectors() -> list[dict]:
    out = []
    for name, defn, expected, rule in INVALID:
        desc = f"Invalid definition. Spec rule, paraphrased: {rule}. Only code and path are asserted."
        if name in CATALOGUE_ONLY:
            src = source("errors", "spec/api-v1.md error catalogue", CHECKED, CATALOGUE_URL)
        else:
            src = source("errors", f"{SPEC} / spec/api-v1.md error catalogue", CHECKED, DEFINITION_URL)
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
                 "conformance/generators/fiscal_oracle.py Calendar.naming_errors scans every fiscal year in the supported range; code and path asserted by the generator",
                 YEAR_NUMBERS_URL)
    out = []
    for name, defn, code, path, why in rows:
        c = case(Calendar(defn), "validate", {})
        # These files predate the spec's rule for details.years (0.2.0) and keep asserting code and path only.
        c["expect_error"].pop("details", None)
        assert c.get("expect_error") == {"code": code, "path": path}, (name, c)
        desc = f"Invalid definition. {why} Only code and path are asserted; details.years is not, because the spec only says it lists the first affected years."
        out.append(vector(f"definition-{name}", desc, src, defn, [c]))
    return out


def runtime(cal: Calendar, op: str, args: dict, code: str) -> dict:
    c = case(cal, op, args)
    assert "expect_error" in c and c["expect_error"]["code"] == code, (op, args, c)
    return c


def runtime_vectors() -> list[dict]:
    src = source("errors", "spec/api-v1.md error catalogue and conventions; expected values computed by conformance/generators/fiscal_oracle.py", RUNTIME_CHECKED, CATALOGUE_URL)
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
        "(INVALID_ARGUMENT at end_exclusive, from the api-v1.md table \"Paths for common errors\")."
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
    first, last = 0, cal.last_key()
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


# 0.2.0: exceptions.history ---------------------------------------------------------------------------------

HISTORY_SPEC = "spec/definition-v1.md, exceptions (fiscalclock 0.2.0, ADR-0032), and spec/api-v1.md, Paths for common errors"
HISTORY_CHECKED = (
    "Manual reading of the 0.2.0 text of " + SPEC_BASE + "definition-v1.md#exceptions and " + SPEC_BASE + "api-v1.md#error "
    "(Paths for common errors), 2026-11-15"
)
CROSS_CHECKED = HISTORY_CHECKED + "; conformance/generators/fiscal_oracle.py Calendar.change_errors gives the same codes, paths and order (asserted)"
H0, H1 = "exceptions.history[0]", "exceptions.history[1]"
T0 = H0 + ".transition"
TS0 = T0 + ".subdivision"
WEEK_BASED_DECEMBER = {"type": "week_based", "anchor": {"edge": "end", "month": 12, "day": "last", "rule": "last"}}
PATTERN_4_4_5 = {"type": "week_pattern", "periods": [4, 4, 5] * 4, "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}}


def hset(path: str, value):
    """setp on history entry 0: hset("transition.start", ...) sets exceptions.history[0].transition.start."""
    return setp("exceptions.history.0." + path, value)


def hdel(path: str):
    return delp("exceptions.history.0." + path)


def jul_to_apr() -> dict:
    return hv.jul_to_apr_definition()


def week_pattern_transition() -> dict:
    return hv.nrf_to_december_definition()


def jan_to(frm: str) -> dict:
    """1 January years changing to 1 April years on frm, without a transition."""
    return hv.with_history(fixed_base(), hv.entry(frm, *hv.fixed_rule(4, 1)))


def jul_to_apr_then(second: dict) -> dict:
    d = jul_to_apr()
    d["exceptions"]["history"].append(second)
    return d


def named(year_number: str) -> dict:
    return naming({"fy": scheme(year_number, {"year": "FY{yyyy}"})})


# (name, definition, expected error(s), section anchor in definition-v1.md, paraphrase of the spec rule)
HISTORY_BLOCK = [
    ("exceptions-empty-object", edit(nrf, setp("exceptions", {})), {"code": "MISSING_FIELD", "path": "exceptions.history"}, "exceptions", "history is required inside exceptions"),
    ("exceptions-history-empty", edit(jul_to_apr(), setp("exceptions.history", [])), {"code": "INVALID_VALUE", "path": "exceptions.history"}, "exceptions", "An empty history array is INVALID_VALUE at exceptions.history"),
    ("exceptions-history-null", edit(jul_to_apr(), setp("exceptions.history", None)), {"code": "INVALID_TYPE", "path": "exceptions.history"}, "exceptions", "history is an array of history entries, so null is the wrong type"),
    ("exceptions-not-object", edit(nrf, setp("exceptions", [])), {"code": "INVALID_TYPE", "path": "exceptions"}, "top-level", "exceptions is an object or null"),
    ("exceptions-overrides", edit(jul_to_apr(), setp("exceptions.overrides", [])), {"code": "UNSUPPORTED_FEATURE", "path": "exceptions.overrides"}, "exceptions", "overrides is reserved: any value is UNSUPPORTED_FEATURE"),
    ("exceptions-overrides-null", edit(jul_to_apr(), setp("exceptions.overrides", None)), {"code": "UNSUPPORTED_FEATURE", "path": "exceptions.overrides"}, "exceptions", "overrides is reserved: any value, null included, is UNSUPPORTED_FEATURE"),
    ("exceptions-transitions-unknown", edit(jul_to_apr(), setp("exceptions.transitions", [])), {"code": "UNKNOWN_FIELD", "path": "exceptions.transitions"}, "exceptions", "exceptions has the fields history and overrides; transitions is an unknown field"),
    ("history-missing-from", edit(jul_to_apr(), hdel("from")), {"code": "MISSING_FIELD", "path": H0 + ".from"}, "history-entries", "from is required in a history entry"),
    ("history-missing-year", edit(jul_to_apr(), hdel("year")), {"code": "MISSING_FIELD", "path": H0 + ".year"}, "history-entries", "year is required in a history entry"),
    ("history-missing-subdivision", edit(jul_to_apr(), hdel("subdivision")), {"code": "MISSING_FIELD", "path": H0 + ".subdivision"}, "history-entries", "subdivision is required in a history entry"),
    ("history-unknown-field", edit(jul_to_apr(), hset("note", "x")), {"code": "UNKNOWN_FIELD", "path": H0 + ".note"}, "history-entries", "Each entry has exactly the fields from, year, subdivision and transition"),
    ("history-week-field", edit(jul_to_apr(), hset("week", {"starts_on": "monday", "numbering": "contains_start"})), {"code": "UNKNOWN_FIELD", "path": H0 + ".week"}, "history-entries", "week is global: a history entry has no week field (UNKNOWN_FIELD)"),
    ("history-naming-field", edit(jul_to_apr(), hset("naming", named("start"))), {"code": "UNKNOWN_FIELD", "path": H0 + ".naming"}, "history-entries", "naming is global: a history entry has no naming field (UNKNOWN_FIELD)"),
    ("history-from-not-string", edit(jul_to_apr(), hset("from", 20250401)), {"code": "INVALID_TYPE", "path": H0 + ".from"}, "history-entries", "A date in exceptions that is not a string is INVALID_TYPE"),
    ("history-from-not-a-date", edit(jul_to_apr(), hset("from", "2025-02-29")), {"code": "INVALID_VALUE", "path": H0 + ".from"}, "history-entries", "A string that is not a valid YYYY-MM-DD date is INVALID_VALUE; INVALID_DATE is only for operation arguments"),
    ("history-from-not-iso", edit(jul_to_apr(), hset("from", "2025-4-1")), {"code": "INVALID_VALUE", "path": H0 + ".from"}, "history-entries", "A date must be written YYYY-MM-DD; any other string is INVALID_VALUE"),
    ("history-from-before-range", edit(jul_to_apr(), both(hset("from", "0001-12-31"), hset("transition", None))), {"code": "INVALID_VALUE", "path": H0 + ".from"}, "history-entries", "A date before 0002-01-01 is INVALID_VALUE"),
    ("history-from-after-range", edit(jul_to_apr(), hset("from", "9999-01-01")), {"code": "INVALID_VALUE", "path": H0 + ".from"}, "history-entries", "A date after 9998-12-31 is INVALID_VALUE"),
    ("history-year-reserved", edit(jul_to_apr(), hset("year", {"type": "table"})), {"code": "UNSUPPORTED_FEATURE", "path": H0 + ".year.type"}, "history-entries", "An entry's year is validated as the top-level year, so year.type table is reserved (UNSUPPORTED_FEATURE) at the prefixed path"),
    ("history-subdivision-stub", edit(jul_to_apr(), hset("subdivision.stub", {})), {"code": "UNSUPPORTED_FEATURE", "path": H0 + ".subdivision.stub"}, "reserved-for-later-releases", "A reserved field inside a history entry is rejected at the prefixed path, for example exceptions.history[0].subdivision.stub"),
    ("history-unsupported-combination", edit(jul_to_apr(), hset("subdivision", copy.deepcopy(week_pattern_in_fixed))), {"code": "UNSUPPORTED_COMBINATION", "path": H0 + ".subdivision.type"}, "history-entries", "week_pattern inside an entry's fixed_date year is UNSUPPORTED_COMBINATION at the prefixed path"),
    ("history-subdivision-year-mismatch", edit(jul_to_apr(), hset("subdivision.month_start_day", 2)), {"code": "SUBDIVISION_YEAR_MISMATCH", "path": H0 + ".subdivision.month_start_day"}, "history-entries", "An entry's month_start_day must equal the entry's own year.start.day"),
    ("history-short-month-policy-missing", edit(jul_to_apr(), both(hset("year.start.day", 30), hset("subdivision.month_start_day", 30))), {"code": "SHORT_MONTH_POLICY_MISSING", "path": H0 + ".year.short_month"}, "history-entries", "An entry's fixed_date year with start.day 29-31 needs short_month"),
    ("history-extra-week-missing", edit(week_pattern_transition(), hdel("subdivision.extra_week")), {"code": "EXTRA_WEEK_MISSING", "path": H0 + ".subdivision.extra_week"}, "history-entries", "An entry's week_pattern needs extra_week, as at the top level; missing is EXTRA_WEEK_MISSING at the prefixed path"),
    ("history-extra-week-out-of-range", edit(week_pattern_transition(), hset("subdivision.extra_week.period", 13)), {"code": "EXTRA_WEEK_OUT_OF_RANGE", "path": H0 + ".subdivision.extra_week.period"}, "history-entries", "An entry's extra_week.period outside 1 to the length of its periods (12) is EXTRA_WEEK_OUT_OF_RANGE at the prefixed path"),
    ("history-weeks-sum-mismatch", edit(week_pattern_transition(), hset("subdivision.periods", [4, 4, 5] * 3 + [4, 4, 4])), {"code": "WEEKS_SUM_MISMATCH", "path": H0 + ".subdivision.periods"}, "history-entries", "An entry's week_pattern periods must sum to 52"),
    ("transition-start-equals-from", edit(jul_to_apr(), hset("transition.start", "2025-04-01")), {"code": "INVALID_VALUE", "path": T0 + ".start"}, "transitions", "transition.start on or after from is INVALID_VALUE at transition.start, a block-level check"),
    ("transition-start-after-from", edit(jul_to_apr(), hset("transition.start", "2025-07-01")), {"code": "INVALID_VALUE", "path": T0 + ".start"}, "transitions", "transition.start on or after from is INVALID_VALUE at transition.start, a block-level check"),
    ("transition-start-before-range", edit(jul_to_apr(), hset("transition.start", "0001-07-01")), {"code": "INVALID_VALUE", "path": T0 + ".start"}, "history-entries", "A date before 0002-01-01 is INVALID_VALUE"),
    ("transition-start-not-string", edit(jul_to_apr(), hset("transition.start", None)), {"code": "INVALID_TYPE", "path": T0 + ".start"}, "history-entries", "A date in exceptions that is not a string is INVALID_TYPE"),
    ("transition-missing-start", edit(jul_to_apr(), hdel("transition.start")), {"code": "MISSING_FIELD", "path": T0 + ".start"}, "transitions", "start is required in a transition"),
    ("transition-missing-subdivision", edit(jul_to_apr(), hdel("transition.subdivision")), {"code": "MISSING_FIELD", "path": TS0}, "transitions", "subdivision is required in a transition"),
    ("transition-unknown-field", edit(jul_to_apr(), hset("transition.year", {"type": "fixed_date", "start": {"month": 7, "day": 1}})), {"code": "UNKNOWN_FIELD", "path": T0 + ".year"}, "transitions", "A transition has the fields start and subdivision; it has no year block"),
    ("transition-week-pattern-extra-week", edit(week_pattern_transition(), hset("transition.subdivision.extra_week", {"period": 11})), {"code": "UNKNOWN_FIELD", "path": TS0 + ".extra_week"}, "transitions", "A transition week_pattern has no extra_week; the field is UNKNOWN_FIELD there"),
    ("transition-week-pattern-periods-empty", edit(week_pattern_transition(), hset("transition.subdivision.periods", [])), {"code": "INVALID_VALUE", "path": TS0 + ".periods"}, "transitions", "An empty transition periods array is INVALID_VALUE (block-level); quarters are then not compared with it"),
    ("transition-week-pattern-quarters-sum", edit(week_pattern_transition(), hset("transition.subdivision.quarters", [3, 3, 3, 3])), {"code": "QUARTERS_SUM_MISMATCH", "path": TS0 + ".quarters"}, "transitions", "A transition week_pattern's quarters must sum to the length of its periods (block-level)"),
    ("transition-month-start-day-29", edit(jul_to_apr(), hset("transition.subdivision.month_start_day", 29)), {"code": "INVALID_VALUE", "path": TS0 + ".month_start_day"}, "transitions", "A transition's month_start_day is 1-28; outside is INVALID_VALUE"),
    ("transition-subdivision-stub", edit(jul_to_apr(), hset("transition.subdivision.stub", {})), {"code": "UNSUPPORTED_FEATURE", "path": TS0 + ".stub"}, "transitions", "The field stub is reserved in a transition subdivision, as at the top level"),
    ("transition-subdivision-special-periods", edit(jul_to_apr(), hset("transition.subdivision.special_periods", [])), {"code": "UNSUPPORTED_FEATURE", "path": TS0 + ".special_periods"}, "transitions", "The field special_periods is reserved in a transition subdivision, as at the top level"),
    ("transition-type-anchored", edit(jul_to_apr(), hset("transition.subdivision.type", "anchored")), {"code": "UNSUPPORTED_FEATURE", "path": TS0 + ".type"}, "transitions", "Transition subdivision type anchored is reserved (UNSUPPORTED_FEATURE), as at the top level"),
    ("transition-type-table", edit(jul_to_apr(), hset("transition.subdivision.type", "table")), {"code": "INVALID_VALUE", "path": TS0 + ".type"}, "transitions", "Any other unknown transition subdivision type, including table, is INVALID_VALUE"),
    ("invalid-template-yy-end-alone", edit(fixed, setp("naming.schemes.fy.templates.year", "FY{yy_end}")), {"code": "INVALID_TEMPLATE", "path": "naming.schemes.fy.templates.year"}, "templates", "Every template must contain {yyyy} or {yy}; {yy_end} alone is not enough"),
    ("invalid-template-yy-next-and-yy-end", edit(fixed, setp("naming.schemes.fy.templates.year", "FY{yy_next}-{yy_end}")), {"code": "INVALID_TEMPLATE", "path": "naming.schemes.fy.templates.year"}, "templates", "{yy_next} and {yy_end} together are not enough without {yyyy} or {yy}"),
]

# Cross-block checks: (name, definition, expected error(s), paraphrase). fiscal_oracle.py computes each (asserted).
HISTORY_CROSS = [
    ("change-not-on-boundary-transition-start", edit(jul_to_apr(), both(hset("transition.start", "2024-08-01"), hset("transition.subdivision.quarters", [3, 3, 2]))),
     {"code": "CHANGE_NOT_ON_BOUNDARY", "path": T0 + ".start"},
     "The change point, transition.start 2024-08-01, is not the first day of a 1 July year under the previous (top-level) rule"),
    ("change-not-on-boundary-from-own-rule", edit(jul_to_apr(), both(hset("from", "2025-05-01"), hset("transition.subdivision.quarters", [3, 3, 4]))),
     {"code": "CHANGE_NOT_ON_BOUNDARY", "path": H0 + ".from"},
     "from 2025-05-01 is not the first day of a 1 April year under the entry's own rule"),
    ("change-not-on-boundary-from-previous-rule", jan_to("2025-04-01"),
     {"code": "CHANGE_NOT_ON_BOUNDARY", "path": H0 + ".from"},
     "Without a transition the change point is from, and 2025-04-01 is not the first day of a 1 January year under the previous (top-level) rule"),
    ("change-not-on-boundary-from-both-rules", jan_to("2025-05-01"),
     [{"code": "CHANGE_NOT_ON_BOUNDARY", "path": H0 + ".from"}, {"code": "CHANGE_NOT_ON_BOUNDARY", "path": H0 + ".from"}],
     "from 2025-05-01, with no transition, is a year start under neither the previous rule (1 January) nor the entry's own rule (1 April), so both errors are reported at from, the previous regime's first"),
    ("exceptions-overlap-from", jul_to_apr_then(hv.entry("2025-04-01", *hv.fixed_rule(4, 1, (4, 4, 4)))),
     {"code": "EXCEPTIONS_OVERLAP", "path": H1 + ".from"},
     "Entry 1's change point, its from 2025-04-01, is on entry 0's from, so entry 0's regime would have no fiscal year"),
    ("exceptions-overlap-transition-start", jul_to_apr_then(hv.entry("2026-04-01", *hv.fixed_rule(4, 1, (4, 4, 4)), start="2025-04-01", transition_sub=hv.months(1, [3, 3, 3, 3]))),
     {"code": "EXCEPTIONS_OVERLAP", "path": H1 + ".transition.start"},
     "Entry 1's change point, its transition.start 2025-04-01, is on entry 0's from"),
    ("exceptions-overlap-and-not-on-boundary", jul_to_apr_then(hv.entry("2024-10-01", *hv.fixed_rule(10, 1))),
     [{"code": "CHANGE_NOT_ON_BOUNDARY", "path": H1 + ".from"}, {"code": "EXCEPTIONS_OVERLAP", "path": H1 + ".from"}],
     "Entry 1's from 2024-10-01 is before entry 0's from and is not the first day of a 1 April year under entry 0's rule; the checks run independently and the errors are sorted by path, then code"),
    ("transition-month-start-day-mismatch", edit(jul_to_apr(), hset("transition.subdivision.month_start_day", 15)),
     {"code": "SUBDIVISION_YEAR_MISMATCH", "path": TS0 + ".month_start_day"},
     "A calendar_months transition's month_start_day (15) differs from the day of transition.start and of from (1)"),
    ("transition-month-start-day-mismatch-skips-quarters", edit(jul_to_apr(), both(hset("transition.subdivision.month_start_day", 15), hset("transition.subdivision.quarters", [3, 3]))),
     {"code": "SUBDIVISION_YEAR_MISMATCH", "path": TS0 + ".month_start_day"},
     "When a calendar_months transition's month_start_day check fails, its quarters are not compared with its number of months, so quarters summing to 6 are not also reported"),
    ("transition-month-start-day-matches-start-only",
     hv.with_history(fixed_date(1, 15, "monday", named("start")), hv.entry("2025-04-01", *hv.fixed_rule(4, 1), start="2024-01-15", transition_sub=hv.months(15, [3] * 5))),
     {"code": "SUBDIVISION_YEAR_MISMATCH", "path": TS0 + ".month_start_day"},
     "15 January years change to 1 April years through a transition from 2024-01-15 to 2025-04-01. The transition's month_start_day 15 is the day of transition.start but not of from (1), which is enough for the error; its quarters are then not compared with its number of months"),
    ("transition-month-start-day-matches-from-only",
     hv.with_history(fixed_date(1, 15, "monday", named("start")), hv.entry("2025-04-01", *hv.fixed_rule(4, 1), start="2024-01-15", transition_sub=hv.months(1, [3] * 5))),
     {"code": "SUBDIVISION_YEAR_MISMATCH", "path": TS0 + ".month_start_day"},
     "As above, but the transition's month_start_day 1 is the day of from and not of transition.start (15)"),
    ("transition-quarters-sum-mismatch", edit(jul_to_apr(), hset("transition.subdivision.quarters", [3, 3])),
     {"code": "QUARTERS_SUM_MISMATCH", "path": TS0 + ".quarters"},
     "The transition from 2024-07-01 to 2025-04-01 has (12 x 2025 + 4) - (12 x 2024 + 7) = 9 months, but its quarters sum to 6"),
    ("transition-week-pattern-wrong-weekday-and-length",
     hv.with_history(fixed_date(7, 1, "sunday", named("majority")), hv.entry("2024-12-29", WEEK_BASED_DECEMBER, PATTERN_4_4_5, start="2024-07-01", transition_sub=hv.weeks([26], [1]))),
     {"code": "SUBDIVISION_YEAR_MISMATCH", "path": TS0 + ".type"},
     "A week_pattern transition from Monday 2024-07-01 (weeks start on Sunday) to 2024-12-29 is 181 days: it fails both the week-start and the whole-weeks condition, which is one error, and its periods are not compared with a length that is not whole weeks"),
    ("transition-week-pattern-wrong-weekday",
     hv.with_history(fixed_date(7, 1, "sunday", named("start")), hv.entry("2025-03-24", *hv.fixed_rule(3, 24), start="2024-07-01", transition_sub=hv.weeks([37], [1]))),
     [{"code": "WEEKS_SUM_MISMATCH", "path": TS0 + ".periods"}, {"code": "SUBDIVISION_YEAR_MISMATCH", "path": TS0 + ".type"}],
     "A week_pattern transition from Monday 2024-07-01 (weeks start on Sunday) to 2025-03-24 lasts 266 days, 38 whole weeks, but does not start on a week start. Because the length is whole weeks, periods are still compared with it, and they sum to 37. Sorted by path, periods comes before type"),
    ("transition-week-pattern-not-whole-weeks",
     hv.with_history(week_based("end", 9, "last", "last", "sunday", [4, 4, 5] * 4, [3, 3, 3, 3], 1, named("start")),
                     hv.entry("2024-01-01", *hv.fixed_rule(1, 1), start="2023-10-01", transition_sub=hv.weeks([13], [1]))),
     {"code": "SUBDIVISION_YEAR_MISMATCH", "path": TS0 + ".type"},
     "A week_pattern transition from Sunday 2023-10-01, a week start, to 2024-01-01 lasts 92 days, not a whole number of weeks"),
    ("transition-weeks-sum-mismatch", hv.nrf_to_december_definition(periods=(4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 3)),
     {"code": "WEEKS_SUM_MISMATCH", "path": TS0 + ".periods"},
     "The week_pattern transition from 2024-02-04 to 2024-12-29 is 47 weeks, but its periods sum to 46"),
]

# Year numbering across a change: (name, definition, code, details.years, explanation). Computed by the oracle's scan (asserted).
HISTORY_NAMING = [
    ("year-number-collision-across-change",
     hv.with_history(fixed_date(1, 1, "monday", named("start")), hv.entry("2024-07-01", *hv.fixed_rule(7, 1), start="2024-01-01", transition_sub=hv.months(1, [3, 3]))),
     "YEAR_NUMBER_COLLISION", [2024],
     "Calendar years change to 1 July years through a six-month transition, 2024-01-01 to 2024-06-30. Numbered by 'start', the transition and the year from 1 July 2024 both get 2024, so the number does not increase across the change. "
     "That pair is across a change, so details.years names the later year's number, 2024; no other pair breaks the rules"),
    ("majority-tie-transition",
     hv.with_history(fixed_date(12, 1, "monday", named("majority")), hv.entry("2025-02-01", *hv.fixed_rule(2, 1), start="2024-12-01", transition_sub=hv.months(1, [2]))),
     "MAJORITY_TIE", [2024],
     "1 December years change to 1 February years through a two-month transition, 2024-12-01 to 2025-01-31, with 31 days in 2024 and 31 in 2025, so no calendar year holds more than half. "
     "Every 1 December year and every 1 February year has a majority, so details.years lists only the calendar year in which the transition starts, 2024"),
    ("majority-tie-three-calendar-years",
     hv.with_history(fixed_date(7, 1, "monday", named("majority")), hv.entry("2026-07-01", *hv.fixed_rule(7, 1, (4, 4, 4)), start="2024-07-01", transition_sub=hv.months(1, [3] * 8))),
     "MAJORITY_TIE", [2024],
     "A 24-month transition, 2024-07-01 to 2026-06-30, has 184 days in 2024, 365 in 2025 and 181 in 2026; 365 of 730 is exactly half, so no calendar year holds more than half. "
     "Every 1 July year has 184 days in its first calendar year, so details.years lists only the calendar year in which the transition starts, 2024"),
]


def as_list(expected) -> list[dict]:
    """The list form of expect_error, which the runners compare with the whole sorted error list (a single object is compared with the first error only)."""
    return expected if isinstance(expected, list) else [expected]


def history_error_vectors() -> list[dict]:
    out = []
    for name, defn, expected, anchor, rule in HISTORY_BLOCK:
        url = DEFINITION_URL if anchor == "top-level" else f"{DEFINITION_URL}#{anchor}"
        desc = (
            f"Invalid definition (fiscalclock 0.2.0). Spec rule, paraphrased: {rule}. The definition has no other error, so the complete "
            "error list is asserted (code and path of each error, sorted by path, then code)."
        )
        out.append(vector(f"definition-{name}", desc, source("errors", HISTORY_SPEC, HISTORY_CHECKED, url), defn, [{"op": "validate", "args": {}, "expect_error": as_list(expected)}]))
    for name, defn, expected, rule in HISTORY_CROSS:
        got = [e.expect_error() for e in Calendar(defn).change_errors()]
        assert got == as_list(expected), (name, got)
        desc = (
            "Invalid definition (fiscalclock 0.2.0), rejected by the cross-block checks on exceptions.history, which run only when the "
            f"definition has no block-level error. {rule}. The complete error list is asserted (code and path of each error, sorted by path, then code)."
        )
        src = source("errors", HISTORY_SPEC, CROSS_CHECKED, DEFINITION_URL + "#cross-block-checks")
        out.append(vector(f"definition-{name}", desc, src, defn, [{"op": "validate", "args": {}, "expect_error": as_list(expected)}]))
    for name, defn, code, years, why in HISTORY_NAMING:
        cal = Calendar(defn)
        assert not cal.change_errors(), name
        got = [e.expect_error() for e in cal.naming_errors()]
        assert got == [{"code": code, "path": "naming.schemes.fy.year_number", "details": {"years": years}}], (name, got)
        c = {"op": "validate", "args": {}, "expect_error": got}
        desc = (
            f"Invalid definition (fiscalclock 0.2.0). {why}. The complete error list is asserted (code, path and details)."
        )
        src = source(
            "errors",
            "spec/definition-v1.md, Year numbers must be unique (fiscalclock 0.2.0); path from spec/api-v1.md, Paths for common errors",
            HISTORY_CHECKED + "; fiscal_oracle.py Calendar.naming_errors scans every fiscal year in the supported range (asserted)",
            YEAR_NUMBERS_URL,
        )
        out.append(vector(f"definition-{name}", desc, src, defn, [c]))
    return out


def build() -> list[dict]:
    return invalid_definition_vectors() + naming_error_vectors() + runtime_vectors() + history_error_vectors()
