<!-- cspell:words Canva Inkscape opensource wordmark workclock -->

# Design decision log

This file records every decision about fiscalclock's visual identity and design system: the question, the options considered, the choice, and the reason. Newest entries go at the bottom of each table. The wiki page "Design decision log" mirrors this file; when they differ, this file wins.

"Owner" means the maintainer chose. "Basis" means a technical call made on evidence, which the maintainer can reopen like any other.

## Phase 1: interview (1 Nov 2026)

| ID | Question | Options considered | Choice | Why |
| --- | --- | --- | --- | --- |
| DD-001 | Relationship to the sibling project workclock | Same system with a different symbol; loose cousins; independent; umbrella brand | **Independent.** Nothing is copied from workclock: no colours, typefaces, logo ideas, tokens, page names or site stack. | Owner: "This project is independent of workclock." |
| DD-002 | Primary audience | Rust developers; Python data users; JS/TS developers; finance and analytics teams; AI agents and their users | **All of them, with no single primary group**: developers in all three languages, finance and analytics teams, and AI agents. | Owner. The identity has to read as credible to an accountant and as precise to an engineer. |
| DD-003 | How often non-programmers read the docs | Often; sometimes, through a developer; rarely | **Rarely.** Docs are written for people who write code; plain-English explanations support them but do not lead. | Owner. |
| DD-004 | Reference brands | Owner named the sites they admire | **Admired:** Stripe Docs, Claude Docs, Linear, Vercel, Resend. **Not a reference:** workclock. | Owner. All five admired sites are restrained, high-contrast and treat dark mode as a full theme; the research checks what they have in common. |
| DD-005 | Dark mode | Equal to light; light first; not needed | **First-class, equal to light.** Every colour pairing and every asset is designed and checked in both themes. | Owner. |
| DD-006 | Personality | Five 1–5 sliders: serious–friendly, technical–approachable, minimal–rich, classic–modern, quiet–distinctive | **2.5 on every slider**: balanced, leaning very slightly towards serious, technical, minimal, classic and quiet. | Owner. The half-point lean is read as "calm and precise, but not cold"; Phase 3 states this reading so the owner can correct it. |
| DD-007 | What the logo should be about | Periods and tiling; the 53rd week; ledger and grid; a clock reference; wordmark only; no preference | **No preference: show options.** | Owner. Phase 3 presents distinct concepts. |
| DD-008 | Vector logo files, given that the Canva plan has no SVG export (checked 1 Nov 2026: export formats offered were PDF, JPG, PNG, PPTX, GIF and MP4) and no Brand Kit | Upgrade Canva; author SVG elsewhere | **Author logo and icon SVGs in an open-source SVG editor (Inkscape) and keep them in the repo as the masters.** Canva is used for raster cards. | Owner: "install any opensource svg editor and work in that." |
| DD-009 | Website and docs stack | Decide now; decide later | **Not decided. Assume nothing.** Tokens are kept framework-neutral (W3C format plus plain CSS custom properties). | Owner: "do not assume anything yet." |
| DD-010 | Font licences | Open licences only; any | **Open licences only (SIL Open Font License).** | Owner. |
| DD-011 | Wiki page names and structure | Follow workclock; independent | **Independent.** | Owner. |
| DD-012 | Extra assets beyond the brief (slides, social banners) | Owner unsure | **Open.** Phase 3 recommends what, if anything, to add. | Owner: "I'm not sure." |

## Phase 3: brand directions (1 Nov 2026)

| ID | Question | Options considered | Choice | Why |
| --- | --- | --- | --- | --- |
| DD-013 | Brand direction | A. Ledger (serif, ledger green, double-rule mark); B. Period grid (IBM Plex, ink blue, 4-5-4 week-grid mark); C. Interval (sans, deep teal, `[ )` mark). Sketches: [`explorations/brand-directions.html`](explorations/brand-directions.html); every colour pair checked by [`explorations/check_palettes.py`](explorations/check_palettes.py). | **C, set in IBM Plex Sans and IBM Plex Mono** instead of Inter and Commit Mono. Owner chose this on 1 Nov 2026. | See the Phase 3 recommendation: the `[ )` mark is the most legible at 16 px and states the library's core rule; Plex removes C's weakest point (Inter reads as a default) and brings tabular figures by default. |
| DD-014 | Extra assets (DD-012) | Slide template; social banner; none | **Proposed:** none beyond the brief for now; reconsider when the website exists. | Each extra asset is another file to keep in step with the tokens; nothing in the roadmap needs slides or banners yet. |

