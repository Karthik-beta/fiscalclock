"""Spec vectors (worked examples transcribed from the spec) and cited vectors (verbatim quotes from published sources).

Expected values here are typed in from the source text, not computed. The builder asserts that
fiscal_oracle.py agrees, so a disagreement between source and oracle stops generation.
"""

from __future__ import annotations

import copy

from fiscal_oracle import Calendar
from oracle_vectors import apple_definition, matches_subset, nrf_definition
from vectorlib import naming, run, scheme, source, validate_case, vector, week_based

SPEC_CHECK = "Transcribed from the spec text 2026-10-31; conformance/generators/fiscal_oracle.py agrees (asserted by the generator)"


def agree(cal: Calendar, c: dict) -> dict:
    got = run(cal, c["op"], c["args"])
    if "expect" in c:
        assert got == c["expect"], (c, got)
    else:
        assert matches_subset(got, c["expect_subset"]), (c, got)
    return c


def spec_vectors() -> list[dict]:
    nrf = nrf_definition()
    cal = Calendar(nrf)
    period_example = {
        "op": "period",
        "args": {"level": "quarter", "year": 2023, "index": 4},
        "expect": {"level": "quarter", "year": 2023, "index": 4, "start": "2023-10-29", "end_exclusive": "2024-02-04", "last_day": "2024-02-03", "days": 98, "weeks": 14, "has_extra_week": True},
    }
    location_example = {
        "op": "locate",
        "args": {"date": "2024-02-03"},
        "expect_subset": {"date": "2024-02-03", "day_of_year": 371, "day_of_quarter": 98, "day_of_period": 35, "day_of_week": 7},
    }
    readme_example = {
        "op": "period",
        "args": {"level": "year", "year": 2023},
        "expect": {"level": "year", "year": 2023, "index": None, "start": "2023-01-29", "end_exclusive": "2024-02-04", "last_day": "2024-02-03", "days": 371, "weeks": 53, "has_extra_week": True},
    }
    bad = copy.deepcopy(nrf)
    bad["subdivision"]["periods"] = [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 4, 4]
    infer = (
        "The spec does not print the definition behind this example; it is read as the NRF 4-5-4 calendar from spec/definition-v1.md "
        "(Saturday nearest 31 January, weeks start Sunday) with periods 4-5-4, quarters of 3 periods, extra week in period 12, named by start year."
    )
    return [
        vector("api-period-example", "The Period example in spec/api-v1.md (fiscal 2023 quarter 4). " + infer,
               source("spec", "spec/api-v1.md, Values > Period", SPEC_CHECK), nrf, [validate_case(nrf), agree(cal, period_example)]),
        vector("api-location-example", "The Location example in spec/api-v1.md (2024-02-03, the last day of a 53-week year). Only the fields the example prints are asserted. " + infer,
               source("spec", "spec/api-v1.md, Values > Location", SPEC_CHECK), nrf, [validate_case(nrf), agree(cal, location_example)]),
        vector("readme-vector-example", "The example case in conformance/README.md (NRF fiscal 2023 year bounds). " + infer,
               source("spec", "conformance/README.md, Vector file", SPEC_CHECK), nrf, [validate_case(nrf), agree(cal, readme_example)]),
        vector("api-error-example", "The Error example in spec/api-v1.md: periods summing to 51 weeks give WEEKS_SUM_MISMATCH at subdivision.periods.",
               source("spec", "spec/api-v1.md, Values > Error", "Transcribed from the spec text 2026-10-31"), bad,
               [{"op": "validate", "args": {}, "expect_error": {"code": "WEEKS_SUM_MISMATCH", "path": "subdivision.periods"}}]),
    ]


NRF_URL = "https://nrf.com/resources/4-5-4-calendar"
APPLE_URL = "https://www.sec.gov/Archives/edgar/data/320193/000032019324000006/aapl-20231230.htm"
FETCHED = "Quotes compared character for character with the fetched page text (HTML tags removed, entities decoded, whitespace collapsed) on 2026-10-31"
KROGER_URL = "https://s202.q4cdn.com/463742399/files/doc_financials/2018/ar/2018-Annual-Report.pdf"
COSTCO_URL = "https://s201.q4cdn.com/287523651/files/doc_financials/2017/ar/2017-Annual-Report-Final-(2).pdf"
PDF_CHECKED = (
    "Quotes compared character for character with the text that pdftotext -layout -enc UTF-8 extracted from the PDF "
    "(line breaks joined) on 2026-11-01; see research/filers/"
)


def kroger_definition() -> dict:
    templates = {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}
    return week_based("end", 1, "last", "nearest", "sunday", [4] * 13, [4, 3, 3, 3], 13, naming({"fy": scheme("start", templates)}))


def costco_definition() -> dict:
    templates = {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}
    return week_based("end", 8, "last", "nearest", "monday", [4] * 13, [3, 3, 3, 4], 13, naming({"fy": scheme("end", templates)}))


