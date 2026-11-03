import AxeBuilder from "@axe-core/playwright";
import { expect, test, type Page } from "@playwright/test";
import { THEMES, storeTheme } from "./site";

const WCAG_TAGS = ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"];

const results = (page: Page) => page.locator("#pg-location");
const errors = (page: Page) => page.locator("#pg-errors");
const status = (page: Page) => page.locator("#pg-status");

/** Counts every change to the status region, since rewriting even the same text can make a screen reader speak again. */
async function countStatusWrites(page: Page): Promise<() => Promise<number>> {
  await status(page).evaluate((node) => {
    (window as any).statusWrites = 0;
    new MutationObserver((records) => ((window as any).statusWrites += records.length)).observe(node, {
      childList: true,
      characterData: true,
      subtree: true,
    });
  });
  return () => page.evaluate(() => (window as any).statusWrites);
}
const editor = (page: Page) => page.getByRole("textbox", { name: "Definition" });

/** Opens the playground once its script has drawn the first results, so typing is not lost to the initial load. */
async function openPlayground(page: Page): Promise<void> {
  await page.goto("/playground/");
  await expect(results(page).getByRole("table")).toBeVisible();
}

async function replaceDefinition(page: Page, text: string): Promise<void> {
  await editor(page).click();
  await page.keyboard.press("ControlOrMeta+a");
  await page.keyboard.insertText(text);
}

async function currentDefinition(page: Page): Promise<Record<string, any>> {
  return JSON.parse((await editor(page).locator(".cm-line").allTextContents()).join("\n"));
}

test("the first example and the fixed example date are loaded", async ({ page }) => {
  await openPlayground(page);
  await expect(page.getByLabel("Example")).toHaveValue("nrf-53-week-years");
  await expect(page.getByLabel("Date (YYYY-MM-DD)")).toHaveValue("2024-02-03");
  await expect(results(page).getByRole("row", { name: /^quarter FY2023 Q4/ })).toBeVisible();
  await expect(page.getByRole("img", { name: "FY2023: 371 days, 53 weeks" })).toBeVisible();
});

test("entering a date shows where it falls, and the strip follows its fiscal year", async ({ page }) => {
  await openPlayground(page);
  const date = page.getByLabel("Date (YYYY-MM-DD)");
  await date.fill("2022-06-01");
  await expect(page.getByLabel("Fiscal year to draw")).toHaveValue("2022");
  await expect(page.getByRole("img", { name: "FY2022: 364 days, 52 weeks" })).toBeVisible();

  await date.fill("2024-02-03");
  await expect(page.getByLabel("Fiscal year to draw")).toHaveValue("2023");
  await expect(results(page).getByRole("row", { name: /^quarter FY2023 Q4 \[2023-10-29, 2024-02-04\) 2024-02-03 14$/ })).toBeVisible();
  await expect(results(page).getByRole("row", { name: /^week FY2023 W53 / })).toBeVisible();
});

test("a definition without extra_week shows the library's error with its path, marked in the editor", async ({ page }) => {
  await openPlayground(page);
  const definition = await currentDefinition(page);
  delete definition.subdivision.extra_week;
  await replaceDefinition(page, JSON.stringify(definition, null, 2));

  await expect(errors(page).getByText("EXTRA_WEEK_MISSING", { exact: true })).toBeVisible();
  const path = errors(page).getByRole("button", { name: "subdivision.extra_week" });
  await expect(path).toBeVisible();
  await expect(page.locator(".cm-fc-error")).toHaveText('"subdivision"');
  await expect(page.getByLabel("Example")).toHaveValue("");
  await expect(results(page).getByRole("table")).toHaveCount(0);
});

test("an invalid date shows the library's error for the date argument", async ({ page }) => {
  await openPlayground(page);
  await page.getByLabel("Date (YYYY-MM-DD)").fill("2024-02-30");
  await expect(errors(page).getByText("INVALID_DATE", { exact: true })).toBeVisible();
  await expect(page.getByLabel("Date (YYYY-MM-DD)")).toHaveAttribute("aria-invalid", "true");
});

test("the date field does not ask for a numeric keypad, which has no hyphen on iOS", async ({ page }) => {
  await openPlayground(page);
  await expect(page.getByLabel("Date (YYYY-MM-DD)")).not.toHaveAttribute("inputmode");
});

test("the error list is not a live region; a separate status line speaks the outcome", async ({ page }) => {
  await openPlayground(page);
  await expect(errors(page)).not.toHaveAttribute("aria-live");
  await expect(status(page)).toHaveText("");
});

