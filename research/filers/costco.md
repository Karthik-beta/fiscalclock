# Costco fiscal 2017

Costco's fiscal 2017 is a 53-week year of thirteen four-week periods that ended on Sunday 3 September 2017, and spec version 1 can express it.

Last reviewed: 1 Nov 2026

## Document

| Item | Value |
| --- | --- |
| Title | Costco Wholesale Corporation Annual Report 2017 |
| Form | Annual report to shareholders. It reproduces the Form 10-K text for fiscal 2017 (it refers to "this Annual Report on Form 10-K" on page 36), but the PDF has no Form 10-K cover page. |
| Fiscal year | 2017, ended 3 September 2017 |
| Publication | The letter to shareholders is dated December 14, 2017. The PDF's creation date is 15 December 2017. |
| URL | <https://s201.q4cdn.com/287523651/files/doc_financials/2017/ar/2017-Annual-Report-Final-(2).pdf> |
| Host | Costco's investor-relations site, `investor.costco.com`, serves its own files (for example its logo) from `s201.q4cdn.com/287523651/` (observed on <https://investor.costco.com/financials/sec-filings/default.aspx> on 1 Nov 2026). |
| Retrieved | 1 Nov 2026; 72 pages; SHA-256 `bcc826726cd6fe93f9905b15a7bd1831351cbd723783d277e6c41881ec0b780f` |

## What the document says

### Fiscal year rule, periods and quarters

> We report on a 52/53-week fiscal year, consisting of thirteen, four-week periods and ending on the Sunday nearest the end of August. The first three quarters consist of three periods each, and the fourth quarter consists of four periods (five weeks in the thirteenth period in a 53-week year).

Page 6, Business Overview.

> Our fiscal year ends on the Sunday closest to August 31.

Page 23, Management's Discussion and Analysis.

> The Company operates on a 52/53 week fiscal year basis with the fiscal year ending on the Sunday closest to August 31.

Page 44, Note 1, "Fiscal Year End".

The document uses "nearest the end of August" and "closest to August 31" for the same rule.

### Year naming and year ends

> References to 2017 relate to the 53-week fiscal year ended September 3, 2017. References to 2016 and 2015 relate to the 52-week fiscal years ended August 28, 2016, and August 30, 2015, respectively.

Page 6, Business Overview; the same sentences appear in Note 1 on page 44. A fiscal year is named by the calendar year in which it ends.

> Fiscal year 2017 was a 53-week fiscal year ending on September 3, 2017, while 2016 and 2015 were 52-week fiscal years ending on August 28, 2016, and August 30, 2015, respectively.

Page 23, Management's Discussion and Analysis.

### Quarters

Note 12, "Quarterly Financial Data (Unaudited)", gives the weeks in each quarter (pages 65 and 66):

| Table heading in the document | First | Second | Third | Fourth | Total |
| --- | --- | --- | --- | --- | --- |
| 53 Weeks Ended September 3, 2017 | (12 Weeks) | (12 Weeks) | (12 Weeks) | (17 Weeks) | (53 Weeks) |
| 52 Weeks Ended August 28, 2016 | (12 Weeks) | (12 Weeks) | (12 Weeks) | (16 Weeks) | (52 Weeks) |

### Reported dates

| What | Date | Where |
| --- | --- | --- |
| End of fiscal 2017 (53 weeks) | 3 September 2017 | Pages 6, 23, 44 and 65 |
| End of fiscal 2016 (52 weeks) | 28 August 2016 | Pages 6, 23, 44 and 66 |
| End of fiscal 2015 (52 weeks) | 30 August 2015 | Pages 6, 23 and 44 |

The document does not print quarter or period end dates.

## Spec version 1 definition

```json
{
  "spec_version": 1,
  "year": {"type": "week_based", "anchor": {"edge": "end", "month": 8, "day": "last", "rule": "nearest"}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {
    "type": "week_pattern",
    "periods": [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
    "quarters": [3, 3, 3, 4],
    "extra_week": {"period": 13}
  },
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "end", "templates": {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}}}}
}
```

| Field | Value | Basis |
| --- | --- | --- |
| `year.type`, `anchor` | week-based; end; 31 August (`"last"`); nearest | Stated by the filing ("Sunday closest to August 31") |
| `week.starts_on` | `monday` | Stated by the filing: the year ends on a Sunday, so weeks start on Monday (spec version 1 derives the year-end weekday from `week.starts_on`) |
| `week.numbering` | `contains_start` | Illustrative. The document does not number weeks. |
| `subdivision.periods` | thirteen 4-week periods | Stated by the filing ("thirteen, four-week periods") |
| `subdivision.quarters` | 3, 3, 3, 4 | Stated by the filing ("The first three quarters consist of three periods each, and the fourth quarter consists of four periods") |
| `extra_week.period` | 13 | Stated by the filing ("five weeks in the thirteenth period in a 53-week year") |
| `year_number` | `end` | Stated by the filing ("References to 2017 relate to the 53-week fiscal year ended September 3, 2017") |
| `templates` | `FY{yyyy}`, `Q{q} FY{yyyy}` | Illustrative |

Nothing in this document falls outside spec version 1.

## Conformance vector

`conformance/vectors/cited/costco-annual-report-2017.json`, from `conformance/generators/spec_cited_vectors.py`, has 14 cases: validation, the end and week count of fiscal 2015, 2016 and 2017, the weeks of every quarter of 2016 and 2017, and the weeks of period 13 in 2016 (4) and 2017 (5, holding the extra week).

## Still to check

- Quarter and period end dates for fiscal 2017, from Costco's quarterly reports.
