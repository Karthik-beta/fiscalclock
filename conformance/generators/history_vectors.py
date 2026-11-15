"""Oracle vectors for calendars whose year rule changes (`exceptions.history`, fiscalclock 0.2.0).

Expected results come from fiscal_oracle.py. The definitions are shaped like changes that Indian listed
companies made (see research/filers/), but their quarter splits, week blocks and labels are illustrative;
the cited vectors in spec_cited_vectors.py assert only what the filings state.
"""

from __future__ import annotations

import copy
import hashlib
from datetime import date

from fiscal_oracle import Calendar, calendar_id
from oracle_vectors import calendar_id_vectors, label_cases, locate_cases, nrf_definition, oracle_source, structure_cases, year_cases
from vectorlib import case, coords, fixed_date, naming, scheme, validate_case, vector, week_based, year_args

CHECKED = "fiscal_oracle.py: Calendar.seq built from the regimes and transitions; change_errors and naming_errors pass"


def months(day: int, quarters) -> dict:
    return {"type": "calendar_months", "month_start_day": day, "quarters": list(quarters)}


def weeks(periods, quarters) -> dict:
    return {"type": "week_pattern", "periods": list(periods), "quarters": list(quarters)}


def entry(frm: str, year: dict, subdivision: dict, start: str | None = None, transition_sub: dict | None = None) -> dict:
    out = {"from": frm}
    if start is not None:
        out["transition"] = {"start": start, "subdivision": transition_sub}
    out["year"] = year
    out["subdivision"] = subdivision
    return out


def fixed_rule(month: int, day: int, quarters=(3, 3, 3, 3)) -> tuple[dict, dict]:
    return {"type": "fixed_date", "start": {"month": month, "day": day}}, months(day, quarters)


def with_history(base: dict, *entries) -> dict:
    d = copy.deepcopy(base)
    d["exceptions"] = {"history": list(entries)}
    return d


def checked_calendar(defn: dict) -> Calendar:
    cal = Calendar(defn)
    assert not cal.validation_errors(), cal.validation_errors()
    return cal


def number_of(cal: Calendar, d: str) -> int:
    return cal.number(cal.find(date.fromisoformat(d).toordinal()))


TEMPLATES = {"year": "{yyyy}-{yy_end}", "quarter": "Q{q} {yyyy}-{yy_end}", "period": "P{pp} {yyyy}-{yy_end}", "week": "W{ww} {yyyy}-{yy_end}"}


# 1. July-June to April-March through nine months (shaped like P&G Hygiene and Health Care) ---------------


def jul_to_apr_definition() -> dict:
    base = fixed_date(7, 1, "monday", naming({"fy": scheme("start", TEMPLATES)}))
    return with_history(base, entry("2025-04-01", *fixed_rule(4, 1), start="2024-07-01", transition_sub=months(1, [3, 3, 3])))


