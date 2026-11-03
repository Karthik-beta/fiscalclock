"""Prints the CHANGELOG.md section for one version, for the GitHub release notes.

Usage: python -I scripts/release_notes.py 0.1.0. Exits with an error if the
section is missing or empty.
"""

import re
import sys
from pathlib import Path


def section(text, version):
    heading = re.compile(rf"^## \[{re.escape(version)}\][^\n]*\n", re.M)
    match = heading.search(text)
    if not match:
        return None
    rest = text[match.end():]
    end = re.search(r"^## \[|^\[[^\]]+\]: ", rest, re.M)
    return rest[: end.start() if end else len(rest)].strip()


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: python -I scripts/release_notes.py <version>, for example 0.1.0 or v0.1.0")
    version = sys.argv[1].removeprefix("v")
    notes = section(Path(__file__).resolve().parents[1].joinpath("CHANGELOG.md").read_text(encoding="utf-8"), version)
    if not notes:
        sys.exit(f"CHANGELOG.md has no section for {version}")
    sys.stdout.reconfigure(newline="\n")
    print(notes)


if __name__ == "__main__":
    main()
