import { expect, test } from "@playwright/test";

test("the site title link is announced once as fiscalclock", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("link", { name: "fiscalclock", exact: true })).toHaveCount(1);
});
