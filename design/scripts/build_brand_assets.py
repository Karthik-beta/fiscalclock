"""Build fiscalclock's logo, icons, cards and README banner into assets/brand/.

Every shape is drawn from the geometry below; every word is IBM Plex Sans,
converted to outlines so that no file depends on an installed font. Colours
are read from the generated design tokens (design/tokens/build/), so the
assets cannot drift from the token files.

Requirements (not needed for anything else in the repository):

- Python packages pinned in design/scripts/requirements-brand.txt
  (fontTools, uharfbuzz, Pillow);
- IBM Plex Sans, variable version, from Google Fonts' repository
  (https://github.com/google/fonts/tree/main/ofl/ibmplexsans), SIL Open Font
  License 1.1; the script checks its SHA-256;
- Inkscape 1.4 (https://inkscape.org), which renders every PNG.

Usage, from the repository root:

    python design/scripts/build_brand_assets.py --font "path/to/IBMPlexSans[wdth,wght].ttf"
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import math
import shutil
import subprocess
import sys
from pathlib import Path

import uharfbuzz as hb
from fontTools.pens.svgPathPen import SVGPathPen
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets" / "brand"
FONT_SHA256 = "3b031aa4216174205bd8471f88a49b91f093169e9e87bd5262242bc5967fe2e3"
WORDMARK = "fiscalclock"


def tokens(theme: str) -> dict[str, str]:
    return json.loads((ROOT / "design" / "tokens" / "build" / f"tokens.{theme}.json").read_text(encoding="utf-8"))


LIGHT, DARK = tokens("light"), tokens("dark")
INK = {"light": LIGHT["--fc-color-logo-ink"], "dark": DARK["--fc-color-logo-ink"]}
OPEN = {"light": LIGHT["--fc-color-logo-open-end"], "dark": DARK["--fc-color-logo-open-end"]}
BG = {"light": LIGHT["--fc-color-bg-default"], "dark": DARK["--fc-color-bg-default"]}
TILE = DARK["--fc-color-bg-subtle"]  # app icons: dark-theme colours on the dark subtle surface
WHITE = LIGHT["--fc-palette-neutral-0"]


def num(v: float) -> str:
    """Format a number for SVG with up to five decimals, so small scale factors keep their precision."""
    return f"{v:.5f}".rstrip("0").rstrip(".") if v != int(v) else str(int(v))


# Symbol geometry -----------------------------------------------------------
# The symbol is the half-open interval [ ): a closed square bracket (the start
# is included) and an open parenthesis (the end is excluded). The bracket is
# straight lines on a grid. The parenthesis is the band between two
# right-hand half-ellipses, cut flat at the bracket's top and bottom, so its
# stroke tapers slightly towards the ends like a typeset parenthesis.

def paren(cx: float, cy: float, rxo: float, ryo: float, rxi: float, ryi: float, top: float, bottom: float) -> str:
    def x_at(rx: float, ry: float, y: float) -> float:
        return cx + rx * math.sqrt(max(0.0, 1 - ((y - cy) / ry) ** 2))
    xo_t, xo_b, xi_t, xi_b = x_at(rxo, ryo, top), x_at(rxo, ryo, bottom), x_at(rxi, ryi, top), x_at(rxi, ryi, bottom)
    return (f"M{num(xi_t)} {num(top)}H{num(xo_t)}A{num(rxo)} {num(ryo)} 0 0 1 {num(xo_b)} {num(bottom)}"
            f"H{num(xi_b)}A{num(rxi)} {num(ryi)} 0 0 0 {num(xi_t)} {num(top)}Z")


SYMBOLS = {
    # Master drawing, 64-unit grid, for 24 px and larger. Stroke 8 units.
    "master": (64, "M11 6H29V14H19V50H29V58H11Z", paren(29, 32, 24, 34, 16, 28, 6, 58)),
    # Small drawing, 16-unit grid, for 16 to 23 px. Stroke 2 px on whole pixels.
    "small": (16, "M2 1H7V3H4V13H7V15H2Z", paren(7, 8, 7, 9.5, 5, 8, 1, 15)),
    # Inside a 16 px tile (favicon.ico): 12 px tall, 2 px strokes.
    "tile16": (16, "M3 2H7V4H5V12H7V14H3Z", paren(7.5, 8, 5.5, 7.5, 3.5, 6.25, 2, 14)),
}


def symbol_paths(kind: str, ink: str, open_end: str, transform: str = "") -> str:
    _, bracket, par = SYMBOLS[kind]
    t = f' transform="{transform}"' if transform else ""
    return f'<g{t}><path fill="{ink}" d="{bracket}"/><path fill="{open_end}" d="{par}"/></g>'


# Text as outlines ------------------------------------------------------------

class Type:
    def __init__(self, font_path: Path):
        data = font_path.read_bytes()
        digest = hashlib.sha256(data).hexdigest()
        if digest != FONT_SHA256:
            sys.exit(f"unexpected font file (SHA-256 {digest}); expected {FONT_SHA256}")
        self.face = hb.Face(data)
        self.upem = self.face.upem

    def path(self, text: str, size: float, weight: int, x: float, baseline: float, tracking: float = 0) -> tuple[str, float]:
        """Return (SVG path data, advance width in px). Tracking is in font units per glyph."""
        font = hb.Font(self.face)
        font.set_variations({"wght": weight, "wdth": 100})
        buf = hb.Buffer()
        buf.add_str(text)
        buf.guess_segment_properties()
        hb.shape(font, buf, {"kern": True, "liga": True})
        scale = size / self.upem
        parts, pen_x = [], 0.0
        for info, pos in zip(buf.glyph_infos, buf.glyph_positions):
            pen = SVGPathPen(None, ntos=lambda v: num(round(v, 2)))
            font.draw_glyph_with_pen(info.codepoint, pen)
            d = pen.getCommands()
            if d:
                gx = x + (pen_x + pos.x_offset) * scale
                parts.append(f'<path transform="translate({num(round(gx, 2))} {num(round(baseline, 2))}) scale({num(round(scale, 5))} {num(round(-scale, 5))})" d="{d}"/>')
            pen_x += pos.x_advance + tracking
        width = (pen_x - tracking) * scale
        return "".join(parts), width


# Logo files -----------------------------------------------------------------
# Lockup proportions, in units of the wordmark's font size (em):
# the bracket spans 0.82 em, overhanging the ascenders of f, l and k (0.74 em)
# and the baseline by 0.04 em each, so its stroke (0.126 em) is a little
# heavier than the letters' stems (about 0.108 em); the gap from the symbol to
# the wordmark is 0.28 em.
WM_TRACK = -10            # font units per glyph
BRACKET_EM = 0.82
BELOW_EM = 0.04
GAP_EM = 0.28


def svg(width: float, height: float, body: str, title: str, extra: str = "") -> str:
    return (f'<svg xmlns="http://www.w3.org/2000/svg" width="{num(width)}" height="{num(height)}" '
            f'viewBox="0 0 {num(width)} {num(height)}" role="img" aria-labelledby="t">'
            f'<title id="t">{title}</title>{extra}{body}</svg>\n')


def lockup_body(t: Type, x: float, baseline: float, size: float, ink: str, open_end: str) -> tuple[str, float]:
    """Symbol and wordmark with the wordmark's baseline at `baseline`. Returns (body, total width)."""
    grid, _, _ = SYMBOLS["master"]
    s = BRACKET_EM * size / 52          # px per grid unit; the bracket is 52 units tall
    sym_top = baseline + BELOW_EM * size - 58 * s
    body = symbol_paths("master", ink, open_end, f"translate({num(round(x - 11 * s, 3))} {num(round(sym_top, 3))}) scale({num(round(s, 5))})")
    sym_right = x + 42 * s              # the symbol spans x = 11 to 53 on its grid
    words, width = t.path(WORDMARK, size, 600, sym_right + GAP_EM * size, baseline, WM_TRACK)
    return body + f'<g fill="{ink}">{words}</g>', sym_right + GAP_EM * size + width - x


