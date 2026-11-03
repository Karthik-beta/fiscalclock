# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- A documentation site at <https://fiscalclock.karthik.nz>, with guides, a date lookup on the first screen, a playground, and code examples in Rust, Python and JavaScript that CI runs against their expected output ([#40](https://github.com/Karthik-beta/fiscalclock/pull/40), [#41](https://github.com/Karthik-beta/fiscalclock/pull/41)).
- The fiscalclock visual identity: logo, icons, README banners and design tokens, in [`assets/brand/`](assets/brand/README.md) and on the wiki's [Design system](https://github.com/Karthik-beta/fiscalclock/wiki/Design-System) page ([#39](https://github.com/Karthik-beta/fiscalclock/pull/39)).
- A machine-readable evidence register, [`research/evidence/`](research/evidence/README.md), that records the source or check behind every factual claim and checks quotes against their sources.
- Cited conformance vectors from the Kroger and Costco annual reports, and NRF's published 53-week years checked against the NRF 4-5-4 definition.
- Week-based year vectors from the `FY5253` offset of pandas 3.0.6, a second source that shares no code with the project's oracle.
- Cited conformance vectors for Indian listed companies (Reliance Industries, Procter & Gamble Hygiene and Health Care, Siemens Limited and ABB India), each quoting the company's own annual report, and filer records that also cover the transition periods of P&G Hygiene, Siemens Limited, Linde India and Thomas Cook (India) ([#42](https://github.com/Karthik-beta/fiscalclock/pull/42)).

### Changed

- The release workflow publishes to crates.io and npm with trusted publishing, as it already did to PyPI. The API tokens used to publish 0.1.0 have been removed ([#35](https://github.com/Karthik-beta/fiscalclock/issues/35)).
- The package READMEs list every member with its arguments and show how to read `FiscalclockError`, and the API spec documents `details.years`, more error paths and the type of each dimension column.
- Python: `Period.level` is typed as the `Level` literal instead of `str`, and the TypeScript `details` of an error as its documented shapes instead of `unknown`. Only type checkers see the difference.
- Python: `validate`, `validate_json`, `call_json` and the `FiscalCalendar` constructor release the GIL while the core runs, as `FiscalCalendar.call_json` and the methods already did.

### Fixed

- `shift` of a week in a `fixed_date` calendar by `-9223372036854775808` panicked in debug builds of the Rust crate, and in release builds stepped through every earlier fiscal year before failing. It now returns `OUT_OF_RANGE` at `n` at once.
- Python and JavaScript raised a bare `TypeError`, `ValueError` or `UnicodeEncodeError`, instead of `FiscalclockError`, for an argument or definition with no JSON form: for example a numpy integer, `Decimal`, `bytes` or `BigInt`, a period given as a tuple or `null`, a period mapping that also holds dates, a cyclic definition, or a lone surrogate in a mapping or argument. They now raise `FiscalclockError` with the core's code and path, the same in both languages. A period mapping that also holds other values, such as `dataclasses.asdict(period)`, now works.
- Python: a `FiscalclockError` can be pickled and copied, so it can cross a process pool.
- Python: a definition given as any `Mapping`, such as a `MappingProxyType`, is accepted like a `dict`. Before, a non-`dict` mapping failed with `MISSING_FIELD` and `UNKNOWN_FIELD` errors.

### Fixed

- Website: when fiscalclock cannot load in the browser, the date lookup and the playground say so, instead of keeping an old answer or showing "Loading fiscalclock…" forever.

## [0.1.0] - 2026-10-31

### Added

- Rust crate `fiscalclock`: definition format and API version 1, with fixed-date and week-based years, calendar-month and week-pattern subdivisions, naming schemes, validation with stable error codes, and the operations `validate`, `locate`, `period`, `shift`, `parent`, `children`, `years`, `format`, `parse`, `dimension`, `dimension_csv` and `explain`.
- Python package `fiscalclock` (CPython 3.10 and later, and free-threaded CPython 3.14) and npm package `fiscalclock` (Node.js 22 and later, and browsers), both returning the Rust core's JSON byte for byte.
- Conformance vectors from independent generators and cited sources, shared by all three packages.

[Unreleased]: https://github.com/Karthik-beta/fiscalclock/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Karthik-beta/fiscalclock/releases/tag/v0.1.0
