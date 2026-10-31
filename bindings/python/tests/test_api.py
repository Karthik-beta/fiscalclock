"""Tests for the Python layer: type conversion, errors and timestamp rejection."""

import datetime as dt
import json
import unittest

import fiscalclock
from fiscalclock import FiscalCalendar, FiscalclockError, Location, Period

NRF = {
    "spec_version": 1,
    "year": {"type": "week_based", "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
    "week": {"starts_on": "sunday", "numbering": "contains_start"},
    "subdivision": {
        "type": "week_pattern",
        "periods": [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4],
        "quarters": [3, 3, 3, 3],
        "extra_week": {"period": 12},
    },
    "naming": {
        "default": "fy",
        "schemes": {"fy": {"year_number": "start", "templates": {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}}},
    },
}


class Basics(unittest.TestCase):
    def setUp(self):
        self.cal = FiscalCalendar(NRF)

    def test_mapping_and_text_give_the_same_calendar(self):
        self.assertEqual(FiscalCalendar(json.dumps(NRF)).calendar_id, self.cal.calendar_id)

    def test_period_has_native_dates(self):
        fy = self.cal.period("year", 2023)
        self.assertIsInstance(fy, Period)
        self.assertEqual(fy.start, dt.date(2023, 1, 29))
        self.assertEqual(fy.end_exclusive, dt.date(2024, 2, 4))
        self.assertEqual(fy.last_day, dt.date(2024, 2, 3))
        self.assertEqual((fy.days, fy.weeks, fy.has_extra_week), (371, 53, True))

    def test_locate_format_parse(self):
        here = self.cal.locate(dt.date(2024, 2, 3))
        self.assertIsInstance(here, Location)
        self.assertEqual(self.cal.format(here.quarter), "Q4 FY2023")
        self.assertEqual(self.cal.parse("Q4 FY2023"), here.quarter)
        self.assertEqual(self.cal.locate("2024-02-03"), here)

    def test_navigation(self):
        q4 = self.cal.period("quarter", 2023, 4)
        self.assertEqual(self.cal.shift(q4, 1), self.cal.period("quarter", 2024, 1))
        self.assertEqual(self.cal.parent(q4), self.cal.period("year", 2023))
        self.assertEqual(len(self.cal.children(q4, "period")), 3)
        self.assertEqual([y.year for y in self.cal.years(2022, 2024)], [2022, 2023, 2024])

    def test_call_json_returns_core_json_unchanged(self):
        args = '{"level":"quarter","year":2023,"index":4}'
        expected = (
            '{"level":"quarter","year":2023,"index":4,"start":"2023-10-29","end_exclusive":"2024-02-04",'
            '"last_day":"2024-02-03","days":98,"weeks":14,"has_extra_week":true}'
        )
        self.assertEqual(self.cal.call_json("period", args), expected)
        self.assertEqual(fiscalclock.call_json(json.dumps(NRF), "period", args), expected)

    def test_dimension(self):
        rows = self.cal.dimension(dt.date(2024, 2, 3), dt.date(2024, 2, 5))
        self.assertEqual([r["date"] for r in rows], ["2024-02-03", "2024-02-04"])
        self.assertEqual([r["fiscal_year"] for r in rows], [2023, 2024])
        csv = self.cal.dimension_csv("2024-02-03", "2024-02-05")
        self.assertEqual(len(csv.splitlines()), 3)

    def test_explain(self):
        out = self.cal.explain(dt.date(2024, 2, 3))
        self.assertEqual(out["calendar_id"], self.cal.calendar_id)


class Errors(unittest.TestCase):
    def test_invalid_definition_raises_with_every_error(self):
        bad = json.loads(json.dumps(NRF))
        bad["subdivision"]["periods"] = [4] * 12
        with self.assertRaises(FiscalclockError) as cm:
            FiscalCalendar(bad)
        self.assertEqual(cm.exception.code, "WEEKS_SUM_MISMATCH")
        self.assertEqual(cm.exception.path, "subdivision.periods")
        self.assertEqual(fiscalclock.validate(bad)["errors"], cm.exception.errors)

    def test_validate_does_not_raise(self):
        self.assertTrue(fiscalclock.validate(NRF)["ok"])

    def test_operation_error(self):
        with self.assertRaises(FiscalclockError) as cm:
            FiscalCalendar(NRF).locate("2024-02-30")
        self.assertEqual((cm.exception.code, cm.exception.path), ("INVALID_DATE", "date"))

    def test_error_is_a_value_error(self):
        self.assertTrue(issubclass(FiscalclockError, ValueError))


class TimestampsAreRejected(unittest.TestCase):
    def setUp(self):
        self.cal = FiscalCalendar(NRF)

    def assert_rejected(self, call, path):
        with self.assertRaises(FiscalclockError) as cm:
            call()
        self.assertEqual((cm.exception.code, cm.exception.path), ("TIMESTAMP_REJECTED", path))
        self.assertIsNotNone(cm.exception.hint)

    def test_naive_and_aware_datetimes(self):
        for value in [dt.datetime(2024, 2, 3), dt.datetime(2024, 2, 3, 23, 30, tzinfo=dt.timezone.utc)]:
            self.assert_rejected(lambda: self.cal.locate(value), "date")
            self.assert_rejected(lambda: self.cal.explain(value), "date")
            self.assert_rejected(lambda: self.cal.dimension(value, "2024-03-01"), "start")
            self.assert_rejected(lambda: self.cal.dimension_csv("2024-01-01", value), "end_exclusive")

    def test_datetime_subclass(self):
        class Stamp(dt.datetime):
            pass

        self.assert_rejected(lambda: self.cal.locate(Stamp(2024, 2, 3)), "date")

    def test_pandas_timestamp(self):
        try:
            import pandas
        except ImportError:
            self.skipTest("pandas is not installed")
        self.assert_rejected(lambda: self.cal.locate(pandas.Timestamp("2024-02-03")), "date")

    def test_error_bytes_come_from_the_core(self):
        with self.assertRaises(FiscalclockError) as cm:
            self.cal.locate(dt.datetime(2024, 2, 3))
        expected = json.loads(fiscalclock._fiscalclock.timestamp_rejected("date"))["errors"]
        self.assertEqual(cm.exception.errors, expected)

    def test_date_subclass_that_is_not_a_datetime_is_not_accepted(self):
        class MyDate(dt.date):
            pass

        with self.assertRaises(FiscalclockError) as cm:
            self.cal.locate(MyDate(2024, 2, 3))
        self.assertIn(cm.exception.code, {"INVALID_DATE", "INVALID_ARGUMENT"})


if __name__ == "__main__":
    unittest.main()
