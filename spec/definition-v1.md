# Calendar definition format, spec version 1

Status: draft for fiscalclock 0.3.0. Last reviewed: 26 Nov 2026.

A calendar definition is one JSON object that describes how a fiscal year is bounded, subdivided, divided into weeks and named, and, optionally, how that rule changes on a date. The library computes every answer from this object and holds no built-in calendars.

This document is normative for the parts that fiscalclock implements on its `main` branch, which become 0.3.0. Parts of the format that are designed but not yet implemented are listed in [Reserved for later releases](#reserved-for-later-releases); a definition that uses them is rejected with `UNSUPPORTED_FEATURE`, so it never produces a silent wrong answer. [ADR-0032](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0032-year-rule-history-and-transitions) records the design of [`exceptions`](#exceptions), new in 0.2.0. Every definition that fiscalclock 0.1.0 accepts keeps its meaning, canonical form and `calendar_id`, and every successful output is byte-identical. Definitions that 0.1.0 rejected may now be valid (`exceptions`, `{yy_end}`), and some messages and hints changed (reserved features now name 0.3). The same holds from 0.2.0 to 0.3.0: table years and the `table` subdivision are new, and `year_number: "table"` changes from `UNSUPPORTED_FEATURE` to `INVALID_VALUE`, as [ADR-0022](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0022-table-years-with-period-boundaries) decides.

The key words "must", "must not" and "may" are used as in RFC 2119.

## Top level

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `spec_version` | integer | yes | Must be `1`. Any other integer is rejected with `UNSUPPORTED_SPEC_VERSION`, and that is then the only error reported, because the rest of the definition may follow another version's rules. |
| `$schema` | string | no | Ignored by the engine and excluded from the canonical form. |
| `year` | object | yes | How the year is bounded. |
| `week` | object | yes | How weeks run. |
| `subdivision` | object | yes | How the year is split into quarters and periods. |
| `naming` | object | yes | How years and periods are numbered and labelled. |
| `exceptions` | object or `null` | no, no default | Changes of the year rule on given dates, with optional transition periods. See [`exceptions`](#exceptions). Absent or `null` means the top-level rule applies to every fiscal year. |

Unknown fields at any level are rejected with `UNKNOWN_FIELD`. A missing required field is rejected with `MISSING_FIELD`. A value of the wrong JSON type is rejected with `INVALID_TYPE`.

An object that repeats a key is rejected with `INVALID_JSON` at path `""`. RFC 8259 leaves the meaning of a repeated key to the parser, so a definition with two `year` blocks has more than one reading.

`null` for an optional field without a default (`short_month`, `century_pivot`, a history entry's `transition`) means the field is absent, so the canonical form, which writes them as `null`, validates again. `null` for `exceptions` also means absent; the canonical form leaves `exceptions` out in that case. `null` for a reserved field (`exceptions.overrides`, `eras`) is still `UNSUPPORTED_FEATURE`.

Weekday values are the lower-case English names `monday` … `sunday`. Month values are integers 1–12.

## `year`

A `year` block together with a `subdivision` block is a *rule*: it produces a fiscal year for every calendar year, as described below. A rule in which either block is a `table` produces only the fiscal years its table covers. Without `exceptions`, the top-level rule produces every fiscal year of the calendar. With `exceptions`, each rule is in force only for part of the timeline; see [Timeline](#timeline).

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

- The NRF 4-5-4 calendar ends on the Saturday nearest 31 January: `{"edge": "end", "month": 1, "day": "last", "rule": "nearest"}` with `week.starts_on: "sunday"`. The [NRF 4-5-4 calendar page](https://nrf.com/resources/4-5-4-calendar) does not state the rule in these words. This anchor gives exactly the 53-week years the page lists for 2006–2023 (check C19 in [`research/verification/`](../research/verification/README.md) and the vector [`nrf-53-week-years.json`](../conformance/vectors/cited/nrf-53-week-years.json)).
- Apple's fiscal year ends on the last Saturday of September: `{"edge": "end", "month": 9, "day": "last", "rule": "last"}` with `week.starts_on: "sunday"`. Apple states the rule in its [Form 10-Q for the quarter ended 30 December 2023](https://www.sec.gov/Archives/edgar/data/320193/000032019324000006/aapl-20231230.htm).
- The ISO 8601 week-numbering year starts on the Monday nearest 1 January: `{"edge": "start", "month": 1, "day": 1, "rule": "nearest"}` with `week.starts_on: "monday"`. This anchor matches Python's `date.isocalendar()` for every year from 1600 to 2399 (check C15 in [`research/verification/`](../research/verification/README.md)). Its years must be numbered with `year_number: "majority"` (see [Year numbers must be unique](#year-numbers-must-be-unique)).

### `type: "table"`

The fiscal years are listed instead of following a rule ([ADR-0022](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0022-table-years-with-period-boundaries)).

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `boundaries` | array of at least 2 `YYYY-MM-DD` date strings | yes | The first day of each fiscal year, in order, followed by the day after the last listed year's last day. |

Fiscal year *k* (from 0) runs from `boundaries[k]` to the day before `boundaries[k + 1]`. The rule produces only these years: the calendar is not defined before `boundaries[0]` or from the last entry on, and a date there returns `OUT_OF_RANGE`.

- Fewer than 2 entries is `INVALID_VALUE` at `year.boundaries`. An entry that is not a string is `INVALID_TYPE`, and one that is not a `YYYY-MM-DD` date from 0001-01-02 to 9999-12-31 is `INVALID_VALUE`, each at `year.boundaries[k]`.
- Each entry must be after the one before it. An entry on or before its predecessor is `TABLE_NOT_INCREASING` at `year.boundaries[k]`, once for every such *k*. Only two neighbouring entries that are both valid dates are compared, so an invalid entry gives its own error and no `TABLE_NOT_INCREASING` beside it.
- A table year needs a `table` subdivision. With `calendar_months` or `week_pattern` it is `UNSUPPORTED_COMBINATION` at `subdivision.type`.
- Year numbers come from `naming`, as for every other year type. A table year has no numbers of its own.

## `week`

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `starts_on` | weekday | yes | First day of every week at every level. |
| `numbering` | `"contains_start"` | yes | How week 1 is chosen. `"first_full"` is reserved and rejected with `UNSUPPORTED_FEATURE`. |

`numbering` has no default, because organisations differ.

There is one `week` block for the whole calendar, including every history entry and transition in `exceptions`. With `contains_start`, the weeks of every fiscal year follow one rule, whatever its year type or subdivision:

- Week 1 runs from the year's first day to the day before the first `starts_on` day after the year's first day, so it has 1–7 days.
- Each later week starts on `starts_on` and has 7 days, except the last week, which ends on the year's last day and may be partial.
- Weeks are numbered from 1 within the fiscal year.

This gives, for each kind of fiscal year:

- A `week_based` year starts on `starts_on` and has 364 or 371 days, so every week has 7 days. Weeks are numbered 1–52, or 1–53 in a year with an extra week.
- In a `fixed_date` year, week 1 and the last week may be partial. A year has 53 or 54 weeks.
- A transition (see [Transitions](#transitions)) has as many weeks as the rule gives for its dates. A `week_pattern` transition starts on `starts_on` and lasts whole weeks, so every week in it has 7 days.

For a definition without `exceptions`, this is the same rule as in fiscalclock 0.1.0.

## `subdivision`

### `type: "calendar_months"`

Valid only inside a `fixed_date` year. Inside a `week_based` or `table` year it is rejected with `UNSUPPORTED_COMBINATION`. A transition has its own `calendar_months` subdivision with different checks; see [Transitions](#transitions).

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `month_start_day` | integer 1–31 | yes | Day on which each fiscal month (period) starts. Must equal `year.start.day`, otherwise `SUBDIVISION_YEAR_MISMATCH`. |
| `quarters` | array of positive integers | yes | Number of periods in each quarter, in order. Must sum to 12, otherwise `QUARTERS_SUM_MISMATCH`. |

Period *k* (1–12) starts on `month_start_day` of the *k*-th month counted from `year.start.month`, adjusted by `year.short_month` when that day does not exist. It ends the day before period *k* + 1 starts, and period 12 ends on the year's last day.

### `type: "week_pattern"`

Valid only inside a `week_based` year. Inside a `fixed_date` year it needs `stub`, which is reserved, so it is rejected with `UNSUPPORTED_COMBINATION`, as it is inside a `table` year. A transition has its own `week_pattern` subdivision, without `extra_week`; see [Transitions](#transitions).

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `periods` | array of 12 or 13 positive integers | yes | Weeks in each period. Must sum to 52, otherwise `WEEKS_SUM_MISMATCH`. Any other length is `INVALID_VALUE`. |
| `quarters` | array of positive integers | yes | Number of periods in each quarter. Must sum to the length of `periods`, otherwise `QUARTERS_SUM_MISMATCH`. |
| `extra_week` | object | yes | Where the 53rd week goes. Missing is `EXTRA_WEEK_MISSING`, because every week-based rule produces 53-week years (71 in every 400 years). |
| `extra_week.period` | integer | yes | The period that gains the extra week as its last week. Outside 1 … length of `periods` is `EXTRA_WEEK_OUT_OF_RANGE`. |

In a 53-week year, period `extra_week.period` has one more week than `periods` says, and the extra week is that period's last week.

### `type: "table"`

The periods are listed instead of following a rule ([ADR-0022](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0022-table-years-with-period-boundaries)). Valid inside every year type. A transition cannot use it: `"table"` there is `INVALID_VALUE`, as any other unknown type is.

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `boundaries` | array of at least 2 `YYYY-MM-DD` date strings | yes | The first day of each period, in order, followed by the day after the last listed period's last day. The entry rules of a table year's `boundaries` apply, at `subdivision.boundaries[k]`, including `TABLE_NOT_INCREASING`. |
| `quarters` | array of positive integers | yes | Number of periods in each quarter. Every fiscal year has as many periods as `quarters` sums to. |

Period *k* of a fiscal year is the *k*-th listed period that starts on or after the year's first day. The listed periods must tile the fiscal years of the `year` block exactly, which the [cross-block checks](#cross-block-checks) verify:

- `boundaries[0]` and the last entry must each be the first day of a fiscal year under the `year` block, or, for a table year, the day after its last listed year;
- every first day of a fiscal year between them must be listed;
- each fiscal year between them must hold exactly as many listed periods as `quarters` sums to.

The rule then produces only the fiscal years between `boundaries[0]` and the last entry, so the calendar is defined only there, whatever the `year` block would produce outside them. This narrowing is the one effect a `table` subdivision has on another block.

A `table` subdivision does not split periods into weeks. Weeks are counted from the fiscal year's first day, as in a `fixed_date` year, and do not nest in quarters or periods.

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
| `year_number` | `"start"`, `"end"` or `"majority"` | yes | Number a fiscal year by the calendar year of its first day, of its last day, or that holds more than half of its days. Any other value, `"table"` included, is `INVALID_VALUE`: table years are numbered by these rules too. |
| `offset` | integer −1,000,000,000 to 1,000,000,000 | no, default 0 | Added to the year number, for example 1 to label each fiscal year by the following calendar year. Outside the range is `INVALID_VALUE`, so that every year number fits a 32-bit integer. |
| `century_pivot` | integer 100–9999 | see below | Resolves two-digit years when parsing. A two-digit value *yy* means the unique year in [`century_pivot` − 99, `century_pivot`] whose last two digits are *yy*. |
| `templates` | object | yes | Label templates keyed by level: `year`, `quarter`, `period`, `week`. At least one; an empty object is `INVALID_VALUE` at `naming.schemes.<name>.templates`. Unknown keys are `UNKNOWN_FIELD`. |
| `eras` | any | no | Reserved (`UNSUPPORTED_FEATURE`). |

### Templates

A template is literal text with placeholders.

| Placeholder | Value | Allowed in |
| --- | --- | --- |
| `{yyyy}` | Year number, zero-padded to 4 digits | every level |
| `{yy}` | Last two digits of the year number | every level |
| `{yy_next}` | Last two digits of the year number plus one | every level, only together with `{yyyy}` or `{yy}` |
| `{yy_end}` | Last two digits, zero-padded, of the calendar year in which the fiscal year's last day falls (the fiscal year's, not the period's, at every level). It does not depend on `year_number` or `offset`. | every level, only together with `{yyyy}` or `{yy}` |
| `{q}` | Quarter index, unpadded (no leading zero) | `quarter` |
| `{pp}` | Period index, zero-padded to 2 digits | `period` |
| `{ww}` | Week index, zero-padded to 2 digits | `week` |

- Every template must contain `{yyyy}` or `{yy}`, so that a label identifies its year. `{yy_next}` or `{yy_end}` alone, or the two together, is not enough. Otherwise `INVALID_TEMPLATE`.
- A `quarter`, `period` or `week` template must contain its index placeholder (`{q}`, `{pp}` or `{ww}`), otherwise its labels cannot tell the periods of one year apart. Otherwise `INVALID_TEMPLATE`.
- An unknown placeholder, an unbalanced brace, or a placeholder used outside the levels shown is `INVALID_TEMPLATE`.
- A template that contains `{yy}` but not `{yyyy}` needs `century_pivot`, otherwise `CENTURY_PIVOT_MISSING`. The pivot resolves `{yy}`, and `{yy_next}` is then checked against it. `{yy_end}` never needs a pivot: `parse` checks it against the fiscal year that the other placeholders name.
- Year numbers above 9999 or below 1 after `offset` cannot be formatted with `{yyyy}`; `format` returns `OUT_OF_RANGE`.
- An index of 100 or more, which only a long transition can have, is written in full by `{q}`, `{pp}` and `{ww}` (for example `105`). `parse` matches at most two digits for these placeholders, so it does not recognise such a label.

`FY {yyyy}-{yy_end}` with `year_number: "start"` gives "FY 2024-26" for the Siemens Limited transition year from 1 October 2024 to 31 March 2026, which the company calls "FY 2024-26" ([`research/filers/siemens-india.md`](../research/filers/siemens-india.md)). For every twelve-month fiscal year from April to March numbered by `"start"` with `offset` 0, `{yy_end}` and `{yy_next}` give the same digits.

### Year numbers must be unique

Every scheme must number the fiscal years of the supported range so that each number names one fiscal year. Validation computes the number of every fiscal year from the first to the last fiscal year inside the supported range, under every scheme. It does this only once the definition is otherwise valid, including the [cross-block checks](#cross-block-checks) of `exceptions`, because the numbers depend on the year boundaries. For each pair of consecutive fiscal years in the supported range:

- When both years are produced by the same regime's rule (see [Timeline](#timeline)), the later year's number must be the earlier year's number plus one.
- When a change of rule lies between them (one of the two is a transition, or they belong to different regimes), the later year's number must be greater than the earlier year's. A skipped number is allowed only here.

A definition without `exceptions` has one regime, so every scheme must give consecutive fiscal years consecutive numbers across the whole supported range, exactly as in fiscalclock 0.1.0. With `exceptions`, Siemens Limited's calendar numbered by `"start"` runs 2023 (1 October 2023 to 30 September 2024), 2024 (the transition year, 1 October 2024 to 31 March 2026) and 2026 (1 April 2026 to 31 March 2027); no fiscal year has the number 2025. The dates are those stated in [`research/filers/siemens-india.md`](../research/filers/siemens-india.md); the numbers follow from `"start"`.

Validation rejects:

- `YEAR_NUMBER_COLLISION` when a pair breaks these rules: two fiscal years get the same number, a number is skipped between two years of the same regime, or a number does not increase across a change of rule. Within one rule, this happens with `"start"` or `"end"` when the year boundary can fall on either side of 1 January. For example, ISO week years numbered by `"start"` give both the year starting Monday 1 January 2018 and the year starting Monday 31 December 2018 the number 2018. The hint suggests a `year_number` that works.
- `MAJORITY_TIE` when `"majority"` is used and some fiscal year, transitions included, has no calendar year that holds more than half of its days. For a fiscal year of at most 371 days this happens only when it has exactly half of its days in each of two calendar years; a long transition can also span three or more calendar years without a majority. A scheme with a tie is not also reported for collisions.

`details.years` of `YEAR_NUMBER_COLLISION` lists the first five distinct affected numbers, in the date order of the pairs that break the rules, with the scheme's `offset` included. The affected number of a pair is:

- within one regime, when the later year repeats the earlier year's number: the repeated number;
- within one regime, for any other break: the earlier year's number plus one, which is the first skipped number;
- across a change of rule: the later year's number. With `"start"`, `"end"` and `"majority"` this is always a repeated number, because those rules never give a later fiscal year a smaller number.

`details.years` of `MAJORITY_TIE` lists, for the first five tied fiscal years in date order, the calendar year in which each starts, without `offset`, as in fiscalclock 0.1.0.

The path is `naming.schemes.<name>.year_number`.

### Coordinates

Every period has the coordinates `(level, year, index)`. `year` is the year number under the default scheme, including its `offset`. `index` is 1-based within the fiscal year and is `null` for the year itself.

## `exceptions`

`exceptions` describes a calendar whose year rule changes on a date, with an optional transition period between the old rule and the new one. The design is recorded in [ADR-0032](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0032-year-rule-history-and-transitions).

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `history` | array of history entries | yes | The changes of rule, in date order. An empty array is `INVALID_VALUE` at `exceptions.history`. The order is kept. |
| `overrides` | any | no | Reserved. Any value, `null` included, is `UNSUPPORTED_FEATURE`. |

### History entries

Each entry of `history` has exactly these fields:

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `from` | date | yes | The first day of the first fiscal year under the entry's rule. |
| `year` | object | yes | The year rule from `from` on, with the fields and checks of the top-level [`year`](#year). |
| `subdivision` | object | yes | The subdivision of those years, with the fields and checks of the top-level [`subdivision`](#subdivision). |
| `transition` | object or `null` | no, no default | A transition period that ends the day before `from`. Absent or `null` means the entry has none. |

A date in `exceptions` is a JSON string in ISO 8601 extended format, `YYYY-MM-DD`, that names a valid proleptic Gregorian date from 0002-01-01 to 9998-12-31 inclusive. A value that is not a string is `INVALID_TYPE`; any other string, including one outside that range, is `INVALID_VALUE`.

An entry's `year` and `subdivision` are validated exactly as the top-level blocks, including reserved features (`UNSUPPORTED_FEATURE`), `UNSUPPORTED_COMBINATION`, `SHORT_MONTH_POLICY_MISSING`, `SUBDIVISION_YEAR_MISMATCH` (against the entry's own `year.start.day`), `WEEKS_SUM_MISMATCH`, `QUARTERS_SUM_MISMATCH`, `EXTRA_WEEK_MISSING` and `EXTRA_WEEK_OUT_OF_RANGE`. Each path is the top-level path prefixed with `exceptions.history[i].`, where *i* is the entry's 0-based index: for example `exceptions.history[0].subdivision.month_start_day`.

The `week` and `naming` blocks are global: there is one of each for the whole calendar, and an entry has no `week` or `naming` field (`UNKNOWN_FIELD`). This is a stated dependency between blocks: a `week_based` year in an entry takes its weekday from the top-level `week.starts_on`, exactly as a top-level `week_based` year does.

### Transitions

A transition is one fiscal year, `[transition.start, from)`, between the last fiscal year of the previous regime and the entry's first fiscal year.

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `start` | date | yes | The transition's first day. It must be before the entry's `from`; on or after `from` is `INVALID_VALUE` at `exceptions.history[i].transition.start`. |
| `subdivision` | object | yes | How the transition is split into quarters and periods: `type` `"calendar_months"` or `"week_pattern"`, with the fields below. |

A transition has no `year` block, so the top-level `UNSUPPORTED_COMBINATION` rules do not apply to it: either subdivision type is allowed whatever the regimes on each side. The reserved parts of a subdivision are rejected as at the top level: `type: "anchored"` and the fields `stub` and `special_periods` are `UNSUPPORTED_FEATURE`, and any other unknown `type`, including `"table"`, is `INVALID_VALUE`.

`calendar_months` in a transition:

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `type` | `"calendar_months"` | yes | |
| `month_start_day` | integer 1–28 | yes | Day on which each period starts. Outside 1–28 is `INVALID_VALUE`. Must equal the day of `transition.start` and the day of `from`, otherwise `SUBDIVISION_YEAR_MISMATCH`. |
| `quarters` | array of positive integers | yes | Number of periods in each quarter. Must sum to the transition's number of months, otherwise `QUARTERS_SUM_MISMATCH`. |

The transition's number of months is (12 × the year of `from` + the month of `from`) − (12 × the year of `transition.start` + the month of `transition.start`). Period *k* starts on `month_start_day` of the *k*-th month counted from the month of `transition.start`, which is month 1. It ends the day before period *k* + 1 starts, and the last period ends on the transition's last day, the day before `from`.

`week_pattern` in a transition:

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `type` | `"week_pattern"` | yes | |
| `periods` | array of one or more positive integers, of any length | yes | Weeks in each period. An empty array is `INVALID_VALUE`. Must sum to the transition's number of weeks, otherwise `WEEKS_SUM_MISMATCH`. |
| `quarters` | array of positive integers | yes | Number of periods in each quarter. Must sum to the length of `periods`, otherwise `QUARTERS_SUM_MISMATCH`. |

A transition `week_pattern` has no `extra_week`; the field is `UNKNOWN_FIELD` there. The transition must start on a `week.starts_on` day and last a whole number of weeks, otherwise `SUBDIVISION_YEAR_MISMATCH`; its number of weeks is its number of days divided by 7. Period *k* is the run of `periods[k − 1]` whole weeks that follows period *k* − 1, and period 1 starts on `transition.start`.

Quarters in a transition are formed from its periods as described in [Quarters](#quarters).

### Timeline

The regimes of a calendar are the top-level `year` and `subdivision` (regime 0), then the `year` and `subdivision` of history entry *i* (regime *i* + 1). Regime 0 is in force from the start of the supported range; regime *i* + 1 is in force from entry *i*'s `from`. A rule with a table produces only the fiscal years its table covers, so its regime has no years outside them.

The change point *c*(*i*) of history entry *i* is its `transition.start` when it has a transition, and its `from` otherwise.

- Regime 0's fiscal years are the years its rule produces that end on or before *c*(0).
- Regime *i* + 1's fiscal years are the years its rule produces that start on or after entry *i*'s `from` and end on or before *c*(*i* + 1). The last regime has no upper bound other than its table's, so a calendar whose last regime has a table ends where that table ends.
- Entry *i*'s transition, when it has one, is the fiscal year `[transition.start, from)`.

The fiscal years of the calendar are, in date order: the years of regime 0, then for each history entry in order its transition (if any) and the years of its regime. Validation guarantees that they tile time with no gap and no overlap. Each fiscal year has its own subdivision: a regime's year uses its regime's `subdivision`, and a transition uses its transition's `subdivision`. The number of quarters, periods and weeks can therefore differ from one fiscal year to the next.

### Cross-block checks

Validation runs in three stages: block-level checks, then cross-block checks, then year numbering. The cross-block checks are exactly the rows of the table below, and year numbering is described in [Year numbers must be unique](#year-numbers-must-be-unique). Every other check is block-level. "Block-level" names the stage, not the number of blocks a check reads: it covers types, required and unknown fields, values and their ranges, reserved features and combinations, in every block and history entry, and also `SUBDIVISION_YEAR_MISMATCH` against `year.start.day`, `DEFAULT_SCHEME_MISSING`, `QUARTERS_SUM_MISMATCH`, `WEEKS_SUM_MISMATCH`, `EXTRA_WEEK_MISSING`, `EXTRA_WEEK_OUT_OF_RANGE`, `INVALID_TEMPLATE`, `CENTURY_PIVOT_MISSING` and `SHORT_MONTH_POLICY_MISSING`, at the top level and in history entries, except the transition rows of the table below. Block-level checks include `transition.start` on or after `from`, an empty transition `periods`, and a transition `week_pattern`'s `quarters` that do not sum to the length of its `periods` (checked when `periods` is otherwise valid). Every error they find is reported together. The checks that compare a transition's subdivision with its dates are cross-block, although the field tables above mention them; they are in the table below.

Only when the definition has no block-level error do the following checks run. Every error they find is reported.

For every rule with a `table` subdivision (the top level, and every history entry with its paths prefixed by `exceptions.history[i].`), where *n* is the index of the last entry of `subdivision.boundaries`:

| Condition | Error |
| --- | --- |
| `subdivision.boundaries[0]` is not the first day of a fiscal year under the `year` block | `SUBDIVISION_YEAR_MISMATCH` at `subdivision.boundaries[0]` |
| `subdivision.boundaries[n]` is not the first day of a fiscal year under the `year` block, nor, for a table year, the day after its last listed year | `SUBDIVISION_YEAR_MISMATCH` at `subdivision.boundaries[n]` |
| Only when neither row above finds an error: a first day of a fiscal year between `boundaries[0]` and `boundaries[n]` is not listed | `SUBDIVISION_YEAR_MISMATCH` at `subdivision.boundaries`; the message names the earliest such day |
| Only when no row above finds an error: a fiscal year between `boundaries[0]` and `boundaries[n]` does not hold exactly as many listed periods as `quarters` sums to | `QUARTERS_SUM_MISMATCH` at `subdivision.quarters`; the message names the earliest such year |

For every history entry *i*: "The previous regime" of entry *i* is regime 0 when *i* = 0 and history entry *i* − 1 otherwise. "The first day of a fiscal year under a rule" means the first day of one of the years that the rule produces for any calendar year, regardless of the regime's own bounds. For a rule with a table it means the first day of one of the years the table covers, where the table covers the dates from `subdivision.boundaries[0]` to the day before its last entry, whether or not the table rows above find an error; for *c*(*i*) under the previous regime's rule, the day after its table's last year counts too, because that regime may end where its table ends.

Each path in the Error column is relative to the entry, `exceptions.history[i]`: for example `transition.start` stands for `exceptions.history[i].transition.start`.

| Condition | Error |
| --- | --- |
| *i* ≥ 1 and *c*(*i*) is on or before entry *i* − 1's `from` (the regimes would overlap, or the previous regime would have no fiscal year) | `EXCEPTIONS_OVERLAP` at the field that holds *c*(*i*): `transition.start` when the entry has a transition, otherwise `from` |
| *c*(*i*) is not the first day of a fiscal year under the previous regime's rule | `CHANGE_NOT_ON_BOUNDARY` at the field that holds *c*(*i*), as above |
| `from` is not the first day of a fiscal year under the entry's own rule | `CHANGE_NOT_ON_BOUNDARY` at `from` |
| Transition `calendar_months`: `month_start_day` differs from the day of `transition.start` or from the day of `from` | `SUBDIVISION_YEAR_MISMATCH` at `transition.subdivision.month_start_day` |
| Transition `calendar_months`, only when the row above finds no error: `quarters` does not sum to the transition's number of months | `QUARTERS_SUM_MISMATCH` at `transition.subdivision.quarters` |
| Transition `week_pattern`: `transition.start` is not a `week.starts_on` day, or the transition's number of days is not a multiple of 7 (one error when both hold) | `SUBDIVISION_YEAR_MISMATCH` at `transition.subdivision.type` |
| Transition `week_pattern`, only when its number of days is a multiple of 7: `periods` does not sum to the transition's number of weeks | `WEEKS_SUM_MISMATCH` at `transition.subdivision.periods` |

The first three checks run independently, so one entry can report more than one error. When an entry has no transition and its `from` fails both the previous-regime check and its own-rule check, both `CHANGE_NOT_ON_BOUNDARY` errors are reported at `exceptions.history[i].from`, the previous-regime error first. Hints name the nearest valid dates where that helps, for example the first days of the previous regime's fiscal years just before and just after *c*(*i*). Validation never picks a date for the user.

Together these checks guarantee the tiling: each regime after regime 0 starts on one of its own year boundaries and has at least one fiscal year before the next change point, and each change point is a year boundary of the regime it ends, inside its table when it has one.

Year numbering (see [Year numbers must be unique](#year-numbers-must-be-unique)) runs only when the cross-block checks find no error.

### Example: a change from July to June years to April to March years

Procter & Gamble Hygiene and Health Care Limited changed its financial year from 1 July to 30 June to 1 April to 31 March, through a nine-month financial year "2024-25" from 1 July 2024 to 31 March 2025 ([`research/filers/pg-hygiene.md`](../research/filers/pg-hygiene.md)). This definition describes the change:

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 7, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start", "templates": {"year": "{yyyy}-{yy_end}"}}}},
  "exceptions": {
    "history": [
      {
        "from": "2025-04-01",
        "transition": {
          "start": "2024-07-01",
          "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3]}
        },
        "year": {"type": "fixed_date", "start": {"month": 4, "day": 1}},
        "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]}
      }
    ]
  }
}
```

The year boundaries before, during and after the change, the label "2024-25" of the transition year and the label "2025-26" of the year after it are stated by the company's annual reports. The transition's periods of one calendar month and its three quarters of three months, the periods and quarters of the other years, and the `week` block are illustrative: the reports print no period, quarter or week dates. The label "2023-24" for the year from 1 July 2023 to 30 June 2024 is inferred: the research record found it next to those dates, not in one sentence with them.

The definition passes the cross-block checks: the change point 2024-07-01 is the first day of a July to June year, `from` 2025-04-01 is the first day of an April to March year, `month_start_day` 1 is the day of both dates, and July 2024 to April 2025 is 9 months, which the transition's `quarters` sum to.

| Year number | Fiscal year | `start` | `last_day` | Days | Label |
| --- | --- | --- | --- | --- | --- |
| 2023 | Regime 0 (July to June) | 2023-07-01 | 2024-06-30 | 366 | 2023-24 |
| 2024 | Transition of entry 0 | 2024-07-01 | 2025-03-31 | 274 | 2024-25 |
| 2025 | Regime 1 (April to March) | 2025-04-01 | 2026-03-31 | 365 | 2025-26 |

Numbered by `"start"`, the three years are consecutive, so no number is skipped. The day counts are calendar arithmetic; for example `date(2025, 4, 1) - date(2024, 7, 1)` in Python is 274 days. The rest of this paragraph follows from the illustrative parts of the definition, not from the reports. Under the illustrative subdivision, periods 1 to 9 of the transition are the calendar months July 2024 to March 2025, and its quarters run 2024-07-01 to 2024-09-30, 2024-10-01 to 2024-12-31 and 2025-01-01 to 2025-03-31. Under the illustrative `week` block, with weeks starting on Monday, the transition has 40 weeks: 1 July 2024 is a Monday, so weeks 1 to 39 have 7 days, and week 40 is Monday 31 March 2025 alone.

## Supported range

The public API covers the fiscal years of the calendar (see [Timeline](#timeline)) that lie wholly between 0001-01-02 and 9999-12-30. A date in, or a request for, any other fiscal year returns `OUT_OF_RANGE`. This keeps every `end_exclusive` representable in Python, where `date(9999, 12, 31) + timedelta(days=1)` raises `OverflowError`.

## Canonical form and `calendar_id`

The canonical form is the definition after parsing, serialised as JSON with:

- `$schema` removed;
- optional fields with a default written out (`offset: 0`), and optional fields without a default written as `null` (`short_month`, `century_pivot`, and a history entry's `transition`); template levels that are absent stay absent;
- `exceptions` left out when it is absent or `null`, so every definition that fiscalclock 0.1.0 accepts keeps its 0.1.0 canonical form and `calendar_id`; when present, it is written with its `history` and the rules above apply inside it (for example, a history entry's `fixed_date` year has `short_month`, `null` when absent);
- object keys sorted by their UTF-8 bytes at every level; array elements, including the entries of `exceptions.history`, keep their order;
- dates in `exceptions` and in `boundaries` written as the `YYYY-MM-DD` strings given;
- no insignificant whitespace, integers in plain decimal, strings with the minimal JSON escapes: `\"`, `\\`, `\b`, `\f`, `\n`, `\r`, `\t`, other characters U+0000 to U+001F as `\u00xx` with lower-case hexadecimal digits, and every other character, including non-ASCII, written as UTF-8.

`calendar_id` is the first 16 lower-case hexadecimal characters of the SHA-256 digest of the canonical form's UTF-8 bytes. Two definitions that differ only in key order, whitespace, `$schema` or written-out defaults have the same `calendar_id`.

## Reserved for later releases

These parts of the format are designed in the [research proposal](https://github.com/Karthik-beta/fiscalclock/wiki/Definition-Format) but not implemented in 0.2.0. Each is rejected with `UNSUPPORTED_FEATURE` and a hint naming the planned release, or saying that none is scheduled, as in 0.1.0; [ADR-0031](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0031-themed-releases) moved them from 0.2 to 0.3, and ADRs 0035, 0036 and 0038 left four of them unscheduled because no cited calendar needs them. The wording of a hint is not part of the API. A reserved field or value inside a history entry or a transition is rejected in the same way, at the prefixed path, for example `exceptions.history[0].subdivision.stub`.

| Field or value | Planned |
| --- | --- |
| `subdivision.type: "anchored"` | 0.3, as decided in [ADR-0033](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0033-anchored-subdivision-and-rule-conflict) |
| `subdivision.stub` | 0.3, as decided in [ADR-0024](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0024-stub-semantics) |
| `subdivision.special_periods` | 0.3, as decided in [ADR-0034](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0034-special-periods) |
| `extra_week.standalone`, `extra_week.by_year` | Not scheduled ([ADR-0035](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0035-extra-week-standalone-and-by-year)) |
| `week.numbering: "first_full"` | Not scheduled ([ADR-0036](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0036-first-full-week-numbering-deferred)) |
| `naming.schemes.*.eras` | 0.3, as decided in [ADR-0037](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0037-eras-and-templates-ambiguous) |
| `exceptions.overrides` | Not scheduled ([ADR-0038](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0038-exceptions-overrides-deferred)) |

`exceptions.history`, with its transitions, is implemented in 0.2.0 and described in [`exceptions`](#exceptions). There is no separate `exceptions.transitions` field: a transition belongs to the history entry it leads into, and `exceptions.transitions` is `UNKNOWN_FIELD`.
