// Tests for the TypeScript layer: conversion, errors and timestamp rejection.

import assert from "node:assert/strict";
import { test } from "node:test";
import vm from "node:vm";

import { FiscalCalendar, FiscalclockError, callJson, coreVersion, validate, validateJson } from "../dist/node.js";

const NRF = {
  spec_version: 1,
  year: { type: "week_based", anchor: { edge: "end", month: 1, day: "last", rule: "nearest" } },
  week: { starts_on: "sunday", numbering: "contains_start" },
  subdivision: { type: "week_pattern", periods: [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4], quarters: [3, 3, 3, 3], extra_week: { period: 12 } },
  naming: { default: "fy", schemes: { fy: { year_number: "start", templates: { year: "FY{yyyy}", quarter: "Q{q} FY{yyyy}" } } } },
};

const cal = new FiscalCalendar(NRF);

function plainDate(iso, tag = "Temporal.PlainDate") {
  return { [Symbol.toStringTag]: tag, toString: () => iso };
}

function assertError(fn, code, path) {
  assert.throws(fn, (e) => {
    assert.ok(e instanceof FiscalclockError, String(e));
    assert.equal(e.code, code);
    assert.equal(e.path, path);
    return true;
  });
}

test("object and text definitions give the same calendar", () => {
  assert.equal(new FiscalCalendar(JSON.stringify(NRF)).calendarId, cal.calendarId);
  assert.equal(coreVersion(), "0.1.0");
});

test("period, locate, format and parse", () => {
  const fy = cal.period("year", 2023);
  assert.deepEqual(
    [fy.start, fy.end_exclusive, fy.last_day, fy.days, fy.weeks, fy.has_extra_week],
    ["2023-01-29", "2024-02-04", "2024-02-03", 371, 53, true],
  );
  const here = cal.locate("2024-02-03");
  assert.equal(cal.format(here.quarter), "Q4 FY2023");
  assert.deepEqual(cal.parse("Q4 FY2023"), here.quarter);
  assert.deepEqual(cal.locate(plainDate("2024-02-03")), here);
});

test("navigation", () => {
  const q4 = cal.period("quarter", 2023, 4);
  assert.deepEqual(cal.shift(q4, 1), cal.period("quarter", 2024, 1));
  assert.deepEqual(cal.parent(q4), cal.period("year", 2023));
  assert.equal(cal.children(q4, "period").length, 3);
  assert.deepEqual(cal.years(2022, 2024).map((y) => y.year), [2022, 2023, 2024]);
});

test("callJson returns the core's JSON unchanged", () => {
  const args = '{"level":"quarter","year":2023,"index":4}';
  const expected =
    '{"level":"quarter","year":2023,"index":4,"start":"2023-10-29","end_exclusive":"2024-02-04","last_day":"2024-02-03","days":98,"weeks":14,"has_extra_week":true}';
  assert.equal(cal.callJson("period", args), expected);
  assert.equal(callJson(JSON.stringify(NRF), "period", args), expected);
});

test("dimension and explain", () => {
  const rows = cal.dimension("2024-02-03", "2024-02-05");
  assert.deepEqual(rows.map((r) => r.fiscal_year), [2023, 2024]);
  assert.equal(cal.dimensionCsv("2024-02-03", "2024-02-05").split("\n").length, 4);
  assert.equal(cal.explain("2024-02-03").calendar_id, cal.calendarId);
});

test("invalid definitions throw with every error; validate does not throw", () => {
  const bad = structuredClone(NRF);
  bad.subdivision.periods = Array(12).fill(4);
  assertError(() => new FiscalCalendar(bad), "WEEKS_SUM_MISMATCH", "subdivision.periods");
  assert.deepEqual(validate(bad).errors, JSON.parse(validateJson(JSON.stringify(bad))).errors);
  assert.equal(validate(NRF).ok, true);
});

test("operation errors carry code and path", () => {
  assertError(() => cal.locate("2024-02-30"), "INVALID_DATE", "date");
});

