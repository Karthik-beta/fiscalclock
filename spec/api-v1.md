# Operations and outputs, API version 1

Status: draft for fiscalclock 0.1.0. Last reviewed: 31 Oct 2026.

Every operation is defined once, in the Rust core, as JSON in and JSON out. The Rust crate, the Python package and the npm package expose these operations with native types at the edge, and each also returns the core's JSON unchanged, so the same definition and inputs give byte-identical JSON in every language.

## Conventions

- Dates are civil dates in ISO 8601 extended format, `YYYY-MM-DD`, proleptic Gregorian, years 0001–9999. Anything else, including a time or zone suffix, is `INVALID_DATE`.
- Every period is the half-open interval `[start, end_exclusive)`. `last_day` is `end_exclusive` minus one day.
- Levels are `"year"`, `"quarter"`, `"period"` and `"week"`.
- No operation reads the clock, the environment or any global state. "Current" questions take the date as an argument.
- JSON output has keys in the order shown in this document, no insignificant whitespace, and integers in plain decimal. Output contains no floating-point numbers in API version 1.

## Values

### Period

```json
{"level":"quarter","year":2023,"index":4,"start":"2023-10-29","end_exclusive":"2024-02-04","last_day":"2024-02-03","days":98,"weeks":14,"has_extra_week":true}
```

| Key | Meaning |
| --- | --- |
| `level` | The period's level. |
| `year` | Fiscal year number under the default naming scheme, including its `offset`. |
| `index` | 1-based position within the fiscal year; `null` for a year. |
| `start`, `end_exclusive`, `last_day` | Bounds. |
| `days` | Number of days. |
| `weeks` | Number of weeks when the year is `week_based`; `null` otherwise. |
| `has_extra_week` | For a year, quarter or period: true when it contains the extra week. For a week: true when it is the extra week. Always false in `fixed_date` years. |

### Location

Returned by `locate`.

```json
{"date":"2024-02-03","year":{…},"quarter":{…},"period":{…},"week":{…},"day_of_year":371,"day_of_quarter":98,"day_of_period":35,"day_of_week":7}
```

`day_of_*` values are 1-based. `day_of_week` counts from `week.starts_on` (1) to the day before it (7).

### Error

```json
{"code":"WEEKS_SUM_MISMATCH","path":"subdivision.periods","message":"periods sum to 51 weeks, expected 52","hint":"add one week to a period","details":null}
```

