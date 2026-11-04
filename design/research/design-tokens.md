# Research: design tokens

<!-- cspell:words Terrazzo Docusaurus Infima VitePress Fumadocs Nextra Mintlify MkDocs Zensical Furo PyData mdBook Penpot Figma Shiki Tailwind kebab oklch oklab srgb hwb prophoto Bézier squidfunk pradyunsg withastro Ström Awn Kalbag Pygments Drasner DTCG designtokens plugin plugin's stylesheet -->

Track: design tokens for the fiscalclock design system. Everything below was checked on 1 Nov 2026 against the linked page or file. Where a statement is a recommendation or inference rather than a sourced fact, it says so.

## Summary

- The current stable specification is **Design Tokens Format Module 2025.10**, a "Final Community Group Report" dated 28 October 2025, with companion **Color** and **Resolver** modules of the same version ([technical reports index](https://www.designtokens.org/technical-reports/), [2025.10 report](https://www.designtokens.org/TR/2025.10/)). It is a Community Group report, not a W3C Standard.
- A `color` value is an object: `colorSpace` (required), `components` (required), `alpha` (optional, default 1) and `hex` (optional six-digit fallback) ([Color module, section 4.1](https://www.designtokens.org/TR/2025.10/color/)).
- Light and dark have an official mechanism: the **Resolver module**, a `.resolver.json` document with `sets` and `modifiers` whose `contexts` (for example `light` and `dark`) select token files ([Resolver module](https://www.designtokens.org/TR/2025.10/resolver/)).
- Recommendation: one primitives file, one semantic file per theme, a resolver document tying them together, and a standard-library Python generator that emits `tokens.css` with light on `:root`, dark under `prefers-color-scheme`, and a `data-theme` attribute override, plus a `--check` mode for CI.

## 1. The DTCG specification

### 1.1 Version and status (as of 1 Nov 2026)

| Item | Finding | Source |
| --- | --- | --- |
| Current stable version | 2025.10, announced as the "first stable version" on 28 October 2025 | [W3C community group announcement](https://www.w3.org/community/design-tokens/2025/10/28/design-tokens-specification-reaches-first-stable-version/) |
| Modules | Format, Color, Resolver | [2025.10 report](https://www.designtokens.org/TR/2025.10/) |
| Status label | "Final Community Group Report", 28 October 2025. The status section also says it "was published by the DTCG as a Candidate Recommendation" and that "This specification is considered stable." | [Format module](https://www.designtokens.org/TR/2025.10/format/) |
| Standards track | "It is not a W3C Standard nor is it on the W3C Standards Track." | [Format module](https://www.designtokens.org/TR/2025.10/format/) |
| W3C-hosted copies | The same three reports are in the W3C community reports index | [Format](https://www.w3.org/community/reports/design-tokens/CG-FINAL-format-20251028/), [Color](https://www.w3.org/community/reports/design-tokens/CG-FINAL-color-20251028/), [Resolver](https://www.w3.org/community/reports/design-tokens/CG-FINAL-resolver-20251028/) |
| Work in progress | A preview draft dated 8 September 2026 is still titled 2025.10 and says "Do not attempt to implement this version". No newer stable version is listed. | [Preview draft](https://www.designtokens.org/TR/drafts/), [technical reports index](https://www.designtokens.org/technical-reports/) |
| JSON Schemas | Published for the format and the resolver | [format.json](https://www.designtokens.org/schemas/2025.10/format.json), [resolver.json](https://www.designtokens.org/schemas/2025.10/resolver.json) |

One inconsistency to note: the resolver JSON Schema fixes `version` to the constant `"2025.10"` ([resolver.json](https://www.designtokens.org/schemas/2025.10/resolver.json)), and so does the designtokens.org text ([Resolver module, 4.1.2](https://www.designtokens.org/TR/2025.10/resolver/)), but the text of the W3C-hosted copy fetched on 1 Nov 2026 rendered a date-shaped value instead ([W3C copy](https://www.w3.org/community/reports/design-tokens/CG-FINAL-resolver-20251028/)). Use `"2025.10"`, which the schema and the Terrazzo docs ([Terrazzo resolvers guide](https://terrazzo.app/docs/guides/resolvers/)) both use.

### 1.2 Files and media type

From the [Format module, section 4](https://www.designtokens.org/TR/2025.10/format/):

- Media type: `application/design-tokens+json` SHOULD be used; `application/json` MAY be used; tools MUST support both.
- File extensions: `.tokens` and `.tokens.json` are both recommended. The spec says the latter "might be useful to make design token files open in users' preferred JSON editors".
- Resolver documents SHOULD use `.resolver.json` and are served as `application/json` ([Resolver module, 3.2 and 3.3](https://www.designtokens.org/TR/2025.10/resolver/)).

### 1.3 Tokens, groups and properties

From the [Format module, sections 5 and 6](https://www.designtokens.org/TR/2025.10/format/):

- A token is an object with `$value`; that is the only required property.
- `$type` is one of the type names. A token without its own `$type` inherits it from the nearest parent group or from the token it aliases. If no type can be determined, "tools MUST consider the token invalid" and not infer one from the value.
- `$description` "MUST be a plain JSON string".
- `$extensions` holds vendor data, keyed by reverse domain names; tools must keep data they do not understand.
- `$deprecated` is `true`, `false` or a string giving the reason.
- Names MUST NOT begin with `$` and MUST NOT contain `{`, `}` or `.`. Names are case-sensitive.
- A group is an object without `$value`. Groups can carry `$type`, `$description`, `$deprecated`, `$extensions` and `$extends`.
- `$root`: a group may contain a token named `$root`, referenced as `{color.accent.$root}`; `{color.accent}` is not a valid token reference.
- `$extends`: a group can inherit another group's tokens, for example `"$extends": "{button}"`, with local tokens overriding inherited ones. It "MUST NOT reference a token", and circular `$extends` chains are errors.

### 1.4 References (aliases)

From the [Format module, section 7](https://www.designtokens.org/TR/2025.10/format/):

- Curly braces: `"$value": "{colors.blue}"` resolves to the target's whole `$value`. Curly-brace references "can ONLY target complete tokens".
- JSON Pointer: `"$ref": "#/colors/blue/$value"`, or `"#/colors/blue/$value/components/0"` for one component. Tools MUST support JSON Pointer syntax.
- Chains are allowed; tools follow each reference until they reach an explicit value.
- Cycles: "References MUST NOT be circular." Tools "MUST detect and report this as an error affecting all tokens in the circular chain."
- Tools SHOULD preserve references and resolve them only when the value is needed.

### 1.5 Value shapes by type

All rows are from the [Format module, sections 8 and 9](https://www.designtokens.org/TR/2025.10/format/), except `color`, which is from the [Color module](https://www.designtokens.org/TR/2025.10/color/).

| `$type` | Value shape | Example |
| --- | --- | --- |
| `color` | Object: `colorSpace`, `components`, optional `alpha`, optional `hex` (see 1.6) | `{"colorSpace": "srgb", "components": [0, 0.4, 0.8], "hex": "#0066cc"}` |
| `dimension` | Object: `value` (number) and `unit`, which is `"px"` or `"rem"`; both required | `{"value": 0.5, "unit": "rem"}` |
| `fontFamily` | A string with one font name, or an array of names from most to least preferred | `["Helvetica", "Arial", "sans-serif"]` |
| `fontWeight` | A number in [1, 1000], or a predefined alias such as `thin`, `normal`, `semi-bold`, `bold`, `black`, `extra-black` | `700` |
| `duration` | Object: `value` (number) and `unit`, which is `"ms"` or `"s"` | `{"value": 200, "unit": "ms"}` |
| `cubicBezier` | Array of four numbers `[P1x, P1y, P2x, P2y]`; x values in [0, 1] | `[0.5, 0, 1, 1]` |
| `number` | A JSON number | `2.3` |
| `strokeStyle` | One of `solid`, `dashed`, `dotted`, `double`, `groove`, `ridge`, `outset`, `inset`; or an object with `dashArray` (dimensions) and `lineCap` (`round`, `butt`, `square`) | `"dashed"` |
| `border` | Object: `color`, `width` (dimension), `style` (strokeStyle); each may be a reference | `{"color": "{color.border}", "width": {"value": 1, "unit": "px"}, "style": "solid"}` |
| `transition` | Object: `duration`, `delay` (durations), `timingFunction` (cubicBezier) | `{"duration": "{duration.fast}", "delay": {"value": 0, "unit": "ms"}, "timingFunction": [0.5, 0, 1, 1]}` |
| `shadow` | One shadow object or an array of shadow objects and references. Each has `color`, `offsetX`, `offsetY`, `blur`, `spread` and optional boolean `inset` | `{"color": {...}, "offsetX": {"value": 0, "unit": "px"}, "offsetY": {"value": 1, "unit": "px"}, "blur": {"value": 2, "unit": "px"}, "spread": {"value": 0, "unit": "px"}}` |
| `gradient` | Array of stops (and references); each stop has `color` and `position` (number in [0, 1], clamped). No gradient kind (linear, radial) is defined. | `[{"color": {...}, "position": 0}, {"color": {...}, "position": 1}]` |
| `typography` | Object: `fontFamily`, `fontSize` (dimension), `fontWeight`, `letterSpacing` (dimension), `lineHeight` (number, a multiplier of `fontSize`) | `{"fontFamily": "{font.body}", "fontSize": {"value": 16, "unit": "px"}, "fontWeight": 400, "letterSpacing": {"value": 0, "unit": "px"}, "lineHeight": 1.5}` |

The examples in the border, transition and typography rows follow the field lists in the spec; values other than those quoted from the spec are illustrative.

Section 8.8 of the [Format module](https://www.designtokens.org/TR/2025.10/format/) lists further types (such as font style and percentage) as possible future work; they are not defined in 2025.10. The spec also keeps open issues on whether `shadow`, `gradient`, `strokeStyle` and `typography` are fit for purpose (issues 98 to 102 in the same document).

### 1.6 The `color` value

From the [Color module, section 4.1](https://www.designtokens.org/TR/2025.10/color/):

- `colorSpace` (required): one of `srgb`, `srgb-linear`, `hsl`, `hwb`, `lab`, `lch`, `oklab`, `oklch`, `display-p3`, `a98-rgb`, `prophoto-rgb`, `rec2020`, `xyz-d65`, `xyz-d50`.
- `components` (required): an array of numbers, or the keyword `"none"`, whose length and ranges depend on the space. For `srgb` each component is in [0, 1].
- `alpha` (optional): 0 to 1; if omitted it "MUST be assumed to be 1".
- `hex` (optional): a fallback that "MUST be formatted in 6 digit CSS hex color notation".

Example (from the spec):

```json
{
  "$type": "color",
  "$value": {
    "colorSpace": "srgb",
    "components": [1, 0, 1],
    "alpha": 1,
    "hex": "#ff00ff"
  }
}
```

Earlier drafts allowed a plain hex string as the value, for example `"$value": "#ff00ff"` in the [second editors' draft](https://www.designtokens.org/TR/second-editors-draft/format/). Tools written against those drafts may still expect strings, so a token file in the 2025.10 shape is not automatically readable by older tooling.

## 2. Themes and modes

### 2.1 The official mechanism: the Resolver module

The [Resolver module 2025.10](https://www.designtokens.org/TR/2025.10/resolver/) has the same "Final Community Group Report" status as the format. Its introduction names "light mode, dark mode, and high contrast color modes" as a motivating case. The parts that matter here:

- A resolver document has a required `version` (`"2025.10"`), optional `name` and `description`, `sets`, `modifiers`, and a required `resolutionOrder`.
- A **set** is a list of token sources (inline tokens or `{"$ref": "file.json"}`), merged in array order; later wins.
- A **modifier** has a `contexts` map from a name (for example `light`, `dark`) to an array of sources, and an optional `default` that must be one of the context names. Tools SHOULD error on a modifier with one context and MUST error on one with none.
- Resolution runs in fixed stages: validate the input (such as `{"theme": "dark"}`; values must be strings), flatten the `resolutionOrder`, then resolve aliases. "Aliases MUST NOT be resolved until this step", so a semantic token in `dark.tokens.json` can alias a primitive declared in another file.
- The number of outputs is the product of the context counts across modifiers. One `theme` modifier with two contexts gives two outputs.

Example adapted from the module's "Color theme modifier" example:

```json
{
  "$schema": "https://www.designtokens.org/schemas/2025.10/resolver.json",
  "version": "2025.10",
  "sets": {
    "primitives": { "sources": [{ "$ref": "primitives.tokens.json" }] }
  },
  "modifiers": {
    "theme": {
      "contexts": {
        "light": [{ "$ref": "semantic-light.tokens.json" }],
        "dark": [{ "$ref": "semantic-dark.tokens.json" }]
      },
      "default": "light"
    }
  },
  "resolutionOrder": [
    { "$ref": "#/sets/primitives" },
    { "$ref": "#/modifiers/theme" }
  ]
}
```

### 2.2 Patterns tools use today (as of 1 Nov 2026)

| Pattern | Who uses it | Source |
| --- | --- | --- |
| Resolver document with a `theme` modifier | Terrazzo calls resolvers "a DTCG standard, and the successor to legacy modes" | [Terrazzo resolvers guide](https://terrazzo.app/docs/guides/resolvers/) |
| `$extensions.mode` inside each token | Terrazzo and Cobalt UI 1.x ("legacy modes"); still read by Terrazzo, mapped to a hidden `tzMode` modifier | [Terrazzo resolvers guide](https://terrazzo.app/docs/guides/resolvers/) |
| One file per mode | Figma's variable import: "A new mode will be created for each file you import"; tokens must appear in every imported file with the same type | [Figma help: modes for variables](https://help.figma.com/hc/en-us/articles/15343816063383-Modes-for-variables) |
| Themes as combinations of token sets, saved in a themes configuration file | Tokens Studio (a Pro feature) | [Tokens Studio themes](https://docs.tokens.studio/manage-themes/themes-overview) |

The "one semantic file per theme" layout fits all four rows: it is what a resolver `contexts` entry points at, and it is what Figma imports as modes.

## 3. Tooling (as of 1 Nov 2026)

| Tool | Version | Licence | DTCG 2025.10 support | Source |
| --- | --- | --- | --- | --- |
| Style Dictionary | 5.7.1 (24 Oct 2026); requires Node.js 22 or later; 13 direct dependencies | Apache-2.0 | Partial. Added 2025.10 colour objects (5.3.0), dimension objects (5.4.0), duration objects and `$root` naming (5.7.0). The tracking issue "Support for DTCG v2025.10", which lists resolvers as not yet done, is open. The changelog has no entry for resolver, `$extends` or `$ref` support. Its own docs say 2025.10 "does not have full support yet". | [npm registry](https://registry.npmjs.org/style-dictionary), [changelog](https://github.com/style-dictionary/style-dictionary/blob/main/CHANGELOG.md), [issue 1590](https://github.com/style-dictionary/style-dictionary/issues/1590), [DTCG page](https://styledictionary.com/info/dtcg/) |
| Terrazzo (formerly Cobalt UI) | `@terrazzo/cli` and `@terrazzo/plugin-css` 2.10.0 (21 Oct 2026); the CLI has 18 direct dependencies, including Vite | MIT | Reads resolver documents (`version: "2025.10"`) and keeps reading legacy modes. The CSS plugin emits one block per permutation, wrapped by a `prepare` function you write (for example `@media (prefers-color-scheme: dark)` or `[data-theme="dark"]`). | [npm registry](https://registry.npmjs.org/@terrazzo/cli), [repository](https://github.com/terrazzoapp/terrazzo), [resolvers guide](https://terrazzo.app/docs/guides/resolvers/), [CSS plugin](https://terrazzo.app/docs/integrations/css/) |
| Tokens Studio (Figma plugin) | Plugin repository active on 30 Oct 2026 | MIT (plugin repository) | Offers a "W3C DTCG" format option; the default is legacy. The docs link the spec as a draft and do not name 2025.10. | [token format docs](https://docs.tokens.studio/manage-settings/token-format), [repository](https://github.com/tokens-studio/figma-plugin) |
| Figma variables | Hosted product | Proprietary | Native import of DTCG JSON, one mode per file. Import limits: colours in `hsl` or `srgb` only, dimensions in `px` only, single-name `fontFamily` only. Export by "Export mode"; the page does not state the export format. | [Figma help: modes for variables](https://help.figma.com/hc/en-us/articles/15343816063383-Modes-for-variables) |
| Penpot | 2.18.5 (26 Oct 2026) | MPL-2.0 | Penpot's blog says it supports the DTCG JSON format. A community report (4 May 2026) says the export is the Tokens Studio multi-set shape, not DTCG 2025.10; a Penpot staff reply thanked the poster and did not address DTCG 2025.10 plans. | [releases](https://github.com/penpot/penpot/releases), [repository](https://github.com/penpot/penpot), [blog guide](https://penpot.app/blog/a-practical-guide-to-the-design-tokens-json-format/), [community thread](https://community.penpot.app/t/token-export-is-tokens-studio-format-not-dtcg-2025-10/10544) |

## 4. Mapping tokens to CSS custom properties

### 4.1 Names

- Custom property names start with `--` and "are case sensitive" ([MDN: using custom properties](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Cascading_variables/Using_custom_properties)).
- Common practice is the token path joined with hyphens. Style Dictionary's built-in name transforms produce names such as `--color-text-warning` ([changelog 5.7.0](https://github.com/style-dictionary/style-dictionary/blob/main/CHANGELOG.md)); Terrazzo's CSS docs show `kebabCase(token.id)` ([CSS plugin](https://terrazzo.app/docs/integrations/css/)).
- Recommendation: restrict token names to lowercase ASCII letters, digits and single hyphens, so the CSS name is the path joined by `-` with no escaping, and add a short project prefix (for example `--fc-`) so tokens cannot collide with framework variables such as Tailwind's `--color-*` ([Tailwind theme variables](https://tailwindcss.com/docs/theme)) or Fumadocs' `--color-fd-*` ([Fumadocs theme](https://fumadocs.dev/docs/ui/theme)). The prefix itself is the owner's choice.

### 4.2 Emitting light and dark

Recommended output (the hex values are illustrative, not a proposed palette):

```css
:root {
  color-scheme: light;
  --fc-color-bg-default: #ffffff;
}

@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {
    color-scheme: dark;
    --fc-color-bg-default: #0b0d10;
  }
}

:root[data-theme="dark"] {
  color-scheme: dark;
  --fc-color-bg-default: #0b0d10;
}
```

How it behaves:

- With no attribute, the page follows the operating system through `prefers-color-scheme` (Baseline widely available since January 2020, per [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@media/prefers-color-scheme)).
- `data-theme="dark"` forces dark. `data-theme="light"` excludes the media block, so the `:root` light values apply even when the system is dark. The user's toggle therefore always wins.
- Specificity: `:not()` takes the specificity of its argument ([MDN: `:not()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/:not)), so both dark selectors outrank plain `:root`.
- `color-scheme` sets the canvas, scrollbar and form-control colours ([MDN: `color-scheme`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/color-scheme)). MDN also recommends `<meta name="color-scheme">` in the page head, "helping prevent unwanted screen flashes during the page load". A toggle script that sets the attribute must run before first paint; Tailwind's docs give the same advice for their equivalent script ([Tailwind dark mode](https://tailwindcss.com/docs/dark-mode)).
- The dark values appear twice. The generator writes both blocks, so nobody edits them by hand.
- The selector strings are the only framework-specific part. A generator option for the dark selector (for example `.dark`, `[data-theme="dark"]` or `[data-md-color-scheme="slate"]`) covers every framework in section 5.

### 4.3 The `light-dark()` alternative

`light-dark(a, b)` returns the first value under a light colour scheme and the second under dark. It needs `color-scheme: light dark` to respond to the system ([MDN: `light-dark()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/color_value/light-dark)). Support as of 1 Nov 2026: Baseline "Newly available" since 13 May 2024 (Chrome and Edge 123, Firefox 120, Safari 17.5), and "Expected to be Widely Available on 2026-11-13" ([web-features explorer](https://web-platform-dx.github.io/web-features-explorer/features/light-dark/)). It accepts only colours or images, so it cannot switch dimensions, fonts or durations.

```css
:root { color-scheme: light dark; --fc-color-bg-default: light-dark(#ffffff, #0b0d10); }
:root[data-theme="light"] { color-scheme: light; }
:root[data-theme="dark"] { color-scheme: dark; }
```

Recommendation: emit the selector-block form as the default and treat `light-dark()` as an optional second output. The block form handles any token type, does not depend on a feature that only becomes Baseline widely available 12 days after this check, and matches the selector-based switching that most frameworks below already use. PyData Sphinx Theme's docs already show `light-dark()` for its own variables ([PyData styling](https://pydata-sphinx-theme.readthedocs.io/en/stable/user_guide/styling.html)), so the second output would be useful there.

## 5. How docs frameworks consume tokens

No framework is chosen (decision DD-009 in [the decision log](../DECISIONS.md)). Versions are the latest published releases on 1 Nov 2026, from each package registry. "Plain CSS enough?" asks whether one generated stylesheet can theme the framework without writing JavaScript or framework config.

| Framework (version) | Colour variables | Font variables | Code blocks | Dark selector | Plain CSS enough? | Docs |
| --- | --- | --- | --- | --- | --- | --- |
| Fumadocs UI (16.17.9) | 16 variables: `--color-fd-background`, `--color-fd-foreground`, `--color-fd-muted`, `--color-fd-primary`, `--color-fd-accent`, `--color-fd-border`, `--color-fd-ring` and their `-foreground` pairs | Not documented on the theme page | Not documented on the theme page | `.dark` (page example; dark mode through `next-themes`) | Partly. "Only Tailwind CSS v4 is supported"; overrides are written in Tailwind `@theme` and `.dark`. | [Theme](https://fumadocs.dev/docs/ui/theme) |
| Astro Starlight (0.42.8) | `--sl-color-accent-low`, `--sl-color-accent`, `--sl-color-accent-high`, `--sl-color-white`, `--sl-color-gray-1` to `--sl-color-gray-6`, `--sl-color-black`, plus `--sl-color-text`, `--sl-color-bg`, `--sl-color-hairline` | `--sl-font`, `--sl-font-mono` | Expressive Code; `useStarlightUiThemeColors` uses Starlight's variables for code-block UI; syntax colours come from the `starlight-dark` and `starlight-light` themes | Dark is the default on `:root`; light is `:root[data-theme='light']`. The grey scale is inverted between themes. | Yes, via `customCss` | [CSS guide](https://starlight.astro.build/guides/css-and-tailwind/), [configuration](https://starlight.astro.build/reference/configuration/), [props.css](https://github.com/withastro/starlight/blob/main/packages/starlight/src/style/props.css) |
| Docusaurus with Infima (3.10.2) | `--ifm-color-primary` and six shades (`-dark`, `-darker`, `-darkest`, `-light`, `-lighter`, `-lightest`), `--ifm-background-color`, `--ifm-font-color-base` | `--ifm-font-family-base`, `--ifm-font-family-monospace` | `--ifm-code-font-size`, `--docusaurus-highlighted-code-line-bg`; syntax colours come from a Prism theme, which is a JavaScript object in `docusaurus.config.js` | `[data-theme='dark']` on `<html>` | UI: yes, in `custom.css`. Syntax colours: need the JS theme object. | [Styling](https://docusaurus.io/docs/styling-layout), [code blocks](https://docusaurus.io/docs/markdown-features/code-blocks), [Infima variables](https://github.com/facebookincubator/infima/blob/main/packages/core/styles/common/variables.pcss) |
| VitePress (1.6.4) | `--vp-c-brand-1`, `--vp-c-brand-2`, `--vp-c-bg`, `--vp-c-text-1`, `--vp-c-divider` and more | `--vp-font-family-base`, `--vp-font-family-mono` | `--vp-code-color`, `--vp-code-block-bg`, `--vp-code-tab-*` | `.dark` (in `vars.css`) | Yes; "The default theme CSS is customizable by overriding root level CSS variables." | [Extending the theme](https://vitepress.dev/guide/extending-default-theme), [vars.css](https://github.com/vuejs/vitepress/blob/main/src/client/theme-default/styles/vars.css) |
| Nextra (4.6.1) | `--nextra-primary-hue`, `--nextra-primary-saturation`, `--nextra-primary-lightness`, `--nextra-bg`, set from the `<Head>` props `color.hue`, `color.saturation`, `color.lightness` and `backgroundColor` | Not documented in the pages checked | Not documented in the pages checked | `.dark` (in the `Head` source) | Partly. The accent is stored as separate HSL numbers and the background is used as `rgb(var(--nextra-bg))`, so a hex token must be converted first. | [Head](https://nextra.site/docs/built-ins/head), [head.tsx source](https://github.com/shuding/nextra/blob/main/packages/nextra/src/client/components/head.tsx) |
| Mintlify (hosted; `mint` CLI 4.2.1184, Elastic-2.0) | `docs.json`: `colors.primary`, `colors.light`, `colors.dark` (hex), `background.color.light` and `.dark` | `docs.json` `fonts` | `styling.codeblocks`; the `"css-variables"` theme reads `--mint-color-text`, `--mint-color-background`, `--mint-token-*`, `--mint-ansi-*` | Not documented in the pages checked | Partly. Brand colours live in `docs.json`. Any `.css` file in the content directory is loaded, but the docs warn that element styling is "subject to change". | [Appearance](https://www.mintlify.com/docs/organize/settings-appearance), [code](https://www.mintlify.com/docs/create/code), [custom CSS](https://www.mintlify.com/docs/customize/custom-scripts) |
| Material for MkDocs (9.8.0) | `--md-primary-fg-color` (with `--light` and `--dark`), `--md-accent-fg-color`, `--md-default-fg-color`, `--md-default-bg-color`, `--md-typeset-a-color` | `--md-text-font`, `--md-code-font` | `--md-code-fg-color`, `--md-code-bg-color` | `[data-md-color-scheme="default"]` and `[data-md-color-scheme="slate"]`, or a named custom scheme; palettes can carry `media` queries | Yes, via `extra_css` | [Colours](https://squidfunk.github.io/mkdocs-material/setup/changing-the-colors/), [fonts](https://squidfunk.github.io/mkdocs-material/setup/changing-the-fonts/), [customisation](https://squidfunk.github.io/mkdocs-material/customization/), [_colors.scss](https://github.com/squidfunk/mkdocs-material/blob/master/src/templates/assets/stylesheets/main/_colors.scss) |
| Zensical (0.0.74, pre-1.0) | Same `--md-*` variables; set `primary` or `accent` to `custom` | Same as Material | Same as Material | Same `default` and `slate` schemes | Yes. It reads `mkdocs.yml`, and additional CSS works "without changes when using the `classic` theme". | [Colours](https://zensical.org/docs/setup/colors/), [MkDocs compatibility](https://zensical.org/docs/compatibility/mkdocs/) |
| Sphinx with Furo (2025.12.19) | `light_css_variables` and `dark_css_variables` dictionaries in `conf.py`, keyed without the `--` (for example `color-brand-primary`, `color-brand-content`) | `font-stack`, `font-stack--monospace`, `font-stack--headings` | Pygments: `pygments_style` and `pygments_dark_style` | Set by Furo; dark inherits light's values and overrides some | Partly. The documented mechanism is the `conf.py` dictionaries, and typos in them "are silently ignored". | [Customisation](https://pradyunsg.me/furo/customisation/), [colours](https://pradyunsg.me/furo/customisation/colors/), [fonts](https://pradyunsg.me/furo/customisation/fonts/), [variables](https://github.com/pradyunsg/furo/tree/main/src/furo/assets/styles/variables) |
| Sphinx with PyData theme (0.23.0) | `--pst-color-primary`, `--pst-color-secondary`, and accent, success, info, warning, danger, background, on-background, surface, on-surface and target roles | `--pst-font-family-base`, `--pst-font-family-monospace`, `--pst-font-family-heading` | Pygments: `pygments_light_style`, `pygments_dark_style` | `html[data-theme="light"]` and `html[data-theme="dark"]`. Use `html[data-theme]`, not `:root`, because the theme's own rules are more specific. | Yes, via `html_css_files`; code colours need `conf.py` | [Styling](https://pydata-sphinx-theme.readthedocs.io/en/stable/user_guide/styling.html) |
| mdBook (0.5.4) | `--bg`, `--fg`, `--sidebar-bg`, `--links`, `--inline-code-color` and others, scoped per theme class | `--mono-font`, `--code-font-size` | `highlight.css` (an overridable theme file) | Theme classes `.light`, `.rust`, `.coal`, `.navy`, `.ayu`; `default-theme` defaults to `light`, `preferred-dark-theme` to `navy` | Yes. Override the theme-class blocks through `additional-css`, or replace `css/variables.css`. | [Theme](https://rust-lang.github.io/mdBook/format/theme/index.html), [renderer options](https://rust-lang.github.io/mdBook/format/configuration/renderers.html), [variables.css](https://github.com/rust-lang/mdBook/blob/main/crates/mdbook-html/front-end/css/variables.css) |
| Tailwind CSS (4.3.3) | `@theme` namespaces `--color-*`, `--font-*`, `--text-*`, `--radius-*`, `--shadow-*`, `--ease-*` generate utilities and are emitted as custom properties on `:root` | `--font-*` | Not applicable | The `dark` variant uses `prefers-color-scheme` by default; `@custom-variant dark (&:where([data-theme=dark], [data-theme=dark] *));` switches it to an attribute | Yes. `@theme inline { --color-bg: var(--fc-color-bg-default); }` maps utilities onto token variables; the docs say `inline` makes the utility use the `var()` reference directly. | [Theme variables](https://tailwindcss.com/docs/theme), [dark mode](https://tailwindcss.com/docs/dark-mode) |

What this means (inference from the table): every framework except Mintlify and Nextra can be themed with a stylesheet that assigns its variables from `--fc-*` tokens under its own dark selector. Mintlify, Nextra and Furo also need values in configuration (hex in `docs.json`, HSL numbers in `<Head>`, a Python dictionary in `conf.py`). A generator that writes a resolved JSON file per theme covers those cases without hand-copied hex.

## 6. Recommended token architecture

This section is a recommendation, not a sourced fact.

### 6.1 Tiers

1. **Primitives** (`primitives.tokens.json`): the raw palette and scales, with no meaning attached, for example `palette.neutral.50` to `palette.neutral.950`, `palette.accent.50` to `palette.accent.950`, and a few status hues. Also font families, a type scale, spacing, radii and durations. Primitives do not change between themes.
2. **Semantic** (`semantic-light.tokens.json`, `semantic-dark.tokens.json`): role-based aliases with identical paths in both files, for example:
   - `color.bg.default`, `color.bg.subtle`, `color.bg.raised`
   - `color.text.default`, `color.text.muted`, `color.text.on-accent`
   - `color.border.default`, `color.border.strong`
   - `color.accent.default`, `color.accent.hover`, `color.link.default`
   - `color.focus.ring`
   - `color.status.success`, `color.status.warning`, `color.status.danger`, `color.status.info`
   - `color.code.bg`, `color.code.text`
3. **Component** tokens: only if a later phase needs them (for example a `calendar-cell` background). They alias semantic tokens, never primitives.

### 6.2 Naming rules

- Lowercase ASCII letters, digits and hyphens in each path segment; this stays inside the spec's restrictions on `$`, `{`, `}` and `.` ([Format, 5.1.1](https://www.designtokens.org/TR/2025.10/format/)) and needs no CSS escaping.
- Use a `default` leaf rather than `$root`. `$root` is valid 2025.10 syntax, but Style Dictionary only added name handling for it in 5.7.0 ([changelog](https://github.com/style-dictionary/style-dictionary/blob/main/CHANGELOG.md)).
- Use only curly-brace aliases between complete tokens. Avoid `$extends` and JSON Pointer `$ref` inside token files until the tools in section 3 document support for them.
- Use `colorSpace: "srgb"` and always include `hex`. Figma imports only `hsl` and `srgb` colours ([Figma help](https://help.figma.com/hc/en-us/articles/15343816063383-Modes-for-variables)), and the project's existing contrast script works on sRGB hex (`design/scripts/contrast.py`).
- Give each semantic token a `$description` saying where it is used; the spec lets translation tools emit it as a code comment ([Format, 5.2.1](https://www.designtokens.org/TR/2025.10/format/)).

### 6.3 How many tokens

About 15 to 25 semantic colour tokens per theme should be enough to feed every framework in section 5. Two data points: Fumadocs exposes 16 colour variables ([Fumadocs theme](https://fumadocs.dev/docs/ui/theme)), and PyData documents 2 named colours plus 10 colour roles ([PyData styling](https://pydata-sphinx-theme.readthedocs.io/en/stable/user_guide/styling.html)). Primitives can be larger (two or three 10-step ramps), since only semantic tokens are meant to be used directly.

### 6.4 One source of truth for hex values

- Token files are the only place a colour is written. `tokens.css`, framework adapters, the docs palette page and any colour list in brand guidelines are generated from them.
- The generated `tokens.css` is committed, and CI reruns the generator with `--check`, failing if the output differs. This matches how the repository already reruns `research/verification/verify_claims.py` and diffs the result against committed output (see `AGENTS.md`).
- A CI check that each colour's `hex` matches its `components`. Arithmetic note: storing sRGB components to 4 decimal places always round-trips, because a step of 1/255 (about 0.0039) is far larger than 0.0001, so `round(component * 255)` recovers the byte exactly.
- A CI check that every `fill` and `stroke` colour in the SVG logo masters (decision DD-008) is a palette primitive.
- The contrast pairs that matter (text on background, accent on background, focus ring) are listed once and checked in both themes with `design/scripts/contrast.py`, since dark mode is first-class (decision DD-005).

## 7. Generator

### 7.1 Standard-library Python design (outline)

A script such as `design/scripts/build_tokens.py`, run as `python -I design/scripts/build_tokens.py [--check]`:

1. **Load** the resolver document and every `$ref` file with `json`, using `object_pairs_hook` to reject duplicate keys ([json docs](https://docs.python.org/3/library/json.html)). Reject anything other than `version == "2025.10"`, and reject `$ref` values that are not relative paths inside `design/tokens/`.
2. **Validate the subset** the project uses: groups and tokens only; names match the project pattern; every token has a `$type` (its own or inherited from a group); values match the shapes in section 1.5; unknown `$`-properties are errors; `$extensions` is kept but ignored.
3. **Flatten per context**: for each `theme` context, merge the sources in `resolutionOrder`, later wins, as in the [Resolver module, section 6.2](https://www.designtokens.org/TR/2025.10/resolver/).
4. **Resolve aliases** after flattening (Resolver 6.3). Walk each `{path}` depth first with a stack of tokens being resolved, as the spec's algorithm describes ([Format, 7.2.1](https://www.designtokens.org/TR/2025.10/format/)). Fail on:
   - an unresolved reference (report the token, the file and the missing path);
   - a circular reference (report every token in the chain, as [Format, 7.2.3](https://www.designtokens.org/TR/2025.10/format/) requires);
   - a type mismatch between the alias and its target.
5. **Check theme parity**: both contexts define exactly the same semantic paths with the same types, so no token exists in only one theme. Figma's import has the same rule ([Figma help](https://help.figma.com/hc/en-us/articles/15343816063383-Modes-for-variables)).
6. **Emit CSS**: primitives once on `:root`. Semantic tokens per theme as `var(--fc-…)` references to primitives, which keeps aliases visible as the spec recommends (Format 7.2). Use the selector layout of section 4.2, with the dark selector configurable. Value conversion:
   - `color` with alpha 1: `hex`; otherwise an `rgb()` or 8-digit hex form
   - `dimension` and `duration`: `value` followed by `unit`
   - `cubicBezier`: `cubic-bezier(...)`
   - `fontFamily`: a comma-separated list in which family names are quoted and CSS generic family keywords such as `sans-serif` and `system-ui` stay unquoted, because a quoted keyword is read as the name of a family ([CSS Fonts Level 4, `font-family`](https://www.w3.org/TR/css-fonts-4/#font-family-prop)). The generator also leaves any single all-lowercase word unquoted. `build_tokens.py --self-test` checks a mixed list.
   - `shadow`: a `box-shadow` list
   - `border`: shorthand
   - `typography`: one variable per sub-value
7. **Emit adapters** (optional): a resolved JSON file per theme for configuration-driven frameworks (Mintlify, Nextra, Furo).
8. **Be deterministic**: sorted paths, LF line endings, a "generated, do not edit" header naming the source files, and no timestamps.
9. **`--check`**: build in memory, compare with the committed file, print a unified diff and exit 1 on any difference.
10. **`--self-test`**: built-in fixtures for a cycle, a missing target, a type mismatch and a theme-parity gap, following the existing `contrast.py --self-test` convention.

### 7.2 Write our own, or use Style Dictionary or Terrazzo?

| Option | For | Against |
| --- | --- | --- |
| Standard-library Python script | No dependencies; runs on the Python CI already uses; same habits as `contrast.py` and `verify_claims.py`; output is fully under project control; only the spec subset in use needs implementing | The project owns its spec conformance; no colour-space conversion beyond what it writes; each new token type needs code |
| Style Dictionary 5.7.1 | Widely used; many output formats; Apache-2.0 | Node.js 22 or later and 13 direct dependencies; 2025.10 support is still partial and the Resolver module is not supported (as of 1 Nov 2026), so light and dark would need two separate builds configured by hand ([issue 1590](https://github.com/style-dictionary/style-dictionary/issues/1590), [changelog](https://github.com/style-dictionary/style-dictionary/blob/main/CHANGELOG.md)) |
| Terrazzo 2.10.0 | Reads 2025.10 resolver documents directly; the CSS plugin's permutations map onto light and dark; MIT ([resolvers guide](https://terrazzo.app/docs/guides/resolvers/), [CSS plugin](https://terrazzo.app/docs/integrations/css/)) | Node.js toolchain; the CLI has 18 direct dependencies, including Vite ([npm registry](https://registry.npmjs.org/@terrazzo/cli)); output format is the plugin's, adjusted through callbacks |

Recommendation: write the standard-library generator for the subset in section 6, and keep the token files strictly 2025.10-conformant (resolver document included) so Terrazzo can read them unchanged. Optionally, run Terrazzo in a non-blocking CI job as an independent second reading of the same files and compare resolved values. This mirrors the repository's practice of checking results against independent generators (`conformance/generators/`). Switching to Terrazzo later would then mean changing the build step, not rewriting the tokens.

## 8. Open questions for the owner

- The CSS prefix (`--fc-` is a placeholder).
- Whether type sizes are `px` (what Figma imports) or `rem` (both are valid 2025.10 units; [Format, 8.2](https://www.designtokens.org/TR/2025.10/format/)).
- Whether to also emit the `light-dark()` variant after it becomes Baseline widely available (expected 13 Nov 2026, per the [web-features explorer](https://web-platform-dx.github.io/web-features-explorer/features/light-dark/)).
- Where the generated `tokens.css` lives (for example `design/tokens/tokens.css`) and whether it is published as a package for a future website.
