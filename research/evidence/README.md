# Evidence register

This folder holds the machine-readable register of every factual claim the wiki and the research documents make, the script that rechecks its quotes against their sources, and the script that renders the wiki [Evidence](https://github.com/Karthik-beta/fiscalclock/wiki/Evidence) page from it.

## Files

| File | What it is |
| --- | --- |
| `claims.json` | The register. Edit this file, never the rendered page. |
| `check_quotes.py` | Validates the shape of `claims.json`, then fetches each source that has a quote and reports every quote as FOUND, NOT FOUND or FETCH FAILED. Standard library only. |
| `render_evidence.py` | Renders `Evidence.md` from `claims.json`. Standard library only; the output depends on `claims.json` alone. |
| `Evidence.md` | The rendered page, committed so that CI can tell when it is stale. The wiki page is a copy of this file. |

## Commands

Run them from the repository root.

```sh
python -I research/evidence/check_quotes.py --offline   # validate claims.json, no network
python -I research/evidence/check_quotes.py             # fetch every source and recheck every quote
python -I research/evidence/render_evidence.py          # rewrite Evidence.md
python -I research/evidence/render_evidence.py --check  # exit 1 if Evidence.md is stale
```

The offline check and `--check` need no network and give the same result on every run. The online check depends on third-party sites, so a source that changes or disappears makes it fail; FETCH FAILED alone does not fail the run.

To publish the page, copy `Evidence.md` over `Evidence.md` in the wiki repository, or render straight into it with `--output <wiki clone>/Evidence.md`.

## The register

`claims.json` is an object with four keys.

- `last_reviewed`: ISO date shown on the page's "Last reviewed" line.
- `cspell_ignore`: proper nouns rendered into the page's `cspell:ignore` comment.
- `topics`: the page sections in order. Each has an `id`, a `heading`, a `layout` and an optional `intro` and `outro` paragraph. `claims` renders the Claim, Source, quote, Method, Status and Verified columns; `checks` renders the check ID instead of the source; `open` renders the Where it appears column for claims with no authoritative source.
- `claims`: one object per claim, in page order.

| Field | Required | Meaning |
| --- | --- | --- |
| `id` | yes | `E` and three digits, unique. Never reuse a deleted ID. |
| `topic` | yes | The `id` of a topic. |
| `claim` | yes | The claim as the research documents or the wiki make it (Markdown). |
| `source_label` | yes | Link text for the source; null only when there is no source. |
| `source_url` | yes | The source a reader should open, or null. |
| `fetch_url` | no | What `check_quotes.py` fetches when it differs from `source_url`, for example an API or a mirror without a bot challenge. |
| `quote` | yes | The verbatim text, a list of verbatim passages from the same source, or null when nothing quotable exists (a number in a JSON API, a script check). |
| `evidence` | yes | The cell shown under the quote or result column (Markdown). Every `quote` must appear in it. |
| `check` | yes | The ID of a check in [`research/verification/README.md`](../verification/README.md), or null. Set exactly when `method` is `script check`. |
| `method` | yes | `fetched page`, `script check`, `filing` or `secondary`; null only when there is no source. |
| `method_note` | no | Shown in brackets after the method. |
| `status` | yes | `verified`, `quote corrected`, `fetch failed` or `unverified`. |
| `status_note` | no | Shown in brackets after the status, for example "as of 1 Nov 2026" for a time-sensitive fact. |
| `verified_on` | yes | ISO date of the last check that confirmed the row, no later than `last_reviewed`. |
| `appears_in` | open topics only | Where the unverified claim appears. |

`check_quotes.py --offline` enforces these rules, and also that every check in `research/verification/README.md` has a claim.

## Adding or changing a claim

1. Add an object to `claims` with the next free ID, under the right topic. Copy the quote from the source page itself, not from another document, and keep the source's spelling and punctuation.
2. Run `python -I research/evidence/check_quotes.py --id E087` (with the new ID) and confirm FOUND. If the host refuses automated clients, set `fetch_url` to an API or archived copy that serves the same text, or record the row as `fetch failed` and say in `evidence` how it was checked instead.
3. Set `verified_on` to the date of the check and update `last_reviewed`.
4. Run the offline check and `render_evidence.py`, and commit `claims.json` and `Evidence.md` together.

Never change a quote to make the check pass. If the source no longer says it, change the status and explain why in `evidence`.

sec.gov is never fetched, because it requires a contact email address in the User-Agent and the project does not send one. SEC rows stay `fetch failed` unless they were compared against a saved copy of the filing.