def jul_to_apr_vector() -> dict:
    defn = jul_to_apr_definition()
    cal = checked_calendar(defn)
    cases = [validate_case(defn), case(cal, "years", {"from": 2022, "to": 2027})]
    cases += year_cases(cal, range(2022, 2028))
    cases += structure_cases(cal, (2024,))
    cases += locate_cases(cal, ["2024-06-30", "2024-07-01", "2024-11-15", "2025-03-30", "2025-03-31", "2025-04-01"])
    nav = [
        ("children", {"period": coords("year", 2024), "level": "week"}),
        ("children", {"period": coords("year", 2024), "level": "quarter"}),
        ("children", {"period": coords("quarter", 2024, 3), "level": "period"}),
        ("children", {"period": coords("quarter", 2024, 1), "level": "week"}),
        ("parent", {"period": coords("week", 2024, 40)}),
        ("parent", {"period": coords("period", 2024, 9)}),
        ("parent", {"period": coords("quarter", 2024, 3)}),
        ("shift", {"period": coords("quarter", 2023, 4), "n": 1}),
        ("shift", {"period": coords("quarter", 2024, 3), "n": 1}),
        ("shift", {"period": coords("quarter", 2025, 1), "n": -1}),
        ("shift", {"period": coords("quarter", 2023, 1), "n": 8}),
        ("shift", {"period": coords("period", 2024, 9), "n": 1}),
        ("shift", {"period": coords("period", 2024, 1), "n": -1}),
        ("shift", {"period": coords("period", 2023, 12), "n": 10}),
        ("shift", {"period": coords("week", 2024, 40), "n": 1}),
        ("shift", {"period": coords("week", 2025, 1), "n": -1}),
        ("shift", {"period": coords("week", 2023, 53), "n": 1}),
        ("shift", {"period": coords("year", 2023), "n": 2}),
        ("shift", {"period": coords("year", 2026), "n": -3}),
        ("period", {"level": "quarter", "year": 2024, "index": 4}),
        ("period", {"level": "period", "year": 2024, "index": 10}),
        ("period", {"level": "week", "year": 2024, "index": 41}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", 2023), coords("year", 2024), coords("year", 2025), coords("quarter", 2024, 3),
                               coords("period", 2024, 9), coords("week", 2024, 40), coords("week", 2024, 1)])
    for label in ("2024-24", "2024-26", "Q4 2024-25", "P10 2024-25", "2025-25"):
        cases.append(case(cal, "parse", {"label": label}))
    cases.append(case(cal, "dimension", {"start": "2025-03-28", "end_exclusive": "2025-04-03"}))
    cases.append(case(cal, "dimension_csv", {"start": "2025-03-28", "end_exclusive": "2025-04-03"}))
    desc = (
        "A fixed-date calendar that changes from 1 July to 1 April years through a nine-month transition year, 2024-07-01 to "
        "2025-03-31, split into three quarters of three calendar months (exceptions.history with a calendar_months transition). "
        "The dates are those of research/filers/pg-hygiene.md; the quarter split, weeks and the period and week templates are "
        "illustrative. Numbered by 'start', the years run 2023, 2024 (the transition), 2025 with no skip, and {yy_end} gives "
        "'2024-25' for the transition. Covers the transition's quarters, periods and 40 weeks (week 40 is 2025-03-31 alone), "
        "LEVEL_NOT_NESTED for weeks of a calendar_months transition quarter, shift across both changes, out-of-range indexes "
        "inside the transition, labels and parse (a {yy_end} that disagrees with the year's last day is LABEL_NOT_RECOGNISED), "
        "and dimension rows across the end of the transition."
    )
    return vector("history-jul-to-apr-9-month-transition", desc, oracle_source(CHECKED), defn, cases)


# 2. October-September to April-March through eighteen months (shaped like Siemens Limited) -----------------


def oct_to_apr_definition(year_number: str, schemes_extra: dict | None = None) -> dict:
    schemes = {"fy": scheme(year_number, {"year": "FY {yyyy}-{yy_end}", "quarter": "Q{q} FY {yyyy}-{yy_end}", "period": "P{pp} FY {yyyy}-{yy_end}"})}
    schemes.update(schemes_extra or {})
    base = fixed_date(10, 1, "monday", naming(schemes, default="fy"))
    return with_history(base, entry("2026-04-01", *fixed_rule(4, 1), start="2024-10-01", transition_sub=months(1, [3] * 6)))


