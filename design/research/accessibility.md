# Accessibility research: constraints for the fiscalclock identity

Research track for the visual identity and design system (see [`../DECISIONS.md`](../DECISIONS.md), DD-005: dark mode is first-class). Checked on 1 Nov 2026. Every rule below links to its source; quotations are verbatim and were checked against the linked page. Where the evidence is weak or missing, this file says so.

<!-- cspell:ignore WCAG wordmark HFES callouts reflow hoverable TVCG Colorblind Okabe Machado Oliveira Fernandes halation Piepenbrock Mayr Mund Buchner Dobres Chahine Reimer Legge protan deutan tritan protanomaly deuteranomaly tritanomaly dichromacy pallete Myndex APCA AGWG sron pault jfly koeln ufrgs Tol Budiu nngroup levelaccess dequelabs Steiner -->

## Summary of hard constraints

WCAG 2.2 is the hard constraint, in both themes, for every pairing. The [WCAG 2.2 Recommendation](https://www.w3.org/TR/WCAG22/) is dated 12 December 2024 (as of 1 Nov 2026).

| Element | Minimum ratio we must meet | Source | Target we should aim for |
| --- | --- | --- | --- |
| Body text, UI text, code, captions (normal size) | **4.5:1** (AA) | [SC 1.4.3](https://www.w3.org/TR/WCAG22/#contrast-minimum) | 7:1 ([SC 1.4.6](https://www.w3.org/TR/WCAG22/#contrast-enhanced), AAA) for body text and code |
| Large text (at least 18pt, or 14pt bold) | **3:1** (AA) | [SC 1.4.3](https://www.w3.org/TR/WCAG22/#contrast-minimum) | 4.5:1 (AAA) |
| UI component boundaries and states, focus indicators, icons without text, diagram lines, bars, slices, markers | **3:1** against adjacent colours (AA) | [SC 1.4.11](https://www.w3.org/TR/WCAG22/#non-text-contrast) | 3:1 against every neighbour, not only the background |
| Focus indicator change | **3:1** between focused and unfocused pixels, area at least a 2 CSS px perimeter (AAA, adopted as our target) | [SC 2.4.13](https://www.w3.org/TR/WCAG22/#focus-appearance) | Same in both themes |
| Inline links distinguished only by colour | **3:1** against surrounding text, plus a non-colour cue on hover and focus | [G183](https://www.w3.org/WAI/WCAG22/Techniques/general/G183), [SC 1.4.1](https://www.w3.org/TR/WCAG22/#use-of-color) | Underline links in prose; then no 3:1 link-to-text rule is needed |
| Pointer targets | **24 × 24 CSS px** or the spacing exception (AA) | [SC 2.5.8](https://www.w3.org/TR/WCAG22/#target-size-minimum) | 44 × 44 CSS px ([SC 2.5.5](https://www.w3.org/TR/WCAG22/#target-size-enhanced), AAA) for primary controls |

Thresholds are compared against the unrounded ratio: [Understanding 1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html) says "4.499:1 would not meet the 4.5:1 threshold", and [Understanding 1.4.11](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html) gives the same rule for 2.999:1 against 3:1.

## 1. Contrast requirements

### 1.4.3 Contrast (Minimum), Level AA

[SC 1.4.3](https://www.w3.org/TR/WCAG22/#contrast-minimum) requires text and images of text to have a contrast ratio of at least 4.5:1, with three exceptions:

- **Large text:** large-scale text and images of it need at least 3:1.
- **Incidental:** no requirement for text that is part of an inactive user interface component, is pure decoration, is not visible to anyone, or is part of a picture that contains significant other visual content.
- **Logotypes:** "Text that is part of a logo or brand name has no contrast requirement."

Limits of the exemptions, from [Understanding 1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html):

- Text with low contrast because of brand guidelines is not exempt; only logos and logotypes are. For us: the wordmark is exempt, but brand colours used for headings, links, buttons, badges or diagram labels are not.
- A logo shown at low contrast by default and at full contrast only on hover is not exempt, because the low-contrast presentation is not essential (Figure 3 in that document).
- The "picture that contains significant other visual content" exception is meant for photos that happen to contain text, not for diagrams whose labels carry the information. Our period and timeline diagrams are not exempt: their labels are text and must meet 4.5:1 (or 3:1 if large).

### 1.4.6 Contrast (Enhanced), Level AAA

[SC 1.4.6](https://www.w3.org/TR/WCAG22/#contrast-enhanced): at least 7:1 for text, 4.5:1 for large-scale text, with the same incidental and logotype exceptions. [Understanding 1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html) explains the numbers: 3:1 comes from ISO 9241-3 and ANSI-HFES-100-1988 for standard vision; 4.5:1 accounts for 20/40 acuity (3 × 1.5); 7:1 corresponds to about 20/80 acuity.

### Definition of large-scale text

The [WCAG 2.2 definition of large scale (text)](https://www.w3.org/TR/WCAG22/#dfn-large-scale) is "at least 18 point or 14 point bold" or the equivalent size for Chinese, Japanese and Korean fonts. [Understanding 1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html) gives the CSS conversion as 1pt = 1.333px, so:

| Rule | Points | CSS pixels (as Understanding 1.4.3 rounds them) | Exact at 1pt = 4/3 px |
| --- | --- | --- | --- |
| Large text, regular weight | 18pt | about 24px | 24px |
| Large text, bold | 14pt | about 18.5px | 18.67px |

So 18px bold text is **not** large (it is below 18.67px), and 20px regular text is **not** large. The definition's notes say the size is the size as delivered (not after user zoom), and that very thin fonts are harder to read at low contrast. Understanding 1.4.3 adds that anti-aliasing can make thin fonts render fainter than their CSS colour, and recommends choosing a heavier font or exceeding the minimum in that case.

### 1.4.11 Non-text Contrast, Level AA

[SC 1.4.11](https://www.w3.org/TR/WCAG22/#non-text-contrast) requires at least 3:1 against adjacent colours for:

- **User interface components:** visual information required to identify components and their states (inactive components, and components whose appearance the user agent sets without author changes, are exempt).
- **Graphical objects:** parts of graphics required to understand the content, unless a particular presentation is essential.

Points from [Understanding 1.4.11](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html) that matter for diagrams and tokens:

- "Graphical object" includes "each line in a graph"; in its line-chart example the graph lines and the grey value grid lines both need 3:1 against their background, and in its pie-chart example each slice is a graphical object.
- Text on or beside a chart (labels and values) can carry the information instead, which reduces the number of graphical objects that must meet 3:1. A long description is also an option. This is how a period diagram can use soft fills for bands as long as each band is also labelled with text that meets 4.5:1.
- Hover and visited colour changes do not need 3:1 against the default state, but a component must not lose contrast with its adjacent colours in any state.
- The focus indicator must have 3:1 against whatever it is adjacent to: outside the component it must contrast with the page background; inside it must contrast with the component's own fill. The worked "Fail" example is a yellow (`#FFFF00`) outline on a white page.
- Gradients: take the central colour of the area required for understanding.

### 1.4.1 Use of Colour, Level A

[SC 1.4.1](https://www.w3.org/TR/WCAG22/#use-of-color): colour must not be "the only visual means of conveying information", indicating an action, prompting a response or distinguishing a visual element. Consequences for fiscalclock:

- **Diagrams:** a 53rd week, an exception period or a "current" period must be marked by a label, pattern, outline or position as well as colour.
- **Links in prose:** underline them, or meet [G183](https://www.w3.org/WAI/WCAG22/Techniques/general/G183): 3:1 between link and body text plus another visual cue on focus and hover.
- **Status (pass, fail, warning) in docs callouts:** pair colour with an icon and a word.

## 2. Formulas and the contrast script

### Relative luminance and contrast ratio, as WCAG 2.2 defines them

From the [WCAG 2.2 definition of relative luminance](https://www.w3.org/TR/WCAG22/#dfn-relative-luminance), for sRGB:

```text
RsRGB = R8bit / 255   (likewise G, B)
R = RsRGB / 12.92                      if RsRGB <= 0.04045
R = ((RsRGB + 0.055) / 1.055) ^ 2.4    otherwise          (likewise G, B)
L = 0.2126 * R + 0.7152 * G + 0.0722 * B
```

From the [WCAG 2.2 definition of contrast ratio](https://www.w3.org/TR/WCAG22/#dfn-contrast-ratio):

```text
contrast = (L1 + 0.05) / (L2 + 0.05)    L1 = lighter, L2 = darker; range 1 to 21
```

Notes from the same definitions that affect how we measure:

- Text contrast is measured against the specified background; it is a failure to set a text colour without a background colour, or the reverse ([WCAG 2.2, contrast ratio, notes 3 and 4](https://www.w3.org/TR/WCAG22/#dfn-contrast-ratio)). Every token pair must therefore specify both.
- Contrast is evaluated with anti-aliasing turned off, from the CSS colours, not the rendered pixels ([Understanding 1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html)).

### The linearisation threshold: 0.04045, not 0.03928

- The current WCAG 2.2 text uses **0.04045** for all three channels ([definition of relative luminance](https://www.w3.org/TR/WCAG22/#dfn-relative-luminance)). Its Note 2 says the value was 0.03928 before May 2021, was taken from an older specification, and that the change "has no practical effect on the calculations" in WCAG's context.
- The [WCAG 2.1 errata](https://www.w3.org/WAI/WCAG21/errata/) record the correction as an editorial erratum dated 2022-02-22 ("updating the red threshold from 0.03928 to 0.04045", issue #1780).
- The [WCAG 2.2 errata](https://www.w3.org/WAI/WCAG22/errata/) (last modified 1 Oct 2026, as of 1 Nov 2026) list no entry about luminance or contrast.
- Reproducible check that the change has no effect on 8-bit colours: no integer `v` from 0 to 255 has `v/255` between 0.03928 and 0.04045 (10/255 ≈ 0.03922 and 11/255 ≈ 0.04314 fall on the same side of both). Run: `python -I -c "print([v for v in range(256) if (v/255 <= 0.03928) != (v/255 <= 0.04045)])"`, which prints `[]`.

### Script: `design/scripts/contrast.py`

Standard library only, type-hinted, with a function API (`relative_luminance`, `contrast_ratio`, `format_ratio`, `parse_hex`) and a CLI. It accepts `#rgb` or `#rrggbb`.

**Rounding rule:** pass and fail use the unrounded ratio (as Understanding 1.4.3 requires). The printed ratio is truncated, never rounded up, to two decimal places, and the unrounded value is printed beside it. So `#777777` on white prints `4.47:1 (unrounded 4.478089)`, and a ratio of 4.4999 can never be shown as 4.50.

```text
$ python -I design/scripts/contrast.py "#111111" "#ffffff"
#111111 on #ffffff: 18.88:1 (unrounded 18.883061)
  AA text (1.4.3)          >= 4.5:1  pass
  AA large text (1.4.3)    >= 3:1  pass
  AA non-text (1.4.11)     >= 3:1  pass
  AAA text (1.4.6)         >= 7:1  pass
  AAA large text (1.4.6)   >= 4.5:1  pass
```

### Verification against published reference values

Reference values come from the JSON API of [WebAIM's contrast checker](https://webaim.org/resources/contrastchecker/); the page documents that appending `&api` to a permalink returns the ratio as JSON. Retrieved 1 Nov 2026, for example [`#777777` on white](https://webaim.org/resources/contrastchecker/?fcolor=777777&bcolor=FFFFFF&api). WebAIM does not document its rounding; every value below is consistent with truncation (5.6994 is reported as 5.69), so the self-test requires `reference <= computed < reference + one unit in the last digit`.

| Pair | WebAIM | `contrast.py` (unrounded) | Also published by W3C |
| --- | --- | --- | --- |
| `#777777` on `#FFFFFF` | 4.47 | 4.4781 | |
| `#767676` on `#FFFFFF` | 4.54 | 4.5422 | Used as a passing input border in [Understanding 1.4.11](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html) |
| `#949494` on `#FFFFFF` | 3.03 | 3.0335 | "contrast ratio of 3:1" in [Understanding 1.4.11](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html) |
| `#000000` on `#FFFFFF` | 21 | 21.0000 | Maximum of the range in the [contrast ratio definition](https://www.w3.org/TR/WCAG22/#dfn-contrast-ratio) |
| `#E5E5E5` on `#6221EA` | 5.69 | 5.6994 | "5.6:1" in [Understanding 1.4.11](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html) |
| `#FFF000` on `#FFFFFF` | 1.18 | 1.1854 | "1.2:1" in [Understanding 1.4.11](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html) |
| `#0066CC` on `#FFFFFF` | 5.56 | 5.5666 | |
| `#121212` on `#FFFFFF` | 18.7 | 18.7337 | |
| `#FFFFFF` on `#FFFFFF` | 1 | 1.0000 | Minimum of the range |

Command and result:

```text
$ python -I design/scripts/contrast.py --self-test
ok   #777777 on #ffffff: computed 4.4781, reference 4.47
ok   #767676 on #ffffff: computed 4.5422, reference 4.54
ok   #949494 on #ffffff: computed 3.0335, reference 3.03
ok   #000000 on #ffffff: computed 21.0000, reference 21
ok   #e5e5e5 on #6221ea: computed 5.6994, reference 5.69
ok   #fff000 on #ffffff: computed 1.1854, reference 1.18
ok   #0066cc on #ffffff: computed 5.5666, reference 5.56
ok   #121212 on #ffffff: computed 18.7337, reference 18.7
ok   #ffffff on #ffffff: computed 1.0000, reference 1
9/9 reference values matched
```

Note the W3C prose rounds loosely (`#949494` is called 3:1 although it is 3.03:1). For our own tokens, use the script, not the prose figures.

## 3. Focus visibility

| SC | Level | Requirement (paraphrased from [WCAG 2.2](https://www.w3.org/TR/WCAG22/)) |
| --- | --- | --- |
| [2.4.7 Focus Visible](https://www.w3.org/TR/WCAG22/#focus-visible) | AA | Every keyboard-operable interface has a mode in which the focus indicator is visible. [Understanding 2.4.7](https://www.w3.org/WAI/WCAG22/Understanding/focus-visible.html) adds that the indicator must not be time-limited. |
| [2.4.11 Focus Not Obscured (Minimum)](https://www.w3.org/TR/WCAG22/#focus-not-obscured-minimum) | AA, new in 2.2 | A focused component is not **entirely** hidden by author-created content. Sticky headers, cookie bars and floating "copy" buttons are the usual causes. |
| [2.4.12 Focus Not Obscured (Enhanced)](https://www.w3.org/TR/WCAG22/#focus-not-obscured-enhanced) | AAA | No **part** of the focused component is hidden. |
| [2.4.13 Focus Appearance](https://www.w3.org/TR/WCAG22/#focus-appearance) | AAA, new in 2.2 | The indicator's area is at least that of a 2 CSS px thick perimeter of the unfocused component, and has at least 3:1 contrast between the same pixels focused and unfocused. Exempt if the user agent draws it and the author has not changed it or its background. |
| [1.4.11 Non-text Contrast](https://www.w3.org/TR/WCAG22/#non-text-contrast) | AA | The focus indicator needs 3:1 against adjacent colours. |

What a good indicator needs, in both themes:

- **A solid outline at least 2px thick.** [Understanding 2.4.13](https://www.w3.org/WAI/WCAG22/Understanding/focus-appearance.html) describes a solid outline at least 2px thick as the "easiest and most common way" to meet it; outlines inset further inside the component need to be thicker than 2px. Use CSS `outline` and `outline-offset` rather than `box-shadow`, because forced colours mode computes `box-shadow` to `none` ([CSS Color Adjustment 1, §3.1](https://www.w3.org/TR/css-color-adjust-1/)), while `outline-color` is forced to a system colour and stays visible.
- **3:1 against every background it can sit on, in each theme.** One colour per theme is enough if the theme's surfaces are few; otherwise use a two-colour indicator (an inner and outer ring, one light and one dark), the approach of technique [C40](https://www.w3.org/WAI/WCAG22/Techniques/css/C40), which lists it as sufficient for 1.4.11, 2.4.7 and 2.4.13.
- **Show it for keyboard users with `:focus-visible`** (technique [C45](https://www.w3.org/WAI/WCAG22/Techniques/css/C45)); never remove the outline without a replacement ([G195](https://www.w3.org/WAI/WCAG22/Techniques/general/G195)).
- **Leave room under sticky headers** (for example `scroll-padding-top` equal to the header height) so 2.4.11 holds. This is a design suggestion, not a W3C technique.

## 4. Target size

[SC 2.5.8 Target Size (Minimum)](https://www.w3.org/TR/WCAG22/#target-size-minimum), Level AA, new in 2.2: pointer targets are at least 24 × 24 CSS px, except when:

- **Spacing:** an undersized target is positioned so that a 24 CSS px diameter circle centred on its bounding box does not intersect another target or another undersized target's circle.
- **Equivalent:** another control on the same page that meets the criterion does the same thing.
- **Inline:** the target is in a sentence or constrained by the line-height of surrounding text (links in prose are exempt).
- **User agent control** or **Essential**.

[SC 2.5.5 Target Size (Enhanced)](https://www.w3.org/TR/WCAG22/#target-size-enhanced), Level AAA, is 44 × 44 CSS px. Docs-site implications: theme toggle, copy-code buttons, language tabs, sidebar disclosure arrows and "on this page" links need 24px or the spacing exception. Interactive diagram elements, if any, count as targets.

## 5. Other WCAG 2.2 criteria for a docs site and diagrams

| SC | What it requires | What it means for us |
| --- | --- | --- |
| [1.1.1 Non-text Content](https://www.w3.org/TR/WCAG22/#non-text-content) (A) | A text alternative for non-text content. For charts and diagrams, [Understanding 1.1.1](https://www.w3.org/WAI/WCAG22/Understanding/non-text-content.html) (Situation B) calls for a short text alternative **and** a long one. | Inline SVG diagrams: `role="img"` ([WAI-ARIA 1.2 `img` role](https://www.w3.org/TR/wai-aria-1.2/): a container of elements that "form an image") plus a first-child `<title>` (accessible name) and `<desc>` (description); [SVG-AAM](https://www.w3.org/TR/svg-aam-1.0/) gives the priority as `aria-labelledby`, `aria-label`, then a direct child `title`, and `aria-describedby` then a direct child `desc` for the description. For period diagrams, also give the same data as a text table or list next to the figure; the [WAI complex images tutorial](https://www.w3.org/WAI/tutorials/images/complex/) describes this two-part approach. Decorative SVGs get `aria-hidden="true"`. |
| [1.3.1 Info and Relationships](https://www.w3.org/TR/WCAG22/#info-and-relationships) (A) | Structure shown visually is available programmatically. | Real `<table>` with `<th>` and `scope` for period tables ([H63](https://www.w3.org/WAI/WCAG22/Techniques/html/H63)); code in `<pre><code>`; language tabs follow the [ARIA Authoring Practices tabs pattern](https://www.w3.org/WAI/ARIA/apg/patterns/tabs/) (`tablist`, `tab`, `tabpanel`, arrow-key navigation). |
| [1.4.4 Resize Text](https://www.w3.org/TR/WCAG22/#resize-text) (AA) | Text resizes to 200% without loss of content or function. | Set type in `rem`; do not fix the heights of text containers. |
| [1.4.10 Reflow](https://www.w3.org/TR/WCAG22/#reflow) (AA) | No two-dimensional scrolling at 320 CSS px width (1280px at 400% zoom). Data tables and diagrams may scroll in two dimensions. | Wide period diagrams and tables may scroll horizontally inside their own container; page text must reflow. |
| [1.4.12 Text Spacing](https://www.w3.org/TR/WCAG22/#text-spacing) (AA) | No loss when users set line height 1.5, paragraph spacing 2×, letter spacing 0.12×, word spacing 0.16× the font size. | Avoid fixed-height badges, pills and tab labels; test with a text-spacing bookmarklet. |
| [1.4.13 Content on Hover or Focus](https://www.w3.org/TR/WCAG22/#content-on-hover-or-focus) (AA) | Hover or focus pop-ups are dismissible, hoverable and persistent. | Applies to diagram tooltips and glossary pop-overs. |
| [2.3.3 Animation from Interactions](https://www.w3.org/TR/WCAG22/#animation-from-interactions) (AAA) | Motion triggered by interaction can be disabled unless essential. [Understanding 2.3.3](https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html) names vestibular disorders and parallax. | Respect `prefers-reduced-motion` ([Media Queries 5, §12.1](https://www.w3.org/TR/mediaqueries-5/); technique [C39](https://www.w3.org/WAI/WCAG22/Techniques/css/C39)). Keep any animated logo or diagram transition static under `reduce`. |
| [3.1.1 Language of Page](https://www.w3.org/TR/WCAG22/#language-of-page) (A) | The default language is programmatically determinable. | `<html lang="en-GB">` (the docs are British English) via [H57](https://www.w3.org/WAI/WCAG22/Techniques/html/H57); in SVG files served standalone, set `lang` on the root. |

## 6. Dark mode

### Pure white on pure black: what the evidence supports

- **The halation claim is widely repeated but weakly evidenced.** [Level Access (30 June 2020)](https://www.levelaccess.com/blog/accessibility-for-people-with-astigmatism/) says white text on black produces a "visual fuzzing effect" called halation for people with astigmatism, and recommends avoiding white text on pure black. It is a vendor blog; it cites prevalence studies of astigmatism, not a study of reading performance by astigmatic readers. I found no controlled study that isolates astigmatism and polarity. **Treat halation as a plausible, unproven rationale.**
- **Stronger evidence concerns polarity in general.** [Nielsen Norman Group, Budiu (2 Feb 2020)](https://www.nngroup.com/articles/dark-mode/) reviews Piepenbrock et al. (2013), Dobres et al. (2017) and others and concludes that for people with normal vision "light mode leads to better performance most of the time", with the gap larger for small text; people with cloudy ocular media (for example cataracts) may read faster in dark mode (Legge et al., 1985). That supports offering both themes as equals, which DD-005 already does.
- **Industry guidance avoids the extremes.** The [Material Components for Android dark theme guide](https://github.com/material-components/material-components-android/blob/master/docs/theming/Dark.md) says its dark backgrounds are "dark grey instead of black", which it says reduces eye strain for light text. [web.dev, Steiner](https://web.dev/articles/prefers-color-scheme) recommends avoiding pure white text in dark mode "to prevent glowing and bleeding" and suggests `rgb(250, 250, 250)`. Both are design guidance, not research.

### Recommended lightness range (derived, reproducible)

No normative source gives a background lightness range. These bounds follow from the WCAG formula and `contrast.py`:

| Question | Answer (greys only) |
| --- | --- |
| Lightest grey background on which white text still reaches 7:1 | `#595959` (7.00:1) |
| Lightest grey background on which white text reaches 4.5:1 | `#767676` (4.54:1) |
| Darkest grey text that reaches 4.5:1 on `#121212` | `#7D7D7D` (4.55:1) |
| Darkest grey text that reaches 7:1 on `#121212` | `#9F9F9F` (7.07:1) |
| `#FAFAFA` on `#121212` | 17.94:1 |
| `#E6E6E6` on `#121212` | 15.01:1 |

Practical rule for the palette track: keep dark backgrounds in the near-black range (for example around `#0F`–`#1A` per channel, which is a proposal, not a sourced value), keep body text off-white (about 15:1 to 18:1), and check that secondary text and borders still meet 4.5:1 and 3:1 respectively. A softer pair is allowed as long as body text stays at or above 7:1, our AAA target.

### Saturated colours on dark backgrounds

- Saturated blues and purples have low relative luminance, so they lose contrast on dark backgrounds. Checked with the script: pure blue `#0000FF` on `#121212` is 2.18:1 (fails even 3:1); `#FF0000` is 4.68:1; `#00FF00` is 13.65:1. A brand hue therefore needs a separate, lighter variant in the dark theme; reusing the light-theme value is not safe.
- [web.dev](https://web.dev/articles/prefers-color-scheme) recommends "slightly less vibrant and brilliant images" in dark mode and shows a CSS filter for it. The Material Android guide says its dark themes provide adjusted defaults for the brand palette. Neither gives a measurable rule; the measurable rule is the contrast table above.

### `prefers-color-scheme`, `color-scheme` and forced colours

- **`prefers-color-scheme`** ([Media Queries 5, §12.5](https://www.w3.org/TR/mediaqueries-5/), Working Draft 19 Feb 2026 as of 1 Nov 2026) reports the user's light or dark preference. Use it for the default theme and allow a manual override.
- **`color-scheme`** ([CSS Color Adjustment 1, §2.1](https://www.w3.org/TR/css-color-adjust-1/), Candidate Recommendation Snapshot 16 Dec 2025 as of 1 Nov 2026) declares which schemes a page supports, so the browser's own form controls, scrollbars and canvas match. Set `color-scheme: light dark` at the root and `light` or `dark` when the user forces one theme.
- **Forced colours (Windows High Contrast and similar):** `@media (forced-colors: active)` matches when the user agent enforces a user-chosen limited palette ([Media Queries 5, §12.4](https://www.w3.org/TR/mediaqueries-5/)). Under it ([CSS Color Adjustment 1, §3.1](https://www.w3.org/TR/css-color-adjust-1/)):
  - colours including `fill`, `stroke`, `outline-color`, `border-color` and `background-color` are forced to system colours;
  - `box-shadow` and `text-shadow` compute to `none`; `background-image` computes to `none` unless it uses `url()`;
  - `background-color` keeps its original alpha, so transparent backgrounds stay transparent; for other properties the user agent picks the forced system colour;
  - user agents are expected to apply `svg|svg { forced-color-adjust: preserve-parent-color; }`, which keeps an SVG's exact colours except that an inherited `color` (and so `currentColor`) follows the forced text colour. The spec's note explains that forced colours "can't generally be usefully applied to illustrations."

  Consequences: do not draw focus rings, borders or diagram separators with shadows; give cards and buttons a real border (commonly a transparent one, which is then forced to a visible system colour; confirm in a browser); use `currentColor` for icon strokes and diagram text so they follow the forced palette; and, because inline SVG diagrams keep their own fills against a user-chosen background, give each diagram its own background and outlines, or restyle it under `@media (forced-colors: active)` with system colours such as `CanvasText` and `Canvas`. The spec says `forced-color-adjust: none` is for authors who adjust colours themselves to meet the user's needs. Test with forced-colours emulation in Chromium developer tools (not verified here).

## 7. APCA and WCAG 3: status as of 1 Nov 2026

- **WCAG 3 is a Working Draft, not a Recommendation.** The [latest published WCAG 3.0](https://www.w3.org/TR/wcag-3.0/) is the Working Draft of 10 September 2026; its status section says it is "inappropriate to cite this document as other than a work in progress." Its text-contrast requirements read "contrast measure to be determined" and an editor's note says "The contrast algorithm used in WCAG 3 is yet to be determined." The draft does not name APCA.
- **WCAG 3 will not replace WCAG 2 soon.** The [WAI WCAG 3 introduction](https://www.w3.org/WAI/standards-guidelines/wcag/wcag3-intro/) (updated 25 Sep 2026) says WCAG 3 is "not expected to be a completed W3C standard for a few more years" and "will not supersede WCAG 2", and recommends meeting WCAG 2.2 now.
- **APCA is a separate, non-normative project.** The [APCA repository](https://github.com/Myndex/apca-w3) describes its readability criterion as "a work in progress", labels the library beta, and says standards that will incorporate APCA "are still developmental"; it also states licence limits for the W3C-licensed version.

**Recommendation:** WCAG 2.2 ratios are the only pass/fail rule. APCA Lc values may appear in a palette report as a secondary column headed "APCA (non-normative, informational)", never as a reason to accept a pair that fails WCAG 2.2. `contrast.py` does not compute APCA.

## 8. Colour-vision deficiency

### How to test

- **Simulation model:** [Machado, Oliveira and Fernandes, "A Physiologically-based Model for Simulation of Color Vision Deficiency", IEEE TVCG 15(6), 2009](https://www.inf.ufrgs.br/~oliveira/pubs_files/CVD_Simulation/CVD_Simulation.html) publishes precomputed 3 × 3 matrices for protanomaly, deuteranomaly and tritanomaly at severities 0.0 to 1.0 in steps of 0.1 (1.0 = dichromacy). The project page does not say whether the matrices apply to linear or gamma-encoded RGB. **Unverified: check the paper before implementing a simulator**; the common practice of applying them in linear RGB is not confirmed here.
- **Procedure for the palette track:** simulate each categorical colour at severity 1.0 for all three types, then check that every pair of adjacent diagram colours is still distinguishable. Because 1.4.1 forbids colour as the only cue, the simulator is a quality check, not the compliance mechanism: labels and patterns are.
- Browser developer tools in Chromium and Firefox offer vision-deficiency emulation; I did not verify their algorithms, so treat them as a convenience.

### Safe categorical palettes (sources and how they fare in our two themes)

**Okabe–Ito.** [Okabe and Ito, "Color Universal Design" (2002, modified 2008)](https://jfly.uni-koeln.de/color/), Fig. 16 "Colorblind barrier-free color pallet", gives RGB 0–255 values: black (0,0,0), orange (230,159,0), sky blue (86,180,233), bluish green (0,158,115), yellow (240,228,66), blue (0,114,178), vermilion (213,94,0), reddish purple (204,121,167). The values are printed in the figure image ([`image/pallete.jpg`](https://jfly.uni-koeln.de/color/image/pallete.jpg)), which I read directly. Caution: the page's prose separately suggests vermilion as `#FF2000`, which does not match the figure's (213,94,0); use the figure.

**Paul Tol.** [Paul Tol's colour schemes](https://sronpersonalpages.nl/~pault/) (qualitative section dated 16 Aug 2026; page last updated 29 Oct 2026, as of 1 Nov 2026; the old `personal.sron.nl` address did not respond on 1 Nov 2026) lists colour-blind-safe qualitative schemes, among them *bright* (`#4477AA`, `#EE6677`, `#228833`, `#CCBB44`, `#66CCEE`, `#AA3377`, `#BBBBBB`), *high-contrast* (`#004488`, `#DDAA33`, `#BB5566`), *vibrant*, *muted*, *medium-contrast* and *dark*. The page says the *dark* scheme has at least 4.5:1 against white; checked with `contrast.py`, all 11 colours do (lowest `#228833` at 4.52:1).

**Neither palette meets 3:1 against both themes.** Graphical objects need 3:1 against adjacent colours ([SC 1.4.11](https://www.w3.org/TR/WCAG22/#non-text-contrast)). Computed with `contrast.py`:

| Colour | On `#FFFFFF` | On `#121212` |
| --- | --- | --- |
| Okabe–Ito orange `#E69F00` | 2.25:1 fail | 8.31:1 |
| Okabe–Ito sky blue `#56B4E9` | 2.30:1 fail | 8.11:1 |
| Okabe–Ito bluish green `#009E73` | 3.42:1 | 5.47:1 |
| Okabe–Ito yellow `#F0E442` | 1.32:1 fail | 14.16:1 |
| Okabe–Ito blue `#0072B2` | 5.18:1 | 3.61:1 |
| Okabe–Ito vermilion `#D55E00` | 3.86:1 | 4.84:1 |
| Okabe–Ito reddish purple `#CC79A7` | 3.06:1 | 6.12:1 |
| Okabe–Ito black `#000000` | 21.00:1 | 1.12:1 fail |
| Tol bright blue `#4477AA` | 4.70:1 | 3.98:1 |
| Tol bright red `#EE6677` | 3.08:1 | 6.06:1 |
| Tol bright green `#228833` | 4.52:1 | 4.13:1 |
| Tol bright yellow `#CCBB44` | 1.95:1 fail | 9.59:1 |
| Tol bright cyan `#66CCEE` | 1.83:1 fail | 10.20:1 |
| Tol bright purple `#AA3377` | 6.08:1 | 3.07:1 |
| Tol bright grey `#BBBBBB` | 1.91:1 fail | 9.75:1 |

`#121212` here is only a test background, not a palette decision. Consequences: diagram colours need a per-theme variant (or an outline that meets 3:1 around light fills), and colours used as large filled areas must also be checked against each other where they touch, not only against the page. Text on coloured fills needs 4.5:1 against that fill.

## 9. Checking a built site

| Tool | What it does | CI | Limits |
| --- | --- | --- | --- |
| [axe-core](https://github.com/dequelabs/axe-core) | Rule engine for WCAG 2.0, 2.1 and 2.2 at A, AA and AAA. Its README says it finds "on average 57% of WCAG issues automatically" and returns "incomplete" results that need manual review. CLI and integrations live in [axe-core-npm](https://github.com/dequelabs/axe-core-npm) (`@axe-core/cli`, Playwright, Puppeteer). | Yes | The README notes the colour-contrast rule does not work under JSDOM; run it in a real browser. |
| [Pa11y](https://github.com/pa11y/pa11y) and [Pa11y CI](https://github.com/pa11y/pa11y-ci) | Pa11y runs HTML_CodeSniffer (default) or axe; standard defaults to `WCAG2AA`. Pa11y CI tests a URL list or a sitemap, exits 2 when the issue count reaches the threshold (default 0). | Yes | The README does not mention WCAG 2.2; use the `axe` runner for 2.2 rules (unverified whether HTML_CodeSniffer covers 2.2). |
| [Lighthouse](https://developer.chrome.com/docs/lighthouse/accessibility/scoring) | Accessibility score is a weighted average of pass/fail audits; its audits link to axe-core rule documentation. Manual checks are listed but do not affect the score. [Lighthouse CI](https://github.com/GoogleChrome/lighthouse-ci) runs it in CI. | Yes | A score of 100 does not mean conformance. |
| Manual keyboard pass | Tab through every page type in both themes: visible focus on every stop, logical order, nothing hidden under sticky bars (2.4.11), tabs operable with arrow keys, skip link works. | No | Needed for 2.4.7, 2.4.11, 2.4.13 and 1.4.13, which tools only partly detect. |
| Manual zoom and spacing pass | 200% text zoom (1.4.4), 320px width (1.4.10), text-spacing override (1.4.12), forced colours emulation, `prefers-reduced-motion` emulation. | Partly (screenshots) | |
| Token contrast check | `python -I design/scripts/contrast.py --self-test`, then every token pair in both themes. | Yes | Checks the palette, not the rendered page. |

Recommended CI order once a site exists: token contrast check (fast, no browser), then axe-core over every built page in both themes (set the theme explicitly, for example with `prefers-color-scheme` emulation in Playwright), then Pa11y CI with the `axe` runner on the sitemap if a second opinion is wanted. Run the manual passes before each release.

## Open questions and unverified items

- Halation for readers with astigmatism: no controlled study found; treated as a plausible rationale only.
- Machado et al. matrices: whether they expect linear or gamma-encoded RGB is not stated on the project page; read the paper before implementing.
- WebAIM's rounding: inferred as truncation from the values returned, not documented.
- HTML_CodeSniffer's coverage of WCAG 2.2: not stated in the Pa11y README.
- A dark background range near `#0F0F0F` to `#1A1A1A` is a proposal for the palette track, not a sourced figure.
