# Linde India transition period, 2022 to 2023

<!-- cspell:words vide -->

Linde India Limited changed its financial year end from 31 December to 31 March, and its Annual Report 2022-23 states a 15-month period from 1 January 2022 to 31 March 2023. From fiscalclock 0.2.0, one definition with `exceptions.history` expresses the years on either side and the period itself.

Last reviewed: 15 Nov 2026

## Document

| Item | Value |
| --- | --- |
| Title | Annual Report 2022-23, with the cover line "For 15 months period from 1 January 2022 to 31 March 2023" |
| Company | Linde India Limited (listed; the report names BSE and NSE) |
| Period covered | 1 January 2022 to 31 March 2023 (15 months) |
| URL | <https://assets.linde.com/-/media/global/apac/linde-india-limited/cp/linde-india_annual-report-2022-23_tcm526-678037.pdf> |
| Host | Listed as "Annual Report 2022-23 (15 months period)" on the company's own page <https://www.lindeindia.in/investors/financials/annual-reports> (fetched 3 Nov 2026). The file is on `assets.linde.com`, the CDN that page links to for every annual report. |
| Retrieved | 3 Nov 2026; 220 pages; SHA-256 `9de475a15d2b826f084a68e6fd4602ed3aa975ec4b18f288e7b53b00f03ecb5e` |
| Quotes | [`quotes/linde-india-2022-23.json`](quotes/linde-india-2022-23.json) |

## What the document says

### The change, who approved it, and the transition period

> The Board of Directors had at its meeting held on 14 November 2022, approved the change in Financial Year of the Company from existing “1 January to 31 December” to “1 April to 31 March” in order to avoid preparation of two sets of Audited Financial Statements - one as per the Calendar Year (Accounting Year) and the other as per the Financial Year (April - March) for Income Tax purposes, which was requiring a lot of management time and substantial efforts, at various levels every year. The Regional Director – Eastern Region, Ministry of Corporate Affairs had vide its Order dated 29 March 2023, approved the Company’s application for change in its Financial Year from Calendar Year (January – December) to uniform Financial Year (April – March). Consequently, the current Financial Year of the Company comprised of 15 months period from 1 January 2022 to 31 March 2023 and thereafter from 1 April every year to 31 March of the subsequent year.

Printed page 14, Directors' Report, "Change in Financial Year". The approval has two steps: the board decided on 14 November 2022, and the Regional Director of the Ministry of Corporate Affairs approved by an order dated 29 March 2023, two days before the period ended.

> We further report that during the period under review, the Board of Directors of the Company has approved the change in the financial year of the Company from January-December to April-March, subject to approval of the Hon’ble Regional Director, Eastern Region, Ministry of Corporate Affairs, which has been obtained vide Order dated 29 March 2023 and consequently, the financial year under review consists of a period of 15 months from 1 January 2022 to 31 March 2023.

