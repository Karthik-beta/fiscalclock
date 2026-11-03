# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Cited conformance vectors for Indian listed companies (Reliance Industries, Procter & Gamble Hygiene and Health Care, Siemens Limited and ABB India), each quoting the company's own annual report, and filer records that also cover the transition periods of P&G Hygiene, Siemens Limited, Linde India and Thomas Cook (India).

### Changed

- Python: `Period.level` is typed as the `Level` literal instead of `str`, and the TypeScript `details` of an error as its documented shapes instead of `unknown`. Only type checkers see the difference.

### Fixed

- Python and JavaScript raised a bare `TypeError`, `ValueError` or `UnicodeEncodeError`, instead of `FiscalclockError`, for an argument or definition with no JSON form: for example a numpy integer, `Decimal`, `bytes` or `BigInt`, a period given as a tuple or `null`, a period mapping that also holds dates, a cyclic definition, or a lone surrogate in a mapping or argument. They now raise `FiscalclockError` with the core's code and path, the same in both languages. A period mapping that also holds other values, such as `dataclasses.asdict(period)`, now works.
- Python: a `FiscalclockError` can be pickled and copied, so it can cross a process pool.
- Python: a definition given as any `Mapping`, such as a `MappingProxyType`, is accepted like a `dict`. Before, a non-`dict` mapping failed with `MISSING_FIELD` and `UNKNOWN_FIELD` errors.

## [0.1.0] - 2026-10-31

### Added

- Rust crate `fiscalclock`: definition format and API version 1, with fixed-date and week-based years, calendar-month and week-pattern subdivisions, naming schemes, validation with stable error codes, and the operations `validate`, `locate`, `period`, `shift`, `parent`, `children`, `years`, `format`, `parse`, `dimension`, `dimension_csv` and `explain`.
- Python package `fiscalclock` (CPython 3.10 and later, and free-threaded CPython 3.14) and npm package `fiscalclock` (Node.js 22 and later, and browsers), both returning the Rust core's JSON byte for byte.
- Conformance vectors from independent generators and cited sources, shared by all three packages.

[Unreleased]: https://github.com/Karthik-beta/fiscalclock/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Karthik-beta/fiscalclock/releases/tag/v0.1.0
