# Typography research

Research notes for fiscalclock's visual identity, typography track, 1 November 2026. These notes propose; they decide nothing. Decisions go in [`DECISIONS.md`](../DECISIONS.md).

## Summary

- Every candidate below is under the SIL Open Font Licence 1.1 (OFL), as the brief requires ([DD-010](../DECISIONS.md)). `Instrument Sans` and `JetBrains Mono` were left out on purpose ([DD-001](../DECISIONS.md)).
- Thirty-one families (one file each, plus IBM's new variable `IBM Plex Mono`) were inspected with `fontTools`. A specimen was rendered for each, and the shortlisted ones were looked at closely. Eight finalists and eight reserves came out of it.
- What matters most for fiscalclock is how a face handles figures, and the faces differ a lot:
  - `IBM Plex Sans`, `Source Sans 3`, `Source Serif 4` and `Newsreader` have **tabular figures by default**.
  - `Inter` and `Geist` have proportional figures by default and need `tnum`.
  - `slashed-zero` does nothing in `Geist`, `Geist Mono`, `Commit Mono` or `Newsreader`, because they have no `zero` feature. In `Atkinson Hyperlegible Mono` it turns the default slashed zero into a **dotted** one.
- Three pairings are proposed:
  - **A, neutral:** `Inter` with `Commit Mono`.
  - **B, technical and slightly warmer:** `IBM Plex Sans` with `IBM Plex Mono`, plus `IBM Plex Serif` if wanted.
  - **C, ledger and classic:** `Source Serif 4` headings, `Source Sans 3` for UI and body, and `Source Code Pro`.
  - Pairing B is the lead suggestion. In both its UI face and its code face, `I`, `l` and `1` are clearly told apart with no feature switched on, and table figures line up with no CSS. Pairing C shares the figure behaviour, but B has the larger x-height and smaller files.

## Constraints taken from the brief

| Constraint | Source |
| --- | --- |
| OFL fonts only | [DD-010](../DECISIONS.md) |
| Dark mode equal to light | [DD-005](../DECISIONS.md) |
| Balanced personality, leaning very slightly serious, technical, minimal, classic and quiet | [DD-006](../DECISIONS.md) |
| Independent of the sibling project, so no `Instrument Sans` or `JetBrains Mono` | [DD-001](../DECISIONS.md) and the brief for this track |
| Numbers (dates, week numbers, quarters, period bounds) must be excellent | Brief for this track |

## Method

All font files were downloaded on 1 November 2026. Nothing was taken from memory. Each fact in the tables comes from one of these three sources:

- the font file, through the scripts in [Reproduce](#reproduce);
- the upstream repository page that is linked;
- a rendered specimen that I looked at.

- **Sources of files.** Most files came from the `main` branch of [google/fonts](https://github.com/google/fonts/tree/main/ofl). `Commit Mono` came from its [v1.143 release](https://github.com/eigilnikolajsen/commit-mono/releases/tag/v1.143). The variable `IBM Plex Mono` came from the [`@ibm/plex-mono-variable@1.0.0` release](https://github.com/IBM/plex/releases/tag/%40ibm/plex-mono-variable%401.0.0).
- **Tools.** Python 3.13.11, `fontTools` 4.68.0, `brotli`, and `Pillow` 13.0.0, all in a scratch virtual environment. This `Pillow` build has no `libraqm`, so it cannot apply OpenType features itself. Instead, the render script "freezes" a feature: it applies the feature's single and alternate substitutions to the character map, chaining them in lookup order, before drawing. The idea is the same as `pyftfeatfreeze`. Contextual substitutions are not applied (see [Not verified](#not-verified-and-open-questions)).
- **Tabular by default.** All ten digit advance widths in `hmtx` are equal at the default instance. For variable fonts they must also be equal in a `wght` 700 instance.
- **Feature present.** The tag appears in the `GSUB` `FeatureList`. "Affects" lists the watched characters (`0O1lI|7S5` and `aglrtfy`) that the feature substitutes.
- **x-height.** `sxHeight` and `sCapHeight` come from the [OS/2 table](https://learn.microsoft.com/en-us/typography/opentype/spec/os2). Two ratios are reported: x/cap, and x/em (`sxHeight` ÷ `unitsPerEm`). A higher x/em looks larger at the same CSS `font-size`.
- **Coverage.** This is the count of `cmap` code points in each Unicode block. Latin Extended-A has 128 code points, so 128 means complete. The Greek (U+0370–U+03FF) and Cyrillic (U+0400–U+04FF) counts show only whether a script is there in substance.
- **WOFF2 size.** The font is subset with `fontTools.subset` to roughly the Google Fonts "latin" range, keeping every layout feature and every variation axis. The result is a ceiling on size, not what a site would ship.
- **Telling characters apart.** Judged by eye from renderings at 56 and 110 px on white, plus one 30 px line on a near-black strip. A rough pixel metric was also computed; it is described in [Telling characters apart](#telling-characters-apart).

## The field: all 31 files

"Digits" means the default figure spacing. "Zero" says how a slashed zero is reached. The Latin Extended-A count is out of 128. WOFF2 is the size of the Latin subset in kB, with all axes kept.

| Family | Version in file | Axes | Digits | `tnum` | Zero | x/cap | x/em | Ext-A | Greek | Cyrillic | WOFF2 | Outcome |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Inter` | 4.001 | `opsz` 14–32, `wght` 100–900 | proportional | yes | `zero` | 0.750 | 0.546 | 127 | 105 | 248 | 99.8 | finalist |
| `Inter Tight` | 3.004 | `wght` 100–900 | proportional | yes | `zero` | 0.750 | 0.546 | 127 | 121 | 254 | 53.9 | dropped: `Inter`'s `opsz` axis already covers display sizes |
| `IBM Plex Sans` | 3.201 | `wght` 100–700, `wdth` 75–100 | tabular | no (no `pnum` either) | `zero` | 0.739 | 0.516 | 128 | 73 | 192 | 69.1 | finalist |
| `IBM Plex Mono` | 2.3 (static) | none | mono | n/a | `zero` | 0.739 | 0.516 | 128 | 1 | 168 | 15.3 (one weight) | finalist |
| `IBM Plex Serif` | 2.6 (static) | none | tabular | no | `zero` | 0.739 | 0.516 | 127 | 1 | 168 | 19.8 (one weight) | reserve (headings in pairing B) |
| `Source Sans 3` | 3.052 | `wght` 200–900 | tabular | no (`pnum` yes) | `zero` | 0.724 | 0.478 | 128 | 88 | 156 | 47.2 | finalist |
| `Source Serif 4` | 4.004 | `opsz` 8–60, `wght` 200–900 | tabular | yes | `zero` | 0.709 | 0.475 | 122 | 80 | 150 | 194.5 | finalist |
| `Source Code Pro` | 1.026 | `wght` 200–900 | mono | n/a | `zero` | 0.724 | 0.478 | 128 | 84 | 150 | 27.2 | finalist |
| `Geist` | 1.800 | `wght` 100–900 | proportional | yes | none (no `zero`; `ss09` zero is not slashed) | 0.746 | 0.530 | 117 | 6 | 134 | 32.0 | reserve |
| `Geist Mono` | 1.701 | `wght` 100–900 | mono | n/a | slashed by default; no `zero` | 0.746 | 0.530 | 115 | 5 | 134 | 33.5 | reserve |
| `Public Sans` | 2.001 | `wght` 100–900 | proportional | yes | none | 0.715 | 0.517 | 124 | 1 | 0 | 26.2 | dropped: no slashed zero, Latin only |
| `Atkinson Hyperlegible Next` | 2.001 | `wght` 200–800 | proportional | yes | slashed by default | 0.743 | 0.496 | 93 | 4 | 0 | 32.8 | reserve: gaps in Latin Extended-A |
| `Atkinson Hyperlegible Mono` | 2.001 | `wght` 200–800 | mono | n/a | slashed by default; `zero` gives dotted | 0.743 | 0.496 | 93 | 4 | 0 | 17.0 | reserve: same gaps |
| `Hanken Grotesk` | 3.013 | `wght` 100–900 | tabular | no | none | 0.707 | 0.493 | 127 | 10 | 0 | 36.1 | dropped: no slashed zero, no `I`/`l` alternates |
| `Schibsted Grotesk` | 1.100 | `wght` 400–900 | proportional | yes | `zero` | 0.750 | 0.527 | 127 | 8 | 0 | 48.5 | dropped: no weights below 400, no `I`/`l` alternates |
| `Onest` | 2.001 | `wght` 100–900 | proportional | yes | none | 0.745 | 0.527 | 128 | 0 | 162 | 36.3 | dropped: no slashed zero |
| `Figtree` | 2.002 | `wght` 300–900 | proportional | yes | `ss02` only | 0.714 | 0.500 | 109 | 0 | 0 | 20.1 | dropped: gaps in Extended-A, Latin only |
| `Mona Sans` | 2.000 | `wdth` 75–125, `wght` 200–900 | proportional | yes | none | 0.709 | 0.517 | 123 | 2 | 0 | 121.9 | dropped: no slashed zero, large file |
| `Hubot Sans` | 2.000 | `wdth` 75–125, `wght` 200–900 | proportional | yes | none | 0.706 | 0.515 | 123 | 2 | 0 | 118.0 | dropped: as `Mona Sans` |
| `Fira Sans` | 4.203 (static) | none | proportional | yes | `zero` | 0.765 | 0.527 | 128 | 121 | 240 | 31.4 (one weight) | reserve |
| `Fira Mono` | 3.206 (static) | none | mono | n/a | `zero` | 0.765 | 0.527 | 128 | 121 | 240 | 18.4 (one weight) | reserve |
| `Fira Code` | 5.002 | `wght` 300–700 | mono | n/a | `zero` | 0.764 | 0.525 | 127 | 121 | 240 | 38.7 | dropped: `calt` is on by default in browsers (see note) |
| `Recursive` | 1.085 | `MONO`, `CASL`, `wght` 300–1000, `slnt`, `CRSV` | tabular | no (`pnum` yes) | `zero` | 0.751 | 0.526 | 127 | 20 | 0 | 418.0 | dropped: largest file by far |
| `Red Hat Text` | 1.030 | `wght` 300–700 | proportional | yes | `zero` | 0.697 | 0.488 | 124 | 0 | 0 | 30.7 | dropped: Latin only |
| `Red Hat Mono` | 1.030 | `wght` 300–700 | mono | n/a | `zero` | 0.697 | 0.488 | 124 | 0 | 0 | 22.8 | dropped: Latin only |
| `Newsreader` | 1.003 | `opsz` 6–72, `wght` 200–800 | tabular | yes | none | 0.636 | 0.426 | 124 | 0 | 0 | 137.0 | finalist (headings only) |
| `Literata` | 3.103 | `opsz` 7–72, `wght` 200–900 | proportional | yes | `zero` | 0.724 | 0.507 | 127 | 87 | 118 | 159.9 | reserve |
| `Martian Mono` | 1.000 | `wght` 100–800, `wdth` 75–112.5 | mono | n/a | no `zero` | 0.750 | 0.600 | 110 | 0 | 84 | 38.7 | dropped: no `zero`, gaps in Extended-A |
| `Spline Sans Mono` | 1.004 | `wght` 300–700 | mono | n/a | `zero` | 0.750 | 0.545 | 115 | 0 | 0 | 44.3 | dropped: Latin only |
| `Victor Mono` | 1.561 | `wght` 100–700 | mono | n/a | `ss02`–`ss05` only | 0.772 | 0.618 | 117 | 77 | 220 | 24.6 | dropped: no `zero` |
| `Commit Mono` | 1.143 (static) | none (400 and 700 files) | mono | n/a | slashed by default; no `zero` | 0.771 | 0.540 | 128 | 72 | 0 | 20.8 (one weight) | finalist |

Note on `Fira Code`: its `GSUB` has `calt`, and the [OpenType registry](https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae) suggests that `calt` "should be active by default". I did not check what this family's `calt` substitutes. It was dropped only because a code face for JSON definitions should show exactly the characters typed, and that would need checking first.

## Licences and versions of the finalists

Every finalist's `name` table (nameID 13) states the OFL 1.1, and GitHub reports `OFL-1.1` for each upstream repository, as of 1 November 2026.

| Family | Licence file | File inspected | Upstream as of 1 Nov 2026 |
| --- | --- | --- | --- |
| `Inter` | [`LICENSE.txt`](https://github.com/rsms/inter/blob/master/LICENSE.txt) | 4.001 from [google/fonts](https://github.com/google/fonts/tree/main/ofl/inter) | [v4.1](https://github.com/rsms/inter/releases/tag/v4.1), 16 Nov 2024 (not inspected) |
| `IBM Plex Sans`, `Mono`, `Serif` | [`LICENSE.txt`](https://github.com/IBM/plex/blob/master/LICENSE.txt) | Sans 3.201, Mono 2.3 and Serif 2.6 from google/fonts; variable Mono 1.000 from the IBM release | Mono variable [1.0.0](https://github.com/IBM/plex/releases/tag/%40ibm/plex-mono-variable%401.0.0) (30 Jul 2026), Mono static [2.5.0](https://github.com/IBM/plex/releases/tag/%40ibm/plex-mono%402.5.0) (11 Jun 2026, not inspected) |
| `Source Sans 3` | [`LICENSE.md`](https://github.com/adobe-fonts/source-sans/blob/release/LICENSE.md) | 3.052 | 3.052R, 4 Apr 2023 (GitHub API) |
| `Source Serif 4` | [`LICENSE.md`](https://github.com/adobe-fonts/source-serif/blob/release/LICENSE.md) | 4.004 | 4.005R, 20 Jan 2023 (GitHub API, not inspected) |
| `Source Code Pro` | [`LICENSE.md`](https://github.com/adobe-fonts/source-code-pro/blob/release/LICENSE.md) | variable 1.026 | 1.026R-vf, 12 Apr 2023 (GitHub API) |
| `Geist`, `Geist Mono` | [`LICENSE.txt`](https://github.com/vercel/geist-font/blob/main/LICENSE.txt) | 1.800 and 1.701 | [v1.9.0](https://github.com/vercel/geist-font/releases/tag/v1.9.0), 30 Oct 2026 (not inspected) |
| `Commit Mono` | [`LICENSE-FONT`](https://github.com/eigilnikolajsen/commit-mono/blob/main/LICENSE-FONT) (OFL 1.1) | 1.143 | [v1.143](https://github.com/eigilnikolajsen/commit-mono/releases/tag/v1.143), 28 Dec 2023 |
| `Newsreader` | [`OFL.txt`](https://github.com/productiontype/Newsreader/blob/master/OFL.txt) | 1.003 | no GitHub releases |
| `Atkinson Hyperlegible Next` and `Mono` | [`OFL.txt`](https://github.com/googlefonts/atkinson-hyperlegible-next-mono/blob/main/OFL.txt) (Mono repository) | 2.001 | not checked |

**Commit Mono's licence, checked because another track was unsure.** The repository holds two licence files:

- [`LICENSE`](https://github.com/eigilnikolajsen/commit-mono/blob/main/LICENSE) is the MIT licence. That is what GitHub's licence detection reports for the repository.
- [`LICENSE-FONT`](https://github.com/eigilnikolajsen/commit-mono/blob/main/LICENSE-FONT) is the SIL Open Font License 1.1.

The [README](https://github.com/eigilnikolajsen/commit-mono/blob/main/README.md) settles which file covers what. Under its "Licenses" heading it says the font files are under the OFL and the website code is under MIT. The v1.143 release zip agrees: it ships `license.txt` with the OFL 1.1 text, and the font's nameID 13 states the OFL. **The fonts are OFL 1.1**, which meets DD-010.

## Finalists in detail

The figures below come from the inspection output in [Reproduce](#reproduce). The visual notes come from the specimens.

### `Inter` (UI and body; `opsz` 32 is the display cut)

- **Figures.**
  - Proportional by default. At 2048 units per em, `0` is 1292 units wide and `1` is 833.
  - `tnum` makes every digit 1328 units wide.
  - `zero` gives a slashed zero, and chains correctly after `tnum` (`zero.tf` becomes `zero.tf.slash`).
  - `case` and `frac` are present; `lnum` and `onum` are not. The default figures are lining.
- **Alternates.** `cv01` gives an alternative `1`, `cv05` an `l` with a tail and `cv08` an `I` with serifs. `ss02` groups the slashed zero, the tailed `l` and the `I` with serifs.
- **By eye.** With default glyphs, `I` and `l` are both plain bars. This was the lowest score of any finalist on the rough metric (0.20). With `ss02` they are clearly different.
- **Coverage and size.** Coverage is the widest of the finalists: 2,849 code points, with Greek and Cyrillic. The Latin WOFF2 is 99.8 kB with both axes, or 63.1 kB with `opsz` pinned at 14.
- **Dates.** With `tnum`, `2026-01-31` looks loose around the `1`s. `FY2026 Q3` reads cleanly.

### `IBM Plex Sans` (UI and body)

- **Figures.**
  - Tabular by default: every digit is 600 units, and stays 600 at `wght` 700.
  - There is no `tnum` and no `pnum`, so figures are tabular everywhere, prose included.
  - `zero` gives a slashed zero. `lnum`, `onum` and `frac` are present; `case` is not.
- **By eye.** With default glyphs, `I` has serifs and `l` has a tail, so `Il1` is told apart with no features. This scored highest of the sans finalists on the metric (I/l 0.83).
- **Axes.** `wght` 100–700, and `wdth` 75–100 for a condensed width.
- **Coverage and size.** Latin Extended-A is complete, with Greek (73 code points) and Cyrillic (192). The Latin WOFF2 is 69.1 kB with both axes; I did not measure it with `wdth` pinned.
- **Dates.** `2026-01-31` is evenly spaced with no CSS. The zero is plain unless `zero` is on.

### `IBM Plex Mono` (code)

- **Figures and glyphs.** The default zero is dotted; `zero` (and `ss03`) give a slashed zero, and `ss04` gives a second alternative. `I` has serifs, `l` has a tail and `1` has a base.
- **Files.**
  - The google/fonts files are static, version 2.3.
  - IBM's July 2026 variable release (1.000) is `wght` 100–700 and adds `lnum`, `onum`, `ss08` (an alternative `l`) and wider Latin Extended Additional coverage (125 against 97).
  - The variable Latin WOFF2 subset is 32.1 kB, and the release's own split `Roman-Latin1.woff2` file is 32,576 bytes.
- **Fit with Plex Sans.** The x-height matches `IBM Plex Sans` exactly (x/em 0.516).

### `Source Sans 3` (UI and body)

- **Figures.** Tabular by default: 472 units at the 200 default instance and 528 at 700. `pnum` exists for prose. `zero` gives a slashed zero; `case`, `frac` and `onum` are present.
- **By eye.** With default glyphs, `l` has a small tail and `I` is a plain bar. `cv01` gives an `I` with serifs. Do not use `cv04`: it removes the tail from `l`.
- **Size and coverage.** x/em is 0.478, the smallest of the sans finalists, so it needs a larger `font-size` to match `Inter` or `IBM Plex Sans` optically. Greek and Cyrillic are included. The Latin WOFF2 is 47.2 kB.

### `Source Serif 4` (headings, ledger direction)

- **Figures.** Tabular by default. `tnum`, `pnum`, `lnum`, `onum`, `zero`, `case` and `frac` are all present.
- **Axes and size.** `opsz` runs 8–60, so headings can use a display cut. The Latin WOFF2 with both axes is 194.5 kB, the largest of the finalists.
- **By eye.** Its slashed zero has a long diagonal that runs outside the bowl, and looks like `Ø`. Leave `zero` off in this face.

### `Newsreader` (headings only, alternative to `Source Serif 4`)

- **Figures.** Tabular by default (1100 of 2000 units). There is no `zero` and no `frac`, and no Greek or Cyrillic.
- **Size.** x/em is 0.426, which suits headings rather than body text.
- **By eye.** It has the most "printed" feel of the serif candidates. With no slashed zero, `0` and `O` differ only in width.

### `Source Code Pro` (code)

- **Figures and glyphs.** Fixed-width. The default zero is dotted, and `zero` gives a slashed one. `I` has serifs, `l` has a tail and `1` has a base. `cv17` and `ss07` give an alternative `1`.
- **Size.** x/em is 0.478, so set it larger than a sans with a bigger x-height, or use [`size-adjust`](https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face/size-adjust). The Latin WOFF2 is 27.2 kB.

### `Commit Mono` (code)

- **Files.** Static only, in 400 and 700.
- **Glyphs.** The zero is slashed by default. There is no `zero` feature, and `cv07` swaps in a dotted zero. `I` has serifs, `l` has a tail and `1` has a base.
- **Features.** No `case` and no `frac`.
- **Coverage.** Latin Extended-A is complete and Greek has 72 code points, but there is no Cyrillic.
- **Fit with Inter.** x/em is 0.540, the closest of the mono faces to `Inter`'s 0.546.

### `Geist` and `Geist Mono` (reserve)

- **`Geist`.** It has `tnum` but no slashed zero at all: `ss09` swaps in a different `0` and `1`, and in the render that `0` still has no slash.
- **`Geist Mono`.** Its zero is slashed by default. It has no `zero` feature, and `ss09` *removes* the slash.
- **Why reserve.** As of 1 November 2026, the HTML of [vercel.com](https://vercel.com/) preloads several `Geist` font files (`curl` and `grep`; see [Reproduce](#reproduce)). For a project that admires Vercel without wanting to look like it, that association is the main risk.

### `Atkinson Hyperlegible Next` and `Mono` (reserve)

- **Glyphs.** Both have a slashed zero by default, and `I`, `l` and `1` are clearly different by default.
- **The catch.** The Mono's `zero` feature gives a *dotted* zero. A shared `slashed-zero` rule would therefore change its zero in the opposite direction to the other faces.
- **Coverage.** Latin Extended-A has only 93 of 128 code points, and there is no Cyrillic.

## Telling characters apart

**By eye.** I rendered `Il1| 0O 5S rnm 17 g` at 56 px and `0O Il1 7` at 110 px, once with default glyphs and once with features frozen. I then compared each pair visually. The table also says which features, if any, the face needs for each pair.

| Face | `I` `l` `1` `\|` | `0` `O` | `5` `S`, `rn` `m`, `1` `7` |
| --- | --- | --- | --- |
| `Inter` | `I` and `l` are near-identical bars by default; `ss02` (or `cv05` and `cv08`) fixes this | `0` is narrower than `O`; `zero` slashes it | all clear |
| `IBM Plex Sans` | clear by default (`I` with serifs, `l` with a tail) | width only; `zero` slashes | all clear |
| `Source Sans 3` | `l` tailed, `I` plain by default; `cv01` adds serifs to `I` | `zero` slashes | all clear |
| `Source Serif 4` | clear (serifs) | `zero` slash looks like `Ø`, so leave it off | all clear |
| `Newsreader` | clear (serifs) | width only; no `zero` | all clear |
| `IBM Plex Mono` | clear | dotted by default, slashed with `zero` | all clear |
| `Source Code Pro` | clear | dotted by default, slashed with `zero` | all clear |
| `Commit Mono` | clear | slashed by default | all clear |
| `Geist Mono` | clear | slashed by default | all clear |
| `Atkinson Hyperlegible Mono` | clear | slashed by default (dotted with `zero`) | all clear |

**Rough metric.** As a cross-check, each string of a pair is drawn at 96 px from the same origin, and the script records 1 − (intersection ÷ union) of the inked pixels. A score of 0 means the two shapes are identical, and higher means easier to tell apart. The metric under-rates fixed-width faces: two glyphs that share a cell overlap a lot even when a serif or tail makes them clear. In `IBM Plex Mono`, for example, `I`/`l` scores 0.26 although it reads clearly. Use it only to compare proportional faces. Output for the finalists:

```text
Atkinson Hyperlegible Mono   default : I/l 0.73  l/1 0.74  I/1 0.33  l/| 0.81  0/O 0.28  5/S 0.45  rn/m 0.91  1/7 0.81
                             zero    : I/l 0.73  l/1 0.74  I/1 0.33  l/| 0.81  0/O 0.22  5/S 0.45  rn/m 0.91  1/7 0.81
Atkinson Hyperlegible Next   default : I/l 0.78  l/1 0.92  I/1 0.73  l/| 0.40  0/O 0.53  5/S 0.44  rn/m 0.56  1/7 0.81
                             tnum    : I/l 0.78  l/1 0.91  I/1 0.83  l/| 0.40  0/O 0.56  5/S 0.53  rn/m 0.56  1/7 0.79
Geist                        default : I/l 0.28  l/1 0.91  I/1 0.94  l/| 0.39  0/O 0.46  5/S 0.50  rn/m 0.68  1/7 0.78
                             tnum+ss09+ss03+ss05: I/l 0.86  l/1 0.92  I/1 0.78  l/| 0.18  0/O 0.60  5/S 0.53  rn/m 0.68  1/7 0.79
Geist Mono                   default : I/l 0.31  l/1 0.41  I/1 0.47  l/| 0.59  0/O 0.31  5/S 0.50  rn/m 0.79  1/7 0.82
                             ss09+ss03: I/l 0.32  l/1 0.57  I/1 0.47  l/| 0.47  0/O 0.17  5/S 0.50  rn/m 0.79  1/7 0.82
IBM Plex Mono                default : I/l 0.26  l/1 0.45  I/1 0.43  l/| 0.52  0/O 0.10  5/S 0.38  rn/m 0.78  1/7 0.76
                             zero    : I/l 0.26  l/1 0.45  I/1 0.43  l/| 0.52  0/O 0.17  5/S 0.38  rn/m 0.78  1/7 0.76
IBM Plex Sans                default : I/l 0.83  l/1 0.89  I/1 0.78  l/| 0.64  0/O 0.60  5/S 0.40  rn/m 0.58  1/7 0.75
                             zero    : I/l 0.83  l/1 0.89  I/1 0.78  l/| 0.64  0/O 0.65  5/S 0.40  rn/m 0.58  1/7 0.75
Inter                        default : I/l 0.20  l/1 0.94  I/1 0.94  l/| 0.76  0/O 0.63  5/S 0.44  rn/m 0.62  1/7 0.79
                             tnum+zero+ss02: I/l 0.88  l/1 0.91  I/1 0.76  l/| 0.74  0/O 0.65  5/S 0.45  rn/m 0.62  1/7 0.77
Inter Tight                  default : I/l 0.22  l/1 0.94  I/1 0.94  l/| 0.82  0/O 0.63  5/S 0.45  rn/m 0.28  1/7 0.82
                             tnum+zero+ss02: I/l 0.89  l/1 0.98  I/1 0.85  l/| 0.79  0/O 0.67  5/S 0.49  rn/m 0.28  1/7 0.82
Newsreader                   default : I/l 0.62  l/1 0.97  I/1 0.94  l/| 0.51  0/O 0.83  5/S 0.65  rn/m 0.68  1/7 0.81
                             tnum    : I/l 0.62  l/1 0.97  I/1 0.94  l/| 0.51  0/O 0.83  5/S 0.65  rn/m 0.68  1/7 0.81
Source Code Pro              default : I/l 0.53  l/1 0.61  I/1 0.41  l/| 0.54  0/O 0.33  5/S 0.38  rn/m 0.92  1/7 0.72
                             zero    : I/l 0.53  l/1 0.61  I/1 0.41  l/| 0.54  0/O 0.38  5/S 0.38  rn/m 0.92  1/7 0.72
Source Sans 3                default : I/l 0.30  l/1 0.91  I/1 0.92  l/| 0.45  0/O 0.72  5/S 0.44  rn/m 0.64  1/7 0.71
                             zero+cv01+cv04: I/l 0.68  l/1 0.93  I/1 0.81  l/| 0.43  0/O 0.75  5/S 0.44  rn/m 0.64  1/7 0.71
Source Serif 4               default : I/l 0.58  l/1 0.90  I/1 0.81  l/| 0.49  0/O 0.79  5/S 0.44  rn/m 0.67  1/7 0.72
                             tnum+zero: I/l 0.58  l/1 0.90  I/1 0.81  l/| 0.49  0/O 0.81  5/S 0.44  rn/m 0.67  1/7 0.72
CommitMono                   default : I/l 0.59  l/1 0.67  I/1 0.36  l/| 0.68  0/O 0.36  5/S 0.40  rn/m 0.94  1/7 0.75
                             cv07    : I/l 0.59  l/1 0.67  I/1 0.36  l/| 0.68  0/O 0.33  5/S 0.40  rn/m 0.94  1/7 0.75
```

## Numbers in UI and tables

These are proposed rules. They should be checked in a browser once the stack is chosen ([DD-009](../DECISIONS.md)).

1. **Data.** Use `font-variant-numeric: tabular-nums lining-nums slashed-zero;` wherever numbers are compared or scanned: tables, period bounds, dates, week and period numbers, fiscal-year labels and conformance output. MDN maps these keywords to the OpenType `tnum`, `lnum` and `zero` features ([MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/font-variant-numeric)). The features themselves are defined in the OpenType registry ([`tnum`](https://learn.microsoft.com/en-us/typography/opentype/spec/features_pt), [`zero`](https://learn.microsoft.com/en-us/typography/opentype/spec/features_uz)).
2. **Prose.** In `Inter`, `Geist` and `Source Serif 4`, leave figures proportional in running text. `Source Sans 3` is tabular by default, so prose needs `proportional-nums`. `IBM Plex Sans` has no `pnum`, so its figures are tabular in prose too. That trade-off is acceptable for a date-heavy product.
3. **Do not trust `slashed-zero` blindly.**
   - It does nothing in `Geist`, `Geist Mono`, `Commit Mono`, `Newsreader` and `Public Sans`, which have no `zero` feature.
   - It turns `Atkinson Hyperlegible Mono`'s slashed zero into a dotted one.
   - In `Source Serif 4` its zero looks like `Ø`.
   - For each face that is chosen, write the zero rule down and test it.
4. **Lining figures only for data.** Every finalist's default figures are lining, as seen in the renders. `onum` exists in `IBM Plex Sans`, the `Source` family and the variable `IBM Plex Mono`, but old-style figures should never be used for data.
5. **ISO dates.**
   - A column of `YYYY-MM-DD` dates lines up whenever the digits are tabular, because every row has its hyphens in the same positions.
   - In `Inter`, `Source Serif 4` and `Newsreader`, `case` substitutes the hyphen and the brackets; `Inter` also substitutes the colon. In `IBM Plex Sans`, `Source Sans 3`, `Source Code Pro` and `Geist`, `case` does not touch these characters. `IBM Plex Mono` and `Commit Mono` have no `case` at all (output in [Reproduce](#reproduce)).
   - `case` is worth testing on all-figure strings such as `2026-01-31`. Whether it looks better was not judged.
6. **Identifiers.** In labels like `FY2026 Q3`, `W1` or `P11`, `1`, `l` and `I` must not be confused. If `Inter` is chosen, turn on `cv05` and `cv08` (or `ss02`) for identifiers and code-like labels.
7. **Mixing faces.** When a mono face sits inline in a sans face, match their x-heights with `size-adjust` or a different `font-size`. For example, `Source Code Pro`'s x/em of 0.478 sits next to `Inter`'s 0.546.
8. **Table cells.** Set dates in tables in the UI face with tabular figures, not in the mono face. Keep mono for code, JSON definitions and terminal output. This is a style suggestion, not a measured result.

## Pairings

### A. Neutral grotesque: `Inter` + `Commit Mono`

- **UI and body:** `Inter`, with `opsz` 14 for text and up to 32 for headings.
- **Code:** `Commit Mono`, 400 and 700.
- **Why.**
  - It is the most neutral of the three.
  - The two faces have nearly the same x-height (0.546 and 0.540).
  - `Commit Mono`'s zero is slashed with no CSS at all.
- **Trade-offs.**
  - `Inter` needs `tnum`, `zero` and `cv05`/`cv08` switched on in the right places.
  - `Commit Mono` is static only, has no `zero`, `case` or `frac`, and has no Cyrillic.
  - Its last release was v1.143 in December 2023, though the repository had commits up to September 2025 (GitHub API).
- **Risks.**
  - `Inter` is common. As of 1 November 2026, [linear.app](https://linear.app/) preloads `InterVariable.woff2?v=4.1` (`curl` and `grep`), so this pairing risks reading as a default rather than a choice.
  - If `Commit Mono` falls behind, `IBM Plex Mono` (variable) is the drop-in alternative.

### B. Technical, slightly warmer: `IBM Plex Sans` + `IBM Plex Mono` (+ `IBM Plex Serif`) — lead suggestion

- **UI and body:** `IBM Plex Sans`.
- **Code:** `IBM Plex Mono`, the variable 1.000 file from IBM.
- **Optional headings:** `IBM Plex Serif`.
- **Why.**
  - Figures are tabular by default.
  - `I`, `l` and `1` are told apart by default in both the sans and the mono.
  - Every member has a slashed zero through `zero`.
  - The members share one x-height (0.516 x/em).
  - Greek and Cyrillic are covered, and the Mono gained a variable version in 2026.
  - In my judgement it feels engineered without being cold, which fits a 2.5 personality leaning technical.
- **Trade-offs.**
  - There are no proportional figures in the sans.
  - There is no `case` feature in the sans.
  - The two-axis Latin WOFF2 is 69.1 kB.
- **Risks.**
  - The name and the look are tied to IBM.
  - `Plex` is used widely enough that it is not distinctive on its own, so the logo and colour have to carry the identity.

### C. Ledger and classic print: `Source Serif 4` headings + `Source Sans 3` + `Source Code Pro`

- **Headings:** `Source Serif 4`, using its display `opsz`.
- **UI and body:** `Source Sans 3`.
- **Code:** `Source Code Pro`.
- **Alternative heading face:** `Newsreader`, for a more editorial feel.
- **Why.**
  - It is one superfamily from Adobe, and all three are tabular by default.
  - The x-heights are matched (0.475–0.478 x/em).
  - The serif headings recall printed financial statements, which fits the "very slightly classic" lean.
- **Trade-offs.**
  - The small x-height means larger body sizes.
  - `Source Serif 4`'s two-axis subset is 194.5 kB, though pinning `opsz` would cut it (not measured).
  - The serif's slashed zero should stay off.
  - The upstream releases date from 2023, so the family is stable but slow-moving.
- **Risks.** A serif-led page can look like a publication rather than developer documentation, so this direction needs the owner's explicit agreement.

## Not verified and open questions

- **Newer upstream versions.** `Inter` 4.1, `IBM Plex Mono` 2.5.0 (static), `Source Serif 4` 4.005R and `Geist` 1.9.0 were not inspected; the tables describe the google/fonts files.
- **No real shaping.** Rendering used FreeType through `Pillow`, with no `HarfBuzz` shaping. Contextual substitutions, kerning and browser hinting were not exercised, and Windows and macOS rendering were not compared.
- **No small sizes.** Sizes of 12–14 px, where UI labels sit, were not looked at. The smallest render was 17 px.
- **No dark-mode check per face.** Dark mode was checked only on one 30 px strip per face. Whether light-on-dark text needs a different weight was not tested.
- **Crude metric.** The pixel metric is crude, and the visual judgements are mine.
- **Thin evidence for "default look".** The risk that `Inter` or `Geist` reads as a default rests on two home pages, not a survey.
- **Unchecked features.** What `Fira Code`'s `calt` substitutes, and what `Geist`'s `ss09` zero is meant to be, were not checked.
- **Line height.** Vertical metrics (ascender, descender and line gap) were not compared, though they affect line height when faces are mixed.

## Reproduce

Downloads, the virtual environment and the outputs live outside the repository. The scripts expect files in a `downloads/` folder and write to an `out/` folder. Run every script with `python -I`, from a folder other than the downloads.

```sh
# Font files: google/fonts at main, fetched 1 Nov 2026
base=https://raw.githubusercontent.com/google/fonts/main/ofl
curl -sSfLo 'downloads/gf/inter/Inter[opsz,wght].ttf' "$base/inter/Inter%5Bopsz,wght%5D.ttf"
# ...one line per file named in the tables; file names come from
# gh api repos/google/fonts/contents/ofl/<family> --jq '.[].name'
curl -sSfLO https://github.com/eigilnikolajsen/commit-mono/releases/download/v1.143/CommitMono-1.143.zip
curl -sSfLO 'https://github.com/IBM/plex/releases/download/%40ibm/plex-mono-variable%401.0.0/plex-mono-variable.zip'

python -m venv venv
venv/Scripts/python -m pip install fonttools brotli pillow   # fontTools 4.68.0, Pillow 13.0.0 used
venv/Scripts/python -I scripts/inspect_fonts.py downloads/gf/*/*.ttf downloads/commitmono/CommitMono-400-Regular.otf
venv/Scripts/python -I scripts/render_specimens.py out/specimens downloads/gf/*/*.ttf downloads/commitmono/CommitMono-400-Regular.otf
venv/Scripts/python -I scripts/case_and_inter.py 'downloads/gf/inter/Inter[opsz,wght].ttf' <fonts to test for case>

# Font references on admired sites (1 Nov 2026)
curl -sL https://linear.app/ | grep -o 'fonts/InterVariable[^"]*' | head -1
curl -sL https://vercel.com/ | grep -o 'media/Geist[^"]*' | head -3
```

### `inspect_fonts.py`

```python
"""Inspect OFL font files for the fiscalclock typography research.

Usage: python -I inspect_fonts.py FONT [FONT ...]
Needs fontTools and brotli (pip install fonttools brotli).
Prints one block per font. Reads the files only; writes WOFF2 subsets to a temp dir.
"""

import io
import sys
from fontTools.ttLib import TTFont
from fontTools import subset
from fontTools.varLib import instancer

DIGITS = "0123456789"
WATCH = "0O1lI|7S5"
# Roughly the Google Fonts "latin" unicode-range.
LATIN = (
    list(range(0x20, 0x7F)) + list(range(0xA0, 0x100))
    + [0x131, 0x152, 0x153, 0x2BB, 0x2BC, 0x2C6, 0x2DA, 0x2DC, 0x2074, 0x20AC,
       0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD]
    + list(range(0x2000, 0x2070))
)
RANGES = {
    "LatinExtA": (0x100, 0x17F),
    "LatinExtB": (0x180, 0x24F),
    "LatinExtAdd": (0x1E00, 0x1EFF),
    "Greek": (0x370, 0x3FF),
    "Cyrillic": (0x400, 0x4FF),
}
FEATURES = ["tnum", "pnum", "lnum", "onum", "zero", "case", "frac", "sups", "calt", "liga"]


def name(font, nid):
    rec = font["name"].getDebugName(nid)
    return rec or ""


def gsub_features(font):
    """Return {tag: [lookup indices]} for GSUB."""
    out = {}
    if "GSUB" not in font or font["GSUB"].table.FeatureList is None:
        return out
    for fr in font["GSUB"].table.FeatureList.FeatureRecord:
        out.setdefault(fr.FeatureTag, set()).update(fr.Feature.LookupListIndex)
    return out


def single_subs(font, lookup_indices):
    """Collect single (type 1) and first-alternate (type 3) substitutions."""
    mapping = {}
    lookups = font["GSUB"].table.LookupList.Lookup
    for li in sorted(lookup_indices):
        lk = lookups[li]
        for st in lk.SubTable:
            ltype = lk.LookupType
            if ltype == 7:
                ltype = st.ExtensionLookupType
                st = st.ExtSubTable
            if ltype == 1:
                for k, v in st.mapping.items():
                    mapping.setdefault(k, v)
            elif ltype == 3:
                for k, v in st.alternates.items():
                    mapping.setdefault(k, v[0])
    return mapping


def digit_widths(font):
    cmap = font.getBestCmap()
    hmtx = font["hmtx"]
    return [hmtx[cmap[ord(d)]][0] if ord(d) in cmap else None for d in DIGITS]


def woff2_latin_size(path):
    font = TTFont(path)
    opts = subset.Options()
    opts.flavor = "woff2"
    opts.layout_features = ["*"]
    opts.name_IDs = ["*"]
    opts.notdef_outline = True
    sub = subset.Subsetter(opts)
    sub.populate(unicodes=LATIN)
    sub.subset(font)
    buf = io.BytesIO()
    font.flavor = "woff2"
    font.save(buf)
    return len(buf.getvalue())


def main(paths):
    for path in paths:
        font = TTFont(path)
        cmap = font.getBestCmap()
        rev = {}
        for cp, g in cmap.items():
            rev.setdefault(g, cp)
        print("=" * 72)
        print("file:", path.replace("\\", "/").split("/")[-1])
        print("family:", name(font, 16) or name(font, 1))
        print("version:", name(font, 5))
        print("licence (nameID 13, first 60 chars):", name(font, 13)[:60].replace("\n", " "))
        print("licence URL (nameID 14):", name(font, 14))
        if "fvar" in font:
            axes = ", ".join(f"{a.axisTag} {a.minValue:g}-{a.maxValue:g} (default {a.defaultValue:g})"
                             for a in font["fvar"].axes)
            print("variable axes:", axes)
        else:
            print("variable axes: none (static)", "usWeightClass", font["OS/2"].usWeightClass)
        os2 = font["OS/2"]
        upm = font["head"].unitsPerEm
        xh, ch = getattr(os2, "sxHeight", 0), getattr(os2, "sCapHeight", 0)
        print(f"unitsPerEm {upm}; xHeight {xh}; capHeight {ch}; x/cap {xh / ch:.3f}; x/em {xh / upm:.3f}" if ch else "no x/cap data")
        widths = digit_widths(font)
        print("default digit advances 0-9:", widths, "-> tabular by default" if len(set(widths)) == 1 else "-> proportional by default")
        if "fvar" in font:
            wtag = [a for a in font["fvar"].axes if a.axisTag == "wght"]
            if wtag:
                bold = instancer.instantiateVariableFont(TTFont(path), {"wght": min(700, wtag[0].maxValue)})
                bw = digit_widths(bold)
                print("digit advances at wght 700:", bw, "equal" if len(set(bw)) == 1 else "NOT equal")
        hm = font["hmtx"].metrics
        adv = {w for g, (w, _) in hm.items() if w > 0}
        print("post.isFixedPitch:", font["post"].isFixedPitch, "| distinct non-zero advances:", len(adv))
        feats = gsub_features(font)
        print("GSUB features present:", " ".join(t for t in FEATURES if t in feats))
        ss = sorted(t for t in feats if t.startswith("ss") or t.startswith("cv"))
        print("ss/cv features:", " ".join(ss) if ss else "none")
        for tag in ["tnum", "zero"] + ss:
            if tag not in feats:
                continue
            m = single_subs(font, feats[tag])
            hit = []
            for ch_ in WATCH + "aglrtfy":
                g = cmap.get(ord(ch_))
                if g in m:
                    hit.append(f"{ch_}->{m[g]}")
            if tag == "tnum":
                changed = [d for d in DIGITS if cmap.get(ord(d)) in m]
                print(f"  tnum substitutes digits: {''.join(changed) or 'none (single-sub)'}")
                if changed:
                    tw = [font['hmtx'][m.get(cmap[ord(d)], cmap[ord(d)])][0] for d in DIGITS]
                    print(f"  digit advances with tnum: {tw}")
            elif hit:
                print(f"  {tag} affects: {' '.join(hit)}")
        cov = {}
        for k, (lo, hi) in RANGES.items():
            cov[k] = sum(1 for cp in cmap if lo <= cp <= hi)
        print("cmap total:", len(cmap), "| " + ", ".join(f"{k} {v}" for k, v in cov.items()))
        try:
            print("WOFF2 latin subset (all axes, all features):", woff2_latin_size(path), "bytes")
        except Exception as exc:  # noqa: BLE001
            print("WOFF2 subset failed:", exc)


if __name__ == "__main__":
    main(sys.argv[1:])
```

<details>
<summary>Output for the finalists (the full run covers all 31 files)</summary>

```text
file: Inter[opsz,wght].ttf
family: Inter
version: Version 4.001;git-66647c0bb
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): https://openfontlicense.org
variable axes: opsz 14-32 (default 14), wght 100-900 (default 400)
unitsPerEm 2048; xHeight 1118; capHeight 1490; x/cap 0.750; x/em 0.546
default digit advances 0-9: [1292, 833, 1249, 1265, 1323, 1215, 1270, 1159, 1267, 1270] -> proportional by default
digit advances at wght 700: [1381, 883, 1289, 1322, 1385, 1274, 1330, 1191, 1333, 1330] NOT equal
post.isFixedPitch: 0 | distinct non-zero advances: 583
GSUB features present: tnum pnum zero case frac sups calt
ss/cv features: cv01 cv02 cv03 cv04 cv05 cv06 cv07 cv08 cv09 cv10 cv11 cv12 cv13 cv14 ss01 ss02 ss03 ss04 ss05 ss06 ss07 ss08
  tnum substitutes digits: 0123456789
  digit advances with tnum: [1328, 1328, 1328, 1328, 1328, 1328, 1328, 1328, 1328, 1328]
  zero affects: 0->zero.slash
  cv01 affects: 1->one.ss01
  cv05 affects: l->l.ss02 l->l.ss02
  cv08 affects: I->I.1
  cv11 affects: a->a.1
  cv12 affects: f->f.1
  cv13 affects: t->t.1
  ss02 affects: 0->zero.slash l->l.ss02 I->I.1 l->l.ss02
  ss04 affects: l->l.ss02 I->I.1 l->l.ss02
  ss05 affects: 0->uni24EA O->uni24C4 1->uni2460 I->uni24BE 7->uni2466 S->uni24C8 5->uni2464
  ss06 affects: 0->zero.squared O->O.squared 1->one.squared I->I.squared 7->seven.squared S->S.squared 5->five.squared
cmap total: 2849 | LatinExtA 127, LatinExtB 207, LatinExtAdd 255, Greek 105, Cyrillic 248
WOFF2 latin subset (all axes, all features): 99816 bytes
file: IBMPlexSans[wdth,wght].ttf
family: IBM Plex Sans
version: Version 3.201
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): http://scripts.sil.org/OFL
variable axes: wght 100-700 (default 400), wdth 75-100 (default 100)
unitsPerEm 1000; xHeight 516; capHeight 698; x/cap 0.739; x/em 0.516
default digit advances 0-9: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] -> tabular by default
digit advances at wght 700: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] equal
post.isFixedPitch: 0 | distinct non-zero advances: 257
GSUB features present: lnum onum zero frac sups liga
ss/cv features: ss01 ss02 ss03 ss04 ss05 ss06
  zero affects: 0->zero.alt02
  ss01 affects: a->a.alt01
  ss02 affects: g->g.alt01
  ss03 affects: 0->zero.alt02
  ss04 affects: 0->zero.alt01
cmap total: 891 | LatinExtA 128, LatinExtB 33, LatinExtAdd 97, Greek 73, Cyrillic 192
WOFF2 latin subset (all axes, all features): 69132 bytes
file: IBMPlexMono-Regular.ttf
family: IBM Plex Mono
version: Version 2.3
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): http://scripts.sil.org/OFL
variable axes: none (static) usWeightClass 400
unitsPerEm 1000; xHeight 516; capHeight 698; x/cap 0.739; x/em 0.516
default digit advances 0-9: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] -> tabular by default
post.isFixedPitch: 1 | distinct non-zero advances: 1
GSUB features present: zero frac sups
ss/cv features: ss01 ss02 ss03 ss04 ss05
  zero affects: 0->zero.alt01
  ss01 affects: a->a.alt01
  ss02 affects: g->g.alt01
  ss03 affects: 0->zero.alt01
  ss04 affects: 0->zero.alt02
