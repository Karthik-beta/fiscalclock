# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Rust crate `fiscalclock`: definition format and API version 1, with fixed-date and week-based years, calendar-month and week-pattern subdivisions, naming schemes, validation with stable error codes, and the operations `validate`, `locate`, `period`, `shift`, `parent`, `children`, `years`, `format`, `parse`, `dimension`, `dimension_csv` and `explain`.
- Python package `fiscalclock` (CPython 3.10 and later, and free-threaded CPython 3.14) and npm package `fiscalclock` (Node.js 22 and later, and browsers), both returning the Rust core's JSON byte for byte.
- Conformance vectors from independent generators and cited sources, shared by all three packages.
