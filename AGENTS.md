# AGENTS.md

Instructions for any coding agent working in this repository. Human contributors follow the same rules; [CONTRIBUTING.md](CONTRIBUTING.md) covers process.

## Project summary

fiscalclock is an open-source fiscal calendar engine.
All logic lives in one Rust core (`crates/core`); thin bindings publish it to crates.io, PyPI and npm with identical results.
A calendar is a JSON definition supplied by the user; the library has no built-in calendars.
People use the results for financial reporting dates, so every behaviour must be backed by cited evidence and tests.
The source of truth for behaviour is the [PDR](https://github.com/Karthik-beta/fiscalclock/wiki/PDR), the [ADR index](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-Index) and the spec in [`spec/`](spec/).

## Repository map

| Path | What belongs here | What never belongs here |
| --- | --- | --- |
| `crates/core/` | The Rust core crate `fiscalclock`: definition parsing and validation, all calendar arithmetic, labels, the JSON API used by every binding. | Language-specific type conversion; I/O; named calendars. |
| `bindings/python/` | The PyO3 extension and pure-Python wrapper: conversion between Python types and the core's JSON API. | Calendar arithmetic or validation logic. |
| `bindings/js/` | The wasm-bindgen wrapper and TypeScript layer: conversion between JS types and the core's JSON API. | Calendar arithmetic or validation logic. |
| `cli/` | The command-line interface and MCP server (planned for v0.4). | Logic that is not also reachable through the core's JSON API. |
| `spec/` | The normative definition format and API contract. | Implementation notes. |
| `conformance/` | Shared vectors loaded by every test suite, their JSON Schema, and the independent generators. | Expected values produced by running fiscalclock. |
| `research/verification/` | Standard-library scripts that reproduce claims made in prose, and their committed output. | Library code. |
| `docs/` | User documentation and the cookbook of example definitions, including `docs/examples/`, the illustrative definitions the website shows and its code examples read. | Normative rules (those go in `spec/`). |
| `website/` | The documentation site at `https://fiscalclock.karthik.nz`: its pages, the code of its examples with their expected output, and its tests. | Calendar logic; example definitions (those go in `docs/`); hand edits to the files its scripts copy or generate. |
| `.github/` | Workflows, issue and discussion forms, templates. | Secrets. |

## Design rules

Each rule is phrased so that a reviewer can check it. A change that breaks one needs an accepted ADR first.

1. **Calendar as data, no presets.** No public function, constructor, constant, enum variant or module in any package is named after an organisation, country, standard or calendar family (for example `nrf()`, `iso_calendar()`, `Calendar::retail_454`). Example definitions live only in `docs/`, `conformance/` and the separate registry.
2. **Independent definition blocks.** `year`, `week`, `subdivision`, `naming` and `exceptions` are parsed and validated as separate blocks. No field in one block changes the meaning of a field in another, except where `spec/definition-v1.md` states the dependency (for example, a week-based year's boundary weekday comes from `week.starts_on`).
3. **No global state.** No `static mut`, `thread_local!`, lazy global cache, module-level mutable variable or environment-variable read exists in `crates/core` or in any binding. Every operation takes a calendar value.
4. **Civil dates only.** Every public function that accepts a date accepts only a civil date: the core's `Date`; in Python an object whose `type()` is exactly `datetime.date`, or an ISO `YYYY-MM-DD` string; in JS an ISO `YYYY-MM-DD` string or a `Temporal.PlainDate`. Python `datetime.datetime`, pandas `Timestamp` and JS `Date` are rejected with `TIMESTAMP_REJECTED`. A test for each binding proves the rejection.
5. **Half-open intervals.** Every period exposes `start` and `end_exclusive`; `last_day` is derived. No API returns an inclusive end without also returning `end_exclusive`.
6. **Deterministic, no clock.** No code in `crates/core` or the bindings calls a clock (`SystemTime`, `Instant`, `datetime.now`, `date.today`, `Date.now`, `new Date()` without arguments) or reads randomness. "Current" queries take the date as an argument.
7. **Errors carry stable codes, paths and hints.** Every error is an object with `code` from the catalogue in `spec/api-v1.md`, a JSON `path`, a `message`, and a `hint` where one helps. Adding a code means adding it to the catalogue in the same pull request. No code path panics, throws a bare exception, or returns an error without a code.
8. **Ambiguity is an error with candidates.** When an input has more than one reading, the operation returns `AMBIGUOUS_LABEL` (or the relevant ambiguity code) with every candidate in `details`. It never picks one.
9. **Identical JSON across languages.** Bindings call the core's JSON API (`fiscalclock::json`) and return its JSON unchanged on their `*_json` methods. No binding reimplements a computation, reorders keys or reformats values in that JSON. The cross-binding test compares the raw strings byte for byte.
10. **No silent defaults for organisation-specific choices.** Naming (`year_number`), extra-week placement (`extra_week`), equivalent-day mode and proration basis have no default value in the spec or in any binding's function signature. Omitting them is an error.

Before adding any dependency to `crates/core`, ask the maintainer in the issue or pull request and wait for agreement. The current core dependencies are `serde`, `serde_json` and `sha2`.

## Evidence rules

These apply to code comments, documentation, tests, commit messages and issue text.

1. Every factual claim (a number, date, version, library capability, company calendar rule or standard) has a source link or a reproducible check. Remove or rewrite as an open question any claim with neither. Do not state facts from memory.
2. Write time-sensitive facts with "as of <date>" and a link.
3. Quotations are verbatim and have been checked against the source. If a quote cannot be confirmed, paraphrase and say so.
4. Mark illustrative or unverified material (for example, example definitions not yet checked against a filing) as such wherever it appears.
5. No marketing language ("blazing", "revolutionary", "best-in-class", "the only"). Comparisons are narrow and defensible.
6. Every link must resolve.

## Correctness workflow

1. Every behaviour change comes with conformance vectors in `conformance/vectors/` whose expected results come from a cited source or an independent generator. Never generate expected values by running fiscalclock.
2. Never change an expected result in an existing vector without a linked issue that explains why.
3. Property tests cover tiling, nesting, round trips and, for rule-only week-based definitions, the 364/371-day and 71-in-400 properties. Do not apply the 364/371 or 400-year properties to definitions with exceptions or table years.
4. The reference implementation in `crates/core/tests/` stays slow and obvious (day-by-day iteration). Do not optimise it or share code with the core's boundary arithmetic.
5. Run the full check list below before declaring work done, and report the commands and their results.

## When an ADR is required

Write an ADR before changing the definition format, the result of any operation for a valid input, the public API or error codes, core dependencies, supported versions, scope or licence. Use the MADR layout of the existing ADRs on the wiki (context, decision drivers, considered options with honest pros and cons, decision, consequences, status, links), give it the next number and the status **Proposed**, and link a Design decisions discussion. Do not implement a Proposed decision as if it were accepted.

## Build, test and lint

These are the commands CI runs (`.github/workflows/ci.yml` and `docs.yml`). Run the ones that cover your change, from the repository root, and report them with their results.

Rust core:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --release -p fiscalclock --locked
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p fiscalclock
cargo build -p fiscalclock --target wasm32-unknown-unknown --release
cargo package -p fiscalclock --locked
cargo +1.85 check -p fiscalclock
```

Python package (needs maturin 1.15.0):

```sh
maturin build --release --locked --out wheels --manifest-path bindings/python/Cargo.toml
python -m pip install --no-index --find-links wheels fiscalclock
python -m unittest discover -s bindings/python/tests
```

npm package (needs `wasm-bindgen-cli` at the version pinned in `bindings/js/Cargo.toml`):

```sh
cd bindings/js && npm ci && npm run build && npm test
```

Each conformance run writes `target/transcripts/<language>.txt`. After running all three, `cmp target/transcripts/rust.txt target/transcripts/python.txt` and the same for `js.txt` must report no difference.

Evidence, vectors and versions:

```sh
python -I research/verification/verify_claims.py | diff -u research/verification/expected_output.txt -
python -I conformance/generators/check.py
uv run --no-project --with pandas==3.0.6 python -I conformance/generators/pandas_vectors.py --check
python -I research/evidence/check_quotes.py --offline
python -I research/evidence/render_evidence.py --check
python -I scripts/check_versions.py
```

`check_quotes.py` without `--offline` fetches every quoted source. CI runs it weekly, so run it when you change a quote.

Website (Node.js 24; `test:examples` also needs Python 3 and Cargo):

```sh
(cd website && npm ci --ignore-scripts && npm run build && npm run check && npm test && npm run test:examples)
lychee --config website/lychee.toml --no-progress --root-dir "$PWD/website/dist" "website/dist/**/*.html"
```

`npm test` runs Playwright, which needs its browser once: `cd website && npx playwright install --with-deps chromium`.

Documentation: `markdownlint-cli2 "**/*.md"`, `cspell --config cspell.json "**/*.md" "**/*.mdx" "**/*.yml" "**/*.yaml" "**/*.toml"`, `lychee --config lychee.toml "./**/*.md"`, `lychee --config lychee.toml --base-url https://fiscalclock.karthik.nz/ "./**/*.mdx"` and `yamllint --strict .`. markdownlint does not lint `.mdx`: it reads a fenced block indented inside a JSX component as an indented code block.

CI also checks that the issue forms, issue template chooser, workflows, discussion forms and Dependabot config match GitHub's schemas:

```sh
uvx check-jsonschema==0.39.0 --builtin-schema vendor.github-issue-forms $(find .github/ISSUE_TEMPLATE -name '*.yml' ! -name config.yml)
uvx check-jsonschema==0.39.0 --builtin-schema vendor.github-issue-config .github/ISSUE_TEMPLATE/config.yml
uvx check-jsonschema==0.39.0 --builtin-schema vendor.github-workflows .github/workflows/*.yml
uvx check-jsonschema==0.39.0 --builtin-schema vendor.github-discussion .github/DISCUSSION_TEMPLATE/*.yml
uvx check-jsonschema==0.39.0 --builtin-schema vendor.dependabot .github/dependabot.yml
```

## Commits and pull requests

- Conventional Commits: `feat:`, `fix:`, `docs:`, `test:`, `ci:`, `chore:`, `refactor:`, `perf:`; `!` and a `BREAKING CHANGE:` footer for breaking changes.
- One logical change per commit and per pull request; pull requests are squash-merged, so the title must be a valid Conventional Commit.
- Never add AI or model attribution to commits, pull requests, issues or files: no `Co-Authored-By` lines for tools, no "Generated with …" lines, no model names.
- Never force-push to `main`, delete branches you did not create, or rewrite published history.
- Fill in every item of the pull request template.

## Out of scope

Holidays, business days, working hours, pay periods and sprints are out of scope. Decline requests to add them; expose periods as plain intervals instead.
