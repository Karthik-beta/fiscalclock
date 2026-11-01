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
| DD-013 | Brand direction | A. Ledger (serif, ledger green, double-rule mark); B. Period grid (IBM Plex, ink blue, 4-5-4 week-grid mark); C. Interval (sans, deep teal, `[ )` mark). Sketches: [`explorations/brand-directions.html`](explorations/brand-directions.html); every colour pair checked by [`explorations/check_palettes.py`](explorations/check_palettes.py). | **Proposed, awaiting the owner:** C, set in IBM Plex Sans and Plex Mono instead of Inter and Commit Mono. | See the Phase 3 recommendation: the `[ )` mark is the most legible at 16 px and states the library's core rule; Plex removes C's weakest point (Inter reads as a default) and brings tabular figures by default. |
| DD-014 | Extra assets (DD-012) | Slide template; social banner; none | **Proposed:** none beyond the brief for now; reconsider when the website exists. | Each extra asset is another file to keep in step with the tokens; nothing in the roadmap needs slides or banners yet. |
