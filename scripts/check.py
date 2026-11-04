"""Runs every check CI runs, from a fresh clone, and prints PASS or FAIL per command.

The commands are those in AGENTS.md "Build, test and lint", split into the groups
in GROUPS. Each command's output goes to target/check/logs/<name>.log, and the
last 20 lines of a failing log are printed. The python group builds the wheel in
its own virtual environment at target/check/venv, with the maturin version that
.github/workflows/ci.yml pins. Lint commands take their file lists from
git ls-files, so untracked copies are never linted. The transcripts group
compares the files the rust, python and js groups write in the same run, so
name those groups too.

Standard library only, and needs Python 3.11 or later. Run with
``python -I scripts/check.py [--list] [group ...]``. With no group it runs all of
them, and it exits 1 if any command failed.
"""

import argparse
import difflib
import filecmp
import functools
import os
import re
import shlex
import shutil
import subprocess
import sys
import time
from pathlib import Path
from typing import NamedTuple

if sys.version_info < (3, 11):
    sys.exit("check.py needs Python 3.11 or later")

ROOT = Path(__file__).resolve().parents[1]
CHECK = ROOT / "target" / "check"
VENV = CHECK / "venv"
VENV_PYTHON = VENV / ("Scripts/python.exe" if os.name == "nt" else "bin/python")
WHEELS = CHECK / "wheels"
TRANSCRIPTS = ROOT / "target" / "transcripts"
STARTED = time.time()

# cmd.exe, which runs npm's .cmd shims on Windows, refuses lines over 8191 characters.
ARGS_LIMIT = 6000
CHECK_JSONSCHEMA = "check-jsonschema==0.39.0"
YAMLLINT = "yamllint==1.38.0"
ZIZMOR = "zizmor==1.32.0"

HINTS = {
    "cargo": "install the Rust toolchain with rustup",
    "node": "install Node.js 24 or later",
    "npm": "install Node.js 24 or later",
    "uv": "install uv, for example with pip install uv",
    "uvx": "install uv, for example with pip install uv",
    "markdownlint-cli2": "npm install --global markdownlint-cli2",
    "cspell": "npm install --global cspell",
    "lychee": "cargo install lychee --locked",
}


def pinned(path, pattern):
    match = re.search(pattern, (ROOT / path).read_text(encoding="utf-8"), re.M)
    if not match:
        sys.exit(f"{path} no longer matches {pattern!r}; update scripts/check.py")
    return match.group(1)


MATURIN = pinned(".github/workflows/ci.yml", r'MATURIN_VERSION: "([^"]+)"')
WASM_BINDGEN = pinned("Cargo.lock", r'name = "wasm-bindgen"\nversion = "([^"]+)"')
MSRV = pinned("crates/core/Cargo.toml", r'^rust-version = "([^"]+)"')
PANDAS = pinned("conformance/generators/pandas_vectors.py", r'^PANDAS_VERSION = "([^"]+)"')


class Files(tuple):
    """The tracked files matching these git pathspecs; always the last argument."""


class Step(NamedTuple):
    """One command: an argument list, or a function that takes the log file and returns success."""

    name: str
    run: object
    desc: str = ""
    cwd: str = ""
    env: dict | None = None
    clean: Path | None = None
    gate: bool = False


def wasm_bindgen_version(log):
    exe = shutil.which("wasm-bindgen")
    found = subprocess.run([exe, "--version"], capture_output=True, text=True).stdout.split()[-1:] if exe else []
    print(f"Cargo.lock wants wasm-bindgen {WASM_BINDGEN}; installed: {found[0] if found else 'none'}", file=log)
    if found != [WASM_BINDGEN]:
        print(f"run: cargo install wasm-bindgen-cli --version {WASM_BINDGEN} --locked", file=log)
        return False
    return True


def node_version(log):
    exe = shutil.which("node")
    found = subprocess.run([exe, "--version"], capture_output=True, text=True).stdout.strip() if exe else ""
    print(f"installed Node.js: {found or 'none'}", file=log)
    if not re.match(r"v(\d+)", found) or int(found[1:].split(".")[0]) < 24:
        print("the website needs Node.js 24 or later", file=log)
        return False
    return True


def verify_claims(log):
    script = "research/verification/verify_claims.py"
    ran = subprocess.run([sys.executable, "-X", "utf8", "-I", script], cwd=ROOT, capture_output=True)
    log.write(ran.stderr.decode("utf-8", "replace"))
    got = ran.stdout.decode("utf-8").splitlines()
    want = (ROOT / "research/verification/expected_output.txt").read_text(encoding="utf-8").splitlines()
    diff = list(difflib.unified_diff(want, got, "expected_output.txt", script, lineterm=""))
    print("\n".join(diff), file=log)
    return ran.returncode == 0 and not diff


