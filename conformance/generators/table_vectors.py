"""Oracle vectors for table years and the `table` subdivision (fiscalclock 0.3.0, ADR-0022).

Expected results come from fiscal_oracle.py. Every date in these definitions is illustrative: none is taken from a
government calendar or an ERP export, and each vector's description says so.
"""

from __future__ import annotations

import copy
from datetime import date, timedelta

from fiscal_oracle import Calendar, calendar_id, iso
from history_vectors import checked_calendar, entry, weeks, with_history
from oracle_vectors import iso_definition, label_cases, locate_cases, nrf_definition, oracle_source, structure_cases, year_cases
from vectorlib import case, coords, fixed_date, naming, scheme, validate_case, vector

CHECKED = "fiscal_oracle.py: Calendar.seq narrowed to the table's years; the table cross-block checks and naming_errors pass"
TEMPLATES = {"year": "FY{yyyy}", "quarter": "FY{yyyy} Q{q}", "period": "FY{yyyy} P{pp}", "week": "FY{yyyy} W{ww}"}


def table(boundaries, quarters) -> dict:
    return {"type": "table", "boundaries": list(boundaries), "quarters": list(quarters)}


def table_year(boundaries) -> dict:
    return {"type": "table", "boundaries": list(boundaries)}


def starts_by_days(first: str, lengths) -> list[str]:
    """Period starts from `first`, each period `lengths[i]` days long."""
    d, out = date.fromisoformat(first), [first]
    for n in lengths:
        d += timedelta(days=n)
        out.append(d.isoformat())
    return out


