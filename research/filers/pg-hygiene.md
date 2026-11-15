# Procter & Gamble Hygiene and Health Care

<!-- cspell:words vide Contentful Ashwin Solanki -->

Procter & Gamble Hygiene and Health Care Limited's financial year ran 1 July to 30 June up to the year ended 30 June 2024, then changed by Board resolution to 1 April to 31 March through a nine-month year from 1 July 2024 to 31 March 2025. From fiscalclock 0.2.0, one definition with `exceptions.history` expresses both regimes and the change between them.

Last reviewed: 15 Nov 2026

## Document

Three annual reports, all downloaded from the company's own investor pages on 3 Nov 2026. The transition report is the primary record. The other two show the calendar before and after the change.

| Item | Transition report (primary) | Latest report | Last July to June report |
| --- | --- | --- | --- |
| Title | Annual Report 2024-25 (no cover title in the extracted text; the running footer reads "Annual Report 2024-25") | Annual Report 2025-26 (cover) | Annual Report 2023-24 (running footer) |
| Period covered | Nine months, 1 July 2024 to 31 March 2025 | 1 April 2025 to 31 March 2026 | 1 July 2023 to 30 June 2024 |
| URL | <https://downloads.ctfassets.net/oe48y40ukei6/1u4fKj5iFk4bevuObG5vJd/f5a65c7f94bab8a53fe33769070ac956/Annual_Report_2025.pdf> | <https://assets.ctfassets.net/oe48y40ukei6/3NtnlryjO6oVyBMEES6pLT/d4aee2a14d8cab5a4d777d8e5f8eba8a/ANNUAL_REPORT_2026__1_.pdf> | <https://assets.ctfassets.net/oe48y40ukei6/7aXOHTgxcGULOBM2hYlblW/6ef76eff5f3da3ecad126944fbaf3391/PGHH_ANNUAL_REPORT_2024.pdf> |
| Pages and SHA-256 | 164 pages; `28ec4d56637da6b97e4049e09772afa971fba4ce88b0457e91e3b5cf836c40cd` | 156 pages; `d5d8b8475cec3a2e852433074f2e21d067e9a938c88e789456b814178f87a0b3` | 160 pages; `c2ac2b0aee19672f233b78a6e054d55ce4bf858fd681590d99890cd71be667cd` |
| Manifest | [`quotes/pg-hygiene-2024-25.json`](quotes/pg-hygiene-2024-25.json) | [`quotes/pg-hygiene-2025-26.json`](quotes/pg-hygiene-2025-26.json) | [`quotes/pg-hygiene-2023-24.json`](quotes/pg-hygiene-2023-24.json) |

Host: the company's own investor page `https://in.pg.com/india-investors/pghh/shareholder-info/info/` lists all three under "Annual Report & Returns" as "ANNUAL REPORT 2025", "ANNUAL REPORT 2026" and "ANNUAL REPORT 2024". The files sit on the company's Contentful content network (`downloads.ctfassets.net` and `assets.ctfassets.net`), the same network its other investor pages link to. Retrieved 3 Nov 2026 with `curl -L -A "Mozilla/5.0"` (HTTP 200 each). The investor home page `https://in.pg.com/india-investors/pghh/` returned HTTP 403 to the same request, so the report links were read from the "shareholder-info/info" page instead. Page counts are the number of form feeds in the `pdftotext` output, because `pdfinfo` was not installed.

## What the document says

### The financial year before the change (July to June)

> The Company’s Financial Year is July 1st to June 30th.

Printed page 8 (PDF page 10), Report of the Directors, Financial Year.

> Presently, the Company follows July-June Financial Year.

Printed page 71 (PDF page 73), Corporate Governance Report, Financial Calendar.

> Financial Year reported July 1, 2023 to June 30, 2024

Printed page 19 (PDF page 21), Business Responsibility & Sustainability Report, General Disclosures.

> The Board of Directors have the pleasure of presenting the 60th Annual Report including the Audited Financial Statements of the Company for the Financial Year ended June 30, 2024.

Printed page 8 (PDF page 10), Report of the Directors.

The 2024 report is the 60th annual report. Its statement of profit and loss is headed "STATEMENT OF PROFIT AND LOSS FOR THE YEAR ENDED JUNE 30, 2024" (printed page 95).

### The change, and its approval

