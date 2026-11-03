"""Checks that every package and manifest states the same version, and that
each pinned tool version agrees everywhere it is written.

With a tag argument (for example ``v0.1.0``), also checks that the tag names
that version. The pinned tools are the minimum supported Rust version, maturin,
wasm-bindgen, pandas and the release the website runs; PINS lists every
file that states one.

Standard library only, and needs Python 3.11 or later (``tomllib``). Run with
``python -I scripts/check_versions.py``.
"""

import json
import re
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    sys.exit("check_versions.py needs Python 3.11 or later (tomllib)")

ROOT = Path(__file__).resolve().parents[1]

CARGO_MSRV = r'^rust-version = "([^"]+)"'
RUST_AND_LATER = r"Rust (\d+\.\d+) and later"
MATURIN_ENV = r'MATURIN_VERSION: "([^"]+)"'
WASM_BINDGEN_ENV = r'WASM_BINDGEN_VERSION: "([^"]+)"'
PANDAS_PIN = r"pandas==(\d+\.\d+\.\d+)"

# Each pin maps to (file, pattern) pairs. The pattern's one group is the
# version, and it must match at least once in the file.
PINS = {
    "minimum supported Rust version": [
        ("crates/core/Cargo.toml", CARGO_MSRV),
        ("bindings/python/Cargo.toml", CARGO_MSRV),
        ("bindings/js/Cargo.toml", CARGO_MSRV),
        (".github/workflows/ci.yml", r'toolchain: "(\d+\.\d+)"'),
        ("AGENTS.md", r"cargo \+(\d+\.\d+) check"),
        ("README.md", RUST_AND_LATER),
        ("crates/core/README.md", r"Minimum supported Rust version: (\d+\.\d+)\."),
        ("website/src/content/docs/start-here/install.mdx", RUST_AND_LATER),
        ("website/src/content/docs/index.mdx", RUST_AND_LATER),
    ],
    "maturin": [
        (".github/workflows/ci.yml", MATURIN_ENV),
        (".github/workflows/release.yml", MATURIN_ENV),
        ("AGENTS.md", r"needs maturin (\d+\.\d+\.\d+)"),
    ],
    "wasm-bindgen": [
        (".github/workflows/ci.yml", WASM_BINDGEN_ENV),
        (".github/workflows/release.yml", WASM_BINDGEN_ENV),
        ("bindings/js/Cargo.toml", r'^wasm-bindgen = "=([^"]+)"'),
        ("Cargo.lock", r'name = "wasm-bindgen"\nversion = "([^"]+)"'),
    ],
    "pandas": [
        ("conformance/generators/pandas_vectors.py", r'(?:^PANDAS_VERSION = "|pandas==)(\d+\.\d+\.\d+)'),
        ("conformance/generators/README.md", PANDAS_PIN),
        (".github/workflows/docs.yml", PANDAS_PIN),
    ],
    # The site runs the published packages, so these may lag the version above,
    # but every language's examples must run the same release.
    "fiscalclock release the website runs": [
        ("website/package.json", r'^    "fiscalclock": "([^"]+)"'),
        ("website/package-lock.json", r'"node_modules/fiscalclock": \{\n\s+"version": "([^"]+)"'),
        ("website/examples/python/requirements.txt", r"^fiscalclock==(\S+)$"),
        ("website/examples/rust/Cargo.toml", r'^fiscalclock = "=([^"]+)"'),
        ("website/examples/rust/Cargo.lock", r'name = "fiscalclock"\nversion = "([^"]+)"'),
    ],
}


def toml(path):
    return tomllib.loads((ROOT / path).read_text(encoding="utf-8"))


def pin_versions(pin, places):
    found = {}
    for path, pattern in places:
        text = (ROOT / path).read_text(encoding="utf-8")
        versions = re.findall(pattern, text, re.M)
        if not versions:
            sys.exit(f"{pin}: {path} no longer matches {pattern!r}; update PINS in scripts/check_versions.py")
        for number, version in enumerate(versions, 1):
            found[f"{path} ({number})" if len(versions) > 1 else path] = version
    return found


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
    declared = re.search(r'^__version__ = "([^"]+)"', init, re.M)
    if not declared:
        sys.exit('bindings/python/python/fiscalclock/__init__.py has no line of the form __version__ = "x.y.z"')
    found["bindings/python/python/fiscalclock/__init__.py"] = declared.group(1)
    for path in ("bindings/python/Cargo.toml", "bindings/js/Cargo.toml"):
        pin = toml(path)["dependencies"]["fiscalclock"]["version"]
        found[f"{path} (fiscalclock dependency)"] = pin.removeprefix("=")
    if len(sys.argv) > 1:
        found["tag"] = sys.argv[1].removeprefix("v")
    for where, version in found.items():
        print(f"{version}  {where}")
    problems = ["versions differ"] if len(set(found.values())) != 1 else []
    for pin, places in PINS.items():
        pinned = pin_versions(pin, places)
        print(f"{pin}:")
        for where, version in pinned.items():
            print(f"{version}  {where}")
        if len(set(pinned.values())) != 1:
            problems.append(f"{pin} differs: " + ", ".join(f"{version} in {where}" for where, version in pinned.items()))
    if problems:
        sys.exit("\n".join(problems))


if __name__ == "__main__":
    main()
