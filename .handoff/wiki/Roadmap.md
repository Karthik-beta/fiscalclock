# Roadmap

fiscalclock is built in six phases, and each of the first five ends at a gate: a list of checks that must all pass before the next release is published. The gates measure correctness evidence, not feature counts. No dates are set.

Last reviewed: 31 Oct 2026

**Status of this page: Proposed.** The research proposal described six phases and five gates but did not include their content. The maintainer asked for this roadmap to be drafted so that 0.1.0 could be built and published, and it will be revised after 0.1.0 ships. Changes to a gate need an ADR (see [ADR-0001](ADR-0001-record-architecture-decisions)).

Each phase has a [milestone](https://github.com/Karthik-beta/fiscalclock/milestones) with the same title, and every issue is tracked on the [fiscalclock Roadmap project](https://github.com/users/Karthik-beta/projects). Terms follow the [Glossary](Glossary).

## Summary

| Phase | Release | Theme | Closing gate |
| --- | --- | --- | --- |
| Phase 0: Foundations | none | Repository, evidence, spec, conformance format | Gate A |
| Phase 1: v0.1 | 0.1.0 | Fixed-date and week-based years, labels, date dimension, explain; Rust, Python and npm packages | Gate B |
| Phase 2: v0.2 | 0.2.0 | Exceptions and table years, anchored periods, range operations, proration, bulk paths | Gate C |
| Phase 3: v0.3 | 0.3.0 | Comparable periods, equivalent day, cross-calendar mapping | Gate D |
| Phase 4: v0.4 | 0.4.0 | Agent tooling: CLI, MCP server, `describe`, `infer`, published schema, registry | Gate E |
| Phase 5: v1.0 | 1.0.0 | Stability: format freeze, second implementation in CI, complete documentation | none |

## Phase 0: Foundations

Scope:

- Repository scaffolding, community files, issue and discussion forms, CI for documentation.
- `research/verification/`: a standard-library script that reproduces every date-arithmetic claim in the research documents.
- Spec version 1 for the parts implemented in 0.1.0 ([`spec/definition-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/definition-v1.md), [`spec/api-v1.md`](https://github.com/Karthik-beta/fiscalclock/blob/main/spec/api-v1.md)).
- The conformance vector format and independent vector generators.
- The wiki: Analysis, Evidence, PDR, ADRs.

**Gate A** (all must pass):

1. CI reruns `research/verification/verify_claims.py` and its output matches the committed output.
2. Spec version 1 and API version 1 are merged on `main`.
3. The conformance vector format is merged, with independently generated vectors for week-based years, NRF 4-5-4, ISO 8601 week years and fixed-date years, and one error vector for every validation code in the catalogue.
4. Every ADR the 0.1.0 scope depends on has the status Accepted.
5. Documentation CI (Markdown lint, link check, British-English spelling, YAML and issue-form schema validation) passes on `main`.

## Phase 1: v0.1

Scope (the 0.1.0 release):

- Year types `fixed_date` (with `short_month`) and `week_based` (with `edge` and `rule`).
- Subdivisions `calendar_months` (inside fixed-date years) and `week_pattern` with `extra_week.period` (inside week-based years).
- The `week` block with `contains_start` numbering.
- Naming schemes with `year_number` `start`, `end` or `majority`, `offset`, `century_pivot` and templates; several schemes per calendar.
- Whole-range validation with stable error codes and paths; canonical form and `calendar_id`.
- Operations: `validate`, `locate`, `period`, `shift`, `parent`, `children`, `years`, `format`, `parse`, `dimension`, `dimension_csv`, `explain`.
- Packages: the Rust crate, the Python package (abi3 wheels plus free-threaded CPython 3.14 wheels) and the npm package (WebAssembly), all at version 0.1.0.

Out of 0.1.0 scope, and rejected with `UNSUPPORTED_FEATURE` so that no definition is silently misread: `table` years, `anchored` subdivision, `stub`, special periods, `extra_week.standalone` and `by_year`, `first_full` week numbering, eras, and `exceptions`.

**Gate B** (all must pass before 0.1.0 is published):

1. Every conformance vector passes in Rust, Python and JavaScript, and the three packages return byte-identical JSON for every case.
2. The differential test against the slow reference implementation passes for every date from 1900-01-01 to 2200-12-31 across at least 200 generated definitions.
3. Property tests pass: tiling, nesting, `shift(n).shift(-n)` round trips, `parse(format(p))` round trips, and for rule-only week-based definitions the 364/371-day and 71-in-400 properties.
4. ISO 8601 oracle: week-based years starting on the Monday nearest 1 January match `date.isocalendar()` for every year from 1600 to 2399.
5. Each binding rejects timestamp inputs (Python `datetime.datetime`, JavaScript `Date`) with `TIMESTAMP_REJECTED`, proven by tests.
6. Packages are built and published by the release workflow from a tagged commit, with build provenance attestations, and the published artefacts install and pass a smoke test on Linux, macOS and Windows.

## Phase 2: v0.2

Scope:

- `exceptions` (`overrides`, `history`, `transitions`) and `table` years, subject to [ADR-0021](ADR-0021-exceptions-and-table-years-release) and [ADR-0022](ADR-0022-table-years-with-period-boundaries).
- `anchored` subdivision (broadcast calendar), `stub` semantics ([ADR-0024](ADR-0024-stub-semantics)), special periods, `extra_week.standalone` and `by_year`, `first_full` numbering, eras, `TEMPLATES_AMBIGUOUS` and `RULE_CONFLICT` validation.
- Range operations: periods overlapping a range, period-to-date ranges, trailing N periods.
- Proration: `split` with a required basis and `allocate` by largest remainder.
- Bulk paths: the Arrow PyCapsule Interface in Python and typed arrays in JavaScript; optional `chrono`, `jiff` and `time` conversions in Rust.
- Benchmarks per binding for single-date latency and million-date throughput.
- The JSON Schema for definitions generated from the Rust types ([ADR-0009](ADR-0009-json-schema-from-rust-types)), with every conformance vector's definition validated against it.

**Gate C**:

1. Cited conformance vectors for Vishay, Costco, Kroger and Care.com pass, each quoting its filing; the Care.com vector expects `RULE_CONFLICT` with the affected years.
2. Proration properties pass in every binding: shares sum to exactly 1, allocated amounts sum to the input.
3. The definition parser and label parser have been fuzzed for at least 24 CPU hours in total with no panic, hang or unbounded allocation.
4. All Gate B checks still pass.

## Phase 3: v0.3

Scope: `comparable` (fiscal, truncated and restated modes with a stated target year), `equivalent_day` (`same_position` and `same_weekday`, no default; `same_date` for fixed-date years), and cross-calendar `map`.

**Gate D**:

1. Restated-view vectors reproduce NRF's published restatement for every 53-week year from 2009 to 2028 that NRF documents.
2. Equivalent-day vectors cover both modes, including `NO_COUNTERPART` for week 53 under `same_position`.
3. Mapping vectors cover calendars with different year types and confirm that overlap days sum to the period's length.
4. All earlier gate checks still pass.

## Phase 4: v0.4

Scope: the CLI and the MCP server (shipped through the npm and PyPI packages), `describe`, `infer`, the JSON Schema published at a versioned URL, `llms.txt`, the definition registry in a separate repository ([ADR-0027](ADR-0027-registry-location-and-reviewers)), the `fiscalyear` compatibility module and a pandas accessor.

**Gate E**:

1. The MCP server passes the conformance checks of the MCP specification revision current at release, and every tool's output validates against its output schema.
2. `infer` ranks the correct definition first for every registry entry, given that entry's reported period end dates.
3. Every conformance vector's definition validates against the published JSON Schema.
4. All earlier gate checks still pass.

## Phase 5: v1.0

Scope: freeze spec version 1 and API version 1; run a second implementation (ThreeTen-Extra's AccountingChronology) as a differential test in CI; complete the user documentation and cookbook; security review of the parsers.

There is no gate after Phase 5; 1.0.0 is published when every earlier gate check passes and the maintainers agree, through an ADR, that the format can be frozen.
