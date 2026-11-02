import { expect, test } from "@playwright/test";
import { IMPORTS } from "../scripts/import-docs.mjs";
import { builtPages } from "./site";

const pages = builtPages().filter((path) => path !== "/404.html");

test("the pages imported from the repository are built", () => {
  for (const { route } of Object.values(IMPORTS)) expect(pages).toContain(route);
});

for (const path of pages) {
  test(`every internal link in the content of ${path} reaches a page and its anchor`, async ({ page }) => {
    await page.goto(path);
    const hrefs = await page
      .locator(".sl-markdown-content a[href]")
      .evaluateAll((links) => links.map((a) => (a as HTMLAnchorElement).href));
    const origin = new URL(page.url()).origin;
    const internal = [...new Set(hrefs.filter((href) => href.startsWith(origin)))];

    for (const href of internal) {
      const url = new URL(href);
      const response = await page.request.get(url.pathname);
      expect(response.status(), href).toBe(200);
      if (url.hash) {
        const id = decodeURIComponent(url.hash.slice(1));
        expect(await response.text(), `${href} has an element with id "${id}"`).toContain(`id="${id}"`);
      }
    }
  });
}
