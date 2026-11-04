# Vishay

Vishay's annual report could not be retrieved from its investor-relations site, so the fiscal year anchor is neither confirmed nor refuted and no vector was added.

Last reviewed: 1 Nov 2026

## Document

Not retrieved. On 1 Nov 2026 Vishay's investor-relations site returned HTTP 403 Forbidden to automated requests, both for its home page (`https://ir.vishay.com/`) and for the 2023 annual report file a web search listed there (`https://ir.vishay.com/static-files/3196bb59-a761-4c35-8018-14b60900810a`). The site was not retried by other means. These addresses are written as code, not links, because they do not resolve for automated link checks. No Vishay-hosted copy of an annual report or Form 10-K was found elsewhere.

## What is recorded elsewhere, and its status

- The research documents quote a Vishay Form 10-Q from 2024 as describing "13-week periods beginning on a Sunday and ending on a Saturday, except for the first fiscal quarter, which always begins on January 1, and the fourth fiscal quarter, which always ends on December 31". The evidence register (claim E010 in [`research/evidence/claims.json`](../evidence/claims.json)) records that this sentence was matched only through a web-fetch summary, not a raw comparison. It has not been checked against a Vishay-hosted document. Unverified.
- The evidence register (claim E011) records that two reported first-quarter ends, 1 April 2023 and 30 March 2024, match "Saturday nearest 31 March". The research proposal states that the filing does not name this anchor. Both are unverified against a Vishay document.

Issue #14 asks whether the anchor "Saturday nearest 31 March" matches the filing's own wording. That remains open.

## Spec version 1

Spec version 1 cannot express this calendar, whatever the anchor turns out to be. The year is fixed (1 January to 31 December), and the quarters are made of whole weeks except at the two ends of the year. In spec version 1 a `week_pattern` subdivision inside a `fixed_date` year needs `stub`, which is reserved, so such a definition is rejected with `UNSUPPORTED_COMBINATION` ([spec/definition-v1.md](../../spec/definition-v1.md), "`subdivision`"). `subdivision.stub` is planned for 0.2, after ADR-0024.

A definition for a later spec version would need, at least:

| Field | Value | Basis |
| --- | --- | --- |
| `year.type`, `start` | `fixed_date`, 1 January | Unverified quote above |
| `week.starts_on` | `sunday` | Unverified quote above ("beginning on a Sunday and ending on a Saturday") |
| Quarters | Four quarters; the second and third are 13 weeks | Unverified quote above |
| First-quarter end | Saturday nearest 31 March | Illustrative; not stated in any quoted sentence |
| Fourth quarter | Ends on 31 December, absorbing the remaining days | Unverified quote above |

## Still to check

1. Obtain a Vishay annual report or Form 10-K from a source the project accepts, and quote the sentence that states the quarter rule, with its page.
2. Record whether the document states where the first quarter ends (an anchor such as "Saturday nearest 31 March", or a rule such as "13 weeks from the first Sunday"), or only lists the dates.
3. Record the reported quarter end dates for at least two fiscal years.
4. Add a cited vector once spec support for `stub` exists (ADR-0024).
