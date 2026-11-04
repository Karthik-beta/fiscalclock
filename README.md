<h1>
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/Karthik-beta/fiscalclock/main/assets/brand/readme/readme-banner-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/Karthik-beta/fiscalclock/main/assets/brand/readme/readme-banner-light.svg">
  <img alt="fiscalclock: fiscal calendar engine. Compute any fiscal calendar from a JSON definition." src="https://raw.githubusercontent.com/Karthik-beta/fiscalclock/main/assets/brand/readme/readme-banner.png" width="840">
</picture>
</h1>

[![CI](https://github.com/Karthik-beta/fiscalclock/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Karthik-beta/fiscalclock/actions/workflows/ci.yml)
[![docs](https://github.com/Karthik-beta/fiscalclock/actions/workflows/docs.yml/badge.svg?branch=main)](https://github.com/Karthik-beta/fiscalclock/actions/workflows/docs.yml)
[![crates.io](https://img.shields.io/crates/v/fiscalclock?label=crates.io)](https://crates.io/crates/fiscalclock)
[![docs.rs](https://img.shields.io/docsrs/fiscalclock?label=docs.rs)](https://docs.rs/fiscalclock)
[![PyPI](https://img.shields.io/pypi/v/fiscalclock?label=PyPI)](https://pypi.org/project/fiscalclock/)
[![npm](https://img.shields.io/npm/v/fiscalclock?label=npm)](https://www.npmjs.com/package/fiscalclock)
[![Licence: MIT OR Apache-2.0](https://img.shields.io/badge/licence-MIT%20OR%20Apache--2.0-blue)](#licence)

fiscalclock is an open-source fiscal calendar engine. You describe your organisation's fiscal calendar as a JSON definition, and it tells you which fiscal year, quarter, period and week any date falls in, the bounds of every period, and the labels your organisation uses for them.

**Status:** 0.1.0 is the first release. The definition format and API are version 1 drafts and may change before 1.0; every change is recorded in the [changelog](CHANGELOG.md). The design is documented on the [wiki](https://github.com/Karthik-beta/fiscalclock/wiki).

## Why it exists

Real fiscal calendars vary in how the year is bounded (a fixed date, or a 52/53-week rule such as "the Saturday nearest 31 January"), how it is subdivided (calendar months, 4-4-5 week patterns, thirteen 4-week periods) and how it is named ("FY2026", "fiscal 2025", "FY 2025-26"). The [Analysis](https://github.com/Karthik-beta/fiscalclock/wiki/Analysis) page compares the tools it surveyed as of 31 Oct 2026. Java, Perl, Power BI, SAP and Elixir tools handle parts of this, but it found no open, data-defined engine shared across Rust, Python and JavaScript. fiscalclock aims to be that engine, with identical answers in all three languages.

## Packages

All three packages are built from the same Rust core and return the same JSON for the same input.

| Package | Install | Supports | Guide |
| --- | --- | --- | --- |
| Rust crate | `cargo add fiscalclock` | Rust 1.85 and later, `no_std` with `alloc` | [`crates/core`](crates/core/README.md) |
| Python package | `pip install fiscalclock` | CPython 3.10 and later, and free-threaded 3.14; wheels for Linux, macOS and Windows | [`bindings/python`](bindings/python/README.md) |
| npm package | `npm install fiscalclock` | Node.js 22 and later, and browsers (WebAssembly) | [`bindings/js`](bindings/js/README.md) |

A command-line interface and an MCP server are planned for 0.4.0.

The [Roadmap](https://github.com/Karthik-beta/fiscalclock/wiki/Roadmap) lists what each release contains and the correctness gate it must pass.

## What a definition looks like

This definition has the year and week structure of the NRF 4-5-4 retail calendar: years end on the Saturday nearest 31 January and are named by the calendar year in which they start. The conformance vector [`nrf-53-week-years.json`](conformance/vectors/cited/nrf-53-week-years.json) checks that structure against the 53-week years NRF publishes. Putting the 53rd week in period 12, and the label templates, are illustrative choices. It is an example, not a built-in: fiscalclock has no named calendars.

```json
{
  "spec_version": 1,
  "year": { "type": "week_based", "anchor": { "edge": "end", "month": 1, "day": "last", "rule": "nearest" } },
  "week": { "starts_on": "sunday", "numbering": "contains_start" },
  "subdivision": {
    "type": "week_pattern",
    "periods": [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4],
    "quarters": [3, 3, 3, 3],
    "extra_week": { "period": 12 }
  },
  "naming": {
    "default": "fy",
    "schemes": { "fy": { "year_number": "start", "templates": { "year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}" } } }
  }
}
```

The format is specified in [`spec/definition-v1.md`](spec/definition-v1.md).

## Why you can check the results

- **Shared conformance vectors.** The vector files in [`conformance/`](conformance/README.md) are run by the test suite of every package. Expected results come from cited sources or from generators written independently of the library, never from the library's own output. CI regenerates the generated vectors and fails if they drift.
- **External oracles.** Week-based years are checked against Python's ISO 8601 week dates (`date.isocalendar()`) for every year from 1600 to 2399, and the NRF 4-5-4 definition against the 53-week years NRF publishes (2006, 2012, 2017 and 2023).
- **A second implementation.** A deliberately slow day-by-day reference implementation is compared with the core for every date from 1900 to 2200 across 960 generated definitions.
- **Property tests.** Periods tile their year without gaps or overlaps, nest correctly, and round-trip through `shift` and through `format` and `parse`; week-based years have 364 or 371 days and 71 long years in every 400.
- **Identical output in every language.** On every push to `main`, and on every pull request that changes the packages or the vectors, [CI](.github/workflows/ci.yml) runs all vectors through Rust and through the Python and npm packages on Linux, macOS and Windows, and requires the 14 raw outputs to match the Rust output byte for byte.
- **Reproducible verification.** Every date-arithmetic claim in the design documents is reproduced by a standard-library script in [`research/verification/`](research/verification/README.md), which CI reruns on every pull request.

## Supply chain

- Releases are built and published only by the [release workflow](.github/workflows/release.yml), from a version tag on `main`. Each publish step needs the maintainer's approval in a protected GitHub environment.
- Before publishing, the workflow installs the built wheel and npm tarball on Linux, macOS and Windows and runs a smoke test.
- As of 3 Nov 2026, every wheel, source distribution, npm tarball and crate of the [0.1.0 GitHub release](https://github.com/Karthik-beta/fiscalclock/releases/tag/v0.1.0) has a [build provenance attestation](https://github.com/Karthik-beta/fiscalclock/attestations), and the files on crates.io, PyPI and npm have the same SHA-256 digests as the release files. `gh attestation verify <file> --repo Karthik-beta/fiscalclock` checks a file downloaded from GitHub or from a registry.
- As of 3 Nov 2026, every [0.1.0 file on PyPI](https://pypi.org/project/fiscalclock/0.1.0/#files) also has PyPI provenance naming the release workflow, and the npm package has npm provenance (`npm view fiscalclock@0.1.0 dist.attestations`).
- The release workflow publishes to all three registries with trusted publishing: it authenticates with short-lived OIDC credentials, and the repository stores no registry tokens. Version 0.1.0 was published to crates.io and npm with API tokens, which have since been removed ([#35](https://github.com/Karthik-beta/fiscalclock/issues/35)).
- The core depends only on `serde`, `serde_json` and `sha2`, forbids `unsafe` code, and reads no clock, environment variable or global state.

Report vulnerabilities privately as described in [SECURITY.md](SECURITY.md).

## Documentation

- [Analysis](https://github.com/Karthik-beta/fiscalclock/wiki/Analysis): how fiscal calendars work and what exists today
- [PDR](https://github.com/Karthik-beta/fiscalclock/wiki/PDR): product design requirements
- [ADR index](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-Index): design decisions and their reasons
- [Roadmap](https://github.com/Karthik-beta/fiscalclock/wiki/Roadmap): phases and release gates
- [Evidence](https://github.com/Karthik-beta/fiscalclock/wiki/Evidence): the source or check behind every factual claim
- [Design system](https://github.com/Karthik-beta/fiscalclock/wiki/Design-System): the logo, colours, type and design tokens; the logo and icon files are in [`assets/brand/`](assets/brand/README.md)

## Out of scope

Holidays, business days, pay periods and sprints are out of scope. fiscalclock exposes periods as plain date intervals that other tools can build on.

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md). Real-world calendars with a cited source are especially welcome as conformance vectors. Questions go to [Discussions](https://github.com/Karthik-beta/fiscalclock/discussions).

## Licence

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT licence](LICENSE-MIT), at your option.

<!-- cspell:words srcset -->
