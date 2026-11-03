import { readFileSync } from "node:fs";
import { expect, test } from "@playwright/test";

interface VectorFile {
  cases: { op: string; args: Record<string, unknown>; expect_subset?: Record<string, unknown> }[];
}
const apple = JSON.parse(readFileSync(new URL("../../conformance/vectors/cited/apple-10q-2023-12-30.json", import.meta.url), "utf8")) as VectorFile;

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

test("at 1280 by 800 pixels the date lookup and its first answer fit on the first screen", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 800 });
  await page.goto("/");
  await page.evaluate(() => document.fonts.ready);
  const lookup = page.getByRole("form", { name: "Try a date" });
  const box = (await lookup.boundingBox())!;
  expect(box.y + box.height).toBeLessThan(800);
  await expect(lookup.getByRole("row", { name: /^quarter/i })).toContainText("[2023-10-29, 2024-02-04)");
});

test("changing the calendar answers in the browser, matching the cited filing", async ({ page }) => {
  const quarter = apple.cases.find((c) => c.op === "period" && c.args.level === "quarter" && c.args.year === 2024 && c.args.index === 1)!;
  const lastDay = quarter.expect_subset!.last_day as string;
  const endExclusive = new Date(Date.parse(lastDay) + 86_400_000).toISOString().slice(0, 10);

  const wasm: string[] = [];
  page.on("request", (request) => request.url().endsWith(".wasm") && wasm.push(request.url()));
  await page.goto("/", { waitUntil: "networkidle" });
  expect(wasm, "no WebAssembly before the visitor asks").toEqual([]);

  const lookup = page.getByRole("form", { name: "Try a date" });
  await lookup.getByLabel("Calendar").selectOption({ label: "Apple, 52/53-week year" });
  await lookup.getByLabel("Date (YYYY-MM-DD)").fill(lastDay);
  await expect(lookup.getByRole("row", { name: /^quarter/i })).toContainText(`, ${endExclusive})`);
  expect(wasm).toHaveLength(1);

  await lookup.getByLabel("Date (YYYY-MM-DD)").fill("2023-02-30");
  await expect(lookup.getByText("INVALID_DATE")).toBeVisible();
  await expect(lookup.getByLabel("Date (YYYY-MM-DD)")).toHaveAttribute("aria-invalid", "true");
});

for (const [width, shown] of [[375, true], [1280, false]] as const) {
  test(`the strip's scroll hint is ${shown ? "shown" : "hidden"} at ${width} pixels wide, where the strip ${shown ? "overflows" : "fits"}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 800 });
    for (const path of ["/", "/playground/"]) {
      await page.goto(path);
      const scroller = page.locator(".strip-scroll");
      await expect(scroller).toHaveCount(1);
      // The playground draws its strip again after it loads, so poll the current one.
      await expect.poll(() => scroller.evaluate((el) => el.scrollWidth > el.clientWidth)).toBe(shown);
      const hint = page.getByText("Scroll sideways to see the whole year.");
      if (shown) await expect(hint).toBeVisible();
      else await expect(hint).toBeHidden();
      await expect(scroller).toHaveAttribute("aria-label", /scrolls sideways$/);
    }
  });
}

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