- `code` is stable and listed in the [error catalogue](#error-catalogue). Code and path must match across languages; `message` and `hint` are produced by the core and are therefore also identical, but tests must not depend on their wording.
- `path` is the JSON path of the offending field in dotted form with `[i]` for array indexes, such as `naming.schemes.fy.templates.year` or `subdivision.periods[3]`. An empty string means the whole input. For operation arguments, `path` names the argument, such as `date`.
- `details` is `null` or an object. `AMBIGUOUS_LABEL` puts its candidates here.

Paths for common errors:

| Error | Path |
| --- | --- |
| `SHORT_MONTH_POLICY_MISSING` | `year.short_month` |
| `EXTRA_WEEK_MISSING` | `subdivision.extra_week` |
| `UNSUPPORTED_COMBINATION` | `subdivision.type` |
| `CENTURY_PIVOT_MISSING` | `naming.schemes.<name>.century_pivot` |
| `YEAR_NUMBER_COLLISION`, `MAJORITY_TIE` | `naming.schemes.<name>.year_number` |
| `INVALID_JSON` | `""` (empty string) |
| An invalid scheme name | `naming.schemes.<name>` |
| `PERIOD_NOT_FOUND` from `period` | `index` (`OUT_OF_RANGE` from `period` is at `year`) |
| Any error about a `period` argument, including `PERIOD_NOT_FOUND` from `shift` | `period` |
| `OUT_OF_RANGE` from `shift` | `n` |
| `TEMPLATE_MISSING`, and `OUT_OF_RANGE` from `format` | `period` |
| `OUT_OF_RANGE` from `dimension` | `start` if its fiscal year is out of range, otherwise `end_exclusive` when the fiscal year of the range's last day (`end_exclusive` minus one day) is out of range. An empty range returns `[]` without this check. |
| `INVALID_ARGUMENT` from `dimension` (`start` after `end_exclusive`, or more than 146,097 days) | `end_exclusive` |
| `LEVEL_NOT_NESTED` and an invalid `level` from `children` | `level` |
| `INVALID_ARGUMENT` from `years` (`from` after `to`, or more than 10,000 years) | `to` |

When validation finds a `MAJORITY_TIE` in a scheme, it does not also report `YEAR_NUMBER_COLLISION` for that scheme.

A failed operation returns `{"errors":[Error, …]}`. Validation reports every error it finds, sorted by `path` and then `code`. Other operations report one error.

## Operations

| Operation | Arguments | Result |
| --- | --- | --- |
| `validate` | `definition` | `{"ok":true,"calendar_id":"…","canonical":"…"}` or errors |
| `locate` | `date` | Location |
| `period` | `level`, `year`, `index` (omit or `null` for a year) | Period, or `PERIOD_NOT_FOUND` |
| `shift` | `period`, `n` (integer) | The period `n` steps away at the same level, crossing years as needed |
| `parent` | `period` | The enclosing period one level up: week → period in a `week_based` year, week → year in a `fixed_date` year, period → quarter, quarter → year. A year has no parent (`INVALID_ARGUMENT`). |
| `children` | `period`, `level` | Periods at `level` inside `period`, in order. `week` children of a quarter or period in a `fixed_date` year are `LEVEL_NOT_NESTED`, because weeks cross month boundaries there. |
| `years` | `from`, `to` (fiscal year numbers, inclusive, `from` ≤ `to`, at most 10,000 years) | Array of year Periods |
| `format` | `period`, `scheme` (optional) | `{"label":"…","scheme":"…"}`, or `TEMPLATE_MISSING` when the scheme has no template for the level, or `SCHEME_NOT_FOUND` |
| `parse` | `label` | Period, or `AMBIGUOUS_LABEL`, or `LABEL_NOT_RECOGNISED` |
| `dimension` | `start`, `end_exclusive` (`start` ≤ `end_exclusive`, at most 146,097 days, which is 400 years) | Array of rows, see below; an empty range gives `[]` |
| `dimension_csv` | `start`, `end_exclusive` | The same rows as CSV text |
| `explain` | `date` | `{"date":"…","calendar_id":"…","steps":[{"level":"…","rule":"…","result":"…"},…]}` |

A `period` argument is a Period value or its coordinates `{"level","year","index"}`. The core recomputes the period from its coordinates and ignores any other keys, so a stale or edited Period cannot produce wrong bounds.

### `parse`

1. Each template of each scheme is compiled to an exact, case-sensitive pattern. `{yyyy}` matches 4 digits, `{yy}` and `{yy_next}` 2 digits, `{pp}` and `{ww}` 2 digits, and `{q}` a digit 1–9 optionally followed by a second digit (no leading zero).
2. Every template that matches the whole label yields a candidate, provided its values are consistent: indexes are in range for that year, and when `{yyyy}` and `{yy_next}` both appear, `{yy_next}` equals the last two digits of `{yyyy}` + 1. Two-digit years without `{yyyy}` resolve through the scheme's `century_pivot`. The scheme's `offset` is removed before the fiscal year is looked up.
3. Candidates that denote the same period (same level and start) are merged.
4. One candidate: return its Period. More than one: `AMBIGUOUS_LABEL` with `details: {"candidates":[{"period":Period,"scheme":"…","level":"…"},…]}`, ordered by start date and then scheme name. None: `LABEL_NOT_RECOGNISED`.

### `dimension`

One row per day in `[start, end_exclusive)`. Columns, in order:

`date, calendar_id, fiscal_year, quarter, period, week, day_of_year, day_of_quarter, day_of_period, day_of_week, year_start, year_last_day, quarter_start, quarter_last_day, period_start, period_last_day, week_start, week_last_day, year_has_extra_week, period_has_extra_week, week_is_extra_week`

followed by one `label_<scheme>_<level>` column for every template, ordered by scheme name and then by level (`year`, `quarter`, `period`, `week`).

In JSON each row is an object with those keys in that order. CSV output follows RFC 4180 with a header row (present even when there are no rows), `\n` line endings, a final newline, booleans as `true`/`false`, and fields quoted only when they contain a comma, quote or line break.

## Error catalogue

| Code | Raised by | When |
| --- | --- | --- |
| `INVALID_JSON` | validate | The definition is not valid JSON or not an object. |
| `UNKNOWN_FIELD` | validate | A field the format does not define. |
| `MISSING_FIELD` | validate | A required field is absent. |
| `INVALID_TYPE` | validate | A field has the wrong JSON type. |
| `INVALID_VALUE` | validate | A value is outside its allowed set or range. |
| `UNSUPPORTED_SPEC_VERSION` | validate | `spec_version` is not 1. |
| `UNSUPPORTED_FEATURE` | validate | A reserved part of the format is used; the hint names the planned release. |
| `UNSUPPORTED_COMBINATION` | validate | A subdivision type is not valid inside the year type. |
| `WEEKS_SUM_MISMATCH` | validate | `periods` does not sum to 52. |
| `QUARTERS_SUM_MISMATCH` | validate | `quarters` does not sum to the number of periods. |
| `EXTRA_WEEK_MISSING` | validate | A week-based year has no `extra_week`. |
| `EXTRA_WEEK_OUT_OF_RANGE` | validate | `extra_week.period` names a period the year does not have. |
| `SHORT_MONTH_POLICY_MISSING` | validate | `start.day` is 29–31 and `short_month` is absent. |
| `SUBDIVISION_YEAR_MISMATCH` | validate | `month_start_day` differs from `year.start.day`. |
| `DEFAULT_SCHEME_MISSING` | validate | `naming.default` is not a scheme name. |
| `INVALID_TEMPLATE` | validate | A template has no year placeholder, an unknown or misplaced placeholder, or unbalanced braces. |
| `CENTURY_PIVOT_MISSING` | validate | A two-digit year template has no `century_pivot`. |
| `YEAR_NUMBER_COLLISION` | validate | A scheme gives two fiscal years the same number, or skips a number. |
| `MAJORITY_TIE` | validate | A `majority` scheme meets a fiscal year split evenly between two calendar years. |
| `INVALID_DATE` | any date argument | Not a valid `YYYY-MM-DD` civil date. |
| `TIMESTAMP_REJECTED` | bindings | A timestamp type (`datetime.datetime`, pandas `Timestamp`, JS `Date`) was passed; the hint names the zone-aware helper. |
| `INVALID_ARGUMENT` | any | An argument is missing, of the wrong type or out of its allowed range. |
| `OUT_OF_RANGE` | any | The fiscal year involved is outside the supported range. |
| `PERIOD_NOT_FOUND` | period, shift | No period has those coordinates. |
| `LEVEL_NOT_NESTED` | children | The requested level does not nest inside the period. |
| `SCHEME_NOT_FOUND` | format | No scheme has that name. |
| `TEMPLATE_MISSING` | format | The scheme has no template for the period's level. |
| `LABEL_NOT_RECOGNISED` | parse | No template matches the label. |
| `AMBIGUOUS_LABEL` | parse | More than one period matches; candidates in `details`. |