def oct_to_apr_vectors() -> list[dict]:
    out = []
    numbering = {
        "start": ("2023, 2024 (the transition), 2026: 2025 is skipped at the change", 2025),
        "end": ("2024, 2026 (the transition, which ends in 2026), 2027: 2025 is skipped at the change", 2025),
        "majority": ("2024, 2025 (the transition: 365 of its 547 days fall in 2025, more than half), 2026: no number is skipped", None),
    }
    for year_number, (story, skipped) in numbering.items():
        extra = {"short": scheme(year_number, {"year": "FY{yy}-{yy_end}"}, pivot=2069)} if year_number == "start" else None
        defn = oct_to_apr_definition(year_number, extra)
        cal = checked_calendar(defn)
        t = number_of(cal, "2024-10-01")
        before, after = number_of(cal, "2024-09-30"), number_of(cal, "2026-04-01")
        cases = [validate_case(defn), case(cal, "years", {"from": 2021, "to": 2028})]
        cases += year_cases(cal, range(2021, 2029))
        cases += structure_cases(cal, (t,), ("quarter",))
        cases.append(case(cal, "period", {"level": "quarter", "year": t, "index": 7}))
        cases.append(case(cal, "period", {"level": "period", "year": t, "index": 18}))
        cases.append(case(cal, "period", {"level": "period", "year": t, "index": 19}))
        cases += locate_cases(cal, ["2024-09-30", "2024-10-01", "2025-06-15", "2026-03-31", "2026-04-01"])
        nav = [
            ("children", {"period": coords("year", t), "level": "quarter"}),
            ("children", {"period": coords("quarter", t, 6), "level": "period"}),
            ("shift", {"period": coords("year", t), "n": 1}),
            ("shift", {"period": coords("year", after), "n": -1}),
            ("shift", {"period": coords("year", before), "n": 2}),
            ("shift", {"period": coords("year", after), "n": -2}),
            ("shift", {"period": coords("quarter", before, 4), "n": 1}),
            ("shift", {"period": coords("quarter", t, 6), "n": 1}),
            ("shift", {"period": coords("quarter", after, 1), "n": -6}),
            ("shift", {"period": coords("period", t, 18), "n": 1}),
            ("shift", {"period": coords("period", before, 12), "n": 19}),
        ]
        cases += [case(cal, op, args) for op, args in nav]
        cases += label_cases(cal, [coords("year", before), coords("year", t), coords("year", after), coords("quarter", t, 6), coords("period", t, 18)],
                             (None, "short") if extra else (None,))
        if skipped is not None:
            assert cal.key_for_scheme_number(skipped, "fy") is None
            cases += [
                case(cal, "period", year_args(skipped)),
                case(cal, "period", {"level": "quarter", "year": skipped, "index": 1}),
                case(cal, "years", {"from": skipped, "to": skipped}),
                case(cal, "years", {"from": skipped, "to": skipped + 1}),
                case(cal, "shift", {"period": coords("year", skipped), "n": 1}),
                case(cal, "format", {"period": coords("year", skipped)}),
                case(cal, "parent", {"period": coords("quarter", skipped, 1)}),
                case(cal, "children", {"period": coords("year", skipped), "level": "quarter"}),
                case(cal, "parse", {"label": "FY %d-%02d" % (skipped, (skipped + 1) % 100)}),
            ]
        cases.append(case(cal, "parse", {"label": "FY 2024-25"}))
        desc = (
            f"A fixed-date calendar that changes from 1 October to 1 April years through an eighteen-month transition year, "
            f"2024-10-01 to 2026-03-31, split into six quarters of three calendar months. The dates and the six quarters are those of "
            f"research/filers/siemens-india.md; the quarter split and the week block are illustrative. Numbered by '{year_number}', "
            f"the years around the change are {story}. shift of a year moves along the sequence of fiscal years, not by the number."
        )
        if skipped is not None:
            desc += (
                f" A skipped number lies inside the supported range: period with year {skipped} is PERIOD_NOT_FOUND at year, a period "
                f"argument naming it is PERIOD_NOT_FOUND at period, years({skipped}, {skipped}) is empty, and a label that names it is "
                f"LABEL_NOT_RECOGNISED."
            )
        if extra:
            desc += " Scheme 'short' writes 'FY24-26' with {yy}, century_pivot 2069 and {yy_end}."
        out.append(vector(f"history-oct-to-apr-18-month-transition-{year_number}", desc, oracle_source(CHECKED), defn, cases))
    return out


# 3. Calendar year to April-March through fifteen months (shaped like Linde India) ---------------------------


