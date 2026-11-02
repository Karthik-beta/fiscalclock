import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect, test } from "@playwright/test";

const SITE = "https://fiscalclock.karthik.nz";
const HOME_TITLE = "Compute any fiscal calendar from a JSON definition";
const definition = readFileSync(join(import.meta.dirname, "..", "src", "fixtures", "style-check", "definition.json"), "utf8");

test("llms.txt lists the home page's Markdown copy and leaves out the style check", async ({ request }) => {
  const response = await request.get("/llms.txt");
  expect(response.ok()).toBe(true);
  const text = await response.text();
  expect(text).toContain(`[${HOME_TITLE}](${SITE}/index.md)`);
  expect(text).toContain(`${SITE}/llms-full.txt`);
  expect(text).not.toContain("style-check");
});

for (const file of ["/llms-full.txt", "/llms-small.txt"]) {
  test(`${file} has the home page and not the style check`, async ({ request }) => {
    const text = await (await request.get(file)).text();
    expect(text).toContain(`# ${HOME_TITLE}\n`);
    expect(text).not.toContain("# Style check");
    expect(text).not.toContain("fiscalclock:page");
  });
}

test("llms-full.txt has a guide's example code", async ({ request }) => {
  const text = await (await request.get("/llms-full.txt")).text();
  const quickStart = text.slice(text.indexOf("# Quick start"));
  expect(quickStart).toContain("```python\n");
  expect(quickStart).toContain("from fiscalclock import FiscalCalendar");
});

test("the home page's Markdown copy opens with its title and description", async ({ request }) => {
  const text = await (await request.get("/index.md")).text();
  expect(text.startsWith(`# ${HOME_TITLE}\n\nfiscalclock is an open-source fiscal calendar engine`)).toBe(true);
});

test("a Markdown copy has the code from a <Code> import, the tab labels and no MDX", async ({ request }) => {
  const response = await request.get("/guides/style-check.md");
  expect(response.ok()).toBe(true);
  const text = await response.text();
  expect(text).toContain("*definition.json*\n\n```json\n" + definition.trimEnd() + "\n```");
  for (const label of ["Python", "JavaScript", "Rust"]) expect(text).toContain(`**${label}**\n\n\`\`\``);
  expect(text).not.toMatch(/<\/?(Tabs|TabItem|Code|Aside)\b/);
  expect(text).not.toContain("@astrojs/starlight/components");
  expect(text).not.toContain("?raw");
});

for (const [path, markdown] of [
  ["/", "/index.md"],
  ["/guides/style-check/", "/guides/style-check.md"],
]) {
  test(`${path} points to ${markdown} in its head`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator('head link[rel="alternate"][type="text/markdown"]')).toHaveAttribute("href", markdown);
  });
}
