"""Smoke test for an installed fiscalclock wheel. Run with ``python -I scripts/smoke.py``."""

import datetime as dt

import fiscalclock

NRF = {
    "spec_version": 1,
    "year": {"type": "week_based", "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
    "week": {"starts_on": "sunday", "numbering": "contains_start"},
    "subdivision": {"type": "week_pattern", "periods": [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4],
                    "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}},
    "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start",
               "templates": {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}}}},
}

cal = fiscalclock.FiscalCalendar(NRF)
fy = cal.period("year", 2023)
assert (fy.start, fy.last_day, fy.weeks) == (dt.date(2023, 1, 29), dt.date(2024, 2, 3), 53), fy
assert cal.format(cal.locate(dt.date(2024, 2, 3)).quarter) == "Q4 FY2023"
try:
    cal.locate(dt.datetime(2024, 2, 3))
except fiscalclock.FiscalclockError as e:
    assert e.code == "TIMESTAMP_REJECTED", e.code
else:
    raise AssertionError("a datetime was accepted")
print(f"fiscalclock {fiscalclock.__version__} from {fiscalclock.__file__}: ok")
