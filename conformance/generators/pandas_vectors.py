"""Write conformance/vectors/pandas/: 52/53-week year bounds whose expected values come from pandas' FY5253 offset.

Usage:
    uv run --no-project --with pandas==3.0.6 python -I conformance/generators/pandas_vectors.py [--out DIR]
    uv run --no-project --with pandas==3.0.6 python -I conformance/generators/pandas_vectors.py --check

pandas is the only source of dates. This script does not import fiscalclock or fiscal_oracle.py;
it imports schema_check.py only to validate its own output. --check regenerates into a temporary
directory and exits 1 when the committed files differ or fail the vector schema.
See conformance/generators/README.md for the mapping from FY5253 parameters to definitions.
"""

from __future__ import annotations

import argparse
import json
import sys
import tempfile
from datetime import date, timedelta
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

import pandas as pd  # noqa: E402

import schema_check  # noqa: E402

PANDAS_VERSION = "3.0.6"
DOCS = "https://pandas.pydata.org/docs/reference/api/pandas.tseries.offsets.FY5253.html"
GROUP = "pandas"
COMMITTED = HERE.parent / "vectors" / GROUP
SCHEMA = HERE.parent / "schema" / "vector.schema.json"

WEEKDAYS = ("monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday")
VARIATIONS = ("last", "nearest")
CYCLE = (1700, 2100)
PERIOD_YEARS = range(2000, 2031)
LOCATE_YEARS = range(2024, 2027)


def fiscal_years(weekday: int, month: int, variation: str, first: int, last: int) -> list[tuple[date, date]]:
    """(start, end_exclusive) of every FY5253 fiscal year whose last day falls in calendar years first..last."""
    offset = pd.offsets.FY5253(weekday=weekday, startingMonth=month, variation=variation)
    end = offset.rollback(pd.Timestamp(first - 1, 12, 31))
    ends = [end.date()]
    while end.year <= last:
        end = end + offset
        ends.append(end.date())
    return [(a + timedelta(days=1), b + timedelta(days=1)) for a, b in zip(ends, ends[1:]) if first <= b.year <= last]


def majority_year(start: date, end_exclusive: date) -> int | None:
    days = (end_exclusive - start).days
    in_first = (date(start.year + 1, 1, 1) - start).days if end_exclusive.year > start.year else days
    if in_first * 2 > days:
        return start.year
    if in_first * 2 < days:
        return start.year + 1
    return None


def year_numbers(years: list[tuple[date, date]], scheme: str) -> list[int] | None:
    numbers = [(e - timedelta(days=1)).year for _, e in years] if scheme == "end" else [majority_year(s, e) for s, e in years]
    if None in numbers or any(b != a + 1 for a, b in zip(numbers, numbers[1:])):
        return None
    return numbers


def choose_year_number(weekday: int, month: int, variation: str) -> str:
    """'end' if it numbers every year of a full 400-year Gregorian cycle consecutively, else 'majority'.

    The cycle is 146,097 days, a whole number of weeks, so FY5253 year ends repeat with it and one
    cycle stands for the whole supported range.
    """
    span = (date(CYCLE[1], 1, 1) - date(CYCLE[0], 1, 1)).days
    assert span == 146097 and span % 7 == 0, span
    years = fiscal_years(weekday, month, variation, *CYCLE)
    for scheme in ("end", "majority"):
        if year_numbers(years, scheme) is not None:
            return scheme
    raise AssertionError(f"no year_number numbers FY5253({weekday}, {month}, {variation}) consecutively")


