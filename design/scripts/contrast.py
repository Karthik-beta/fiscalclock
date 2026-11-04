"""Compute the WCAG 2.x contrast ratio between two sRGB hex colours.

Standard library only. The formulas follow the WCAG 2.2 definitions:

- relative luminance: https://www.w3.org/TR/WCAG22/#dfn-relative-luminance
- contrast ratio: https://www.w3.org/TR/WCAG22/#dfn-contrast-ratio

The sRGB linearisation threshold is 0.04045, as in the current WCAG 2.2 text
(the definition notes that versions before May 2021 used 0.03928).

Thresholds are compared against the unrounded ratio, as Understanding SC 1.4.3
requires ("4.499:1 would not meet the 4.5:1 threshold"):
https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html

The displayed ratio is truncated (rounded down) to two decimal places, so a
printed value never overstates the true ratio.

Usage:
    python -I design/scripts/contrast.py "#111111" "#ffffff"
    python -I design/scripts/contrast.py --self-test
"""

from __future__ import annotations

import argparse
import math
import re
import sys

LINEARISATION_THRESHOLD = 0.04045

# (name, minimum ratio) pairs from WCAG 2.2 SC 1.4.3, 1.4.6 and 1.4.11.
THRESHOLDS: tuple[tuple[str, float], ...] = (
    ("AA text (1.4.3)", 4.5),
    ("AA large text (1.4.3)", 3.0),
    ("AA non-text (1.4.11)", 3.0),
    ("AAA text (1.4.6)", 7.0),
    ("AAA large text (1.4.6)", 4.5),
)

# Reference values from WebAIM's contrast checker JSON API, retrieved 1 Nov 2026
# (append "&api" to a permalink, as https://webaim.org/resources/contrastchecker/
# documents), for example
# https://webaim.org/resources/contrastchecker/?fcolor=777777&bcolor=FFFFFF&api
# The values are kept as the strings WebAIM returned. WebAIM does not document
# its rounding; these values are consistent with truncation, so the check
# requires reference <= computed < reference + one unit in the last digit.
REFERENCES: tuple[tuple[str, str, str], ...] = (
    ("#777777", "#ffffff", "4.47"),
    ("#767676", "#ffffff", "4.54"),
    ("#949494", "#ffffff", "3.03"),
    ("#000000", "#ffffff", "21"),
    ("#e5e5e5", "#6221ea", "5.69"),
    ("#fff000", "#ffffff", "1.18"),
    ("#0066cc", "#ffffff", "5.56"),
    ("#121212", "#ffffff", "18.7"),
    ("#ffffff", "#ffffff", "1"),
)

_HEX = re.compile(r"^#?([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$")


def parse_hex(colour: str) -> tuple[int, int, int]:
    """Parse ``#rgb`` or ``#rrggbb`` (the ``#`` is optional) into 8-bit channels."""
    match = _HEX.match(colour.strip())
    if match is None:
        raise ValueError(f"not a hex colour: {colour!r}")
    digits = match.group(1)
    if len(digits) == 3:
        digits = "".join(ch * 2 for ch in digits)
    return (int(digits[0:2], 16), int(digits[2:4], 16), int(digits[4:6], 16))


def _linearise(channel_8bit: int) -> float:
    c = channel_8bit / 255
    if c <= LINEARISATION_THRESHOLD:
        return c / 12.92
    return ((c + 0.055) / 1.055) ** 2.4


def relative_luminance(colour: str) -> float:
    """Relative luminance of an sRGB colour, 0 for black and 1 for white."""
    r, g, b = (_linearise(v) for v in parse_hex(colour))
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def contrast_ratio(colour_a: str, colour_b: str) -> float:
    """WCAG 2.x contrast ratio (L1 + 0.05) / (L2 + 0.05); order does not matter."""
    la = relative_luminance(colour_a)
    lb = relative_luminance(colour_b)
    lighter, darker = max(la, lb), min(la, lb)
    return (lighter + 0.05) / (darker + 0.05)


def format_ratio(ratio: float) -> str:
    """Format a ratio truncated (never rounded up) to two decimal places."""
    truncated = math.floor(ratio * 100) / 100
    return f"{truncated:.2f}:1"


def self_test() -> int:
    """Compare against the published reference values; return a process exit code."""
    failures = 0
    for fg, bg, expected in REFERENCES:
        got = contrast_ratio(fg, bg)
        decimals = len(expected.partition(".")[2])
        unit = 10.0**-decimals
        low = float(expected)
        ok = low <= got < low + unit
        failures += not ok
        status = "ok  " if ok else "FAIL"
        print(f"{status} {fg} on {bg}: computed {got:.4f}, reference {expected}")
    print(f"{len(REFERENCES) - failures}/{len(REFERENCES)} reference values matched")
    return 1 if failures else 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="WCAG 2.x contrast ratio of two hex colours.")
    parser.add_argument("foreground", nargs="?", help="for example #111111")
    parser.add_argument("background", nargs="?", help="for example #ffffff")
    parser.add_argument("--self-test", action="store_true", help="check against reference values")
    args = parser.parse_args(argv)

    if args.self_test:
        return self_test()
    if args.foreground is None or args.background is None:
        parser.error("two colours are required unless --self-test is given")
    try:
        ratio = contrast_ratio(args.foreground, args.background)
    except ValueError as exc:
        parser.error(str(exc))

    print(f"{args.foreground} on {args.background}: {format_ratio(ratio)} (unrounded {ratio:.6f})")
    for name, minimum in THRESHOLDS:
        verdict = "pass" if ratio >= minimum else "fail"
        print(f"  {name:<24} >= {minimum:g}:1  {verdict}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