test("typing that keeps the same errors neither changes the status nor redraws the error list", async ({ page }) => {
  await openPlayground(page);
  const definition = await currentDefinition(page);
  delete definition.subdivision.extra_week;
  await replaceDefinition(page, JSON.stringify(definition, null, 2));
  const announcement = "fiscalclock reported 1 error: EXTRA_WEEK_MISSING at subdivision.extra_week.";
  await expect(status(page)).toHaveText(announcement);
  const item = await errors(page).getByRole("listitem").elementHandle();
  const statusWrites = await countStatusWrites(page);

  // Each pause is longer than the 200 ms debounce, so every key leads to a render.
  for (const key of [" ", " ", "Enter", " "]) {
    await page.keyboard.press(key);
    await page.waitForTimeout(300);
  }
  await expect(status(page)).toHaveText(announcement);
  expect(await statusWrites()).toBe(0);
  expect(await item!.evaluate((node) => node.isConnected)).toBe(true);
});

test("a change of outcome is announced once, with the library's labels for where the date falls", async ({ page }) => {
  await openPlayground(page);
  const statusWrites = await countStatusWrites(page);
  await page.getByLabel("Date (YYYY-MM-DD)").selectText();
  await page.keyboard.type("2022-06-01", { delay: 30 });
  // NRF FY2022 starts on 2022-01-30 with 4-5-4 periods: 2022-06-01 is day 124, in May 29 to July 2.
  await expect(status(page)).toHaveText("2022-06-01 is in FY2022, FY2022 Q2, FY2022 P05, FY2022 W18.");
  await page.waitForTimeout(500);
  expect(await statusWrites()).toBe(1);
});

test("the state survives a reload through the URL fragment", async ({ page }) => {
  await openPlayground(page);
  await page.getByLabel("Example").selectOption("apple-10q-2023-12-30");
  await page.getByLabel("Date (YYYY-MM-DD)").fill("2023-12-30");
  const definition = await currentDefinition(page);
  await expect(page).toHaveURL(/#s=/);

  await page.reload();
  await expect(page.getByLabel("Date (YYYY-MM-DD)")).toHaveValue("2023-12-30");
  await expect(page.getByLabel("Example")).toHaveValue("apple-10q-2023-12-30");
  expect(await currentDefinition(page)).toEqual(definition);
});

test("examples have short labels, with the full description and its source under the select", async ({ page }) => {
  await openPlayground(page);
  const example = page.getByLabel("Example");
  await expect(example.locator("option")).toHaveText([
    "Your own definition",
    "NRF, 52/53-week year",
    "ABB, year from 1 January",
    "Apple, 52/53-week year",
    "Costco, 52/53-week year",
    "Kroger, 52/53-week year",
    "PGHH, year from 1 July",
    "PGHH, year from 1 April",
    "Reliance, year from 1 April",
    "Siemens, year from 1 April",
    "Siemens, year from 1 October",
  ]);
  await example.selectOption({ label: "Costco, 52/53-week year" });
  const source = page.locator("#pg-example-source");
  await expect(source).toContainText("Costco Wholesale Corporation 2017 Annual Report");
  await expect(source.getByRole("link", { name: "Source" })).toHaveAttribute("href", /q4cdn\.com/);
});

test("an unreadable fragment falls back to the first example with a notice", async ({ page }) => {
  await page.goto("/playground/#s=not-a-state");
  await expect(page.getByRole("status").filter({ hasText: "could not be read" })).toBeVisible();
  await expect(page.getByLabel("Example")).toHaveValue("nrf-53-week-years");
});

test("when the WebAssembly module cannot load, the results say so instead of loading forever", async ({ page }) => {
  await page.route("**/*.wasm", (route) => route.abort());
  await page.goto("/playground/");
  await expect(results(page)).toContainText("fiscalclock could not be loaded");
  await expect(results(page)).not.toContainText("Loading fiscalclock");
});

test("the keyboard can leave the editor with Tab", async ({ page }) => {
  await openPlayground(page);
  await editor(page).click();
  await expect(editor(page)).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.getByLabel("Date (YYYY-MM-DD)")).toBeFocused();
  await page.keyboard.press("Shift+Tab");
  await expect(editor(page)).toBeFocused();
});

for (const width of [1280, 375]) {
  test(`the results table fits its column without scrolling at ${width} pixels wide`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await openPlayground(page);
    await page.evaluate(() => document.fonts.ready);
    const box = page.locator(".pg-table-scroll");
    expect(await box.evaluate((el) => el.scrollWidth - el.clientWidth)).toBe(0);
    expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBe(width);
    const weeks = (await box.getByRole("columnheader", { name: "Weeks" }).boundingBox())!;
    expect(weeks.x + weeks.width).toBeLessThanOrEqual(width);
  });
}

for (const theme of THEMES) {
  test(`the playground has no WCAG 2.2 AA violations with results and with errors, in ${theme}`, async ({ page }) => {
    await storeTheme(page, theme);
    await openPlayground(page);
    expect((await new AxeBuilder({ page }).withTags(WCAG_TAGS).analyze()).violations).toEqual([]);

    await page.getByLabel("Date (YYYY-MM-DD)").fill("not a date");
    await expect(errors(page).getByRole("listitem")).toHaveCount(1);
    expect((await new AxeBuilder({ page }).withTags(WCAG_TAGS).analyze()).violations).toEqual([]);
  });
}
