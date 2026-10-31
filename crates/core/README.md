# fiscalclock

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

## Testing

Results are checked against conformance vectors whose expected values come from independent generators and cited sources, never from this crate. Other checks are a day-by-day differential test against a slow reference implementation for every date from 1900 to 2200, and property tests. See [`conformance/`](https://github.com/Karthik-beta/fiscalclock/tree/main/conformance).

## Licence

Licensed under either of [Apache License, Version 2.0](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-APACHE) or [MIT licence](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-MIT) at your option.
