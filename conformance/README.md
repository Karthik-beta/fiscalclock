# Conformance vectors

This directory holds the shared test vectors that the Rust, Python and JavaScript test suites all load, so the three packages cannot drift apart. Each vector states where its expected results come from, and no expected result is produced by the fiscalclock implementation itself.

## Layout

- `vectors/<group>/<name>.json` holds the vectors. Groups: `oracle` (generated from an independent implementation or standard library), `cited` (taken from a published calendar, filing or standard, with a citation), `spec` (worked examples from the spec), `errors` (definitions and arguments that must fail with a given code). `pandas` holds week-based year bounds from pandas' `FY5253` offset, a second generator independent of `oracle`; its files have `source.kind` `oracle`.
- `generators/` holds standard-library Python scripts that write the `oracle` vectors. They do not import fiscalclock. CI regenerates them and fails if the output differs from the committed files. `generators/pandas_vectors.py` writes the `pandas` group and needs pandas; see [generators/README.md](generators/README.md#pandas-fy5253-vectors).
- `schema/vector.schema.json` is the JSON Schema for a vector file.

## Vector file

```json
{
  "name": "nrf-4-5-4-years",
  "description": "NRF 4-5-4 calendar year bounds, 2016-2030",
  "source": {
    "kind": "oracle",
    "citation": "Independent computation in conformance/generators/fiscal_oracle.py",
    "url": null,
    "checked_by": "research/verification/verify_claims.py C03, C04"
  },
  "definition": { "spec_version": 1, "...": "..." },
  "cases": [
    {
      "op": "period",
      "args": { "level": "year", "year": 2023 },
      "expect": { "level": "year", "year": 2023, "index": null, "start": "2023-01-29", "end_exclusive": "2024-02-04", "last_day": "2024-02-03", "days": 371, "weeks": 53, "has_extra_week": true }
    }
  ]
}
```

| Key | Meaning |
| --- | --- |
| `source.kind` | `oracle`, `cited`, `spec` or `errors`. |
| `source.citation` | For `cited`: the verbatim sentence from the source, in quotation marks, and what it establishes. |
| `source.url` | Link to the source, or `null` for a generator. |
| `source.checked_by` | How the expected results were checked: a script and check ID, or a manual comparison with the date it was made. |
| `definition` | A calendar definition (spec version 1). |
| `cases[].op` | An operation from `spec/api-v1.md`. |
| `cases[].args` | Its arguments, without `definition`. |
| `cases[].expect` | The exact result. Compared as parsed JSON (key order ignored). |
| `cases[].expect_subset` | Instead of `expect`: every key given must be present with this value, applied recursively to nested objects; other keys are ignored and arrays are compared whole. Used when a source only establishes some fields. |
| `cases[].expect_error` | Instead of `expect`: `{"code": "…", "path": "…"}`, optionally with `details`, which is compared exactly. Compared with the first error, or with the whole sorted list when it is an array. |

A vector whose `definition` is itself invalid has `"op": "validate"` cases with `expect_error`.

## Rules

1. Every vector has a source. Generated vectors name their generator; cited vectors quote their source verbatim and link it.
2. Never change an expected result without a linked issue explaining why.
3. A new behaviour lands with at least one vector that exercises it.
