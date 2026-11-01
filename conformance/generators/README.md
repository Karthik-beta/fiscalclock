# Vector generators

These scripts write every file under `conformance/vectors/`. All but `pandas_vectors.py` use the Python standard library only, read no network or clock, and never import fiscalclock. The expected results come from `fiscal_oracle.py`, a separate implementation of `spec/definition-v1.md` and `spec/api-v1.md` built on Python `datetime`. ISO week vectors come from `date.isocalendar()` instead.

## Run

```sh
python -I conformance/generators/generate.py   # rewrite all vectors
python -I conformance/generators/check.py      # CI: regenerate into a temp dir, diff, validate the schema
```

`generate.py` is deterministic, so two runs give identical bytes. It deletes any `*.json` file in a vector group that it did not produce. `check.py` exits 1 when a committed file is missing, extra or different, or when a file fails `conformance/schema/vector.schema.json`. It ignores CRLF versus LF differences. Each script takes about 15 seconds, most of it spent checking year numbering over every fiscal year in the supported range.

## Files

- `fiscal_oracle.py` is the independent model. It covers year bounds, structure, `locate`, `shift`, `parent`, `children`, `years`, `format`, `parse`, `dimension`, `dimension_csv`, the canonical form with `calendar_id`, and the two computed validation checks, `YEAR_NUMBER_COLLISION` and `MAJORITY_TIE`, over every fiscal year in the supported range. It does not model the other validation codes; their vectors take code and path from the spec text. Dates are proleptic ordinals, so boundary arithmetic works past 0001-01-01 and 9999-12-31.
- `vectorlib.py` holds the definition builders, the case runner and the writer.
- `oracle_vectors.py` writes `vectors/oracle/`:
  - `week-based-<edge>-<rule>-<weekday>` holds 42 week-based definitions, one for each edge, rule and week-start weekday, with rotating anchor months and period patterns. Each has year bounds for 2015 to 2030 and full locations on boundary days. Each is named by its edge year when that passes validation, otherwise by the other edge, otherwise by `majority`. Two combinations, start/nearest/saturday and start/nearest/sunday, collide under both `start` and `end` and use `majority`; their descriptions say so.
  - `nrf-4-5-4` covers NRF years 2016 to 2030, the structure of 2017, 2023 and 2024, all weeks of 2023, navigation across 53-week years, and labels, including `FY2023 Q01`, which is not recognised because `{q}` has no leading zero.
  - `apple-style` uses an illustrative 4-4-5 pattern with the extra week in period 1, plus two-digit labels with `century_pivot`.
  - `iso-week-year` uses `year_number: "majority"`. It locates 28 December and 4 January for every year 1600 to 2399, and 1 January and 31 December for 1970 to 2069, and formats and parses week and year labels for 2015 to 2030. The expected values, year numbers included, come from `date.isocalendar()` and `date.fromisocalendar()`.
  - The `fixed-*` files cover 1 January, 1 April (India FY and AY, and a `{yy}-{yy_next}` scheme with `century_pivot`), 1 July, 1 October (US federal), 6 April (UK tax year), and day 30 or 31 starts under `clamp` and `next`.
  - `dimension-nrf` has JSON rows and the exact CSV text, and an empty range as `[]` and as the CSV header row alone.
  - `labels-ambiguous-india` has an `AMBIGUOUS_LABEL` case with its candidates and `LABEL_NOT_RECOGNISED` cases.
  - `calendar-id-pair-a` and `calendar-id-pair-b` must share one `calendar_id`.
  - `canonical-non-ascii` has Japanese templates written as raw UTF-8 and a template with a tab, U+001F, a quote and a backslash. Its canonical string is typed by hand in the generator, which asserts that the oracle agrees.
  - `range-edges-*` covers the first and last supported fiscal years.
