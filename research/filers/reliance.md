# Reliance Industries Limited, financial year 2024-25

<!-- cspell:words EBITDA -->

Reliance Industries' financial year runs from 1 April to 31 March, it labels the year ended 31 March 2025 "FY 2024-25", and spec version 1 can express it.

Last reviewed: 3 Nov 2026

## Document

| Item | Value |
| --- | --- |
| Title | Integrated Annual Report 2024-25 (the Board's Report calls it "the Company’s Forty-eighth Annual Report (Post-IPO)") |
| Form | Integrated annual report. It holds the Management Discussion and Analysis, Corporate Governance Report, Board's Report, auditor's reports and the standalone and consolidated financial statements. |
| Fiscal year | 2024-25, ended 31 March 2025 |
| URL | <https://www.ril.com/ar2024-25/pdf/RIL-IAR-2025.pdf> |
| Host | The company's own domain, `www.ril.com`. The landing page <https://www.ril.com/ar2024-25/index.html> links to `pdf/RIL-IAR-2025.pdf` (observed on 3 Nov 2026). |
| Retrieved | 3 Nov 2026; 147 PDF pages; SHA-256 `bc18c9a8579a5baf35b11fc089baec489c962e08a6d812824f8a88859b4747a2` |
| Page numbers | Each PDF page is a two-page spread. The printed page numbers below are the page of the spread the text sits on (checked by its position on the page). [`quotes/reliance-2024-25.json`](quotes/reliance-2024-25.json) gives both the PDF page and the printed page. |

## What the document says

### The financial year

> Financial Year April 1 to March 31

Printed page 58, Corporate Governance Report, "General Shareholder Information". This is a labelled field in a table; the label and the value are the whole sentence.

> The Board of Directors present the Company’s Forty-eighth Annual Report (Post-IPO) and the Company’s audited financial statements for the financial year ended March 31, 2025.

Printed page 76, Board's Report.

> Note: All Revenue and EBITDA figures are for the year ended March 31, 2025

Printed page 2, "Reliance at a Glance".

### Year naming

The report labels a year by the two calendar years it touches, a four-digit start year and a two-digit end year, and it places the label next to the dates in the statements:

> The Company’s financial performance (standalone and consolidated) for the year ended March 31, 2025 is summarised below: Standalone Consolidated 2024-25 2023-24 2024-25 2023-24

Printed page 76, Board's Report, "Financial Results". The column headers "2024-25" and "2023-24" sit under the sentence that says the year ended 31 March 2025.

> For the year ended 31st March, 2025 A. Equity Share Capital Balance as at 1st April, 2023 6,766 Change during the year 2023-24 -* Balance as at 31st March, 2024 6,766 Change during the year 2024-25** 6,766

Printed page 114, Standalone Financial Statements, "Statement of Changes in Equity". The year "2023-24" opens on 1 April 2023 and closes on 31 March 2024.

> B. Other Equity Balance as at 1st April, 2024 Total Comprehensive Income for the Year

Printed page 114, the same statement. The opening balance of the year "2024-25" is at 1 April 2024.

> Consolidated Statement of Profit and Loss For the year ended 31st March, 2025 Income Notes 2024-25

Printed page 196, Consolidated Financial Statements.

The prefix "FY" with a space and the full two-year form appears in running text:

> There has been no revision in credit ratings during the FY 2024-25.

Printed page 58, Corporate Governance Report, "Credit Rating".

> Annual listing fees for the FY 2025-26 has been paid by the Company to BSE Limited and National Stock Exchange of India Limited.

Printed page 58, "Payment of Listing Fees". "FY 2025-26" is the year that starts on 1 April 2025 (see the quarter calendar below, which ends on 31 March 2026).

The report also uses a short form, and it names calendar years separately:

> Revenue of Oil to Chemicals for FY25 increased by 11 % Y-o-Y

Printed page 77, Board's Report. "FY25" most likely means FY 2024-25, because the Board's Report is about the year ended 31 March 2025. That is an inference from the context, not a sentence in the document.

> Global economic growth remained healthy at 3.3% in CY24 consistent with the prior year.

Printed page 7, Management Discussion and Analysis. "CY24" is a calendar year, which the report keeps distinct from "FY".

### Quarters

> Financial Calendar (Tentative) Results for the quarter ending June 30, 2025 - Third week of July, 2025; September 30, 2025 - Third week of October, 2025; December 31, 2025 - Third week of January, 2026; and March 31, 2026 - Fourth week of April, 2026.

Printed page 58, Corporate Governance Report, "General Shareholder Information". The dates are the ends of the four quarters of FY 2025-26. "Tentative" qualifies when results come out. The report does not number the quarters ("Q1" to "Q4") and does not print the quarter dates of FY 2024-25.

### Change of financial year

The document says nothing about a change of financial year. A search of the extracted text for "transition" found only energy-transition and Ind AS-transition sentences.

### Reported dates

| What | Date | Where |
| --- | --- | --- |
| Financial year | 1 April to 31 March | Printed page 58 |
| End of FY 2024-25 | 31 March 2025 | Printed pages 2, 76, 114, 196 |
| Opening of FY 2024-25 | 1 April 2024 | Printed page 114 |
| Opening and end of FY 2023-24 | 1 April 2023 and 31 March 2024 | Printed page 114 |
| Quarter ends of FY 2025-26 | 30 June 2025, 30 September 2025, 31 December 2025, 31 March 2026 | Printed page 58 |

## Spec version 1 definition

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 4, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start", "templates": {"year": "FY {yyyy}-{yy_next}", "quarter": "Q{q} FY {yyyy}-{yy_next}"}}}}
}
```

| Field | Value | Basis |
| --- | --- | --- |
| `year.type`, `start` | `fixed_date`, 1 April | Stated by the filing ("Financial Year April 1 to March 31") |
| `week.starts_on`, `numbering` | `monday`, `contains_start` | Illustrative. The document does not define weeks; spec version 1 requires a `week` block. |
| `subdivision.type`, `month_start_day` | `calendar_months`, 1 | Illustrative. The document does not name fiscal months; calendar months follow from quarters that end on 30 June, 30 September, 31 December and 31 March. |
| `subdivision.quarters` | 3, 3, 3, 3 | Stated by the filing ("quarter ending June 30, 2025 ... September 30, 2025 ... December 31, 2025 ... March 31, 2026") |
| `year_number` | `start` | Stated by the filing ("Change during the year 2023-24" between "1st April, 2023" and "31st March, 2024") |
| `templates.year` | `FY {yyyy}-{yy_next}` | Stated by the filing ("FY 2024-25") |
| `templates.quarter` | `Q{q} FY {yyyy}-{yy_next}` | Illustrative. The report names quarters by their end date, not by number. |

Nothing in this document falls outside spec version 1.

## Conformance vector

`conformance/vectors/cited/reliance-annual-report-2024-25.json`, from `conformance/generators/spec_cited_vectors.py`, has 11 cases: validation, the start and last day of 2023-24 and 2024-25, the last day of 2025-26 and of each of its quarters, the labels `FY 2024-25` and `FY 2025-26`, and parsing `FY 2024-25`.

## Still to check

- The document does not number or name quarters. A Reliance quarterly results release for a quarter of FY 2024-25 (for example "quarter ended December 31, 2024") would show how the company labels quarters ("Q3 FY25" or similar). It is not in this PDF.
- The document does not say why the year is April to March. That is not needed for the definition.
- The date the report was approved or first published is not recorded here. The Board's Report and certifications carry dates in April 2025; they were not quoted.
- Some subsidiaries or associates may have a different year end. This report's consolidation notes were not searched for them.
- The same report is also published as separate PDFs on `ril.com` (Board's Report, standalone and consolidated statements). They were not used.