def build_logos(t: Type) -> dict[str, str]:
    files = {}
    title = "fiscalclock"
    variants = {"": ("light", None), "-dark": ("dark", None), "-mono": ("light", "mono"), "-mono-white": ("dark", "white")}
    for suffix, (theme, mono) in variants.items():
        ink = WHITE if mono == "white" else INK[theme]
        open_end = ink if mono else OPEN[theme]
        # Symbol, master drawing.
        files[f"logo/fiscalclock-symbol{suffix}.svg"] = svg(64, 64, symbol_paths("master", ink, open_end), title)
        # Symbol, small drawing.
        files[f"logo/fiscalclock-symbol-small{suffix}.svg"] = svg(16, 16, symbol_paths("small", ink, open_end), title)
        # Wordmark.
        size = 100
        words, width = t.path(WORDMARK, size, 600, 0, 74, WM_TRACK)
        files[f"logo/fiscalclock-wordmark{suffix}.svg"] = svg(round(width, 2), 76, f'<g fill="{ink}">{words}</g>', title)
        # Lockup: tight box from the bracket top to its bottom.
        top = (BRACKET_EM - BELOW_EM) * size
        body, lw = lockup_body(t, 0, top, size, ink, open_end)
        files[f"logo/fiscalclock-lockup{suffix}.svg"] = svg(round(lw, 2), BRACKET_EM * size, body, title)
    return files


