<!-- cspell:words Canva DTCG Furo Mintlify Nextra stylesheet Terrazzo WCAG -->

# Design tokens

Design tokens are named design values, such as `color.text.default` or `space.4`, stored as data so that the website, diagrams, social cards and documentation all use the same numbers. The files in this folder are the **single source of truth** for fiscalclock's colours, type, spacing and motion. If a value appears anywhere else, it was copied from here; change it here first.

The format is the W3C Design Tokens Community Group (DTCG) [2025.10 report](https://www.designtokens.org/TR/2025.10/format/), with the [Color](https://www.designtokens.org/TR/2025.10/color/) and [Resolver](https://www.designtokens.org/TR/2025.10/resolver/) modules. The design pages on the [wiki](https://github.com/Karthik-beta/fiscalclock/wiki/Design-Tokens) explain each token's purpose.

## Files

| File | What it holds |
| --- | --- |
| [`primitives.tokens.json`](primitives.tokens.json) | Raw values with no meaning attached: the colour palette, font families, the type scale, spacing, radii, breakpoints, layout widths, durations and easing. The same in both themes. |
| [`semantic.light.tokens.json`](semantic.light.tokens.json) | Role-based colours for the light theme, such as `color.text.default` and `color.accent.default`. Each is an alias of a palette colour. |
| [`semantic.dark.tokens.json`](semantic.dark.tokens.json) | The same roles, with the same names, for the dark theme. |
| [`fiscalclock.resolver.json`](fiscalclock.resolver.json) | The resolver document: primitives first, then one `theme` modifier with `light` and `dark` contexts. |
| [`contrast-checks.json`](contrast-checks.json) | Colour pairs checked against WCAG 2.2 in both themes on every build. |
| [`generated/`](generated/) | Generated files. Never edit them by hand. |

Use semantic tokens in products. Primitives exist so that semantic tokens can alias them; using `palette.teal.700` directly in a page would bypass the theme switch.

## Building

```sh
python -I design/scripts/build_tokens.py            # regenerate generated/
python -I design/scripts/build_tokens.py --check    # fail if generated/ is out of date
python -I design/scripts/build_tokens.py --self-test
```

The script uses only the Python standard library. It rejects any part of the format the files do not use, checks that every colour's `hex` matches its `components`, resolves aliases (failing on missing, circular or mistyped references), requires both themes to define exactly the same tokens, and fails if any pair in `contrast-checks.json` falls below its minimum.

| Output | Use |
| --- | --- |
| [`generated/tokens.css`](generated/tokens.css) | CSS custom properties, prefixed `--fc-`. |
| [`generated/tokens.light.json`](generated/tokens.light.json), [`generated/tokens.dark.json`](generated/tokens.dark.json) | Every token resolved to a plain CSS value, for tools that take values in configuration rather than CSS. |
| [`generated/contrast.md`](generated/contrast.md) | The contrast report, with every checked pair's ratio in both themes. |

## How the website will consume them

No website stack has been chosen yet (decision DD-009 in [`DECISIONS.md`](../DECISIONS.md)), so the tokens assume none.

1. **Load `tokens.css` first**, before any framework stylesheet. It defines `--fc-*` variables only and styles nothing.
2. **Map the framework's own variables onto the tokens** in one small stylesheet, for example `--vp-c-bg: var(--fc-color-bg-default);` for VitePress, or `--sl-color-accent: var(--fc-color-accent-default);` for Starlight. Most docs frameworks can be themed this way; the [token research](../research/design-tokens.md#5-how-docs-frameworks-consume-tokens) lists each framework's variables and dark-mode selector.
3. **Set the theme on `<html>`.** With no attribute, the page follows the operating system. `data-theme="dark"` or `data-theme="light"` overrides it, so a theme switch always wins. Run the script that sets the attribute before the first paint, and add `<meta name="color-scheme" content="light dark">` to avoid a flash of the wrong theme.
4. **For frameworks configured with values instead of CSS** (Mintlify's `docs.json`, Nextra's colour props, Furo's `conf.py`), read the values from `tokens.light.json` and `tokens.dark.json` at build time rather than copying hex values by hand.
5. **Breakpoints** cannot be read from custom properties inside media queries. Copy the `breakpoint.*` values into the site's media queries and keep them in step.

Example:

```css
@import "tokens.css";

body {
  background: var(--fc-color-bg-default);
  color: var(--fc-color-text-default);
  font-family: var(--fc-type-body-font-family);
  font-size: var(--fc-type-body-font-size);
  line-height: var(--fc-type-body-line-height);
}

table, time, .date { font-variant-numeric: tabular-nums lining-nums; }

:focus-visible {
  outline: var(--fc-border-width-thick) solid var(--fc-color-focus-ring);
  outline-offset: 2px;
}
```

## Changing a token

1. Edit the token file, then run the build.
2. If the change touches a colour, the build reruns every contrast check; fix any failure by changing the colour, never by removing the check.
3. Record the change and its reason in [`DECISIONS.md`](../DECISIONS.md), and update the wiki pages that show the value.
4. If it is a brand colour, regenerate the logo and card exports in [`assets/brand/`](../../assets/brand/) and update the Canva designs listed on the wiki's asset inventory.

The files follow the 2025.10 report strictly, so an independent tool such as Terrazzo can read them unchanged as a cross-check.
