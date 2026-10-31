"""Spec vectors (worked examples transcribed from the spec) and cited vectors (verbatim quotes from published sources).

Expected values here are typed in from the source text, not computed. The builder asserts that
fiscal_oracle.py agrees, so a disagreement between source and oracle stops generation.
"""

from __future__ import annotations

import copy

from fiscal_oracle import Calendar
from oracle_vectors import apple_definition, matches_subset, nrf_definition
from vectorlib import run, source, validate_case, vector

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
    ]