cmap total: 930 | LatinExtA 128, LatinExtB 33, LatinExtAdd 97, Greek 1, Cyrillic 168
WOFF2 latin subset (all axes, all features): 15256 bytes
file: SourceSans3[wght].ttf
family: Source Sans 3
version: Version 3.052;hotconv 1.1.0;makeotfexe 2.6.0
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): http://scripts.sil.org/OFL
variable axes: wght 200-900 (default 200)
unitsPerEm 1000; xHeight 478; capHeight 660; x/cap 0.724; x/em 0.478
default digit advances 0-9: [472, 472, 472, 472, 472, 472, 472, 472, 472, 472] -> tabular by default
digit advances at wght 700: [528, 528, 528, 528, 528, 528, 528, 528, 528, 528] equal
post.isFixedPitch: 0 | distinct non-zero advances: 320
GSUB features present: pnum onum zero case frac sups liga
ss/cv features: cv01 cv02 cv03 cv04 cv05 cv06 cv07 cv08 cv09 cv10 cv11 cv12 cv13 cv14 cv15 cv16 cv17 cv18 cv19 ss01 ss02 ss03 ss04 ss05 ss06 ss07 ss08 ss09 ss10
  zero affects: 0->zero.0s
  cv01 affects: I->uniA7AE
  cv02 affects: a->a.a
  cv03 affects: g->g.a
  cv04 affects: l->l.a l->l.a
  cv05 affects: a->a.b
  cv17 affects: 0->zero.c 1->one.c 7->seven.c 5->five.c
  cv18 affects: 0->zero.0
  cv19 affects: 0->zero.0s
  ss01 affects: I->uniA7AE
  ss02 affects: l->l.a a->a.a g->g.a l->l.a
  ss03 affects: a->a.a
  ss04 affects: g->g.a
  ss05 affects: l->l.a l->l.a
  ss06 affects: a->a.b
  ss09 affects: 0->zero.c 1->one.c 7->seven.c 5->five.c
  ss10 affects: 0->zero.0
