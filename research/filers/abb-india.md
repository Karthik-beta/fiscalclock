# ABB India Limited, financial year 2025 (calendar year, January to December)

<!-- cspell:words BRSR -->

ABB India's financial year is the calendar year, 1 January to 31 December, and spec version 1 can express it.

Last reviewed: 4 Nov 2026

## Document

| Item | Value |
| --- | --- |
| Company | ABB India Limited, listed on BSE and NSE |
| Title | ABB India Limited Integrated Annual Report 2025 |
| Form | Integrated annual report with the notice of the 76th AGM, Boards' Report, corporate governance report, management discussion, BRSR and audited financial statements |
| Financial year | 2025, ended 31 December 2025 |
| URL | <https://library.e.abb.com/public/93c9bda6800643c789fc96fbb964ae1b/ABB%20India%20Limited%20Integrated%20Annual%20Report%202025.pdf> |
| Host | ABB India's own page `new.abb.com/indian-subcontinent/investors/abb-india-limited-integrated-annual-report-2025` has a "Download here" link (<https://to.abb/nOTyPCwY3>). It redirects to ABB's document library page `search.abb.com/library/Download.aspx?DocumentID=9AKK108472A5046`, titled "ABB India Limited Integrated Annual Report 2025", which embeds this PDF from the library host `library.e.abb.com` (observed in a browser on 3 Nov 2026). The address that the viewer page sends the browser to carries a signed token (`x-sign`) and may stop working; the URL recorded above is the same file without a query string. Reproduce on 4 Nov 2026: `curl -s -o /dev/null -D - -A "Mozilla/5.0" "https://search.abb.com/library/Download.aspx?DocumentID=9AKK108472A5046"` answered `302` with a `Location` of the recorded URL followed by `?x-sign=` and a token, and `curl -sI -A "Mozilla/5.0"` on the recorded URL answered `200` with `Content-Type: application/pdf`. The ABB India page above is the stable entry point. `curl` to `new.abb.com` timed out from this machine; the file itself downloaded with `curl -L -A "Mozilla/5.0"`. |
| Retrieved | 3 Nov 2026; 348 pages; SHA-256 `f33064f1169d5cde931e8a39a63dcc8e6f8b28785ab216d40ea020736c70090b` |
| Quotes | [`quotes/abb-india-2025.json`](quotes/abb-india-2025.json) |

## What the document says

### Financial year rule

> Company’s financial year is January – December.

Printed page 126 (PDF page 128), Corporate Governance Report, General shareholder information.

> This Report covers the financial and operational performance of ABB India Limited for the period January 01, 2025 to December 31, 2025, unless otherwise stated.

PDF page 7 (no printed page number found), Key elements: Reporting period and boundary.

> This Integrated Annual Report provides the most important information about the Company’s strategy, performance, governance and value creation of its operations for the financial year ended December 31, 2025.

PDF page 6 (no printed page number found), About the report.

> Your Directors have the pleasure in presenting their Seventy Sixth Annual Report together with Audited Financial Statements for the financial year ended December 31, 2025.

Printed page 96 (PDF page 98), Boards' Report.

> These standalone financial statements for the year ended December 31, 2025 are prepared in accordance with Indian Accounting Standards (Ind AS)

Printed page 226 (PDF page 228), Notes to the standalone financial statements, Note 2.1.

The Secretarial Audit paragraph also gives the span of a financial year as a date range:

> to undertake the Secretarial Audit of the Company for (five) years from Financial Year January 1, 2025 to December 31, 2029.

Printed page 100 (PDF page 102), Boards' Report, Secretarial audit.

### Year naming

> During CY 2025, ABB India delivered another solid year, surpassing its already strong CY 2024 performance.

Printed page 96 (PDF page 98), Boards' Report, Review of operations.

> The Committee has held four meetings during the Financial Year 2025 i.e., February 17, May 09, August 02 and November 06, 2025.

Printed page 120 (PDF page 122), Corporate Governance Report, Audit Committee.

> Ratification of remuneration to Cost Auditor of the Company for financial year ending December 31, 2026

Printed page 86 (PDF page 88), Notice of the 76th AGM, Item No. 4.

> Q4 CY2025 alone saw a 52% surge in orders, the highest fourth‑quarter intake in five years

Printed page 96 (PDF page 98), Boards' Report, Review of operations.

The document names a year in three ways: "Financial Year 2025" (the reporting year), "CY 2025" (the same year, in review of operations) and "Year 2025" (in the calendar of events). All use the calendar year in which the year both starts and ends, so a year number is unambiguous.

