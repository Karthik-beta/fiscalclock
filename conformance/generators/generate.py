"""Rewrite every generated vector file under conformance/vectors/.

Usage: python -I conformance/generators/generate.py [--out DIR]
Standard library only; deterministic; no network; does not import fiscalclock.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

import error_vectors  # noqa: E402
import oracle_vectors  # noqa: E402
import spec_cited_vectors  # noqa: E402
from vectorlib import write_all  # noqa: E402

DEFAULT_OUT = HERE.parent / "vectors"


def build() -> dict[str, list[dict]]:
    return {
        "oracle": oracle_vectors.build(),
        "errors": error_vectors.build(),
        "spec": spec_cited_vectors.spec_vectors(),
        "cited": spec_cited_vectors.cited_vectors(),
    }


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, default=DEFAULT_OUT)
    args = ap.parse_args(argv)
    groups = build()
    written = write_all(args.out, groups)
    for group, vectors in groups.items():
        print(f"{group}: {len(vectors)} files, {sum(len(v['cases']) for v in vectors)} cases")
    print(f"wrote {len(written)} files to {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
