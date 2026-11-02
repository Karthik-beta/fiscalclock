<!-- cspell:words lockups plugin WCAG -->

# fiscalclock website

The source of the fiscalclock website at `https://fiscalclock.karthik.nz`, which will hold the documentation, a landing page and a playground. So far it has the site's theme and two placeholder pages. It is built with [Astro Starlight](https://starlight.astro.build/) as a static site. The choice of framework and host is recorded in decisions DD-030 to DD-036 in [`design/DECISIONS.md`](../design/DECISIONS.md).

Node.js 22.23.4 or later is required, because Astro 7.4.4 declares `"node": ">=22.23.4"` in its `package.json` `engines` field.

## Commands

Run these in `website/`.

| Command | What it does |
| --- | --- |
| `npm ci` | Installs the exact versions in `package-lock.json`. |
| `npm run dev` | Copies the generated files (see below), then serves the site with live reload. |
| `npm run build` | Copies the generated files, then builds the static site into `dist/`. |
| `npm run check` | Type-checks the site with `astro check`. |
| `npm test` | Runs the unit tests in `tests/unit/` with `node --test`, then the Playwright checks against `astro preview` of `dist/`: accessibility (WCAG 2.2 AA, with axe) on every built page in both themes, theme colours, fonts, language tabs and the skip link. Run `npm run build` first. |
| `npm run shots` | Saves full-page screenshots of every built page, light and dark, at 1280 and 375 pixels wide, into `test-results/shots/` for review. |

## How the theme gets its values

1. The design tokens live in [`design/tokens/`](../design/tokens/). Their build writes `design/tokens/generated/tokens.css` and a resolved JSON file per theme.
2. [`scripts/sync.mjs`](scripts/sync.mjs) runs before `dev`, `build` and `check`. It copies, without changing them, the generated token files into `src/styles/generated/`, the favicon, Apple touch icon, Open Graph image and logo lockups from [`assets/brand/`](../assets/brand/), and the IBM Plex font files and their licences from the `@ibm/plex-sans` and `@ibm/plex-mono` packages. It stops with an error if any source file is missing. The copy list is one table at the top of the script.
3. [`src/styles/theme.css`](src/styles/theme.css) maps Starlight's `--sl-*` variables onto the semantic `--fc-*` tokens, for both themes at once.
4. [`ec.config.mjs`](ec.config.mjs) builds the syntax-highlighting colours for code blocks from the four code tokens in the copied JSON files.

The copied files are generated and ignored by Git. Change a value in `design/tokens/` or a file in `assets/brand/`, never in `website/`.

The font files are served unmodified, as the packages ship them. The SIL Open Font License does not let a modified version use the Reserved Font Name "Plex", so do not subset or convert them.

## Files for AI agents

Decision DD-033 makes AI agents part of the audience. The build publishes:

- `/llms.txt`, `/llms-full.txt` and `/llms-small.txt`, written by the [`starlight-llms-txt`](https://delucis.github.io/starlight-llms-txt/) plugin. A build step in [`src/agents/llms-postbuild.ts`](src/agents/llms-postbuild.ts) then adds a list of every page's Markdown copy to `llms.txt` and drops the pages in `LLMS_EXCLUDE` ([`src/agents/paths.ts`](src/agents/paths.ts)), because the plugin can do neither.
- A Markdown copy of every docs page at the page's path plus `.md`, with the home page at `/index.md`. [`src/pages/[...slug].md.ts`](src/pages/[...slug].md.ts) converts each page's MDX with [`src/agents/mdx-to-markdown.ts`](src/agents/mdx-to-markdown.ts). Its `COMPONENTS` table says how each MDX component becomes Markdown. A page that uses a component missing from the table fails the build, so add an entry when a page needs a new component.
- A `<link rel="alternate" type="text/markdown">` to that copy in each page's head, and a "Copy page as Markdown" button and a "View as Markdown" link under the page title. Pages with a `hero` show no title, so they have no button.

## Install scripts

`@ibm/plex-sans` and `@ibm/plex-mono` declare a `postinstall` script, `ibmtelemetry --config=telemetry.yml`, in their `package.json`. No other dependency has an install script (`npm install-scripts ls` lists only these two), so `package.json` denies them in `allowScripts`, which npm 11.7.0 honours, and [`vercel.json`](vercel.json) installs with `npm ci --ignore-scripts`.