def transcripts_match(log, language):
    paths = {name: TRANSCRIPTS / f"{name}.txt" for name in ("rust", language)}
    for name, path in paths.items():
        if not path.exists() or path.stat().st_mtime < STARTED:
            print(f"{path.relative_to(ROOT).as_posix()} was not written by this run; "
                  f"run: python -I scripts/check.py rust {language} transcripts", file=log)
            return False
    same = filecmp.cmp(paths["rust"], paths[language], shallow=False)
    print(f"rust.txt and {language}.txt {'match' if same else 'differ'}", file=log)
    return same


GROUPS = {
    "rust": [
        Step("fmt", ["cargo", "fmt", "--all", "--check"]),
        Step("clippy", ["cargo", "clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"]),
        Step("test", ["cargo", "test", "--release", "-p", "fiscalclock", "--locked"]),
        Step("json_api", ["cargo", "test", "-p", "fiscalclock", "--locked", "--test", "json_api"]),
        Step("doc", ["cargo", "doc", "--no-deps", "-p", "fiscalclock"], env={"RUSTDOCFLAGS": "-D warnings"}),
        Step("wasm_build", ["cargo", "build", "-p", "fiscalclock", "--target", "wasm32-unknown-unknown", "--release"]),
        Step("package", ["cargo", "package", "-p", "fiscalclock", "--locked"]),
        Step("msrv", ["cargo", f"+{MSRV}", "check", "-p", "fiscalclock"]),
    ],
    "python": [
        Step("py_venv", [sys.executable, "-m", "venv", VENV]),
        Step("py_maturin", [VENV_PYTHON, "-m", "pip", "install", f"maturin=={MATURIN}"]),
        Step(
            "py_wheel",
            [VENV_PYTHON, "-m", "maturin", "build", "--release", "--locked", "--out", WHEELS,
             "--manifest-path", "bindings/python/Cargo.toml", "--interpreter", VENV_PYTHON],
            clean=WHEELS,
        ),
        Step(
            "py_install",
            [VENV_PYTHON, "-m", "pip", "install", "--force-reinstall", "--no-deps", "--no-index",
             "--find-links", WHEELS, "fiscalclock"],
        ),
        Step("py_tests", [VENV_PYTHON, "-m", "unittest", "discover", "-s", "bindings/python/tests"]),
    ],
    "js": [
        Step("wasm_bindgen_version", wasm_bindgen_version, "wasm-bindgen --version equals the version in Cargo.lock", gate=True),
        Step("js_install", ["npm", "ci"], cwd="bindings/js"),
        Step("js_build", ["npm", "run", "build"], cwd="bindings/js"),
        Step("js_tests", ["npm", "test"], cwd="bindings/js"),
    ],
    "transcripts": [
        Step("transcript_python", functools.partial(transcripts_match, language="python"),
             "target/transcripts/rust.txt equals target/transcripts/python.txt"),
        Step("transcript_js", functools.partial(transcripts_match, language="js"),
             "target/transcripts/rust.txt equals target/transcripts/js.txt"),
    ],
    "evidence": [
        Step("verify_claims", verify_claims, "verify_claims.py output equals expected_output.txt"),
        Step("generators", [sys.executable, "-I", "conformance/generators/check.py"]),
        Step("pandas_vectors", ["uv", "run", "--no-project", "--with", f"pandas=={PANDAS}",
                                "python", "-I", "conformance/generators/pandas_vectors.py", "--check"]),
        Step("quotes", [sys.executable, "-I", "research/evidence/check_quotes.py", "--offline"]),
        Step("render_evidence", [sys.executable, "-I", "research/evidence/render_evidence.py", "--check"]),
        Step("versions", [sys.executable, "-I", "scripts/check_versions.py"]),
    ],
    "website": [
        Step("node_version", node_version, "node --version is 24 or later", gate=True),
        Step("site_install", ["npm", "ci", "--ignore-scripts"], cwd="website"),
        Step("site_build", ["npm", "run", "build"], cwd="website"),
        Step("site_check", ["npm", "run", "check"], cwd="website"),
        Step("site_tests", ["npm", "test"], cwd="website"),
        Step("site_examples", ["npm", "run", "test:examples"], cwd="website"),
        Step("site_links", ["lychee", "--config", "website/lychee.toml", "--no-progress",
                            "--root-dir", ROOT / "website" / "dist", "website/dist/**/*.html"]),
    ],
    "docs": [
        Step("markdownlint", ["markdownlint-cli2", "--no-globs", Files(("*.md",))]),
        Step("cspell", ["cspell", "--no-progress", "--config", "cspell.json",
                        Files(("*.md", "*.mdx", "*.yml", "*.yaml", "*.toml"))]),
        Step("yamllint", ["uvx", YAMLLINT, "--strict", Files(("*.yml", "*.yaml"))]),
        Step("local_links", ["lychee", "--config", "lychee.toml", "--no-progress", "--offline", Files(("*.md",))]),
    ],
    "links": [
        Step("lychee_md", ["lychee", "--config", "lychee.toml", "--no-progress", Files(("*.md",))]),
        Step("lychee_mdx", ["lychee", "--config", "lychee.toml", "--no-progress",
                            "--base-url", "https://fiscalclock.karthik.nz/", Files(("*.mdx",))]),
    ],
    "workflows": [
        Step("schema_issue_forms", ["uvx", CHECK_JSONSCHEMA, "--builtin-schema", "vendor.github-issue-forms",
                                    Files((".github/ISSUE_TEMPLATE/*.yml", ":(exclude).github/ISSUE_TEMPLATE/config.yml"))]),
        Step("schema_issue_config", ["uvx", CHECK_JSONSCHEMA, "--builtin-schema", "vendor.github-issue-config",
                                     ".github/ISSUE_TEMPLATE/config.yml"]),
        Step("schema_workflows", ["uvx", CHECK_JSONSCHEMA, "--builtin-schema", "vendor.github-workflows",
                                  Files((".github/workflows/*.yml",))]),
        Step("schema_discussions", ["uvx", CHECK_JSONSCHEMA, "--builtin-schema", "vendor.github-discussion",
                                    Files((".github/DISCUSSION_TEMPLATE/*.yml",))]),
        Step("schema_dependabot", ["uvx", CHECK_JSONSCHEMA, "--builtin-schema", "vendor.dependabot",
                                   ".github/dependabot.yml"]),
        Step("zizmor", ["uvx", ZIZMOR, "--offline", ".github/workflows"]),
    ],
}