cmap total: 1615 | LatinExtA 128, LatinExtB 100, LatinExtAdd 186, Greek 88, Cyrillic 156
WOFF2 latin subset (all axes, all features): 47244 bytes
file: SourceSerif4[opsz,wght].ttf
family: Source Serif 4
version: Version 4.004;hotconv 1.0.116;makeotfexe 2.5.65601
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): http://scripts.sil.org/OFL
variable axes: wght 200-900 (default 400), opsz 8-60 (default 20)
unitsPerEm 1000; xHeight 475; capHeight 670; x/cap 0.709; x/em 0.475
default digit advances 0-9: [500, 500, 500, 500, 500, 500, 500, 500, 500, 500] -> tabular by default
digit advances at wght 700: [542, 542, 542, 542, 542, 542, 542, 542, 542, 542] equal
post.isFixedPitch: 0 | distinct non-zero advances: 338
GSUB features present: tnum pnum lnum onum zero case frac sups liga
ss/cv features: ss01 ss02
  tnum substitutes digits: none (single-sub)
  zero affects: 0->glyph00427
cmap total: 918 | LatinExtA 122, LatinExtB 31, LatinExtAdd 142, Greek 80, Cyrillic 150
WOFF2 latin subset (all axes, all features): 194520 bytes
file: SourceCodePro[wght].ttf
family: Source Code Pro
version: Version 1.026;hotconv 1.1.0;makeotfexe 2.6.0
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): https://openfontlicense.org
variable axes: wght 200-900 (default 200)
unitsPerEm 1000; xHeight 478; capHeight 660; x/cap 0.724; x/em 0.478
default digit advances 0-9: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] -> tabular by default
digit advances at wght 700: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] equal
post.isFixedPitch: 1 | distinct non-zero advances: 1
GSUB features present: onum zero case frac sups
ss/cv features: cv01 cv02 cv04 cv06 cv07 cv08 cv09 cv10 cv11 cv12 cv14 cv15 cv16 cv17 ss01 ss02 ss03 ss04 ss05 ss06 ss07
  zero affects: 0->zero.a
  cv01 affects: a->a.a
  cv02 affects: g->g.a
  cv04 affects: l->l.a l->l.a
  cv12 affects: 0->zero.a
  cv17 affects: 1->one.a
  ss02 affects: a->a.a
  ss03 affects: g->g.a
  ss04 affects: l->l.a l->l.a
  ss07 affects: 1->one.a
