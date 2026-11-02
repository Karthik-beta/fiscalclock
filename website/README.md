<!-- cspell:words lockups WCAG -->

# fiscalclock website

The source of the fiscalclock website at `https://fiscalclock.karthik.nz`, which will hold the documentation, a landing page and a playground. It has the documentation pages and a placeholder landing page. It is built with [Astro Starlight](https://starlight.astro.build/) as a static site. The choice of framework and host is recorded in decisions DD-030 to DD-036 in [`design/DECISIONS.md`](../design/DECISIONS.md).

Node.js 22.23.4 or later is required, because Astro 7.4.4 declares `"node": ">=22.23.4"` in its `package.json` `engines` field.

## Commands

Run these in `website/`.

| Command | What it does |
| --- | --- |
| `npm ci` | Installs the exact versions in `package-lock.json`. |
| `npm run dev` | Copies the generated files (see below), then serves the site with live reload. |
| `npm run build` | Copies the generated files, then builds the static site into `dist/`. |
| `npm run check` | Type-checks the site with `astro check`. |
| `npm test` | Runs the Playwright checks against `astro preview` of `dist/`: accessibility (WCAG 2.2 AA, with axe) on every built page in both themes, theme colours, fonts, language tabs, the skip link, internal links and their anchors, and the spec import. Run `npm run build` first. |
| `npm run test:examples` | Runs every code example in `examples/` in Python, JavaScript and Rust against fiscalclock 0.1.0, and fails unless each prints exactly its `.out` file. Needs Python 3 and Cargo; Python runs in `website/.venv`, which the script creates. |
| `npm run shots` | Saves full-page screenshots of every built page, light and dark, at 1280 and 375 pixels wide, into `test-results/shots/` for review. |

## How the theme gets its values

1. The design tokens live in [`design/tokens/`](../design/tokens/). Their build writes `design/tokens/generated/tokens.css` and a resolved JSON file per theme.
2. [`scripts/sync.mjs`](scripts/sync.mjs) runs before `dev`, `build` and `check`. It copies, without changing them, the generated token files into `src/styles/generated/`, the favicon, Apple touch icon, Open Graph image and logo lockups from [`assets/brand/`](../assets/brand/), and the IBM Plex font files and their licences from the `@ibm/plex-sans` and `@ibm/plex-mono` packages. It stops with an error if any source file is missing. The copy list is one table at the top of the script.
3. [`src/styles/theme.css`](src/styles/theme.css) maps Starlight's `--sl-*` variables onto the semantic `--fc-*` tokens, for both themes at once.
4. [`ec.config.mjs`](ec.config.mjs) builds the syntax-highlighting colours for code blocks from the four code tokens in the copied JSON files.

The copied files are generated and ignored by Git. Change a value in `design/tokens/` or a file in `assets/brand/`, never in `website/`.

The font files are served unmodified, as the packages ship them. The SIL Open Font License does not let a modified version use the Reserved Font Name "Plex", so do not subset or convert them.

## Install scripts

`@ibm/plex-sans` and `@ibm/plex-mono` declare a `postinstall` script, `ibmtelemetry --config=telemetry.yml`, in their `package.json`. No other dependency has an install script (`npm install-scripts ls` lists only these two), so `package.json` denies them in `allowScripts`, which npm 11.7.0 honours, and [`vercel.json`](vercel.json) installs with `npm ci --ignore-scripts`.

## Landing page and playground

Both pages run the published `fiscalclock` npm package, pinned to an exact version in `package.json` (DD-032).

- The landing page draws NRF fiscal 2023 from the package's output at build time ([`src/lib/landing.ts`](src/lib/landing.ts)). The build fails unless that output matches the cited NRF vector in `conformance/vectors/cited/` and the independent oracle's dates in `conformance/vectors/oracle/nrf-4-5-4.json`, so the picture cannot drift from its sources.
- The playground ([`src/pages/playground.astro`](src/pages/playground.astro)) loads the package's WebAssembly build and the CodeMirror editor. No other page loads either. Its state lives in the URL fragment (DD-036), and its example list is every file in `conformance/vectors/cited/`.
- One function, [`src/lib/year-strip.ts`](src/lib/year-strip.ts), draws the to-scale year strip on both pages. It draws only the periods it is given and computes no dates.

## Imported pages

[`scripts/import-docs.mjs`](scripts/import-docs.mjs) runs with `sync` and copies `spec/definition-v1.md`, `spec/api-v1.md` and `CHANGELOG.md` into `src/content/docs/` as pages. It takes each title from the file's first heading and rewrites links: a link to another imported file becomes that page's URL, and a link to any other repository path becomes its GitHub URL. A relative link to a path that does not exist stops the import. The copies are ignored by Git, so edit the files in the repository, never the copies. The list of imported files and their URLs is one table at the top of the script.

## Code examples

Every code example in the documentation is a file in [`examples/`](examples/): `python/<name>.py`, `js/<name>.mjs` and `rust/src/bin/<name>.rs`, each next to a `<name>.out` file holding its exact output. A page imports both with a `?raw` import and shows them with Starlight's `<Code>` component in `<Tabs syncKey="lang">`, Python first, so the page shows the same files that `npm run test:examples` runs. The expected outputs are written from the spec and the cited sources, not copied from a run, except where the text is not part of the API (the wording of `explain`, and pandas' table layout). The Rust examples are their own Cargo workspace, outside the repository's workspace.
