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

let cal = FiscalCalendar::from_json(r#"{
  "spec_version": 1,
  "year": {"type": "week_based",
           "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
  "week": {"starts_on": "sunday", "numbering": "contains_start"},
  "subdivision": {"type": "week_pattern", "periods": [4,5,4,4,5,4,4,5,4,4,5,4],
                  "quarters": [3,3,3,3], "extra_week": {"period": 12}},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start",
             "templates": {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}}}}
}"#).unwrap();

let fy2023 = cal.period(Level::Year, 2023, None).unwrap();
assert_eq!(fy2023.start, Date::from_ymd(2023, 1, 29).unwrap());
assert_eq!(fy2023.last_day(), Date::from_ymd(2024, 2, 3).unwrap());
assert_eq!(fy2023.weeks, Some(53));

let here = cal.locate(Date::from_ymd(2024, 2, 3).unwrap()).unwrap();
assert_eq!(cal.format(&here.quarter, None).unwrap(), "Q4 FY2023");
```

## JSON API

`fiscalclock::json::validate` and `fiscalclock::json::call` take and return JSON text. The Python and npm packages call these functions and return their output unchanged, so the same definition and arguments give byte-identical JSON in every language.

```rust
let out = fiscalclock::json::call(definition, "locate", r#"{"date":"2024-02-03"}"#);
```

Errors are JSON objects with a stable `code`, the JSON `path` of the offending field, a `message` and, where it helps, a `hint`. The codes are listed in the [error catalogue](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md#error-catalogue).

## Properties

- Every period is a half-open interval `[start, end_exclusive)`; `last_day()` is derived.
- Dates are civil dates (no time or time zone) in the proleptic Gregorian calendar. Fiscal years must lie wholly between 0001-01-02 and 9999-12-30.
- `#![no_std]` with `alloc`, and `#![forbid(unsafe_code)]`. The crate reads no clock, environment variable or global state.
- Invalid input returns an error value. The test suite feeds malformed input to every operation and checks that it never panics.
- Dependencies: `serde`, `serde_json` and `sha2`.
- Minimum supported Rust version: 1.85.

## How the results are checked

- 112 shared conformance vector files (4,070 cases) are run by the Rust, Python and npm test suites. Expected results come from cited sources or from independent generators, never from fiscalclock itself. See [`conformance/`](https://github.com/Karthik-beta/fiscalclock/tree/main/conformance).
- Week-based years are checked against Python's `date.isocalendar()` for every year from 1600 to 2399, and a slow day-by-day reference implementation is compared with the core for every date from 1900 to 2200.
- Property tests check that periods tile and nest, and that `shift` and `format`/`parse` round-trip.
- On every push, CI runs all vectors through Rust and through the Python and npm packages on Linux, macOS and Windows and requires the outputs to match byte for byte.

## Supply chain

This package is built and published by the [release workflow](https://github.com/Karthik-beta/fiscalclock/blob/main/.github/workflows/release.yml) from a version tag on `main`, after a smoke test of the built packages on Linux, macOS and Windows and the maintainer's approval. Every release file has a [build provenance attestation](https://github.com/Karthik-beta/fiscalclock/attestations). To check the `.crate` file attached to the GitHub release:

```sh
gh attestation verify <file> --repo Karthik-beta/fiscalclock
```

Report vulnerabilities privately as described in [SECURITY.md](https://github.com/Karthik-beta/fiscalclock/blob/main/SECURITY.md). Changes are listed in the [changelog](https://github.com/Karthik-beta/fiscalclock/blob/main/CHANGELOG.md).

## Licence

Licensed under either of [Apache License, Version 2.0](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-APACHE) or [MIT licence](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-MIT) at your option.
