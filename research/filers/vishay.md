# Vishay

Vishay's own 10-Qs state its quarter rule and list its quarter end dates for 2024, 2025 and 2026. The dates fit a `stub` anchor of the Saturday nearest 1 April, not the Saturday nearest 31 March that the research documents assumed.

Last reviewed: 21 Nov 2026

## Documents

Both are HTML copies of Form 10-Q on Vishay's investor-relations site, reached from its SEC filings page (`https://ir.vishay.com/financial-information/sec-filings`). They were read in a browser on 21 Nov 2026. The site answers automated requests with HTTP 403 Forbidden, as it did on 1 Nov 2026, so these addresses are written as code, not links, because they do not resolve for automated link checks.

| Document | Address |
| --- | --- |
| Form 10-Q for the quarterly period ended October 3, 2026, filed 4 Nov 2026 | `https://ir.vishay.com/node/21866/html` |
| Form 10-Q for the quarterly period ended March 29, 2025, filed 7 May 2025 | `https://ir.vishay.com/node/20161/html` |

The rule below is in a 10-Q, not in the annual report: the Forms 10-K for 2024 and 2025 on the same site do not contain the sentence. This departs from the [rule for these pages](README.md#rules-for-these-pages), which names the annual report or Form 10-K. The 10-Q is the company's own copy on its own investor-relations host. The quotes were compared character for character with the page text that the browser extracted. They are recorded as claims E010 and E011 in [`research/evidence/claims.json`](../evidence/claims.json).

## What the filings say

In the notes to the financial statements of both 10-Qs:

> The Company reports interim financial information for 13-week periods beginning on a Sunday and ending on a Saturday, except for the first fiscal quarter, which always begins on January 1, and the fourth fiscal quarter, which always ends on December 31.

The 10-Q for the quarter ended October 3, 2026:

> The four fiscal quarters in 2026 end on April 4, 2026, July 4, 2026, October 3, 2026, and December 31, 2026, respectively.

and, in the same note:

> The four fiscal quarters in 2025 ended on March 29, 2025, June 28, 2025, September 27, 2025, and December 31, 2025, respectively.

The 10-Q for the quarter ended March 29, 2025:

> The four fiscal quarters in 2024 ended on March 30, 2024, June 29, 2024, September 28, 2024, and December 31, 2024, respectively.

Neither filing names a rule for where the first quarter ends. It gives the dates only.

## The anchor

Checks C20 and C21 in [`research/verification/`](../verification/README.md) reproduce these results with the Python standard library:

- Every reported first-quarter end (30 March 2024, 29 March 2025, 4 April 2026) is the Saturday nearest 1 April, and the second and third quarters are 13 weeks each (C20).
- "Saturday nearest 31 March", the anchor in the research documents, gives 28 March 2026, not the reported 4 April 2026 (C21). It is refuted.

"Saturday nearest 1 April" fits three years. It is still an inference from dates, not the filing's own wording, so the anchor is marked illustrative below. Issue #14 asked for the filing sentence that states the rule; the filings state the rule for quarters 2 and 3 and give the dates, but no sentence names the anchor.

## Spec version 1

fiscalclock 0.2 cannot express this calendar. The year is fixed (1 January to 31 December) and the quarters are made of whole weeks except at the two ends of the year. A `week_pattern` subdivision inside a `fixed_date` year needs `stub`, which is reserved, so such a definition is rejected with `UNSUPPORTED_COMBINATION` ([spec/definition-v1.md](../../spec/definition-v1.md), "`subdivision`"). `stub` is planned for 0.3 ([ADR-0024](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0024-stub-semantics)).

With the shape that ADR-0024 decides, the definition would be:

| Field | Value | Basis |
| --- | --- | --- |
| `year.type`, `start` | `fixed_date`, 1 January | Stated by the filing ("always begins on January 1", "always ends on December 31") |
| `week.starts_on` | `sunday` | Stated by the filing ("beginning on a Sunday and ending on a Saturday") |
| `subdivision.periods` | `[13, 13, 13, 13]`, one period per quarter | Stated by the filing for quarters 2 and 3; quarters 1 and 4 are the stubs |
| `subdivision.stub.anchor` | `{ "month": 4, "day": 1, "rule": "nearest" }` | Illustrative: fits the reported dates for 2024 to 2026 (C20) |
| `subdivision.stub.absorb` | `"first_and_last"` | Stated by the filing (the first quarter starts on 1 January, the fourth ends on 31 December) |

## Still to check

1. A cited vector, once `stub` is implemented (issue #86). Its expected quarter ends are the dates quoted above.
2. Earlier years' quarter ends, from older 10-Qs on the same site, would test the anchor further. The SEC filings page listed filings back to February 2025 when it was read.