def jan_to_apr_vector() -> dict:
    base = fixed_date(1, 1, "monday", naming({
        "fy": scheme("start", {"year": "{yyyy}-{yy_end}", "quarter": "Q{q} {yyyy}-{yy_end}"}),
        "cy": scheme("start", {"year": "FY {yyyy}"}),
        "fe": scheme("end", {"year": "FE{yyyy}"}),
    }, default="fy"))
    defn = with_history(base, entry("2023-04-01", *fixed_rule(4, 1), start="2022-01-01", transition_sub=months(1, [3] * 5)))
    cal = checked_calendar(defn)
    cases = [validate_case(defn), case(cal, "years", {"from": 2020, "to": 2025})]
    cases += year_cases(cal, range(2020, 2026))
    cases += structure_cases(cal, (2022,))
    cases += locate_cases(cal, ["2021-12-31", "2022-01-01", "2022-12-31", "2023-01-01", "2023-03-31", "2023-04-01"])
    cases += [case(cal, "shift", {"period": coords("quarter", 2021, 4), "n": 6}), case(cal, "shift", {"period": coords("period", 2023, 1), "n": -15})]
    cases += label_cases(cal, [coords("year", 2021), coords("year", 2022), coords("year", 2023), coords("quarter", 2022, 5)], (None, "cy", "fe"))
    for label in ("FE2022", "FY 2022", "2022-22"):
        cases.append(case(cal, "parse", {"label": label}))
    desc = (
        "A fixed-date calendar that changes from calendar years to 1 April years through a fifteen-month transition year, "
        "2022-01-01 to 2023-03-31, split into five quarters of three calendar months. The dates are those of "
        "research/filers/linde-india.md; the quarter split, weeks and schemes are illustrative. Scheme 'fy' (the default, "
        "'start') gives 2021, 2022, 2023 and the label '2022-23' for the transition. Scheme 'fe' numbers by 'end', giving "
        "2021, 2023, 2024: 2022 is skipped at the change, which a non-default scheme may do, so 'FE2022' is LABEL_NOT_RECOGNISED."
    )
    return vector("history-jan-to-apr-15-month-transition", desc, oracle_source(CHECKED), defn, cases)


# 4. A change of quarter pattern with no transition ----------------------------------------------------------


def quarters_change_definition() -> dict:
    base = fixed_date(1, 1, "monday", naming({"fy": scheme("start", {"year": "FY{yyyy}", "quarter": "FY{yyyy} Q{q}"})}))
    return with_history(base, entry("2025-01-01", *fixed_rule(1, 1, (4, 4, 4))))


def quarters_change_vector() -> dict:
    defn = quarters_change_definition()
    cal = checked_calendar(defn)
    cases = [validate_case(defn), case(cal, "years", {"from": 2023, "to": 2026})]
    cases += structure_cases(cal, (2024, 2025), ("quarter",))
    cases.append(case(cal, "period", {"level": "quarter", "year": 2025, "index": 4}))
    cases += locate_cases(cal, ["2024-12-31", "2025-01-01", "2025-05-01"])
    nav = [
        ("shift", {"period": coords("quarter", 2024, 4), "n": 1}),
        ("shift", {"period": coords("quarter", 2025, 1), "n": -1}),
        ("shift", {"period": coords("quarter", 2024, 1), "n": 7}),
        ("shift", {"period": coords("quarter", 2026, 1), "n": -7}),
        ("children", {"period": coords("year", 2025), "level": "quarter"}),
        ("children", {"period": coords("quarter", 2025, 2), "level": "period"}),
        ("parent", {"period": coords("period", 2025, 8)}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("quarter", 2024, 4), coords("quarter", 2025, 3)])
    cases.append(case(cal, "parse", {"label": "FY2025 Q4"}))
    desc = (
        "A history entry without a transition: a 1 January calendar changes its quarters from 3-3-3-3 months to 4-4-4 months from "
        "2025-01-01, which is a year boundary of both rules. The canonical form writes the entry's transition as null. Years from "
        "2025 have three quarters, so quarter 4 of 2025 is PERIOD_NOT_FOUND and 'FY2025 Q4' is LABEL_NOT_RECOGNISED; shift counts "
        "each year's own quarters. Numbered by 'start', the years are consecutive across the change. Illustrative."
    )
    return vector("history-quarters-change-no-transition", desc, oracle_source(CHECKED), defn, cases)


# 5. Week-based to week-based through a week_pattern transition ---------------------------------------------


