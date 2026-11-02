import { expect, test } from "@playwright/test";
import { IMPORTS } from "../scripts/import-docs.mjs";

for (const [source, { route }] of Object.entries(IMPORTS)) {
  test(`every internal link on ${route} (from ${source}) reaches a page and, if it has one, an anchor`, async ({ page }) => {
    await page.goto(route);
    const hrefs = await page
      .locator(".sl-markdown-content a[href]")
      .evaluateAll((links) => links.map((a) => (a as HTMLAnchorElement).href));
    const origin = new URL(page.url()).origin;
    const internal = [...new Set(hrefs.filter((href) => href.startsWith(origin)))];
    expect(internal.length, "the page has internal links to check").toBeGreaterThan(0);

    for (const href of internal) {
      const url = new URL(href);
      const response = await page.request.get(url.pathname);
      expect(response.status(), href).toBe(200);
      if (url.hash) {
        const html = await response.text();
        const id = decodeURIComponent(url.hash.slice(1));
        expect(html, `${href} has an element with id "${id}"`).toContain(`id="${id}"`);
      }
    }
  });
}
