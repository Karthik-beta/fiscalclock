import { expect, test } from "@playwright/test";

test("the landing page draws fiscal 2023 to scale with its 12 periods and the extra week", async ({ page }) => {
  await page.goto("/");
  const strip = page.getByRole("img", { name: /^FY2023: 371 days, 53 weeks$/ });
  await expect(strip).toBeVisible();

  const description = await strip.evaluate((svg) => svg.querySelector("desc")!.textContent!);
  expect(description).toContain("Year [2023-01-29, 2024-02-04).");
  expect(description.match(/\bP\d+ \[/g)).toHaveLength(12);
  expect(description).toContain("P12 [2023-12-31, 2024-02-04), 5 weeks, including the extra week as its last week.");

  await expect(strip.getByText("extra week 53")).toBeVisible();
  await page.getByText("Every period's dates").click();
  await expect(page.locator(".strip-figure tbody tr")).toHaveCount(12);
});

test("at 1280 by 800 pixels the year strip starts on the first screen, beside a definition that is not wrapped", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 800 });
  await page.goto("/");
  await page.evaluate(() => document.fonts.ready);
  const strip = page.getByRole("img", { name: /^FY2023: / });
  expect((await strip.boundingBox())!.y).toBeLessThan(800);

  const code = page.locator(".landing-definition pre");
  const heights = await code.locator(".ec-line").evaluateAll((els) => els.map((el) => el.getBoundingClientRect().height));
  expect(new Set(heights).size, "every line of the definition is one row high").toBe(1);
  expect(JSON.parse((await code.locator(".ec-line").allTextContents()).join("\n"))).toMatchObject({ spec_version: 1, subdivision: { extra_week: { period: 12 } } });
});

test("the question's three snippets share one output block with the label and the core's JSON", async ({ page }) => {
  await page.goto("/");
  const output = page.locator("figure.raw-output", { has: page.getByText("Output", { exact: true }) });
  await expect(output).toHaveCount(1);
  const [label, json] = (await output.locator("pre").textContent())!.split("\n");
  expect(label).toBe("FY2023 Q4");
  expect(JSON.parse(json!)).toMatchObject({
    date: "2024-02-03",
    quarter: { start: "2023-10-29", end_exclusive: "2024-02-04", weeks: 14 },
  });
  for (const tab of ["Python", "JavaScript", "Rust"]) {
    await expect(page.getByRole("tab", { name: tab })).toHaveCount(2);
  }
});

test("only the playground loads the WebAssembly module and the editor", async ({ page }) => {
  const requested: string[] = [];
  page.on("request", (request) => requested.push(request.url()));
  await page.goto("/", { waitUntil: "networkidle" });
  expect(requested.filter((url) => url.endsWith(".wasm") || url.includes("playground"))).toEqual([]);
});
