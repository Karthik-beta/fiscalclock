"""Candidate palettes for the three Phase 3 directions, with WCAG 2.2 checks.

Run from the repository root: python -I design/explorations/check_palettes.py
"""
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from contrast import contrast_ratio, format_ratio  # noqa: E402

DIRECTIONS = {
    "A Ledger": {
        "light": dict(bg="#F7F9F8", surface="#EDF1EF", text="#14201B", text2="#46534D", rule="#CFD8D3", control="#7E8A84",
                      accent="#1D5C3F", focus="#1D5C3F", success="#1D6B3A", warning="#8A5A00", danger="#B3261E", info="#24508F"),
        "dark": dict(bg="#101614", surface="#17201C", text="#E3EAE6", text2="#A2AFA9", rule="#2B3631", control="#6E7B75",
                     accent="#7FC8A3", focus="#7FC8A3", success="#7CCB92", warning="#E7B45A", danger="#FF8A80", info="#8DB4F2"),
    },
    "B Grid": {
        "light": dict(bg="#FFFFFF", surface="#F4F5F7", text="#14171C", text2="#4A515D", rule="#D8DCE2", control="#7A818C",
                      accent="#1F4C86", focus="#1F4C86", success="#17703F", warning="#8A5800", danger="#B42318", info="#1F4C86"),
        "dark": dict(bg="#101216", surface="#171A20", text="#E5E8EC", text2="#A0A7B2", rule="#2A2F38", control="#6F7783",
                     accent="#8EB4EE", focus="#8EB4EE", success="#6FCF97", warning="#EDB95E", danger="#FF8B7E", info="#8EB4EE"),
    },
    "C Interval": {
        "light": dict(bg="#FDFDFC", surface="#F2F2F0", text="#191918", text2="#555552", rule="#DDDDD9", control="#7D7D79",
                      accent="#0B6B69", focus="#0B6B69", success="#1C6E3D", warning="#8C5600", danger="#B4261F", info="#1F55A0"),
        "dark": dict(bg="#121212", surface="#1A1A19", text="#ECECEA", text2="#A5A5A1", rule="#2D2D2B", control="#71716D",
                     accent="#5CC6BE", focus="#5CC6BE", success="#73CC93", warning="#E9B65C", danger="#FF8A80", info="#8CB2F0"),
    },
}

# (foreground role, background role, minimum ratio, criterion)
PAIRS = [
    ("text", "bg", 4.5, "1.4.3 text"),
    ("text", "surface", 4.5, "1.4.3 text"),
    ("text2", "bg", 4.5, "1.4.3 text"),
    ("text2", "surface", 4.5, "1.4.3 text"),
    ("accent", "bg", 4.5, "1.4.3 link text"),
    ("accent", "surface", 4.5, "1.4.3 link text"),
    ("control", "bg", 3.0, "1.4.11 control border"),
    ("control", "surface", 3.0, "1.4.11 control border"),
    ("focus", "bg", 3.0, "1.4.11 focus"),
    ("focus", "surface", 3.0, "1.4.11 focus"),
    ("success", "bg", 4.5, "1.4.3 text"),
    ("warning", "bg", 4.5, "1.4.3 text"),
    ("danger", "bg", 4.5, "1.4.3 text"),
    ("info", "bg", 4.5, "1.4.3 text"),
    ("success", "surface", 4.5, "1.4.3 text"),
    ("warning", "surface", 4.5, "1.4.3 text"),
    ("danger", "surface", 4.5, "1.4.3 text"),
    ("info", "surface", 4.5, "1.4.3 text"),
]

BADGE_RULE = re.compile(r"\.badge span:(first|last)-child \{ background: (#[0-9a-fA-F]{3,6}); color: (#[0-9a-fA-F]{3,6}); \}")


def badge_pairs():
    """The two badge colour pairs that brand-directions.html draws, read from its stylesheet."""
    css = Path(__file__).with_name("brand-directions.html").read_text(encoding="utf-8")
    pairs = {m.group(1): (m.group(3), m.group(2)) for m in BADGE_RULE.finditer(css)}
    if set(pairs) != {"first", "last"}:
        sys.exit("brand-directions.html: badge colour rules not found")
    return pairs


failures = 0
for name, themes in DIRECTIONS.items():
    for theme, p in themes.items():
        rows = []
        for fg, bg, need, crit in PAIRS:
            r = contrast_ratio(p[fg], p[bg])
            ok = r >= need
            failures += not ok
            rows.append(f"  {'ok  ' if ok else 'FAIL'} {fg:8} {p[fg]} on {bg:8} {p[bg]}  {format_ratio(r):>8}  (>= {need}, {crit})")
        # Button: bg-coloured text on the accent fill.
        r = contrast_ratio(p["bg"], p["accent"])
        failures += r < 4.5
        rows.append(f"  {'ok  ' if r >= 4.5 else 'FAIL'} bg-on-accent button text             {format_ratio(r):>8}  (>= 4.5)")
        print(f"{name} / {theme}")
        print("\n".join(rows))
for part, (fg, bg) in badge_pairs().items():
    r = contrast_ratio(fg, bg)
    failures += r < 4.5
    print(f"{'ok  ' if r >= 4.5 else 'FAIL'} README badge {part}-child text {fg} on {bg}  {format_ratio(r):>8}  (>= 4.5, 1.4.3 text)")
print("failures:", failures)
sys.exit(1 if failures else 0)
