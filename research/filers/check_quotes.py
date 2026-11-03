"""Check the quotes in filer manifests against the annual-report PDFs they cite.

Usage: python -I research/filers/check_quotes.py [--pdfs DIR] research/filers/quotes/*.json

Each manifest names a PDF, its URL and its SHA-256. A PDF missing from DIR (default: a
pdfs/ folder next to this script) is downloaded from its URL. The PDF's SHA-256 must match,
and every quote must appear on its stated PDF page in the text that pdftotext -enc UTF-8
extracts, with or without -layout, after runs of whitespace are collapsed to one space.
Needs pdftotext (Poppler). Exit status 1 if anything fails.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
import urllib.request
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent


def squash(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()


def page_text(pdf: Path, page: int) -> str:
    # -layout interleaves the columns of a two-column page, so plain reading order is checked too.
    texts = []
    for mode in (["-layout"], []):
        out = subprocess.run(
            ["pdftotext", *mode, "-enc", "UTF-8", "-f", str(page), "-l", str(page), str(pdf), "-"],
            capture_output=True, check=True,
        ).stdout.decode("utf-8")
        texts.append(squash(out))
    return "\n".join(texts)


def fetch(url: str, dest: Path) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    request = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
    with urllib.request.urlopen(request, timeout=120) as response:
        dest.write_bytes(response.read())


def check(manifest_path: Path, pdfs: Path) -> list[str]:
    m = json.loads(manifest_path.read_text(encoding="utf-8"))
    pdf = pdfs / m["pdf"]
    if not pdf.exists():
        fetch(m["url"], pdf)
    digest = hashlib.sha256(pdf.read_bytes()).hexdigest()
    if digest != m["sha256"]:
        return [f"sha256 of {pdf.name} is {digest}, manifest says {m['sha256']}"]
    problems = []
    pages: dict[int, str] = {}
    for i, q in enumerate(m["quotes"], 1):
        n = q["pdf_page"]
        if n not in pages:
            pages[n] = page_text(pdf, n)
        if squash(q["text"]) not in pages[n]:
            problems.append(f"quote {i} not on PDF page {n}: {q['text'][:80]!r}")
    return problems


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pdfs", type=Path, default=HERE / "pdfs")
    parser.add_argument("manifests", nargs="+", type=Path)
    args = parser.parse_args()
    failed = False
    for path in args.manifests:
        problems = check(path, args.pdfs)
        count = len(json.loads(path.read_text(encoding="utf-8"))["quotes"])
        print(f"{path.stem}: {count} quotes, {'OK' if not problems else f'{len(problems)} problem(s)'}")
        for p in problems:
            print(f"  {p}")
        failed |= bool(problems)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
