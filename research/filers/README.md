# Filer records

Each page here records, in the company's own words, how one public company defines its fiscal calendar, so that a cited conformance vector can be traced back to its source.

Last reviewed: 1 Nov 2026

## Rules for these pages

- The source is the company's own investor-relations copy of its annual report or Form 10-K. Pages do not use sec.gov or archive copies of it.
- Quotes are verbatim. Each one gives the page of the document where it appears.
- Each page maps the calendar to a [spec version 1](../../spec/definition-v1.md) definition and marks every field "stated by the filing" or "illustrative".
- Expected values in the matching vectors come from the document text, never from running fiscalclock.
- A document that could not be retrieved is recorded as such, with what remains to be checked.

## Records

| Company | Document | Status | Vector |
| --- | --- | --- | --- |
| [Kroger](kroger.md) | 2019 Proxy Statement and 2018 Annual Report (fiscal year ended 2 February 2019) | Quoted | `conformance/vectors/cited/kroger-annual-report-2018.json` |
| [Costco](costco.md) | 2017 Annual Report (fiscal year ended 3 September 2017) | Quoted | `conformance/vectors/cited/costco-annual-report-2017.json` |
| [Vishay](vishay.md) | Annual report or Form 10-K | Not retrieved | None; spec version 1 cannot express the calendar |

## How the quotes were checked

The PDFs were downloaded from the investor-relations hosts named on each page on 1 Nov 2026. Text was extracted with `pdftotext -layout -enc UTF-8` (and without `-layout` for tables, to keep reading order). Each quote was compared character for character with the extracted text after joining line breaks. Each page records the PDF's SHA-256 digest, so a reader can confirm they are reading the same file.