# Icons ----------------------------------------------------------------------

def build_icons() -> dict[str, str]:
    files = {}
    ink_l, open_l, ink_d, open_d = INK["light"], OPEN["light"], INK["dark"], OPEN["dark"]
    style = (f'<style>.i{{fill:{ink_l}}}.o{{fill:{open_l}}}'
             f'@media (prefers-color-scheme: dark){{.i{{fill:{ink_d}}}.o{{fill:{open_d}}}}}</style>')
    _, bracket, par = SYMBOLS["small"]
    files["icons/favicon.svg"] = svg(16, 16, f'<path class="i" d="{bracket}"/><path class="o" d="{par}"/>', "fiscalclock", style)
    # Tiles: dark-theme colours on the dark subtle surface. Used where the
    # background behind the icon is unknown (app icons, docs.rs, registries).
    def tile(size: int, radius: float, inset: float) -> str:
        s = (size - 2 * inset) / 42                # symbol width is 42 grid units
        top = (size - 52 * s) / 2 - 6 * s
        left = inset - 11 * s
        body = (f'<rect width="{size}" height="{size}" rx="{num(radius)}" fill="{TILE}"/>'
                + symbol_paths("master", ink_d, open_d, f"translate({num(round(left, 3))} {num(round(top, 3))}) scale({num(round(s, 5))})"))
        return svg(size, size, body, "fiscalclock")
    files["icons/icon-tile.svg"] = tile(512, 112, 136)           # rounded tile for 'any' purpose icons
    files["icons/icon-maskable.svg"] = tile(512, 0, 168)         # full bleed; symbol inside the central safe zone
    files["icons/docsrs-logo.svg"] = tile(64, 14, 17)
    _, b16, p16 = SYMBOLS["tile16"]
    files["icons/favicon-tile-16.svg"] = svg(16, 16, f'<rect width="16" height="16" rx="3" fill="{TILE}"/><path fill="{ink_d}" d="{b16}"/><path fill="{open_d}" d="{p16}"/>', "fiscalclock")
    return files


