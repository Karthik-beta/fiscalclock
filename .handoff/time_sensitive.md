# Time-sensitive fact check for fiscalclock (as of 31 Oct 2026)

Method notes:
- JSON APIs (PyPI, crates.io, npm, ecosyste.ms, pypistats) and the GitHub API (`gh api`) were fetched with curl or gh. Snippets from these are given as JSON field/value (no verbatim prose), or copied character-for-character from the fetched file.
- PEP pages were fetched as HTML and converted to text locally, so PEP quotes are verbatim.
- Items marked **[WebFetch summary]** come from a model-summarized WebFetch result. The quoted text is what the summarizer returned, not a guaranteed raw copy. Re-verify those quotes before publishing.
- Scratch files: `C:\Users\Admin\AppData\Local\Temp\claude\C--Users-Admin-Desktop-fiscalclock\4e154076-abdf-4126-ab33-630145647bdc\scratchpad\evidence\` (raw JSON, HTML, text).

Status legend: MATCH = same as claim; DIFFERS = value differs from claim; PARTIAL = source only partly supports claim; UNVERIFIED = could not verify.

---

## 1. fiscalyear (PyPI) latest version and release date

- **Current value:** 0.4.0, uploaded 2022-02-17T03:18:26 (wheel, sdist 03:18:28).
- **URL:** https://pypi.org/pypi/fiscalyear/json (raw file: `pypi_fiscalyear.json`)
- **Evidence (JSON):** `info.version` = `"0.4.0"`; `releases["0.4.0"][0].upload_time` = `"2022-02-17T03:18:26"`.
- **Status:** MATCH.

## 2. fiscalyear monthly downloads

- **Current values (differ by source):**
  - ecosyste.ms: `downloads` = 339074, `downloads_period` = `"last-month"`, `last_synced_at` = `"2026-08-25T01:53:23.088Z"` (stale by about 2 months).
  - pypistats: `"last_month": 206407`, `"last_week": 63948`, `"last_day": 11535`.
- **URLs:** https://packages.ecosyste.ms/api/v1/registries/pypi.org/packages/fiscalyear (raw: `ecosyste_fiscalyear.json`); https://pypistats.org/api/packages/fiscalyear/recent (raw: `pypistats_fiscalyear.json`, full text: `{"data":{"last_day":11535,"last_month":206407,"last_week":63948},"package":"fiscalyear","type":"recent_downloads"}`)
- **Claim:** ~345,000/month.
- **Status:** DIFFERS. Neither source gives ~345,000. ecosyste.ms gives 339,074 (stale snapshot). pypistats gives 206,407. Do not publish one figure without naming the source and date.

## 3. fiscalyear GitHub issues

Source: `gh api repos/adamjstewart/fiscalyear/issues/N` (raw: `gh_issue_N.json`); comment counts also checked via `.../comments`.

| # | Title (JSON `title`) | state | created_at | comments | Claim | Status |
|---|---|---|---|---|---|---|
| 1 | "Support for a 4-4-5 calendar" | open | 2017-05-27T20:33:13Z | 1 | open since May 2017 | MATCH |
| 6 | "Make it possible to use multiple calendar settings in the same app." | open | 2018-02-08T18:14:00Z | 5 | several calendars in one app | MATCH |
| 10 | "Support for week number of a date in a FiscalYear" | open | 2020-01-14T06:23:53Z | 20 | week numbers, 20 comments | MATCH |
| 20 | "Support February 29 month end date for leap year" | open | 2021-09-04T05:39:10Z | 5 | 29 February month end, open | MATCH |

- URLs: https://github.com/adamjstewart/fiscalyear/issues/1, /6, /10, /20

## 4. fiscalyear v0.2.0 crash fix

- **URL:** https://github.com/adamjstewart/fiscalyear/releases/tag/v0.2.0 (release published 2019-10-26T02:50:31Z per API; raw: `gh_releases.json`).
- **Verbatim line (from release body):** `Corner case where fiscal calendar starting at the end of the month could crash is fixed (#8)`
- **Also in the same release:** `@jmhansen reported a bug that could cause `fiscalyear` to crash for some fiscal calendars`
- **Status:** MATCH. The docs changelog was not checked separately. The GitHub release is the source used.

## 5. PEP 803 (abi3t)

- **URL:** https://peps.python.org/pep-0803/ (raw: `pep803.html`, text: `pep803.txt`)
- **Current value:** `Status : Final`, `Python-Version : 3.15`, `Resolution : 30-Mar-2026` (verbatim from the page header).
- **Verbatim sentence (present):** `The Stable ABI is currently not available for free-threaded builds.`
- **Verbatim Abstract:** `Add a new variant of the Stable ABI, called "Stable ABI for Free-Threaded Python" (or abi3t for short).`
- **Status:** MATCH. Final, 30 Mar 2026. abi3t targets Python 3.15.

## 6. Python 3.14 and 3.15 release status

- **PEP 745 (3.14):** URL https://peps.python.org/pep-0745/ (raw `pep745.txt`). Verbatim: `3.14.0 final: Tuesday, 2025-10-07`. Bugfix "Actual" list includes `3.14.10: Monday, 2026-10-26`; "Expected" starts `3.14.11: Tuesday, 2026-12-01`. Page last modified `2026-10-26 09:17:51 UTC`.
- **PEP 790 (3.15):** URL https://peps.python.org/pep-0790/ (raw `pep790.txt`). Verbatim: `3.15.0 final: Friday, 2026-10-09`. Bugfix "Actual" list includes `3.15.1: Wednesday, 2026-10-21` and `3.15.2: Monday, 2026-10-26`.
- **python.org downloads [WebFetch summary]:** URL https://www.python.org/downloads/ . Summarizer reports "Python 3.14.10", released "Oct. 25, 2026", and "Python 3.15.2", released "Oct. 25, 2026".
- **python.org release page [WebFetch summary]:** URL https://www.python.org/downloads/release/python-3150/ . Summarizer reports the note "Python 3.15.0 has been superseded by Python 3.15.2." and release date "Oct. 9, 2026".
- **Status:**
  - DIFFERS (minor): python.org summary says 3.14.10 and 3.15.2 released Oct 25; PEP 745/790 record Oct 26 as the actual date. Confirm the exact date on python.org before publishing.
  - Python 3.15.0 final (Oct 9) is superseded by 3.15.2; state that 3.15.2 is the current 3.15 release.
  - 3.14 is the current bugfix line (PEP 745 status: Active).

## 7. Temporal in Safari

- **WebKit bug 320164:** https://bugs.webkit.org/show_bug.cgi?id=320164 [WebFetch summary]
  - Title: "Immediate Full Adoption of Temporal API in Safari Stable Required"
  - Status: RESOLVED, Resolution: DUPLICATE
  - Duplicate of: Bug 223166, "[JSC] Implement Temporal", listed as REOPENED.
- **Status of duplicate (bug 223166):** REOPENED, per the same page. Not separately fetched. Confirm at https://bugs.webkit.org/show_bug.cgi?id=223166 .
- **caniuse Temporal [WebFetch summary]:** https://caniuse.com/temporal . Safari: `3.1 - 26.6: Not supported`; `27: Not supported`; `27.1 - 27.2: Not supported`; `TP: Supported`. Global usage: `70.18%`.
- **MDN Temporal [WebFetch]:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal . Compat table not in fetched text. Page says "Limited availability" and "This feature is not Baseline because it does not work in some of the most widely-used browsers." Safari entry UNVERIFIED on MDN.
- **Status:** Safari stable does not ship Temporal as of 31 Oct 2026 (caniuse). The WebKit bug is a duplicate, not an open fix. Claim check: the bug is a DUPLICATE of 223166, not an independent open bug.

## 8. Node.js: Temporal in Node 26 and release lines

- **Node 26 release notes [WebFetch summary]:** https://nodejs.org/en/blog/release/v26.0.0 . Verbatim sentence: `The Temporal API is now enabled by default in Node.js 26.` Release heading: `2026-05-05, Version 26.0.0 (Current), @RafaelGSS`.
- **Status:** MATCH. Temporal is unflagged in Node 26.
- **Release lines [WebFetch summary]:** https://nodejs.org/en/about/previous-releases . Page status column uses only Alpha, LTS, EOL (not "Active LTS" or "Maintenance"). Rows:
  - v27: Alpha, first released Oct 29 2026
  - v26 (Lithium): LTS, first released May 05 2026, "Last updated" Oct 28 2026. Page banner calls it "Latest LTS."
  - v24 (Krypton): LTS, first released May 06 2025, "Last updated" Oct 27 2026
  - v22 (Jod): LTS, first released Apr 24 2024, "Last updated" Oct 20 2026
  - v25: EOL, Mar 31 2026; v23: EOL, May 14 2025; v20: EOL, Mar 24 2026; v18: EOL, Mar 27 2025 (and older lines EOL)
- **Status:** PARTIAL. The page does not label lines "Active LTS" or "Maintenance LTS." Map LTS/EOL from this page only with that caveat. The "Last updated" column is not necessarily the official EOL date for LTS lines. Check https://github.com/nodejs/Release for official schedule dates.

## 9. Jiff 1.0 and latest jiff

- **README (raw via gh API, decoded):** https://github.com/BurntSushi/jiff . Verbatim: `It is now April 2026,` / `so that deadline has clearly slipped.` / `I don't currently have a` / `timeline for a Jiff 1.0 release.`
- **Current value:** README still says no timeline for 1.0. Repo `pushed_at` 2026-10-30.
- **Latest on crates.io:** https://crates.io/api/v1/crates/jiff . `crate.max_version` = `0.2.38`, `updated_at` = `2026-10-06T15:01:05.919581Z`.
- **Status:** README says "no timeline" for 1.0 (MATCH with the "no timeline" premise). Latest jiff is 0.2.38. Pre-1.0 README dates are from April 2026.

## 10. wasm-bindgen and wasm-pack

- **Rust blog [WebFetch summary]:** https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org (post dated July 21, 2025 by Alex Crichton). Summarizer quotes: "The wasm-bindgen repository is going to be transferred to a new wasm-bindgen organization with new additional maintainers." and "The current plan at this time is to archive all other repositories in the rustwasm organization."
- **wasm-bindgen repo:** https://github.com/wasm-bindgen/wasm-bindgen (GitHub API `full_name` = `wasm-bindgen/wasm-bindgen`, `archived` = `False`, `pushed_at` = `2026-10-30T20:00:48Z`). Latest crate: https://crates.io/api/v1/crates/wasm-bindgen `max_version` = `0.2.130`.
- **wasm-pack repo:** https://github.com/wasm-bindgen/wasm-pack (GitHub API `full_name` = `wasm-bindgen/wasm-pack`, `archived` = `False`, `pushed_at` = `2026-10-30T05:56:37Z`). The old rustwasm path resolves to the same repo. Latest crate: https://crates.io/api/v1/crates/wasm-pack `max_version` = `0.15.1`.
- **Status:** MATCH to the migration (wasm-bindgen moved to the wasm-bindgen org; wasm-pack is maintained and not archived). Note: the blog text is from a summarizer, so confirm the exact sentences before quoting.

## 11. MCP specification latest revision

- **URL:** https://modelcontextprotocol.io/specification (WebFetch). Verbatim: `based on the TypeScript schema in` ... `schema/2026-07-28/schema.ts`; links use `https://modelcontextprotocol.io/specification/2026-07-28/...`.
- **Current value:** 2026-07-28 (per the spec page). GitHub releases page not fetched.
- **Status:** MATCH for the spec site. GitHub releases page UNVERIFIED (not fetched). Re-check https://github.com/modelcontextprotocol/specification/releases before publishing.

## 12. crates.io names and PyPI/npm existence

- **`fiscal` crate:** https://crates.io/api/v1/crates/fiscal . `max_version` = `0.8.1`, `updated_at` = `2026-07-06T16:04:07.399452Z`, `description` = `Brazilian fiscal document library (NF-e/NFC-e) — Rust port of sped-nfe`, `repository` = `https://github.com/JoaoHenriqueBarbosa/fiscal-rs`. Owner (https://crates.io/api/v1/crates/fiscal/owners): `login` = `JoaoHenriqueBarbosa`, name `John Enrique`.
- **`fiscal-rs` crate:** https://crates.io/api/v1/crates/fiscal-rs . `max_version` = `0.1.0`, `created_at` = `2025-06-18T21:52:10.500161Z`, `description` = `Reserved crate name for future Fiscal related work in Rust`, `repository` = null. Owner: `login` = `IvonaldoAmaral`. GitHub repo (JoaoHenriqueBarbosa/fiscal-rs) `archived` = `False`, `pushed_at` = `2026-10-09T18:38:37Z`.
- **Search total:** https://crates.io/api/v1/crates?q=fiscal&per_page=30 `meta.total` = `109`.
- **`fiscalclock` crate:** https://crates.io/api/v1/crates/fiscalclock returns `{"errors":[{"detail":"crate `fiscalclock` does not exist"}]}`. Does not exist.
- **PyPI `fiscalclock`:** https://pypi.org/pypi/fiscalclock/json returns `{"message": "Not Found"}`. Does not exist.
- **npm `fiscalclock`:** https://registry.npmjs.org/fiscalclock returns `{"error":"Not found"}`. Does not exist.
- **Status:** `fiscal` and `fiscal-rs` both exist and are owned by other people. `fiscalclock` is free on crates.io, PyPI and npm as of today. Re-check just before publishing.

## 13. NAPI-RS v3

- **URL:** https://napi.rs/blog/announce-v3 [WebFetch summary]. Post dated 2025/07/07. Tagline: `NAPI-RS v3 - WebAssembly! Safer API design and new cross compilation features.`
- **Status:** PARTIAL. The post announces v3 but does not use the word "released" in the fetched text. Verify with the npm `@napi-rs/cli` registry entry before saying "released." Not checked in this run.

## 14. retail-calendar (npm)

- **Latest version:** `4.1.2`, `time["4.1.2"]` = `2024-10-30T17:48:46.253Z`. Source: https://registry.npmjs.org/retail-calendar (raw `npm_retail.json`). `dist-tags.latest` = `"4.1.2"`.
- **Weekly downloads:** https://api.npmjs.org/downloads/point/last-week/retail-calendar returns `{"downloads":5252,"start":"2026-10-23","end":"2026-10-29","package":"retail-calendar"}`.
- **`restated` in README:** README (from registry `readme`) contains the heading `#### restated [Removed]` and the line `⚠ *This option has been superseded by [LeapYearStrategy](#LeapYearStrategy)* ⚠`. Also `#### LeapYearStrategy [Removed]`.
- **npm `deprecated` flag:** none of the versions has a `deprecated` field.
- **Status:** DIFFERS if the claim is "marked deprecated." The README marks `restated` as **[Removed]** and superseded by `LeapYearStrategy`. npm does not flag it as deprecated.

## 15. fiscal-year-calendar (npm)

- **Source:** https://registry.npmjs.org/fiscal-year-calendar (raw `npm_fyc.json`).
- **Latest version:** `3.2.1` (`dist-tags.latest`).
- **Last publish:** `time["3.2.1"]` = `2025-07-30T01:40:30.544Z`.
- **Status:** MATCH to the endpoint (no claim value was supplied in the brief for comparison; values reported as current).

## 16. Latest crate and PyPI versions; PyO3 free-threading

| Package | Latest | Source | Field |
|---|---|---|---|
| pyo3 | 0.30.1 | https://crates.io/api/v1/crates/pyo3 | `max_version`, updated 2026-10-14 |
| serde | 1.0.229 | https://crates.io/api/v1/crates/serde | `max_version`, updated 2026-07-18 |
| serde_json | 1.0.154 | https://crates.io/api/v1/crates/serde_json | `max_version`, updated 2026-10-11 |
| sha2 | 0.11.1 | https://crates.io/api/v1/crates/sha2 | `max_version`, updated 2026-10-12 |
| proptest | 1.12.0 | https://crates.io/api/v1/crates/proptest | `max_version`, updated 2026-10-10 |
| wasm-bindgen | 0.2.130 | https://crates.io/api/v1/crates/wasm-bindgen | `max_version`, updated 2026-10-30 |
| maturin | 1.16.0 | https://pypi.org/pypi/maturin/json | `info.version`; upload 2026-10-29T14:09:06 |

- **PyO3 free-threading [WebFetch summary]:** https://pyo3.rs/main/changelog . Under 0.29.0 (2026-06-11), Packaging section: `CPython declared free-threading supported starting with Python 3.14`. The summary says this came from the first 100,000 characters of a truncated page, and the note says the changelog also drops 3.13t support. Re-check the exact wording and version before quoting.
- **Status:** versions reported as current. PyO3 free-threaded 3.14 support: MATCH per summary; quote needs re-verification.

## 17. Trusted publishing

**(a) crates.io: must the crate exist first?**
- https://crates.io/docs/trusted-publishing fetched but returned only the page title. UNVERIFIED from the docs page.
- Rust blog (July 11 2025) [WebFetch summary]: https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/ . Verbatim (per summarizer): `To get started with Trusted Publishing, you'll need to publish your first release manually.` and `After that, you can set up trusted publishing for future releases.`
- Search snippet (not a verbatim source): the crates.io docs reportedly say the crate must already be published, with an initial publish via API token. Not verified from the page itself.
- **Status:** PARTIAL. Strongly suggests the crate must exist (first release manual), but the crates.io docs sentence is UNVERIFIED. Re-fetch the docs page before quoting.

**(b) PyPI pending publisher for a project that does not exist yet.**
- URL: https://docs.pypi.org/trusted-publishers/creating-a-project-through-oidc/ [WebFetch summary]
- Verbatim (per summarizer): `The steps for configuring a "pending" publisher are similar to those for a normal publisher,` / `except that the page is under your account sidebar instead of any project's sidebar (since the project doesn't exist yet).`
- **Status:** MATCH. A pending publisher is configured from the account sidebar for a project that does not yet exist.

**(c) npm trusted publishing.**
- URL: https://docs.npmjs.com/trusted-publishers [WebFetch summary]
- Verbatim (per summarizer): `Trusted publishing requires npm CLI version 11.5.1 or later and Node version 22.14.0 or higher.`
- Verbatim (per summarizer): `npm automatically generates and publishes provenance attestations for your package.` Provenance applies only to GitHub Actions or GitLab CI/CD; can be disabled with `provenance` set to `false`.
- Must the package already exist? The page does not state this. It says a new configuration must complete a first successful publish within 2 days to be validated.
- **Status:** minimum CLI 11.5.1 and Node 22.14.0; provenance automatic by default (on GitHub/GitLab runners). Existence requirement UNVERIFIED (not stated on the page).

## 18. ThreeTen-Extra AccountingChronologyBuilder

- **URL:** https://www.threeten.org/threeten-extra/apidocs/org.threeten.extra/org/threeten/extra/chrono/AccountingChronologyBuilder.html [WebFetch summary]
- **Verbatim (per summarizer):** `There are approximately 7 x 2 x 12 x 4 x 12/13 x 2 = 8064 combinations.`
- **Status:** MATCH in value (8064 = about 8,064). Wording is "approximately ... = 8064", not "about 8,064". Verify the raw sentence before quoting.

---

## Summary of claim differences and gaps

Differs from claim:
- #2 downloads: ecosyste.ms 339,074 (stale snapshot); pypistats 206,407. Neither equals ~345,000.
- #14 restated: README marks it **[Removed]** and superseded; no npm deprecation flag.
- #6 Python 3.15.0 final (9 Oct) superseded by 3.15.2; python.org and PEP 790/745 disagree on the 3.14.10 and 3.15.2 date (Oct 25 vs Oct 26).

Partial or unverified:
- #13 NAPI-RS v3 "released": post announces v3 but never says "released" in the fetched text.
- #17(a) crates.io docs requirement: UNVERIFIED from the docs page; Rust blog supports a manual first release.
- #17(c) npm: package-existence requirement not stated on the page.
- #11 MCP: GitHub releases page not checked.
- #7 MDN Safari compat: table not captured; caniuse used instead.
- #8 Node release-line labels: page does not use "Active LTS" or "Maintenance".

Matches (no change): #1, #3, #4, #5, #7 (duplicate of 223166 confirmed), #8 (Temporal enabled in Node 26), #9 (jiff 1.0 still no timeline), #10 (wasm-bindgen org move; wasm-pack active), #11 (2026-07-28 on spec site), #12 (`fiscalclock` free on all three registries), #17(b), #18 (8064).

Not individually verified (values reported as current only): #15 fiscal-year-calendar (no claim supplied), #16 crate versions.
