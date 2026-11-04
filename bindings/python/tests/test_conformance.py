"""Runs every shared conformance vector through the Python package.

It also writes the raw output of every case to ``target/transcripts/python.txt``.
The Rust and JavaScript runners write the same transcript, and CI requires the
three files to be identical byte for byte.
"""

import json
import unittest
from pathlib import Path

import fiscalclock

ROOT = Path(__file__).resolve().parents[3]
VECTORS = ROOT / "conformance" / "vectors"
TRANSCRIPT = ROOT / "target" / "transcripts" / "python.txt"


def same(a, b):
    """JSON equality that keeps true and 1 apart, unlike Python's ==."""
    if isinstance(a, bool) or isinstance(b, bool):
        return type(a) is type(b) and a == b
    if isinstance(a, dict) and isinstance(b, dict):
        return a.keys() == b.keys() and all(same(a[k], b[k]) for k in a)
    if isinstance(a, list) and isinstance(b, list):
        return len(a) == len(b) and all(same(x, y) for x, y in zip(a, b))
    return type(a) is type(b) and a == b


def subset(want, got):
    if isinstance(want, dict) and isinstance(got, dict):
        return all(k in got and subset(v, got[k]) for k, v in want.items())
    return same(want, got)


def error_matches(want, got):
    return (
        same(want["code"], got["code"])
        and same(want["path"], got["path"])
        and ("details" not in want or same(want["details"], got["details"]))
    )


def run_case(definition, case):
    if case["op"] == "validate":
        return fiscalclock.validate_json(definition)
    args = json.dumps(case["args"], ensure_ascii=False, separators=(",", ":"))
    return fiscalclock.call_json(definition, case["op"], args)


def check_case(out, case):
    got = json.loads(out)
    if "expect" in case:
        return same(case["expect"], got)
    if "expect_subset" in case:
        return subset(case["expect_subset"], got)
    errors = got.get("errors", []) if isinstance(got, dict) else []
    want = case["expect_error"]
    if isinstance(want, list):
        return len(want) == len(errors) and all(error_matches(w, g) for w, g in zip(want, errors))
    return bool(errors) and error_matches(want, errors[0])


class Conformance(unittest.TestCase):
    def test_every_vector_passes(self):
        files = sorted(VECTORS.rglob("*.json"), key=lambda p: p.relative_to(VECTORS).as_posix())
        self.assertTrue(files, f"no vectors under {VECTORS}")
        lines, failures, cases = [], [], 0
        for path in files:
            rel = path.relative_to(VECTORS).as_posix()
            vector = json.loads(path.read_text(encoding="utf-8"))
            definition = json.dumps(vector["definition"], ensure_ascii=False, separators=(",", ":"))
            for i, case in enumerate(vector["cases"]):
                cases += 1
                out = run_case(definition, case)
                lines.append(f"{rel}#{i}\t{out}\n")
                if not check_case(out, case):
                    failures.append(f"{rel}#{i}: {case['op']} {json.dumps(case.get('args'))} -> {out[:600]}")
        TRANSCRIPT.parent.mkdir(parents=True, exist_ok=True)
        with open(TRANSCRIPT, "w", encoding="utf-8", newline="\n") as f:
            f.writelines(lines)
        self.assertEqual(failures, [], f"{len(failures)} of {cases} cases failed")


if __name__ == "__main__":
    unittest.main()
