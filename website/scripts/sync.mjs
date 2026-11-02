// Copies the design tokens, brand assets and font files the site needs into
// website/. Files are copied byte for byte and never edited: the sources in
// design/tokens/generated/ and assets/brand/ are themselves generated, and the
// IBM Plex files must stay unmodified under the SIL Open Font License.

import { copyFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const website = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repo = resolve(website, "..");
const plexSans = "website/node_modules/@ibm/plex-sans";
const plexMono = "website/node_modules/@ibm/plex-mono";

// Paths are relative to the repository root.
const COPIES = [
  { from: "design/tokens/generated/tokens.css", to: "website/src/styles/generated/tokens.css" },
  { from: "design/tokens/generated/tokens.light.json", to: "website/src/styles/generated/tokens.light.json" },
  { from: "design/tokens/generated/tokens.dark.json", to: "website/src/styles/generated/tokens.dark.json" },
  { from: "assets/brand/icons/favicon.svg", to: "website/public/favicon.svg" },
  { from: "assets/brand/icons/favicon.ico", to: "website/public/favicon.ico" },
  { from: "assets/brand/icons/apple-touch-icon.png", to: "website/public/apple-touch-icon.png" },
  { from: "assets/brand/social/og-image.png", to: "website/public/og-image.png" },
  { from: "assets/brand/logo/fiscalclock-lockup.svg", to: "website/src/assets/brand/fiscalclock-lockup.svg" },
  { from: "assets/brand/logo/fiscalclock-lockup-dark.svg", to: "website/src/assets/brand/fiscalclock-lockup-dark.svg" },
  { from: `${plexSans}/fonts/complete/woff2/IBMPlexSans-Regular.woff2`, to: "website/public/fonts/ibm-plex-sans/IBMPlexSans-Regular.woff2" },
  { from: `${plexSans}/fonts/complete/woff2/IBMPlexSans-Italic.woff2`, to: "website/public/fonts/ibm-plex-sans/IBMPlexSans-Italic.woff2" },
  { from: `${plexSans}/fonts/complete/woff2/IBMPlexSans-SemiBold.woff2`, to: "website/public/fonts/ibm-plex-sans/IBMPlexSans-SemiBold.woff2" },
  { from: `${plexSans}/LICENSE.txt`, to: "website/public/fonts/ibm-plex-sans/LICENSE.txt" },
  { from: `${plexMono}/fonts/complete/woff2/IBMPlexMono-Regular.woff2`, to: "website/public/fonts/ibm-plex-mono/IBMPlexMono-Regular.woff2" },
  { from: `${plexMono}/fonts/complete/woff2/IBMPlexMono-SemiBold.woff2`, to: "website/public/fonts/ibm-plex-mono/IBMPlexMono-SemiBold.woff2" },
  { from: `${plexMono}/LICENSE.txt`, to: "website/public/fonts/ibm-plex-mono/LICENSE.txt" },
];

const missing = COPIES.filter(({ from }) => !existsSync(resolve(repo, from)));
if (missing.length > 0) {
  console.error("sync: these source files are missing, so nothing was copied:");
  for (const { from } of missing) console.error(`  ${from}`);
  console.error("Run `npm ci` in website/ for the fonts, or rebuild design/tokens and assets/brand.");
  process.exit(1);
}

for (const { from, to } of COPIES) {
  const target = resolve(repo, to);
  mkdirSync(dirname(target), { recursive: true });
  copyFileSync(resolve(repo, from), target);
}
console.log(`sync: copied ${COPIES.length} files`);
