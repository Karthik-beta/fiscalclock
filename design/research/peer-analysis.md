# Peer analysis: eleven developer documentation sites

<!-- cspell:words Tokio Polars Pydantic DuckDB Deno Resend Vercel Geist Mintlify MkDocs favicon favicons wordmark Roboto Menlo Consolas Astral Ruff dataframe Proxima Diatype Domaine Favorit pnpm Anthropic Claude Inkscape cURL monospace pola stylesheets callout -->

This note records what eleven documentation sites look like and how they behave, as input to fiscalclock's visual identity. Five are the sites the owner admires (DD-004 in [`../DECISIONS.md`](../DECISIONS.md)): Stripe Docs, Claude Docs, Linear, Vercel (with its Geist design system) and Resend. Six are open-source developer or data tools: uv (Astral), Polars, DuckDB, Tokio, Deno and Pydantic.

Every value below was observed on 1 Nov 2026 unless marked **inferred**. Sites change often; treat each value as "as of 1 Nov 2026".

## Method

- Each page was loaded in Chromium through Playwright at a 1440 × 900 viewport, once with `prefers-color-scheme: light` and once with `dark` emulated. Colours, font stacks, sizes and widths are `getComputedStyle` values from the rendered page, converted from `rgb()` to hex. Values the site writes as `oklch()` or `hsla()` are quoted as written.
- Where emulation did not change the theme, the site's own theme toggle was clicked and the values read again. Those rows say "toggle".
- "Text column" is the rendered width of the first long paragraph in the article. "Nav" and "TOC" are the widths of the left navigation and right table of contents.
- Favicons were loaded from each site's `<link rel="icon">` and rendered at 16, 32 and 64 px on white and on near-black, and the SVG sources were read.
- "Tabular figures" means any element on the page whose computed `font-variant-numeric` includes `tabular-nums`.
- Screenshots were kept outside the repository. Nothing here was taken from memory; where a check was not possible the note says so.

Pages sampled:

| Site | Landing or home | Article pages sampled |
| --- | --- | --- |
| Stripe Docs | [docs.stripe.com](https://docs.stripe.com/) | [Accept a payment](https://docs.stripe.com/payments/accept-a-payment) |
| Claude Docs | [platform.claude.com/docs](https://platform.claude.com/docs/en/home) | [Intro to Claude](https://platform.claude.com/docs/en/intro), [Get started](https://platform.claude.com/docs/en/get-started) |
| Linear | [linear.app](https://linear.app/) | [Linear Docs](https://linear.app/docs), [Creating issues](https://linear.app/docs/creating-issues), [GraphQL API](https://linear.app/developers/graphql) |
| Vercel | [vercel.com](https://vercel.com/) | [Docs](https://vercel.com/docs), [CLI overview](https://vercel.com/docs/cli), [Geist introduction](https://vercel.com/geist/introduction), [Geist colours](https://vercel.com/geist/colors), [Geist typography](https://vercel.com/geist/typography) |
| Resend | [resend.com](https://resend.com/) | [Introduction](https://resend.com/docs/introduction), [Send Email API](https://resend.com/docs/api-reference/emails/send-email) |
| uv (Astral) | [docs.astral.sh/uv](https://docs.astral.sh/uv/) | [Installation](https://docs.astral.sh/uv/getting-started/installation/) |
| Polars | [pola.rs](https://pola.rs/) | [User guide](https://docs.pola.rs/), [Getting started](https://docs.pola.rs/user-guide/getting-started/) |
| DuckDB | [duckdb.org](https://duckdb.org/) | [Python API](https://duckdb.org/docs/stable/clients/python/overview) |
| Tokio | [tokio.rs](https://tokio.rs/) | [Hello Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) |
| Deno | [deno.com](https://deno.com/) | [Runtime docs](https://docs.deno.com/runtime/), [TypeScript](https://docs.deno.com/runtime/fundamentals/typescript/) |
| Pydantic | [pydantic.dev](https://pydantic.dev/) | [Pydantic Validation docs](https://pydantic.dev/docs/validation/latest/get-started/) |

Three addresses moved as of 1 Nov 2026: `docs.claude.com/en/docs/intro` redirects to [platform.claude.com/docs/en/intro](https://platform.claude.com/docs/en/intro); `docs.pydantic.dev/latest/` redirects to [pydantic.dev/docs/validation/latest/get-started/](https://pydantic.dev/docs/validation/latest/get-started/); and DuckDB's `/docs/stable/` path resolves to `/docs/2.0/`, shown as "2.0 LTS" in its version selector ([DuckDB Python API](https://duckdb.org/docs/stable/clients/python/overview)).

## Colour

Body text is the colour of the first long article paragraph; headings are sometimes darker. "Link" is the computed colour of an inline link in body text.

| Site | How dark mode is chosen | Page background (light / dark) | Body text (light / dark) | Link (light / dark) | Code block background (light / dark) |
| --- | --- | --- | --- | --- | --- |
| [Stripe Docs](https://docs.stripe.com/payments/accept-a-payment) | No dark theme found: emulated dark changed nothing and no theme control was found | `#ffffff` / none | `#3c4257`; `h1` `#1a1f36` | `#5469d4`, not underlined | `#212d63` (dark navy panel even in light) |
| [Claude Docs](https://platform.claude.com/docs/en/get-started) | Follows system; control labelled "Color theme: System" | `#fcfcfb` / `#151515` | `#52514e` / `#c3c2b7`; `h1` `#0b0b0b` / `#f0efec` | `#184f95` / `#6da7ec`, underlined | `#ffffff` / `#1a1a19` |
| [Linear](https://linear.app/developers/graphql) | Marketing site and Linear Docs render dark under both emulated preferences; the developer docs have a "Toggle theme" control | `#ffffff` (toggle) / `#08090a` | `#3c4149` / `#d0d6e0`; marketing body `#8a8f98` on `#08090a` | Same colour as heading text (`#282a30` / `#f7f8f8`) | `#f9f8f9` / `#1c1c1f` |
| [Vercel Docs](https://vercel.com/docs/cli) | Follows system; light, dark and system controls | `#fafafa` / `#000000` | `#171717` / `#ededed`; secondary `#4d4d4d` / `#a1a1a1` | Same as body text, underlined | `#ffffff` / `#0a0a0a` |
| [Resend Docs](https://resend.com/docs/api-reference/emails/send-email) | Docs follow system with system, light and dark controls; [marketing home](https://resend.com/) is black under both | `#ffffff` / `#000000` | `#3e3e3e` / `#9e9e9e`; `h1` `#161616` / `#dedede` | none found in sampled paragraphs | `#ffffff` / `#101010` |
| [uv](https://docs.astral.sh/uv/getting-started/installation/) | Follows system; light and dark toggle | `#ffffff` / `#261230` (deep purple) | black at 87 % opacity / `#ffffff` | `#6340ac` / `#d7ff64` (lime) | same as page / same as page |
| [Polars](https://docs.pola.rs/user-guide/getting-started/) | Light by default whatever the system says; toggle switches to dark | `#ffffff` / `#0b0c0f` (toggle) | black at 87 % / `rgba(226, 228, 233, 0.82)` | `#0075ff` in both, underlined | same as page / same as page |
| [DuckDB](https://duckdb.org/docs/stable/clients/python/overview) | Follows system (adds a `darkmode` class); toggle | `#fcfcfc` / `#0d0d0d` | `#0d0d0d` / `#f2f2f2` | Same as body text, underlined | `#f2f2f2` / `#1a1a1a` |
| [Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) | No dark theme: no dark rules found in readable stylesheets and no toggle | `#ffffff`, sidebar `#f0f0f0` / none | `#4a4a4a`; headings `#363636` | `#c83895` (magenta), not underlined | `#f5f5f5` / none |
| [Deno](https://docs.deno.com/runtime/fundamentals/typescript/) | Light by default whatever the system says; toggle | `#fdfeff` / `#14161a` (toggle) | `#383a3e` / `#e5eaea` | `#0077cc` / `#70ffaf`, underlined | `#f6f8fb` / `#191b1f` |
| [Pydantic](https://pydantic.dev/docs/validation/latest/get-started/) | Follows system; toggle | `#ffffff` / `oklch(0.21 0.006 285.885)` | `oklch(0.274 0.006 286.033)` / `oklch(0.92 0.004 286.32)` | `oklch(0.446 0.1 257)` / `oklch(0.929 0.035 255)`, underlined | `#f6f8fa` / `#0d1117` |

How much accent colour each site uses (from the screenshots):

- **Almost none.** Vercel and DuckDB links are the body text colour with an underline. Linear's header and hero are monochrome; a `#5e6ad2` indigo button background was found on the home page. Claude Docs keeps its coral to the logo mark and favicon; links are a muted blue and everything else is warm grey.
- **One working accent.** Stripe Docs uses `#5469d4` indigo for links, the active nav item and the TOC. Polars uses `#0075ff` for links and the active nav item. Deno and Pydantic use blue links. Resend Docs adds small coloured method badges (POST, GET, PATCH) and a red "required" label in its API reference.
- **Strong accent.** uv's dark theme pairs a deep purple page with lime links. Tokio uses magenta links. Pydantic's logo and active top-nav item are magenta.

Vercel publishes its scales as CSS custom properties ([Geist colours](https://vercel.com/geist/colors)). Read from the page: `--ds-background-100` is `hsla(0, 0%, 100%, 1)` in light and `hsla(0, 0%, 4%, 1)` in dark; `--ds-gray-1000` is `hsla(0, 0%, 9%, 1)` and `hsla(0, 0%, 93%, 1)`; `--ds-blue-700` stays `hsla(212, 100%, 48%, 1)` in both. Section headings on that page include "Choose gray or gray-alpha" and "Draw each border once".

## Typography

Font names are the first family in the computed stack. "Loaded" lists the web fonts the browser reported as loaded.

| Site | UI and body | Headings | Code | Body size / line height | Tabular figures |
| --- | --- | --- | --- | --- | --- |
| [Stripe Docs](https://docs.stripe.com/payments/accept-a-payment) | System stack (`-apple-system`, `Segoe UI`, `Roboto` …) | Same, 32 px, weight 700 | `Menlo, Consolas, monospace` | 16 / 26 px | None found |
| [Claude Docs](https://platform.claude.com/docs/en/get-started) | `anthropicSans` (loaded) | `anthropicSerif`, weight 520 | `anthropicMono` | 15 / 22 px (article); 16.875 / 25.3 px (intro) | None found |
| [Linear](https://linear.app/) | `Inter Variable` with `font-feature-settings: "cv01", "ss03"` | Same, 64 px weight 510, letter-spacing −1.408 px (home) | `Berkeley Mono` | 15 / 24 px | None found |
| [Vercel](https://vercel.com/docs/cli) | `GeistSans` | Same, weight 600, letter-spacing −3.36 px at 56 px | `Geist Mono` (also used for some marketing paragraphs) | 16 / 27.2 px | Only on "New" badges |
| [Resend](https://resend.com/) | `inter`; docs use `Inter` with `cv02`, `cv03`, `cv04`, `cv11` | Home: `domaine` serif display at 96 px; docs: Inter 36 px weight 600 | Home: `commitMono`; docs: `Paper Mono` | 18 / 28 px (docs intro) | None found |
| [uv](https://docs.astral.sh/uv/) | `Roboto` | `Roboto`, 32 px weight 300 | `Roboto Mono` | 16 / 25.6 px | None found |
| [Polars](https://docs.pola.rs/user-guide/getting-started/) | Stack begins `Proxima Nova`, but no web font was reported as loaded | Same, weight 700 | `Source Code Pro` | 15 / 27 px | None found |
| [DuckDB](https://duckdb.org/docs/stable/clients/python/overview) | `SuisseIntl` | Same, 34 px weight 600 | `JetBrainsMono-Regular` | 15 / 24.75 px | None found |
| [Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) | `Roboto` | `Roboto`, weight 400 | `monospace` (browser default) | about 18 / 27 px | None found |
| [Deno](https://docs.deno.com/runtime/fundamentals/typescript/) | `Inter` | `Inter`, 36 px weight 700 | `ui-monospace` system stack | 16 / 24 px | None found |
| [Pydantic](https://pydantic.dev/docs/validation/latest/get-started/) | `ABC Diatype` | `Dwight`, 42 px weight 500 | `DejaVu Sans Mono` | 16 / 28 px | None found |

Notes:

- Two admired sites pair a serif display face with a sans body: Claude Docs (serif headings throughout the docs) and Resend (serif hero on the marketing page only).
- **Inferred:** Polars probably renders a fallback sans, because its first-choice family was not among the loaded fonts.
- Licences matter because DD-010 allows only SIL Open Font Licence faces. Geist ([repository](https://github.com/vercel/geist-font)), Inter ([repository](https://github.com/rsms/inter)) and JetBrains Mono ([repository](https://github.com/JetBrains/JetBrainsMono)) are each published under OFL 1.1. Inter's README lists tabular numbers among its OpenType features ([repository](https://github.com/rsms/inter)). The licences of the other faces above were not checked; treat them as unavailable until checked.

## Layout and density

| Site | Text column | Left nav | TOC | Density (from screenshots) |
| --- | --- | --- | --- | --- |
| [Stripe Docs](https://docs.stripe.com/payments/accept-a-payment) | 799 px | present (width not isolated) | present | Medium; top product tabs plus left nav, TOC with "Optional" tags on steps |
| [Claude Docs](https://platform.claude.com/docs/en/get-started) | 788 px | 264 px | present | Medium; numbered steps with a vertical rule |
| [Linear Docs](https://linear.app/docs/creating-issues) | 650 px | 280 px | 250 px | Airy |
| [Vercel Docs](https://vercel.com/docs/cli) | 768 px | 288–300 px | 240 px | Medium-dense; long TOC |
| [Resend Docs](https://resend.com/docs/api-reference/emails/send-email) | 576 px on the introduction | 264 px | API pages swap the TOC for a sticky code column | Medium |
| [uv](https://docs.astral.sh/uv/getting-started/installation/) | 688 px | 224 px | 230 px | Medium |
| [Polars](https://docs.pola.rs/user-guide/getting-started/) | 688 px | 230 px | 230 px | Medium |
| [DuckDB](https://duckdb.org/docs/stable/clients/python/overview) | 862 px | 249 px | present | Dense nav, generous text |
| [Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) | 656 px | 280 px | 340 px | Airy, large body text |
| [Deno](https://docs.deno.com/runtime/fundamentals/typescript/) | 768 px | 256 px | present | Medium |
| [Pydantic](https://pydantic.dev/docs/validation/latest/get-started/) | 752 px | present | 198–240 px | Medium |

Every site uses the same three-column layout: left navigation, a text column of 650–860 px, and an "On this page" list on the right. uv and Polars measure identically (688 px text, 224–230 px side columns). **Inferred:** both use the same off-the-shelf theme, Material for MkDocs, because their markup carries its `data-md-color-scheme` attribute and `js-focus-visible` class. Resend's docs load assets from a `mintlify-assets` path, so they are **inferred** to be built on Mintlify.

## Code blocks and multi-language examples

| Site | Copy button | Language or variant tabs | How multi-language examples are shown |
| --- | --- | --- | --- |
| [Stripe Docs](https://docs.stripe.com/payments/accept-a-payment) | Yes | Integration-type tabs ("Checkout", "Elements", "Mobile") above the content | Dark navy panels in a light page; integration choice changes the whole guide |
| [Claude Docs](https://platform.claude.com/docs/en/get-started) | Yes | One selector for the whole page: cURL, CLI, Python, TypeScript, C#, Go, Java, PHP, Ruby | A single page-level language switch; code blocks carry a filename or language header |
| [Linear](https://linear.app/developers/graphql) | Yes | none found | Single-language GraphQL examples |
| [Vercel](https://vercel.com/docs/cli) | Yes | Package-manager tabs: pnpm, yarn, npm, bun | Block has a "Terminal" header strip with the copy icon; bottom corners rounded 8 px |
| [Resend](https://resend.com/docs/api-reference/emails/send-email) | Yes (icon in panel header) | Node.js, PHP, Python, Ruby, Go, Rust, Java, .NET | Sticky right-hand request panel with a separate "Response" panel below it |
| [uv](https://docs.astral.sh/uv/getting-started/installation/) | Yes | OS and shell tabs ("macOS and Linux", "Windows"; Bash, Zsh, fish …) | Tabs per block |
| [Polars](https://docs.pola.rs/user-guide/getting-started/) | Yes | Python and Rust on every block | Tabs per block, each with a small link to the API reference entry used |
| [DuckDB](https://duckdb.org/docs/stable/clients/python/overview) | Yes | none on sampled page | Rounded 10 px grey block |
| [Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) | No | none | Plain grey block, shell lines start with `$` |
| [Deno](https://docs.deno.com/runtime/fundamentals/typescript/) | Yes | none on sampled page | Pale blue-grey block |
| [Pydantic](https://pydantic.dev/docs/validation/latest/get-started/) | Yes | none on sampled page | GitHub-like palette (`#f6f8fa` / `#0d1117`) |

Line highlighting was not found on any sampled page; the check looked for common highlight classes only, so it may exist elsewhere on these sites. Vercel's [Geist typography](https://vercel.com/geist/typography) page shows line numbers on its code sample.

## Logo and favicon

| Site | Logo kind and idea | Favicon behaviour at 16 px |
| --- | --- | --- |
| [Stripe Docs](https://docs.stripe.com/payments/accept-a-payment) | Wordmark "stripe" plus a "DOCS" label in the header | Orange tile with radial gradients and a white slanted parallelogram; reads at 16 px |
| [Claude Docs](https://platform.claude.com/docs/en/home) | Coral radiating mark plus "Claude Platform Docs" in the serif | Coral mark on a `#30302E` rounded tile; works on light and dark |
| [Linear](https://linear.app/) | Symbol plus wordmark; circle cut by diagonal lines | Black rounded tile with the white mark; diagonal lines merge into a grey disc at 16 px |
| [Vercel](https://vercel.com/) | Triangle symbol, with "Docs" or "Geist Design System" labels beside it | White triangle in a black circle; the clearest of the eleven at 16 px |
| [Resend](https://resend.com/) | Wordmark; favicon is the stylised "R" alone | Letterform; legible |
| [uv](https://docs.astral.sh/uv/) | Square symbol plus lowercase "uv" | Dark purple square with a lime frame; high contrast |
| [Polars](https://pola.rs/) | Pixel-built polar bear plus wordmark | Blue bear becomes a smudge at 16 px; the SVG switches the fill to white under `prefers-color-scheme: dark` |
| [DuckDB](https://duckdb.org/) | Yellow circle with a smaller dot, plus wordmark | Yellow on black; very legible |
| [Tokio](https://tokio.rs/) | Line-drawn radiating star plus wordmark | Thin black strokes; the SVG contains a `prefers-color-scheme: dark` rule that changes the fill |
| [Deno](https://deno.com/) | Not checked | The `.ico` file did not render in the test; not checked |
| [Pydantic](https://pydantic.dev/) | Magenta triangular mark plus wordmark | Magenta outline triangle; legible |

Two cheap techniques stand out: a filled tile behind the mark (Claude, Linear, uv, Stripe), which works on any tab-bar colour, and an SVG favicon with an internal `prefers-color-scheme` rule (Tokio, Polars). Both are verified from the SVG sources served at the favicon URLs on the pages above.

## Tone

Headline and subhead from each site's landing page (verbatim, shortened where marked):

| Site | Headline | Supporting line or docs voice |
| --- | --- | --- |
| [Stripe Docs](https://docs.stripe.com/) | "Documentation" | Guides are task-named ("Accept a payment"); steps tagged "Optional" in the TOC |
| [Claude Docs](https://platform.claude.com/docs/en/home) | "Start building with Claude" | "From first API call to production." |
| [Linear](https://linear.app/) | "The product development system for teams and agents" | "Designed for the AI era." |
| [Vercel](https://vercel.com/) | "Agentic Infrastructure" | Docs: plain, second person, task-led |
| [Resend](https://resend.com/) | "Email for developers" | "The best way to reach humans instead of spam folders." |
| [uv](https://docs.astral.sh/uv/) | "uv" | "An extremely fast Python package and project manager, written in Rust." |
| [Polars](https://pola.rs/) | "DataFrames for the new era" | Docs voice: plain, British spelling ("familiarise") on the getting-started page |
| [DuckDB](https://duckdb.org/) | "Your universal data wrangling tool" | Factual; states the minimum Python version and what is not supported |
| [Tokio](https://tokio.rs/) | "Build reliable network applications without compromising speed." | Tutorial voice, first person plural ("We will get started …") |
| [Deno](https://deno.com/) | "Better, faster JavaScript" | Docs: plain, task-led |
| [Pydantic](https://pydantic.dev/docs/validation/latest/get-started/) | Docs page opens with "Documentation for version: v2.14.2." | Then a superlative: "the most widely used data validation library for Python" |

Observations: the docs are calmer than the marketing pages on every site. uv is the one site that backs a speed claim on the page itself, with a benchmark chart and a caption naming the test conditions ([uv](https://docs.astral.sh/uv/)). Two of the five admired sites (Linear, Vercel) lead with AI or agents; Claude Docs documents an AI product.

## Trust signals

| Site | Changelog or release notes | Status page | Version shown | Edit this page | Last updated | Machine-readable docs |
| --- | --- | --- | --- | --- | --- | --- |
| [Stripe Docs](https://docs.stripe.com/payments/accept-a-payment) | Link to a changelog | none found on page | none found | none found | none found; "Was this page helpful?" | "Copy for LLM", "View as Markdown", link to `llms.txt` |
| [Claude Docs](https://platform.claude.com/docs/en/get-started) | "Release notes" link | "Status" link to a status site | none found | none found | none found; "Was this page helpful?" | "Copy page" menu |
| [Linear](https://linear.app/developers/graphql) | none found on sampled page | none found | none found | none found | none found | none found |
| [Vercel](https://vercel.com/docs/cli) | "Changelog" and CLI "Release Notes" | "Status" link with live state | none found | none found | "Last updated October 7, 2026" | "Copy page" menu |
| [Resend](https://resend.com/docs/api-reference/emails/send-email) | none found on sampled page | none found | none found | none found | "Was this page helpful?" | `llms.txt` link |
| [uv](https://docs.astral.sh/uv/getting-started/installation/) | none found on sampled page | none | Release `0.13.6` and star count in the header repository widget | none found | none found | none found |
| [Polars](https://docs.pola.rs/user-guide/getting-started/) | "Changelog" in nav | none | `py-2.1.0` in the header repository widget | none found | none found | none found |
| [DuckDB](https://duckdb.org/docs/stable/clients/python/overview) | none found on sampled page | none | "2.0 LTS" selector; callout states the latest stable client version | "Edit on GitHub" | none found | "Copy Markdown" and "View Markdown" |
| [Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) | none found | none | none | "Edit this page" | none found | none found |
| [Deno](https://docs.deno.com/runtime/fundamentals/typescript/) | none found on sampled page | "Deploy System Status" link | none found | "Edit this page" | none found | "View Page as Markdown", `llms.txt` |
| [Pydantic](https://pydantic.dev/docs/validation/latest/get-started/) | "Changelog" in nav | none | "Version: Latest (stable)" selector; CI, coverage, PyPI, conda, licence badges | none found | "Was this page helpful?" | `llms.txt` badge |

Five of the eleven expose a Markdown view or an `llms.txt` file for agents (Stripe, Resend, DuckDB, Deno, Pydantic), and two more offer a "Copy page" menu (Claude, Vercel). The open-source sites lean on repository facts (edit links, version selectors, release widgets, CI badges); the company sites lean on changelogs and status pages.

## Synthesis

### Patterns to adopt

What the admired sites share, confirmed against the tables above:

1. **Neutral surfaces, one restrained accent.** Vercel and Linear are effectively monochrome; Claude keeps its brand colour to the mark; Stripe uses one indigo. Links are either the text colour with an underline (Vercel, DuckDB) or a single muted blue (Claude).
2. **Dark mode that follows the system, with an explicit override.** Claude, Vercel, Resend Docs, uv, DuckDB and Pydantic follow `prefers-color-scheme` and offer a toggle; Claude, Vercel and Resend Docs offer a "system" choice alongside light and dark.
3. **Near-black and off-white, not pure extremes for text.** Claude (`#fcfcfb` and `#151515`), Vercel (`#fafafa` and `#171717`) and DuckDB (`#fcfcfc` and `#0d0d0d`) avoid pure white text on pure black or the reverse for body copy.
4. **A three-column docs layout with a 650–860 px text column**, a 224–300 px left nav and a right-hand "On this page" list.
5. **One open sans for UI and body, one monospace for code.** Inter and Geist (both OFL 1.1) cover the sans; JetBrains Mono (OFL 1.1) is used by DuckDB. A serif for headings is optional: Claude uses one in docs; nobody else does.
6. **Language choice made once, not per block.** Claude switches the whole page; Polars puts Python and Rust side by side on every example. fiscalclock's three bindings return identical JSON, so a Rust / Python / JS switch that persists across the page fits well.
7. **Copy buttons everywhere, plus a "copy page as Markdown" and `llms.txt` route** for AI agents (Stripe, Claude, Vercel, DuckDB, Deno, Pydantic, Resend).
8. **Visible version and provenance on every page**: version selector or stated version (DuckDB, Pydantic), changelog link (Stripe, Vercel, Polars, Pydantic), "Edit this page" (DuckDB, Tokio, Deno) and a last-updated date (Vercel).

### Clichés to avoid

1. **Glow and light effects behind a hero object.** Vercel's home hero is a triangle in a radial glow; Resend's hero has diagonal light streaks. Both are company marketing pages; nothing like it appears in any of the docs.
2. **Gradient-filled marks.** Stripe's docs favicon uses radial gradients; at 16 px fiscalclock gains nothing from one, and gradients fight a calm, flat system.
3. **AI-era and agentic slogans.** Linear ("Designed for the AI era") and Vercel ("Agentic Infrastructure") lead with them. fiscalclock serves agents, but should say what it computes, not which era it belongs to.
4. **Unbacked superlatives.** Pydantic's docs open with "most widely used"; uv's "extremely fast" is at least followed by a benchmark. fiscalclock's evidence rules already forbid the first kind.
5. **Customer-logo walls and product-screenshot heroes.** Vercel's home page shows a row of company logos; Linear's hero is a large app screenshot. A library has neither to show and should not imitate them.
6. **Fine detail that fails at favicon size.** Linear's diagonal lines and Polars' pixel bear both blur at 16 px; Vercel's single solid shape and DuckDB's two circles do not.

The brief's other clichés (purple-to-blue gradients, mesh backgrounds, sparkle icons, vague bento grids) did not appear on the eleven docs pages sampled; the closest is the Geist introduction page, which uses a grid of tiles, each tied to a specific asset ([Geist](https://vercel.com/geist/introduction)).

### What one maintainer can carry, and what needs a design team

Transferable to a small library:

- An off-the-shelf docs theme already provides the three-column layout, the system-following dark mode, the theme toggle, tabbed code blocks and copy buttons. uv and Polars show this on Material for MkDocs (**inferred** theme), and Resend Docs on Mintlify (**inferred**). DD-009 leaves the stack open; this only shows that these features need no custom design.
- An SVG favicon with an internal `prefers-color-scheme` rule (Tokio, Polars), or a solid tile behind the mark (Claude, Linear, uv).
- OFL fonts already used by admired sites: Inter or Geist for text, JetBrains Mono or Geist Mono for code.
- Repository-backed trust signals: "Edit this page", a changelog link, a stated version, a Markdown view of each page and an `llms.txt` file.
- A tiny token set: one neutral ramp mirrored for light and dark, one accent, and semantic colours used only where they mean something (Resend's method badges, Vercel's red, amber and green scales).
- **An opening none of the eleven took:** no sampled docs page applies tabular figures to prose or tables (only Vercel's "New" badges). fiscalclock's output is dates, week numbers and period bounds, so setting `font-variant-numeric: tabular-nums` on tables, results and code-adjacent figures is a cheap, visible precision cue. Inter lists tabular numbers among its features.

Needs a company:

- Custom typefaces (Claude's `anthropicSans` and `anthropicSerif`, Vercel's Geist family) and commissioned display faces.
- Animated or rendered heroes, live status pages, "Ask AI" assistants and localisation (Stripe shows a country and language selector).
- Sticky multi-language request and response panels across a large API reference (Resend, Stripe). A smaller version, one example per binding with identical output, is within reach because fiscalclock's JSON is byte-identical across languages.

## Not verified

- Whether Stripe Docs offers dark mode through an account or browser setting not visible on the page. Neither emulation nor the page controls produced one as of 1 Nov 2026.
- The Deno favicon (the `.ico` did not render in the test page) and Deno's logo construction.
- Line highlighting: no highlighted lines were found, but only common class names were checked.
- Resend's article text width: the sampled API page has no long paragraph in the main column.
- Licences of `anthropicSans`, `Berkeley Mono`, `SuisseIntl`, `ABC Diatype`, `Dwight`, `Proxima Nova`, `Commit Mono` and `Paper Mono`.
- Whether Polars actually renders Proxima Nova on systems that have it installed locally.
- Linear Docs in light mode: the docs pages rendered dark under both emulated preferences, and only the developer docs exposed a toggle.