cmap total: 1334 | LatinExtA 128, LatinExtB 48, LatinExtAdd 171, Greek 84, Cyrillic 150
WOFF2 latin subset (all axes, all features): 27228 bytes
file: Geist[wght].ttf
family: Geist
version: Version 1.800
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): https://openfontlicense.org
variable axes: wght 100-900 (default 400)
unitsPerEm 1000; xHeight 530; capHeight 710; x/cap 0.746; x/em 0.530
default digit advances 0-9: [663, 384, 619, 613, 615, 626, 593, 524, 604, 593] -> proportional by default
digit advances at wght 700: [693, 449, 653, 650, 656, 671, 627, 544, 664, 631] NOT equal
post.isFixedPitch: 0 | distinct non-zero advances: 243
GSUB features present: tnum pnum case frac sups liga
ss/cv features: ss01 ss02 ss03 ss04 ss05 ss06 ss07 ss08 ss09 ss10 ss11
  tnum substitutes digits: 0123456789
  digit advances with tnum: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600]
  ss01 affects: a->a.ss01
  ss02 affects: a->a.ss02
  ss03 affects: l->l.ss03 l->l.ss03
  ss05 affects: I->I.ss05
  ss09 affects: 0->zero.ss09 1->one.ss09
cmap total: 728 | LatinExtA 117, LatinExtB 19, LatinExtAdd 99, Greek 6, Cyrillic 134
WOFF2 latin subset (all axes, all features): 31960 bytes
file: GeistMono[wght].ttf
family: Geist Mono
version: Version 1.701
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): https://openfontlicense.org
variable axes: wght 100-900 (default 400)
unitsPerEm 1000; xHeight 530; capHeight 710; x/cap 0.746; x/em 0.530
default digit advances 0-9: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] -> tabular by default
digit advances at wght 700: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] equal
post.isFixedPitch: 1 | distinct non-zero advances: 2
GSUB features present: case frac sups
ss/cv features: ss01 ss02 ss03 ss04 ss06 ss07 ss08 ss09 ss10 ss11
  ss01 affects: a->a.ss01
  ss02 affects: a->a.ss02
  ss03 affects: l->l.ss03 l->l.ss03
  ss09 affects: 0->zero.ss09
