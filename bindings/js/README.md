# fiscalclock for JavaScript

[![npm](https://img.shields.io/npm/v/fiscalclock?label=npm)](https://www.npmjs.com/package/fiscalclock)
[![Node.js](https://img.shields.io/node/v/fiscalclock)](https://www.npmjs.com/package/fiscalclock)
[![Types](https://img.shields.io/npm/types/fiscalclock)](https://www.npmjs.com/package/fiscalclock)
[![CI](https://github.com/Karthik-beta/fiscalclock/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Karthik-beta/fiscalclock/actions/workflows/ci.yml)
[![Licence: MIT OR Apache-2.0](https://img.shields.io/badge/licence-MIT%20OR%20Apache--2.0-blue)](#licence)

npm package for [fiscalclock](https://github.com/Karthik-beta/fiscalclock), a fiscal calendar engine. You describe a fiscal calendar as a JSON definition, and the package answers questions about it: which fiscal year, quarter, period and week a date falls in, the bounds of any period, labels such as `Q4 FY2023`, and a day-by-day date dimension.

The package has no built-in calendars. Every calendar is a definition you supply, in the format specified in [`spec/definition-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/definition-v1.md).

All computation happens in the fiscalclock Rust core, compiled to WebAssembly. The TypeScript layer only converts values, so results are identical to the Rust crate and the Python package.

Status: 0.1.0 is the first release. The definition format and API are version 1 drafts and may change before 1.0.

## Install

```sh
npm install fiscalclock
```

The package is an ES module with TypeScript types and needs Node.js 22 or later. In Node.js it loads the WebAssembly module from disk when imported. Other environments get an entry point that fetches the module relative to its own URL with top-level `await`.

## Example

A 4-5-4 calendar whose year ends on the Saturday nearest 31 January:

```js
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar({
  spec_version: 1,
  year: { type: "week_based", anchor: { edge: "end", month: 1, day: "last", rule: "nearest" } },
  week: { starts_on: "sunday", numbering: "contains_start" },
  subdivision: { type: "week_pattern", periods: [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4],
                 quarters: [3, 3, 3, 3], extra_week: { period: 12 } },
  naming: { default: "fy", schemes: { fy: { year_number: "start",
            templates: { year: "FY{yyyy}", quarter: "Q{q} FY{yyyy}" } } } },
});

const fy2023 = cal.period("year", 2023);
fy2023.start;    // "2023-01-29"
fy2023.last_day; // "2024-02-03"
fy2023.weeks;    // 53

const here = cal.locate("2024-02-03");
cal.format(here.quarter); // "Q4 FY2023"
```

## API

`new FiscalCalendar(definition)` takes a JSON string or an object and throws `FiscalclockError` if the definition is invalid. Its methods are `locate`, `period`, `shift`, `parent`, `children`, `years`, `format`, `parse`, `dimension`, `dimensionCsv` and `explain`, as described in [`spec/api-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md).

- Periods and locations are plain objects with the keys of the core's JSON output, and dates as ISO `YYYY-MM-DD` strings. Every period is the half-open interval `[start, end_exclusive)`.
- `callJson(op, args)` and the exported `callJson` and `validateJson` take and return JSON text exactly as the core produces it.
- `validate(definition)` returns the validation result and does not throw.
- `FiscalclockError` carries the stable `code`, the JSON `path`, `hint` and `details` of the first error, and `errors` holding all of them.

## Dates

Date arguments accept an ISO `YYYY-MM-DD` string or a `Temporal.PlainDate`. A JavaScript `Date`, and a Temporal `PlainDateTime`, `ZonedDateTime` or `Instant`, throws `FiscalclockError` with code `TIMESTAMP_REJECTED`, because the calendar day of a timestamp depends on the time zone. Convert it to a date in the time zone you mean first.

## How the results are checked

- 282 shared conformance vector files (10,483 cases) are run by the Rust, Python and npm test suites. Expected results come from cited sources or from independent generators, never from fiscalclock itself. See [`conformance/`](https://github.com/Karthik-beta/fiscalclock/tree/main/conformance).
- Week-based years are checked against Python's `date.isocalendar()` for every year from 1600 to 2399, and a slow day-by-day reference implementation is compared with the core for every date from 1900 to 2200.
- Property tests check that periods tile and nest, and that `shift` and `format`/`parse` round-trip.
- On every push, CI runs all vectors through Rust and through the Python and npm packages on Linux, macOS and Windows and requires the outputs to match byte for byte.

## Supply chain

This package is built and published by the [release workflow](https://github.com/Karthik-beta/fiscalclock/blob/main/.github/workflows/release.yml) from a version tag on `main`, after a smoke test of the built packages on Linux, macOS and Windows and the maintainer's approval. Every release file has a [build provenance attestation](https://github.com/Karthik-beta/fiscalclock/attestations). To check the `.tgz` from `npm pack fiscalclock` or the GitHub release:

```sh
gh attestation verify <file> --repo Karthik-beta/fiscalclock
```

Report vulnerabilities privately as described in [SECURITY.md](https://github.com/Karthik-beta/fiscalclock/blob/main/SECURITY.md). Changes are listed in the [changelog](https://github.com/Karthik-beta/fiscalclock/blob/main/CHANGELOG.md).

## Licence

Licensed under either of [Apache License, Version 2.0](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-APACHE) or [MIT licence](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-MIT), at your option.
