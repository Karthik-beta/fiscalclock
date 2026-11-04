"""CI check: regenerate into a temporary directory, compare with the committed vectors, validate the schema.

Usage: python -I conformance/generators/check.py
Exit status 0 when the committed vectors equal a fresh generation and every file matches
conformance/schema/vector.schema.json; 1 otherwise.
"""

from __future__ import annotations

import json
import sys
import tempfile
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

import generate  # noqa: E402
import schema_check  # noqa: E402
from vectorlib import GROUPS  # noqa: E402

COMMITTED = HERE.parent / "vectors"
SCHEMA = HERE.parent / "schema" / "vector.schema.json"


def read(path: Path) -> bytes:
    return path.read_bytes().replace(b"\r\n", b"\n")


def main() -> int:
    problems = []
    with tempfile.TemporaryDirectory() as tmp:
        fresh = Path(tmp)
        generate.write_all(fresh, generate.build())
        for group in GROUPS:
            new = {p.name for p in (fresh / group).glob("*.json")}
            old = {p.name for p in (COMMITTED / group).glob("*.json")} if (COMMITTED / group).is_dir() else set()
            problems += [f"missing from commit: {group}/{n}" for n in sorted(new - old)]
            problems += [f"not produced by generate.py: {group}/{n}" for n in sorted(old - new)]
            problems += [f"differs from generate.py output: {group}/{n}" for n in sorted(new & old) if read(fresh / group / n) != read(COMMITTED / group / n)]
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    count = 0
    for group in GROUPS:
        for path in sorted((COMMITTED / group).glob("*.json")):
            count += 1
            doc = json.loads(path.read_text(encoding="utf-8"))
            problems += [f"{group}/{path.name}: {e}" for e in schema_check.errors(doc, schema, schema)]
            if doc.get("source", {}).get("kind") != group:
                problems.append(f"{group}/{path.name}: source.kind is not {group!r}")
            if doc.get("name") != path.stem:
                problems.append(f"{group}/{path.name}: name does not match file name")
    for p in problems:
        print(p)
    print(f"checked {count} vector files: {'OK' if not problems else f'{len(problems)} problem(s)'}")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
