# Verification scripts

This folder reproduces every date-arithmetic claim that the project's research documents and wiki describe as "verified by script". Anyone can rerun it with a stock Python interpreter and compare the result with the committed output.

## Run it

```sh
python -I research/verification/verify_claims.py
```

The script uses the Python standard library only and needs Python 3.9 or later. It prints one line per check and exits with status 1 if any check fails. The Python version goes to standard error, so standard output is identical on every platform and version.

CI runs the script on every pull request and compares its standard output with [`expected_output.txt`](expected_output.txt) byte for byte. To update the committed output after an intentional change:

```sh
python -I research/verification/verify_claims.py > research/verification/expected_output.txt
```

A change to `expected_output.txt` needs a linked issue that explains why the result changed.

## Checks

| ID | Claim | Where it is used |
| --- | --- | --- |
| C01 | All 168 "last" and "nearest" 52/53-week rules (7 weekdays × 12 months × 2 rules) have exactly 71 years of 53 weeks in each of two 400-year windows | Proposal: Testing and verification; wiki Evidence |
| C02 | 400 Gregorian years hold exactly 20,871 weeks, which is why C01 holds | Proposal: Testing and verification |
| C03 | NRF 4-5-4 (Saturday nearest 31 January, named by start year): 53-week fiscal years in 2016–2030 are 2017, 2023 and 2028 | Proposal: Worked definitions |
| C04 | NRF fiscal 2023 runs from 29 January 2023 to 3 February 2024 | Proposal: Worked definitions |
| C05 | NRF fiscal 2024 starts on 4 February 2024 | Proposal: Semantics |
| C06 | 364 days before 4 February 2024 is 5 February 2023, in week 2 of NRF fiscal 2023 | Proposal: Equivalent day |
| C07 | In the year after a 53-week year, the same fiscal position is 371 days back | Proposal: Equivalent day |
| C08 | NRF's 2017 restatement moves the first comparable week from 29 Jan–4 Feb to 5–11 Feb | Proposal: Comparable periods |
| C09 | Apple (last Saturday of September): 53-week fiscal years in 2016–2030 are 2017, 2023 and 2028 | Proposal: Testing and verification |
| C10 | Broadcast-calendar months (ending on the last Sunday) are 4 or 5 weeks in 2020–2031; the full month and quarter table | Proposal: Shapes the families miss |
| C11 | Broadcast 2024 months run 4-4-5 in every quarter | Proposal: Shapes the families miss |
| C12 | Broadcast 2021 Q1 months run 5-4-4 | Proposal: Shapes the families miss (corrected, see C12a) |
| C12a | The proposal's "5-4-4 in 2021" holds for Q1 and Q4 only; Q2 and Q3 run 4-5-4 | Correction recorded on the wiki Evidence page |
| C13 | Broadcast 2029 quarters are 12, 13, 14 and 13 weeks | Proposal: Shapes the families miss |
| C14 | Broadcast quarters run 12 to 14 weeks in 2020–2031 | Proposal: Testing and verification |
| C15 | The ISO 8601 week-numbering year equals a week-based year starting on the Monday nearest 1 January, for every year 1600–2399, checked against `date.isocalendar()` | Proposal: Core model |
| C16 | Care.com's year rule and quarter rule give different Q4 ends in 2020, 2021 and 2025–2027 (window 2019–2028), for example 27 Dec 2025 against 3 Jan 2026 | Proposal: Shapes the families miss |
| C17 | Informational: the C16 result depends on reading "closest and prior to December 31" as "on or before 31 December", and on the window | Wiki Evidence (caveat) |
| C18 | `date(9999, 12, 31) + timedelta(days=1)` raises `OverflowError` | Proposal: Edge cases (supported range) |
| C19 | The NRF rule gives exactly the 53-week years NRF publishes for 2006–2023 (2006, 2012, 2017 and 2023); NRF's page lists none after FY23, so 2028 (C03) is the rule's prediction only | Conformance vector `cited/nrf-53-week-years.json`; wiki Evidence |

## Relationship to the conformance vectors

These checks test claims in prose. The conformance vectors in [`conformance/`](../../conformance/README.md) test the library. The vector generators reimplement the same arithmetic independently of the Rust core, and CI checks both.