> The Board of Directors of the Company, on January 23, 2025, approved the change in the Financial Year of the Company from “July 1 – June 30” period to “April 1 – March 31” period. Consequently, the Financial Year of the Company for the period under review, viz., 2024-25, is a period of 9 months commencing on July 1, 2024, and ending on March 31, 2025. Subsequent financial years of the Company shall commence on April 1 every year and end on March 31 of the succeeding year.

Printed page 10 (PDF page 12), Directors' Report, Financial Year.

> Effective this year, the Company changed its Financial Year from July 1 – June 30, to April 1 – March 31. Accordingly, the current Financial Year of the Company ended on March 31, 2025, covering a period of 9 months from July 1, 2024, to March 31, 2025.

Printed page 6 (PDF page 8), Managing Director's Statement.

The approval is described as a Board resolution in every place that states one:

> The Board of Directors of the Company have, vide a circular resolution dated January 23, 2025, resolved to change the financial year end from June 30 to March 31.

Printed page 86 (PDF page 88), Independent Auditors' Report, Other Matter.

> The Board of Directors of the Company, on January 23, 2025, have approved the change of the Financial Year end from June 30 to March 31 of each year. Accordingly, the current Financial Statements of the Company are for a period of nine months commencing on July 1, 2024 and ending on March 31, 2025.

Printed page 102 (PDF page 104), Notes to Financial Statements, note 2.2.

> Accordingly, the Ind-AS financial statements for the current financial year of the Company as per the provisions of section 2(41) of the Companies Act, 2013, is for a period of nine months from July 1, 2024, to March 31, 2025.

Printed page 86 (PDF page 88), Independent Auditors' Report, Other Matter.

The auditors' report cites section 2(41) of the Companies Act, 2013, for the length of the period. The 2024-25 and 2025-26 reports quote no approval, exemption or permission from a government authority, a tribunal or a stock exchange for the change (searched for "exempt", "Tribunal", "Regional Director" and "Ministry of Corporate Affairs"; the hits are unrelated). They say only that the Board approved it.

### How the company names its years

The transition year is "2024-25", the number of the calendar year in which it began, then the last two digits of the next:

> *Financial Year 2024-25 is a 9 month period from July 1, 2024 to March 31, 2025, and hence, the figures are not comparable with the previous financial year which is a 12 month period.

Printed page 10 (PDF page 12), Directors' Report, Financial Year.

> The Company, vide approval of its Board of Directors dated January 23, 2025, changed its Financial Year from July 1- June 30 period to April 1- March 31 period. Accordingly, the Company’s financial year 2024-25 was a nine month period from July 1, 2024 and ending March 31, 2025.

Printed page 55 (PDF page 57), Corporate Social Responsibility Report, note.

> Financial Year reported July 1, 2024 to March 31, 2025

Printed page 21 (PDF page 23), Business Responsibility & Sustainability Report, General Disclosures.

Statements and report titles follow the dates ("STATEMENT OF PROFIT AND LOSS FOR THE NINE MONTH PERIOD ENDED MARCH 31, 2025", printed page 97).

The year that began on 1 April 2025 is "2025-26":

> The Board of Directors of the Company, on January 23, 2025, had approved the change in Financial Year of the Company from “July 1 – June 30” period to “April 1 – March 31” period. The Financial Year of the Company for period under review, viz., 2025-26, commenced on April 1, 2025, and ended on March 31, 2026.

Printed page 9 (PDF page 11), Directors' Report, Financial Year.

> *Previous Financial Year 2024-25 was a 9-month period from July 1, 2024, to March 31, 2025, and hence not comparable with current Financial Year 2025-26 (being 12-month Financial Year from April 1, 2025, to March 31, 2026).

Printed page 9 (PDF page 11), Directors' Report, Financial Results.

The following year is "2026-27":

> re-appointed M/s. Ashwin Solanki & Associates, as the Cost Auditors of the Company for the financial year 2026-27.

Printed page 18 (PDF page 20), Directors' Report, Cost Auditors.

> to audit the cost records of the Company for the financial year ending March 31, 2027, is being placed for the approval of the shareholders of the Company

Printed page 18 (PDF page 20), Directors' Report, Cost Auditors.

The year before the transition is labelled "2023-24" in the Directors' Report table of the 2024-25 report (the column next to "2024-25*", on printed page 10), and the text beside that table says "the numbers are not comparable to the previous financial year, which was a twelve months period (July 1, 2023 to June 30, 2024).". The company never prints the label and these dates in one sentence, so that pairing is inferred from adjacency. The 2024 report is titled "Annual Report 2023-24" in its running footers and states "The Board of Directors have the pleasure of presenting the 60th Annual Report including the Audited Financial Statements of the Company for the Financial Year ended June 30, 2024." (printed page 8).