def nrf_to_december_definition(year_number: str = "majority", periods=(4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4)) -> dict:
    base = nrf_definition()
    base["naming"] = naming({"fy": scheme(year_number, {"year": "FY{yyyy}", "period": "FY{yyyy} P{pp}", "week": "FY{yyyy} W{ww}"})})
    december = {"type": "week_based", "anchor": {"edge": "end", "month": 12, "day": "last", "rule": "nearest"}}
    pattern = {"type": "week_pattern", "periods": [4, 4, 5] * 4, "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}}
    return with_history(base, entry("2024-12-29", december, pattern, start="2024-02-04", transition_sub=weeks(periods, [3, 3, 3, 2])))


def nrf_to_december_vector() -> dict:
    defn = nrf_to_december_definition()
    cal = checked_calendar(defn)
    k = cal.find(date(2024, 2, 4).toordinal())
    assert cal.key_bounds(k) == (date(2024, 2, 4).toordinal(), date(2024, 12, 29).toordinal()) and cal.count(k, "week") == 47
    t = cal.number(k)
    cases = [validate_case(defn), case(cal, "years", {"from": 2021, "to": 2027})]
    cases += year_cases(cal, range(2021, 2028))
    cases += structure_cases(cal, (t,))
    cases += locate_cases(cal, ["2024-02-03", "2024-02-04", "2024-12-28", "2024-12-29", "2025-12-27", "2025-12-28"])
    nav = [
        ("children", {"period": coords("year", t), "level": "week"}),
        ("children", {"period": coords("quarter", t, 4), "level": "week"}),
        ("children", {"period": coords("period", t, 3), "level": "week"}),
        ("parent", {"period": coords("week", t, 47)}),
        ("parent", {"period": coords("week", t, 13)}),
        ("shift", {"period": coords("week", 2023, 53), "n": 1}),
        ("shift", {"period": coords("week", t, 47), "n": 1}),
        ("shift", {"period": coords("week", t + 1, 1), "n": -48}),
        ("shift", {"period": coords("period", 2023, 12), "n": 1}),
        ("shift", {"period": coords("period", t, 11), "n": 1}),
        ("shift", {"period": coords("quarter", t, 4), "n": 1}),
        ("period", {"level": "week", "year": t, "index": 48}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", t), coords("period", t, 11), coords("week", t, 47), coords("week", 2023, 53)])
    cases.append(case(cal, "dimension", {"start": "2024-12-26", "end_exclusive": "2025-01-01"}))
    desc = (
        "A week-based calendar that changes from years ending on the Saturday nearest 31 January (the NRF rule, weeks from Sunday) "
        "to years ending on the Saturday nearest 31 December, through a 47-week transition from 2024-02-04 to 2024-12-28 with a "
        "week_pattern of 11 periods (4-4-5 repeated, then 4-4) in quarters of 3, 3, 3 and 2 periods. Numbered by 'majority': "
        "2023, 2024 (the transition), 2025. Covers week Periods with weeks 1 and has_extra_week false in the transition, weeks "
        "nested in transition periods and quarters, and shift from the 53rd week of 2023 into the transition and out of it. "
        "Illustrative."
    )
    return vector("history-week-based-to-week-based-week-pattern-transition", desc, oracle_source(CHECKED), defn, cases)


# 6. Fixed-date to week-based, without and with a transition --------------------------------------------------


def iso_rule() -> tuple[dict, dict]:
    year = {"type": "week_based", "anchor": {"edge": "start", "month": 1, "day": 1, "rule": "nearest"}}
    return year, {"type": "week_pattern", "periods": [4] * 13, "quarters": [3, 3, 3, 4], "extra_week": {"period": 13}}


def fixed_to_week_based_vector() -> dict:
    base = fixed_date(1, 1, "monday", naming({"cal": scheme("majority", {"year": "{yyyy}", "week": "{yyyy}-W{ww}"})}))
    defn = with_history(base, entry("2024-01-01", *iso_rule()))
    cal = checked_calendar(defn)
    cases = [validate_case(defn), case(cal, "years", {"from": 2021, "to": 2027})]
    cases += locate_cases(cal, ["2023-01-01", "2023-01-02", "2023-12-31", "2024-01-01", "2024-12-29", "2024-12-30", "2026-12-31", "2027-01-03", "2027-01-04"])
    for d, iso_year in (("2024-12-30", 2025), ("2026-12-31", 2026), ("2027-01-03", 2026)):
        assert date.fromisoformat(d).isocalendar()[0] == iso_year == number_of(cal, d)
    nav = [
        ("children", {"period": coords("quarter", 2024, 1), "level": "week"}),
        ("children", {"period": coords("quarter", 2023, 1), "level": "week"}),
        ("parent", {"period": coords("week", 2023, 53)}),
        ("parent", {"period": coords("week", 2024, 1)}),
        ("shift", {"period": coords("week", 2023, 53), "n": 1}),
        ("shift", {"period": coords("week", 2024, 1), "n": -2}),
        ("shift", {"period": coords("period", 2023, 12), "n": 14}),
        ("period", {"level": "week", "year": 2023, "index": 53}),
        ("period", {"level": "week", "year": 2026, "index": 53}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", 2023), coords("year", 2024), coords("week", 2023, 53), coords("week", 2026, 53)])
    desc = (
        "A change without a transition from calendar years (fixed_date, 1 January) to ISO 8601-style week years (week_based, "
        "Monday nearest 1 January) from 2024-01-01, a Monday that is a year boundary of both rules. Numbered by 'majority', "
        "the years are consecutive. Weeks of 2023 follow the fixed-date rule (week 1 is Sunday 1 January 2023 alone, weeks are "
        "LEVEL_NOT_NESTED in its quarters, a week's parent is the year); from 2024 weeks are whole and nest in periods. The week "
        "years from 2024 agree with Python's date.isocalendar() (asserted by the generator). Illustrative."
    )
    return vector("history-fixed-to-week-based-no-transition", desc, oracle_source(CHECKED + "; ISO years checked against date.isocalendar()"), defn, cases)


def fixed_to_week_based_transition_vector() -> dict:
    base = fixed_date(7, 1, "monday", naming({"fy": scheme("majority", {"year": "FY{yyyy}", "period": "FY{yyyy} P{pp}", "week": "FY{yyyy} W{ww}"})}))
    december = {"type": "week_based", "anchor": {"edge": "end", "month": 12, "day": "last", "rule": "last"}}
    pattern = {"type": "week_pattern", "periods": [4, 4, 5] * 4, "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}}
    defn = with_history(base, entry("2024-12-30", december, pattern, start="2024-07-01", transition_sub=weeks([4, 4, 5, 4, 4, 5], [3, 3])))
    cal = checked_calendar(defn)
    cases = [validate_case(defn), case(cal, "years", {"from": 2022, "to": 2026})]
    cases += structure_cases(cal, (2024,))
    cases += locate_cases(cal, ["2024-06-30", "2024-07-01", "2024-12-29", "2024-12-30", "2025-12-28", "2025-12-29"])
    nav = [
        ("children", {"period": coords("period", 2024, 6), "level": "week"}),
        ("parent", {"period": coords("week", 2024, 26)}),
        ("parent", {"period": coords("week", 2023, 1)}),
        ("shift", {"period": coords("week", 2023, 53), "n": 1}),
        ("shift", {"period": coords("week", 2024, 26), "n": 1}),
        ("shift", {"period": coords("quarter", 2024, 2), "n": 1}),
        ("shift", {"period": coords("quarter", 2023, 4), "n": 1}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", 2024), coords("week", 2024, 26), coords("period", 2024, 6)])
    desc = (
        "A change from 1 July fixed-date years to week-based years ending on the last Sunday of December (weeks from Monday), "
        "through a 26-week transition from Monday 2024-07-01 to Sunday 2024-12-29 with a week_pattern of 4-4-5-4-4-5 weeks in "
        "two quarters. Numbered by 'majority': 2023, 2024 (the transition), 2025. In the transition, weeks nest in periods and "
        "Period.weeks is set; in the 1 July years before it a week's parent is the year. Illustrative."
    )
    return vector("history-fixed-to-week-based-week-pattern-transition", desc, oracle_source(CHECKED), defn, cases)


# 7. Week-based to fixed-date through a calendar_months transition -------------------------------------------


def week_based_to_fixed_vector() -> dict:
    base = week_based("end", 9, "last", "last", "sunday", [4, 4, 5] * 4, [3, 3, 3, 3], 1,
                      naming({"fy": scheme("start", {"year": "FY{yyyy}", "quarter": "FY{yyyy} Q{q}", "week": "FY{yyyy} W{ww}"})}))
    defn = with_history(base, entry("2024-01-01", *fixed_rule(1, 1), start="2023-10-01", transition_sub=months(1, [3])))
    cal = checked_calendar(defn)
    cases = [validate_case(defn), case(cal, "years", {"from": 2020, "to": 2025})]
    cases += year_cases(cal, range(2020, 2026))
    cases += structure_cases(cal, (2023,))
    cases.append(case(cal, "period", {"level": "quarter", "year": 2023, "index": 2}))
    cases += locate_cases(cal, ["2023-09-30", "2023-10-01", "2023-12-31", "2024-01-01"])
    nav = [
        ("children", {"period": coords("year", 2023), "level": "week"}),
        ("children", {"period": coords("quarter", 2023, 1), "level": "week"}),
        ("children", {"period": coords("quarter", 2022, 1), "level": "week"}),
        ("parent", {"period": coords("week", 2023, 14)}),
        ("parent", {"period": coords("week", 2022, 53)}),
        ("shift", {"period": coords("week", 2022, 53), "n": 1}),
        ("shift", {"period": coords("week", 2023, 14), "n": 1}),
        ("shift", {"period": coords("quarter", 2022, 4), "n": 2}),
        ("shift", {"period": coords("quarter", 2024, 1), "n": -2}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", 2022), coords("year", 2023), coords("quarter", 2023, 1), coords("week", 2022, 53), coords("week", 2023, 14)])
    desc = (
        "A change from week-based years ending on the last Saturday of September (weeks from Sunday; 2022, numbered by 'start', "
        "is the 53-week year 2022-09-25 to 2023-09-30) to calendar years, through a three-month transition 2023-10-01 to "
        "2023-12-31 with one quarter of three calendar months. Numbered by 'start': 2022, 2023 (the transition), 2024. The "
        "transition has 14 weeks, the last of them Sunday 31 December alone; its weeks do not nest in its quarter "
        "(LEVEL_NOT_NESTED) and their parent is the year, while weeks of 2022 nest in periods. Illustrative."
    )
    return vector("history-week-based-to-fixed-calendar-months-transition", desc, oracle_source(CHECKED), defn, cases)


# 8. Two history entries -----------------------------------------------------------------------------------


def two_changes_vector() -> dict:
    base = fixed_date(1, 1, "monday", naming({"fy": scheme("start", {"year": "FY{yyyy}", "quarter": "FY{yyyy} Q{q}"})}))
    defn = with_history(
        base,
        entry("2023-04-01", *fixed_rule(4, 1), start="2022-01-01", transition_sub=months(1, [3] * 5)),
        entry("2029-01-01", *fixed_rule(1, 1), start="2028-04-01", transition_sub=months(1, [3, 3, 3])),
    )
    cal = checked_calendar(defn)
    cases = [validate_case(defn), case(cal, "years", {"from": 2020, "to": 2030})]
    cases += structure_cases(cal, (2022, 2028), ("quarter",))
    cases += locate_cases(cal, ["2021-12-31", "2022-01-01", "2023-04-01", "2028-03-31", "2028-04-01", "2028-12-31", "2029-01-01"])
    nav = [
        ("shift", {"period": coords("year", 2021), "n": 8}),
        ("shift", {"period": coords("year", 2030), "n": -9}),
        ("shift", {"period": coords("quarter", 2021, 4), "n": 6}),
        ("shift", {"period": coords("quarter", 2027, 4), "n": 1}),
        ("shift", {"period": coords("quarter", 2027, 1), "n": 7}),
        ("shift", {"period": coords("period", 2028, 9), "n": 1}),
        ("shift", {"period": coords("period", 2029, 1), "n": -82}),
    ]
    cases += [case(cal, op, args) for op, args in nav]
    cases += label_cases(cal, [coords("year", 2022), coords("year", 2028), coords("quarter", 2028, 3)])
    cases.append(case(cal, "dimension", {"start": "2028-12-30", "end_exclusive": "2029-01-02"}))
    desc = (
        "Two history entries: calendar years until 2021, a fifteen-month transition 2022-01-01 to 2023-03-31, 1 April years "
        "from 2023 to 2027, a nine-month transition 2028-04-01 to 2028-12-31, then calendar years again from 2029. Numbered by "
        "'start', every number from 2020 to 2030 names one fiscal year. shift crosses both transitions, whose quarter counts "
        "(5 and 3) differ from the regular years'. Illustrative."
    )
    return vector("history-two-entries", desc, oracle_source(CHECKED), defn, cases)


# 9. Canonical form and calendar_id ---------------------------------------------------------------------

CANONICAL_HISTORY = (
    '{"exceptions":{"history":[{"from":"2025-01-01","subdivision":{"month_start_day":1,"quarters":[4,4,4],"type":"calendar_months"},'
    '"transition":null,"year":{"short_month":null,"start":{"day":1,"month":1},"type":"fixed_date"}}]},'
    '"naming":{"default":"fy","schemes":{"fy":{"century_pivot":null,"offset":0,"templates":{"quarter":"FY{yyyy} Q{q}","year":"FY{yyyy}"},'
    '"year_number":"start"}}},"spec_version":1,"subdivision":{"month_start_day":1,"quarters":[3,3,3,3],"type":"calendar_months"},'
    '"week":{"numbering":"contains_start","starts_on":"monday"},"year":{"short_month":null,"start":{"day":1,"month":1},"type":"fixed_date"}}'
)


def canonical_vectors() -> list[dict]:
    jan_1_calendar_id = calendar_id(calendar_id_vectors()[0]["definition"])
    a = quarters_change_definition()
    ca = validate_case(a)
    assert ca["expect"]["canonical"] == CANONICAL_HISTORY, ca["expect"]["canonical"]
    assert ca["expect"]["calendar_id"] == hashlib.sha256(CANONICAL_HISTORY.encode("utf-8")).hexdigest()[:16]
    b = {
        "exceptions": {"history": [{"year": {"type": "fixed_date", "start": {"day": 1, "month": 1}, "short_month": None}, "transition": None,
                                    "subdivision": {"type": "calendar_months", "quarters": [4, 4, 4], "month_start_day": 1}, "from": "2025-01-01"}]},
        "naming": {"schemes": {"fy": {"templates": {"quarter": "FY{yyyy} Q{q}", "year": "FY{yyyy}"}, "offset": 0, "year_number": "start", "century_pivot": None}}, "default": "fy"},
        "$schema": "https://example.invalid/fiscalclock/definition-v1.json",
        "week": {"starts_on": "monday", "numbering": "contains_start"},
        "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
        "year": {"type": "fixed_date", "start": {"month": 1, "day": 1}},
        "spec_version": 1,
    }
    cb = validate_case(b)
    assert cb["expect"] == ca["expect"]
    cid = ca["expect"]["calendar_id"]
    desc = (
        "{} of a pair of definitions with exceptions.history that differ only in key order, $schema, and the history entry's "
        "transition and short_month written as null or left out. Both have calendar_id {}. The canonical string is typed into "
        "history_vectors.CANONICAL_HISTORY by hand: exceptions sorts with the other top-level keys, the entry keeps 'transition': "
        "null and 'short_month': null, and calendar_id is SHA-256 of its UTF-8 bytes, first 16 hex digits."
    )
    src = oracle_source("Canonical string typed by hand in history_vectors.CANONICAL_HISTORY; fiscal_oracle.canonical agrees (asserted)")
    plain = fixed_date(1, 1, "monday", naming({"cal": scheme("start", {"year": "{yyyy}"})}))
    plain["exceptions"] = None
    cn = validate_case(plain)
    assert cn["expect"]["calendar_id"] == jan_1_calendar_id, (cn, jan_1_calendar_id)
    assert "exceptions" not in cn["expect"]["canonical"]
    null_desc = (
        f"A definition with 'exceptions': null, which means no exceptions. The canonical form leaves exceptions out, so calendar_id "
        f"is {jan_1_calendar_id}, the same as calendar-id-pair-a, which has no exceptions field (asserted by the generator)."
    )
    return [
        vector("history-calendar-id-pair-a", desc.format("First", cid), src, a, [ca]),
        vector("history-calendar-id-pair-b", desc.format("Second", cid), src, b, [cb]),
        vector("history-exceptions-null", null_desc, oracle_source("fiscal_oracle.canonical; equality with calendar-id-pair-a asserted"), plain, [cn]),
    ]


def build() -> list[dict]:
    return [
        jul_to_apr_vector(),
        *oct_to_apr_vectors(),
        jan_to_apr_vector(),
        quarters_change_vector(),
        nrf_to_december_vector(),
        fixed_to_week_based_vector(),
        fixed_to_week_based_transition_vector(),
        week_based_to_fixed_vector(),
        two_changes_vector(),
        *canonical_vectors(),
    ]

