# Contributing to fiscalclock

Thank you for helping. fiscalclock computes dates that people use in financial reports, so every change has to show its evidence. This guide explains how to propose a change, when a design record is needed, and how to contribute a real-world calendar.

## Before you start

- Questions go to [Discussions](https://github.com/Karthik-beta/fiscalclock/discussions), not issues. See [SUPPORT.md](SUPPORT.md).
- Read the [design requirements](https://github.com/Karthik-beta/fiscalclock/wiki/PDR) and the [decision records](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-Index). They are the source of truth for behaviour.
- Coding agents follow [AGENTS.md](AGENTS.md). Its rules apply to human contributors too.

## How to propose a change

1. Search the open and closed [issues](https://github.com/Karthik-beta/fiscalclock/issues) for duplicates.
2. Open an issue with the matching form. Describe the problem before the solution.
3. If the change needs a decision record (see below), say so in the issue. A maintainer adds `status: needs-adr`.
4. Wait for `status: ready` before starting large work, so effort is not wasted on a direction that will be declined.
5. Open a pull request that links the issue and completes the template checklist.

## When a decision record (ADR) is required

Write an ADR when a change:

- adds, removes or changes the meaning of a field in the calendar definition format;
- changes the result of any operation for an input that was previously valid;
- adds a public operation, error code or output field;
- adds a dependency to the Rust core, or changes the toolchain or supported versions of a published package;
- changes the project's scope, licence or governance.

To write one, copy the format of an existing ADR on the wiki (MADR: context, decision drivers, considered options with honest pros and cons, decision, consequences, status, links). Give it the next free number and the status **Proposed**, open a thread in the [Design decisions](https://github.com/Karthik-beta/fiscalclock/discussions) category, and link the two. [GOVERNANCE.md](GOVERNANCE.md) explains how a Proposed ADR becomes Accepted.

## Contributing a real-world calendar as a conformance vector

A real company's or government's calendar is the most valuable contribution. It becomes a permanent regression test. Each submission needs four things.

1. **The definition.** A calendar definition in the [spec version 1 format](spec/definition-v1.md).
2. **A verbatim citation with a link.** The exact sentence from a primary source that states the calendar rule: an annual or quarterly filing, a statute, or an official publication. Copy it character for character; do not paraphrase. Link to the document itself, not to a summary of it.
3. **Expected results.** Period start and end dates that the source reports, for example the "period ended" dates on several quarterly filings. Prefer dates the source states over dates you computed.
4. **How you checked them.** Say which documents you compared and on what date. If you computed any expected value, say how.

Put the vector in `conformance/vectors/cited/` using the [vector format](conformance/README.md), or open a [Wrong result for a real calendar](https://github.com/Karthik-beta/fiscalclock/issues/new?template=wrong_result.yml) issue if you are not ready to write JSON. Use the filing's own period dates, not SEC EDGAR "frames", which align facts to calendar periods and can mix different reporting dates in one frame.

## Evidence rules

These apply to code, documentation, tests and issue text.

1. Every factual claim has a source link or a reproducible check. A claim with neither is removed or rewritten as an open question.
2. Time-sensitive facts (versions, release status, download counts) are written with "as of <date>" and a link.
3. Quotations are verbatim and checked against the source. If you cannot confirm a quote, paraphrase it and say so.
4. Keep facts, decisions and proposals apart. Mark illustrative or unverified material as such wherever it appears.
5. No marketing language. Comparisons are narrow and defensible: "no open, data-defined engine shared across Rust, Python and JavaScript", never "no library does this".
6. Every link must resolve. CI checks links in Markdown files.

## Correctness rules

- Every behaviour change lands with conformance vectors whose expected results come from a cited source or an independent generator, never from fiscalclock's own output.
- Never change an expected result in a vector without a linked issue that explains why.
- The slow reference implementation stays slow and obvious. Optimise the core, not the reference.

## Commits and pull requests

- Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/): `feat:`, `fix:`, `docs:`, `test:`, `ci:`, `chore:`, `refactor:`, `perf:`. Mark breaking changes with `!` and a `BREAKING CHANGE:` footer.
- Keep each commit and pull request to one logical change.
- Do not add AI or model attribution lines, such as "Co-Authored-By" for a tool or "Generated with …", to commits, pull requests, issues or files.
- Pull requests are squash-merged. The pull request title becomes the commit message, so write it as a Conventional Commit.
- Update `CHANGELOG.md` under "Unreleased" for any user-visible change.

## Out of scope

fiscalclock answers fiscal-calendar questions only. These are out of scope and will be declined:

- holidays and holiday calendars;
- business days and working hours;
- pay periods and payroll calendars;
- sprints and other project-management cycles.

Periods are exposed as plain date intervals so that other libraries can build these on top.

## Licence

By contributing, you agree that your contribution is dual-licensed under the [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE) licences, as described in the Apache-2.0 licence, section 5.
