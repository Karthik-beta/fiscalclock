# Thomas Cook (India) transition period, 2014 to 2015

<!-- cspell:words ﬁnancial ﬁfteen Proﬁt Quess -->

Thomas Cook (India) Limited's Annual Report 2014-2015 states a fifteen-month period from 1 January 2014 to 31 March 2015, after which its years run from 1 April to 31 March, but the report does not say who approved the change, and spec version 1 cannot express the 15-month period.

Last reviewed: 3 Nov 2026

## Document

| Item | Value |
| --- | --- |
| Title | Annual Report 2014-2015 (the Directors' Report calls it "the Thirty – Eighth Annual Report") |
| Company | Thomas Cook (India) Limited (listed; the report names BSE Limited and the National Stock Exchange of India Limited) |
| Period covered | 1 January 2014 to 31 March 2015 (fifteen months) |
| URL | <https://resources.thomascook.in/downloads/Thomas%20Cook%20(India)%20Limited%20-%20Annual_Report_2014.pdf> |
| Host | Listed under "2014-15" on the company's own page <https://www.thomascook.in/annual-report> (fetched 3 Nov 2026). The file is on `resources.thomascook.in`, which the company's investor pages link to for its documents. |
| Retrieved | 3 Nov 2026; 156 pages; SHA-256 `85584b46b2e646f9e2f02ec167bc1e9b8de5fd917d08d1939d46666c279dc96e` |
| Quotes | [`quotes/thomas-cook-india-2014-15.json`](quotes/thomas-cook-india-2014-15.json) |

The evidence register quotes a news report that the company "could have a 15-month period from January 2015 to March 2016" ([Evidence.md](../evidence/Evidence.md#india-erp-and-transition-years)). The company's own report gives a different period: the 15 months run from 1 January 2014 to 31 March 2015, and 2015-16 is the first regular April to March year.

## What the document says

### The change and the transition period

> Your Directors have pleasure in presenting the Thirty – Eighth Annual Report, together with audited ﬁnancial statements for the ﬁfteen months period ended 31st March, 2015 (from 1st January, 2014 to 31st March, 2015).

Printed page 1, Directors' Report.

> \# The Company extended its ﬁnancial year to bring it in line with the requirements of the provisions of sub-section (41) of Section 2 of the Companies Act, 2013. The current ﬁnancial period is for the period of ﬁfteen months from 1st January, 2014 to 31st March, 2015 and therefore not comparable with the previous year.

Printed page 1, footnote to the financial performance table. The text is printed with "ﬁ" as a single ligature character, which is kept here.

> The subsequent ﬁnancial years of the Company will commence from 1st day of April every year and end on 31st March of the following year.

Printed page 1, next paragraph.

### Approval

The report does not state an approval of the change by the board, the shareholders or a tribunal. The only reason it gives is "to bring it in line with the requirements of the provisions of sub-section (41) of Section 2 of the Companies Act, 2013" (quoted above). Searches of the extracted text for "Regional Director", "Registrar", "Company Law Board" and "National Company" found no approval of a change in financial year. This record therefore has no approval wording to quote.

### How the transition period is labelled

> We have reviewed the financial statements and the cash flow statement for the fifteen months period ended 31st March, 2015 (01.01.2014 to 31.03.2015)

Printed page 49, CEO/CFO Certification.

> Statement of Proﬁt and Loss for the ﬁfteen months period ended March 31, 2015

Printed page 53, heading of the standalone Statement of Profit and Loss.

> Note: All the above figures are compared for the period of 15 Months i.e. January 2014 to March 2015 versus previous year of 12 months i.e. January 2013 to December 2013.

Printed page 19, Management Discussion and Analysis Report.

> ** 2014-15 is for ﬁfteen months.

Printed page 124, Consolidated Performance Summary.

> Annual Report 2014-2015

Inside front cover (no printed number). The 15-month period carries the label "2014-15" (or "2014-2015") in the report. The text calls it "the current ﬁnancial period", "fifteen months period" and "15 Months".

### The year before and the year after

> the consolidated ﬁnancial statements for the year ended 31st December, 2013

Printed page 1. The old year is a calendar year labelled by its end date. The standalone Statement of Profit and Loss (printed page 53) also heads its comparative column "Year ended December 31, 2013", which is not quoted in [`quotes/thomas-cook-india-2014-15.json`](quotes/thomas-cook-india-2014-15.json). (The quoted span is from a sentence about Quess Corp Limited's consolidation and is used here only for the year-end date.)

> Financial Years 2014-15 and 2015-2016

Printed page 6, in the sentence about listing fees paid to the stock exchanges. It names the 15-month period ("2014-15") and the next year ("2015-2016") together as financial years.

### Quarters

> for the quarter ended 31st December, 2014

Printed page 6, in a corporate governance remark. This is the only quarter end in the report that falls inside the transition period, and it is incidental. The report gives no quarter structure for the 15 months.

### Reported dates

| What | Date | Where |
| --- | --- | --- |
| Old year | 1 January 2013 to 31 December 2013 | Pages 1 and 19 |
| Transition period | 1 January 2014 to 31 March 2015, 15 months | Pages 1, 19, 49 |
| New year | 1 April to 31 March, from 1 April 2015 | Page 1 |
| Quarter end inside the transition period | 31 December 2014 | Page 6 |

The 455-day length is arithmetic on those dates (365 days in 2014, plus 31, 28 and 31 for January to March 2015), not a figure printed in the report.

## Spec version 1

Spec version 1 cannot express the transition period. A `fixed_date` year starts on the same month and day every year ([spec/definition-v1.md](../../spec/definition-v1.md), "`type: "fixed_date"`"), so no single definition has a year from 1 January 2014 to 31 March 2015 and also regular years from 1 April. A transition year needs a later spec version (planned for 0.2). Below are one definition for the years before it and one for the years after it. Neither covers 1 January 2014 to 31 March 2015.

A naming problem remains. The report labels the 15-month period "2014-15". Under the April definition below, numbered by start year, "2014-15" would mean 1 April 2014 to 31 March 2015, which overlaps the last 12 months of the transition period.

Before (calendar years, last regular one is 2013):

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 1, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start", "templates": {"year": "{yyyy}", "quarter": "Q{q} {yyyy}"}}}}
}
```

After (years from 1 April 2015):

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 4, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start", "templates": {"year": "{yyyy}-{yy_next}", "quarter": "Q{q} {yyyy}-{yy_next}"}}}}
}
```

| Field | Value | Basis |
| --- | --- | --- |
| Before: `year.start` | 1 January | Stated by the filing ("the year ended 31st December, 2013"; "January 2013 to December 2013") |
| After: `year.start` | 1 April | Stated by the filing ("commence from 1st day of April every year and end on 31st March of the following year") |
| Transition period | 1 January 2014 to 31 March 2015 | Stated by the filing; not expressible in spec version 1 |
| `year_number` | `start` | Illustrative. The report labels the April to March period "2014-15", which fits the start year for a regular year, but its only "2014-15" is the 15-month period. |
| After: year template | `{yyyy}-{yy_next}` | Stated by the filing for the digits ("Financial Years 2014-15 and 2015-2016"). The report writes the second year both as "15" and as "2016", so the two-digit form is illustrative. |
| Before: year template | `{yyyy}` | Illustrative. The old year is described by its end date ("year ended 31st December, 2013"); the report's table column header "2013" was not quoted. |
| Quarter templates | `Q{q} …` | Illustrative. The report gives no quarter label. |
| `subdivision` | calendar months, quarters 3, 3, 3, 3 | Illustrative. The report states no quarter structure. |
| `week` | `monday`, `contains_start` | Illustrative. The report does not describe weeks. The block is required by the spec. |

## Still to check

1. Who approved the change. The Companies Act, 2013 requirement cited at "sub-section (41) of Section 2" may come with an Annual Report 2015-16 sentence, a stock exchange board-outcome notice, or a Registrar's or Regional Director's order. Look in the Annual Report 2015-16 on the company page (`https://resources.thomascook.in/downloads/ThomasCookIndiaLtdAnnulReport2015-16.pdf`, listed there but not downloaded) and in the company's own filings. A search result suggested the March 2016 results notes describe the change, but that was not checked against a company document.
2. The company's label for the comparative column of the old year in the tables (the quoted span only gives "year ended 31st December, 2013").
3. Quarter boundaries for the 15 months. The report gives none; the quarterly results for 31 December 2014 and 31 March 2015 may.
4. The two-digit label form for regular years ("2015-16"), from the 2015-16 report.
