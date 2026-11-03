import { expect, test } from "@playwright/test";
import { THEMES, rgb, storeTheme, tokens } from "./site";

const opposite = { light: "dark", dark: "light" } as const;

test("the self-hosted Mona Sans and IBM Plex Mono faces load", async ({ page }) => {
  await page.goto("/guides/style-check/");
  const faces = await page.evaluate(async () => {
    await document.fonts.ready;
    return [...document.fonts].map((face) => ({
      face: `${face.family.replaceAll('"', "")} ${face.weight} ${face.style}`,
      status: face.status,
    }));
  });
  expect(faces.filter(({ status }) => status === "error")).toEqual([]);
  const loaded = faces.filter(({ status }) => status === "loaded").map(({ face }) => face);
  expect(loaded).toContain("Mona Sans 200 900 normal");
  expect(loaded).toContain("IBM Plex Mono 400 normal");
});

for (const theme of THEMES) {
  const bg = rgb(tokens(theme)["--fc-color-bg-default"]);
  const text = rgb(tokens(theme)["--fc-color-text-default"]);

  test(`${theme}: the system colour scheme sets the token colours`, async ({ page }) => {
    await page.emulateMedia({ colorScheme: theme });
    await page.goto("/guides/style-check/");
    await expect(page.locator("body")).toHaveCSS("background-color", bg);
    await expect(page.locator("body")).toHaveCSS("color", text);
  });

  test(`${theme}: a stored choice overrides the opposite system colour scheme`, async ({ page }) => {
    await page.emulateMedia({ colorScheme: opposite[theme] });
    await storeTheme(page, theme);
    await page.goto("/guides/style-check/");
    await expect(page.locator("body")).toHaveCSS("background-color", bg);
    await expect(page.locator("body")).toHaveCSS("color", text);
  });
}