- `error_vectors.py` writes `vectors/errors/`. The `definition-*` files hold one invalid definition per validation code, and the expected code and path come from reading the spec. The `definition-year-number-collision-*` and `definition-majority-tie-*` files are computed by the oracle's scan of the supported range, and assert code and path only, because the spec does not pin the content of `details.years`. The `runtime-*` files cover runtime errors computed by the oracle, with paths from the api-v1.md table "Paths for common errors".
- `spec_cited_vectors.py` writes `vectors/spec/` and `vectors/cited/`. Their expected values are typed in from the spec or from a quoted source, and the generator stops if the oracle disagrees.
- `pandas_vectors.py` writes `vectors/pandas/` (see [pandas FY5253 vectors](#pandas-fy5253-vectors)).
- `schema_check.py` is a small validator for the JSON Schema keywords the vector schema uses. It rejects any keyword it does not implement.

## pandas FY5253 vectors

`pandas_vectors.py` gives the week-based year bounds a second source that shares no code with `fiscal_oracle.py`. It takes every year-end date from the `FY5253` offset of pandas 3.0.6 (released 17 September 2026, per [PyPI](https://pypi.org/project/pandas/3.0.6/)), and derives the other fields by date arithmetic on those dates. It does not import fiscalclock or `fiscal_oracle.py`. pandas is a dependency of this script only; `generate.py` and `check.py` stay standard library only, and they neither write nor delete `vectors/pandas/`, because `vectorlib.write_all` touches only the groups in `GROUPS`. The script refuses to run under any other pandas version.

```sh
uv run --no-project --with pandas==3.0.6 python -I conformance/generators/pandas_vectors.py           # rewrite vectors/pandas/
uv run --no-project --with pandas==3.0.6 python -I conformance/generators/pandas_vectors.py --check   # CI: regenerate into a temp dir, diff, validate the schema
```

### Mapping from FY5253 to a definition

The [pandas documentation for `FY5253`](https://pandas.pydata.org/docs/reference/api/pandas.tseries.offsets.FY5253.html) (as of 1 November 2026) describes `weekday` as an integer from 0 (Monday) to 6 (Sunday), `startingMonth` as the month in which the fiscal year ends, and `variation` as `"nearest"` (the year ends on the weekday closest to the last day of that month) or `"last"` (the year ends on the final such weekday of that month). Each `FY5253(weekday=w, startingMonth=m, variation=v)` maps to:

| FY5253 | Definition | Reason |
| --- | --- | --- |
| `startingMonth=m` | `year.anchor.edge: "end"`, `month: m`, `day: "last"` | pandas fixes the year's last day relative to the last day of month `m`. |
| `variation="last"` | `rule: "last"` | The final weekday `w` of the month is the latest `w` on or before its last day, which is the spec's `last` rule. |
| `variation="nearest"` | `rule: "nearest"` | Both pick the weekday closest to the target; a weekday is never four days from the target both ways, so neither needs a tie rule. |
| `weekday=w` | `week.starts_on`: the weekday after `w` | The spec has no year-end weekday; with edge `end` the year's last day is the day before `week.starts_on` (definition-v1.md, "Week-based year"). |

The subdivision is an arbitrary valid 4-4-5 pattern with the extra week in period 12. No case asserts quarters, periods or weeks, because pandas does not model them.

### Coverage and year numbers

The script writes one file, `fy5253-<variation>-<weekday>-m<month>`, for each of the 168 combinations of 7 weekdays, 12 months and 2 variations. Each file has a `validate` case that asserts only `ok: true`, `period` cases with the exact year for the fiscal years numbered 2000 to 2030, and `locate` cases that assert the date and the containing year on each year's last day and the next year's first day for 2024 to 2026.

The spec rejects a scheme that gives two fiscal years one number or skips a number (`YEAR_NUMBER_COLLISION`) or a `majority` tie (`MAJORITY_TIE`) anywhere in the supported range. The script picks `year_number: "end"` when it numbers consecutively every FY5253 year whose last day falls in 1700 to 2099, and `"majority"` otherwise. Those 400 years are 146,097 days, a whole number of weeks, which the script asserts, so the pattern of year ends repeats in every 400-year cycle and one cycle stands for the whole range. The result is `"majority"` for the seven December `nearest` combinations, whose year can end on 1 to 3 January, and `"end"` for the other 161.

## Reading conventions used by the vectors

- `expect_subset` applies recursively. A nested object lists only the keys it asserts.
- `expect_error` may carry `details`, and when present it is compared exactly. The `AMBIGUOUS_LABEL` candidates use it.
