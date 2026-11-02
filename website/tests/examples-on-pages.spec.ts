import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect, test } from "@playwright/test";

const examples = join(import.meta.dirname, "..", "examples");
const file = (path: string) => readFileSync(join(examples, path), "utf8").replace(/\r\n/g, "\n").trimEnd();

test("the landing page's question shows each language's tested example file above their shared expected output", async ({ page }) => {
  await page.goto("/");
  const question = page.locator(".landing-prose");
  for (const [tab, source, output] of [
    ["Python", "python/ask_quarter.py", "python/ask_quarter.out"],
    ["JavaScript", "js/ask_quarter.mjs", "js/ask_quarter.out"],
    ["Rust", "rust/src/bin/ask_quarter.rs", "rust/src/bin/ask_quarter.out"],
  ]) {
    await question.getByRole("tab", { name: tab }).click();
    const code = await question
      .getByRole("tabpanel")
      .filter({ visible: true })
      .locator("pre .ec-line")
      .evaluateAll((lines) => lines.map((line) => line.textContent!.replace(/\n/g, "")).join("\n").trimEnd());
    expect(code).toBe(file(source));
    expect((await question.locator("figure.raw-output pre").textContent())!.trimEnd()).toBe(file(output));
  }
});

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