cmap total: 889 | LatinExtA 115, LatinExtB 19, LatinExtAdd 99, Greek 5, Cyrillic 134
WOFF2 latin subset (all axes, all features): 33480 bytes
file: Newsreader[opsz,wght].ttf
family: Newsreader
version: Version 1.003
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): http://scripts.sil.org/OFL
variable axes: wght 200-800 (default 400), opsz 6-72 (default 18)
unitsPerEm 2000; xHeight 852; capHeight 1340; x/cap 0.636; x/em 0.426
default digit advances 0-9: [1100, 1100, 1100, 1100, 1100, 1100, 1100, 1100, 1100, 1100] -> tabular by default
digit advances at wght 700: [1230, 1230, 1230, 1230, 1230, 1230, 1230, 1230, 1230, 1230] equal
post.isFixedPitch: 0 | distinct non-zero advances: 187
GSUB features present: tnum pnum case sups liga
ss/cv features: none
  tnum substitutes digits: 0123456789
  digit advances with tnum: [1100, 1100, 1100, 1100, 1100, 1100, 1100, 1100, 1100, 1100]
cmap total: 564 | LatinExtA 124, LatinExtB 62, LatinExtAdd 97, Greek 0, Cyrillic 0
WOFF2 latin subset (all axes, all features): 136952 bytes
file: AtkinsonHyperlegibleMono[wght].ttf
family: Atkinson Hyperlegible Mono
version: Version 2.001
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): https://openfontlicense.org
variable axes: wght 200-800 (default 200)
unitsPerEm 1000; xHeight 496; capHeight 668; x/cap 0.743; x/em 0.496
default digit advances 0-9: [632, 632, 632, 632, 632, 632, 632, 632, 632, 632] -> tabular by default
digit advances at wght 700: [632, 632, 632, 632, 632, 632, 632, 632, 632, 632] equal
post.isFixedPitch: 1 | distinct non-zero advances: 1
GSUB features present: zero case frac sups
ss/cv features: none
  zero affects: 0->zero.zero
