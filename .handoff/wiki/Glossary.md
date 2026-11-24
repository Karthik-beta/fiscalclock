# Glossary

This page defines the terms used across the fiscalclock wiki, README, spec and code; every other page uses these words with these meanings.

Last reviewed: 31 Oct 2026

## Calendar concepts

| Term | Meaning | Do not write |
| --- | --- | --- |
| fiscal year | One year of an organisation's calendar, bounded by its `year` block. | "financial year" (except when quoting), "FY" as a noun in prose |
| fixed-date year | A fiscal year that starts on the same month and day every calendar year (`year.type: "fixed_date"`). | "calendar-month year" when meaning the year type |
| week-based year | A fiscal year made of whole weeks whose boundary is set by an anchor rule (`year.type: "week_based"`); it has 364 or 371 days. | "52/53-week year" as a type name (fine as a description), "retail year" |
| table year | A fiscal year whose boundaries are listed explicitly (`year.type: "table"`); reserved for 0.2. | "irregular year" |
| anchor | The rule that fixes a week-based year's boundary: `edge`, `month`, `day` and `rule`. | "end rule" |
| `last` rule, `first` rule, `nearest` rule | The qualifying weekday on or before, on or after, or nearest to the target date. | "closest", "latest" |
| extra week | The 53rd week of a 371-day week-based year. Its placement is set by `extra_week`. | "leap week", "long week", "week 53" when the extra week is not numbered 53 |
| `has_extra_week` | The output flag on a year, quarter or period that contains the extra week, and on the week that is the extra week. | `is_long`, `is_53_week` |
| period | The subdivision level below a quarter: a fiscal month in a calendar-month subdivision, or a group of weeks in a week pattern. | "fiscal month" as the level name (fine in prose for calendar months) |
| quarter | A group of consecutive periods given by `subdivision.quarters`. Quarters need not be equal. | "13-week quarter" as a definition |
| week | A 7-day block starting on `week.starts_on`; partial at the edges of a fixed-date year. | |
| level | One of `year`, `quarter`, `period`, `week`. | "granularity" |
| coordinates | `(level, year, index)` identifying a period. `year` is the default scheme's year number. | |
| week pattern | The weeks-per-period list in `subdivision.periods`, such as 4-5-4. | "445 calendar" as a type name |
| 4-4-5 family | Week-based years whose quarters split 4-4-5, 4-5-4 or 5-4-4 weeks. Descriptive only; not an API name. | |
| `short_month` | The policy (`clamp` or `next`) for a fixed-date start day that does not exist in a month. | |
| stub | Partial weeks absorbed by the first and last periods of a fixed-date year with week-based periods; reserved, see ADR-0024. | |
| special period | A zero-length ERP adjustment period such as SAP periods 13–16; reserved for 0.2. | "adjustment period" as the field name |
| transition year | A short or long year when an organisation changes its year end; reserved for 0.2. | |

## Naming

| Term | Meaning | Do not write |
| --- | --- | --- |
| naming scheme | A named set of `year_number`, `offset`, `century_pivot` and templates. A calendar has one or more. | "label format" |
| default scheme | The scheme named by `naming.default`; it sets coordinates and is used by `format` when no scheme is given. | |
| year number | The number a scheme gives a fiscal year: by `start` year, `end` year or `majority` year, plus `offset`. | "fiscal year label" |
| template | A label pattern such as `FY{yyyy} Q{q}`. | "format string" |
| label | A string produced by a template, such as "FY2026 Q3". | |
| century pivot | The year that resolves two-digit years in parsing. | "reference year" |

## Engine and API

| Term | Meaning | Do not write |
| --- | --- | --- |
| definition | The JSON object describing a calendar (spec version 1). | "config", "preset" |
| block | One of the five top-level parts of a definition: `year`, `week`, `subdivision`, `naming`, `exceptions`. | "axis" in API text (fine in analysis prose) |
| canonical form | The definition serialised with sorted keys, defaults written out and no whitespace. | |
| `calendar_id` | The first 16 hex characters of the SHA-256 of the canonical form. | "calendar hash" |
| civil date | A date with no time or zone, proleptic Gregorian. | "naive date" |
| half-open interval | `[start, end_exclusive)`; `last_day` is derived. | "inclusive end" without `end_exclusive` |
| supported range | Fiscal years lying wholly between 0001-01-02 and 9999-12-30. | |
| error code | The stable `code` of an error, such as `WEEKS_SUM_MISMATCH`. | "error type" |
| core | The Rust crate `fiscalclock` in `crates/core`, where all logic lives. | "engine library" |
| binding | A thin package (Python, npm) that converts native types and calls the core's JSON API. | "port", "wrapper library" |
| conformance vector | A JSON file of definition, inputs and expected outputs shared by all test suites. | "fixture" |
| oracle | An independent source of expected results: a standard library, a published calendar, a filing or another implementation. | |
| reference implementation | The deliberately slow day-by-day implementation used in differential tests. | |
| registry | The separate, versioned repository of cited real-world definitions. Not part of any package API. | "presets" |

## Project process

| Term | Meaning |
| --- | --- |
| phase | One of Phase 0 to Phase 5 on the [Roadmap](Roadmap). Names: "Phase 0: Foundations", "Phase 1: v0.1", "Phase 2: v0.2", "Phase 3: v0.3", "Phase 4: v0.4", "Phase 5: v1.0". |
| gate | The checks that close a phase: Gate A (Phase 0) to Gate E (Phase 4). |
| ADR | Architecture Decision Record, in MADR format. Status is Proposed, Accepted, Rejected or Superseded. |
| Decided | Settled by the research documents. |
| Proposed | An open decision awaiting discussion. |
| illustrative | An example not checked against its source; never used as a conformance vector until checked. |