### Quarters

> Fourth Quarter Financial Results (Year 2025) February 2026 First Quarter Financial Results May 2026 Annual General Meeting May 2026 Second Quarter Financial Results July 2026 Third Quarter Financial Results November 2026

Printed page 126 (PDF page 128), Corporate Governance Report, General shareholder information.

The Corporate Governance Report names quarters by ordinal ("Fourth Quarter Financial Results (Year 2025)") and gives the months in which results are announced, not the quarter end dates. The Boards' Report uses "Q4 CY2025" and speaks of a fourth quarter. The document does not print any quarter end date.

### Reported dates

| What | Dates | Where |
| --- | --- | --- |
| Financial year 2025 | 1 January 2025 to 31 December 2025 | Printed pages 96, 126; PDF pages 6 and 7 (front matter, no printed number found) |
| Financial year 2024 | ended 31 December 2024 | Printed page 225 (footnote), and the comparative column of the balance sheet and profit and loss statement |
| Financial year 2026 | ending 31 December 2026 | Printed page 86 |
| Secretarial audit term | 1 January 2025 to 31 December 2029, five years | Printed page 100 |
| Fourth-quarter results (year 2025) announced | February 2026 | Printed page 126 |
| First, second and third quarter results announced | May, July and November 2026 | Printed page 126 |

## Spec version 1 definition

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 1, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {
    "default": "cy",
    "schemes": {
      "cy": {"year_number": "start", "templates": {"year": "CY {yyyy}", "quarter": "Q{q} CY{yyyy}"}},
      "fy": {"year_number": "start", "templates": {"year": "Financial Year {yyyy}"}}
    }
  }
}
```

| Field | Value | Basis |
| --- | --- | --- |
| `year.type`, `start` | `fixed_date`; 1 January | Stated by the filing ("Company’s financial year is January – December."; "for the period January 01, 2025 to December 31, 2025") |
| `week` | `monday`, `contains_start` | Illustrative. The document does not describe weeks. |
| `subdivision.type`, `month_start_day` | `calendar_months`; 1 | Illustrative. The document does not describe fiscal months. |
| `subdivision.quarters` | 3, 3, 3, 3 | Illustrative. The document names four quarters ("First Quarter" to "Fourth Quarter", "Q4 CY2025") but prints no quarter end dates. |
| `year_number` | `start` | Stated by the filing ("During CY 2025" for the year from 1 January to 31 December 2025). `start`, `end` and `majority` give the same number for a calendar year. |
| `cy` year template | `CY {yyyy}` | Stated by the filing ("CY 2025") |
| `cy` quarter template | `Q{q} CY{yyyy}` | Stated by the filing in one place only ("Q4 CY2025"); the spacing differs from "CY 2025" |
| `fy` year template | `Financial Year {yyyy}` | Stated by the filing ("the Financial Year 2025") |
| Default scheme | `cy` | Illustrative. The document uses "Financial Year 2025" in governance text and "CY 2025" in review of operations. |

## Conformance vector

`conformance/vectors/cited/abb-india-integrated-annual-report-2025.json`, from `conformance/generators/spec_cited_vectors.py`, has 6 cases: validation, the start and last day of 2025, the last day of 2026, the labels `CY 2025` and `Q4 CY2025`, and parsing `Q4 CY2025`.

## Still to check

1. Quarter end dates are not printed in the annual report. Obtain ABB India's quarterly results filings from its own investor page (for example the "Reg33 Quarterly Results" PDFs on `new.abb.com/indian-subcontinent/investors/financial-results-and-presentations/quarterly-results-and-annual-reports-2025`) and quote the quarter end dates. Not done here; they are separate documents.
2. The printed page numbers of PDF pages 6 and 7 (front matter) were not found, so `printed_page` is null for those quotes. Printed page numbers elsewhere are PDF page minus 2.
3. The document gives no history of ABB India's financial year (whether it has ever been different). Only 2024, 2025 and 2026 are covered.
4. The `library.e.abb.com` address carries no query string and downloaded without a session on 3 Nov 2026, but ABB does not say it is permanent. If it stops resolving, take the current link from the company's annual-report page and compare the SHA-256 above.
5. `new.abb.com` refused automated requests on 3 Nov 2026 (`curl`, Python `urllib` and lychee could not connect; a browser could), so its addresses on this page are written as code, not links.
