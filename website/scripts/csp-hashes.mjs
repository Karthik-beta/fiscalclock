// Keeps the script-src hashes in vercel.json's Content-Security-Policy equal to
// the set of inline <script> elements in the built site, so the policy allows
// Starlight's inline scripts without 'unsafe-inline'.
//
//   node scripts/csp-hashes.mjs          rewrite vercel.json from dist/
//   node scripts/csp-hashes.mjs --check  exit 1 if vercel.json is out of date
//
// Run `npm run build` first.

import { createHash } from "node:crypto";
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const website = resolve(dirname(fileURLToPath(import.meta.url)), "..");
export const VERCEL_JSON = join(website, "vercel.json");
export const DIST = join(website, "dist");

const EXECUTABLE_TYPES = new Set(["", "module", "text/javascript", "application/javascript"]);

/** Sorted, unique `'sha256-…'` sources for every inline script in every built HTML page. */
export function inlineScriptHashes(dist = DIST) {
  const hashes = new Set();
  const pages = readdirSync(dist, { recursive: true, encoding: "utf8" }).filter((f) => f.endsWith(".html"));
  for (const page of pages) {
    const html = readFileSync(join(dist, page), "utf8");
    for (const [, attrs, body] of html.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/g)) {
      if (/\bsrc\s*=/.test(attrs)) continue;
      const type = /\btype\s*=\s*"([^"]*)"/.exec(attrs)?.[1] ?? "";
      if (!EXECUTABLE_TYPES.has(type.toLowerCase())) continue;
      hashes.add(`'sha256-${createHash("sha256").update(body, "utf8").digest("base64")}'`);
    }
  }
  return [...hashes].sort();
}

/** The Content-Security-Policy header object that vercel.json applies to every path. */
export function cspHeader(config) {
  const rule = config.headers.find((r) => r.source === "/(.*)");
  const header = rule?.headers.find((h) => h.key === "Content-Security-Policy");
  if (!header) throw new Error('vercel.json has no Content-Security-Policy header on source "/(.*)"');
  return header;
}

/** The policy with script-src's hash sources replaced by `hashes`, other sources kept in order. */
export function withScriptHashes(policy, hashes) {
  return policy
    .split(";")
    .map((d) => d.trim())
    .map((directive) => {
      const [name, ...sources] = directive.split(/\s+/);
      if (name !== "script-src") return directive;
      return [name, ...sources.filter((s) => !s.startsWith("'sha256-")), ...hashes].join(" ");
    })
    .join("; ");
}

function main() {
  const check = process.argv.includes("--check");
  const config = JSON.parse(readFileSync(VERCEL_JSON, "utf8"));
  const header = cspHeader(config);
  const wanted = withScriptHashes(header.value, inlineScriptHashes());
  if (wanted === header.value) {
    console.log("csp-hashes: vercel.json matches dist/");
    return;
  }
  if (check) {
    console.error("csp-hashes: vercel.json's script-src hashes do not match the inline scripts in dist/.");
    console.error("Run `node scripts/csp-hashes.mjs` in website/ after `npm run build`, then commit vercel.json.");
    process.exit(1);
  }
  header.value = wanted;
  writeFileSync(VERCEL_JSON, JSON.stringify(config, null, 2) + "\n");
  console.log("csp-hashes: updated vercel.json");
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main();