# Cards and banner -----------------------------------------------------------
# The year strip shows NRF 4-5-4 fiscal 2023, an illustrative definition, not
# a built-in. Its dates are computed here with the standard library, not by
# fiscalclock: the year starts on Sunday 29 January 2023 (NRF's published
# calendar, checked in design/research/domain-visuals.md) and has 53 weeks,
# with periods of 4-5-4 weeks and the extra week in period 12.
FY_START = dt.date(2023, 1, 29)
PERIOD_WEEKS = [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 5]
assert sum(PERIOD_WEEKS) == 53
FY_END_EXCLUSIVE = FY_START + dt.timedelta(weeks=53)
assert FY_END_EXCLUSIVE == dt.date(2024, 2, 4)


def year_strip(t: Type, theme: str, x: float, y: float, width: float, height: float, label_size: float) -> str:
    c = LIGHT if theme == "light" else DARK
    ink, muted, rule = c["--fc-color-diagram-ink"], c["--fc-color-diagram-ink-muted"], c["--fc-color-diagram-rule"]
    fill, extra, surface = c["--fc-color-diagram-period-fill"], c["--fc-color-diagram-extra-week"], c["--fc-color-diagram-surface"]
    unit = width / 53
    out = [f'<defs><pattern id="hatch-{theme}" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">'
           f'<rect width="2" height="6" fill="{extra}"/></pattern></defs>']
    acc = 0
    for i, weeks in enumerate(PERIOD_WEEKS):
        px = x + acc * unit
        if i % 2 == 0:
            out.append(f'<rect x="{num(round(px, 2))}" y="{num(y)}" width="{num(round(weeks * unit, 2))}" height="{num(height)}" fill="{fill}"/>')
        if i == 11:
            ex = px + 4 * unit
            out.append(f'<rect x="{num(round(ex, 2))}" y="{num(y)}" width="{num(round(unit, 2))}" height="{num(height)}" fill="url(#hatch-{theme})"/>'
                       f'<rect x="{num(round(ex + 1, 2))}" y="{num(y + 1)}" width="{num(round(unit - 2, 2))}" height="{num(height - 2)}" fill="none" stroke="{extra}" stroke-width="2"/>')
        for k in range(1, weeks):
            wx = px + k * unit
            out.append(f'<rect x="{num(round(wx - 0.5, 2))}" y="{num(y + height - height * 0.22)}" width="1" height="{num(round(height * 0.22, 2))}" fill="{muted}"/>')
        if i:
            heavy = i % 3 == 0
            w = 3 if heavy else 1.5
            col = ink if heavy else rule
            ext = 10 if heavy else 0
            out.append(f'<rect x="{num(round(px - w / 2, 2))}" y="{num(y - ext)}" width="{num(w)}" height="{num(height + ext)}" fill="{col}"/>')
        acc += weeks
    # Quarter labels above the strip.
    q_start = 0
    for q in range(4):
        weeks = sum(PERIOD_WEEKS[q * 3:q * 3 + 3])
        label, lw = t.path(f"Q{q + 1}", label_size, 600, x + q_start * unit + 8, y - 12)
        sub, _ = t.path(f"{weeks} weeks", label_size, 400, x + q_start * unit + 8 + lw + label_size * 0.4, y - 12)
        out.append(f'<g fill="{ink}">{label}</g><g fill="{muted}">{sub}</g>')
        q_start += weeks
    # Axis with the closed start marker and the open end marker.
    ay = y + height + 14
    out.append(f'<rect x="{num(x)}" y="{num(ay - 1)}" width="{num(width)}" height="2" fill="{ink}"/>')
    out.append(f'<circle cx="{num(x)}" cy="{num(ay)}" r="7" fill="{ink}"/>')
    out.append(f'<circle cx="{num(x + width)}" cy="{num(ay)}" r="6" fill="{surface}" stroke="{extra}" stroke-width="3"/>')
    d1, _ = t.path(FY_START.isoformat(), label_size * 1.15, 600, x, ay + label_size * 2.1)
    n1, _ = t.path("start, included", label_size, 400, x, ay + label_size * 3.6)
    end_text = FY_END_EXCLUSIVE.isoformat()
    _, ew = t.path(end_text, label_size * 1.15, 600, 0, 0)
    _, nw = t.path("end_exclusive, excluded", label_size, 400, 0, 0)
    d2, _ = t.path(end_text, label_size * 1.15, 600, x + width - ew, ay + label_size * 2.1)
    n2, _ = t.path("end_exclusive, excluded", label_size, 400, x + width - nw, ay + label_size * 3.6)
    out.append(f'<g fill="{ink}">{d1}{d2}</g><g fill="{muted}">{n1}{n2}</g>')
    return "".join(out)


