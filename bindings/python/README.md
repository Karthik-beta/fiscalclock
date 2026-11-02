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

Wheels are published for CPython 3.10 and later (one stable-ABI wheel per platform) and for free-threaded CPython 3.14.

## Example

A 4-5-4 calendar whose year ends on the Saturday nearest 31 January:

```python
import datetime as dt
from fiscalclock import FiscalCalendar

cal = FiscalCalendar({
    "spec_version": 1,
    "year": {"type": "week_based",
             "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
    "week": {"starts_on": "sunday", "numbering": "contains_start"},
    "subdivision": {"type": "week_pattern", "periods": [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4],
                    "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}},
    "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start",
               "templates": {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}}}},
})

fy2023 = cal.period("year", 2023)
assert fy2023.start == dt.date(2023, 1, 29)
assert fy2023.last_day == dt.date(2024, 2, 3)
assert fy2023.weeks == 53

here = cal.locate(dt.date(2024, 2, 3))
assert cal.format(here.quarter) == "Q4 FY2023"
assert cal.parse("Q4 FY2023") == here.quarter
```

## API

`FiscalCalendar(definition)` takes a JSON string or a mapping and raises `FiscalclockError` if the definition is invalid. Its methods are `locate`, `period`, `shift`, `parent`, `children`, `years`, `format`, `parse`, `dimension`, `dimension_csv` and `explain`, as described in [`spec/api-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md).

- `Period` and `Location` are frozen dataclasses with `datetime.date` fields. Every period is the half-open interval `[start, end_exclusive)`, and `last_day` is the day before `end_exclusive`.
- `dimension` returns a list of dicts with dates as ISO strings, ready for a data frame.
- `call_json(op, args)` and the module-level `call_json` and `validate_json` take and return JSON text exactly as the core produces it.
- `validate(definition)` returns the validation result as a dict and does not raise.
- `FiscalclockError` is a `ValueError` with the stable `code`, the JSON `path`, `message`, `hint` and `details` of the first error, and `errors` holding all of them.

## Dates

Date arguments accept only an object whose type is exactly `datetime.date`, or an ISO `YYYY-MM-DD` string. A `datetime.datetime`, including a pandas `Timestamp`, raises `FiscalclockError` with code `TIMESTAMP_REJECTED`, because the calendar day of a timestamp depends on the time zone. Convert it to a date in the time zone you mean first.

## How the results are checked

- 282 shared conformance vector files (10,483 cases) are run by the Rust, Python and npm test suites. Expected results come from cited sources or from independent generators, never from fiscalclock itself. See [`conformance/`](https://github.com/Karthik-beta/fiscalclock/tree/main/conformance).
- Week-based years are checked against Python's `date.isocalendar()` for every year from 1600 to 2399, and a slow day-by-day reference implementation is compared with the core for every date from 1900 to 2200.
- Property tests check that periods tile and nest, and that `shift` and `format`/`parse` round-trip.
- On every push, CI runs all vectors through Rust and through the Python and npm packages on Linux, macOS and Windows and requires the outputs to match byte for byte.

## Supply chain

This package is built and published by the [release workflow](https://github.com/Karthik-beta/fiscalclock/blob/main/.github/workflows/release.yml) from a version tag on `main`, after a smoke test of the built packages on Linux, macOS and Windows and the maintainer's approval. Every release file has a [build provenance attestation](https://github.com/Karthik-beta/fiscalclock/attestations). To check a wheel or sdist downloaded from PyPI or the GitHub release:

```sh
gh attestation verify <file> --repo Karthik-beta/fiscalclock
```

Report vulnerabilities privately as described in [SECURITY.md](https://github.com/Karthik-beta/fiscalclock/blob/main/SECURITY.md). Changes are listed in the [changelog](https://github.com/Karthik-beta/fiscalclock/blob/main/CHANGELOG.md).

## Licence

Licensed under either of [Apache License, Version 2.0](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-APACHE) or [MIT licence](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-MIT), at your option.