def definition(weekday: int, month: int, variation: str, year_number: str) -> dict:
    return {
        "spec_version": 1,
        "year": {"type": "week_based", "anchor": {"edge": "end", "month": month, "day": "last", "rule": variation}},
        "week": {"starts_on": WEEKDAYS[(weekday + 1) % 7], "numbering": "contains_start"},
        "subdivision": {"type": "week_pattern", "periods": [4, 4, 5] * 4, "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}},
        "naming": {"default": "fy", "schemes": {"fy": {"year_number": year_number, "templates": {"year": "FY{yyyy}"}}}},
    }


def year_period(number: int, start: date, end_exclusive: date) -> dict:
    days = (end_exclusive - start).days
    assert days in (364, 371), (start, end_exclusive)
    return {
        "level": "year",
        "year": number,
        "index": None,
        "start": start.isoformat(),
        "end_exclusive": end_exclusive.isoformat(),
        "last_day": (end_exclusive - timedelta(days=1)).isoformat(),
        "days": days,
        "weeks": days // 7,
        "has_extra_week": days == 371,
    }


def vector(weekday: int, month: int, variation: str) -> dict:
    year_number = choose_year_number(weekday, month, variation)
    years = fiscal_years(weekday, month, variation, PERIOD_YEARS.start - 1, PERIOD_YEARS.stop)
    periods = [year_period(n, s, e) for n, (s, e) in zip(year_numbers(years, year_number), years)]
    cases = [{"op": "validate", "args": {}, "expect_subset": {"ok": True}}]
    cases += [{"op": "period", "args": {"level": "year", "year": p["year"]}, "expect": p} for p in periods if p["year"] in PERIOD_YEARS]
    by_number = {p["year"]: p for p in periods}
    for y in LOCATE_YEARS:
        for p, day in ((by_number[y], by_number[y]["last_day"]), (by_number[y + 1], by_number[y + 1]["start"])):
            cases.append({"op": "locate", "args": {"date": day}, "expect_subset": {"date": day, "year": p}})
    call = f"FY5253(weekday={weekday}, startingMonth={month}, variation={variation!r})"
    return {
        "name": f"fy5253-{variation}-{WEEKDAYS[weekday]}-m{month:02d}",
        "description": (
            f"pandas {call}: the fiscal year ends on the {WEEKDAYS[weekday].capitalize()} "
            f"{'on or before and closest to' if variation == 'last' else 'closest to'} the last day of month {month}. "
            f"Mapped to anchor edge end, month {month}, day last, rule {variation}, weeks start {WEEKDAYS[(weekday + 1) % 7]}; "
            f"years named by year_number '{year_number}', which the generator checked numbers every FY5253 year of the "
            f"400-year cycle {CYCLE[0]}-{CYCLE[1] - 1} consecutively. Year bounds {PERIOD_YEARS.start}-{PERIOD_YEARS.stop - 1}, "
            f"and the fiscal year located on each year's last day and the next year's first day for {LOCATE_YEARS.start}-{LOCATE_YEARS.stop - 1}. "
            f"The subdivision is an arbitrary valid 4-4-5 pattern that no case asserts."
        ),
        "source": {
            "kind": "oracle",
            "citation": f"Year-end dates from pandas {PANDAS_VERSION} pandas.tseries.offsets.{call}; days, weeks and year numbers are arithmetic on those dates. No fiscalclock code.",
            "url": DOCS,
            "checked_by": f"conformance/generators/pandas_vectors.py --check with pandas=={PANDAS_VERSION}",
        },
        "definition": definition(weekday, month, variation, year_number),
        "cases": cases,
    }


def build() -> list[dict]:
    return [vector(w, m, v) for v in VARIATIONS for w in range(7) for m in range(1, 13)]


def write(folder: Path, vectors: list[dict]) -> None:
    folder.mkdir(parents=True, exist_ok=True)
    names = {v["name"] for v in vectors}
    assert len(names) == len(vectors), "duplicate vector name"
    for stale in folder.glob("*.json"):
        if stale.stem not in names:
            stale.unlink()
    for v in vectors:
        with open(folder / f"{v['name']}.json", "w", encoding="utf-8", newline="\n") as fh:
            fh.write(json.dumps(v, indent=2, ensure_ascii=False) + "\n")


def check(vectors: list[dict]) -> list[str]:
    read = lambda p: p.read_bytes().replace(b"\r\n", b"\n")  # noqa: E731
    problems = []
    with tempfile.TemporaryDirectory() as tmp:
        fresh = Path(tmp)
        write(fresh, vectors)
        new = {p.name for p in fresh.glob("*.json")}
        old = {p.name for p in COMMITTED.glob("*.json")} if COMMITTED.is_dir() else set()
        problems += [f"missing from commit: {GROUP}/{n}" for n in sorted(new - old)]
        problems += [f"not produced by pandas_vectors.py: {GROUP}/{n}" for n in sorted(old - new)]
        problems += [f"differs from pandas_vectors.py output: {GROUP}/{n}" for n in sorted(new & old) if read(fresh / n) != read(COMMITTED / n)]
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    for path in sorted(COMMITTED.glob("*.json")):
        doc = json.loads(path.read_text(encoding="utf-8"))
        problems += [f"{GROUP}/{path.name}: {e}" for e in schema_check.errors(doc, schema, schema)]
        if doc.get("name") != path.stem:
            problems.append(f"{GROUP}/{path.name}: name does not match file name")
    return problems


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", type=Path, default=COMMITTED)
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args(argv)
    if pd.__version__ != PANDAS_VERSION:
        print(f"needs pandas=={PANDAS_VERSION}, found {pd.__version__}")
        return 1
    vectors = build()
    summary = f"{len(vectors)} files, {sum(len(v['cases']) for v in vectors)} cases"
    if args.check:
        problems = check(vectors)
        for p in problems:
            print(p)
        print(f"{GROUP}: {summary}: {'OK' if not problems else f'{len(problems)} problem(s)'}")
        return 1 if problems else 0
    write(args.out, vectors)
    print(f"{GROUP}: {summary}, written to {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
