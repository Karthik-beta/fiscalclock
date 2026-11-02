import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect, test } from "@playwright/test";
import { builtPages } from "./site";

const site = "https://fiscalclock.karthik.nz";
const dist = join(import.meta.dirname, "..", "dist");
const listed = [...readFileSync(join(dist, "sitemap-0.xml"), "utf8").matchAll(/<loc>([^<]+)<\/loc>/g)].map((m) => m[1]);

const htmlFile = (path: string) => join(dist, path.endsWith(".html") ? path : `${path}index.html`);
const noindex = (path: string) => /<meta name="robots" content="noindex"/.test(readFileSync(htmlFile(path), "utf8"));

test("the sitemap lists every indexable page and no page marked noindex", () => {
  const pages = builtPages().filter((path) => path !== "/404.html");
  const hidden = pages.filter(noindex);
  expect(hidden).toContain("/guides/style-check/");
  expect(listed.toSorted()).toEqual(pages.filter((path) => !hidden.includes(path)).map((path) => site + path));
});
