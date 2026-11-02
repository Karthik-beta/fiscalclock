import { join } from "node:path";
import { test } from "@playwright/test";
import { THEMES, builtPages, storeTheme } from "./site";

const WIDTHS = [1280, 375];
const outDir = join(import.meta.dirname, "..", "test-results", "shots");

for (const path of builtPages()) {
  for (const theme of THEMES) {
    for (const width of WIDTHS) {
      const name = `${path.replace(/^\/|\/$/g, "").replace(/[/.]/g, "-") || "home"}-${theme}-${width}`;
      test(name, async ({ page }) => {
        await storeTheme(page, theme);
        await page.setViewportSize({ width, height: 900 });
        await page.goto(path);
        await page.evaluate(() => document.fonts.ready);
        await page.screenshot({ path: join(outDir, `${name}.png`), fullPage: true });
      });
    }
  }
}
