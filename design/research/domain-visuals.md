# Domain visuals: how to draw fiscal calendars

<!-- cspell:words WCAG Okabe Tufte Tol's SQLBI Holistics TMDL Kroger Costco ThreeTen Gantt chartjunk pdftotext heatmap tabular nums labelledby srcset rackcdn -->

Research track for the diagrams and data visuals in fiscalclock's documentation. Written 1 Nov 2026. Every page linked here was fetched on that date. Every date in this note comes from a cited source or from a standard-library script described in [Sample: NRF fiscal 2023 year strip](#sample-nrf-fiscal-2023-year-strip). The diagram grammar in [Proposed diagram grammar](#proposed-diagram-grammar) is a proposal, not a decision. Sizes, ratios and layouts in it are design judgements to be tested, not facts.

The concepts drawn here are defined in [`spec/definition-v1.md`](../../spec/definition-v1.md) and [`spec/api-v1.md`](../../spec/api-v1.md): every period is `[start, end_exclusive)`, `last_day` is `end_exclusive` minus one day, a week-based year has 364 or 371 days, and the extra week is the last week of `extra_week.period`.

## How fiscal and retail calendars are shown today

### NRF 4-5-4 calendar

- **Source.** The [NRF 4-5-4 calendar page](https://nrf.com/resources/4-5-4-calendar) links one PDF per three-year span, for example the [2023-2025 calendar](https://6a83cd4f6d8a17c1b6dd-0490b3ba35823e24e2c50ce7533598b0.ssl.cf1.rackcdn.com/454%20Calendars/3%20Year%20Calendar%2023-25.pdf) and the [2023-2025 calendar with 2023 restated](https://3e193cf16d86b4be9f49-478852ca0c5c18669502b2f9c7e01a0d.ssl.cf1.rackcdn.com/3%20Year%20Calendar%2023-25%20with%202023%20Restated.pdf). The page itself describes the 53rd week only in prose.
- **Visual approach**, read from `pdftotext -layout` output of both PDFs (no rendered image was available, so colours and line weights were not seen). One landscape page holds three fiscal years side by side, split into a spring and a fall season. Each fiscal month is a block of week rows under a Sunday-to-Saturday header. Each row shows seven day-of-month numbers. NRF week numbers 1-53 run down the outer edges, and the month name is spelt vertically, one letter per row. The 53rd week of 2023 is one extra row at the bottom of the January block, with a footnote that fiscal 2023 has 53 weeks. A second footnote says green and black shaded boxes mark sales release dates and holidays.
- **What works.** Counting weeks is trivial: one row per week, with the 4-5-4 rhythm visible as block heights. Putting years side by side lets a reader compare the same fiscal week across years.
- **What confuses.** Day numbers carry no month, so a row such as `29 30 31 1 2 3 4` (week 1 of 2023) needs the reader to infer the month change. Shading is the only marker of release dates and holidays, so colour alone carries meaning there. The restated PDF draws fiscal 2023 from 5 February 2023 with 52 rows, and the other PDF draws it from 29 January 2023 with 53 rows. Both are labelled 2023, so a diagram must always say which convention it shows. Holidays are out of scope for fiscalclock and should not appear in its diagrams.

### Wikipedia: 4-4-5 calendar and ISO week date

- The [4-4-5 calendar article](https://en.wikipedia.org/wiki/4%E2%80%934%E2%80%935_calendar) has no diagram or table of the structure. It describes the pattern in prose and gives year-end dates as bulleted lists, one list per year-end method. A list of dates is hard to scan for the pattern; it is the natural input for the [multi-year drift](#d6-multi-year-drift) dot plot.
- The [ISO week date article](https://en.wikipedia.org/wiki/ISO_week_date) uses tables only: weekday-by-dominical-letter tables with week-date cells such as `W01-1`, a 400-year table of long years with bold highlighting, and Gregorian-to-week-date lookup rows. Bold is the only marker of long years in that table. The tables answer lookup questions well but do not show a year as a span of time.

### Business-intelligence and ERP documentation

- **SQLBI.** [Week-based time intelligence in DAX](https://www.sqlbi.com/articles/week-based-time-intelligence-in-dax/) uses pivot-table screenshots that put a calendar-based and an ISO-based year-to-date column side by side, so the error shows up as a wrong number in a cell. [Using weekly calendars in Power BI](https://www.sqlbi.com/articles/using-weekly-calendars-in-power-bi/) explains that 4-4-5, 4-5-4 and 5-4-4 are the possible orders of a 13-week quarter, and uses screenshots to compare 4-4-5 quarters with Gregorian quarters and to show a 53-week year. The comparisons are in report tables, not drawn to scale.
- **Microsoft Power BI.** [Implement time-based calculations in Power BI](https://learn.microsoft.com/en-us/power-bi/transform-model/desktop-time-intelligence) lists retail 445, 454 and 544 patterns among the calendars that calendar-based time intelligence supports. Its one conceptual diagram is a lattice of time categories (year, quarter, month, week, day) joined by dependency arrows. It shows nesting, not dates.
- **SAP.** [Managing Fiscal Year Variants](https://learning.sap.com/courses/maintaining-financial-accounting-core-settings/managing-fiscal-year-variants) draws the fiscal year as a clock-like circle of 12 posting periods, with special periods 13-16 as boxes pointing at period 12. A circle conveys the cycle but cannot show unequal period lengths or dates. Periods are defined by end dates with a year shift of -1 or +1.
- **Oracle.** The [Accounting Cycles](https://docs.oracle.com/cd/E71871_01/doc.90/e72023/c_calendar_accountingcycle.htm) page explains 4-4-5, 5-4-4, 4-5-4 and 4-4-4 (with a 13th period) in prose, and shows Q3 and Q4 examples as images.
- **Microsoft Dynamics 365.** [Fiscal calendars, fiscal years, and periods](https://learn.microsoft.com/en-us/dynamics365/finance/budgeting/fiscal-calendars-fiscal-years-periods) has no diagram. Its example names a period by inclusive dates ("August 1 through August 31"), the convention fiscalclock exposes as `last_day`.

### Annual reports

- Costco's [fiscal 2017 Form 10-K](https://www.sec.gov/Archives/edgar/data/0000909832/000090983217000014/cost10k90317.htm) and Kroger's [fiscal 2018 Form 10-K](https://www.sec.gov/Archives/edgar/data/56873/000155837019002756/kr-20190202x10k.htm) mark year length as text under each column's year-end date in the selected financial data table, "(53 weeks)" or "(52 weeks)". Neither has a chart of the calendar. The habit is worth copying: every label of a week-based year in fiscalclock diagrams should carry its length in weeks.

### Libraries

- The pandas [`FY5253` reference](https://pandas.pydata.org/docs/reference/api/pandas.tseries.offsets.FY5253.html) explains the `nearest` and `last` variations in one sentence each and has no diagram.
- ThreeTen-Extra's [`AccountingChronologyBuilder`](https://www.threeten.org/threeten-extra/apidocs/org.threeten.extra/org/threeten/extra/chrono/AccountingChronologyBuilder.html) is text-only API documentation.

### What this survey shows

None of the sources fetched draws a fiscal year to scale on a day axis, and none marks where a period ends with an excluded boundary. Retail sources use week-row grids, ERP sources use tables or a circle, and annual reports use text. The extra week appears as an extra row, a footnote or a "(53 weeks)" caption. fiscalclock can borrow the NRF week grid and the annual-report week count. The to-scale year strip and the half-open boundary marking have no precedent in the sources fetched, so fiscalclock has to design and test them itself.

## Guidance on drawing intervals precisely

- **Data-ink and chartjunk.** Edward Tufte's [*The Visual Display of Quantitative Information*](https://www.edwardtufte.com/book/the-visual-display-of-quantitative-information/) covers the data-ink ratio and small multiples. The [Chartjunk article](https://en.wikipedia.org/wiki/Chartjunk) summarises Tufte's view of non-data ink and the research that questions it, including the finding that some embellishment helps memorability. For reference diagrams that people count days from, removing non-data ink is the safer default.
- **Small multiples.** The [Small multiple article](https://en.wikipedia.org/wiki/Small_multiple) defines them as a series of similar charts sharing the same scale and axes. That is the rule for comparing years: same scale, same alignment, one panel per year.
- **Interval bars.** In a [Gantt chart](https://en.wikipedia.org/wiki/Gantt_chart) the bar width shows each activity's duration on a time axis. A year strip is a one-row Gantt chart of periods.
- **Half-open intervals.** [Interval notation](https://en.wikipedia.org/wiki/Interval_(mathematics)) writes the half-open interval as `[a, b)`, or `[a, b[` in the ISO 31-11 style. Rust's [`Range`](https://doc.rust-lang.org/std/ops/struct.Range.html) is documented as "A (half-open) range bounded inclusively below and exclusively above". ThreeTen-Extra's [`LocalDateRange`](https://www.threeten.org/threeten-extra/apidocs/org.threeten.extra/org/threeten/extra/LocalDateRange.html) stores a date range with an inclusive start and an exclusive end and derives an inclusive end on request, the same model as `start`, `end_exclusive` and `last_day`. Filled and open circles for closed and open endpoints are a common number-line convention, but no source fetched for this note shows it, so treat that convention as unverified.
- **Calendar heat maps.** The [Holistics calendar heat map](https://docs.holistics.io/docs/charts/dynamic-content-blocks/gallery/calendar-heatmap) places each day by week index (column) and day of week (row). The [week grid](#d2-week-grid) uses the same idea turned 90 degrees, one row per week, as NRF does.
- **Colour not alone.** WCAG 2.2 [Use of Color (1.4.1)](https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html) requires that colour is not the only visual means of conveying information. It lists colour plus pattern as a technique. [Non-text Contrast (1.4.11)](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html) asks for a 3:1 contrast ratio for graphical objects needed to understand the content, including adjacent segments. Its understanding document exempts segments whose information is also given by labels and values.
- **Text alternatives.** The W3C [complex images tutorial](https://www.w3.org/WAI/tutorials/images/complex/) asks for a short description plus a long description that carries the values and relationships, kept visible on the page, for example in a `figure` with a `figcaption`. In SVG, MDN describes [`title`](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/title) as the accessible name and [`desc`](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/desc) as the long description. MDN recommends referencing visible text with `aria-labelledby` where it exists.
- **Colour-blind-safe palettes.** Okabe and Ito's [Color Universal Design](https://jfly.uni-koeln.de/color/) names a set of hues chosen for colour-blind readers and advises combining colour with shapes, line types, fill patterns, hatching and direct labels instead of a separate key. Paul Tol's [colour schemes](https://sronpersonalpages.nl/~pault/) include qualitative sets (bright, high-contrast, vibrant, muted, medium-contrast, pale, dark, light) of 3 to 11 colours. Of these, the high-contrast scheme is described as working in greyscale too. The page does not discuss dark backgrounds, so dark-theme values must be checked separately.

## Proposed diagram grammar

Six diagram types share one set of rules. The types are D1 year strip, D2 week grid, D3 boundary detail, D4 52 vs 53-week comparison, D5 two-calendar comparison and D6 multi-year drift.

### Shared rules

1. **Days are cells, boundaries are lines.** On every day axis, a date occupies the interval from its left edge to the next date's left edge. A period boundary is drawn at the left edge of its `start` date, which is also the right edge of the previous period's `last_day`. Every boundary is then exactly one line, and tiling without gaps or overlaps is visible.
2. **Linear scale, no truncation.** Width is proportional to days, with the same pixels per day everywhere in one diagram and in every panel of a set. No axis break, no omitted weeks, no rounding of a period to a whole number of pixels other than the per-day scale itself.
3. **Closed start, open end.** Where a span's bounds are marked, `start` gets a filled marker and `end_exclusive` an open marker, both on the boundary line. The open marker sits at the left edge of the first excluded date. `last_day` is never drawn as a boundary. When it is shown, it labels the last cell inside the span and is marked "derived".
4. **Dates.** Full dates are ISO `YYYY-MM-DD` with tabular figures (`font-variant-numeric: tabular-nums`). Inside grids, a day-of-month number may stand alone, but the first day of each calendar month carries its month name, which avoids NRF's `29 30 31 1` ambiguity. Fiscal labels follow the definition's templates, for example `FY2023 Q4`. A week-based year's label always carries its length in weeks.
5. **Hierarchy without colour.** Quarters are outlined boxes or brackets with heavier boundary lines. Periods alternate between two light fills, with thin separators and a text label. Weeks are short ticks. The extra week is hatched, outlined and labelled. Every distinction has a non-colour channel (line weight, outline, hatch or text), so a greyscale print still reads correctly.
6. **Direct labels.** Labels sit on or beside what they name. A legend is used only for the hatch, and even then the hatched element is also labelled in place.
7. **Minimum sizes (proposed).** Text at least 11 px at rendered size, 10 px for secondary notes. At least 2 px per day for a full year strip, or 742 px for a 371-day year. At least 20 px square for a week-grid day cell. Week ticks at least 6 px tall. Markers at least 8 px across. Below a strip's minimum width, the page shows the week grid instead of shrinking the strip.
8. **Alt text pattern.** `title`: "<calendar> fiscal year <n>: <days> days, <weeks> weeks". `desc`: the year's `start` (included), `end_exclusive` (excluded) and `last_day`, then each quarter's dates and weeks, the period lengths in order and the extra week's dates and period. The surrounding page also carries the same numbers as a visible table under the figure, following the W3C long-description guidance.
9. **Every date is checked.** A diagram that shows dates names its definition and the source or independent generator its dates came from (see [How to produce them](#how-to-produce-them)).

### Colour roles

Final values come from the design system and must meet 1.4.11 against `surface` in both themes. Hex values in the sample SVG are placeholders.

| Role | Used for | Non-colour partner |
| --- | --- | --- |
| `surface` | Diagram background | none |
| `ink` | Labels, primary text | none |
| `ink-muted` | Secondary text, week ticks | Smaller size |
| `rule` | Period separators, grid lines | Thin line |
| `quarter-rule` | Quarter outlines and boundaries | Heavier, taller line |
| `period-fill-a`, `period-fill-b` | Alternating period backgrounds | Separators and labels |
| `extra-week` | Extra-week outline and hatch | Hatch pattern and "week 53" label |
| `boundary` | Start and end markers, axis line | Filled versus open marker |
| `calendar-b` | Second calendar in a comparison | Dashed separators and row label |
| `today-marker` | A caller-supplied reference date, if a diagram shows one | Labelled vertical line with its ISO date |

`today-marker` must never imply a clock. fiscalclock reads none, so any such date is an explicit input and its label prints the date.

### D1 year strip

- **Answers.** How a fiscal year divides into quarters, periods and weeks, and how long each is.
- **Layout.** One horizontal band per level, top to bottom: quarters, periods (with week ticks along the bottom edge), then a boundary axis with the year's `start` and `end_exclusive` markers. Quarter start dates are labelled under the axis.
- **Scale.** Proportional to days (rule 2). A 371-day year is 7 days longer than a 364-day year at the same scale.
- **Distinguishing.** Rule 5. Period labels give the index and weeks, for example `P12` and `4 + 1 wk` when the period holds the extra week. Quarter labels give the index, weeks and days.
- **Labels.** Inside the bands where they fit. Dates go below the axis, never rotated.
- **Minimum size.** 2 px per day, so 742 px of drawing width for 371 days.
- **Alt text.** Rule 8.
- **Colour roles.** `period-fill-a/b`, `quarter-rule`, `rule`, `ink-muted` (ticks), `extra-week`, `boundary`.

### D2 week grid

- **Answers.** Which dates fall in which week and period. Supports lookup ("which period is 2023-05-30 in?") and counting weeks.
- **Layout.** One row per week, seven day columns starting on `week.starts_on`. A left column holds the week number, rows are grouped into period blocks with a period label, and a heavier rule separates quarters. This follows the NRF layout. Several years can sit side by side as small multiples aligned on week number.
- **Scale.** Equal cell size. Weeks are equal by definition in week-based years. For a `fixed_date` year, whose first and last weeks can have 1-7 days under `contains_start`, the missing days are empty cells with a dotted outline.
- **Distinguishing.** Period blocks separated by gaps or rules and labelled. The extra-week row is hatched and labelled with its week number (53). The first day of each calendar month is bold and carries the month name (rule 4).
- **Minimum size.** 20 px cells, so about 140 px of width per year plus label columns. This fits a phone screen for one year.
- **Alt text.** As rule 8. Because the grid is long, the visible companion is a table of period `start`, `end_exclusive` and weeks, not a list of every date.
- **Colour roles.** `rule`, `quarter-rule`, `extra-week`, `ink`, `ink-muted`.

### D3 boundary detail

- **Answers.** What `start`, `end_exclusive` and `last_day` mean and how they relate.
- **Layout.** A zoomed day axis of about two weeks around one boundary, each day a labelled cell (weekday initial and day of month, with full ISO dates under the cells that matter). It shows the closed marker at `start`, the open marker at `end_exclusive`, the `last_day` cell bracketed and labelled "derived: end_exclusive − 1 day", and the bracket notation printed beside it, for example `[2023-01-29, 2024-02-04)`.
- **Scale.** Equal day cells; at this zoom, at least 32 px per day.
- **Distinguishing.** Filled versus open marker, plus the words "included" and "excluded" under them.
- **Alt text.** "Boundary between <period a> and <period b>: <period a> includes up to and including <last_day>; <period b> starts on <start>, which equals <period a>'s end_exclusive."
- **Colour roles.** `boundary`, `ink`, `rule`.

### D4 52 vs 53-week comparison

- **Answers.** Where the extra week goes and what it does to the following periods and quarters.
- **Layout.** Two D1 strips, a 52-week year above a 53-week year of the same definition, stacked as small multiples with week 1 aligned at the left and the same pixels per day. Periods before the extra-week period line up exactly. The extra week extends the 53-week row by one week, and a dashed vertical guide at the 52-week year's end shows the overhang. Each row label carries its week count.
- **Scale.** Shared, proportional to days. The rows are aligned by fiscal week, not by calendar date, and the figure says so.
- **Distinguishing.** The extra week is hatched and labelled. The overhang guide is dashed.
- **Restatement.** NRF also publishes a restated 2023 layout ([see above](#nrf-4-5-4-calendar)). If a diagram shows a restated year, it is a third row labelled as restated. It is never drawn in place of the actual year.
- **Alt text.** "Two fiscal years of <calendar> on the same scale: <year a>, 52 weeks, <start> to <last_day>; <year b>, 53 weeks, <start> to <last_day>. Periods 1 to <k − 1> have equal lengths; period <k> has one more week in <year b>."
- **Colour roles.** As D1.

### D5 two-calendar comparison

- **Answers.** How a fixed-date calendar and a week-based calendar differ over the same dates. For example, a 1 February fixed-date year with calendar months against NRF 4-5-4 (illustrative pairing).
- **Layout.** A shared day axis of absolute dates, with calendar-month ticks. The upper row shows calendar A's periods and the lower row calendar B's. Thin vertical guides drop from each boundary of one row to the other, and the offset between paired boundaries is labelled in days (for example `+3 d`).
- **Scale.** Shared, proportional to days. The axis spans the union of both years.
- **Distinguishing.** Row labels name each calendar. Calendar B's separators are dashed, and its fill uses `calendar-b`.
- **Alt text.** Lists both years' bounds and the per-boundary offsets.
- **Colour roles.** `period-fill-a/b` (calendar A), `calendar-b`, `rule`, `boundary`.

### D6 multi-year drift

- **Answers.** How a week-based year end moves around its anchor date, and which years have 53 weeks.
- **Layout.** A dot plot. Rows are fiscal years in order and the horizontal axis is the year end's offset in days from the anchor date. The axis runs from −3 to +3 for `nearest` and from −6 to 0 for `last` or 0 to +6 for `first`. A vertical line marks the anchor at 0, and thin lines join consecutive years to show the sawtooth. A 53-week year is a ring with a "53" label, a 52-week year a filled dot. Each row is also labelled with the full `last_day` date.
- **Scale.** One day per tick, the same for every row. The offset axis never omits days, even where no year lands.
- **Example (computed).** For "Saturday nearest 31 January" in fiscal years 2015-2030, the offsets are −1, −3, +3, +2, +1, −1, −2, −3, +3, +1, 0, −1, −2, +3, +2, +1 days. The 53-week years are 2017, 2023 and 2028, each one where the offset jumps from −3 to +3. This matches the NRF years in the evidence register (checks C03 and C19 in [`Evidence.md`](../../research/evidence/Evidence.md)), and Kroger's 10-K above gives the same fiscal 2017 and 2018 year-end dates. Reproduced by `drift_check.py` in the scratch folder (see [Sample](#sample-nrf-fiscal-2023-year-strip)).
- **Alt text.** "Year-end dates of <calendar>, <first>–<last>, as days from <anchor>: <year> <offset> …; 53-week years: <list>."
- **Colour roles.** `ink`, `extra-week` for the 53-week rings and labels, `rule`.

## How to produce them

**Recommendation: generate every diagram that shows dates from a standard-library Python script, and draw only the purely conceptual ones by hand.**

- **Data source.** The project rule is that expected values never come from running fiscalclock ([`AGENTS.md`](../../AGENTS.md), correctness workflow). A diagram is a claim about dates, so the same rule applies. Its numbers should come from [`conformance/generators/fiscal_oracle.py`](../../conformance/generators/fiscal_oracle.py), which describes itself as written from the spec without reference to the fiscalclock sources, or from a cited vector. Each diagram should also check its numbers against a cited source where one exists, as the sample below does against NRF's PDF.
- **Reproducibility.** Scripts and their SVG output are committed, as `research/verification/` already does for prose claims. CI reruns the scripts and fails on any difference. Each SVG carries a comment naming its definition, its data source and the script that drew it.
- **Hand-drawn SVG** is suitable only for diagrams with no dates, such as the block structure of a definition. It still follows the shared rules and colour roles.

**Light and dark.**

1. **Inline SVG on the docs site.** Colour roles become CSS custom properties defined by the site's tokens, so the diagram follows the page theme with no media query of its own.
2. **Standalone `.svg` file loaded with `<img>`.** Put the roles in an internal `<style>` with a `prefers-color-scheme: dark` override, as the sample does. MDN's [`prefers-color-scheme`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme) page says an SVG embedded this way follows the colour scheme of the embedding element, and shows this pattern.
3. **GitHub README and other GitHub Markdown.** Publish two files and switch with `<picture>` and `<source media="(prefers-color-scheme: dark)">`, as described on the [GitHub blog](https://github.blog/developer-skills/github/how-to-make-your-images-in-markdown-on-github-adjust-for-dark-mode-and-light-mode/). GitHub's [formatting syntax page](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax) confirms that `<picture>` is supported. The same script writes both files from one set of roles. Whether GitHub honours `prefers-color-scheme` inside an SVG shown through `<img>` was not checked, so the two-file method is the safe choice there.

## Sample: NRF fiscal 2023 year strip

A rough D1 year strip for NRF 4-5-4 fiscal 2023 (a 53-week year), built to test the grammar. It is illustrative, not a docs asset. The files are in the session scratch folder, not in the repository:

- `year_strip_nrf_2023.py`: standard library only, no fiscalclock import. It applies NRF's rule as stated in the README example definition: the year ends on the Saturday nearest 31 January, weeks start on Sunday, periods follow 4-5-4 and the extra week goes in period 12. It then compares the result with dates transcribed from NRF's 2023-2025 (not restated) PDF.
- `year-strip-nrf-2023.svg`: the output, 790 × 168 px at 2 px per day, with `role="img"`, `title`, `desc`, a hatch pattern for week 53 and a `prefers-color-scheme` override.
- `drift_check.py`: the year-end offsets used in [D6](#d6-multi-year-drift).

Run as `python -I year_strip_nrf_2023.py year-strip-nrf-2023.svg nrf-2023-2025.txt`, where the text file is `pdftotext -layout` output of the NRF PDF. The script exits non-zero on any mismatch.

**Result.** Fiscal 2023 is `[2023-01-29, 2024-02-04)`, `last_day` 2024-02-03, 371 days. Quarters are 13, 13, 13 and 14 weeks (Q4 is `[2023-10-29, 2024-02-04)`, 98 days). Period 12 is `[2023-12-31, 2024-02-04)` with week 53 `[2024-01-28, 2024-02-04)`.

**Verification against NRF.** NRF's [2023-2025 PDF](https://6a83cd4f6d8a17c1b6dd-0490b3ba35823e24e2c50ce7533598b0.ssl.cf1.rackcdn.com/454%20Calendars/3%20Year%20Calendar%2023-25.pdf) was fetched and converted with `pdftotext -layout`. The first Sunday of all 12 fiscal months, week 53 (row `28 29 30 31 1 2 3`) and the first day of fiscal 2024 were transcribed from it, and all 14 dates matched. All 53 computed week rows' day-of-month runs were also found in the extracted text, a weaker check because the runs are not tied to row positions. The year bounds agree with checks C04 and C05 in [`Evidence.md`](../../research/evidence/Evidence.md).

**Rendering.** Headless Chrome screenshots of the SVG in light and dark were readable in both themes. The colour values are placeholders. Contrast ratios were not measured.

**Lessons from the sample.**

- At 2 px per day, a 4-week period is 56 px wide: enough for `P10` and `4 wk`, and too narrow for a date. Period dates therefore belong in the companion table or the week grid, not in the strip.
- Five quarter-boundary dates fit under a 742 px axis without rotation. Thirteen period-boundary dates would not.
- Labelling period 12 as `4 + 1 wk` reads better than `5 wk`, which looks like a pattern error in a 4-5-4 year.

## Open questions and unverified points

- Filled and open endpoint circles as a convention were not confirmed in any fetched teaching source (see [Guidance](#guidance-on-drawing-intervals-precisely)). Test them with readers, and keep the words "included" and "excluded" beside the markers either way.
- How GitHub renders `prefers-color-scheme` inside an `<img>` SVG, and whether system fonts in such an SVG match the page, were not tested.
- The NRF PDFs were read as extracted text only. Their colours, line weights and the exact position of the 53rd-week row were not seen.
- Minimum sizes and the 2 px per day scale are proposals, not tested with readers or on small screens.
- The Okabe-Ito colour values are shown only as an image on the cited page, so this note gives no values. Final palette values are left to the design-system track.
