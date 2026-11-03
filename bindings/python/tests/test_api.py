"""Tests for the Python layer: type conversion, errors and timestamp rejection."""

import copy
import dataclasses
import datetime as dt
import decimal
import json
import pickle
import unittest
from pathlib import Path

import fiscalclock
from fiscalclock import FiscalCalendar, FiscalclockError, Location, Period

EXAMPLES = Path(__file__).resolve().parents[3] / "docs" / "examples"

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

    def test_period_as_a_mapping(self):
        q4 = self.cal.period("quarter", 2023, 4)
        coordinates = {"level": "quarter", "year": 2023, "index": 4}
        self.assertEqual(self.cal.shift(coordinates, 1), self.cal.shift(q4, 1))
        self.assertEqual(self.cal.parent(coordinates), self.cal.parent(q4))
        self.assertEqual(self.cal.children(coordinates, "period"), self.cal.children(q4, "period"))
        self.assertEqual(self.cal.format(coordinates), "Q4 FY2023")
        self.assertEqual(self.cal.format({"level": "year", "year": 2023}), "FY2023")

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

    def test_several_validation_errors(self):
        bad = json.loads(json.dumps(NRF))
        bad["subdivision"]["periods"] = [4] * 12
        bad["week"]["starts_on"] = "sun"
        with self.assertRaises(FiscalclockError) as cm:
            FiscalCalendar(bad)
        self.assertEqual(
            [(e["code"], e["path"]) for e in cm.exception.errors],
            [("WEEKS_SUM_MISMATCH", "subdivision.periods"), ("INVALID_VALUE", "week.starts_on")],
        )
        self.assertEqual((cm.exception.code, cm.exception.path), ("WEEKS_SUM_MISMATCH", "subdivision.periods"))
        self.assertTrue(str(cm.exception).endswith("; and 1 more"), str(cm.exception))

    def test_validate_does_not_raise(self):
        self.assertTrue(fiscalclock.validate(NRF)["ok"])

    def test_validate_json_returns_core_json_unchanged(self):
        bad = json.loads(json.dumps(NRF))
        bad["subdivision"]["periods"] = [4] * 12
        for definition in [json.dumps(NRF), json.dumps(bad), "{"]:
            self.assertEqual(fiscalclock.validate_json(definition), fiscalclock.call_json(definition, "validate", "{}"))

    def test_ambiguous_label(self):
        cal = FiscalCalendar((EXAMPLES / "two-schemes.json").read_text(encoding="utf-8"))
        with self.assertRaises(FiscalclockError) as cm:
            cal.parse("FY2023")
        self.assertEqual((cm.exception.code, cm.exception.path), ("AMBIGUOUS_LABEL", "label"))
        candidates = cm.exception.details["candidates"]
        self.assertEqual([(c["scheme"], c["level"], c["period"]["year"]) for c in candidates], [("by_end", "year", 2022), ("by_start", "year", 2023)])
        raw = json.loads(cal.call_json("parse", '{"label":"FY2023"}'))
        self.assertEqual(cm.exception.details, raw["errors"][0]["details"])

    def test_wrong_argument_types(self):
        cal = FiscalCalendar(NRF)
        q4 = cal.period("quarter", 2023, 4)
        cases = [
            (lambda: cal.period("quarter", "2023", 4), "year"),
            (lambda: cal.years("2022", 2024), "from"),
            (lambda: cal.shift(q4, "1"), "n"),
            (lambda: cal.parent(None), "period"),
            (lambda: cal.parse(2023), "label"),
        ]
        for call, path in cases:
            with self.assertRaises(FiscalclockError) as cm:
                call()
            self.assertEqual((cm.exception.code, cm.exception.path), ("INVALID_ARGUMENT", path))

    def test_operation_error(self):
        with self.assertRaises(FiscalclockError) as cm:
            FiscalCalendar(NRF).locate("2024-02-30")
        self.assertEqual((cm.exception.code, cm.exception.path), ("INVALID_DATE", "date"))

    def test_error_is_a_value_error(self):
        self.assertTrue(issubclass(FiscalclockError, ValueError))

    def test_error_survives_pickle_and_copy(self):
        with self.assertRaises(FiscalclockError) as cm:
            FiscalCalendar(NRF).locate("2024-02-30")
        original = cm.exception
        for clone in [pickle.loads(pickle.dumps(original)), copy.copy(original), copy.deepcopy(original)]:
            self.assertIsInstance(clone, FiscalclockError)
            self.assertEqual(clone.errors, original.errors)
            self.assertEqual((clone.code, clone.path, clone.message, clone.hint), (original.code, original.path, original.message, original.hint))
            self.assertEqual(str(clone), str(original))