## Phase 4: design system (1 Nov 2026)

| ID | Question | Options considered | Choice | Why |
| --- | --- | --- | --- | --- |
| DD-015 | Light-theme text colour | Keep the Phase 3 value `#191918`; use `#1a1a19` | `#1a1a19`, the same value as the dark theme's subtle background | One value fewer in the palette; 17.11:1 on the light background, so nothing is lost. Basis. |
| DD-016 | Token format and build | Style Dictionary 5.7.1; Terrazzo 2.10.0; a standard-library Python generator | **Standard-library generator** ([`scripts/build_tokens.py`](scripts/build_tokens.py)) reading strict DTCG 2025.10 files and a resolver document | No dependencies, same habits as the repository's other check scripts, and Style Dictionary does not yet read resolver documents. The files stay strict so Terrazzo can read them as a cross-check ([research](research/design-tokens.md#72-write-our-own-or-use-style-dictionary-or-terrazzo)). Basis. |
| DD-017 | CSS variable prefix | None; `--fc-` | `--fc-` | Prevents collisions with framework variables such as Tailwind's `--color-*`. Basis. |
| DD-018 | Dark-mode switching | `light-dark()`; selector blocks | **Selector blocks**: system preference by default, `data-theme` on `<html>` overrides | Works for every token type, not only colours, and maps onto every docs framework surveyed. Basis. |
| DD-019 | Shadows | A shadow scale; none | **None.** Raised surfaces use `color.bg.raised` and a border. | Fewer tokens, and borders read the same in both themes; a shadow on near-black is nearly invisible. Basis. |
| DD-020 | Number style | Default figures; tabular figures in tables only; tabular figures wherever numbers are compared | **Tabular lining figures wherever numbers or dates are read in columns or compared**: tables, results, diagram labels, code-adjacent figures | Dates are the product's output; equal-width digits keep them aligned. IBM Plex Sans figures are tabular by default; the CSS rule also covers fallback fonts ([research](research/typography.md)). Basis. |
| DD-021 | Logo geometry | Phase 3 sketch as drawn; refined drawing | **Refined.** The bracket is straight lines on a 64-unit grid with an 8-unit stroke. The parenthesis is the band between two half-ellipses, cut flat at the bracket's height, so its stroke tapers from about 8.3 to 6.9 units like a typeset parenthesis. A separate 16-unit drawing with 2 px strokes serves 16 to 23 px. | Exact geometry can be rebuilt and checked; the small drawing keeps the symbol legible at favicon size ([`scripts/build_brand_assets.py`](scripts/build_brand_assets.py)). Basis. |
| DD-022 | Lockup proportions | Symbol as tall as the ascenders; taller | **Bracket height 0.82 em**, overhanging the ascenders and the baseline by 0.04 em each; gap 0.28 em; wordmark in IBM Plex Sans SemiBold with -10 units of tracking per letter. | A first version at 0.93 em made the bracket stroke (0.143 em) clearly heavier than the letters' stems (about 0.108 em); at 0.82 em it is 0.126 em, a little heavier, which gives the symbol presence without dominating. Basis, checked by eye on rendered exports. |
| DD-023 | How the brand files are made | Draw in Canva; draw by hand in Inkscape; generate from geometry and font outlines, render with Inkscape | **Generate** every SVG from the geometry and IBM Plex Sans outlines with a script, and **render every PNG with Inkscape**. Canva holds an editable social-card template built on the generated background. | Reproducible and exact: the colours are read from the token build, so assets cannot drift from the tokens. The Canva plan cannot export SVG (DD-008), and Canva's editing tools cannot set a typeface, so Canva text cannot match the wordmark. Basis. |
| DD-024 | Icon background where the surrounding colour is unknown (app icons, `favicon.ico`, docs.rs) | Transparent; light tile; dark tile | **Dark tile** (`color.bg.subtle` of the dark theme) with the dark-theme logo colours. The SVG favicon stays transparent and switches colours with `prefers-color-scheme`. | A tile reads on light and dark surfaces alike; docs.rs gives custom logos no help in its dark themes ([research](research/asset-specs.md)). Basis. |
| DD-025 | README banner content | Lockup, description and a small year strip; lockup and description only | **Lockup and description only.** | Without labels, the small strip was decoration rather than information. Basis. |