cmap total: 359 | LatinExtA 93, LatinExtB 6, LatinExtAdd 9, Greek 4, Cyrillic 0
WOFF2 latin subset (all axes, all features): 17012 bytes
file: AtkinsonHyperlegibleNext[wght].ttf
family: Atkinson Hyperlegible Next
version: Version 2.001
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): https://openfontlicense.org
variable axes: wght 200-800 (default 400)
unitsPerEm 1000; xHeight 496; capHeight 668; x/cap 0.743; x/em 0.496
default digit advances 0-9: [648, 407, 548, 576, 614, 588, 598, 510, 615, 595] -> proportional by default
digit advances at wght 700: [659, 440, 579, 600, 624, 607, 626, 537, 624, 571] NOT equal
post.isFixedPitch: 0 | distinct non-zero advances: 144
GSUB features present: tnum pnum case frac sups
ss/cv features: none
  tnum substitutes digits: 0123456789
  digit advances with tnum: [632, 632, 632, 632, 632, 632, 632, 632, 632, 632]
cmap total: 362 | LatinExtA 93, LatinExtB 6, LatinExtAdd 9, Greek 4, Cyrillic 0
WOFF2 latin subset (all axes, all features): 32772 bytes
file: CommitMono-400-Regular.otf
family: CommitMono
version: Version 1.143
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): https://scripts.sil.org/OFL
variable axes: none (static) usWeightClass 400
unitsPerEm 1000; xHeight 540; capHeight 700; x/cap 0.771; x/em 0.540
default digit advances 0-9: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] -> tabular by default
post.isFixedPitch: 1 | distinct non-zero advances: 1
GSUB features present: calt
ss/cv features: cv01 cv02 cv03 cv04 cv05 cv06 cv07 cv08 cv09 cv10 cv11 ss01 ss02 ss03 ss04 ss05
  cv01 affects: a->a.cv01
  cv02 affects: g->g.cv02
  cv07 affects: 0->zero.cv07
  cv08 affects: a->a.cv08 g->g.cv08 f->f.cv08 y->y.cv08
  cv10 affects: l->l.cv10 l->l.cv10
  cv11 affects: 1->one.cv11
