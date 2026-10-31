# fiscalclock for Python

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

## Licence

Licensed under either of the Apache License, Version 2.0 or the MIT licence, at your option.
