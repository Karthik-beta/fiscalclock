import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { expect, test } from "@playwright/test";
import { DEFINITIONS } from "../examples/languages.mjs";
import { IMPORTS } from "../scripts/import-docs.mjs";
import { COPIES } from "../scripts/sync.mjs";

// scripts/should-build.sh decides whether Vercel builds, and the paths of
// .github/workflows/website.yml decide whether CI checks the site. Both must
// name exactly the repository paths outside website/ that the site reads.

const website = resolve(import.meta.dirname, "..");
const repo = resolve(website, "..");
const toRepo = (path: string) => relative(repo, path).replaceAll("\\", "/");

const SKIP_DIRS = new Set(["node_modules", "dist", ".astro", ".venv", "target", "test-results", "playwright-report"]);
const SOURCE = /\.(ts|mjs|mdx|astro)$/;
// Relative specifiers of static imports, dynamic imports and import.meta.glob,
// without any `?raw` query. A glob is cut at its first wildcard.
const SPECIFIER = /(?:\bfrom\s*|\bimport\s*\(\s*|import\.meta\.glob(?:<[^>]*>)?\(\s*)["'](\.{1,2}\/[^"'?*]+)/g;

function sourceFiles(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) return SKIP_DIRS.has(entry.name) ? [] : sourceFiles(path);
    return SOURCE.test(entry.name) ? [path] : [];
  });
}

/** Every repository path the site's code imports, plus the files its scripts copy, import or run from. */
function reads(): string[] {
  const imported = sourceFiles(website).flatMap((file) =>
    [...readFileSync(file, "utf8").matchAll(SPECIFIER)].map((match) => toRepo(resolve(dirname(file), match[1]!)).replace(/\/$/, "")),
  );
  const copied = COPIES.map(({ from }) => from).filter((from) => !from.startsWith("website/node_modules/"));
  return [...new Set([...imported, ...copied, ...Object.keys(IMPORTS), DEFINITIONS])].filter((path) => !path.startsWith("website/"));
}

function shouldBuildPaths(): string[] {
  const script = readFileSync(join(website, "scripts", "should-build.sh"), "utf8");
  const list = script.match(/^set -- \\\n((?: {2}":\/[^"]+"(?: \\)?\n)+)/m);
  if (!list) throw new Error("should-build.sh has no `set --` path list");
  return [...list[1]!.matchAll(/":\/([^"]+)"/g)].map((match) => match[1]!);
}

function workflowPaths(): string[] {
  const lines = readFileSync(join(repo, ".github", "workflows", "website.yml"), "utf8").split(/\r?\n/);
  const start = lines.findIndex((line) => line.trim() === "paths: &site-inputs");
  if (start === -1) throw new Error("website.yml has no `paths: &site-inputs` list");
  const paths: string[] = [];
  for (const line of lines.slice(start + 1)) {
    const item = line.match(/^\s+- "([^"]+)"$/);
    if (!item) break;
    paths.push(item[1]!);
  }
  return paths;
}

const covers = (input: string, path: string) => path === input || path.startsWith(`${input}/`);

test("every repository path the site reads outside website/ is in should-build.sh", () => {
  const inputs = shouldBuildPaths();
  const missed = reads().filter((path) => !inputs.some((input) => covers(input, path)));
  expect(missed).toEqual([]);
});

test("should-build.sh names no path the site does not read", () => {
  const read = reads();
  const unread = shouldBuildPaths().filter((input) => input !== "website" && !read.some((path) => covers(input, path)));
  expect(unread).toEqual([]);
});

test("website.yml runs on the should-build.sh paths, its own file and lychee.toml", () => {
  const expected = shouldBuildPaths().map((input) => (statSync(join(repo, input)).isDirectory() ? `${input}/**` : input));
  expect(workflowPaths().sort()).toEqual([...expected, "lychee.toml", ".github/workflows/website.yml"].sort());
});

test("the scan finds the inputs it is meant to find", () => {
  expect(reads()).toEqual(
    expect.arrayContaining([
      "conformance/vectors/oracle/nrf-4-5-4.json",
      "conformance/vectors/cited",
      "conformance/vectors",
      "docs/examples/retail-4-5-4.json",
      "spec/api-v1.md",
      "design/tokens/generated/tokens.css",
    ]),
  );
});
