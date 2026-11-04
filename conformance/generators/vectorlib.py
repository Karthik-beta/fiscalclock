"""Shared helpers: definition builders, case builders and the vector writer."""

from __future__ import annotations

import json
from pathlib import Path

from fiscal_oracle import Calendar, FiscalError, calendar_id, canonical

GROUPS = ("oracle", "spec", "errors", "cited")
ORACLE_CITATION = "Independent implementation of the spec in conformance/generators/fiscal_oracle.py, written with Python datetime and no fiscalclock code"


def scheme(year_number: str, templates: dict, offset: int | None = None, pivot: int | None = None) -> dict:
    out = {"year_number": year_number}
    if offset is not None:
        out["offset"] = offset
    if pivot is not None:
        out["century_pivot"] = pivot
    out["templates"] = templates
    return out


def naming(schemes: dict, default: str | None = None) -> dict:
    return {"default": default or next(iter(schemes)), "schemes": schemes}


def week_based(edge, month, day, rule, starts_on, periods, quarters, extra_period, names: dict) -> dict:
    return {
        "spec_version": 1,
        "year": {"type": "week_based", "anchor": {"edge": edge, "month": month, "day": day, "rule": rule}},
        "week": {"starts_on": starts_on, "numbering": "contains_start"},
        "subdivision": {"type": "week_pattern", "periods": list(periods), "quarters": list(quarters), "extra_week": {"period": extra_period}},
        "naming": names,
    }


def fixed_date(month, day, starts_on, names: dict, short_month: str | None = None, quarters=(3, 3, 3, 3)) -> dict:
    year = {"type": "fixed_date", "start": {"month": month, "day": day}}
    if short_month is not None:
        year["short_month"] = short_month
    return {
        "spec_version": 1,
        "year": year,
        "week": {"starts_on": starts_on, "numbering": "contains_start"},
        "subdivision": {"type": "calendar_months", "month_start_day": day, "quarters": list(quarters)},
        "naming": names,
    }


def run(cal: Calendar, op: str, args: dict):
    if op == "validate":
        errors = cal.naming_errors()
        if errors:
            raise errors[0]
        return {"ok": True, "calendar_id": calendar_id(cal.definition), "canonical": canonical(cal.definition)}
    if op == "locate":
        return cal.locate(args["date"])
    if op == "period":
        return cal.period(args["level"], args["year"], args.get("index"))
    if op == "shift":
        return cal.shift(args["period"], args["n"])
    if op == "parent":
        return cal.parent(args["period"])
    if op == "children":
        return cal.children(args["period"], args["level"])
    if op == "years":
        return cal.years(args["from"], args["to"])
    if op == "format":
        return cal.format(args["period"], args.get("scheme"))
    if op == "parse":
        return cal.parse(args["label"])
    if op == "dimension":
        return cal.dimension(args["start"], args["end_exclusive"])
    if op == "dimension_csv":
        return cal.dimension_csv(args["start"], args["end_exclusive"])
    raise ValueError(op)


def case(cal: Calendar, op: str, args: dict) -> dict:
    try:
        return {"op": op, "args": args, "expect": run(cal, op, args)}
    except FiscalError as err:
        return {"op": op, "args": args, "expect_error": err.expect_error()}


def validate_case(definition: dict) -> dict:
    c = case(Calendar(definition), "validate", {})
    assert "expect" in c, c
    return c


def year_args(year: int) -> dict:
    return {"level": "year", "year": year}


def coords(level: str, year: int, index=None) -> dict:
    return {"level": level, "year": year, "index": index}


def source(kind: str, citation: str, checked_by: str, url: str | None = None) -> dict:
    return {"kind": kind, "citation": citation, "url": url, "checked_by": checked_by}


def vector(name: str, description: str, src: dict, definition, cases: list) -> dict:
    seen, unique = set(), []
    for c in cases:
        key = json.dumps([c["op"], c["args"]], sort_keys=True)
        if key not in seen:
            seen.add(key)
            unique.append(c)
    cases = unique
    return {"name": name, "description": description, "source": src, "definition": definition, "cases": cases}


def dumps(obj) -> str:
    return json.dumps(obj, indent=2, ensure_ascii=False) + "\n"


def write_all(out_root: Path, vectors: dict[str, list[dict]]) -> list[Path]:
    written = []
    for group in GROUPS:
        folder = out_root / group
        folder.mkdir(parents=True, exist_ok=True)
        names = {v["name"] for v in vectors.get(group, [])}
        assert len(names) == len(vectors.get(group, [])), f"duplicate vector name in {group}"
        for stale in folder.glob("*.json"):
            if stale.stem not in names:
                stale.unlink()
        for v in vectors.get(group, []):
            path = folder / f"{v['name']}.json"
            with open(path, "w", encoding="utf-8", newline="\n") as fh:
                fh.write(dumps(v))
            written.append(path)
    return written
