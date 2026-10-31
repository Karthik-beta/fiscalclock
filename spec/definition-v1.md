# Calendar definition format, spec version 1

Status: draft for fiscalclock 0.1.0. Last reviewed: 31 Oct 2026.

A calendar definition is one JSON object that describes how a fiscal year is bounded, subdivided, divided into weeks and named. The library computes every answer from this object and holds no built-in calendars.

This document is normative for the parts that fiscalclock 0.1.0 implements. Parts of the format that are designed but not yet implemented are listed in [Reserved for later releases](#reserved-for-later-releases); a definition that uses them is rejected with `UNSUPPORTED_FEATURE`, so it never produces a silent wrong answer.

The key words "must", "must not" and "may" are used as in RFC 2119.

## Top level

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `spec_version` | integer | yes | Must be `1`. A higher value is rejected with `UNSUPPORTED_SPEC_VERSION`. |
| `$schema` | string | no | Ignored by the engine and excluded from the canonical form. |
| `year` | object | yes | How the year is bounded. |
| `week` | object | yes | How weeks run. |
| `subdivision` | object | yes | How the year is split into quarters and periods. |
| `naming` | object | yes | How years and periods are numbered and labelled. |
| `exceptions` | object | no | Reserved. Any value is rejected with `UNSUPPORTED_FEATURE` in 0.1.0. |

Unknown fields at any level are rejected with `UNKNOWN_FIELD`. A missing required field is rejected with `MISSING_FIELD`. A value of the wrong JSON type is rejected with `INVALID_TYPE`.

Weekday values are the lower-case English names `monday` … `sunday`. Month values are integers 1–12.

## `year`

### `type: "fixed_date"`

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `start.month` | integer 1–12 | yes | Month in which every fiscal year starts. |
| `start.day` | integer 1–31 | yes | Day of that month. |
| `short_month` | `"clamp"` or `"next"` | when `start.day` ≥ 29 | What to do when the day does not exist in a month. `clamp` uses the month's last day; `next` uses the first day of the following month. |

The fiscal year that starts in calendar year *Y* begins on `(Y, start.month, start.day)`, adjusted by `short_month` if that day does not exist. It ends the day before the next fiscal year begins.

- `start.day` ≥ 29 without `short_month` is rejected with `SHORT_MONTH_POLICY_MISSING`.
- `short_month` when `start.day` ≤ 28 is rejected with `INVALID_VALUE`, because it would never apply.

`short_month` exists only on `fixed_date` years, and the canonical form writes it as `null` there when it is absent.

### `type: "week_based"`

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `anchor.edge` | `"end"` or `"start"` | yes | Whether the rule fixes the year's last day or its first day. |
| `anchor.month` | integer 1–12 | yes | Month of the target date. |
| `anchor.day` | integer 1–28 or `"last"` | yes | Day of the target date. `"last"` is the month's last day. |
| `anchor.rule` | `"last"`, `"first"` or `"nearest"` | yes | `last`: the latest qualifying weekday on or before the target date. `first`: the earliest on or after it. `nearest`: the one closest to it (never a tie, because it is at most three days away). |

The anchor has no weekday field. A week-based year consists of whole weeks, so its first day is always `week.starts_on` and its last day is always the weekday before `week.starts_on`. The qualifying weekday for `edge: "end"` is therefore the day before `week.starts_on`, and for `edge: "start"` it is `week.starts_on` itself. This removes the possibility of a year-end weekday that disagrees with the week definition.

For each calendar year *Y*, the target date is `(Y, anchor.month, anchor.day)` and the rule gives one boundary date *B(Y)*.

- With `edge: "end"`, the fiscal year with anchor year *Y* runs from *B(Y − 1)* + 1 day to *B(Y)* inclusive.
- With `edge: "start"`, it runs from *B(Y)* to *B(Y + 1)* − 1 day inclusive.

Every such year has 364 or 371 days. A 371-day year has a 53rd week, called the extra week.

`anchor.day` above 28 is rejected with `INVALID_VALUE`, with a hint to use `"last"`, because days 29–31 do not exist in every month in every year.

Examples:

- The NRF 4-5-4 calendar ends on the Saturday nearest 31 January: `{"edge": "end", "month": 1, "day": "last", "rule": "nearest"}` with `week.starts_on: "sunday"`.
- Apple's fiscal year ends on the last Saturday of September: `{"edge": "end", "month": 9, "day": "last", "rule": "last"}` with `week.starts_on: "sunday"`.
- The ISO 8601 week-numbering year starts on the Monday nearest 1 January: `{"edge": "start", "month": 1, "day": 1, "rule": "nearest"}` with `week.starts_on: "monday"`. Its years must be numbered with `year_number: "majority"` (see [Year numbers must be unique](#year-numbers-must-be-unique)).

## `week`

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `starts_on` | weekday | yes | First day of every week at every level. |
| `numbering` | `"contains_start"` | yes | How week 1 is chosen. `"first_full"` is reserved and rejected with `UNSUPPORTED_FEATURE` in 0.1.0. |

`numbering` has no default, because organisations differ.

- In a `week_based` year, week 1 starts on the year's first day and every week has 7 days. Weeks are numbered 1–52, or 1–53 in a year with an extra week.
- In a `fixed_date` year with `contains_start`, week 1 runs from the year's first day to the day before the next `starts_on`, so it may have 1–7 days. Each later week starts on `starts_on`. The last week ends on the year's last day and may also be partial. A year has 53 or 54 weeks.

## `subdivision`

### `type: "calendar_months"`

Valid only inside a `fixed_date` year. Inside a `week_based` year it is rejected with `UNSUPPORTED_COMBINATION`.

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `month_start_day` | integer 1–31 | yes | Day on which each fiscal month (period) starts. Must equal `year.start.day`, otherwise `SUBDIVISION_YEAR_MISMATCH`. |
| `quarters` | array of positive integers | yes | Number of periods in each quarter, in order. Must sum to 12, otherwise `QUARTERS_SUM_MISMATCH`. |

Period *k* (1–12) starts on `month_start_day` of the *k*-th month counted from `year.start.month`, adjusted by `year.short_month` when that day does not exist. It ends the day before period *k* + 1 starts, and period 12 ends on the year's last day.

### `type: "week_pattern"`

Valid only inside a `week_based` year in 0.1.0. Inside a `fixed_date` year it needs `stub`, which is reserved, so it is rejected with `UNSUPPORTED_COMBINATION`.

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `periods` | array of 12 or 13 positive integers | yes | Weeks in each period. Must sum to 52, otherwise `WEEKS_SUM_MISMATCH`. Any other length is `INVALID_VALUE`. |
| `quarters` | array of positive integers | yes | Number of periods in each quarter. Must sum to the length of `periods`, otherwise `QUARTERS_SUM_MISMATCH`. |
| `extra_week` | object | yes | Where the 53rd week goes. Missing is `EXTRA_WEEK_MISSING`, because every week-based rule produces 53-week years (71 in every 400 years). |
| `extra_week.period` | integer | yes | The period that gains the extra week as its last week. Outside 1 … length of `periods` is `EXTRA_WEEK_OUT_OF_RANGE`. |

In a 53-week year, period `extra_week.period` has one more week than `periods` says, and the extra week is that period's last week.

### Quarters

Quarter *q* consists of the periods given by `quarters[q − 1]`, taken in order. A quarter's length comes only from its periods. No rule assumes 13-week quarters.

## `naming`

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `default` | string | yes | Name of the scheme used for coordinates and for `format` when no scheme is given. Must be a key of `schemes`, otherwise `DEFAULT_SCHEME_MISSING`. |
| `schemes` | object | yes | One or more named schemes. Must not be empty (`INVALID_VALUE`). Scheme names match `^[a-z][a-z0-9_]{0,31}$`, otherwise `INVALID_VALUE`. |

Each scheme:

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `year_number` | `"start"`, `"end"` or `"majority"` | yes | Number a fiscal year by the calendar year of its first day, of its last day, or that holds more than half of its days. `"table"` is reserved (`UNSUPPORTED_FEATURE`). |
| `offset` | integer | no, default 0 | Added to the year number, for example 1 for an Indian assessment year or 543 for a Buddhist Era year. |
| `century_pivot` | integer 100–9999 | see below | Resolves two-digit years when parsing. A two-digit value *yy* means the unique year in [`century_pivot` − 99, `century_pivot`] whose last two digits are *yy*. |
| `templates` | object | yes | Label templates keyed by level: `year`, `quarter`, `period`, `week`. At least one. Unknown keys are `UNKNOWN_FIELD`. |
| `eras` | any | no | Reserved (`UNSUPPORTED_FEATURE`). |

### Templates

A template is literal text with placeholders.

| Placeholder | Value | Allowed in |
| --- | --- | --- |
| `{yyyy}` | Year number, zero-padded to 4 digits | every level |
| `{yy}` | Last two digits of the year number | every level |
| `{yy_next}` | Last two digits of the year number plus one | every level, only together with `{yyyy}` or `{yy}` |
| `{q}` | Quarter index, unpadded (no leading zero) | `quarter` |
| `{pp}` | Period index, zero-padded to 2 digits | `period` |
| `{ww}` | Week index, zero-padded to 2 digits | `week` |

- Every template must contain `{yyyy}` or `{yy}`, so that a label identifies its year. `{yy_next}` alone is not enough. Otherwise `INVALID_TEMPLATE`.
- An unknown placeholder, an unbalanced brace, or a placeholder used outside the levels shown is `INVALID_TEMPLATE`.
- A template that contains `{yy}` but not `{yyyy}` needs `century_pivot`, otherwise `CENTURY_PIVOT_MISSING`. The pivot resolves `{yy}`, and `{yy_next}` is then checked against it.
- Year numbers above 9999 or below 1 after `offset` cannot be formatted with `{yyyy}`; `format` returns `OUT_OF_RANGE`.

### Year numbers must be unique

Every scheme must give consecutive fiscal years consecutive numbers across the whole supported range. Validation computes the number of every fiscal year in the range under every scheme and rejects:

- `YEAR_NUMBER_COLLISION` when two fiscal years get the same number or a number is skipped. This happens with `"start"` or `"end"` when the year boundary can fall on either side of 1 January. For example, ISO week years numbered by `"start"` give both the year starting Monday 1 January 2018 and the year starting Monday 31 December 2018 the number 2018. `details.years` lists the first affected years, and the hint suggests another `year_number`.
- `MAJORITY_TIE` when `"majority"` is used and some fiscal year has exactly half of its days in each of two calendar years. `details.years` lists the first affected years.

The path is `naming.schemes.<name>.year_number`.

### Coordinates

Every period has the coordinates `(level, year, index)`. `year` is the year number under the default scheme, including its `offset`. `index` is 1-based within the fiscal year and is `null` for the year itself.

## Supported range

The public API covers fiscal years that lie wholly between 0001-01-02 and 9999-12-30. A date in, or a request for, any other fiscal year returns `OUT_OF_RANGE`. This keeps every `end_exclusive` representable in Python, where `date(9999, 12, 31) + timedelta(days=1)` raises `OverflowError`.

## Canonical form and `calendar_id`

The canonical form is the definition after parsing, serialised as JSON with:

- `$schema` removed;
- optional fields with a default written out (`offset: 0`), and optional fields without a default written as `null` (`short_month`, `century_pivot`); template levels that are absent stay absent;
- object keys sorted by their UTF-8 bytes at every level;
- no insignificant whitespace, integers in plain decimal, strings with the minimal JSON escapes: `\"`, `\\`, `\b`, `\f`, `\n`, `\r`, `\t`, other characters U+0000 to U+001F as `\u00xx` with lower-case hexadecimal digits, and every other character, including non-ASCII, written as UTF-8.

`calendar_id` is the first 16 lower-case hexadecimal characters of the SHA-256 digest of the canonical form's UTF-8 bytes. Two definitions that differ only in key order, whitespace, `$schema` or written-out defaults have the same `calendar_id`.

## Reserved for later releases

These parts of the format are designed in the [research proposal](https://github.com/Karthik-beta/fiscalclock/wiki/Definition-Format) but not implemented in 0.1.0. Each is rejected with `UNSUPPORTED_FEATURE` and a hint naming the planned release.

| Field or value | Planned |
| --- | --- |
| `year.type: "table"` | 0.2 |
| `subdivision.type: "anchored"` | 0.2 |
| `subdivision.stub` | 0.2, after ADR-0024 |
| `subdivision.special_periods` | 0.2 |
| `extra_week.standalone`, `extra_week.by_year` | 0.2 |
| `week.numbering: "first_full"` | 0.2 |
| `naming.schemes.*.year_number: "table"`, `eras` | 0.2 |
| `exceptions` (`overrides`, `history`, `transitions`) | 0.2 |
