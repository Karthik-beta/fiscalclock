"""Check that brand assets and design documents use only token values.

- Every colour in an SVG under assets/brand/ must be a palette primitive in
  design/tokens/generated/tokens.light.json. Hex colours of 3, 4, 6 and 8
  digits are read; a shorthand such as #f00 is expanded to #ff0000, and the
  alpha digits of a 4- or 8-digit colour are ignored, so the colour itself is
  what is checked.
- Every hex colour written in a Markdown file under design/ or in a wiki
  folder passed with --wiki must also be a palette primitive (six digits only
  there, because #123 in prose is an issue or anchor link). The research
  notes, the research brief, the explorations and the decision log are
  skipped: they record other sites' colours and rejected options.
- Every `--fc-*` name mentioned in those files must exist in the tokens.

Standard library only. Usage, from the repository root:

    python -I design/scripts/check_brand.py [--wiki path/to/fiscalclock.wiki]
    python -I design/scripts/check_brand.py --self-test
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HEX = re.compile(r"#[0-9a-fA-F]{6}\b")
SVG_HEX = re.compile(r"(?<![\w&])(?<!url\()(?<!href=\")(?<!href=')#([0-9a-fA-F]{3,8})\b")
VAR = re.compile(r"--fc-[a-z0-9-]+")
SKIP = {"research", "explorations"}
SKIP_FILES = {"research-brief.md", "DECISIONS.md", "Design-Decisions.md"}


def svg_colours(text: str) -> set[str]:
    """Hex colours in SVG text as lowercase #rrggbb, with shorthand expanded and any alpha dropped."""
    found = set()
    for digits in SVG_HEX.findall(text):
        if len(digits) in (3, 4):
            digits = "".join(ch * 2 for ch in digits)
        if len(digits) in (6, 8):
            found.add("#" + digits[:6].lower())
    return found


def self_test() -> int:
    cases = {
        '<rect fill="#f00" stroke="#FFF"/>': {"#ff0000", "#ffffff"},
        '<path fill="#1A1A19CC"/><path style="fill:#abcd"/>': {"#1a1a19", "#aabbcc"},
        '<use href="#abc"/><g fill="url(#bad)"/><circle fill="#12345"/>': set(),
    }
    failures = [text for text, want in cases.items() if svg_colours(text) != want]
    for text in failures:
        print(f"FAIL {text}: got {sorted(svg_colours(text))}")
    print(f"{len(cases) - len(failures)}/{len(cases)} SVG colour cases matched")
    return 1 if failures else 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--wiki", type=Path, help="a clone of the wiki to check as well")
    parser.add_argument("--self-test", action="store_true", help="check the SVG colour reader and exit")
    args = parser.parse_args(argv)
    if args.self_test:
        return self_test()
    tokens = json.loads((ROOT / "design/tokens/generated/tokens.light.json").read_text(encoding="utf-8"))
    palette = {v.lower() for k, v in tokens.items() if k.startswith("--fc-palette-")}
    names = set(tokens) | set(json.loads((ROOT / "design/tokens/generated/tokens.dark.json").read_text(encoding="utf-8")))
    problems = []

    for svg in sorted((ROOT / "assets/brand").rglob("*.svg")):
        for colour in sorted(svg_colours(svg.read_text(encoding="utf-8"))):
            if colour not in palette:
                problems.append(f"{svg.relative_to(ROOT).as_posix()}: {colour} is not a palette colour")

    docs = [p for p in (ROOT / "design").rglob("*.md") if not SKIP & set(p.relative_to(ROOT / "design").parts) and p.name not in SKIP_FILES]
    docs.append(ROOT / "design/tokens/generated/contrast.md")
    if args.wiki:
        docs += sorted(p for p in args.wiki.glob("*.md") if p.name not in SKIP_FILES)
    for doc in sorted(set(docs)):
        text = doc.read_text(encoding="utf-8")
        label = doc.relative_to(ROOT).as_posix() if doc.is_relative_to(ROOT) else f"wiki/{doc.name}"
        for colour in sorted(set(HEX.findall(text))):
            if colour.lower() not in palette:
                problems.append(f"{label}: {colour} is not a palette colour")
        for name in sorted(set(VAR.findall(text))):
            # Typography tokens are emitted as one variable per property.
            if name not in names and not any(n.startswith(name + "-") for n in names):
                problems.append(f"{label}: {name} is not a token")

    for line in problems:
        print(line)
    print(f"{len(problems)} problem(s)" if problems else "brand assets and documents use only token values")
    return 1 if problems else 0


if __name__ == "__main__":
    raise SystemExit(main())
