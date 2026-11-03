# Filer records

<!-- cspell:words Poppler's -->

Each page here records, in the company's own words, how one public company defines its fiscal calendar, so that a cited conformance vector can be traced back to its source.

Last reviewed: 3 Nov 2026

## Rules for these pages

- The source is the company's own investor-relations copy of its annual report or Form 10-K. Pages do not use sec.gov, stock-exchange or archive copies of it.
- Quotes are verbatim. Each one gives the page of the document where it appears.
- Each page maps the calendar to a [spec version 1](../../spec/definition-v1.md) definition and marks every field "stated by the filing" or "illustrative".
- Expected values in the matching vectors come from the document text, never from running fiscalclock.
- A document that could not be retrieved is recorded as such, with what remains to be checked.

## Records

Indian listed companies come first. Most report on a year from 1 April to 31 March; the records below also cover a company that moved from July to June, and two that moved from a calendar year, each through a transition period of other than twelve months. Spec version 1 cannot express a transition period, so those records give the regular years on either side and keep the transition as a candidate case for release 0.2.

### India

| Company | Document | Calendar | Status | Vector |
| --- | --- | --- | --- | --- |
| [Reliance Industries](reliance.md) | Integrated Annual Report 2024-25 (financial year ended 31 March 2025) | 1 April to 31 March, labelled `FY 2024-25` | Quoted | `conformance/vectors/cited/reliance-annual-report-2024-25.json` |
| [Procter & Gamble Hygiene and Health Care](pg-hygiene.md) | Annual Reports 2023-24, 2024-25 and 2025-26 | 1 July to 30 June until 30 June 2024; a nine-month 2024-25 from 1 July 2024 to 31 March 2025; then 1 April to 31 March | Quoted | `conformance/vectors/cited/pg-hygiene-annual-report-2025-26.json` and `pg-hygiene-annual-report-2023-24.json`; none for the nine-month year |
| [Linde India](linde-india.md) | Annual Report 2022-23 (15 months ended 31 March 2023) | Calendar years until 2021; 15 months from 1 January 2022 to 31 March 2023; then 1 April to 31 March | Quoted | None; spec version 1 cannot express the transition period |
| [Thomas Cook (India)](thomas-cook-india.md) | Annual Report 2014-2015 (fifteen months ended 31 March 2015) | Calendar years until 2013; 15 months from 1 January 2014 to 31 March 2015; then 1 April to 31 March | Quoted | None; spec version 1 cannot express the transition period |

### United States

| Company | Document | Calendar | Status | Vector |
| --- | --- | --- | --- | --- |
| [Kroger](kroger.md) | 2019 Proxy Statement and 2018 Annual Report (fiscal year ended 2 February 2019) | 52/53 weeks ending on the Saturday nearest 31 January | Quoted | `conformance/vectors/cited/kroger-annual-report-2018.json` |
| [Costco](costco.md) | 2017 Annual Report (fiscal year ended 3 September 2017) | 52/53 weeks ending on the Sunday nearest 31 August | Quoted | `conformance/vectors/cited/costco-annual-report-2017.json` |
| [Vishay](vishay.md) | Annual report or Form 10-K | 13-week quarters inside a calendar year | Not retrieved | None; spec version 1 cannot express the calendar |

## How the quotes were checked

The PDFs were downloaded from the investor-relations hosts named on each page. Each page records the PDF's SHA-256 digest, so a reader can confirm they are reading the same file.

For the Kroger and Costco pages (1 Nov 2026), text was extracted with `pdftotext -layout -enc UTF-8` (and without `-layout` for tables, to keep reading order), and each quote was compared character for character with the extracted text after joining line breaks.

For the Indian pages (3 Nov 2026), every quote is also in a manifest in [`quotes/`](quotes/) with the PDF page it sits on, and [`check_quotes.py`](check_quotes.py) checks them. It downloads each PDF from its URL into `pdfs/` (ignored by git) unless it is already there, confirms the SHA-256, and looks for each quote on its stated page in the text that `pdftotext -enc UTF-8` extracts, with and without `-layout`, after runs of whitespace are collapsed. It needs Poppler's `pdftotext`. From the repository root:

```sh
python -I research/filers/check_quotes.py research/filers/quotes/*.json
```