def card(t: Type, width: int, height: int, theme: str = "light", headline: bool = True) -> str:
    """A social card. Without the headline, it is the background of the Canva template, whose text stays editable."""
    c = LIGHT if theme == "light" else DARK
    margin = round(width * 0.0667)
    body = [f'<rect width="{width}" height="{height}" fill="{c["--fc-color-bg-default"]}"/>']
    lock, _ = lockup_body(t, margin, margin + (BRACKET_EM - BELOW_EM) * 44, 44, INK[theme], OPEN[theme])
    body.append(lock)
    head1, _ = t.path("Fiscal years, quarters, periods and weeks", 50, 600, margin, 232)
    head2, _ = t.path("from one JSON definition.", 50, 600, margin, 296)
    sub, _ = t.path("Rust, Python and JavaScript, with byte-identical results.", 26, 400, margin, 350)
    if headline:
        body.append(f'<g fill="{c["--fc-color-text-default"]}">{head1}{head2}</g><g fill="{c["--fc-color-text-muted"]}">{sub}</g>')
    body.append(year_strip(t, theme, margin, height - 205, width - 2 * margin, 48, 17))
    note, nw = t.path("Example: NRF 4-5-4 fiscal 2023, 53 weeks", 17, 400, 0, 0)
    note, _ = t.path("Example: NRF 4-5-4 fiscal 2023, 53 weeks", 17, 400, (width - nw) / 2, height - 205 + 48 + 14 + 17 * 3.6)
    body.append(f'<g fill="{c["--fc-color-text-muted"]}">{note}</g>')
    return svg(width, height, "".join(body), "fiscalclock: fiscal years, quarters, periods and weeks from one JSON definition")


def banner(t: Type, theme: str) -> str:
    """README header: the lockup and the one-line description, nothing else."""
    c = LIGHT if theme == "light" else DARK
    w, h = 840, 150
    body = [f'<rect width="{w}" height="{h}" fill="{c["--fc-color-bg-default"]}"/>']
    lock, _ = lockup_body(t, 32, 32 + (BRACKET_EM - BELOW_EM) * 40, 40, INK[theme], OPEN[theme])
    desc, _ = t.path("Fiscal calendar engine: compute any fiscal calendar from a JSON definition.", 19, 400, 32, 116)
    body.append(lock + f'<g fill="{c["--fc-color-text-muted"]}">{desc}</g>')
    return svg(w, h, "".join(body), "fiscalclock: fiscal calendar engine. Compute any fiscal calendar from a JSON definition.")


# Wiki illustrations ---------------------------------------------------------
# The GitHub wiki does not display SVG, so these are rendered to PNG and
# committed to the wiki repository. They explain the rules; they are not brand
# assets.