cmap total: 1175 | LatinExtA 128, LatinExtB 12, LatinExtAdd 8, Greek 72, Cyrillic 0
WOFF2 latin subset (all axes, all features): 20800 bytes
file: IBM_Plex_Mono_Var-Roman.ttf
family: IBM Plex Mono Var
version: Version 1.000
licence (nameID 13, first 60 chars): This Font Software is licensed under the SIL Open Font Licen
licence URL (nameID 14): http://scripts.sil.org/OFL
variable axes: wght 100-700 (default 400)
unitsPerEm 1000; xHeight 516; capHeight 698; x/cap 0.739; x/em 0.516
default digit advances 0-9: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] -> tabular by default
digit advances at wght 700: [600, 600, 600, 600, 600, 600, 600, 600, 600, 600] equal
post.isFixedPitch: 1 | distinct non-zero advances: 1
GSUB features present: lnum onum zero frac sups
ss/cv features: ss01 ss02 ss03 ss04 ss05 ss06 ss07 ss08 ss09
  zero affects: 0->zero.alt01
  ss01 affects: a->a.alt01
  ss02 affects: g->g.alt01
  ss03 affects: 0->zero.alt01
  ss04 affects: 0->zero.alt02
  ss08 affects: l->l.alt01 l->l.alt01
cmap total: 1045 | LatinExtA 128, LatinExtB 33, LatinExtAdd 125, Greek 1, Cyrillic 192
WOFF2 latin subset (all axes, all features): 32120 bytes
```

</details>

### `render_specimens.py`

<details>
<summary>Script that renders the specimens and the metric</summary>

```python
"""Render specimen PNGs and a glyph-confusion metric for candidate fonts.

Usage: python -I render_specimens.py OUT_DIR FONT [FONT ...]
Needs fontTools and Pillow. Pillow here has no libraqm, so OpenType
features are applied by "freezing" single substitutions into the cmap
(the same idea as pyftfeatfreeze) before rendering.
"""

import os
import sys
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer
from PIL import Image, ImageDraw, ImageFont, ImageChops

# Font-specific features that disambiguate 0/O, 1/l/I (from inspect_fonts.py output).
EXTRA = {
    "Inter": ["ss02"], "Inter Tight": ["ss02"],
    "Geist": ["ss09", "ss03", "ss05"], "Geist Mono": ["ss09", "ss03"],
    "Source Sans 3": ["cv01", "cv04"], "Figtree": ["ss02"], "Public Sans": ["ss01"],
    "Mona Sans": ["ss02", "ss03"], "Hubot Sans": ["ss02", "ss04"],
    "Victor Mono": ["ss02"], "CommitMono": ["cv07"],
}
PAIRS = [("I", "l"), ("l", "1"), ("I", "1"), ("l", "|"), ("0", "O"), ("5", "S"), ("rn", "m"), ("1", "7")]


