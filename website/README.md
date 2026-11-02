<!-- cspell:words lockups WCAG -->

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
| `npm test` | Runs the Playwright checks against `astro preview` of `dist/`: accessibility (WCAG 2.2 AA, with axe) on every built page in both themes, theme colours, fonts, language tabs and the skip link. It also checks the deployment configuration (see [Deployment](#deployment)). Run `npm run build` first. |
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

## Deployment

Vercel builds and serves the site. [`vercel.json`](vercel.json) holds the install command, the build-skip check, the redirects and the response headers. The other settings are made by hand in the Vercel project.

### What `vercel.json` does

- Installs with `npm ci --ignore-scripts` (see [Install scripts](#install-scripts)).
- Runs [`scripts/should-build.sh`](scripts/should-build.sh) as the [`ignoreCommand`](https://vercel.com/docs/project-configuration/vercel-json#ignorecommand). It builds only when `website/`, `design/tokens/`, `assets/brand/`, `spec/`, `CHANGELOG.md`, `conformance/vectors/cited/` or `bindings/js/package.json` changed since the branch's last successful deployment ([`VERCEL_GIT_PREVIOUS_SHA`](https://vercel.com/docs/environment-variables/system-environment-variables)), or since the parent commit when there is none. It builds when it cannot tell, for example when the commit to compare with is not in the clone.
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
| Node.js Version ([Node.js versions](https://vercel.com/docs/functions/runtimes/node-js/node-js-versions)) | 24.x. Vercel uses the `engines.node` range in `package.json` instead when one is set; its table maps `>=22.0.0` to the newest major it offers. |
| Ignored Build Step | Leave the default; `ignoreCommand` in `vercel.json` overrides it. |
| Git Fork Protection ([security settings](https://vercel.com/docs/project-configuration/security-settings#git-fork-protection)) | On. A pull request from a fork then deploys only after you authorise it. |

Nothing deploys from a fork without that authorisation. The [`website.yml`](../.github/workflows/website.yml) workflow builds and tests pull requests from forks with read-only repository access and no secrets.

### DNS

At Cloudflare, add a `CNAME` record named `fiscalclock` whose target is the value the Vercel project's Domains page shows for `fiscalclock.karthik.nz`; each project has its own ([add a domain](https://vercel.com/docs/domains/working-with-domains/add-a-domain)). Set the record to DNS only (grey cloud), not proxied: Vercel recommends connecting directly ([proxies in front of Vercel](https://vercel.com/kb/guide/can-i-use-a-proxy-on-top-of-my-vercel-deployment)).
