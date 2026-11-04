# Kroger fiscal 2018

Kroger's fiscal 2018 is a 52-week year of thirteen 28-day periods that ended on Saturday 2 February 2019, and spec version 1 can express it.

Last reviewed: 1 Nov 2026

## Document

| Item | Value |
| --- | --- |
| Title | Notice of 2019 Annual Meeting of Shareholders, 2019 Proxy Statement and 2018 Annual Report |
| Form | Combined proxy statement and annual report. The annual report section (pages A-1 onwards) reproduces the Form 10-K text for fiscal 2018 (it refers to "this Form 10-K" on page A-6), but the PDF has no Form 10-K cover page. |
| Fiscal year | 2018, ended 2 February 2019 |
| Publication | The document says it was "first furnished to shareholders on May 14, 2019." |
| URL | <https://s202.q4cdn.com/463742399/files/doc_financials/2018/ar/2018-Annual-Report.pdf> |
| Host | Kroger's investor-relations site, `ir.kroger.com`, serves its own files and current annual report from `s202.q4cdn.com/463742399/` (observed on `ir.kroger.com/overview/default.aspx` on 1 Nov 2026; the site refuses automated link checks, so the address is not a link). |
| Retrieved | 1 Nov 2026; 175 pages; SHA-256 `83d09c8d99d1babdab83c01a9425738724756793c3278b1aae088b70f323b77f` |

## What the document says

### Fiscal year rule

> Kroger’s fiscal year ends on the Saturday closest to January 31.

Page A-3, below the performance graph.

> The Company’s fiscal year ends on the Saturday nearest January 31. The last three fiscal years consist of the 52-week period ended February 2, 2019, 53-week period ended February 3, 2018 and 52-week period ended January 28, 2017.

Page A-35, Note 1, "Fiscal Year".

The two sentences use "closest" and "nearest" for the same rule.

### Year naming

> All references to 2018, 2017 and 2016 are to the fiscal years ended February 2, 2019, February 3, 2018 and January 28, 2017, respectively, unless specifically indicated otherwise.

Page A-5, Item 1 text. A fiscal year is named by the calendar year in which it starts.

### Periods and quarters

> The reported periods conform to our fiscal calendar composed of thirteen 28-day periods. The fourth quarter of 2018 contained three 28-day periods.

Page A-4, footnote (1) to the Issuer Purchases of Equity Securities table.

The same table lists the three periods of the fourth quarter of 2018 (page A-4; the table's cells are quoted in reading order):

| Row label in the document | Dates in the document |
| --- | --- |
| First period - four weeks | November 11, 2018 to December 8, 2018 |
| Second period - four weeks | December 9, 2018 to January 5, 2019 |
| Third period — four weeks | January 6, 2019 to February 2, 2019 |

Note 21, "QUARTERLY DATA (UNAUDITED)", gives the weeks in each quarter (pages A-77 and A-78):

| Fiscal year | First | Second | Third | Fourth | Total year |
| --- | --- | --- | --- | --- | --- |
| 2018 | (16 Weeks) | (12 Weeks) | (12 Weeks) | (12 Weeks) | (52 Weeks) |
| 2017 | (16 Weeks) | (12 Weeks) | (12 Weeks) | (13 Weeks) | (53 Weeks) |

### Reported dates

| What | Date | Where |
| --- | --- | --- |
| End of fiscal 2018 (52 weeks) | 2 February 2019 | Pages A-5 and A-35 |
| End of fiscal 2017 (53 weeks) | 3 February 2018 | Pages A-5 and A-35 |
| End of fiscal 2016 (52 weeks) | 28 January 2017 | Pages A-5 and A-35 |
| Fourth quarter of 2018 | 11 November 2018 to 2 February 2019 | Page A-4 |

The document does not print the end dates of the first three quarters of 2018.

## Spec version 1 definition

```json
{
  "spec_version": 1,
  "year": {"type": "week_based", "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
  "week": {"starts_on": "sunday", "numbering": "contains_start"},
  "subdivision": {
    "type": "week_pattern",
    "periods": [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
    "quarters": [4, 3, 3, 3],
    "extra_week": {"period": 13}
  },
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start", "templates": {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}}}}
}
```

| Field | Value | Basis |
| --- | --- | --- |
| `year.type`, `anchor` | week-based; end; 31 January (`"last"`); nearest | Stated by the filing ("Saturday closest to January 31") |
| `week.starts_on` | `sunday` | Stated by the filing: the year ends on a Saturday, so weeks start on Sunday (spec version 1 derives the year-end weekday from `week.starts_on`) |
| `week.numbering` | `contains_start` | Illustrative. The document does not number weeks; in a week-based year spec version 1 starts week 1 on the year's first day. |
| `subdivision.periods` | thirteen 4-week periods | Stated by the filing ("thirteen 28-day periods") |
| `subdivision.quarters` | 4, 3, 3, 3 | Stated by the filing: quarters of 16, 12, 12 and 12 weeks in a 52-week year, and three periods in the fourth quarter of 2018 |
| `extra_week.period` | 13 | Illustrative. Note 21 shows the 53rd week of 2017 in the fourth quarter (13 weeks), but the document does not say which period held it. |
| `year_number` | `start` | Stated by the filing ("All references to 2018, 2017 and 2016 are to the fiscal years ended February 2, 2019") |
| `templates` | `FY{yyyy}`, `Q{q} FY{yyyy}` | Illustrative |

Nothing in this document falls outside spec version 1.

## Conformance vector

`conformance/vectors/cited/kroger-annual-report-2018.json`, from `conformance/generators/spec_cited_vectors.py`, has 15 cases: validation, the end and week count of fiscal 2016, 2017 and 2018, the weeks of every quarter of 2017 and 2018, the first and last day of the fourth quarter of 2018, and the first and last day of periods 11, 12 and 13 of 2018. Periods 11 to 13 are the fourth-quarter periods because the 16- and 12-week quarters before them hold ten four-week periods. The extra week's period is not asserted.

## Still to check

- Which period held the 53rd week of fiscal 2017. A Kroger document for fiscal 2017 may state it.
- The end dates of the first three quarters of 2018, from Kroger's quarterly reports for fiscal 2018.
