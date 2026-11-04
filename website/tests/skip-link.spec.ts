import { expect, test } from "@playwright/test";
import type { Locator } from "@playwright/test";

async function isPaintedOnTop(link: Locator): Promise<boolean> {
  return link.evaluate((el) => {
    const box = el.getBoundingClientRect();
    const hit = document.elementFromPoint(box.left + box.width / 2, box.top + box.height / 2);
    return hit !== null && el.contains(hit);
  });
}

test("the skip link is the first focusable element and appears when focused", async ({ page }) => {
  await page.goto("/guides/style-check/");
  const link = page.getByRole("link", { name: "Skip to content" });
  expect(await isPaintedOnTop(link), "hidden before focus").toBe(false);

  await page.keyboard.press("Tab");
  await expect(link).toBeFocused();
  expect(await isPaintedOnTop(link), "visible once focused").toBe(true);
});