Printed page 37, Secretarial Audit Report (the secretarial auditor's statement of the same facts).

> During the year under review, Regional Director - Eastern Region, MCA, approved our application to change our Financial Year from a calendar year to a uniform FY. The current FY will be for 15 months, from 1 January 2022 to 31 March 2023. Going forward, our FY will be from 1 April to 31 March.

Printed page 1, Chairman's Message.

### How the transition period is labelled

> The Directors have pleasure in submitting their Report together with the Audited Financial Statements of your Company for the 15 months period ended 31 March 2023 (1 January 2022 to 31 March 2023):

Printed page 13, Directors' Report and Management Discussion and Analysis.

> for the fifteen months period ended 31 March 2023

Printed page 83, heading of the Statement of Standalone Profit and Loss.

> For 15 months period from 1 January 2022 to 31 March 2023

Cover page (no printed number).

> Annual Report 2022-23

Printed page 1, page header, repeated on every page. The report's title is "2022-23", and the report covers the 15-month period, but no sentence or title in the checked text names both the label and the dates: the quote manifest records the title as a page header (PDF page 3, printed page 1) and the cover line "For 15 months period from 1 January 2022 to 31 March 2023" on PDF page 1, as separate texts. Under the project's evidence standard (a label is asserted only when one sentence, or one title that names the period, ties it to the period), the pairing of "2022-23" with 1 January 2022 to 31 March 2023 is therefore inferred. In the text the period is "the current Financial Year", "the 15 months period" and "the fifteen months period". The report gives it no "FY" label.

### The years on either side

> the previous financial year ended 31 December 2021 (FY 2021)

Printed page 13. A 12-month calendar year is labelled "FY 2021".

> Balance as at 1 January 2021 Changes in equity share capital during the year Balance as at 31 Dec 2021

Printed page 84, Statement of Standalone Changes in equity. The span is the first and last rows of a table in extraction order. It gives the opening and closing dates of the year ended 31 December 2021.

> Financial Calendar 2023-24 (tentative and subject to change)

Printed page 73, General Shareholder Information. The next table row in the document reads:

> i. Financial Year April 2023 to March 2024

Same page. This is the only place in the report where an April to March year appears near a label: "2023-24" is the heading of a tentative calendar and the dates are a row of it. The pairing comes from the table's layout, not from one sentence, so it is inferred.

### Reported dates

| What | Date | Where |
| --- | --- | --- |
| Old year | 1 January to 31 December (calendar year) | Page 14 |
| Last regular old-style year | 1 January 2021 to 31 December 2021, "FY 2021" | Pages 13 and 84 |
| Board approval | Meeting of 14 November 2022 | Page 14 |
| Regional Director's order | 29 March 2023 | Pages 14 and 37 |
| Transition period | 1 January 2022 to 31 March 2023, 15 months | Pages 1, 13, 14, 37 |
| New year | 1 April to 31 March, first one 1 April 2023 to 31 March 2024 (label "2023-24" inferred from a tentative calendar's heading) | Pages 1, 14 and 73 |

The 455-day length of the transition period is arithmetic on those dates (365 days in 2022, plus 31, 28 and 31 days for January to March 2023). It is not a figure printed in the report.

## Definition with the change (fiscalclock 0.2.0)

fiscalclock 0.2.0 adds `exceptions.history` to spec version 1 ([spec/definition-v1.md](../../spec/definition-v1.md#exceptions)): a history entry starts a new year rule on its `from` date, after an optional transition year. The `{yy_end}` placeholder writes the last two digits of the calendar year in which the fiscal year ends, so `{yyyy}-{yy_end}` gives "2022-23" for the 15-month period, numbered by its start year, and "2023-24" for the April to March year after it. These match the report's title and the heading of its tentative calendar, but both pairings are inferred (see "How the transition period is labelled"), so the `fy` scheme is illustrative and no vector asserts its labels. The report writes the calendar year before in another form, "FY 2021", stated in one sentence with the year's end ("the previous financial year ended 31 December 2021 (FY 2021)"), so the definition has a second scheme for it.

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 1, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {"default": "fy", "schemes": {
    "fy": {"year_number": "start", "templates": {"year": "{yyyy}-{yy_end}"}},
    "cy": {"year_number": "start", "templates": {"year": "FY {yyyy}"}}
  }},
  "exceptions": {
    "history": [
      {
        "from": "2023-04-01",
        "transition": {
          "start": "2022-01-01",
          "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3, 3]}
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
| `year.start` (top level) | 1 January | Stated by the filing ("from existing “1 January to 31 December”") |
| `exceptions.history[0].transition.start` | 2022-01-01 | Stated by the filing ("15 months period from 1 January 2022 to 31 March 2023") |
| `exceptions.history[0].from`, `year.start` | 2023-04-01; 1 April | Stated by the filing ("and thereafter from 1 April every year to 31 March of the subsequent year") |
| `transition.subdivision` | calendar months, quarters 3, 3, 3, 3, 3 | Illustrative. The report states no quarter structure for the period. |
| `subdivision` (both regimes) | calendar months, quarters 3, 3, 3, 3 | Illustrative |
| `week` | `monday`, `contains_start` | Illustrative. The report does not describe weeks. |
| `year_number` (`cy`) | `start` | Consistent with the filing, not stated by it: for the calendar year 2021 ("FY 2021"), `start`, `end` and `majority` all give 2021. |
| `year_number` (`fy`) | `start` | Inferred. It gives 2022 to the 15-month period and 2023 to the year after it, the numbers that begin the title "2022-23" and the tentative calendar's heading "2023-24", whose pairings with those periods are inferred. `majority` gives the same numbers (365 of the period's 455 days fall in 2022, and 275 of the next year's 366 days fall in 2023). |
| `fy` year template | `{yyyy}-{yy_end}` | Inferred, from the title "2022-23" and the heading "2023-24". The label it gives calendar years ("2021-21") is illustrative and not used. |
| `cy` year template | `FY {yyyy}` | Stated by the filing ("(FY 2021)") |

The 15-month period has 455 days by calendar arithmetic on the stated dates (see "Reported dates").

## Spec version 1 definitions of each regime (before 0.2.0)

Before fiscalclock 0.2.0, `exceptions` was reserved. A `fixed_date` year starts on the same month and day every year ([spec/definition-v1.md](../../spec/definition-v1.md), "`type: "fixed_date"`"), so no single definition had both a year that runs from 1 January 2022 to 31 March 2023 and regular years from 1 April. Below are one definition for the years before the transition and one for the years after it. Neither covers 1 January 2022 to 31 March 2023.

A naming problem arose. The report's title is "2022-23" and its tentative calendar for the following April to March year is headed "2023-24" (both pairings inferred; see above). Under the April definition below, numbered by start year, "2022-23" would mean 1 April 2022 to 31 March 2023, which overlaps 12 of the 15 months of the transition period. The 0.2.0 definition above resolves it with a transition year and `{yy_end}`.

Before (calendar years, last regular one is 2021):

```json
{
  "spec_version": 1,
  "year": {"type": "fixed_date", "start": {"month": 1, "day": 1}},
  "week": {"starts_on": "monday", "numbering": "contains_start"},
  "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
  "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start", "templates": {"year": "FY {yyyy}", "quarter": "Q{q} FY {yyyy}"}}}}
}
```

After (years from 1 April 2023):

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
| Before: `year.start` | 1 January | Stated by the filing ("from existing “1 January to 31 December”") |
| After: `year.start` | 1 April | Stated by the filing ("from 1 April every year to 31 March of the subsequent year") |
| Transition period | 1 January 2022 to 31 March 2023 | Stated by the filing; not expressible before fiscalclock 0.2.0 (see the definition above) |
| Before: `year_number` | `start` | Illustrative. For a calendar year, `start` and `end` give the same number. |
| After: `year_number` | `start` | Inferred. The row "Financial Year April 2023 to March 2024" sits under the heading "Financial Calendar 2023-24" of a tentative calendar; the pairing comes from the table's layout, one year only. |
| Before: year template | `FY {yyyy}` | Stated by the filing ("(FY 2021)"); the space is printed |
| After: year template | `{yyyy}-{yy_next}` | Inferred, from the same heading ("Financial Calendar 2023-24"). The report prints no "FY" prefix with it. |
| Quarter templates | `Q{q} …` | Illustrative. The report gives no quarter label. |
| `subdivision` | calendar months, quarters 3, 3, 3, 3 | Illustrative. The report states no quarter structure for the regular years. |
| `week` | `monday`, `contains_start` | Illustrative. The report does not describe weeks. The block is required by the spec. |

## Conformance vectors

One vector from `conformance/generators/spec_cited_vectors.py`, on the 0.2.0 definition above. The two spec version 1 definitions have no vector.

- `conformance/vectors/cited/linde-india-annual-report-2022-23-transition.json` has 7 cases: validation; the start, last day and 455 days of the 15-month period (located by date on its first and last days); the bounds of the calendar year 2021, its label `FY 2021` under scheme `cy` and parsing it; and the bounds of the year from 1 April 2023 to 31 March 2024 (located by date; the first year of the stated rule "thereafter from 1 April every year to 31 March of the subsequent year"). It asserts no `fy` label, because the pairings of "2022-23" and "2023-24" with their periods are inferred.

The oracle vector `conformance/vectors/oracle/history-jan-to-apr-15-month-transition.json` uses the same dates with illustrative quarters, weeks and schemes.

## Still to check

1. Whether the cover page (PDF page 1) prints the title "Annual Report 2022-23" together with the line "For 15 months period from 1 January 2022 to 31 March 2023". If it does, add that text to the quote manifest as one quote and assert the label "2022-23" in the vector.
2. A label for the first regular April to March year that is not a tentative calendar. Read the next annual report (the year ended 31 March 2024, listed on the same company page) for a sentence that ties a label to 1 April 2023 to 31 March 2024.
3. Quarter boundaries. The report lists tentative result-announcement dates (for example 8 August 2023 for first-quarter results) but not quarter end dates, for the transition period or for the regular years. Check the company's quarterly results filings.
4. Whether the Regional Director's order of 29 March 2023 is on the company's site. This record relies on the report's description of it.
5. The report compares the 15 months with "12 months ended 31 December 2022" (printed page 13). That is a management comparison, not a statutory period, and it is not used here.
