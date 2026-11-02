import { expect, test } from "@playwright/test";

test("the copy button copies the page's Markdown and says so", async ({ page, context, request }) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  const expected = await (await request.get("/guides/style-check.md")).text();
  await page.goto("/guides/style-check/");
  await page.getByRole("button", { name: "Copy page as Markdown" }).click();
  await expect(page.getByRole("status")).toHaveText("Copied the page as Markdown.");
  // Windows clipboards store line breaks as CRLF.
  const copied = await page.evaluate(() => navigator.clipboard.readText());
  expect(copied.replaceAll("\r\n", "\n")).toBe(expected);
});

test("without the Clipboard API the copy button opens the Markdown copy", async ({ page }) => {
  await page.addInitScript(() => Object.defineProperty(navigator, "clipboard", { value: undefined }));
  await page.goto("/guides/style-check/");
  await page.getByRole("button", { name: "Copy page as Markdown" }).click();
  await expect(page).toHaveURL(/\/guides\/style-check\.md$/);
});

test("View as Markdown links to the Markdown copy", async ({ page }) => {
  await page.goto("/guides/style-check/");
  await expect(page.getByRole("link", { name: "View as Markdown" })).toHaveAttribute("href", "/guides/style-check.md");
});

test("the landing page links its Markdown copy but shows no copy controls under its headline", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator('link[rel="alternate"][type="text/markdown"]')).toHaveAttribute("href", /index\.md$/);
  await expect(page.getByRole("button", { name: "Copy page as Markdown" })).toHaveCount(0);
});

test("the playground, which has no Markdown copy, neither links nor offers one", async ({ page }) => {
  await page.goto("/playground/");
  await expect(page.locator('link[rel="alternate"][type="text/markdown"]')).toHaveCount(0);
  await expect(page.getByRole("button", { name: "Copy page as Markdown" })).toHaveCount(0);
});
