# Operations and outputs, API version 1

Status: draft for fiscalclock 0.2.0. Last reviewed: 15 Nov 2026.

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
| `weeks` | Number of weeks, the period's `days` divided by 7, when the subdivision of the period's fiscal year is `week_pattern` (a `week_based` year, or a `week_pattern` transition); `null` otherwise. |
| `has_extra_week` | For a year, quarter or period: true when it contains the extra week. For a week: true when it is the extra week. Only a 371-day year of a `week_based` regime has an extra week, so this is always false in `fixed_date` years and in every period of a transition. |

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
- `details` is `null` or an object. `AMBIGUOUS_LABEL` puts its candidates here as `{"candidates":[…]}` (see [`parse`](#parse)). `YEAR_NUMBER_COLLISION` and `MAJORITY_TIE` put the affected years here as `{"years":[…]}`, as described in [Year numbers must be unique](definition-v1.md#year-numbers-must-be-unique). Every other code has `details: null`.

Paths for common errors:

| Error | Path |
| --- | --- |
| `SHORT_MONTH_POLICY_MISSING` | `year.short_month` |
| `EXTRA_WEEK_MISSING` | `subdivision.extra_week` |
| `UNSUPPORTED_COMBINATION` | `subdivision.type` |
| `CENTURY_PIVOT_MISSING` | `naming.schemes.<name>.century_pivot` |
| `YEAR_NUMBER_COLLISION`, `MAJORITY_TIE` | `naming.schemes.<name>.year_number` |
| `INVALID_JSON` | `""` (empty string) |
| Any error in a history entry's `year` or `subdivision` block | The top-level path prefixed with `exceptions.history[i].`, such as `exceptions.history[0].year.short_month` |
| `EXCEPTIONS_OVERLAP`, and `CHANGE_NOT_ON_BOUNDARY` for the previous regime's rule | `exceptions.history[i].transition.start` when entry *i* has a transition, otherwise `exceptions.history[i].from` |
| `CHANGE_NOT_ON_BOUNDARY` for the entry's own rule | `exceptions.history[i].from` |
| `INVALID_VALUE` when `transition.start` is on or after `from` | `exceptions.history[i].transition.start` |
| `SUBDIVISION_YEAR_MISMATCH` for a `calendar_months` transition | `exceptions.history[i].transition.subdivision.month_start_day` |
| `SUBDIVISION_YEAR_MISMATCH` for a `week_pattern` transition | `exceptions.history[i].transition.subdivision.type` |
| `QUARTERS_SUM_MISMATCH` for a transition | `exceptions.history[i].transition.subdivision.quarters` |
| `WEEKS_SUM_MISMATCH` for a transition | `exceptions.history[i].transition.subdivision.periods` |
| An invalid scheme name | `naming.schemes.<name>` |
| `PERIOD_NOT_FOUND` from `period` for an index the fiscal year does not have | `index` (`OUT_OF_RANGE` from `period` is at `year`) |
| `PERIOD_NOT_FOUND` from `period` for a year number between the first and last supported numbers that no fiscal year has (a number skipped at a change of rule) | `year` |
| Any error about a `period` argument, including `PERIOD_NOT_FOUND` from `shift` and `INVALID_ARGUMENT` from `parent` for a year | `period` |
| `OUT_OF_RANGE` from `shift` | `n` |
| `TEMPLATE_MISSING`, and `OUT_OF_RANGE` from `format` | `period` |
| `SCHEME_NOT_FOUND` from `format` | `scheme` |
| `LABEL_NOT_RECOGNISED` and `AMBIGUOUS_LABEL` from `parse` | `label` |
| `OUT_OF_RANGE` from `locate` and `explain` | `date` |
| `OUT_OF_RANGE` from `dimension` | `start` if its fiscal year is out of range, otherwise `end_exclusive` when the fiscal year of the range's last day (`end_exclusive` minus one day) is out of range. An empty range returns `[]` without this check. |
| `INVALID_ARGUMENT` from `dimension` (`start` after `end_exclusive`, or more than 146,097 days) | `end_exclusive` |
| `LEVEL_NOT_NESTED` and an invalid `level` from `children` | `level` |
| `INVALID_ARGUMENT` from `years` (`from` after `to`, or more than 10,000 years) | `to` |
| `OUT_OF_RANGE` from `years` | `from` if that fiscal year is out of range, otherwise `to`. A number is out of range when it is below the first or above the last supported year number; a skipped number between them is not. |
| `PERIOD_NOT_FOUND` from `period` for a negative index or one too large to represent | `index` |
| `INVALID_ARGUMENT` from `period` when an `index` is given for a year, or none is given for a quarter, period or week | `index` |
| `OUT_OF_RANGE` for a `year` argument that is an integer too large for 32 bits | `year` |
| `OUT_OF_RANGE` from `parse` when at least one template match is out of range and none is in range (see [`parse`](#parse)) | `label` |
| `INVALID_DATE` | The date argument: `date`, `start` or `end_exclusive` |
| `INVALID_ARGUMENT` for a missing argument, or one of the wrong JSON type | The argument's name |
| `INVALID_ARGUMENT` for an unknown operation name | `op` |
| `INVALID_ARGUMENT` when the arguments are not a JSON object, or repeat a key | `""` (empty string) |

When validation finds a `MAJORITY_TIE` in a scheme, it does not also report `YEAR_NUMBER_COLLISION` for that scheme.

A failed operation returns `{"errors":[Error, …]}`. Validation reports every error it finds, sorted by the UTF-8 bytes of `path`, then by `code`; errors with the same path and code keep the order in which validation found them. For example, when a history entry without a transition has a `from` that fails both `CHANGE_NOT_ON_BOUNDARY` checks, the previous regime's error comes first (see [Cross-block checks](definition-v1.md#cross-block-checks)). Other operations report one error.

## Operations

| Operation | Arguments | Result |
| --- | --- | --- |
| `validate` | `definition` | `{"ok":true,"calendar_id":"…","canonical":"…"}` or errors. `FiscalCalendar::call_json("validate", args)` runs on a definition that is already valid, so it returns the `ok` result and ignores `args` once they parse as a JSON object. |
| `locate` | `date` | Location |
| `period` | `level`, `year`, `index` (omit or `null` for a year) | Period, or `PERIOD_NOT_FOUND` |
| `shift` | `period`, `n` (integer) | The period `n` steps away at the same level, crossing years as needed |
| `parent` | `period` | The enclosing period one level up: week → period when the subdivision of the week's fiscal year is `week_pattern` (a `week_based` year or a `week_pattern` transition), week → year otherwise (a `fixed_date` year or a `calendar_months` transition), period → quarter, quarter → year. A year has no parent (`INVALID_ARGUMENT`). |
| `children` | `period`, `level` | Periods at `level` inside `period`, in order. `week` children of a quarter or period are `LEVEL_NOT_NESTED` unless the subdivision of its fiscal year is `week_pattern`, because weeks cross month boundaries in a `fixed_date` year and in a `calendar_months` transition. |
| `years` | `from`, `to` (fiscal year numbers, inclusive, `from` ≤ `to`, at most 10,000 years: `to` − `from` + 1 ≤ 10,000) | Array of the year Periods whose numbers lie in [`from`, `to`], in order. It is empty only when every number in the range is skipped at a change of rule. |
| `format` | `period`, `scheme` (optional) | `{"label":"…","scheme":"…"}`, or `TEMPLATE_MISSING` when the scheme has no template for the level, or `SCHEME_NOT_FOUND` |
| `parse` | `label` | Period, or `AMBIGUOUS_LABEL`, or `LABEL_NOT_RECOGNISED` |
| `dimension` | `start`, `end_exclusive` (`start` ≤ `end_exclusive`, at most 146,097 days, which is 400 years) | Array of rows, see below; an empty range gives `[]` |
| `dimension_csv` | `start`, `end_exclusive` | The same rows as CSV text |
| `explain` | `date` | `{"date":"…","calendar_id":"…","steps":[{"level":"…","rule":"…","result":"…"},…]}`. The `rule` and `result` strings are English for people to read. Their wording is not part of the API and may change in any release. |

Argument keys that an operation does not use are ignored, so a binding may pass a whole Period where coordinates are expected.

A `period` argument is a Period value or its coordinates `{"level","year","index"}`. The core recomputes the period from its coordinates and ignores any other keys, so a stale or edited Period cannot produce wrong bounds.

### `period`

`period(level, year, index)` reads its arguments in the order `level`, `year`, `index`. A missing argument, or one of the wrong JSON type, is `INVALID_ARGUMENT` at its name; a `year` integer too large for 32 bits is `OUT_OF_RANGE` at `year`; and a negative `index`, or one too large to represent, is `PERIOD_NOT_FOUND` at `index` for a quarter, period or week (for a year, any `index` is `INVALID_ARGUMENT` at `index`, as below). With arguments that can be read, the checks run in this order, and the first that fails gives the error:

1. `year` is below the first or above the last supported year number: `OUT_OF_RANGE` at `year`.
2. `year` is a number skipped at a change of rule: `PERIOD_NOT_FOUND` at `year`.
3. An `index` is given for a year, or none is given for a quarter, period or week: `INVALID_ARGUMENT` at `index`.
4. The fiscal year has no quarter, period or week with that `index`: `PERIOD_NOT_FOUND` at `index`.

### Calendars whose rule changes

A definition with `exceptions` has a sequence of fiscal years in which the year rule changes, possibly through a transition year ([Timeline](definition-v1.md#timeline)). Every output keeps the shape described in this document. Every definition that fiscalclock 0.1.0 accepts keeps its meaning, canonical form and `calendar_id`, and every successful output is byte-identical. Definitions that 0.1.0 rejected may now be valid (`exceptions`, `{yy_end}`), and some messages and hints changed (reserved features now name 0.3).

- Each fiscal year has its own subdivision: a regime's year uses its regime's `subdivision`, and a transition uses its transition's `subdivision`. The number of quarters, periods and weeks, and so the valid indexes, can differ from one fiscal year to the next. Weeks follow the single rule in [`week`](definition-v1.md#week) in every fiscal year.
- `shift` steps through the sequence of fiscal years, and through each fiscal year's own quarters, periods or weeks. `shift` of a year by `n` moves `n` fiscal years along the sequence, not by `n` in the year number. A result outside the supported range is `OUT_OF_RANGE` at `n`, as for any calendar.
- `period` with a `year` between the first and last supported year numbers that no fiscal year has, because the number is skipped at a change of rule, is `PERIOD_NOT_FOUND` at `year`. A `year` below the first or above the last supported number is `OUT_OF_RANGE` at `year`, as for any calendar. [`period`](#period) gives the order of these checks. A `period` argument to another operation whose coordinates name a skipped number is `PERIOD_NOT_FOUND` at `period`.
- `years(from, to)` returns every fiscal year whose number lies in [`from`, `to`], in order, and can be empty only when every number in the range is skipped. The bounds for `OUT_OF_RANGE` are the first and last supported year numbers, and the limit of 10,000 years means `to` − `from` + 1 ≤ 10,000, whether or not numbers are skipped.
- `parse`: a label that names a skipped number gives no candidate from that template.
- `explain` names the regime or transition that the date falls in. As for every `explain` string, the wording is not part of the API.
- `dimension` and `dimension_csv` have the same columns as for any calendar.

### `parse`

1. Each template of each scheme is compiled to an exact, case-sensitive pattern. `{yyyy}` matches 4 digits, `{yy}`, `{yy_next}` and `{yy_end}` 2 digits, `{pp}` and `{ww}` 2 digits, and `{q}` a digit 1–9 optionally followed by a second digit (no leading zero).
2. A template *matches* the label when its pattern matches the whole label. Each match names a fiscal year number: the value of `{yyyy}`, or, in a template without `{yyyy}`, the value of `{yy}` resolved through the scheme's `century_pivot`, with the scheme's `offset` removed. The match is *out of range* when that number is below the first or above the last supported year number under that scheme (also without `offset`), and *in range* otherwise. A number skipped at a change of rule lies between the first and last supported numbers, so a match that names it is in range.
3. An in-range match yields a candidate when its values are consistent: a fiscal year has the year number, indexes are in range for that year, when `{yyyy}` and `{yy_next}` both appear, `{yy_next}` equals the last two digits of `{yyyy}` + 1, and when `{yy_end}` appears, it equals the last two digits of the calendar year of that fiscal year's last day. A match that names a skipped number yields no candidate, because no fiscal year has that number. An out-of-range match yields no candidate.
4. Candidates that denote the same period (same level and start) are merged.
5. One candidate: return its Period. More than one: `AMBIGUOUS_LABEL` with `details: {"candidates":[{"period":Period,"scheme":"…","level":"…"},…]}`, ordered by start date, then scheme name, then level (`year` first). None: `OUT_OF_RANGE` at `label` when there is at least one out-of-range match and no in-range match; otherwise `LABEL_NOT_RECOGNISED` at `label`. So a label that no template matches, or whose in-range matches all fail the checks of step 3, is `LABEL_NOT_RECOGNISED`, even when another template's match is out of range; and a label whose only matches name skipped numbers is `LABEL_NOT_RECOGNISED`.

### `dimension`

One row per day in `[start, end_exclusive)`. Columns, in order:

`date, calendar_id, fiscal_year, quarter, period, week, day_of_year, day_of_quarter, day_of_period, day_of_week, year_start, year_last_day, quarter_start, quarter_last_day, period_start, period_last_day, week_start, week_last_day, year_has_extra_week, period_has_extra_week, week_is_extra_week`

| Column | JSON type | Meaning |
| --- | --- | --- |
| `date` | string | The day, `YYYY-MM-DD`. |
| `calendar_id` | string | The calendar's `calendar_id`. |
| `fiscal_year` | integer | The `year` coordinate of the day's fiscal year: its number under the default scheme, `offset` included. |
| `quarter`, `period`, `week` | integer | The 1-based `index` of the day's quarter, period and week. |
| `day_of_year`, `day_of_quarter`, `day_of_period`, `day_of_week` | integer | As in Location. |
| `year_start`, `quarter_start`, `period_start`, `week_start` | string | The `start` of the day's year, quarter, period and week, `YYYY-MM-DD`. |
| `year_last_day`, `quarter_last_day`, `period_last_day`, `week_last_day` | string | Their `last_day`, `YYYY-MM-DD`. |
| `year_has_extra_week`, `period_has_extra_week` | boolean | `has_extra_week` of the day's year and period. |
| `week_is_extra_week` | boolean | `has_extra_week` of the day's week: true when the day is in the extra week. |

The fixed columns are never `null`. They are followed by one `label_<scheme>_<level>` column for every template, ordered by scheme name and then by level (`year`, `quarter`, `period`, `week`). A label that cannot be formatted, such as a `{yyyy}` year number outside 1–9999, is `null` in JSON and an empty field in CSV.

In JSON each row is an object with those keys in that order. CSV output follows RFC 4180 with a header row (present even when there are no rows), `\n` line endings, a final newline, booleans as `true`/`false`, and fields quoted only when they contain a comma, quote or line break.

## Error catalogue

| Code | Raised by | When |
| --- | --- | --- |
| `INVALID_JSON` | validate | The definition is not valid JSON, is not an object, or repeats a key in some object. |
| `UNKNOWN_FIELD` | validate | A field the format does not define. |
| `MISSING_FIELD` | validate | A required field is absent. |
| `INVALID_TYPE` | validate | A field has the wrong JSON type. |
| `INVALID_VALUE` | validate | A value is outside its allowed set or range. |
| `UNSUPPORTED_SPEC_VERSION` | validate | `spec_version` is not 1. |
| `UNSUPPORTED_FEATURE` | validate | A reserved part of the format is used; the hint names the planned release. |
| `UNSUPPORTED_COMBINATION` | validate | A subdivision type is not valid inside the year type. |
| `WEEKS_SUM_MISMATCH` | validate | `periods` does not sum to 52, or a `week_pattern` transition's `periods` does not sum to the transition's number of weeks. |
| `QUARTERS_SUM_MISMATCH` | validate | `quarters` does not sum to the number of periods, which for a `calendar_months` transition is its number of months. |
| `EXTRA_WEEK_MISSING` | validate | A week-based year has no `extra_week`. |
| `EXTRA_WEEK_OUT_OF_RANGE` | validate | `extra_week.period` names a period the year does not have. |
| `SHORT_MONTH_POLICY_MISSING` | validate | `start.day` is 29–31 and `short_month` is absent. |
| `SUBDIVISION_YEAR_MISMATCH` | validate | `month_start_day` differs from `year.start.day`; or a transition's subdivision does not fit its dates: a `calendar_months` transition's `month_start_day` differs from the day of `transition.start` or of `from`, or a `week_pattern` transition does not start on `week.starts_on` or is not a whole number of weeks. |
| `DEFAULT_SCHEME_MISSING` | validate | `naming.default` is not a scheme name. |
| `INVALID_TEMPLATE` | validate | A template has no year placeholder, an unknown or misplaced placeholder, or unbalanced braces. |
| `CENTURY_PIVOT_MISSING` | validate | A two-digit year template has no `century_pivot`. |
| `YEAR_NUMBER_COLLISION` | validate | A scheme gives two fiscal years the same number, skips a number between two fiscal years of the same regime, or does not increase the number across a change of rule. |
| `MAJORITY_TIE` | validate | A `majority` scheme meets a fiscal year in which no calendar year holds more than half of the days, such as one split evenly between two calendar years. |
| `EXCEPTIONS_OVERLAP` | validate | A history entry's change point (its `transition.start`, or its `from` when it has no transition) is on or before the previous entry's `from`. |
| `CHANGE_NOT_ON_BOUNDARY` | validate | A history entry's change point is not the first day of a fiscal year under the previous regime's rule, or its `from` is not the first day of a fiscal year under its own rule. |
| `INVALID_DATE` | any date argument | Not a valid `YYYY-MM-DD` civil date. |
| `TIMESTAMP_REJECTED` | bindings | A timestamp type (`datetime.datetime`, pandas `Timestamp`, JS `Date`, `Temporal.PlainDateTime`, `Temporal.ZonedDateTime` or `Temporal.Instant`) was passed. The hint says to convert it to a date in the intended time zone first. |
| `INVALID_ARGUMENT` | any | An argument is missing, of the wrong type or out of its allowed range. |
| `OUT_OF_RANGE` | any | The fiscal year involved is outside the supported range. |
| `PERIOD_NOT_FOUND` | period, shift, parent, children, format | No period has those coordinates. |
| `LEVEL_NOT_NESTED` | children | The requested level does not nest inside the period. |
| `SCHEME_NOT_FOUND` | format | No scheme has that name. |
| `TEMPLATE_MISSING` | format | The scheme has no template for the period's level. |
| `LABEL_NOT_RECOGNISED` | parse | No template matches the label. |
| `AMBIGUOUS_LABEL` | parse | More than one period matches; candidates in `details`. |