def kroger_vector() -> dict:
    defn = kroger_definition()
    cal = Calendar(defn)
    year = {"level": "year"}
    cases = [validate_case(defn)] + [agree(cal, c) for c in (
        {"op": "period", "args": {**year, "year": 2018}, "expect_subset": {"last_day": "2019-02-02", "weeks": 52, "has_extra_week": False}},
        {"op": "period", "args": {**year, "year": 2017}, "expect_subset": {"last_day": "2018-02-03", "weeks": 53, "has_extra_week": True}},
        {"op": "period", "args": {**year, "year": 2016}, "expect_subset": {"last_day": "2017-01-28", "weeks": 52, "has_extra_week": False}},
        {"op": "period", "args": {"level": "quarter", "year": 2018, "index": 1}, "expect_subset": {"weeks": 16}},
        {"op": "period", "args": {"level": "quarter", "year": 2018, "index": 2}, "expect_subset": {"weeks": 12}},
        {"op": "period", "args": {"level": "quarter", "year": 2018, "index": 3}, "expect_subset": {"weeks": 12}},
        {"op": "period", "args": {"level": "quarter", "year": 2018, "index": 4}, "expect_subset": {"start": "2018-11-11", "last_day": "2019-02-02", "weeks": 12}},
        {"op": "period", "args": {"level": "quarter", "year": 2017, "index": 1}, "expect_subset": {"weeks": 16}},
        {"op": "period", "args": {"level": "quarter", "year": 2017, "index": 2}, "expect_subset": {"weeks": 12}},
        {"op": "period", "args": {"level": "quarter", "year": 2017, "index": 3}, "expect_subset": {"weeks": 12}},
        {"op": "period", "args": {"level": "quarter", "year": 2017, "index": 4}, "expect_subset": {"weeks": 13}},
        {"op": "period", "args": {"level": "period", "year": 2018, "index": 11}, "expect_subset": {"start": "2018-11-11", "last_day": "2018-12-08", "weeks": 4}},
        {"op": "period", "args": {"level": "period", "year": 2018, "index": 12}, "expect_subset": {"start": "2018-12-09", "last_day": "2019-01-05", "weeks": 4}},
        {"op": "period", "args": {"level": "period", "year": 2018, "index": 13}, "expect_subset": {"start": "2019-01-06", "last_day": "2019-02-02", "weeks": 4}},
    )]
    citation = (
        "\"Kroger’s fiscal year ends on the Saturday closest to January 31.\" (p. A-3) "
        "\"The reported periods conform to our fiscal calendar composed of thirteen 28-day periods. The fourth quarter of 2018 contained three 28-day periods.\" (p. A-4) "
        "\"First period - four weeks November 11, 2018 to December 8, 2018\", \"Second period - four weeks December 9, 2018 to January 5, 2019\" and "
        "\"January 6, 2019 to February 2, 2019\" (third period; Issuer Purchases of Equity Securities table, p. A-4)."
        "\"All references to 2018, 2017 and 2016 are to the fiscal years ended February 2, 2019, February 3, 2018 and January 28, 2017, respectively, unless specifically indicated otherwise.\" (p. A-5) "
        "\"The Company’s fiscal year ends on the Saturday nearest January 31. The last three fiscal years consist of the 52-week period ended February 2, 2019, "
        "53-week period ended February 3, 2018 and 52-week period ended January 28, 2017.\" (Note 1, p. A-35) "
        "Note 21, Quarterly Data (pp. A-77 and A-78): 2018 quarters of 16, 12, 12 and 12 weeks (52 weeks); 2017 quarters of 16, 12, 12 and 13 weeks (53 weeks). "
        "Establishes the year-end rule, start-year naming, thirteen four-week periods, the three fiscal year ends and week counts, the quarter week counts, "
        "and the start and last day of the three fourth-quarter periods of 2018 (periods 11 to 13, because the 16- and 12-week quarters before them hold ten four-week periods)."
    )
    return vector(
        "kroger-annual-report-2018",
        "The Kroger Co. 2019 Proxy Statement and 2018 Annual Report (fiscal year ended 2 February 2019), which reproduces the Form 10-K text. "
        "The extra week's period (13) and the label templates are illustrative; only fields the document states are asserted.",
        source("cited", citation, PDF_CHECKED, KROGER_URL), defn, cases,
    )


