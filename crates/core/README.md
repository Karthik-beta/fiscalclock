# fiscalclock

[![crates.io](https://img.shields.io/crates/v/fiscalclock?label=crates.io)](https://crates.io/crates/fiscalclock)
[![docs.rs](https://img.shields.io/docsrs/fiscalclock?label=docs.rs)](https://docs.rs/fiscalclock)
[![MSRV](https://img.shields.io/crates/msrv/fiscalclock?label=MSRV)](https://crates.io/crates/fiscalclock)
[![CI](https://github.com/Karthik-beta/fiscalclock/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Karthik-beta/fiscalclock/actions/workflows/ci.yml)
[![Licence: MIT OR Apache-2.0](https://img.shields.io/badge/licence-MIT%20OR%20Apache--2.0-blue)](#licence)

The Rust core of [fiscalclock](https://github.com/Karthik-beta/fiscalclock), a fiscal calendar engine. You describe a fiscal calendar as a JSON definition, and the crate answers questions about it: which fiscal year, quarter, period and week a date falls in, the bounds of any period, labels such as `Q4 FY2023`, and a day-by-day date dimension.

The crate has no built-in calendars. Every calendar, including well-known ones, is a definition you supply. The format is specified in [`spec/definition-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/definition-v1.md) and the operations in [`spec/api-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md).

Status: 0.1.0 is the first release. The definition format and API are version 1 drafts and may change before 1.0.

## Example

A 4-5-4 calendar whose year ends on the Saturday nearest 31 January:

```rust
use fiscalclock::{Date, FiscalCalendar, Level};

let definition = r#"{
  "spec_version": 1,
  "year": {"type": "week_based",
           "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
  "week": {"starts_on": "sunday", "numbering": "contains_start"},
  "subdivision": {"type": "week_pattern", "periods": [4,5,4,4,5,4,4,5,4,4,5,4],
                  "quarters": [3,3,3,3], "extra_week": {"period": 12}},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start",
             "templates": {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}}}}
}"#;
let cal = FiscalCalendar::from_json(definition).unwrap();

let fy2023 = cal.period(Level::Year, 2023, None).unwrap();
assert_eq!(fy2023.start, Date::from_ymd(2023, 1, 29).unwrap());
assert_eq!(fy2023.last_day(), Date::from_ymd(2024, 2, 3).unwrap());
assert_eq!(fy2023.weeks, Some(53));

let here = cal.locate(Date::from_ymd(2024, 2, 3).unwrap()).unwrap();
assert_eq!(cal.format(&here.quarter, None).unwrap(), "Q4 FY2023");
```

## API

`FiscalCalendar` validates a definition once and answers every operation from it. The operations are specified in [`spec/api-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md).

```text
impl FiscalCalendar {
    fn from_json(definition: &str) -> Result<FiscalCalendar, Errors>
    fn new(def: Definition) -> FiscalCalendar          // from Definition::from_json(&str)
    fn definition(&self) -> &Definition
    fn calendar_id(&self) -> &str                      // first 16 hex digits of the SHA-256 of canonical_json
    fn canonical_json(&self) -> &str
    fn locate(&self, date: Date) -> Result<Location, Error>
    fn period(&self, level: Level, year: i32, index: Option<u32>) -> Result<Period, Error>
    fn shift(&self, period: &Period, n: i64) -> Result<Period, Error>
    fn parent(&self, period: &Period) -> Result<Period, Error>
    fn children(&self, period: &Period, level: Level) -> Result<Vec<Period>, Error>
    fn years(&self, from: i32, to: i32) -> Result<Vec<Period>, Error>
    fn format(&self, period: &Period, scheme: Option<&str>) -> Result<String, Error>
    fn parse(&self, label: &str) -> Result<Period, Error>
    fn dimension(&self, start: Date, end_exclusive: Date) -> Result<Vec<DimensionRow>, Error>
    fn dimension_csv(&self, start: Date, end_exclusive: Date) -> Result<String, Error>
    fn explain(&self, date: Date) -> Result<Explanation, Error>
    fn label_columns(&self) -> Vec<String>
    fn call_json(&self, op: &str, args: &str) -> String
}
```

- `Level` is `Year`, `Quarter`, `Period` or `Week`. `index` is `None` for a year.
- `Date` is a civil date: `Date::from_ymd(2024, 2, 3)` and `Date::parse_iso("2024-02-03")` return `Option<Date>`, and `Date` implements `FromStr` and `Display` in `YYYY-MM-DD` form.
- `Period` has the public fields `level`, `year`, `index`, `start`, `end_exclusive`, `days`, `weeks` and `has_extra_week`, and the method `last_day()`. Operations that take a `Period` recompute it from `(level, year, index)`.
- `from_json` and `Definition::from_json` return `Errors`, every validation error found. The operations return one `Error`.

## JSON API

`fiscalclock::json::validate(definition)` and `fiscalclock::json::call(definition, op, args)` take and return JSON text. The Python and npm packages call these functions and return their output unchanged, so the same definition and arguments give byte-identical JSON in every language. `json::call` validates the definition on every call. For repeated calls with one definition, build a `FiscalCalendar` once and use `FiscalCalendar::call_json(op, args)`, which returns the same text.

```rust
let args = r#"{"period":{"level":"quarter","year":2023,"index":4}}"#;
let out = fiscalclock::json::call(definition, "format", args);
assert_eq!(out, r#"{"label":"Q4 FY2023","scheme":"fy"}"#);
assert_eq!(cal.call_json("format", args), out);
```

## Errors

Every failure is an `Error` value with a stable `code`, the JSON `path` of the offending field or argument, a `message`, an optional `hint` and optional `details`. In JSON, errors are returned as `{"errors":[...]}`. Match on `code` and `path`, not on the wording of `message` or `hint`. The codes are listed in the [error catalogue](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md#error-catalogue).

When a label matches more than one period, `parse` returns `AMBIGUOUS_LABEL` with every candidate in `details`, and never picks one:

```rust
use fiscalclock::{Details, ErrorCode};

let two_schemes = FiscalCalendar::from_json(r#"{
  "spec_version": 1,
  "year": {"type": "week_based",
           "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
  "week": {"starts_on": "sunday", "numbering": "contains_start"},
  "subdivision": {"type": "week_pattern", "periods": [4,5,4,4,5,4,4,5,4,4,5,4],
                  "quarters": [3,3,3,3], "extra_week": {"period": 12}},
  "naming": {"default": "by_start", "schemes": {
    "by_start": {"year_number": "start", "templates": {"year": "FY{yyyy}"}},
    "by_end": {"year_number": "end", "templates": {"year": "FY{yyyy}"}}}}
}"#).unwrap();

let err = two_schemes.parse("FY2024").unwrap_err();
assert_eq!(err.code, ErrorCode::AmbiguousLabel);
assert_eq!(err.path, "label");
let Some(Details::Candidates(candidates)) = &err.details else {
    panic!("AMBIGUOUS_LABEL always has candidates");
};
let found: Vec<_> = candidates
    .iter()
    .map(|c| (c.scheme.as_str(), c.period.start.to_string()))
    .collect();
assert_eq!(found, [("by_end", "2023-01-29".to_string()), ("by_start", "2024-02-04".to_string())]);
```

## Properties

- Every period is a half-open interval `[start, end_exclusive)`; `last_day()` is derived.
- Dates are civil dates (no time or time zone) in the proleptic Gregorian calendar. Fiscal years must lie wholly between 0001-01-02 and 9999-12-30.
- `#![no_std]` with `alloc`, and `#![forbid(unsafe_code)]`. The crate reads no clock, environment variable or global state.
- Invalid input returns an error value. The test suite feeds malformed input to every operation and checks that it never panics.
- Dependencies: `serde`, `serde_json` and `sha2`.
- Minimum supported Rust version: 1.85.

## How the results are checked

- Shared conformance vectors are run by the Rust, Python and npm test suites. Expected results come from cited sources or from independent generators, never from fiscalclock itself. See [`conformance/`](https://github.com/Karthik-beta/fiscalclock/tree/main/conformance).
- Week-based years are checked against Python's `date.isocalendar()` for every year from 1600 to 2399, and a slow day-by-day reference implementation is compared with the core for every date from 1900 to 2200.
- Property tests check that periods tile and nest, and that `shift` and `format`/`parse` round-trip.
- On every push to `main`, and on every pull request that changes a file the package jobs read, [CI](https://github.com/Karthik-beta/fiscalclock/blob/main/.github/workflows/ci.yml) runs all vectors through Rust on Linux and through the Python and npm packages on Linux, macOS and Windows, and requires the 14 raw outputs to match the Rust output byte for byte.

## Supply chain

This package is built and published by the [release workflow](https://github.com/Karthik-beta/fiscalclock/blob/main/.github/workflows/release.yml) from a version tag on `main`, after a smoke test of the built packages on Linux, macOS and Windows and the maintainer's approval. Every release file has a [build provenance attestation](https://github.com/Karthik-beta/fiscalclock/attestations). To check the `.crate` file attached to the GitHub release:

```sh
gh attestation verify <file> --repo Karthik-beta/fiscalclock
```

Report vulnerabilities privately as described in [SECURITY.md](https://github.com/Karthik-beta/fiscalclock/blob/main/SECURITY.md). Changes are listed in the [changelog](https://github.com/Karthik-beta/fiscalclock/blob/main/CHANGELOG.md).

## Licence

Licensed under either of [Apache License, Version 2.0](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-APACHE) or [MIT licence](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-MIT) at your option.
