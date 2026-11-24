# Handoff

Paused on 1 Nov 2026 because of a usage limit. Do not commit this file or `.handoff/`.

## Goal and overall plan

Build fiscalclock's visual identity and design system, in four phases set by the owner:

1. Understand the library and interview the owner. **Done.**
2. Research with sub-agents, then a research brief. **Done.**
3. Present 2–3 brand directions and a recommendation, then stop. **Done.** The owner chose direction C, "Interval", set in IBM Plex.
4. Create the documents (wiki), the machine-readable tokens (repo) and the visual assets (repo and Canva), then run a quality gate. **Done and published.**

All four phases are complete. Only follow-up work remains (see "Exact next step").

## Done so far

The repository work is merged to `main` in [#39](https://github.com/Karthik-beta/fiscalclock/pull/39) (merge commit `cd4ba95`, merged 1 Nov 2026).

- `design/DECISIONS.md` is the decision log, DD-001 to DD-029. It is the source of truth; the wiki page Design-Decisions mirrors it.
- `design/research/` holds seven cited research notes, and `design/research-brief.md` summarises them.
- `design/explorations/` holds the Phase 3 sketches (`brand-directions.html`) and `check_palettes.py`.
- `design/tokens/` holds DTCG 2025.10 token files:
  - `primitives.tokens.json`;
  - `semantic.light.tokens.json` and `semantic.dark.tokens.json`;
  - `fiscalclock.resolver.json`;
  - `contrast-checks.json`, which lists 46 colour pairs;
  - `generated/`, which holds `tokens.css`, `tokens.light.json`, `tokens.dark.json` and `contrast.md`;
  - a `README.md`.
- `design/scripts/` holds four scripts:
  - `contrast.py`: WCAG contrast ratios; standard library only.
  - `build_tokens.py`: builds the token outputs; standard library only.
  - `check_brand.py`: checks that assets and documents use only token colours and names; standard library only.
  - `build_brand_assets.py`: generates every brand file. It needs `requirements-brand.txt` (fontTools, uharfbuzz, Pillow), IBM Plex Sans and Inkscape 1.4.
- `assets/brand/` holds:
  - `logo/`: lockup, symbol and wordmark, each in colour, `-dark`, `-mono` and `-mono-white`, plus the symbol's `-small` drawing;
  - `icons/`: favicons, app icons and the docs.rs logo;
  - `social/`: the Open Graph card, the GitHub social preview and the Canva template background;
  - `readme/`: the README banner.
- `design/README.md` and `assets/brand/README.md`.

The wiki is pushed; its head is `1851eb2`. It has 15 design pages:

- Design-System, Brand-Foundation, Voice-and-Writing, Logo-Guidelines, Colour-System;
- Typography, Design-Tokens, Layout-and-Spacing, Iconography-and-Illustration, Diagrams-and-Data-Visuals;
- Docs-Components, Accessibility, Asset-Inventory, Design-Glossary, Design-Decisions.

It also has `images/design/*.png` and a "Design system" section in Home and `_Sidebar`. The working clone is in the session scratchpad at `C:\Users\Admin\AppData\Local\Temp\claude\C--Users-Admin-Desktop-fiscalclock\57163dd6-b20e-4376-8fbc-795c3c91ec24\scratchpad\wiki`. Re-clone if that folder is gone.

The Canva design "fiscalclock social card template (1200 x 630)" (ID `DAHZsdFZm-k`) is saved with the owner's approval. Its background is `assets/brand/social/og-template-background.png`, and its two text boxes are editable.

Local state:

- The checkout is on branch `design-foundation`, which is merged. Run `git switch main && git pull` before any new work.
- Inkscape 1.4.4 is installed through winget.
- The `CLAUDE.md` change from this pause is uncommitted (see below).

## In progress, and the exact next step

Nothing is in progress.

**Waiting on the owner:**

1. Upload `assets/brand/social/github-social-preview.png` in the repository settings (Settings → Social preview).
2. In Canva, set the template's two text boxes to IBM Plex Sans; the headline in SemiBold. Canva's API cannot set fonts.
3. Make the open decisions:
   - the website and docs stack (DD-009);
   - an icon set (Lucide and Tabler are listed as candidates, not chosen);
   - the default language tab in the docs;
   - terms for using the logo outside the project;
   - whether subset IBM Plex files can be self-hosted under the "Plex" name (a possible OFL Reserved Font Name issue).
4. Decide whether to commit the `CLAUDE.md` addition.

**Next step:** only if the owner asks, open a separate pull request that adds the design checks to CI (`.github/workflows/docs.yml`):

```sh
python -I design/scripts/contrast.py --self-test
python -I design/scripts/build_tokens.py --check
python -I design/scripts/check_brand.py
```

Ask the owner first: it is outside the design task's scope, which was `design/`, `assets/brand/` and the wiki.

## Key decisions, constraints and instructions from the owner

- **fiscalclock is independent of workclock.** Copy nothing from it: no colours, fonts, logo idea, tokens, page names or stack.
- **Load the `frontend-design` skill before designing any logo or visual asset.**
- **The owner is a software engineer, not a designer.** Explain design choices in plain language, define each term, and give the trade-off.
- **Ask before pushing** to the repository or the wiki. **Ask before saving Canva edits** (Canva requires an explicit approval for each commit).
- **Brand:**
  - **Logo:** `[ )`, the half-open interval, in ink `#1a1a19`, with the open parenthesis in teal `#0b6b69`. Dark-theme values are `#ececea` and `#5cc6be`.
  - **Neutrals:** near-black and off-white.
  - **Type:** IBM Plex Sans and IBM Plex Mono, under the OFL only.
  - **Numbers:** tabular figures wherever numbers or dates are compared.
- **Dark mode is equal to light.** WCAG 2.2 AA is a hard floor, and `build_tokens.py` fails if any listed pair falls below it.
- **Tokens:** the token files are the single source of truth. Brand files are generated from them; never edit generated files or exported PNGs by hand.
- **Canva:** the plan has no SVG export, no Brand Kit and no resizes left. Logo SVGs are generated by script, and PNGs are rendered with Inkscape.
- **Unset defaults:** the website stack is undecided (DD-009). Never assume one.

## Gotchas and useful commands

- **Ignored `build/` folders.** The repository's `.gitignore` ignores every `build/` folder. That is why generated tokens live in `design/tokens/generated/`.
- **`sed -i` in the wiki clone.** It runs with `core.autocrlf=true`, so `sed -i` over all pages rewrites line endings in every file. Edit only the files you mean to change. If status shows phantom changes, run `git diff --ignore-cr-at-eol`, which should be empty, then check those files out again.
- **cSpell.** It uses en-GB with `project-words.txt`. Design pages allow extra words through a `<!-- cspell:words ... -->` comment on line 1.
- **Wiki checks.** markdownlint needs a config copy without the repository's globs. lychee needs `--fallback-extensions md` for links between wiki pages.
- **Headless Chrome screenshots.** Use `--force-dark-mode` for dark and `--blink-settings=preferredColorScheme=1` for light, because Windows is in dark mode. The in-app browser pane cannot load local images or emulate dark mode for local files.
- **Rebuild the brand assets.** Use a virtual environment with `design/scripts/requirements-brand.txt`, and IBM Plex Sans `IBMPlexSans[wdth,wght].ttf` from `google/fonts/ofl/ibmplexsans`, whose SHA-256 is checked:

  ```sh
  python design/scripts/build_brand_assets.py --font "path/to/IBMPlexSans[wdth,wght].ttf" --wiki-images path/to/wiki/images/design
  ```

- **The Phase 3 venv.** The scratchpad venv at `.../scratchpad/research-type/venv` has these packages, and the font is at `.../scratchpad/research-type/downloads/gf/ibmplexsans/`.
- **No failing tests.** Every design check passes on `main`.