### Stated rule going forward

> The Financial Year of the Company has been changed from “July 1st – June 30th” period to “April 1st – March 31st” period. Accordingly, the Financial Year of the Company for purpose of this report viz., 2024-25 is a period of 9 months commencing on July 1, 2024, and ending on March 31, 2025.

Printed page 73 (PDF page 75), Corporate Governance Report, Financial Calendar.

> The Financial Year of the Company has been changed from "July 1st – June 30th” period to “April 1st – March 31st” period, effective Financial Year 2024-25. Accordingly, the Financial Year of the Company for purpose of this report viz., 2025-26 was April 1, 2025 to March 31, 2026.

Printed page 68 (PDF page 70), Corporate Governance Report, Financial Results.

### Quarters

None of the three annual reports prints the first or last day of a quarter (searched each extracted text for "quarter"; the only hits are generic mentions such as board meetings "at least once a quarter").

## Reported dates

| What | Dates | Where |
| --- | --- | --- |
| Financial year 2023-24 (12 months; label inferred) | 1 July 2023 to 30 June 2024 | 2024-25 report, printed page 10; 2024 report, printed page 19 |
| Board approval of the change | 23 January 2025 | 2024-25 report, printed page 10; auditors' report, printed page 86 |
| Financial year 2024-25 (9 months) | 1 July 2024 to 31 March 2025 | 2024-25 report, printed page 10, printed page 6, printed page 102 and printed page 21 |
| Financial year 2025-26 (12 months) | 1 April 2025 to 31 March 2026 | 2025-26 report, printed page 9, printed page 9 and printed page 68 |
| Financial year 2026-27 | Ends 31 March 2027 (start not printed) | 2025-26 report, printed page 18 |

A typographical error in the 2024-25 Directors' Report reads "have been prepared for the nine months period from July 1, 2025 to March 31, 2025." (printed page 10). The start date there cannot be 2025, since the same section, the Managing Director's statement, the notes and the auditors' report all give 1 July 2024. Record it as the document's own error and do not use it.

## Definition with the change (fiscalclock 0.2.0)

