# Asset specifications for fiscalclock brand assets

<!-- cspell:words WWDC everyplace stylesheet stylesheets Flavored favicon favicons maskable unfurl unfurling unfurls Inkscape Canva rustdoc Camo camo Mastodon Bluesky WebKit Wayback ammonia identicon webp avif Fastly ayu wordmark lockup Discord npmjs srcset -->

Research track: sizes, formats, file-size limits and placement rules for every place the fiscalclock logo, icon or social card appears. Everything below was checked against the linked page on 1 Nov 2026 unless the entry says otherwise. "Observed" means the behaviour was seen in the live page or its CSS on 1 Nov 2026 and is not promised by any documentation; it can change without notice.

Context from [DECISIONS.md](../DECISIONS.md): logo and icon masters are SVG files authored in Inkscape (DD-008); social cards are made in Canva, which on the plan in use exports PNG, JPG and PDF but not SVG; dark mode is first-class (DD-005); the website stack is not chosen (DD-009).

## 1. GitHub repository social preview

Source: [Customising your repository's social media preview](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/customizing-your-repositorys-social-media-preview) (GitHub Docs, as of 1 Nov 2026).

| Requirement | What GitHub says |
| --- | --- |
| Formats | PNG, JPG or GIF. |
| File size | Under 1 MB. |
| Size | "at least 640 by 320 pixels (1280 by 640 pixels for best display)". This is a recommendation; the page states no hard minimum. |
| Transparency | PNG transparency is supported, but GitHub warns the image may sit on different backgrounds or on platforms without transparency support, and recommends a solid background if unsure. |

Deliverable: `github-social-preview.png`, 1280 × 640 px (2:1), opaque background, under 1 MB. Export from Canva as PNG; if the PNG is over 1 MB, export JPG instead. Upload in the repository settings; it is not a file in the repository.

## 2. Open Graph and X cards

### What each consumer documents (as of 1 Nov 2026)

| Consumer | Size and ratio | File size | Formats | Source |
| --- | --- | --- | --- | --- |
| Open Graph protocol | No recommended size. Defines `og:image`, `og:image:width`, `og:image:height`, `og:image:type`, `og:image:alt`, and says a page that sets `og:image` should also set `og:image:alt`. | Not stated | Not stated | [ogp.me](https://ogp.me/) |
| Facebook (Meta) | At least 1200 × 630 px for best display on high-resolution devices; minimum 200 × 200 px; keep "as close to 1.91:1 aspect ratio as possible" to avoid cropping in Feed. | 8 MB maximum | Not listed | [Images in link shares](https://developers.facebook.com/documentation/sharing/webmasters/images) |
| LinkedIn | Minimum 1200 × 627 px; recommended ratio 1.91:1; images under 401 px wide show as a thumbnail. | 5 MB maximum | Not listed for link shares (JPG, PNG, GIF are listed for single-image ads on the same page) | [Make your website shareable on LinkedIn](https://www.linkedin.com/help/linkedin/answer/a521928) |
| X (Twitter) `summary_large_image` | 2:1 aspect ratio; minimum 300 × 157 px; maximum 4096 × 4096 px; `twitter:image:alt` up to 420 characters. | Under 5 MB | JPG, PNG, WEBP, GIF (first frame only). SVG is not supported. | Archived copy only, see note below |
| Discord | No recommended size (its quick-start example uses 1200 × 630). Reads `og:image`, `twitter:image` and their width, height and alt tags; asks pages to declare `og:image:width` and `og:image:height`, otherwise it downloads the image to measure it. The whole fetch must finish within 10 seconds. `summary_large_image` gives a large image below the text. | Not stated | PNG, GIF, JPG/JPEG, WEBP, AVIF | [Link previews overview](https://docs.discord.com/developers/link-previews/overview) |
| Slack | Classic unfurling reads "common OpenGraph and X (formerly known as Twitter) Card metadata". No image size or format guidance. | Not stated | Not stated | [Unfurling links in messages](https://docs.slack.dev/messaging/unfurling-links-in-messages/) |
| Mastodon | Preview cards are "generated using OpenGraph tags"; the card entity has `image`, `width`, `height` and `blurhash`. No size, crop or resize rules documented. | Not stated | Not stated | [PreviewCard entity](https://docs.joinmastodon.org/entities/PreviewCard/) |
| Bluesky | Not verified. The developer docs page on posts and link cards redirected to a page that returned no content when fetched on 1 Nov 2026. | Not verified | Not verified | None |

Note on X: as of 1 Nov 2026 the card documentation is no longer published by X. The old developer.twitter.com and developer.x.com card URLs redirect to the docs.x.com home page, and the docs.x.com page index ([llms.txt](https://docs.x.com/llms.txt)) and full text dump contain no card markup or `summary_large_image` pages. The figures in the table come from the [Wayback Machine copy of the X card page captured on 31 May 2025](https://web.archive.org/web/20250531182428/https://developer.x.com/en/docs/x-for-websites/cards/overview/summary-card-with-large-image). Treat them as the last published rules, not current ones.

Safe areas: none of the official pages above document a crop safe area. Facebook only says images close to 1.91:1 are shown without cropping. The arithmetic below is ours, not a platform rule: a 1200 × 630 image shown at 2:1 loses about 15 px at top and bottom, and a 1280 × 640 image shown at 1.91:1 loses about 29 px at each side. Keeping all text and the logo inside a centred box with at least 60 px clear on every edge covers both cases with room to spare.

### Recommendation

- One Canva design, exported twice:
  - `og-image.png`, 1200 × 630 px (1.91:1), for `og:image` and `twitter:image` on the website. It meets the Facebook recommendation, the LinkedIn minimum and is within the X limits; it is under X's 2:1 by about 5 %, so X may crop slightly.
  - `github-social-preview.png`, 1280 × 640 px (2:1), for the GitHub repository setting (section 1).
- PNG, opaque background, under 1 MB (GitHub is the tightest limit). No SVG anywhere in social cards: X states SVG is unsupported and Discord's format list excludes it.
- Markup: set `og:image:width`, `og:image:height`, `og:image:type` and `og:image:alt` (Open Graph and Discord ask for them) plus `twitter:card` = `summary_large_image` and `twitter:image:alt`.
- Social cards are seen inside other sites' light or dark themes that we do not control, so one opaque card is used for both; no light and dark pair.

## 3. Favicons and app icons

### Sources and what they say (as of 1 Nov 2026)

- HTML Living Standard, [link type "icon"](https://html.spec.whatwg.org/multipage/links.html#rel-icon): `sizes` lists sizes in raw pixels and must not list sizes the file does not contain; the keyword `any` means the file is scalable. The user agent picks the most appropriate icon using `type`, `media` and `sizes`, uses the last one in tree order when several are equally appropriate, and falls back to the next one if the chosen icon turns out unusable. With no icon link at all, a user agent "may" fetch `/favicon.ico` for HTTP(S) documents.
- MDN, [`rel` attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel): repeats the selection rule and says iOS uses the non-standard `apple-touch-icon` rather than `rel="icon"` for Home Screen icons.
- Apple, [Configuring Web Applications](https://developer.apple.com/library/archive/documentation/AppleApplications/Reference/SafariWebContent/ConfiguringWebApplications/ConfiguringWebApplications.html) (archived documentation): a site-wide `apple-touch-icon.png` in the site root, or `<link rel="apple-touch-icon">`; 180 × 180 appears only in an example (`touch-icon-iphone-retina.png`), alongside 152 × 152 and 167 × 167. When no icon matches the device size, "the smallest icon larger than the recommended size is used". Apple's current documentation does not state a required size; 180 × 180 is therefore a convention, and a single 180 px icon is the largest of Apple's examples, so by that rule it is used wherever none of the others exists.
- WebKit, [News from WWDC25](https://webkit.org/blog/16993/news-from-wwdc25-web-technology-coming-this-fall-in-safari-26-beta/): Safari 26 supports SVG "for icons everyplace there are icons in the interface, including favicons", and icons defined in a web app manifest are used for the Home Screen and Dock. The [Safari 26.0 release notes](https://developer.apple.com/documentation/safari-release-notes/safari-26-release-notes) list "Added support for SVG icons."
- Can I use, [SVG favicons](https://caniuse.com/link-icon-svg) (as of 1 Nov 2026): Chrome and Edge 80+, Firefox 41+, Safari and iOS Safari 26+; 92.06 % global usage.
- Google Search Central, [Define a favicon to show in search results](https://developers.google.com/search/docs/appearance/favicon-in-search) (page last updated 27 Oct 2026): square, at least 8 × 8 px, larger than 48 × 48 px recommended; supported formats are BMP, GIF, ICO, PNG, JPEG, PPM and TIFF. **SVG is not in the list.** Google accepts `icon`, `shortcut icon`, `apple-touch-icon` and `apple-touch-icon-precomposed`.
- Web App Manifest, [W3C Working Draft of 26 October 2026](https://www.w3.org/TR/appmanifest/): icon `purpose` is `any` (default), `monochrome` or `maskable`; the maskable safe zone is a centred circle with a radius of 40 % of the icon size.
- web.dev, [Maskable icons](https://web.dev/articles/maskable-icon): same 40 % radius safe zone; the outer 10 % may be cropped; advises against declaring one file as both `any` and `maskable`.
- web.dev, [What does it take to be installable?](https://web.dev/articles/install-criteria): for Chrome's install prompt the manifest `icons` "must include a 192px and a 512px icon".
- MDN, [manifest `icons`](https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Manifest/Reference/icons): only `src` is required; give exact `sizes` for raster files; `sizes: "any"` for SVG.

### Open points the official sources leave unanswered

- **`prefers-color-scheme` inside an SVG favicon.** No browser vendor documentation or Can I use entry describes whether a media query inside an SVG favicon is honoured or updates live. Only forum reports exist, and they conflict. Do not rely on it: design the icon so one file reads on both light and dark tab strips (for example a filled tile with the mark knocked out), and treat any in-SVG dark rule as an enhancement to be tested in each browser.
- **Which sizes `favicon.ico` should contain.** No primary source states this. The HTML Standard only covers the `/favicon.ico` fallback. Our choice is 16, 32 and 48 px. Google's advice to use an icon larger than 48 px is then met by the 180 px `apple-touch-icon`, a `rel` value Google accepts; which of a page's icons Google picks is not documented.
- **Safari pinned tab `mask-icon`.** The only Apple page describing it is [Creating Pinned Tab Icons](https://developer.apple.com/library/archive/documentation/AppleApplications/Reference/SafariWebContent/pinnedTabs/pinnedTabs.html), in the documentation archive, last updated 12 Dec 2016 (single-colour black SVG, `viewBox="0 0 16 16"`, `color` attribute). The [Safari 12 release notes](https://developer.apple.com/documentation/safari-release-notes/safari-12-release-notes) add "Show website icons in tabs", and Safari 26 supports SVG favicons. No current Apple page says whether `mask-icon` is still read. Recommendation: omit it.

### Recommended minimal set

Core (any website):

| File | Size | Format | Markup |
| --- | --- | --- | --- |
| `favicon.ico` | 16, 32, 48 px in one file | ICO | At the site root; `<link rel="icon" href="/favicon.ico" sizes="32x32">` |
| `icon.svg` | Scalable, square `viewBox` | SVG, text outlined | `<link rel="icon" href="/icon.svg" type="image/svg+xml">` |
| `apple-touch-icon.png` | 180 × 180 px, opaque background | PNG | At the site root and `<link rel="apple-touch-icon" href="/apple-touch-icon.png">` |

Add only if the site should be installable as an app (the manifest is otherwise optional):

| File | Size | Format | Manifest entry |
| --- | --- | --- | --- |
| `icon-192.png` | 192 × 192 px | PNG | `purpose: "any"` |
| `icon-512.png` | 512 × 512 px | PNG | `purpose: "any"` |
| `icon-maskable-512.png` | 512 × 512 px, mark inside the central circle of radius 205 px (40 %) | PNG | `purpose: "maskable"` |
| `site.webmanifest` | Not applicable | JSON | `<link rel="manifest" href="/site.webmanifest">` |

## 4. README banner on GitHub

| Topic | Finding | Basis |
| --- | --- | --- |
| Display width | Not documented. GitHub's markdown CSS sets `.markdown-body img { max-width: 100% }` and adds `style="max-width: 100%;"` to README images, so images scale down to the column but never up. At a 1600 px wide window the README column was 838 CSS px wide. | Observed on the [fiscalclock repository page](https://github.com/Karthik-beta/fiscalclock) |
| Relative paths | Documented: use relative links for images stored in the repository. | [Basic writing and formatting syntax](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#images) |
| `<picture>` | Documented only as "The `<picture>` HTML element is supported." The current page does not describe theme-specific images. GitHub announced light and dark images via `<picture>` with `prefers-color-scheme` in the [August 2022 changelog](https://github.blog/changelog/2022-08-15-specify-theme-context-for-images-in-markdown-ga/). The GitHub Markdown API wraps it in a `themed-picture` element (observed). | Docs and changelog as linked |
| `<picture>` and the GitHub theme | A [community report of 22 Oct 2026](https://github.com/orgs/community/discussions/210983) says the picked image follows the operating system's colour scheme, not the theme chosen in GitHub settings. No GitHub staff reply. Unconfirmed. | Community discussion |
| `#gh-dark-mode-only` and `#gh-light-mode-only` | Not mentioned anywhere in the current GitHub Docs or in the 2022 changelog. Their status (supported, deprecated or removed) cannot be confirmed from an official source as of 1 Nov 2026. Do not use them: other renderers (crates.io, PyPI, npm) ignore the fragment and would show both images. | Absence from docs |
| Proxying | Images on external hosts are rewritten to `camo.githubusercontent.com` (Camo). Camo only proxies content types on its list. Images from the repository itself (`raw.githubusercontent.com`) are not proxied (observed). | [About anonymised URLs](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/about-anonymized-urls) |
| SVG | SVG is a supported upload type in issues and comments; 10 MB limit for images. | [Attaching files](https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/attaching-files) |
| SVG styles and fonts | README images are shown through `<img>`, where browsers disable scripts and block external resources such as stylesheets ([MDN, SVG as an image](https://developer.mozilla.org/en-US/docs/Web/SVG/Guides/SVG_as_an_image)). `raw.githubusercontent.com` also serves SVG with `Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline'; sandbox` (observed with `curl -I`). Inline `<style>` therefore works, but a web font cannot be loaded, so text would render in a fallback system font. **Convert all text to paths in Inkscape** before export. | MDN plus observed headers |
| `<picture>` structure | The HTML Standard requires the `img` to be a direct child of `picture`, after the `source` elements ([picture element](https://html.spec.whatwg.org/multipage/embedded-content.html#the-picture-element)). | HTML Standard |

Deliverables:

- `readme-banner-light.svg` and `readme-banner-dark.svg`: same layout, transparent background, all text outlined, about 840 × 210 px `viewBox` (4:1) so it fills the 838 px column; the 4:1 shape is our choice, not a rule.
- `readme-banner.png`: 1680 × 420 px (2× of the above) with an opaque background that reads in both themes. This is the `<img>` fallback inside `<picture>` and the version used on PyPI, npm and the wiki (sections 6, 7 and 8).
- Use absolute `https://raw.githubusercontent.com/Karthik-beta/fiscalclock/<tag-or-main>/...` URLs, not relative ones, because the same README is published to crates.io, PyPI and npm.

## 5. crates.io and docs.rs

crates.io:

- No per-crate logo. The [Cargo manifest reference](https://doc.rust-lang.org/cargo/reference/manifest.html) has no logo or icon field.
- The README is rendered by crates.io's own [Markdown crate](https://github.com/rust-lang/crates.io/blob/main/crates/crates_io_markdown/src/lib.rs) (source read on 1 Nov 2026): the `ammonia` sanitiser allows `picture` and `source` with `media` and `srcset`; relative links are resolved against the `repository` URL only when its host is github.com, gitlab.com or bitbucket.org, and images go to `raw/HEAD/<path>` (the default branch, not the published version). Relative SVG links get `?sanitize=true`. Whether relative URLs inside `srcset` are rewritten was not verified; use absolute URLs.
- crates.io has a colour-scheme switch ("system" by default) and was dark when the OS was dark (observed), so `<picture>` sources work there when the setting is "system". External images were not proxied (observed).

docs.rs (rustdoc):

- [The `#[doc]` attribute](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html): `html_favicon_url` adds `<link rel="icon" href="...">`; `html_logo_url` adds a logo at the upper left of the docs. The book gives no size or format rules and still shows `width='100'` in the generated markup.
- Observed on [docs.rs/rand](https://docs.rs/rand/latest/rand/) (rustdoc 1.100.0-nightly of 19 Sep 2026): the generated `<img>` has no width attribute; the stylesheet sizes `.logo-container > img` to 48 × 48 CSS px in the sidebar and at most 35 × 35 px in the mobile top bar. The dark and ayu themes apply a white outline filter only to rustdoc's default Rust logo (class `rust-logo`), not to a custom logo. A custom logo therefore appears unchanged on the light (`#f5f5f5`), dark (`#505050`) and ayu (`#14191f`) sidebar backgrounds.
- Only one URL each is possible, so there is no light and dark pair. Any format the browser shows in `<img>` works, SVG included.

Deliverables:

- `docsrs-logo.svg`: square, mark on an opaque rounded tile so it reads on all three sidebar colours; designed to be legible at 48 px and 35 px.
- `html_favicon_url`: reuse `favicon.ico` (served as `image/vnd.microsoft.icon` by `raw.githubusercontent.com`, observed) for the widest browser support; `icon.svg` would miss Safari before 26.
- Point both attributes at a tagged, absolute URL so old documentation does not change when the files do.

## 6. PyPI

- PyPI shows no project logo; the project page has no field for one (observed; no PyPI documentation describes one).
- [Making a PyPI-friendly README](https://packaging.python.org/en/latest/guides/making-a-pypi-friendly-readme/) only covers markup formats; it says nothing about images or relative links.
- PyPI renders the README with [readme_renderer](https://github.com/pypa/readme_renderer/blob/main/readme_renderer/clean.py) (latest release [46.0](https://github.com/pypa/readme_renderer/releases/tag/46.0), 28 Aug 2026). Its allow-list includes `picture` and `img` (with `src`, `width`, `height`, `alt`, `align`, `class`) but **not `source`**. A `<picture>` therefore keeps only its fallback `<img>` on PyPI (derived from the code, not tested by upload).
- Images are proxied through `pypi-camo.freetls.fastly.net`, SVG badges included (observed). PyPI's page stayed light with the OS set to dark, and its stylesheet has no `prefers-color-scheme` rule (observed).
- Relative image URLs: not documented. Use absolute URLs.

Deliverable: none new. The `<picture>` fallback `readme-banner.png` (or an SVG with an opaque background) is what PyPI shows.

## 7. npm

- npmjs.com shows no package logo (observed; npm documents no field for one).
- [About package README files](https://docs.npmjs.com/about-package-readme-files/): the README is rendered "as GitHub Flavored Markdown via GitHub's API" and only updates on publish. The page says nothing about images or relative links; use absolute URLs.
- Observed on the npm pages for `tailwindcss` and `@biomejs/biome`: npm wraps every README `<img>` in a link. Inside `<picture>` that makes the `img` no longer a direct child, so the browser ignores the `source` elements and always shows the fallback. With the OS in dark mode, both pages showed their light image. npm has its own light and dark theme setting.

Deliverable: none new. The fallback image must read on npm's light and dark themes, which is why `readme-banner.png` has an opaque background.

## 8. GitHub avatar, wiki and Discussions

- Avatar: the current [profile picture instructions](https://docs.github.com/en/account-and-profile/tutorials/personalize-your-profile) give no size, format or file-size rule. The repository belongs to a personal account, so the avatar is the owner's, not a project asset. If an organisation is created later, a square 500 × 500 px PNG of the mark on an opaque tile is a reasonable choice; that size is not from GitHub's documentation.
- Wiki: "Wikis can display PNG, JPEG, and GIF images" ([Editing wiki content](https://docs.github.com/en/communities/documenting-your-project-with-wikis/editing-wiki-content)). SVG is not listed, so the wiki needs PNG versions: `readme-banner.png` and `fiscalclock-logo.png`. There is no setting for a wiki logo.
- Discussions: use the same Markdown and image rules as issues (section 4); no separate logo setting is documented.

## 9. Other placements considered

- Google Search result favicon: covered in section 3; needs a non-SVG icon, which the core set already has.
- Mastodon and Bluesky profile images, JSR, Read the Docs and code mirrors: not included. fiscalclock publishes to none of them as of 1 Nov 2026, and the Bluesky documentation could not be retrieved. Add them when an account or listing exists.

## Asset inventory

| Asset | File name | Pixel size | Format | Light and dark | Used in | Source |
| --- | --- | --- | --- | --- | --- | --- |
| Mark master | `fiscalclock-mark.svg` | Square `viewBox` | SVG (Inkscape, text outlined) | One file, designed for both | Source for every icon | [DECISIONS.md](../DECISIONS.md) DD-008 |
| Logo lockup | `fiscalclock-logo-light.svg`, `fiscalclock-logo-dark.svg`, `fiscalclock-logo.png` | Lockup `viewBox`; PNG at 2× display width | SVG, PNG for the wiki | Pair plus opaque PNG | README `<picture>`, wiki, website header | [GitHub wiki images](https://docs.github.com/en/communities/documenting-your-project-with-wikis/editing-wiki-content) |
| README banner | `readme-banner-light.svg`, `readme-banner-dark.svg`, `readme-banner.png` | 840 × 210 `viewBox`; PNG 1680 × 420 | SVG, PNG | Pair plus opaque fallback | GitHub, crates.io, PyPI, npm, wiki | [GitHub changelog](https://github.blog/changelog/2022-08-15-specify-theme-context-for-images-in-markdown-ga/), [readme_renderer](https://github.com/pypa/readme_renderer/blob/main/readme_renderer/clean.py) |
| GitHub social preview | `github-social-preview.png` | 1280 × 640 | PNG (JPG if over 1 MB) | One opaque card | Repository settings | [GitHub Docs](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/customizing-your-repositorys-social-media-preview) |
| Open Graph and X card | `og-image.png` | 1200 × 630 | PNG, under 1 MB | One opaque card | Website `og:image`, `twitter:image` | [Meta](https://developers.facebook.com/documentation/sharing/webmasters/images), [LinkedIn](https://www.linkedin.com/help/linkedin/answer/a521928) |
| Favicon fallback | `favicon.ico` | 16, 32, 48 | ICO | One file, readable on both | Website root, docs.rs favicon | [HTML Standard](https://html.spec.whatwg.org/multipage/links.html#rel-icon) |
| Scalable favicon | `icon.svg` | Square `viewBox` | SVG | One file; in-SVG dark rule optional and untested | Website | [Can I use](https://caniuse.com/link-icon-svg) |
| Apple touch icon | `apple-touch-icon.png` | 180 × 180 | PNG, opaque | One | Website, Google Search | [Apple archive](https://developer.apple.com/library/archive/documentation/AppleApplications/Reference/SafariWebContent/ConfiguringWebApplications/ConfiguringWebApplications.html), [Google](https://developers.google.com/search/docs/appearance/favicon-in-search) |
| App icons (optional) | `icon-192.png`, `icon-512.png`, `icon-maskable-512.png`, `site.webmanifest` | 192, 512, 512 | PNG, JSON | One | Website, only if installable | [web.dev](https://web.dev/articles/install-criteria), [W3C](https://www.w3.org/TR/appmanifest/) |
| docs.rs logo | `docsrs-logo.svg` | Square, shown at 48 and 35 CSS px | SVG | One, on an opaque tile | `html_logo_url` | [rustdoc book](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html) |
