import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect, test } from "@playwright/test";

const examples = join(import.meta.dirname, "..", "examples");
const file = (path: string) => readFileSync(join(examples, path), "utf8").replace(/\r\n/g, "\n").trimEnd();

test("the quick start shows each language's tested example file and its expected output", async ({ page }) => {
  await page.goto("/start-here/quick-start/");
  for (const [tab, source, output] of [
    ["Python", "python/quick_start.py", "python/quick_start.out"],
    ["JavaScript", "js/quick_start.mjs", "js/quick_start.out"],
    ["Rust", "rust/src/bin/quick_start.rs", "rust/src/bin/quick_start.out"],
  ]) {
    await page.getByRole("tab", { name: tab }).first().click();
    const panel = page.getByRole("tabpanel").filter({ visible: true }).first();
    const blocks = await panel
      .locator("pre")
      .evaluateAll((pres) =>
        pres.map((pre) => [...pre.querySelectorAll(".ec-line")].map((line) => line.textContent!.replace(/\n/g, "")).join("\n").trimEnd()),
      );
    expect(blocks).toEqual([file(source), file(output)]);
  }
});
