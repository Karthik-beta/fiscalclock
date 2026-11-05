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

const definition = {
  spec_version: 1,
  year: { type: "week_based", anchor: { edge: "end", month: 1, day: "last", rule: "nearest" } },
  week: { starts_on: "sunday", numbering: "contains_start" },
  subdivision: { type: "week_pattern", periods: [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4],
                 quarters: [3, 3, 3, 3], extra_week: { period: 12 } },
  naming: { default: "fy", schemes: { fy: { year_number: "start",
            templates: { year: "FY{yyyy}", quarter: "Q{q} FY{yyyy}" } } } },
};
const cal = new FiscalCalendar(definition);

const fy2023 = cal.period("year", 2023);
fy2023.start;    // "2023-01-29"
fy2023.last_day; // "2024-02-03"
fy2023.weeks;    // 53

const here = cal.locate("2024-02-03");
cal.format(here.quarter); // "Q4 FY2023"
```

## API

`new FiscalCalendar(definition)` takes a JSON string or an object, validates it once, and throws `FiscalclockError` if it is invalid. The operations are specified in [`spec/api-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md).

Method and getter names are camelCase. The objects they return keep the core's snake_case keys, such as `end_exclusive`, `last_day`, `has_extra_week` and `calendar_id`.

| Member | Returns |
| --- | --- |
| `locate(date)` | The `Location` of `date`: its year, quarter, period and week. |
| `period(level, year, index?)` | The `Period` with those coordinates. Omit `index`, or pass `null`, for a year. |
| `shift(period, n)` | The `Period` `n` steps away at the same level. |
| `parent(period)` | The enclosing `Period` one level up. |
| `children(period, level)` | A `Period[]` of the periods at `level` inside `period`. |
| `years(first, last)` | A `Period[]` of the years numbered `first` to `last`, inclusive. |
| `format(period, scheme?)` | The label, a string, under `scheme` or the default scheme. |
| `parse(label)` | The `Period` that `label` names. |
| `dimension(start, endExclusive)` | An array of row objects, one per day, with dates as ISO strings. |
| `dimensionCsv(start, endExclusive)` | The same rows as CSV text. |
| `explain(date)` | `{ date, calendar_id, steps }`: the rules applied to locate `date`, step by step. |
| `callJson(op, args)` | The core's JSON text for operation `op` with the JSON text `args`. |
| `calendarId` | A string: the first 16 hex digits of the SHA-256 of `canonicalJson`. |
| `canonicalJson` | A string: the canonical form of the definition. |

- `level` is `"year"`, `"quarter"`, `"period"` or `"week"`.
- `date`, `start` and `endExclusive` are an ISO `YYYY-MM-DD` string or a `Temporal.PlainDate` (see [Dates](#dates)).
- `period` is a `Period`, or an object with its `level`, `year` and `index`.
- Periods and locations are plain objects with dates as ISO `YYYY-MM-DD` strings. Every period is the half-open interval `[start, end_exclusive)`, and `last_day` is the day before `end_exclusive`.

The package also exports these functions, which take a definition on each call:

- `validate(definition)` takes a JSON string or an object and returns the validation result. It does not throw.
- `validateJson(definition)` takes definition text and returns the core's JSON text.
- `callJson(definition, op, args)` takes definition text, an operation name and argument text, and returns the core's JSON text.
- `coreVersion()` returns the version of the Rust core compiled into the package.

The JSON text from `callJson` and `validateJson` is exactly what the core produces, byte for byte the same as in Rust and Python:

```js
import { callJson } from "fiscalclock";

const args = '{"period":{"level":"quarter","year":2023,"index":4}}';
cal.callJson("format", args);                              // '{"label":"Q4 FY2023","scheme":"fy"}'
callJson(JSON.stringify(definition), "format", args);      // '{"label":"Q4 FY2023","scheme":"fy"}'
```

## Errors

`FiscalclockError` extends `Error`. Its `code`, `path`, `hint` and `details` describe the first error, and `errors` holds every error as `{ code, path, message, hint, details }`, because validation can report several. Its `message` combines the code, path, core message and hint; the core's own message is `errors[0].message`. Match on `code` and `path`, not on the wording of messages or hints. The codes are listed in the [error catalogue](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md#error-catalogue).

When a label matches more than one period, `parse` throws `AMBIGUOUS_LABEL` with every candidate in `details.candidates`, and never picks one:

```js
import { FiscalclockError } from "fiscalclock";

const twoSchemes = new FiscalCalendar({ ...definition, naming: { default: "by_start", schemes: {
  by_start: { year_number: "start", templates: { year: "FY{yyyy}" } },
  by_end: { year_number: "end", templates: { year: "FY{yyyy}" } },
} } });

try {
  twoSchemes.parse("FY2024");
} catch (error) {
  if (!(error instanceof FiscalclockError)) throw error;
  error.code;                                                // "AMBIGUOUS_LABEL"
  error.path;                                                // "label"
  error.details.candidates.map((c) => [c.scheme, c.period.start]);
  // [["by_end", "2023-01-29"], ["by_start", "2024-02-04"]]
}
```

## Dates

Date arguments accept an ISO `YYYY-MM-DD` string or a `Temporal.PlainDate`. A JavaScript `Date`, and a Temporal `PlainDateTime`, `ZonedDateTime` or `Instant`, throws `FiscalclockError` with code `TIMESTAMP_REJECTED`, because the calendar day of a timestamp depends on the time zone. Convert it to a date in the time zone you mean first.

## How the results are checked

- Shared conformance vectors are run by the Rust, Python and npm test suites. Expected results come from cited sources or from independent generators, never from fiscalclock itself. See [`conformance/`](https://github.com/Karthik-beta/fiscalclock/tree/main/conformance).
- Week-based years are checked against Python's `date.isocalendar()` for every year from 1600 to 2399 ([`oracle_vectors.py`](https://github.com/Karthik-beta/fiscalclock/blob/main/conformance/generators/oracle_vectors.py)), and a slow day-by-day reference implementation is compared with the core for every date from 1900 to 2200 ([`differential.rs`](https://github.com/Karthik-beta/fiscalclock/blob/main/crates/core/tests/differential.rs)).
- [Property tests](https://github.com/Karthik-beta/fiscalclock/blob/main/crates/core/tests/properties.rs) check that periods tile and nest, and that `shift` and `format`/`parse` round-trip.
- On every push to `main`, and on every pull request that changes a file the package jobs read, [CI](https://github.com/Karthik-beta/fiscalclock/blob/main/.github/workflows/ci.yml) runs all vectors through Rust on Linux and through the Python and npm packages on Linux, macOS and Windows, and requires the 14 raw outputs to match the Rust output byte for byte.

## Supply chain

This package is built and published by the [release workflow](https://github.com/Karthik-beta/fiscalclock/blob/main/.github/workflows/release.yml) from a version tag on `main`, after a smoke test of the built packages on Linux, macOS and Windows and the maintainer's approval. Every release file has a [build provenance attestation](https://github.com/Karthik-beta/fiscalclock/attestations). To check the `.tgz` from `npm pack fiscalclock` or the GitHub release:

```sh
gh attestation verify <file> --repo Karthik-beta/fiscalclock
```

Report vulnerabilities privately as described in [SECURITY.md](https://github.com/Karthik-beta/fiscalclock/blob/main/SECURITY.md). Changes are listed in the [changelog](https://github.com/Karthik-beta/fiscalclock/blob/main/CHANGELOG.md).

## Licence

Licensed under either of [Apache License, Version 2.0](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-APACHE) or [MIT licence](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-MIT), at your option.
