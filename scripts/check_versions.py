"""Checks that every package and manifest states the same version.

With a tag argument (for example ``v0.1.0``), also checks that the tag names
that version. Standard library only; run with ``python -I scripts/check_versions.py``.
"""

import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def toml(path):
    return tomllib.loads((ROOT / path).read_text(encoding="utf-8"))


def main():
    found = {
        "crates/core/Cargo.toml": toml("crates/core/Cargo.toml")["package"]["version"],
        "bindings/python/Cargo.toml": toml("bindings/python/Cargo.toml")["package"]["version"],
        "bindings/python/pyproject.toml": toml("bindings/python/pyproject.toml")["project"]["version"],
        "bindings/js/Cargo.toml": toml("bindings/js/Cargo.toml")["package"]["version"],
        "bindings/js/package.json": json.loads((ROOT / "bindings/js/package.json").read_text(encoding="utf-8"))["version"],
        "bindings/js/package-lock.json": json.loads((ROOT / "bindings/js/package-lock.json").read_text(encoding="utf-8"))["version"],
    }
    init = (ROOT / "bindings/python/python/fiscalclock/__init__.py").read_text(encoding="utf-8")
    found["bindings/python/python/fiscalclock/__init__.py"] = re.search(r'^__version__ = "([^"]+)"', init, re.M).group(1)
    for path in ("bindings/python/Cargo.toml", "bindings/js/Cargo.toml"):
        pin = toml(path)["dependencies"]["fiscalclock"]["version"]
        found[f"{path} (fiscalclock dependency)"] = pin.removeprefix("=")
    if len(sys.argv) > 1:
        found["tag"] = sys.argv[1].removeprefix("v")
    for where, version in found.items():
        print(f"{version}  {where}")
    if len(set(found.values())) != 1:
        sys.exit("versions differ")


if __name__ == "__main__":
    main()