test("Date objects are rejected with TIMESTAMP_REJECTED", () => {
  const d = new Date(Date.UTC(2024, 1, 3));
  assertError(() => cal.locate(d), "TIMESTAMP_REJECTED", "date");
  assertError(() => cal.explain(d), "TIMESTAMP_REJECTED", "date");
  assertError(() => cal.dimension(d, "2024-03-01"), "TIMESTAMP_REJECTED", "start");
  assertError(() => cal.dimensionCsv("2024-01-01", d), "TIMESTAMP_REJECTED", "end_exclusive");
});

test("a Date from another realm is rejected", () => {
  const foreign = vm.runInNewContext("new Date(0)");
  assert.ok(!(foreign instanceof Date));
  assertError(() => cal.locate(foreign), "TIMESTAMP_REJECTED", "date");
});

test("Temporal timestamp types are rejected", () => {
  for (const tag of ["Temporal.PlainDateTime", "Temporal.ZonedDateTime", "Temporal.Instant"]) {
    assertError(() => cal.locate(plainDate("2024-02-03", tag)), "TIMESTAMP_REJECTED", "date");
  }
});

test("the timestamp error is the same JSON as in every binding", () => {
  try {
    cal.locate(new Date());
    assert.fail("did not throw");
  } catch (e) {
    assert.deepEqual(e.errors, [
      {
        code: "TIMESTAMP_REJECTED",
        path: "date",
        message: "a timestamp was passed where a civil date (year, month, day) is expected",
        hint: "convert the timestamp to a date in the time zone you mean, then pass that date",
        details: null,
      },
    ]);
  }
});

test("values JSON cannot carry still produce a coded error", () => {
  for (const v of [10n, Symbol("x"), () => 1, undefined]) {
    assert.throws(() => cal.locate(v), (e) => e instanceof FiscalclockError && ["INVALID_ARGUMENT", "INVALID_DATE"].includes(e.code));
  }
});

test("a BigInt year or n is a coded error at its path", () => {
  assertError(() => cal.period("quarter", 2023n, 4), "INVALID_ARGUMENT", "year");
  assertError(() => cal.shift(cal.period("quarter", 2023, 4), 1n), "INVALID_ARGUMENT", "n");
});

test("a period that is not an object is a coded error at period", () => {
  for (const v of [null, undefined, "Q4 FY2023", 7]) {
    assertError(() => cal.shift(v, 1), "INVALID_ARGUMENT", "period");
  }
});

test("a definition without a JSON form is INVALID_JSON", () => {
  const cyclic = structuredClone(NRF);
  cyclic.self = cyclic;
  for (const v of [undefined, 1n, Symbol("x"), () => 1, cyclic]) {
    assertError(() => new FiscalCalendar(v), "INVALID_JSON", "");
    assert.equal(validate(v).errors[0].code, "INVALID_JSON");
  }
  assertError(() => new FiscalCalendar({ ...NRF, spec_version: 1n }), "INVALID_TYPE", "spec_version");
});

test("lone surrogates are a coded error", () => {
  assertError(() => cal.parse("\ud800"), "INVALID_ARGUMENT", "");
  assertError(() => new FiscalCalendar({ ...NRF, spec_version: "\udfff" }), "INVALID_JSON", "");
});

test("real Temporal values, where the runtime has Temporal", { skip: typeof globalThis.Temporal === "undefined" && "no Temporal in this runtime" }, () => {
  const { Temporal } = globalThis;
  assert.deepEqual(cal.locate(Temporal.PlainDate.from("2024-02-03")), cal.locate("2024-02-03"));
  assertError(() => cal.locate(Temporal.PlainDateTime.from("2024-02-03T12:00")), "TIMESTAMP_REJECTED", "date");
  assertError(() => cal.locate(Temporal.Now.zonedDateTimeISO()), "TIMESTAMP_REJECTED", "date");
  assertError(() => cal.locate(Temporal.Now.instant()), "TIMESTAMP_REJECTED", "date");
});