def wiki_images(t: Type) -> dict[str, tuple[str, int]]:
    L, D = LIGHT, DARK
    files: dict[str, tuple[str, int]] = {}

    def label(text: str, x: float, y: float, colour: str, size: float = 18, weight: int = 400, centre: bool = False) -> str:
        if centre:
            _, w = t.path(text, size, weight, 0, 0)
            x -= w / 2
        d, _ = t.path(text, size, weight, x, y)
        return f'<g fill="{colour}">{d}</g>'

    # Variants: colour and one-colour lockups on light and dark backgrounds.
    tiles = [("light", None, "Colour, light background"), ("dark", None, "Colour, dark background"),
             ("light", "mono", "One colour, light background"), ("dark", "white", "One colour, dark background")]
    body = []
    for i, (theme, mono, caption) in enumerate(tiles):
        x0, y0 = (i % 2) * 800, (i // 2) * 300
        c = L if theme == "light" else D
        ink = WHITE if mono == "white" else INK[theme]
        open_end = ink if mono else OPEN[theme]
        _, lw = lockup_body(t, 0, 0, 60, ink, open_end)
        lock, _ = lockup_body(t, x0 + (800 - lw) / 2, y0 + 130 + 0.37 * 60, 60, ink, open_end)
        body.append(f'<rect x="{x0}" y="{y0}" width="800" height="300" fill="{c["--fc-color-bg-default"]}"/>' + lock)
        body.append(label(caption, x0 + 400, y0 + 262, c["--fc-color-text-muted"], 18, 400, True))
    files["logo-variants.svg"] = (svg(1600, 600, "".join(body), "The fiscalclock logo in colour and in one colour, on light and dark backgrounds"), 1600)

    # Clear space: half the symbol's height on every side.
    size, W, H = 100, 1200, 480
    h = BRACKET_EM * size
    pad = h / 2
    _, lw = lockup_body(t, 0, 0, size, INK["light"], OPEN["light"])
    x, top = (W - lw) / 2, (H - h) / 2 - 24
    lock, _ = lockup_body(t, x, top + (BRACKET_EM - BELOW_EM) * size, size, INK["light"], OPEN["light"])
    guide, accent, muted = L["--fc-color-border-strong"], L["--fc-color-accent-default"], L["--fc-color-text-muted"]
    body = [f'<rect width="{W}" height="{H}" fill="{L["--fc-color-bg-default"]}"/>',
            f'<rect x="{num(round(x - pad, 2))}" y="{num(round(top - pad, 2))}" width="{num(round(lw + 2 * pad, 2))}" height="{num(round(h + 2 * pad, 2))}" fill="{L["--fc-color-bg-subtle"]}" stroke="{guide}" stroke-width="2" stroke-dasharray="8 6"/>',
            f'<rect x="{num(round(x, 2))}" y="{num(round(top, 2))}" width="{num(round(lw, 2))}" height="{num(round(h, 2))}" fill="{L["--fc-color-bg-default"]}"/>',
            lock,
            f'<rect x="{num(round(x - pad, 2))}" y="{num(round(top + h / 2 - 1, 2))}" width="{num(round(pad, 2))}" height="2" fill="{accent}"/>',
            label("x", x - pad / 2, top + h / 2 - 10, accent, 22, 600, True),
            f'<rect x="{num(round(x + lw + pad / 2, 2))}" y="{num(round(top, 2))}" width="2" height="{num(round(h, 2))}" fill="{accent}"/>',
            label("2x", x + lw + pad / 2 + 10, top + h / 2 + 8, accent, 22, 600),
            label("Clear space: x is half the height of the symbol, on every side.", W / 2, H - 30, muted, 20, 400, True)]
    files["logo-clear-space.svg"] = (svg(W, H, "".join(body), "Clear space around the fiscalclock lockup: half the symbol's height on every side"), W)

    # Sizes, at actual pixels.
    body = []
    for row, theme in enumerate(("light", "dark")):
        c = L if theme == "light" else D
        y0 = row * 200
        body.append(f'<rect x="0" y="{y0}" width="1000" height="200" fill="{c["--fc-color-bg-default"]}"/>')
        x = 50
        for px in (16, 20, 24, 32, 48, 64):
            kind = "small" if px < 24 else "master"
            sc = px / SYMBOLS[kind][0]
            body.append(symbol_paths(kind, INK[theme], OPEN[theme], f"translate({x} {y0 + 90 - px / 2}) scale({num(round(sc, 5))})"))
            body.append(label(f"{px} px", x + px / 2, y0 + 155, c["--fc-color-text-muted"], 16, 400, True))
            body.append(label(f"{kind} drawing", x + px / 2, y0 + 177, c["--fc-color-text-muted"], 13, 400, True))
            x += max(px, 64) + 90
    files["logo-sizes.svg"] = (svg(1000, 400, "".join(body), "The fiscalclock symbol at 16, 20, 24, 32, 48 and 64 pixels on light and dark backgrounds"), 1000)

    # Misuse.
    cells = [("colours swapped", "swap"), ("stretched", "stretch"), ("rotated", "rotate"),
             ("off-palette colours", "recolour"), ("busy background", "busy"), ("effects added", "effects")]
    body = [f'<defs><pattern id="busy" width="14" height="14" patternUnits="userSpaceOnUse" patternTransform="rotate(30)">'
            f'<rect width="14" height="14" fill="{L["--fc-palette-teal-50"]}"/><rect width="7" height="14" fill="{L["--fc-palette-amber-300"]}"/></pattern></defs>']
    for i, (caption, kind) in enumerate(cells):
        x0, y0 = (i % 3) * 400, (i // 3) * 260
        body.append(f'<rect x="{x0 + 0.5}" y="{y0 + 0.5}" width="399" height="259" fill="{L["--fc-color-bg-default"]}" stroke="{L["--fc-color-border-default"]}"/>')
        ink, open_end = INK["light"], OPEN["light"]
        tf = f"translate({x0 + 152} {y0 + 30}) scale(1.5)"
        if kind == "swap":
            ink, open_end = open_end, ink
        elif kind == "stretch":
            tf = f"translate({x0 + 100} {y0 + 54}) scale(3 1.2)"
        elif kind == "rotate":
            tf = f"translate({x0 + 200} {y0 + 102}) rotate(-15) translate(-48 -48) scale(1.5)"
        elif kind == "recolour":
            ink, open_end = L["--fc-color-status-danger"], L["--fc-color-status-info"]
        elif kind == "busy":
            body.append(f'<rect x="{x0 + 110}" y="{y0 + 20}" width="180" height="160" fill="url(#busy)"/>')
        elif kind == "effects":
            body.append(f'<g transform="translate(7 7)" opacity="0.4">{symbol_paths("master", L["--fc-color-text-muted"], L["--fc-color-text-muted"], tf)}</g>')
        body.append(symbol_paths("master", ink, open_end, tf))
        body.append(label(f"Don't: {caption}", x0 + 200, y0 + 228, L["--fc-color-text-default"], 18, 600, True))
    files["logo-misuse.svg"] = (svg(1200, 520, "".join(body), "Six ways not to use the fiscalclock symbol"), 1200)

    # Year strip in both themes.
    for theme in ("light", "dark"):
        c = L if theme == "light" else D
        body = f'<rect width="1200" height="250" fill="{c["--fc-color-bg-default"]}"/>' + year_strip(t, theme, 80, 60, 1040, 56, 17)
        files[f"diagram-year-strip-{theme}.svg"] = (svg(1200, 250, body,
            "NRF 4-5-4 fiscal 2023 drawn to scale: 53 weeks from 2023-01-29 (included) to 2024-02-04 (excluded), "
            "quarters of 13, 13, 13 and 14 weeks, with week 53 hatched in period 12"), 1200)
    return files


# Rendering ------------------------------------------------------------------

def render(inkscape: str, src: Path, dst: Path, width: int, height: int | None = None, background: str | None = None) -> None:
    cmd = [inkscape, str(src), "--export-type=png", f"--export-filename={dst}", f"--export-width={width}"]
    if height:
        cmd.append(f"--export-height={height}")
    if background:
        cmd += [f"--export-background={background}", "--export-background-opacity=1"]
    subprocess.run(cmd, check=True, capture_output=True)


def pad_png(path: Path, background: str) -> None:
    """Surround a tightly cropped lockup PNG with its clear space: half the symbol's height on every side."""
    im = Image.open(path).convert("RGBA")
    pad = round(im.height / 2)
    canvas = Image.new("RGBA", (im.width + 2 * pad, im.height + 2 * pad), background)
    canvas.alpha_composite(im, (pad, pad))
    canvas.convert("RGB").save(path, optimize=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--font", required=True, type=Path, help="IBM Plex Sans variable font file")
    parser.add_argument("--wiki-images", type=Path, help="also write the wiki's PNG illustrations into this folder")
    parser.add_argument("--inkscape", default=shutil.which("inkscape") or r"C:\Program Files\Inkscape\bin\inkscape.com")
    args = parser.parse_args()
    t = Type(args.font)

    files = {**build_logos(t), **build_icons()}
    files["social/og-image.svg"] = card(t, 1200, 630)
    files["social/github-social-preview.svg"] = card(t, 1280, 640)
    files["social/og-template-background.svg"] = card(t, 1200, 630, headline=False)
    files["readme/readme-banner-light.svg"] = banner(t, "light")
    files["readme/readme-banner-dark.svg"] = banner(t, "dark")
    for name, text in files.items():
        path = OUT / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8", newline="\n")

    ink = args.inkscape
    r = lambda name, png, w, h=None, bg=None: render(ink, OUT / name, OUT / png, w, h, bg)
    r("social/og-image.svg", "social/og-image.png", 1200, 630)
    r("social/github-social-preview.svg", "social/github-social-preview.png", 1280, 640)
    r("readme/readme-banner-light.svg", "readme/readme-banner.png", 1680, 300)
    r("icons/icon-tile.svg", "icons/icon-512.png", 512)
    r("icons/icon-tile.svg", "icons/icon-192.png", 192)
    r("icons/icon-tile.svg", "icons/registry-icon-512.png", 512)
    r("icons/icon-maskable.svg", "icons/icon-maskable-512.png", 512)
    r("icons/icon-maskable.svg", "icons/apple-touch-icon.png", 180)
    for suffix, bg in (("", BG["light"]), ("-dark", BG["dark"])):
        r(f"logo/fiscalclock-lockup{suffix}.svg", f"logo/fiscalclock-lockup{suffix}.png", 960, None, bg)
        pad_png(OUT / "logo" / f"fiscalclock-lockup{suffix}.png", bg)
        r(f"logo/fiscalclock-symbol{suffix}.svg", f"logo/fiscalclock-symbol{suffix}-512.png", 512)
    # favicon.ico: the 16 px layer uses its own pixel-fitted drawing.
    tmp = OUT / "icons" / "_ico"
    tmp.mkdir(exist_ok=True)
    r("icons/favicon-tile-16.svg", "icons/_ico/16.png", 16)
    for size in (32, 48):
        render(ink, OUT / "icons" / "icon-tile.svg", tmp / f"{size}.png", size)
    layers = [Image.open(tmp / f"{s}.png").convert("RGBA") for s in (48, 32, 16)]
    layers[0].save(OUT / "icons" / "favicon.ico", format="ICO", sizes=[(48, 48), (32, 32), (16, 16)], append_images=layers[1:])
    shutil.rmtree(tmp)
    (OUT / "icons" / "favicon-tile-16.svg").unlink()
    r("social/og-template-background.svg", "social/og-template-background.png", 2400, 1260)
    if args.wiki_images:
        args.wiki_images.mkdir(parents=True, exist_ok=True)
        for name, (text, width) in wiki_images(t).items():
            src = args.wiki_images / name
            src.write_text(text, encoding="utf-8", newline="\n")
            render(ink, src, src.with_suffix(".png"), width)
            src.unlink()
        print(f"wrote the wiki illustrations to {args.wiki_images}")
    print(f"wrote {len(files)} SVG files and their PNG exports to {OUT.relative_to(ROOT).as_posix()}/")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