def nepal_periods(year_starts: list[str]) -> list[str]:
    """Twelve periods per year, each starting on an illustrative day of the next Gregorian month, as Bikram Sambat months roughly do."""
    days = [17, 17, 17, 18, 16, 16, 15, 14, 13, 14, 15]
    out = []
    for s in year_starts[:-1]:
        d = date.fromisoformat(s)
        out.append(s)
        for j, day in enumerate(days, start=1):
            m0 = d.month - 1 + j
            out.append(date(d.year + m0 // 12, m0 % 12 + 1, day).isoformat())
    return out + [year_starts[-1]]


def operation_cases(cal: Calendar, dates, dim_ranges) -> list[dict]:
    """Every operation over the table's years, including the edges of the table and the steps that run off it."""
    first, last = cal.number(0), cal.number(cal.last_key())
    s0, e_last = cal.seq[0].start, cal.seq[-1].end
    cases = [case(cal, "years", {"from": first, "to": last}), case(cal, "years", {"from": first - 1, "to": last}),
             case(cal, "years", {"from": first, "to": last + 1})]
    cases += year_cases(cal, range(first - 1, last + 2))
    cases += structure_cases(cal, (first,))
    cases += locate_cases(cal, [iso(s0 - 1), iso(s0), *dates, iso(e_last - 1), iso(e_last)])
    n_periods = cal.count(0, "period")
    n_weeks_first, n_weeks_last = cal.count(0, "week"), cal.count(cal.last_key(), "week")
    nav = [
        ("period", {"level": "period", "year": first, "index": n_periods + 1}),
        ("period", {"level": "week", "year": first, "index": n_weeks_first}),
        ("period", {"level": "week", "year": first, "index": n_weeks_first + 1}),
        ("parent", {"period": coords("week", first, 1)}),
        ("parent", {"period": coords("week", first, 20)}),
        ("parent", {"period": coords("period", first, n_periods)}),
        ("parent", {"period": coords("quarter", first, 2)}),
        ("parent", {"period": coords("year", first)}),
        ("children", {"period": coords("year", first), "level": "quarter"}),
        ("children", {"period": coords("quarter", first, 1), "level": "period"}),
        ("children", {"period": coords("quarter", first, 1), "level": "week"}),
        ("children", {"period": coords("period", first, 1), "level": "week"}),
        ("children", {"period": coords("year", last), "level": "period"}),
        ("shift", {"period": coords("period", first, n_periods), "n": 1}),
        ("shift", {"period": coords("period", first, 2), "n": n_periods}),
        ("shift", {"period": coords("period", last, n_periods), "n": -n_periods}),
        ("shift", {"period": coords("period", last, n_periods), "n": 1}),
        ("shift", {"period": coords("period", first, 1), "n": -1}),
        ("shift", {"period": coords("quarter", first, 4), "n": 1}),
        ("shift", {"period": coords("quarter", last, 4), "n": 1}),
        ("shift", {"period": coords("week", first, n_weeks_first), "n": 1}),
        ("shift", {"period": coords("week", last, n_weeks_last), "n": 1}),
        ("shift", {"period": coords("week", first, 1), "n": -1}),
        ("shift", {"period": coords("year", first), "n": last - first}),
        ("shift", {"period": coords("year", first), "n": last - first + 1}),
        ("shift", {"period": coords("year", last), "n": -(last - first) - 1}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", first), coords("year", last), coords("quarter", first, 3), coords("period", first, 1),
                               coords("period", last, n_periods), coords("week", first, 1), coords("week", last, n_weeks_last)])
    for label in (TEMPLATES["year"].format(yyyy=first - 1), TEMPLATES["year"].format(yyyy=last + 1),
                  TEMPLATES["period"].format(yyyy=first, pp="%02d" % (n_periods + 1))):
        cases.append(case(cal, "parse", {"label": label}))
    for a, b in dim_ranges:
        cases.append(case(cal, "dimension", {"start": a, "end_exclusive": b}))
        cases.append(case(cal, "dimension_csv", {"start": a, "end_exclusive": b}))
    edge_ranges = [(iso(s0 - 1), iso(s0 + 2)), (iso(e_last - 2), iso(e_last + 1))]
    cases += [case(cal, "dimension", {"start": a, "end_exclusive": b}) for a, b in edge_ranges]
    return cases


EDGES = (
    "The calendar is defined only between the table's first and last entry, so dates, year numbers, shifts, years, labels and "
    "dimension ranges outside it are OUT_OF_RANGE. A table subdivision does not split periods into weeks: weeks count from the "
    "fiscal year's first day, 'weeks' is null, the parent of a week is the year, and the weeks of a quarter or period are LEVEL_NOT_NESTED."
)


# 1. A table year with a table subdivision, shaped like the illustrative Nepal example of ADR-0022 ---------------


def nepal_definition() -> dict:
    years = ["2024-07-16", "2025-07-17", "2026-07-17", "2027-07-17"]
    return {
        "spec_version": 1,
        "year": table_year(years),
        "week": {"starts_on": "sunday", "numbering": "contains_start"},
        "subdivision": table(nepal_periods(years), [3, 3, 3, 3]),
        "naming": naming({"bs": scheme("start", TEMPLATES, offset=57)}),
    }


def nepal_vector() -> dict:
    defn = nepal_definition()
    cal = checked_calendar(defn)
    assert [cal.number(k) for k in range(len(cal.seq))] == [2081, 2082, 2083]
    cases = [validate_case(defn)]
    cases += operation_cases(cal, ["2024-08-16", "2024-08-17", "2025-01-13", "2025-07-16", "2025-07-17"],
                             [("2025-07-14", "2025-07-20")])
    desc = (
        "ILLUSTRATIVE: a table year with a table subdivision, shaped like the Nepal example in ADR-0022. The year starts "
        "2024-07-16, 2025-07-17 and 2026-07-17 (the calendar ends 2027-07-16) come from the research proposal and are unverified; the "
        "twelve period starts in each year are invented, on the 13th to 18th of each Gregorian month. year_number 'start' with offset "
        "57 numbers the years 2081, 2082 and 2083. " + EDGES
    )
    return vector("table-year-table-subdivision", desc, oracle_source(CHECKED), defn, cases)


# 2. A fixed_date year with a table subdivision: an ERP-style import of 13 irregular periods -----------------------


def erp_periods(first_year: int, n_years: int) -> list[str]:
    """Thirteen periods per calendar year: twelve of 28 days, then the rest of the year (29 or 30 days)."""
    out = []
    for y in range(first_year, first_year + n_years):
        out += starts_by_days(f"{y}-01-01", [28] * 12)
    return out + [f"{first_year + n_years}-01-01"]


def erp_definition() -> dict:
    defn = fixed_date(1, 1, "monday", naming({"fy": scheme("start", TEMPLATES)}))
    defn["subdivision"] = table(erp_periods(2023, 3), [3, 3, 3, 4])
    return defn


def erp_vector() -> dict:
    defn = erp_definition()
    cal = checked_calendar(defn)
    cases = [validate_case(defn)]
    cases += operation_cases(cal, ["2023-01-29", "2023-12-02", "2024-02-26", "2024-12-31", "2025-01-01"],
                             [("2024-12-29", "2025-01-03")])
    desc = (
        "ILLUSTRATIVE: a fixed_date year from 1 January with a table subdivision listing thirteen periods per year for 2023 to 2025, "
        "as an ERP period table import might: twelve periods of 28 days, then the rest of the year. The dates are invented. The table "
        "narrows the fixed_date rule to 2023-01-01 to 2025-12-31. " + EDGES
    )
    return vector("table-fixed-date-erp-13-periods", desc, oracle_source(CHECKED), defn, cases)


# 3. A week_based year with a table subdivision ---------------------------------------------------------------


def week_based_definition() -> dict:
    defn = nrf_definition()
    defn["naming"] = naming({"fy": scheme("start", TEMPLATES)})
    cal = Calendar(nrf_definition())
    k = cal.key_for_scheme_number(2024, cal.default_scheme)
    periods = []
    for j in range(k, k + 3):
        s, e = cal.key_bounds(j)
        assert e - s == 364, "keep the 53-week question out of this vector"
        periods += starts_by_days(iso(s), [31, 28, 35, 30, 28, 35, 28, 30, 35, 28, 28])
    periods.append(iso(cal.key_bounds(k + 2)[1]))
    defn["subdivision"] = table(periods, [3, 3, 3, 3])
    return defn


def week_based_vector() -> dict:
    defn = week_based_definition()
    cal = checked_calendar(defn)
    cases = [validate_case(defn)]
    cases += operation_cases(cal, ["2024-03-06", "2024-03-07", "2025-02-01", "2025-02-02"], [("2025-01-30", "2025-02-05")])
    desc = (
        "ILLUSTRATIVE: a week_based year (ends on the Saturday nearest the end of January, weeks from Sunday, the NRF anchor) with a "
        "table subdivision for the fiscal years 2024 to 2026, each 364 days, split into twelve invented periods of 28 to 35 days that do "
        "not follow week boundaries. None of the years has 53 weeks, so has_extra_week is false throughout. " + EDGES
    )
    return vector("table-week-based", desc, oracle_source(CHECKED), defn, cases)


# 4. Table regimes in exceptions.history -----------------------------------------------------------------------


def table_then_rule_definition() -> dict:
    years = ["2022-07-16", "2023-07-17", "2024-07-16"]
    base = {
        "spec_version": 1,
        "year": table_year(years),
        "week": {"starts_on": "tuesday", "numbering": "contains_start"},
        "subdivision": table(nepal_periods(years), [3, 3, 3, 3]),
        "naming": naming({"fy": scheme("start", TEMPLATES)}),
    }
    april = ({"type": "fixed_date", "start": {"month": 4, "day": 1}}, {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]})
    return with_history(base, entry("2025-04-01", *april, start="2024-07-16", transition_sub=weeks([13, 12, 12], [1, 1, 1])))


def table_then_rule_vector() -> dict:
    defn = table_then_rule_definition()
    cal = checked_calendar(defn)
    assert [cal.number(k) for k in range(4)] == [2022, 2023, 2024, 2025]
    cases = [validate_case(defn), case(cal, "years", {"from": 2022, "to": 2026}), case(cal, "years", {"from": 2021, "to": 2023})]
    cases += year_cases(cal, range(2021, 2027))
    cases += structure_cases(cal, (2023, 2024), levels=("period",))
    cases += locate_cases(cal, ["2022-07-15", "2022-07-16", "2024-07-15", "2024-07-16", "2025-03-31", "2025-04-01"])
    nav = [
        ("shift", {"period": coords("period", 2023, 12), "n": 1}),
        ("shift", {"period": coords("period", 2024, 1), "n": -1}),
        ("shift", {"period": coords("period", 2022, 1), "n": -1}),
        ("shift", {"period": coords("quarter", 2022, 1), "n": 10}),
        ("shift", {"period": coords("week", 2023, 1), "n": -1}),
        ("shift", {"period": coords("year", 2022), "n": -1}),
        ("parent", {"period": coords("week", 2023, 5)}),
        ("parent", {"period": coords("week", 2024, 5)}),
        ("children", {"period": coords("period", 2023, 1), "level": "week"}),
        ("children", {"period": coords("period", 2024, 1), "level": "week"}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", 2023), coords("period", 2023, 12), coords("year", 2024)])
    cases.append(case(cal, "parse", {"label": "FY2021"}))
    cases.append(case(cal, "dimension", {"start": "2024-07-14", "end_exclusive": "2024-07-18"}))
    cases.append(case(cal, "dimension", {"start": "2022-07-15", "end_exclusive": "2022-07-18"}))
    desc = (
        "ILLUSTRATIVE: regime 0 is a table year (2022-07-16, 2023-07-17, ending 2024-07-15) with invented table periods, so the calendar "
        "starts on 2022-07-16. History entry 0 changes to 1 April years from 2025-04-01 through a 37-week week_pattern transition that "
        "starts on 2024-07-16, the day after the table's last year, which counts as a year start of the previous rule for its change "
        "point. Weeks start on Tuesday, the weekday of 2024-07-16. Numbered by 'start': 2022 and 2023 (table), 2024 (transition), 2025 on. "
        "Covers locate, shift and labels across the change, weeks nested in the transition's periods but not in the table's, and "
        "OUT_OF_RANGE before the table."
    )
    return vector("table-history-table-then-rule", desc, oracle_source(CHECKED), defn, cases)


def rule_then_table_definition() -> dict:
    base = fixed_date(1, 1, "monday", naming({"fy": scheme("start", TEMPLATES)}))
    return with_history(base, entry("2024-01-01", {"type": "fixed_date", "start": {"month": 1, "day": 1}}, table(erp_periods(2024, 2), [3, 3, 3, 4])))


def rule_then_table_vector() -> dict:
    defn = rule_then_table_definition()
    cal = checked_calendar(defn)
    assert iso(cal.seq[-1].end) == "2026-01-01"
    cases = [validate_case(defn), case(cal, "years", {"from": 2022, "to": 2025}), case(cal, "years", {"from": 2024, "to": 2026})]
    cases += year_cases(cal, (2023, 2024, 2025, 2026))
    cases += structure_cases(cal, (2023, 2024), levels=("period",))
    cases += locate_cases(cal, ["2023-12-31", "2024-01-01", "2025-12-31", "2026-01-01"])
    nav = [
        ("shift", {"period": coords("period", 2023, 12), "n": 1}),
        ("shift", {"period": coords("period", 2024, 1), "n": -1}),
        ("shift", {"period": coords("period", 2023, 1), "n": 37}),
        ("shift", {"period": coords("period", 2023, 1), "n": 38}),
        ("shift", {"period": coords("quarter", 2025, 4), "n": 1}),
        ("shift", {"period": coords("year", 2023), "n": 2}),
        ("shift", {"period": coords("year", 2023), "n": 3}),
        ("parent", {"period": coords("week", 2023, 10)}),
        ("parent", {"period": coords("week", 2024, 10)}),
        ("children", {"period": coords("quarter", 2023, 4), "level": "period"}),
        ("children", {"period": coords("quarter", 2024, 4), "level": "period"}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("period", 2023, 12), coords("period", 2024, 13), coords("year", 2025)])
    cases.append(case(cal, "parse", {"label": "FY2026"}))
    cases.append(case(cal, "dimension", {"start": "2023-12-30", "end_exclusive": "2024-01-03"}))
    cases.append(case(cal, "dimension", {"start": "2025-12-30", "end_exclusive": "2026-01-02"}))
    desc = (
        "ILLUSTRATIVE: 1 January years with twelve calendar-month periods, then from 2024-01-01 (no transition) the same 1 January years "
        "with an invented table of thirteen periods for 2024 and 2025. The last regime has a table, so the calendar ends on 2025-12-31: "
        "year 2026, years to 2026, shifts past 2025, locate on 2026-01-01 and a dimension range that reaches it are OUT_OF_RANGE. Numbered "
        "by 'start', 2023 and 2024 are consecutive across the change."
    )
    return vector("table-history-rule-then-table", desc, oracle_source(CHECKED), defn, cases)


# 5. Year numbering runs over the narrowed years only ------------------------------------------------------------


def narrowed_numbering_vector() -> dict:
    defn = iso_definition("start")
    defn["naming"] = naming({"fy": scheme("start", {"year": "FY{yyyy}", "period": "FY{yyyy} P{pp}"})})
    assert Calendar(defn).naming_errors(), "start must collide over the whole range"
    defn["subdivision"] = table(["2016-01-04", "2016-07-04", "2017-01-02", "2017-07-03", "2018-01-01", "2018-07-02", "2018-12-31"], [1, 1])
    cal = checked_calendar(defn)
    cases = [validate_case(defn), case(cal, "years", {"from": 2016, "to": 2018})]
    cases += locate_cases(cal, ["2016-01-03", "2016-01-04", "2018-12-30", "2018-12-31"])
    cases += label_cases(cal, [coords("year", 2018), coords("period", 2017, 2)])
    desc = (
        "ILLUSTRATIVE: the ISO 8601 week-year anchor (Monday nearest 1 January) numbered by 'start', which collides over the whole supported "
        "range (the years starting 2018-01-01 and 2018-12-31 both get 2018), with a table subdivision of two invented half-years per year for "
        "the ISO years 2016 to 2018, which all start in January. Year numbering runs only over the years the table covers, so 2016, 2017 "
        "and 2018 are consecutive and the definition is valid."
    )
    return vector("table-week-based-narrowed-numbering", desc, oracle_source(CHECKED), defn, cases)


# 6. calendar_id ------------------------------------------------------------------------------------------------


def calendar_id_vectors() -> list[dict]:
    a = erp_definition()
    b = copy.deepcopy(a)
    b = {k: b[k] for k in reversed(list(b))}
    b["subdivision"] = {"quarters": b["subdivision"]["quarters"], "boundaries": b["subdivision"]["boundaries"], "type": "table"}
    b["naming"]["schemes"]["fy"]["offset"] = 0
    b["$schema"] = "https://example.invalid/fiscalclock/definition-v1.json"
    ca, cb = validate_case(a), validate_case(b)
    assert ca["expect"] == cb["expect"]
    cid = calendar_id(a)
    desc = (
        "{} of a pair of table-subdivision definitions that differ only in key order, $schema and a written-out default (offset 0); "
        "both must have calendar_id " + cid + ". The boundaries are written in the canonical form as the date strings given. Illustrative dates."
    )
    src = oracle_source("hashlib.sha256 over the canonical form built in fiscal_oracle.canonical")
    return [vector("table-calendar-id-pair-a", desc.format("First"), src, a, [ca]), vector("table-calendar-id-pair-b", desc.format("Second"), src, b, [cb])]


def build() -> list[dict]:
    return [nepal_vector(), erp_vector(), week_based_vector(), table_then_rule_vector(), rule_then_table_vector(), narrowed_numbering_vector(), *calendar_id_vectors()]
