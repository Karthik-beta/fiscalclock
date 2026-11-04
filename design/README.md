<!-- cspell:words Inkscape lockups WCAG -->

# Design

This folder holds fiscalclock's visual identity and design system as files that can be checked: the decisions behind it, the research they rest on, the design tokens, and the scripts that build and check everything. The rules themselves are explained on the [wiki's design pages](https://github.com/Karthik-beta/fiscalclock/wiki/Design-System).

| Path | What it holds |
| --- | --- |
| [`DECISIONS.md`](DECISIONS.md) | Every design decision: the options considered, the choice and the reason. |
| [`research-brief.md`](research-brief.md) | The research summary the identity is built on. |
| [`research/`](research/) | Seven cited research notes: peer sites, trust and credibility, accessibility, design tokens, typography, asset specifications and calendar diagrams. |
| [`tokens/`](tokens/) | The design tokens (colours, type, spacing, motion), the single source of truth for every value. See its [README](tokens/README.md). |
| [`scripts/`](scripts/) | `contrast.py` (WCAG 2.2 contrast ratios), `build_tokens.py` (tokens to CSS and JSON, with contrast checks), `check_brand.py` (assets and documents use only token values) and `build_brand_assets.py` (logo, icons, cards and banner). |
| [`explorations/`](explorations/) | The three brand directions compared in Phase 3, kept as a record; direction C was chosen (DD-013). |

The brand files themselves are in [`assets/brand/`](../assets/brand/).

## Checks

The first three need only the Python standard library:

```sh
python -I design/scripts/contrast.py --self-test
python -I design/scripts/build_tokens.py --check
python -I design/scripts/check_brand.py
```

Rebuilding the brand assets needs the packages in [`scripts/requirements-brand.txt`](scripts/requirements-brand.txt), the IBM Plex Sans font file and Inkscape 1.4; the script's docstring has the details.
