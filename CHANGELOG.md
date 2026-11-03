# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Cited conformance vectors for Indian listed companies (Reliance Industries, Procter & Gamble Hygiene and Health Care, Siemens Limited and ABB India), each quoting the company's own annual report, and filer records that also cover the transition periods of P&G Hygiene, Siemens Limited, Linde India and Thomas Cook (India).

### Changed

- Python: `validate`, `validate_json`, `call_json` and the `FiscalCalendar` constructor release the GIL while the core runs, as `FiscalCalendar.call_json` and the methods already did.

## [0.1.0] - 2026-10-31

### Added

- Rust crate `fiscalclock`: definition format and API version 1, with fixed-date and week-based years, calendar-month and week-pattern subdivisions, naming schemes, validation with stable error codes, and the operations `validate`, `locate`, `period`, `shift`, `parent`, `children`, `years`, `format`, `parse`, `dimension`, `dimension_csv` and `explain`.
- Python package `fiscalclock` (CPython 3.10 and later, and free-threaded CPython 3.14) and npm package `fiscalclock` (Node.js 22 and later, and browsers), both returning the Rust core's JSON byte for byte.
- Conformance vectors from independent generators and cited sources, shared by all three packages.

[Unreleased]: https://github.com/Karthik-beta/fiscalclock/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Karthik-beta/fiscalclock/releases/tag/v0.1.0