fiscalclock 0.2.0 adds `exceptions.history` to spec version 1 ([spec/definition-v1.md](../../spec/definition-v1.md#exceptions)): a history entry starts a new year rule on its `from` date, after an optional transition year. The spec uses this calendar as its worked example.

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 7, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start", "templates": {"year": "{yyyy}-{yy_end}"}}}},
  "exceptions": {
    "history": [
      {
        "from": "2025-04-01",
        "transition": {
          "start": "2024-07-01",
          "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3]}
        },
        "year": {"type": "fixed_date", "start": {"month": 4, "day": 1}},
        "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]}
      }
    ]
  }
}
```

| Field | Value | Basis |
| --- | --- | --- |
| `year.start` (top level) | 1 July | Stated by the filing ("The Company’s Financial Year is July 1st to June 30th.") |
| `exceptions.history[0].transition.start` | 2024-07-01 | Stated by the filing ("2024-25, is a period of 9 months commencing on July 1, 2024, and ending on March 31, 2025") |
| `exceptions.history[0].from`, `year.start` | 2025-04-01; 1 April | Stated by the filing ("Subsequent financial years of the Company shall commence on April 1 every year and end on March 31 of the succeeding year") |
| `transition.subdivision` | calendar months, quarters 3, 3, 3 | Illustrative. No report prints a quarter end. |
| `subdivision` (both regimes) | calendar months, quarters 3, 3, 3, 3 | Illustrative |
| `week` | `monday`, `contains_start` | Illustrative. The reports do not number weeks and the field is required. |
| `year_number` | `start` | Stated by the filing: "2024-25" for the year beginning 1 July 2024 and "2025-26" for the year beginning 1 April 2025 |
| `templates.year` | `{yyyy}-{yy_end}` | Stated by the filing for the transition ("2024-25") and for 2025-26. `{yy_end}` writes the last two digits of the calendar year in which the fiscal year ends, which for the transition is 2025. The label "2023-24" that it gives the year before is inferred (see "How the company names its years"). |

The transition has 274 days, by calendar arithmetic on the stated dates (`date(2025, 4, 1) - date(2024, 7, 1)` in Python); the reports say "9 months" and do not print a day count.

## Spec version 1 definitions of each regime (before 0.2.0)

Before fiscalclock 0.2.0, `exceptions` was reserved, and a `fixed_date` year has one start date for every fiscal year. Each regime below is a separate calendar valid only for the years the document states. The cited vectors for these two definitions predate 0.2.0 and are kept.

### From financial year 2025-26: April to March

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 4, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start", "templates": {"year": "{yyyy}-{yy_next}"}}}}
}
```

| Field | Value | Basis |
| --- | --- | --- |
| `year.type`, `start` | `fixed_date`, 1 April | Stated by the filing ("Subsequent financial years of the Company shall commence on April 1 every year and end on March 31 of the succeeding year"; "2025-26, commenced on April 1, 2025, and ended on March 31, 2026") |
| `subdivision.type`, `month_start_day` | `calendar_months`, day 1 | Illustrative. The reports state no month or period structure; calendar months follow from a 1 April start. |
| `subdivision.quarters` | 3, 3, 3, 3 | Illustrative. No annual report prints a quarter end (see "Still to check"). |
| `week.starts_on`, `numbering` | `monday`, `contains_start` | Illustrative. The reports do not number weeks and the field is required. |
| `year_number` | `start` | Stated by the filing: "2025-26" for the year beginning 1 April 2025 and "2024-25" for the year beginning 1 July 2024 |
| `templates.year` | `{yyyy}-{yy_next}` | Stated by the filing: the label is printed as "2025-26" |

This definition is wrong for the year numbered 2024, which it would run from 1 April 2024 to 31 March 2025. The document says that year ran from 1 July 2024. No case below asserts the year 2024.

### Before the change: July to June (years up to 2023-24)

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 7, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start", "templates": {"year": "{yyyy}-{yy_next}"}}}}
}
```

| Field | Value | Basis |
| --- | --- | --- |
| `year.type`, `start` | `fixed_date`, 1 July | Stated by the filing ("The Company’s Financial Year is July 1st to June 30th.") |
| `subdivision`, `week` | As above | Illustrative |
| `year_number`, `templates.year` | `start`, `{yyyy}-{yy_next}` | Inferred. "2023-24" is the label beside the text "(July 1, 2023 to June 30, 2024)"; see "How the company names its years". |

## Conformance vectors

Three vectors from `conformance/generators/spec_cited_vectors.py`.

- `conformance/vectors/cited/pghh-annual-report-2024-25-transition.json` has 8 cases on the 0.2.0 definition above: validation; the start, last day and 274 days of the transition 2024-25; its label `2024-25` and parsing it; the year that contains 30 June 2024 (1 July 2023 to 30 June 2024, located by date because its label is inferred); `shift` from the transition one year back to that year and one year on to 2025-26 (1 April 2025 to 31 March 2026); and the label `2025-26`.
- `conformance/vectors/cited/pghh-annual-report-2025-26.json` has 6 cases on the April calendar alone: validation, the start and last day of 2025-26, the last day of 2026-27, the label `2025-26`, and parsing `2025-26` and `2026-27`.
- `conformance/vectors/cited/pghh-annual-report-2023-24.json` has 2 cases on the July calendar alone: validation and the start and last day of the year that began on 1 July 2023. It asserts no label, because the pairing of "2023-24" with those dates is inferred.

The spec's worked example (`conformance/vectors/spec/definition-exceptions-example.json`) and the oracle vector `conformance/vectors/oracle/history-jul-to-apr-9-month-transition.json` use the same dates with illustrative quarters and weeks.

## Still to check

- Quarter end dates. No annual report prints them. The company's compliance page `https://in.pg.com/india-investors/pghh/compliance-reports-and-announcements/compliance-report/` links quarterly results and press releases whose file names include "Quarter_ended_30062026" and "4th_Quarter_ended_30062024". Open them and quote the period each states, including the quarters of the nine-month year.
- The authority under which the change was made. The reports cite only the Board's resolution of 23 January 2025 and section 2(41) of the Companies Act, 2013. Read the section and any filing with the Registrar of Companies before stating that no further approval was needed.
- A single sentence that pairs the label "2023-24" with 1 July 2023 to 30 June 2024.
- Whether a pre-2024 report (for example 2022-23) names years the same way. Only 2023-24 was read.