def show(argument):
    if isinstance(argument, Files):
        return f"<tracked {' '.join(argument)}>"
    if argument == sys.executable:
        return "python"
    if isinstance(argument, Path) and argument.is_relative_to(ROOT):
        return argument.relative_to(ROOT).as_posix()
    return str(argument)


def tracked(patterns):
    listed = subprocess.run(["git", "ls-files", "-z", "--", *patterns], cwd=ROOT, capture_output=True, check=True)
    names = listed.stdout.decode("utf-8").split("\0")
    return [name for name in names if name and (ROOT / name).exists()]


def command_lines(argv):
    *head, last = argv
    if not isinstance(last, Files):
        return [[str(argument) for argument in argv]]
    head = [str(argument) for argument in head]
    lines, chunk, size = [], [], 0
    for name in tracked(last):
        if chunk and size + len(name) + 1 > ARGS_LIMIT:
            lines.append(head + chunk)
            chunk, size = [], 0
        chunk.append(name)
        size += len(name) + 1
    return lines + [head + chunk] if chunk else lines


def run_command(step, log):
    if step.clean:
        shutil.rmtree(step.clean, ignore_errors=True)
    env = {**os.environ, **(step.env or {})}
    for argv in command_lines(step.run):
        print("$", shlex.join(argv), file=log, flush=True)
        exe = shutil.which(argv[0])
        if exe is None:
            print(f"{argv[0]} not found: {HINTS.get(argv[0], 'install it and put it on PATH')}", file=log)
            return False
        ran = subprocess.run([exe, *argv[1:]], cwd=ROOT / step.cwd, env=env,
                             stdin=subprocess.DEVNULL, stdout=log, stderr=subprocess.STDOUT)
        if ran.returncode:
            return False
    return True


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("groups", nargs="*", metavar="group", help="one or more of: " + ", ".join(GROUPS))
    parser.add_argument("--list", action="store_true", help="print each group and its commands without running them")
    args = parser.parse_args()
    unknown = [name for name in args.groups if name not in GROUPS]
    if unknown:
        parser.error(f"unknown group {', '.join(unknown)}; valid groups: {', '.join(GROUPS)}")
    chosen = [group for group in GROUPS if not args.groups or group in args.groups]

    if args.list:
        for group in chosen:
            print(group)
            for step in GROUPS[group]:
                where = f"cd {step.cwd} && " if step.cwd else ""
                where += "".join(f"{key}='{value}' " for key, value in (step.env or {}).items())
                what = step.desc if callable(step.run) else " ".join(show(argument) for argument in step.run)
                print(f"  {step.name}: {where}{what}")
        return 0

    logs = CHECK / "logs"
    logs.mkdir(parents=True, exist_ok=True)
    results = []
    for group in chosen:
        blocked = None
        for step in GROUPS[group]:
            path = (logs / f"{step.name}.log").relative_to(ROOT).as_posix()
            if blocked:
                results.append(f"FAIL {step.name} (skipped, see {blocked})")
                continue
            print(f"running {step.name}", flush=True)
            with open(ROOT / path, "w", encoding="utf-8") as log:
                try:
                    ok = step.run(log) if callable(step.run) else run_command(step, log)
                except OSError as error:
                    print(error, file=log)
                    ok = False
            if ok:
                results.append(f"PASS {step.name}")
                continue
            results.append(f"FAIL {step.name} ({path})")
            lines = (ROOT / path).read_text(encoding="utf-8", errors="replace").splitlines()
            print(f"--- {step.name} failed; last 20 lines of {path}")
            print("\n".join(lines[-20:]))
            if step.gate:
                blocked = path
    print("\n".join(results))
    return int(any(line.startswith("FAIL") for line in results))


if __name__ == "__main__":
    sys.exit(main())