def single_subs(font, tags):
    if "GSUB" not in font or font["GSUB"].table.FeatureList is None:
        return {}
    idx = set()
    for fr in font["GSUB"].table.FeatureList.FeatureRecord:
        if fr.FeatureTag in tags:
            idx.update(fr.Feature.LookupListIndex)
    # Apply lookups in lookup-list order and chain them, as a shaper would
    # (for example tnum zero -> zero.tf, then zero zero.tf -> zero.tf.slash).
    mapping = {g: g for g in font.getGlyphOrder()}
    lookups = font["GSUB"].table.LookupList.Lookup
    for li in sorted(idx):
        lk = lookups[li]
        step = {}
        for st in lk.SubTable:
            t = lk.LookupType
            if t == 7:
                t, st = st.ExtensionLookupType, st.ExtSubTable
            if t == 1:
                for k, v in st.mapping.items():
                    step.setdefault(k, v)
            elif t == 3:
                for k, v in st.alternates.items():
                    step.setdefault(k, v[0])
        mapping = {k: step.get(v, v) for k, v in mapping.items()}
    return mapping


def static_instance(path):
    font = TTFont(path)
    if "fvar" in font:
        loc = {}
        for a in font["fvar"].axes:
            v = a.defaultValue
            if a.axisTag == "wght":
                v = min(max(400, a.minValue), a.maxValue)
            if a.axisTag == "wdth":
                v = min(max(100, a.minValue), a.maxValue)
            loc[a.axisTag] = v
        font = instancer.instantiateVariableFont(font, loc)
    return font


def frozen(path, tags, out_path):
    font = static_instance(path)
    if tags:
        m = single_subs(font, tags)
        for table in font["cmap"].tables:
            if table.isUnicode():
                table.cmap = {cp: m.get(g, g) for cp, g in table.cmap.items()}
    font.save(out_path)
    return out_path


def family(path):
    f = TTFont(path)
    return f["name"].getDebugName(16) or f["name"].getDebugName(1)


def ink(fontfile, text, size=96):
    img = Image.new("L", (size * 3, int(size * 1.6)), 0)
    ImageDraw.Draw(img).text((8, 8), text, font=ImageFont.truetype(fontfile, size), fill=255)
    return img.point(lambda p: 255 if p > 127 else 0)


def confusion(fontfile):
    """1 - IoU of the inked pixels of two strings drawn at the same origin.
    0 means identical shapes; higher means easier to tell apart."""
    out = []
    for a, b in PAIRS:
        ia, ib = ink(fontfile, a), ink(fontfile, b)
        inter = ImageChops.multiply(ia, ib).histogram()[255]
        union = ImageChops.lighter(ia, ib).histogram()[255]
        out.append(f"{a}/{b} {1 - inter / union:.2f}")
    return "  ".join(out)


def specimen(fam, default_ttf, feat_ttf, feat_tags, out_png):
    W, H = 1400, 560
    img = Image.new("RGB", (W, H), "white")
    d = ImageDraw.Draw(img)
    label = ImageFont.truetype(default_ttf, 18)
    big_d = ImageFont.truetype(default_ttf, 56)
    big_f = ImageFont.truetype(feat_ttf, 56)
    mid_d = ImageFont.truetype(default_ttf, 30)
    mid_f = ImageFont.truetype(feat_ttf, 30)
    body = ImageFont.truetype(default_ttf, 17)
    d.text((24, 12), f"{fam} - default glyphs (row 1, 3) vs {'+'.join(feat_tags)} (row 2, 4)", font=label, fill="#555")
    d.text((24, 40), "Il1| 0O 5S rnm 17 g", font=big_d, fill="black")
    d.text((24, 112), "Il1| 0O 5S rnm 17 g", font=big_f, fill="black")
    rows = ["2026-01-31  FY2026 Q3  W53  P13", "2025-11-01  1111-0808  9,876.10"]
    y = 200
    for r in rows:
        d.text((24, y), r, font=mid_d, fill="black")
        d.text((720, y), r, font=mid_f, fill="black")
        y += 44
    # dark strip, featured glyphs
    d.rectangle((0, 300, W, 400), fill="#0d0f12")
    d.text((24, 316), "2026-01-31  FY2026 Q3", font=mid_f, fill="#e8e8e8")
    d.text((24, 356), "start 2026-02-01  end_exclusive 2027-01-31", font=ImageFont.truetype(feat_ttf, 22), fill="#a0a8b0")
    d.text((24, 420), "A fiscal year is a definition, not a preset. Week 53 falls in years", font=body, fill="black")
    d.text((24, 444), "where the rule says so; every period has a start and an end_exclusive.", font=body, fill="black")
    col = "2026-01-31\n2021-11-11\n2028-07-17"
    d.text((880, 476), "column, featured:", font=label, fill="#555")
    d.multiline_text((1100, 420), col, font=ImageFont.truetype(feat_ttf, 22), fill="black", spacing=6)
    img.save(out_png)


def main(out_dir, paths):
    os.makedirs(os.path.join(out_dir, "frozen"), exist_ok=True)
    for path in paths:
        fam = family(path)
        base = os.path.splitext(os.path.basename(path))[0].split("[")[0]
        f = TTFont(path)
        tags_present = set()
        if "GSUB" in f and f["GSUB"].table.FeatureList is not None:
            tags_present = {fr.FeatureTag for fr in f["GSUB"].table.FeatureList.FeatureRecord}
        want = [t for t in ["tnum", "zero"] + EXTRA.get(fam, []) if t in tags_present]
        d_ttf = frozen(path, [], os.path.join(out_dir, "frozen", base + "-default.ttf"))
        f_ttf = frozen(path, want, os.path.join(out_dir, "frozen", base + "-feat.ttf"))
        specimen(fam, d_ttf, f_ttf, want or ["no features"], os.path.join(out_dir, base + ".png"))
        print(f"{fam:28s} default : {confusion(d_ttf)}")
        print(f"{'':28s} {'+'.join(want) or 'none':8s}: {confusion(f_ttf)}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2:])
```

</details>

### `case_and_inter.py`

```python
"""Which punctuation does `case` change, and how big is Inter's latin WOFF2 with opsz pinned?
python -I case_and_inter.py INTER_TTF FONT [FONT ...]"""
import io, sys
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer
from fontTools import subset

def case_hits(path):
    f = TTFont(path); cmap = f.getBestCmap(); g = f["GSUB"].table
    idx = {i for fr in g.FeatureList.FeatureRecord if fr.FeatureTag == "case" for i in fr.Feature.LookupListIndex}
    hits = set()
    for li in idx:
        lk = g.LookupList.Lookup[li]
        for st in lk.SubTable:
            t = lk.LookupType
            if t == 7: t, st = st.ExtensionLookupType, st.ExtSubTable
            if t == 1:
                for ch in "-:/()[]":
                    if cmap.get(ord(ch)) in st.mapping: hits.add(ch)
    return "".join(sorted(hits)) or "none of - : / ( ) [ ]"

for p in sys.argv[2:]:
    print(p.split("/")[-1], "case changes:", case_hits(p))
LATIN = list(range(0x20, 0x7F)) + list(range(0xA0, 0x100)) + list(range(0x2000, 0x2070)) + [0x20AC, 0x2122, 0x2212]
for opsz in (14, 32):
    tmp = io.BytesIO()
    instancer.instantiateVariableFont(TTFont(sys.argv[1]), {"opsz": opsz}).save(tmp)
    tmp.seek(0)
    f = TTFont(tmp)
    o = subset.Options(); o.flavor = "woff2"; o.layout_features = ["*"]
    s = subset.Subsetter(o); s.populate(unicodes=LATIN); s.subset(f)
    b = io.BytesIO(); f.flavor = "woff2"; f.save(b)
    print(f"Inter opsz pinned at {opsz}, wght 100-900 kept: {len(b.getvalue())} bytes")
```

Output:

```text
Inter[opsz,wght].ttf case changes: ()-:[]
IBMPlexSans[wdth,wght].ttf case changes: none of - : / ( ) [ ]
SourceSans3[wght].ttf case changes: none of - : / ( ) [ ]
SourceSerif4[opsz,wght].ttf case changes: ()-[]
SourceCodePro[wght].ttf case changes: none of - : / ( ) [ ]
Newsreader[opsz,wght].ttf case changes: ()-[]
Geist[wght].ttf case changes: none of - : / ( ) [ ]
Inter opsz pinned at 14, wght 100-900 kept: 63100 bytes
Inter opsz pinned at 32, wght 100-900 kept: 62916 bytes
```