def costco_vector() -> dict:
    defn = costco_definition()
    cal = Calendar(defn)
    year = {"level": "year"}
    quarter_weeks = {2017: (12, 12, 12, 17), 2016: (12, 12, 12, 16)}
    cases = [validate_case(defn)] + [agree(cal, c) for c in (
        {"op": "period", "args": {**year, "year": 2017}, "expect_subset": {"last_day": "2017-09-03", "weeks": 53, "has_extra_week": True}},
        {"op": "period", "args": {**year, "year": 2016}, "expect_subset": {"last_day": "2016-08-28", "weeks": 52, "has_extra_week": False}},
        {"op": "period", "args": {**year, "year": 2015}, "expect_subset": {"last_day": "2015-08-30", "weeks": 52, "has_extra_week": False}},
        *({"op": "period", "args": {"level": "quarter", "year": y, "index": i + 1}, "expect_subset": {"weeks": w}}
          for y, ws in quarter_weeks.items() for i, w in enumerate(ws)),
        {"op": "period", "args": {"level": "period", "year": 2017, "index": 13}, "expect_subset": {"weeks": 5, "has_extra_week": True}},
        {"op": "period", "args": {"level": "period", "year": 2016, "index": 13}, "expect_subset": {"weeks": 4, "has_extra_week": False}},
    )]
    citation = (
        "\"We report on a 52/53-week fiscal year, consisting of thirteen, four-week periods and ending on the Sunday nearest the end of August. "
        "The first three quarters consist of three periods each, and the fourth quarter consists of four periods (five weeks in the thirteenth period in a 53-week year).\" "
        "\"References to 2017 relate to the 53-week fiscal year ended September 3, 2017. References to 2016 and 2015 relate to the 52-week fiscal years ended "
        "August 28, 2016, and August 30, 2015, respectively.\" (Business Overview, p. 6) "
        "\"The Company operates on a 52/53 week fiscal year basis with the fiscal year ending on the Sunday closest to August 31.\" (Note 1, p. 44) "
        "Note 12, Quarterly Financial Data (pp. 65 and 66): \"53 Weeks Ended September 3, 2017\" with quarters of 12, 12, 12 and 17 weeks; "
        "\"52 Weeks Ended August 28, 2016\" with quarters of 12, 12, 12 and 16 weeks. "
        "Establishes the year-end rule, end-year naming, thirteen four-week periods in quarters of 3, 3, 3 and 4 periods, the extra week as the thirteenth period's fifth week, "
        "the three fiscal year ends and week counts, and the quarter week counts."
    )
    return vector(
        "costco-annual-report-2017",
        "Costco Wholesale Corporation 2017 Annual Report (fiscal year ended 3 September 2017), which reproduces the Form 10-K text. "
        "The label templates are illustrative; only fields the document states are asserted.",
        source("cited", citation, PDF_CHECKED, COSTCO_URL), defn, cases,
    )


def cited_vectors() -> list[dict]:
    nrf = nrf_definition()
    cal = Calendar(nrf)
    nrf_cases = [validate_case(nrf)]
    for y in (2006, 2012, 2017, 2023):
        nrf_cases.append(agree(cal, {"op": "period", "args": {"level": "year", "year": y}, "expect_subset": {"weeks": 53, "has_extra_week": True}}))
    nrf_citation = (
        "\"2006, 2012, 2017 and 2023 are all 53-week years.\" and \"This anomaly has most recently occurred in FY12, FY17 and FY23.\" "
        "Establishes that NRF fiscal years 2006, 2012, 2017 and 2023 have 53 weeks."
    )
    apple = apple_definition()
    cal = Calendar(apple)
    apple_cases = [validate_case(apple)] + [agree(cal, c) for c in (
        {"op": "period", "args": {"level": "year", "year": 2023}, "expect_subset": {"weeks": 53, "has_extra_week": True, "last_day": "2023-09-30"}},
        {"op": "period", "args": {"level": "year", "year": 2024}, "expect_subset": {"weeks": 52, "has_extra_week": False}},
        {"op": "period", "args": {"level": "quarter", "year": 2024, "index": 1}, "expect_subset": {"weeks": 13, "last_day": "2023-12-30"}},
        {"op": "period", "args": {"level": "quarter", "year": 2023, "index": 1}, "expect_subset": {"weeks": 14, "has_extra_week": True}},
    )]
    apple_citation = (
        "\"The Company’s fiscal year is the 52- or 53-week period that ends on the last Saturday of September.\" "
        "\"The Company’s fiscal years 2024 and 2023 span 52 and 53 weeks, respectively.\" "
        "\"The Company’s first quarter of 2024 included 13 weeks, compared to 14 weeks during the first quarter of 2023.\" "
        "\"For the quarterly period ended December 30, 2023\" (cover page). "
        "\"...included in its Annual Report on Form 10-K for the fiscal year ended September 30, 2023\". "
        "Establishes the year-end rule, 53 weeks in fiscal 2023 and 52 in fiscal 2024, a 14-week first quarter in 2023 and a 13-week "
        "first quarter of 2024 ending 30 December 2023, and fiscal 2023 ending 30 September 2023."
    )
    return [
        vector("nrf-53-week-years", "NRF's published list of 53-week fiscal years, checked against the NRF 4-5-4 definition.",
               source("cited", nrf_citation, FETCHED, NRF_URL), nrf, nrf_cases),
        vector("apple-10q-2023-12-30", "Apple Inc. Form 10-Q for the quarter ended 30 December 2023. The definition's period pattern is illustrative; only fields the filing states are asserted.",
               source("cited", apple_citation, FETCHED, APPLE_URL), apple, apple_cases),
        kroger_vector(),
        costco_vector(),
    ]
