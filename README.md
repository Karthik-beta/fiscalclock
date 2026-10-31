# fiscalclock

fiscalclock is an open-source fiscal calendar engine. You describe your organisation's fiscal calendar as a JSON definition, and it tells you which fiscal year, quarter, period and week any date falls in, the bounds of every period, and the labels your organisation uses for them.

**Status:** pre-release. The 0.1.0 release is in preparation and nothing has been published to any registry yet. The design is documented on the [wiki](https://github.com/Karthik-beta/fiscalclock/wiki).

## Why it exists

Real fiscal calendars vary in how the year is bounded (a fixed date, or a 52/53-week rule such as "the Saturday nearest 31 January"), how it is subdivided (calendar months, 4-4-5 week patterns, thirteen 4-week periods) and how it is named ("FY2026", "fiscal 2025", "FY 2025-26"). Existing tools each cover part of this inside one ecosystem; the [Analysis](https://github.com/Karthik-beta/fiscalclock/wiki/Analysis) page compares them. fiscalclock aims to be one open, data-defined engine whose answers are identical in Rust, Python and JavaScript.

## Planned packages

All packages are planned; none is published yet.

| Package | Registry | Planned for |
| --- | --- | --- |
| `fiscalclock` Rust crate | crates.io | 0.1.0 |
| `fiscalclock` Python package | PyPI | 0.1.0 |
| `fiscalclock` npm package (WebAssembly) | npm | 0.1.0 |
| Command-line interface and MCP server | npm and PyPI | 0.4.0 |

The [Roadmap](https://github.com/Karthik-beta/fiscalclock/wiki/Roadmap) lists what each release contains and the correctness gate it must pass.

## What a definition looks like

This definition describes the NRF 4-5-4 retail calendar: years end on the Saturday nearest 31 January, are named by the calendar year in which they start, and put the 53rd week in period 12. It is an example, not a built-in: fiscalclock has no named calendars.

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
    "default": "fiscal",
    "schemes": { "fiscal": { "year_number": "start", "templates": { "year": "FY{yyyy}", "quarter": "FY{yyyy} Q{q}" } } }
  }
}
```

The format is specified in [`spec/definition-v1.md`](spec/definition-v1.md).

## Why you can check the results

- **Shared conformance vectors.** One set of JSON test vectors in [`conformance/`](conformance/README.md) is loaded by the test suite of every package. Expected results come from cited sources or from generators written independently of the library, never from the library's own output.
- **External oracles.** Week-based years are checked against Python's ISO 8601 week dates (`date.isocalendar()`) for every year from 1600 to 2399, and the NRF calendar against NRF's published 53-week years.
- **Reproducible verification.** Every date-arithmetic claim in the design documents is reproduced by a standard-library script in [`research/verification/`](research/verification/README.md), which CI reruns on every pull request.

## Documentation

- [Analysis](https://github.com/Karthik-beta/fiscalclock/wiki/Analysis): how fiscal calendars work and what exists today
- [PDR](https://github.com/Karthik-beta/fiscalclock/wiki/PDR): product design requirements
- [ADR index](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-Index): design decisions and their reasons
- [Roadmap](https://github.com/Karthik-beta/fiscalclock/wiki/Roadmap): phases and release gates
- [Evidence](https://github.com/Karthik-beta/fiscalclock/wiki/Evidence): the source or check behind every factual claim

## Out of scope

Holidays, business days, pay periods and sprints are out of scope. fiscalclock exposes periods as plain date intervals that other tools can build on.

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md). Real-world calendars with a cited source are especially welcome as conformance vectors. Questions go to [Discussions](https://github.com/Karthik-beta/fiscalclock/discussions).

## Licence

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT licence](LICENSE-MIT), at your option.
