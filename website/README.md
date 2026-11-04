<!-- cspell:words fontsource lockups Mona plugin WCAG -->

# fiscalclock website

The source of the fiscalclock website at `https://fiscalclock.karthik.nz`, which holds the documentation, a landing page with a live date lookup, and a playground. It is built with [Astro Starlight](https://starlight.astro.build/) as a static site. The choice of framework and host is recorded in decisions DD-030 to DD-036 in [`design/DECISIONS.md`](../design/DECISIONS.md).

Use Node.js 24. `package.json` sets `engines.node` to `24.x`, so Vercel builds with the newest Node.js 24 release, the same major version as the [`website.yml`](../.github/workflows/website.yml) workflow. Astro 7.4.4 needs Node.js 22.23.4 or later (the `engines` field of its `package.json`).

## Commands

Run these in `website/`.

| Command | What it does |
| --- | --- |
| `npm ci` | Installs the exact versions in `package-lock.json`. |
| `npm run dev` | Copies the generated files (see below), then serves the site with live reload. |
| `npm run build` | Copies the generated files, then builds the static site into `dist/`. |
| `npm run check` | Type-checks the site with `astro check`. |
| `npm test` | Runs the unit tests in `tests/unit/` with `node --test`, then the Playwright checks against `astro preview` of `dist/`: accessibility (WCAG 2.2 AA, with axe) on every built page in both themes, theme colours, fonts, language tabs, the skip link, internal links and their anchors, the spec import, the sitemap and the files for AI agents. It also checks the deployment configuration (see [Deployment](#deployment)). Run `npm run build` first. |
| `npm run test:examples` | Runs every code example in `examples/` in Python, JavaScript and Rust against the fiscalclock versions pinned for each language (see [Code examples](#code-examples)), and fails unless each prints exactly its `.out` file. Needs Python 3 and Cargo; Python runs in `website/.venv`, which the script creates. |
| `npm run shots` | Saves full-page screenshots of every built page, light and dark, at 1280 and 375 pixels wide, into `test-results/shots/` for review. |

## How the theme gets its values

1. The design tokens live in [`design/tokens/`](../design/tokens/). Their build writes `design/tokens/generated/tokens.css` and a resolved JSON file per theme.
2. [`scripts/sync.mjs`](scripts/sync.mjs) runs before `dev`, `build` and `check`. It copies, without changing them, the generated token files into `src/styles/generated/`, the favicon, Apple touch icon, Open Graph image and logo lockups from [`assets/brand/`](../assets/brand/), and the Mona Sans and IBM Plex Mono font files with their licences from the `@fontsource-variable/mona-sans` and `@ibm/plex-mono` packages. It stops with an error if any source file is missing. The copy list is one table at the top of the script.
3. [`src/styles/theme.css`](src/styles/theme.css) maps Starlight's `--sl-*` variables onto the semantic `--fc-*` tokens, for both themes at once.
4. [`ec.config.mjs`](ec.config.mjs) builds the syntax-highlighting colours for code blocks from the four code tokens in the copied JSON files.

The copied files are generated and ignored by Git. Change a value in `design/tokens/` or a file in `assets/brand/`, never in `website/`.

The font files are served unmodified, as the packages ship them. The SIL Open Font License does not let a modified version use the Reserved Font Name "Plex", so do not subset or convert them.

## Files for AI agents

Decision DD-033 makes AI agents part of the audience. The build publishes:

- `/llms.txt`, `/llms-full.txt` and `/llms-small.txt`, written by the [`starlight-llms-txt`](https://delucis.github.io/starlight-llms-txt/) plugin. A build step in [`src/agents/llms-postbuild.ts`](src/agents/llms-postbuild.ts) then adds a list of every page's Markdown copy to `llms.txt` and drops the pages in `HIDDEN_PAGES` ([`src/agents/paths.ts`](src/agents/paths.ts)), because the plugin can do neither.
- A Markdown copy of every docs page at the page's path plus `.md`, with the home page at `/index.md`. [`src/pages/[...slug].md.ts`](src/pages/[...slug].md.ts) converts each page's MDX with [`src/agents/mdx-to-markdown.ts`](src/agents/mdx-to-markdown.ts). Its `COMPONENTS` table says how each MDX component becomes Markdown. A page that uses a component missing from the table fails the build, so add an entry when a page needs a new component. Expressions in braces may be comments, strings, `?raw` imports or dotted names exported by a module in `src/lib/`; any other expression also fails the build.
- A `<link rel="alternate" type="text/markdown">` to that copy in each page's head, and a "Copy page as Markdown" button and a "View as Markdown" link under the page title. Pages with a `hero` show no title, so they have no button.

## Install scripts

`@ibm/plex-mono` declares a `postinstall` script, `ibmtelemetry --config=telemetry.yml`, in its `package.json`. No other dependency has an install script (`npm install-scripts ls` lists only this one), so `package.json` denies it in `allowScripts`, which npm 11.7.0 honours, and [`vercel.json`](vercel.json) installs with `npm ci --ignore-scripts`.

## Landing page and playground

Both pages run the published `fiscalclock` npm package, pinned to an exact version in `package.json` (DD-032).

- The landing page draws fiscal 2023 of its illustrative 4-5-4 definition from the package's output at build time ([`src/lib/landing.ts`](src/lib/landing.ts)). The build fails unless that output matches the cited NRF vector in `conformance/vectors/cited/` and the independent oracle's dates in `conformance/vectors/oracle/nrf-4-5-4.json`, so the picture cannot drift from its sources.
- The playground ([`src/pages/playground.astro`](src/pages/playground.astro)) loads the package's WebAssembly build and the CodeMirror editor. No other page loads either. Its state lives in the URL fragment (DD-036), and its example list is every file in `conformance/vectors/cited/`.
- One function, [`src/lib/year-strip.ts`](src/lib/year-strip.ts), draws the to-scale year strip on both pages. It draws only the periods it is given and computes no dates.

## Imported pages

[`scripts/import-docs.mjs`](scripts/import-docs.mjs) runs with `sync` and copies `spec/definition-v1.md`, `spec/api-v1.md` and `CHANGELOG.md` into `src/content/docs/` as pages. It takes each title from the file's first heading and rewrites links: a link to another imported file becomes that page's URL, and a link to any other repository path becomes its GitHub URL. A relative link to a path that does not exist stops the import. The copies are ignored by Git, so edit the files in the repository, never the copies. The list of imported files and their URLs is one table at the top of the script.

## Code examples

Every code example in the documentation is a file in [`examples/`](examples/): `python/<name>.py`, `js/<name>.mjs` and `rust/src/bin/<name>.rs`, each next to a `<name>.out` file holding its exact output. A page imports both with a `?raw` import and shows them with Starlight's `<Code>` component in `<Tabs syncKey="lang">`, Python first, so the page shows the same files that `npm run test:examples` runs. The expected outputs are written from the spec and the cited sources, not copied from a run, except where the text is not part of the API (the wording of `explain`, and pandas' table layout). The Rust examples are their own Cargo workspace, outside the repository's workspace. The package versions the examples run against are pinned per language: `examples/python/requirements.txt`, the `fiscalclock` dependency in `package.json`, and `examples/rust/Cargo.toml` with its `Cargo.lock`. [`examples/languages.mjs`](examples/languages.mjs) lists each language's folder and file extension and the folder of example definitions; the pages do not read it, only `scripts/test-examples.mjs` and `tests/build-inputs.spec.ts` do.

The example definitions the code opens, such as `retail-4-5-4.json`, live in [`docs/examples/`](../docs/examples/), because example definitions belong in `docs/` (design rule 1 in [`AGENTS.md`](../AGENTS.md)). `npm run test:examples` runs every example from that folder, so each opens its definition by bare file name, as the page shows.

## Deployment

Vercel builds and serves the site. [`vercel.json`](vercel.json) holds the install command, the build-skip check, the redirects and the response headers. The other settings are made by hand in the Vercel project.

### What `vercel.json` does

- Installs with `npm ci --ignore-scripts` (see [Install scripts](#install-scripts)).
- Runs [`scripts/should-build.sh`](scripts/should-build.sh) as the [`ignoreCommand`](https://vercel.com/docs/project-configuration/vercel-json#ignorecommand). It builds only when a repository path the site reads changed since the branch's last successful deployment ([`VERCEL_GIT_PREVIOUS_SHA`](https://vercel.com/docs/environment-variables/system-environment-variables)), or since the parent commit when there is none. It builds when it cannot tell, for example when the commit to compare with is not in the clone. The paths are listed once in the script; [`tests/build-inputs.spec.ts`](tests/build-inputs.spec.ts) fails when the list misses a path that the site's imports, `sync.mjs`, `import-docs.mjs` or the example runner read, names a path they do not read, or differs from the `paths` of `website.yml`.
- Sets [`trailingSlash: true`](https://vercel.com/docs/project-configuration/vercel-json#trailingslash), so `/guides/style-check` redirects to `/guides/style-check/`, the form Astro builds and Starlight links to. Paths with a file extension, such as `.md` copies, do not redirect.
- Redirects `fiscalclock.vercel.app`, and only that host, to `https://fiscalclock.karthik.nz` with a 308. Preview deployments have other host names and are not redirected. If Vercel gives the project a different `vercel.app` name, change the `eq` value.
- Sends `Content-Security-Policy`, `X-Content-Type-Options`, `Referrer-Policy` and `Permissions-Policy` on every response. Files under `/_astro/`, whose names contain a content hash, get `Cache-Control: public, max-age=31536000, immutable`; files under `/fonts/`, whose names do not, are cached for a week. `.wasm`, `.md` and `llms*.txt` files get their content types. Everything else gets Vercel's default `public, max-age=0, must-revalidate` ([Cache-Control headers](https://vercel.com/docs/caching/cache-control-headers)).

The policy allows Starlight's inline scripts by their SHA-256 hashes instead of `'unsafe-inline'`. A change that adds or edits an inline script makes `npm test` fail; run `npm run build`, then `node scripts/csp-hashes.mjs`, and commit `vercel.json`. Styles use `'unsafe-inline'`, because Starlight writes `style` attributes into the HTML.

`npm test` checks all of this. It compiles `vercel.json` with `@vercel/routing-utils`, the package Vercel publishes for that step; loads every built page from [`scripts/serve-with-headers.mjs`](scripts/serve-with-headers.mjs), which applies the same headers, and fails on any policy violation; and runs `should-build.sh` in temporary Git repositories.

### Vercel project settings

Set these by hand. Links are to Vercel's documentation as of 2 November 2026.

| Setting | Value |
| --- | --- |
| Root Directory ([build settings](https://vercel.com/docs/builds/configure-a-build#root-directory)) | `website` |
| Framework Preset ([project settings](https://vercel.com/docs/project-configuration/project-settings)) | Astro |
| Node.js Version ([Node.js versions](https://vercel.com/docs/functions/runtimes/node-js/node-js-versions)) | 24.x. Vercel uses the `engines.node` range in `package.json` instead when one is set, and `24.x` there maps to the latest 24.x release; an open range such as `>=22.0.0` would map to the newest major it offers, 26.x as of 2 Nov 2026. |
| Ignored Build Step | Leave the default; `ignoreCommand` in `vercel.json` overrides it. |
| Git Fork Protection ([security settings](https://vercel.com/docs/project-configuration/security-settings#git-fork-protection)) | On. A pull request from a fork then deploys only after you authorise it. |

Nothing deploys from a fork without that authorisation. The [`website.yml`](../.github/workflows/website.yml) workflow builds and tests pull requests from forks with read-only repository access and no secrets.

### DNS

At Cloudflare, add a `CNAME` record named `fiscalclock` whose target is the value the Vercel project's Domains page shows for `fiscalclock.karthik.nz`; each project has its own ([add a domain](https://vercel.com/docs/domains/working-with-domains/add-a-domain)). Set the record to DNS only (grey cloud), not proxied: Vercel recommends connecting directly ([proxies in front of Vercel](https://vercel.com/kb/guide/can-i-use-a-proxy-on-top-of-my-vercel-deployment)).
