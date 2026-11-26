# Fuzzing

Coverage-guided fuzz targets for the definition parser and the label parser, run with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and libFuzzer. Gate D on the [Roadmap](https://github.com/Karthik-beta/fiscalclock/wiki/Roadmap#phase-3-v03) asks for at least 24 CPU hours of fuzzing with no panic, hang or unbounded allocation; the [log](#cpu-hours-so-far) below counts them.

This crate is not part of the repository's Cargo workspace. It builds only with nightly Rust, and its dependency `libfuzzer-sys` is a dependency of this crate alone, not of `fiscalclock`.

## Targets

| Target | Input | What must hold |
| --- | --- | --- |
| `validate_json` | Any UTF-8 text, passed to `fiscalclock::json::validate` | No panic; the output is JSON, and an error result is a non-empty list of errors that each have a `code`, a `path` and a `message`. |
| `validate_structured` | A conformance vector's definition with one to eight fields replaced, added or removed, using the format's own field names and values | As above. A definition that validates also runs `locate`, `explain`, `period`, `format`, `shift`, `children`, `years` and `parse` with arguments taken from the input, and each output must meet the same rule. |
| `parse_label` | A conformance vector's definition with arbitrary `year`, `quarter`, `period` and `week` templates and `century_pivot` | As above, including `parse` of an arbitrary label and `format` of a period. |

libFuzzer itself reports a hang (`-timeout=10`, in seconds per input) and a runaway allocation (`-rss_limit_mb=2048`).

`seed_corpus.py` writes every `definition` in [`conformance/vectors/`](../conformance/vectors/) into `corpus/<target>/` as a seed. The corpus directory is not committed: the vectors are its source, and libFuzzer adds to it on every run.

## Run it

Install the dated nightly and cargo-fuzz once, with the versions that [`ci.yml`](../.github/workflows/ci.yml) pins (`FUZZ_NIGHTLY` and `CARGO_FUZZ_VERSION`):

```sh
rustup toolchain install nightly-2026-11-01 --profile minimal --component rustfmt,clippy
cargo +nightly-2026-11-01 install cargo-fuzz --version 0.13.2 --locked
```

Then, from the repository root:

```sh
python -I fuzz/seed_corpus.py
cd fuzz
cargo +nightly-2026-11-01 fuzz run validate_json corpus/validate_json -- -max_total_time=60 -timeout=10 -rss_limit_mb=2048
```

`python -I scripts/check.py fuzz` runs the same one-minute run of each target, after `fmt` and `clippy`, as the `fuzz` job in CI does.

On Windows, cargo-fuzz builds with the MSVC AddressSanitizer, so the folder holding `clang_rt.asan_dynamic-x86_64.dll` (in the Visual Studio build tools, under `VC/Tools/MSVC/<version>/bin/Hostx64/x64`) must be on `PATH`; without it the target exits with `STATUS_DLL_NOT_FOUND`. `--sanitizer none` does not help on Windows: the link then fails on the `__sancov_pcs` symbols. On 24 Nov 2026, a Windows run with AddressSanitizer managed 6 to 17 inputs per second, so long runs belong on Linux.

A crash, hang or out-of-memory input is saved in `artifacts/<target>/`. `cargo +nightly-2026-11-01 fuzz run <target> <file>` reproduces it. Each one becomes a regression test in `crates/core/tests/` and a fix, in the same pull request.

## Long runs

The [`fuzz` workflow](../.github/workflows/fuzz.yml) is started by hand (Actions, "fuzz", "Run workflow"). It runs each target with `-fork` on every core of one runner for the chosen number of minutes. Each job's summary gives a row with the cores, the seconds and the CPU hours (seconds × cores ÷ 3600). Copy the rows into the log below. Crashes and the grown corpus are uploaded as artifacts.

## CPU hours so far

CPU hours are wall-clock time multiplied by the number of fuzzing processes. Gate D needs 24 in total, across the targets, with nothing found.

| Date | Where | Target | Commit | Cores | Seconds | CPU hours | Result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 24 Nov 2026 | Windows 11, local | `validate_json` | issue #84 branch | 1 | 72 | 0.02 | Nothing found |
| 24 Nov 2026 | Windows 11, local | `validate_structured` | issue #84 branch | 1 | 61 | 0.02 | Nothing found |
| 24 Nov 2026 | Windows 11, local | `parse_label` | issue #84 branch | 1 | 61 | 0.02 | Nothing found |
| 24 Nov 2026 | [`fuzz.yml` run 45326630385](https://github.com/Karthik-beta/fiscalclock/actions/runs/45326630385) | `validate_json` | `aad5e8e` | 4 | 7238 | 8.04 | Nothing found |
| 24 Nov 2026 | [`fuzz.yml` run 45326630385](https://github.com/Karthik-beta/fiscalclock/actions/runs/45326630385) | `validate_structured` | `aad5e8e` | 4 | 7205 | 8.01 | Nothing found |
| 24 Nov 2026 | [`fuzz.yml` run 45326630385](https://github.com/Karthik-beta/fiscalclock/actions/runs/45326630385) | `parse_label` | `aad5e8e` | 4 | 7206 | 8.01 | Nothing found |

Total: 24.12 CPU hours as of 24 Nov 2026, which meets Gate D's 24.

The seconds for run 45326630385 are libFuzzer's own fuzzing time, from the last line of each job's log (`INFO: exiting: 0 time: …s`), and the cores are its `-fork=4`. Each job's last status line reports `oom/timeout/crash: 0/0/0`. The workflow's job summary also counted about 30 seconds of build in each row; from the next run it builds before it starts timing.

The first local runs timed out on an empty input. The cause was in the harness, not in fiscalclock: reading the 377 seed definitions on the first input took longer than the ten-second limit in the instrumented build, so the targets now read them in libFuzzer's `init` step.
