import { readdirSync, readFileSync } from "node:fs";
import { join, relative, sep } from "node:path";
import type { Page } from "@playwright/test";

export const THEMES = ["light", "dark"] as const;
export type Theme = (typeof THEMES)[number];

const website = join(import.meta.dirname, "..");

/** URL path of every HTML page in the built site, so new pages are covered without editing tests. */
export function builtPages(): string[] {
  const dist = join(website, "dist");
  return readdirSync(dist, { recursive: true, encoding: "utf8" })
    .filter((file) => file.endsWith(".html"))
    .map((file) => "/" + relative(dist, join(dist, file)).split(sep).join("/"))
    .map((path) => path.replace(/index\.html$/, ""))
    .sort();
}

const MONTHS = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

/**
 * The example definitions the playground and the landing page offer, read from the files in
 * conformance/vectors/cited/ that both build their lists from. Each has the label its select
 * must show: the organisation, as the description spells it, then what kind of year it has.
 */
export function citedExamples(): { name: string; label: string }[] {
  const dir = join(website, "..", "conformance", "vectors", "cited");
  return readdirSync(dir)
    .filter((file) => file.endsWith(".json"))
    .map((file) => JSON.parse(readFileSync(join(dir, file), "utf8")) as { name: string; description: string; definition: { year: { type: string; start?: { month: number; day: number } } } })
    .sort((a, b) => Number(b.name.startsWith("nrf-")) - Number(a.name.startsWith("nrf-")) || a.name.localeCompare(b.name))
    .map(({ name, description, definition }) => {
      const word = name.split("-")[0]!;
      const organisation = description.match(new RegExp(`\\b${word}\\b`, "i"))![0];
      const { type, start } = definition.year;
      return { name, label: `${organisation}, ${type === "week_based" ? "52/53-week year" : `year from ${start!.day} ${MONTHS[start!.month - 1]}`}` };
    });
}

/** Resolved token values, read from the token build in design/, not from the copy the site uses. */
export function tokens(theme: Theme): Record<string, string> {
  const file = join(website, "..", "design", "tokens", "generated", `tokens.${theme}.json`);
  return JSON.parse(readFileSync(file, "utf8"));
}

/** Converts `#rrggbb` to the `rgb(r, g, b)` form that getComputedStyle returns. */
export function rgb(hex: string): string {
  const [r, g, b] = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16));
  return `rgb(${r}, ${g}, ${b})`;
}

/** Stores a theme choice the way Starlight's theme picker does, before any page script runs. */
export async function storeTheme(page: Page, theme: Theme): Promise<void> {
  await page.addInitScript((value) => localStorage.setItem("starlight-theme", value), theme);
}