class ValuesWithoutAJsonForm(unittest.TestCase):
    """Values JSON cannot carry still give a coded error, with the same code and path as in JS."""

    def setUp(self):
        self.cal = FiscalCalendar(NRF)
        self.q4 = self.cal.period("quarter", 2023, 4)

    def assert_error(self, call, code, path):
        with self.assertRaises(FiscalclockError) as cm:
            call()
        self.assertEqual((cm.exception.code, cm.exception.path), (code, path))

    def test_unsupported_year_and_n(self):
        class Thing:
            pass

        for value in [decimal.Decimal(2023), Thing(), b"2023"]:
            self.assert_error(lambda: self.cal.period("quarter", value, 4), "INVALID_ARGUMENT", "year")
            self.assert_error(lambda: self.cal.shift(self.q4, value), "INVALID_ARGUMENT", "n")

    def test_numpy_integers(self):
        try:
            import numpy
        except ImportError:
            self.skipTest("numpy is not installed")
        self.assert_error(lambda: self.cal.period("quarter", numpy.int64(2023), 4), "INVALID_ARGUMENT", "year")

    def test_period_mapping_with_date_values(self):
        self.assertEqual(self.cal.shift(dataclasses.asdict(self.q4), 1), self.cal.shift(self.q4, 1))

    def test_period_that_is_not_a_mapping(self):
        for value in [None, ("quarter", 2023, 4), "Q4 FY2023", 7]:
            self.assert_error(lambda: self.cal.shift(value, 1), "INVALID_ARGUMENT", "period")

    def test_definition_without_a_json_form(self):
        for value in [json.dumps(NRF).encode(), {1, 2}, decimal.Decimal(1)]:
            self.assert_error(lambda: FiscalCalendar(value), "INVALID_JSON", "")
            self.assertEqual(fiscalclock.validate(value)["errors"][0]["code"], "INVALID_JSON")

    def test_definition_with_an_unsupported_value(self):
        self.assert_error(lambda: FiscalCalendar({**NRF, "spec_version": decimal.Decimal(1)}), "INVALID_TYPE", "spec_version")

    def test_circular_definition(self):
        cyclic = json.loads(json.dumps(NRF))
        cyclic["self"] = cyclic
        self.assert_error(lambda: FiscalCalendar(cyclic), "INVALID_JSON", "")

    def test_lone_surrogates(self):
        self.assert_error(lambda: self.cal.parse("\ud800"), "INVALID_ARGUMENT", "")
        self.assert_error(lambda: FiscalCalendar({**NRF, "spec_version": "\udfff"}), "INVALID_JSON", "")

    def test_non_ascii_text_reaches_the_core_unchanged(self):
        definition = json.loads(json.dumps(NRF))
        definition["naming"]["schemes"]["fy"]["templates"] = {"year": "Geschäftsjahr {yyyy}", "quarter": "第{q}四半期 {yyyy} 📅"}
        cal = FiscalCalendar(definition)
        q4 = cal.period("quarter", 2023, 4)
        label = cal.format(q4)
        self.assertEqual(label, "第4四半期 2023 📅")
        self.assertEqual(cal.parse(label), q4)
        for text in ["Geschäftsjahr 99999", "第9四半期 2023 📅"]:
            with self.assertRaises(FiscalclockError) as cm:
                cal.parse(text)
            raw = cal.call_json("parse", json.dumps({"label": text}, ensure_ascii=False))
            self.assertEqual(cm.exception.errors, json.loads(raw)["errors"])


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
        expected = [
            {
                "code": "TIMESTAMP_REJECTED",
                "path": "date",
                "message": "a timestamp was passed where a civil date (year, month, day) is expected",
                "hint": "convert the timestamp to a date in the time zone you mean, then pass that date",
                "details": None,
            }
        ]
        self.assertEqual(cm.exception.errors, expected)

    def test_date_subclass_that_is_not_a_datetime_is_not_accepted(self):
        class MyDate(dt.date):
            pass

        with self.assertRaises(FiscalclockError) as cm:
            self.cal.locate(MyDate(2024, 2, 3))
        self.assertEqual((cm.exception.code, cm.exception.path), ("INVALID_ARGUMENT", "date"))


if __name__ == "__main__":
    unittest.main()
