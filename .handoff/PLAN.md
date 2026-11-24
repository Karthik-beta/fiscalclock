# fiscalclock set-up plan (for approval)

Prepared 31 Oct 2026. Nothing in the repository, wiki, settings, labels, project, issues or discussions has been changed yet.

## 0. What I found while looking

| Item | State today |
| --- | --- |
| `gh` auth | Logged in as `Karthik-beta`; scopes `gist, project, read:org, repo, user, workflow`. `project` already covers `read:project`, so no refresh is needed. |
| Repository | `Karthik-beta/fiscalclock`, public, **empty** (no commits, no default branch yet). Issues, wiki, projects on; discussions off; merge, squash and rebase all allowed; head branches not auto-deleted; no description or topics. |
| Security | Private vulnerability reporting is **already on**. Dependabot alerts are **off** (`GET /vulnerability-alerts` returns 404). |
| Labels | The 10 GitHub defaults. No issues or PRs exist, so none is in use. |
| Wiki | One page, `Home.md`, containing only "Welcome to the fiscalclock wiki!". I will fold it into the new Home page (the sentence becomes the page's opening line). |
| Projects | Five user projects exist (`workclock roadmap`, `Aria ERP`, …). None for fiscalclock. |
| Sibling repo | `Karthik-beta/workclock` has milestones M0 Scaffold … M8 Release and `type:`/`area:` labels. The proposal says this M0–M8 structure "can be reused". |
| Package names | `fiscalclock` returns 404 (not taken) on crates.io, PyPI and npm as of 31 Oct 2026. |

## 1. Blockers and contradictions (need your answer before I execute)

### B1. The roadmap and architecture figures are missing from the source document

The proposal's Roadmap section contains only `[embedded content: roadmap · 6 phases, 5 gates]`, and the bindings section contains only `[embedded content: architecture · one core, five distributions]`. The phase scopes and the five gate criteria are therefore **not in the text I was given**. You asked for milestone descriptions "copied exactly from the roadmap", so I cannot write them, the Roadmap wiki page, the Gate field mapping or the Phase 0/1 issue split without that content.

Options:

1. **(Recommended)** You paste the roadmap figure's text (phase names, scope per phase, gate criteria) and, if possible, the architecture figure's text.
2. I reconstruct phases from the feature table's release tags (v0.1, v0.2, v0.3) and label every phase scope and gate as **Proposed, reconstructed**. v0.4 and v1.0 have no feature rows in the proposal, so their scope would be guesswork. I do not recommend this.

### B2. The verification script contradicts one claim

The proposal says the broadcast calendar's "months run 4-4-5 in 2024 and 5-4-4 in 2021". The script (standard library only, months ending on the last Sunday, years ending on the last Sunday of December) gives:

- 2024: 4-4-5 in every quarter. **Holds.**
- 2021: months `5-4-4 | 4-5-4 | 4-5-4 | 5-4-4`. Only Q1 and Q4 are 5-4-4. **The claim holds for Q1 only, not for the whole year.**

Every other script claim reproduces (17 of 18 checks pass; full output in `scratchpad/verify/`). Proposed fix: in every artifact, write "Q1 runs 4-4-5 in 2024 and 5-4-4 in 2021", which is true, and record the correction on the Evidence page. I will not edit your source documents.

### B3. Two of your standing rules conflict with this task

1. `~/.claude/rules/github-issues.md` says to create **every** issue through the web form, never `gh issue create --body` or the API. This task says to seed the backlog through the API with labels, milestones and project fields. Seeding about 40 issues through the browser, with you submitting each, conflicts with "don't work around the API with browser automation".
   **Recommended:** create the seeded issues through the API for this one task. The rule's reasons (template labels, read-only accounts) do not apply: you own the repo and I set labels explicitly. I will still draft each issue in its template's sections.
2. The same rule says to end each issue body with an AI-assistance disclosure. This task says no AI or model name in issues, discussions or files.
   **Recommended:** follow this task (no disclosure line), because it is the more specific, later instruction.

### B4. Smaller decisions with a default I will apply unless you object

| Decision | Default |
| --- | --- |
| Copyright holder in `LICENSE-*` and author in `CITATION.cff` | "Karthik Parameshwara" (the name on your personal-website repo). Tell me if you prefer "fiscalclock contributors" or another form. |
| Project name | The repo is already `fiscalclock`, but the proposal lists the name as an open decision. ADR-0026 stays **Proposed**, records that `fiscalclock` is free on all three registries as of 31 Oct 2026, and I do **not** claim any package name (that is an outward, hard-to-reverse action). |
| Initial commits | Pushed straight to `main`, because the repo is empty and no ruleset exists yet. The ruleset is proposed afterwards and applied only on your yes. |
| Extra ADRs | The documents make three more decisions than your list names. I propose adding them (ADR-0017 to 0019 below). Say if you want them dropped. |
| ADR-0001 status | Recording decisions with ADRs is decided by your brief, not by the documents. Its status will read "Accepted (maintainer decision, 31 Oct 2026)" rather than citing the documents. |
| `dependabot.yml` | Added for `github-actions` only, so the SHA-pinned actions get update PRs. Not requested; drop it if unwanted. |

### B5. Contradictions inside the source documents (flagged, not resolved)

These go into the PDR's open questions and the relevant ADRs, never silently resolved.

| # | Contradiction | Where |
| --- | --- | --- |
| D1 | `stub` is documented as `stub: first_and_last` in the block table, but the Vishay example uses `stub: { "anchor": {...} }`. | Proposal: Core model vs Worked definitions |
| D2 | The Nepal example uses `year_number: "table"` and `boundaries[].number`. Neither appears in the block table (`year_number` lists only `start`, `end`, `majority`; `boundaries` is `[date, …]`). | Proposal: Core model vs Nepal example |
| D3 | `anchor: {edge, month, day, rule}` has no weekday field. The worked examples imply the year-end weekday is the day before `week.starts_on` (NRF Sunday start gives Saturday ends), but the proposal never states it. | Proposal: Core model |
| D4 | `WEEKS_SUM_MISMATCH` defines the required sum only inside a `week_based` year. What a `week_pattern` must sum to inside a `fixed_date` year (Vishay) is unspecified. | Proposal: Validation rules |
| D5 | The executive summary says "Five oracles"; the oracle table lists seven (adds `fiscalyear` and the broadcast calendar). | Proposal |
| D6 | The gap analysis puts `equivalent_day` in v0.1; the proposal's feature table puts it in v0.3. I follow the proposal (source of truth). | Gap analysis: v0.1 scope changes |
| D7 | The gap analysis puts the `fiscalyear` compat module in v0.1; the proposal gives it no release. | Both |
| D8 | The gap analysis says "Every claim in the proposal was checked against a primary source", but some sources are Wikipedia and a forum post (Akaunting), which are secondary. The Evidence page will classify each source honestly. | Gap analysis: Evidence base |
| D9 | The gap analysis uses `is_long`; the proposal uses `has_extra_week`. Canonical term: `has_extra_week`. | Both |
| D10 | Unsourced claims in the proposal: Thai government years use the Buddhist Era (CE + 543); "many Japanese companies" name April–March by start year; "FY 2025-26" naming is common in India, the UK and Australia; Indian income tax FY/AY naming. I will look for a primary source for each; any I cannot source becomes an open question. | Proposal: The naming axis |
| D11 | The Care.com divergence years depend on reading "closest and prior to December 31" as "on or before". Read strictly (excluding 31 Dec), 2022 also diverges. The documents' list (2020, 2021, 2025–2027) also holds only for a 2019–2028 window; 2010, 2014, 2015, 2031 and 2032 diverge too. | Proposal and gap analysis |
| D12 | Whether `anchored` subdivision and `special_periods` are implemented in v0.1 is unclear: the risk table says to add `table`, `anchored` and `transitions` to the format "before v0.1", while v0.1 is described as "calendar-month and week-based years". | Proposal |

## 2. Execution order

Each step ends in a check before the next starts. Commits use Conventional Commits with no attribution lines.

1. **Evidence first.** Move the verification script into `research/verification/` with its exact output and a README. Re-fetch every source and every time-sensitive fact (section 9). Build the claims register as data (one row per claim) before writing any prose, so every page cites a register row.
2. **Glossary and canonical-names table.** Fix the terms, field names, error codes, phase names and label names once; every later artifact uses them.
3. **Repository files** in five commits (section 3).
4. **Mechanical checks locally**, then push and confirm the CI workflow passes on GitHub.
5. **Wiki** (section 4), pushed through the wiki git remote.
6. **Settings, labels, milestones, project fields** through `gh api` / GraphQL.
7. **Issue list shown to you**, then created after your yes.
8. **Discussion categories**: you create them by hand; I seed posts after you confirm.
9. **Review loop**: independent sceptical reviewer, fixes, second pass on the changed parts, consistency sweep, mechanical checks again.
10. **Read everything back through the API** and write the final report.

## 3. Repository files

Commit 1, `test: add reproducible verification of date-arithmetic claims`

- `research/verification/verify_claims.py`: standard library only; checks C01–C18 (400-year rule for all 168 rules in two windows, NRF and Apple 53-week years 2016–2030, NRF fiscal 2023/2024 bounds, the same-weekday and same-position examples, NRF 2017 restated week, broadcast month and quarter lengths 2020–2031, ISO week equivalence 1600–2399, Care.com divergence with both readings, Python `date` overflow). Exits non-zero on any failure.
- `research/verification/expected_output.txt`: exact output.
- `research/verification/README.md`: how to run it, what each check maps to on the Evidence page, how CI compares output.
- C12 (broadcast 2021) will be rewritten to assert what is true (Q1 5-4-4) and the original wording recorded as corrected, once you approve B2.

Commit 2, `chore: add licences and repository configuration`

- `LICENSE-MIT`, `LICENSE-APACHE`, `.editorconfig`, `.gitignore` (Rust, Python, Node), `.github/CODEOWNERS` (`* @Karthik-beta`), `.github/dependabot.yml` (actions only, see B4).

Commit 3, `docs: add README and community health files`

- `README.md`: what it is, why, status "design phase, pre-v0.1", planned packages (marked planned), what makes results trustworthy, wiki links, contributing, licence, out-of-scope list.
- `CONTRIBUTING.md`, `GOVERNANCE.md`, `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1, fetched verbatim from the official source), `SECURITY.md`, `SUPPORT.md`, `CITATION.cff`, `CHANGELOG.md` (Keep a Changelog, Unreleased only).

Commit 4, `chore: add issue forms, discussion forms and PR template`

- `.github/ISSUE_TEMPLATE/config.yml` (`blank_issues_enabled: false`; contact links to Discussions, wiki, security policy).
- `bug_report.yml`, `wrong_result.yml` (real calendar), `new_calendar_shape.yml`, `feature_request.yml`, `documentation.yml`, `binding_packaging.yml`. Each applies its type label plus `status: needs-triage`; dropdowns for binding, capability and phase.
- `.github/PULL_REQUEST_TEMPLATE.md`.
- `.github/DISCUSSION_TEMPLATE/ideas.yml`, `calendar-showcase.yml`, `q-a.yml` (file names must equal the category slugs, so the categories must exist first; see section 7).
- Validation: `check-jsonschema --builtin-schema vendor.github-issue-forms` and `vendor.github-discussion` where available.

Commit 5, `ci: add documentation and verification checks`

- `.github/workflows/docs.yml` on `pull_request` and `push` to `main`: markdownlint-cli2, lychee (link check), cSpell with the `en-GB` dictionary, yamllint plus issue-form schema validation, and `python research/verification/verify_claims.py | diff - research/verification/expected_output.txt`. Actions pinned to full commit SHAs (resolved through the API, with the tag in a trailing comment); top-level `permissions: contents: read`.
- Config files: `.markdownlint-cli2.yaml`, `cspell.json` plus `project-words.txt`, `lychee.toml`, `.yamllint.yaml`.

Commit 6, `docs: add AGENTS.md and CLAUDE.md` (section 5).

Commit 7, `docs: add directory skeleton`

- One-paragraph `README.md` in each of `crates/core`, `bindings/python`, `bindings/js`, `cli`, `spec`, `conformance`, `docs`. No code.

## 4. Wiki pages

All pages: accurate title, first sentence states the point, "Last reviewed: 31 Oct 2026", Glossary terms only, sources cited through the Evidence page.

| Page | Content |
| --- | --- |
| `Home` | Overview, status, map of every page. The existing one-line draft is folded in here. |
| `Analysis` | Families, extra shapes, naming axis; landscape with the corrections from the gap analysis; gap table. |
| `Evidence` | Claims register: claim, source link, verbatim quote or check, method (fetched / script / filing), date verified. Script claims link to `research/verification/` with their check ID. |
| `PDR` | Problem, goals, non-goals, users and their questions, FR-001… by capability and release, NFR-… (correctness, cross-language identity, range, performance, size budget, security, supply chain: trusted publishing and provenance on every registry), acceptance criteria per gate, success metrics, open questions (B5 items), traceability table. |
| `Definition-Format` | Five blocks field by field; validation codes; worked definitions, with Vishay's stub anchor and Nepal's dates marked illustrative. |
| `Semantics` | Comparable periods, equivalent day, proration bases, labels, date inputs, edge cases. |
| `Architecture` | One core plus bindings; per-package table; Mermaid diagram. |
| `Agents-and-MCP` | Agent contract, `describe` / `infer` / `explain`, the nine tools, the registry. |
| `Testing-Strategy` | Vectors, oracles, properties (with the rule-only scoping), differential tests, fuzzing, benchmarks, and how `research/verification/` relates. |
| `Roadmap` | Six phases and five gates (blocked on B1), linked to milestones and the project. |
| `Risks` | The risk table. |
| `Glossary` | Canonical terms. |
| `ADR-Index` | Table of all ADRs with status and links. |
| `_Sidebar`, `_Footer` | Footer: licence, last-reviewed date, link to the documentation issue form with a pre-filled "Wiki:" title. |

### ADRs (MADR: Context, Decision drivers, Considered options, Decision, Consequences, Status, Links)

Decided by the documents (status **Accepted**):

| ADR | Title |
| --- | --- |
| 0001 | Record architecture decisions with ADRs (maintainer decision, see B4) |
| 0002 | Single Rust core with thin bindings |
| 0003 | Own date type, no date-crate dependency in the core |
| 0004 | Calendar as data with no presets; registry kept outside the API |
| 0005 | Independent definition blocks |
| 0006 | Civil dates only |
| 0007 | Half-open intervals |
| 0008 | Canonical JSON and `calendar_id` |
| 0009 | JSON Schema generated from Rust types |
| 0010 | Structured errors; ambiguity is an error |
| 0011 | PyO3 and maturin with abi3 plus free-threaded wheels |
| 0012 | Arrow PyCapsule Interface as the Python bulk path |
| 0013 | MCP server shipped through the npm and PyPI packages |
| 0014 | Dual MIT / Apache-2.0 licence |
| 0015 | Scope boundaries |
| 0016 | Required modes for equivalent day and proration basis |
| 0017 | Pure, deterministic core with no global state (proposed addition) |
| 0018 | Supported date range 0001-01-02 to 9999-12-30 (proposed addition) |
| 0019 | Cross-language conformance by comparing JSON output (proposed addition) |

Open decisions (status **Proposed**, each linked to its Design decisions discussion):

| ADR | Title |
| --- | --- |
| 0020 | npm build toolchain: wasm-bindgen alone or NAPI-RS v3 with WASM fallback |
| 0021 | Whether `history`, `transitions` and `table` years ship in v0.1 or v0.2 |
| 0022 | Whether `table` years accept period boundaries |
| 0023 | Week-numbering default for calendar-month years |
| 0024 | `stub` semantics for fixed-date years with week-based periods (includes D1) |
| 0025 | Minimum supported versions: Rust MSRV, Python floor, Node.js floor |
| 0026 | Project name |
| 0027 | Definition registry location and reviewers |

## 5. AGENTS.md outline

- Five-line summary with links to the PDR and ADR index as the source of truth.
- Repo map: what belongs in each folder, and what never does.
- Design rules, each phrased as a check a reviewer can run (for example "No public constructor, function or constant names a company, country or calendar family"; "No function in `crates/core` reads the system clock or environment"; "Every public function that accepts a date rejects `datetime.datetime`, pandas `Timestamp` and JS `Date` with error code …"). Covers all ten rules you listed.
- Evidence rules applied to code, docs and tests.
- Correctness workflow: conformance vectors with a cited source for every behaviour change; property-test scope; slow reference implementation; never change an expected result without a linked issue.
- When an ADR is required and how to write one.
- Build, test and lint commands: "TBD — added when code lands".
- Commit and PR conventions, including no AI or model attribution lines.
- Out-of-scope list; "ask before adding any dependency to the core".
- `CLAUDE.md` contains only `@AGENTS.md`.

## 6. Settings, labels, milestones, project

Settings through `gh api -X PATCH repos/…`: description, the 10 topics, `allow_squash_merge: true`, `allow_merge_commit: false`, `allow_rebase_merge: false`, `delete_branch_on_merge: true`, `has_issues`, `has_discussions`, `has_wiki: true`; `PUT /vulnerability-alerts`; private vulnerability reporting already on.

Proposed `main` ruleset (applied only on your yes): require a pull request (0 approvals while you are the sole maintainer), require the `docs` workflow checks to pass, require linear history, block force-pushes and deletion, repository admin bypass off.

Labels: delete the six unused defaults (`bug`, `documentation`, `enhancement`, `invalid`, `question`, `accessibility`); keep and recolour `duplicate`, `wontfix`, `good first issue`, `help wanted`; create the full set you listed with the `type: bug` style (colon, space), one colour family per prefix, and a one-line description each. Total 49 labels.

Milestones: six, titled `Phase 0: Foundations`, `Phase 1: v0.1`, `Phase 2: v0.2`, `Phase 3: v0.3`, `Phase 4: v0.4`, `Phase 5: v1.0`. Descriptions copied from the roadmap, **blocked on B1**.

Project "fiscalclock Roadmap" (user-level, linked to the repo). Fields: Status (Backlog, Ready, In progress, In review, Done), Phase (0–5), Area (15 options matching the area labels), Priority (P0–P3), Size (XS, S, M, L, XL), Gate (A–E, None), Target release (v0.1, v0.2, v0.3, v0.4, v1.0). Manual steps for views and auto-add go in the final report.

## 7. Discussions

You create these categories by hand, in this order, before I seed: Announcements (announcement), Design decisions (open-ended discussion), Ideas (open-ended discussion), Q&A (question and answer), Calendar showcase (open-ended discussion), Show and tell (open-ended discussion). Exact click-path in the final report and in my next message.

Seed posts (GraphQL `createDiscussion`) after you confirm:

- Announcements: welcome and project status (design stage, links to PDR, ADRs, how to help).
- Design decisions: eight threads, one per Proposed ADR 0020–0027, options and trade-offs stated neutrally.
- Calendar showcase: invitation for real-world calendars with the citation requirement spelt out.

## 8. Draft issue list (titles only; full bodies come to you before creation)

Phase split depends on B1. This draft follows the proposal's checklist and feature table.

Tracking: `Phase 0 tracking`, `Phase 1 tracking` (task lists linking each issue).

Phase 0 (foundations: format, evidence, decisions)

1. Specify definition format v1 in `spec/`, block by block
2. Resolve the spec contradictions D1–D4 (stub shape, `year_number: table`, boundary numbers, anchor weekday, fixed-year week sums)
3. Define canonical JSON and the `calendar_id` hash (length, encoding, defaults written out)
4. Write the validation error catalogue (codes, paths, hints, message rules)
5. Define the conformance vector file format and `conformance/` layout
6. Convert the verification script's checks into the first conformance vectors
7. Vishay: confirm the Q1 anchor against reported quarter ends (`status: needs-source`)
8. Care.com: vector expecting `RULE_CONFLICT` with the affected years
9. to 16. One issue per Proposed ADR (0020–0027): "Decide ADR-00xx: …", each linked to its discussion, `status: needs-adr`
17. Claim the package name on crates.io, PyPI and npm (blocked on ADR-0026)
18. Set up trusted publishing and build provenance on crates.io, PyPI and npm

Phase 1 (v0.1)

19. Own `Date` type: proleptic Gregorian, `i32` day numbers, public range checks
20. `fixed_date` year bounds with `short_month`
21. `week_based` year bounds with `edge` and `rule`
22. `calendar_months` subdivision
23. `week_pattern` subdivision with `extra_week`
24. `week` block and week numbering
25. Whole-range validator emitting the catalogue codes
26. `locate`, period objects and navigation
27. Year inspection (`years`, `has_extra_week`)
28. Labels: `format`, `parse`, `century_pivot`, `AMBIGUOUS_LABEL`, `TEMPLATES_AMBIGUOUS`
29. Date dimension export stamped with `calendar_id`
30. `explain` trace
31. JSON Schema generation from Rust types, with every vector validated against it
32. Slow reference implementation and differential test, 1900–2200
33. Property tests scoped as in the proposal (tiling, nesting, round trips, rule-only properties)
34. Fuzz the definition and label parsers
35. pandas `FY5253` and ISO oracles as vector generators
36. Python package: PyO3/maturin, abi3 plus `cp314t`, exact `date` check, ISO strings, type stubs
37. Python Arrow PyCapsule bulk path
38. Benchmarks: single-date latency and million-date throughput (Python)

`good first issue` candidates (each completable from the issue and its links alone)

39. Record Kroger fiscal 2018 year and quarter end dates, quoted from its 10-K, in `research/filers/kroger.md`
40. Record Costco fiscal 2017 period structure and year end, quoted from its 10-K, in `research/filers/costco.md`
41. Record Apple's fiscal 2023 quarter end dates from its 10-K and confirm the 53-week year
42. Add an NRF 2009–2028 53-week-year check to the verification script, citing NRF's published list

## 9. Time-sensitive facts to re-verify (each written "as of <date>" with its link)

1. `fiscalyear`: latest release and date, open issues #1, #6, #10, #20 (state, dates, comment counts), monthly downloads, and the v0.2.0 month-end crash fix.
2. PEP 803 status and the Python version that adds `abi3t`; Python 3.14 and 3.15 release status.
3. Temporal: Safari / WebKit status (bug 320164 and the bug it duplicates) and Node.js 26 unflagged support.
4. Jiff 1.0 status ("no timeline" wording in the README).
5. wasm-bindgen's new organisation and wasm-pack's archive status.
6. Latest MCP specification revision.
7. crates.io: `fiscal` and `fiscal-rs` status, the "fiscal" search count, and `fiscalclock` availability; PyPI and npm availability of `fiscalclock`.
8. NAPI-RS v3 release status.
9. retail-calendar weekly downloads and `restated` deprecation; fiscal-year-calendar version and last publish date.
10. Every verbatim quote (Vishay, Costco, Kroger, Care.com, broadcast calendar, IRS Pub. 538, NRF, SEC frames, PEP 803, Polars, Jiff, Solar Hijri, SAP) checked against the live source text.

## 10. Review and checks

- Self-review of each artifact against its definition of done.
- Independent reviewer subagent (no access to my reasoning) given the two source documents, the Evidence register and every artifact; asked for unsupported claims, cross-page contradictions, misquotes, broken links, vague requirements, straw-man ADR options and domain-expert objections. Fix all findings, then a second review on changed parts.
- Consistency sweep against a canonical-names table (terms, field names, error codes, phase names, gates, versions, dates) covering README, AGENTS.md, wiki, ADRs, milestones, labels, issues.
- Mechanical checks: markdownlint, lychee, cSpell (en-GB), yamllint, issue-form schema validation, verification script. All findings fixed.
