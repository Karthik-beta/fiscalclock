# fiscalclock for Python

[![PyPI](https://img.shields.io/pypi/v/fiscalclock?label=PyPI)](https://pypi.org/project/fiscalclock/)
[![Python versions](https://img.shields.io/pypi/pyversions/fiscalclock)](https://pypi.org/project/fiscalclock/)
[![CI](https://github.com/Karthik-beta/fiscalclock/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Karthik-beta/fiscalclock/actions/workflows/ci.yml)
[![Licence: MIT OR Apache-2.0](https://img.shields.io/badge/licence-MIT%20OR%20Apache--2.0-blue)](#licence)

Python package for [fiscalclock](https://github.com/Karthik-beta/fiscalclock), a fiscal calendar engine. You describe a fiscal calendar as a JSON definition, and the package answers questions about it: which fiscal year, quarter, period and week a date falls in, the bounds of any period, labels such as `Q4 FY2023`, and a day-by-day date dimension.

The package has no built-in calendars. Every calendar is a definition you supply, in the format specified in [`spec/definition-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/definition-v1.md).

All computation happens in the fiscalclock Rust core, compiled into this package. The Python layer only converts values, so results are identical to the Rust crate and the npm package.

Status: 0.1.0 is the first release. The definition format and API are version 1 drafts and may change before 1.0.

## Install

```sh
pip install fiscalclock
```

As of 4 Nov 2026, the [0.1.0 files on PyPI](https://pypi.org/project/fiscalclock/0.1.0/#files) include wheels for CPython 3.10 and later (one stable-ABI `cp310-abi3` wheel per platform) and for free-threaded CPython 3.14 (`cp314t`).

## Example

A 4-5-4 calendar whose year ends on the Saturday nearest 31 January:

```python
import datetime as dt
from fiscalclock import FiscalCalendar

definition = {
    "spec_version": 1,
    "year": {"type": "week_based",
             "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
    "week": {"starts_on": "sunday", "numbering": "contains_start"},
    "subdivision": {"type": "week_pattern", "periods": [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4],
                    "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}},
    "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start",
               "templates": {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}}}},
}
cal = FiscalCalendar(definition)

fy2023 = cal.period("year", 2023)
assert fy2023.start == dt.date(2023, 1, 29)
assert fy2023.last_day == dt.date(2024, 2, 3)
assert fy2023.weeks == 53

here = cal.locate(dt.date(2024, 2, 3))
assert cal.format(here.quarter) == "Q4 FY2023"
assert cal.parse("Q4 FY2023") == here.quarter
```

## API

`FiscalCalendar(definition)` takes a JSON string or a mapping, validates it once, and raises `FiscalclockError` if it is invalid. The operations are specified in [`spec/api-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md).

| Member | Returns |
| --- | --- |
| `locate(date)` | The `Location` of `date`: its year, quarter, period and week. |
| `period(level, year, index=None)` | The `Period` with those coordinates. Omit `index` for a year. |
| `shift(period, n)` | The `Period` `n` steps away at the same level. |
| `parent(period)` | The enclosing `Period` one level up. |
| `children(period, level)` | A `list[Period]` of the periods at `level` inside `period`. |
| `years(first, last)` | A `list[Period]` of the years numbered `first` to `last`, inclusive. |
| `format(period, scheme=None)` | The label, a `str`, under `scheme` or the default scheme. |
| `parse(label)` | The `Period` that `label` names. |
| `dimension(start, end_exclusive)` | A `list[dict]`, one row per day, with dates as ISO strings, ready for a data frame. |
| `dimension_csv(start, end_exclusive)` | The same rows as CSV text. |
| `explain(date)` | A `dict` with the rules applied to locate `date`, step by step. |
| `call_json(op, args)` | The core's JSON text for operation `op` with the JSON text `args`. |
| `calendar_id` | A `str`: the first 16 hex digits of the SHA-256 of `canonical_json`. |
| `canonical_json` | A `str`: the canonical form of the definition. |

- `level` is `"year"`, `"quarter"`, `"period"` or `"week"`.
- `date`, `start` and `end_exclusive` are a `datetime.date` or an ISO `YYYY-MM-DD` string (see [Dates](#dates)).
- `period` is a `Period`, or a mapping with its `level`, `year` and `index`.
- `Period` and `Location` are frozen dataclasses with `datetime.date` fields. Every period is the half-open interval `[start, end_exclusive)`, and `last_day` is the day before `end_exclusive`.

The module also has three functions that take a definition on each call:

- `validate(definition)` takes a JSON string or a mapping and returns the validation result as a dict. It does not raise.
- `validate_json(definition)` takes definition text and returns the core's JSON text.
- `call_json(definition, op, args)` takes definition text, an operation name and argument text, and returns the core's JSON text.

The JSON text from `call_json` and `validate_json` is exactly what the core produces, byte for byte the same as in Rust and JavaScript:

```python
import json
from fiscalclock import call_json

args = '{"period":{"level":"quarter","year":2023,"index":4}}'
print(cal.call_json("format", args))
print(call_json(json.dumps(definition), "format", args))
# {"label":"Q4 FY2023","scheme":"fy"}
# {"label":"Q4 FY2023","scheme":"fy"}
```

## Errors

`FiscalclockError` is a `ValueError`. Its `code`, `path`, `message`, `hint` and `details` describe the first error, and `errors` holds every error as a dict, because validation can report several. `message` is the core's message; `str(error)` combines the code, path, message and hint. Match on `code` and `path`, not on the wording of `message` or `hint`. The codes are listed in the [error catalogue](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md#error-catalogue).

When a label matches more than one period, `parse` raises `AMBIGUOUS_LABEL` with every candidate in `details["candidates"]`, and never picks one:

```python
from fiscalclock import FiscalclockError

two_schemes = FiscalCalendar({**definition, "naming": {"default": "by_start", "schemes": {
    "by_start": {"year_number": "start", "templates": {"year": "FY{yyyy}"}},
    "by_end": {"year_number": "end", "templates": {"year": "FY{yyyy}"}},
}}})

try:
    two_schemes.parse("FY2024")
except FiscalclockError as error:
    print(error.code, error.path)
    for candidate in error.details["candidates"]:
        print(candidate["scheme"], candidate["period"]["start"])
# AMBIGUOUS_LABEL label
# by_end 2023-01-29
# by_start 2024-02-04
```

## Dates

Date arguments accept only an object whose type is exactly `datetime.date`, or an ISO `YYYY-MM-DD` string. A `datetime.datetime`, including a pandas `Timestamp`, raises `FiscalclockError` with code `TIMESTAMP_REJECTED`, because the calendar day of a timestamp depends on the time zone. Convert it to a date in the time zone you mean first.

## How the results are checked

- Shared conformance vectors are run by the Rust, Python and npm test suites. Expected results come from cited sources or from independent generators, never from fiscalclock itself. See [`conformance/`](https://github.com/Karthik-beta/fiscalclock/tree/main/conformance).
- Week-based years are checked against Python's `date.isocalendar()` for every year from 1600 to 2399 ([`oracle_vectors.py`](https://github.com/Karthik-beta/fiscalclock/blob/main/conformance/generators/oracle_vectors.py)), and a slow day-by-day reference implementation is compared with the core for every date from 1900 to 2200 ([`differential.rs`](https://github.com/Karthik-beta/fiscalclock/blob/main/crates/core/tests/differential.rs)).
- [Property tests](https://github.com/Karthik-beta/fiscalclock/blob/main/crates/core/tests/properties.rs) check that periods tile and nest, and that `shift` and `format`/`parse` round-trip.
- On every push, [CI](https://github.com/Karthik-beta/fiscalclock/blob/main/.github/workflows/ci.yml) runs all vectors through Rust and through the Python and npm packages on Linux, macOS and Windows and requires the outputs to match byte for byte.

## Supply chain

This package is built and published by the [release workflow](https://github.com/Karthik-beta/fiscalclock/blob/main/.github/workflows/release.yml) from a version tag on `main`, after a smoke test of the built packages on Linux, macOS and Windows and the maintainer's approval. Every release file has a [build provenance attestation](https://github.com/Karthik-beta/fiscalclock/attestations). To check a wheel or sdist downloaded from PyPI or the GitHub release:

```sh
gh attestation verify <file> --repo Karthik-beta/fiscalclock
```

Report vulnerabilities privately as described in [SECURITY.md](https://github.com/Karthik-beta/fiscalclock/blob/main/SECURITY.md). Changes are listed in the [changelog](https://github.com/Karthik-beta/fiscalclock/blob/main/CHANGELOG.md).

## Licence

Licensed under either of [Apache License, Version 2.0](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-APACHE) or [MIT licence](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-MIT), at your option.
