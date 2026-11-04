import { expect, test } from "@playwright/test";

test("Python is the first language tab a new visitor sees, and a choice persists", async ({ context }) => {
  const first = await context.newPage();
  await first.goto("/guides/style-check/");
  await expect(first.getByRole("tab", { name: "Python" })).toHaveAttribute("aria-selected", "true");

  await first.getByRole("tab", { name: "JavaScript" }).click();
  await expect(first.getByRole("tab", { name: "JavaScript" })).toHaveAttribute("aria-selected", "true");

  const next = await context.newPage();
  await next.goto("/guides/style-check/");
  await expect(next.getByRole("tab", { name: "JavaScript" })).toHaveAttribute("aria-selected", "true");
  await expect(next.getByRole("tab", { name: "Python" })).toHaveAttribute("aria-selected", "false");
});
