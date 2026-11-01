<!-- cspell:words DTCG Inkscape Canva wordmark wordmarks Terrazzo APCA Crameri Fumadocs Geist Mintlify Okabe Reinecke Trockman Tuch Tufte maskable monospace -->

# Research brief

This brief condenses seven research tracks into the constraints and choices that shape fiscalclock's visual identity. Each section links to the full notes in [`research/`](research/), which hold the sources, the checks that were run and what could not be verified. Research was done on 1 Nov 2026; facts below are as of that date.

## What the identity has to do

fiscalclock computes dates that end up in financial reports. Its whole method is "shown, not claimed": cited sources, independent generators, byte-identical output in three languages. The identity's job is to make that method visible and to stay out of its way. The owner's answers ([`DECISIONS.md`](DECISIONS.md), DD-001 to DD-012) add: independent of any sibling project, every audience from Rust developers to finance teams and AI agents, dark mode equal to light, a balanced personality that leans very slightly serious and quiet, and open-licence fonts only.

## 1. What respected docs sites have in common

Notes: [`research/peer-analysis.md`](research/peer-analysis.md). Eleven sites measured in a browser, light and dark: Stripe Docs, Claude Docs, Linear, Vercel and Resend (the owner's references), plus uv, Polars, DuckDB, Tokio, Deno and Pydantic.

- **Neutral surfaces and one restrained accent.** Vercel and DuckDB set links in the text colour with an underline; Claude Docs keeps its brand colour almost entirely to the logo. Trust comes from restraint, not from colour.
- **Off-white and near-black, never pure extremes**, for example `#fcfcfb` / `#151515` (Claude Docs) and `#fafafa` / `#171717` (Vercel).
- **Dark mode follows the system and has a light / dark / system switch.** Stripe Docs and Tokio have no dark theme at all, so "equal to light" is above the norm, not at it.
- **Three columns**: navigation 224–300 px, text 650–860 px, "On this page" on the right.
- **One sans for text, one monospace for code**; a serif appears only for Claude Docs' headings.
- **One language choice per page** (Claude Docs) or tabs on every example (Polars). fiscalclock's identical JSON makes a page-wide Rust / Python / JS switch natural.
- **Repository facts as trust signals**: version, changelog, "Edit this page", last-updated date, a Markdown view and `llms.txt` for agents.
- **An opening nobody took:** none of the eleven sets tabular figures in prose or tables. For a library whose output is dates and week numbers, aligned figures are a cheap, visible sign of precision.

Clichés found and to avoid: glow and light-streak heroes, gradient-filled marks, "AI era" slogans, unbacked superlatives, logo walls and screenshot heroes, and logo detail that turns to mush at 16 px (Linear's lines, Polars' pixel bear).

## 2. What builds trust, and what fakes it

Notes: [`research/trust-and-credibility.md`](research/trust-and-credibility.md). Each rule is graded by its evidence; the strongest are experiments.

1. **Simple, conventional structure** reads as more trustworthy than novel layouts (experiments: Tuch 2012, Reinecke 2013). Distinctiveness should come from type, diagrams and real content, not unusual page structure.
2. **Neutrals plus one accent.** High colourfulness was judged least favourably, most of all by highly educated participants (experiment).
3. **Polish raises credibility even for identical content** (small experiment). The duty that follows: anything illustrative or unverified must be labelled as such in the design itself, so polish never outruns proof.
4. **Draw intervals to scale.** A 53-week year is exactly 7/364 longer than a 52-week one; truncated axes mislead even when marked (Tufte's lie factor; truncation experiments).
5. **Colour never carries meaning alone** (WCAG 1.4.1); no rainbow or red–green scales (Crameri 2020).
6. **Signals computed by CI beat decorative badges** (study of 294,941 npm repositories, Trockman 2018).
7. **Put evidence where people look first** (Fogg's prominence-interpretation theory): conformance counts and verification commands belong near the top, not in a footer.
8. **Finance experts used design mainly to reject flashy sites** (qualitative, seven experts). Flash costs more than plainness.

Evidence on what makes a design look "AI-generated" is thin (practitioner commentary only); the working rule is to avoid stock gradients, glows and generic three-card feature grids, and to lead with the product's real output.

## 3. Accessibility: hard constraints

Notes: [`research/accessibility.md`](research/accessibility.md). Script: [`scripts/contrast.py`](scripts/contrast.py) (standard library; `--self-test` matches nine published WebAIM reference values).

| Requirement (WCAG 2.2) | Threshold |
| --- | --- |
| Body text, 1.4.3 | 4.5:1 |
| Large text (24 px, or 18.67 px bold), 1.4.3 | 3:1 |
| Borders of controls, focus rings, diagram lines and fills that carry meaning, 1.4.11 | 3:1 against every adjacent colour |
| Target size, 2.5.8 | 24 × 24 CSS px |
| Logos and wordmarks | Exempt; but the same colour used for text is not |

- Ratios are never rounded up: 4.499:1 fails 4.5:1. The script truncates the printed value and decides on the exact one.
- The linearisation threshold in the current text is 0.04045; for 8-bit colours it gives the same result as the older 0.03928.
- Dark theme: near-black background and off-white text; saturated hues need lighter dark-theme versions (pure blue `#0000ff` on `#121212` is only 2.18:1). Neither Okabe–Ito nor Paul Tol's "bright" set passes 3:1 on both white and near-black, so diagram colours need one value per theme.
- Forced colours (Windows High Contrast) remove `box-shadow`, so focus rings use `outline`.
- APCA and WCAG 3 are drafts; APCA may appear only as an informational column.

## 4. Design tokens

Notes: [`research/design-tokens.md`](research/design-tokens.md).

- **Format:** W3C Design Tokens Community Group (DTCG) **2025.10**, a stable Final Community Group Report with Format, Color and Resolver modules. Colours are objects: `{"colorSpace": "srgb", "components": [...], "hex": "#..."}`. Files end in `.tokens.json`.
- **Themes:** one primitives file, plus semantic light and dark files with identical paths, tied together by a Resolver document with a `theme` modifier.
- **CSS output:** light values on `:root`; dark values under `prefers-color-scheme: dark` unless the reader chose light, and again under `[data-theme="dark"]`, so a toggle always wins. `light-dark()` stays optional.
- **Generator:** a standard-library Python script that resolves aliases, fails on missing or circular references and on themes that differ in shape, with a `--check` mode for CI. Style Dictionary (no Resolver support yet) and Terrazzo (supports it, but brings 18 direct dependencies) are not needed; the files stay strictly 2025.10 so Terrazzo can serve as an independent cross-check.
- **Frameworks:** every docs framework surveyed (Fumadocs, Starlight, Docusaurus, VitePress, Nextra, Mintlify, Material for MkDocs, Zensical, Furo, PyData, mdBook, Tailwind v4) can be themed from plain CSS custom properties, with a thin mapping layer each. Nextra wants HSL numbers and Mintlify wants hex in its config, so the generator should be able to emit those too. No choice of stack is needed now (DD-009).

## 5. Typography

Notes: [`research/typography.md`](research/typography.md). 31 open-licence families inspected with `fontTools` and rendered as specimens.

| Pairing | Faces | Strength | Cost |
| --- | --- | --- | --- |
| Neutral | Inter + Commit Mono | Familiar, compact, wide weight range | Inter reads as a default; its `I` and `l` need `ss02`; Commit Mono is static and last released in 2023 |
| Technical | IBM Plex Sans + Plex Mono (+ Plex Serif) | Figures are tabular by default; `I l 1` clearly distinct; one designed family | Strong IBM association; no proportional figures in the sans |
| Ledger | Source Serif 4 headings + Source Sans 3 + Source Code Pro | All three tabular by default; matched x-heights; classic financial-print feel | Small x-height; serif's slashed zero looks like Ø, so the serif must not use `zero` |

Rules that hold whichever is chosen: tables, dates and results use `font-variant-numeric: tabular-nums lining-nums`; `slashed-zero` is applied only where the face has the `zero` feature (Geist, Commit Mono and Newsreader do not); dates are ISO `YYYY-MM-DD`.

## 6. Asset specifications

Notes: [`research/asset-specs.md`](research/asset-specs.md).

| Asset | Size and format | Note |
| --- | --- | --- |
| GitHub social preview | 1280 × 640 PNG, under 1 MB | Opaque background |
| Open Graph / X card | 1200 × 630 PNG | X no longer publishes card docs; figures from an archived copy |
| Favicon set | `favicon.ico` (16, 32, 48), `icon.svg`, `apple-touch-icon.png` 180 × 180 | Google Search does not list SVG favicons, so a raster icon stays |
| Web app icons | 192, 512 and maskable 512 PNG plus a manifest | Only if the site should be installable |
| README banner | Light and dark SVG via `<picture>`, plus an opaque PNG fallback | PyPI strips `<source>` and npm defeats it, so the fallback shows there in both themes |
| docs.rs logo | Square SVG shown at 48 CSS px | Gets no dark-theme help; needs its own opaque tile |
| GitHub wiki images | PNG | The wiki does not list SVG as supported |

Text in standalone SVGs is converted to outlines so it renders the same everywhere.

## 7. Calendar diagrams

Notes: [`research/domain-visuals.md`](research/domain-visuals.md). None of the published fiscal calendars surveyed (NRF, ERP and BI docs, annual reports) draws a year to scale or marks an excluded end, so fiscalclock's diagrams can be the precise reference others lack.

- **Six diagram types:** year strip, week grid, boundary detail, 52 vs 53-week comparison, two-calendar comparison, and multi-year drift.
- **Shared rules:** days are cells and boundaries are lines; width is linear in days and never truncated; `start` gets a filled marker and `end_exclusive` an open one, labelled "included" and "excluded"; the extra week is hatched and labelled, never colour alone; ISO dates in tabular figures; a visible table of the same numbers under each figure.
- **Production:** every diagram that shows dates is generated by a standard-library script from the independent oracle in [`conformance/generators/`](../conformance/generators/), committed, and rerun by CI. A sample NRF 2023 year strip matched all 14 dates transcribed from NRF's published PDF.
- **Colour roles** (values chosen with the palette): `surface`, `ink`, `ink-muted`, `rule`, `quarter-rule`, `period-fill-a`, `period-fill-b`, `extra-week`, `boundary`, `calendar-b`, `today-marker`.

## What this means for the brand directions

1. **Make precision the signature.** The identity's memorable element should come from the library's own subject (periods that tile a year exactly, the 53rd week, half-open bounds), not from decoration.
2. **One accent, earned.** Neutrals carry the system; the accent marks one meaning (for example the extra week, or the boundary) and is checked at 3:1 or 4.5:1 in both themes.
3. **Type does the talking.** Tabular figures everywhere numbers appear; the typeface choice is the biggest single lever on personality.
4. **Evidence is a design element.** Conformance counts, verification commands and "last reviewed" dates get a designed place near the top of pages and in the README.
5. **The mark must survive 16 px, one colour, and both themes**, as solid shapes with no thin lines or gradients, drawn in Inkscape as the SVG master.
6. **Avoid the generated-design defaults:** cream and terracotta, near-black with an acid accent, glows and gradients, card grids, tracked capitals above every heading.
