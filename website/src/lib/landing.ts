/**
 * Everything the landing page shows about fiscal 2023, computed at build time
 * by the pinned fiscalclock npm package.
 *
 * AGENTS.md forbids using fiscalclock's output as expected values in tests.
 * This is the other direction: the output is only drawn, and the build fails
 * unless it agrees with values that did not come from fiscalclock (NRF's
 * published 53-week years, and the independent oracle in conformance/).
 */
import { FiscalCalendar, type Period } from "fiscalclock";
import oracle from "../../../conformance/vectors/oracle/nrf-4-5-4.json";
import retail454 from "../../../docs/examples/retail-4-5-4.json";
import askPy from "../../examples/python/ask_quarter.out?raw";
import askJs from "../../examples/js/ask_quarter.out?raw";
import askRs from "../../examples/rust/src/bin/ask_quarter.out?raw";
import { nrf, type VectorFile } from "./examples";
import { formatJsonBlocks } from "./json-format";

const definition = nrf.definition;
// 70 columns fit the definition's column at 1280 pixels wide without wrapping.
export const definitionText = formatJsonBlocks(definition, 70);
export const sourceUrl = nrf.source.url!;

const cal = new FiscalCalendar(definition);
export const year = cal.period("year", 2023);
export const quarters = cal.children(year, "quarter");
export const periods = cal.children(year, "period");
export const yearLabel = cal.format(year);

// The question's snippets are the ask_quarter example, which `npm run test:examples`
// runs in every language; the page shows their one shared expected output.
const askOutputs = { python: askPy, javascript: askJs, rust: askRs };
export const askOutput = askPy.replace(/\r\n/g, "\n").replace(/\n$/, "");

// Counted as the README counts them: every vector file with a cases array.
const vectors = Object.values(
  import.meta.glob<{ cases?: unknown[] }>("../../../conformance/vectors/**/*.json", { eager: true, import: "default" }),
).filter((v) => Array.isArray(v.cases));
export const vectorFiles = vectors.length.toLocaleString("en");
export const vectorCases = vectors.reduce((n, v) => n + v.cases!.length, 0).toLocaleString("en");

crossCheck();

function crossCheck(): void {
  const problems: string[] = [];
  const same = (what: string, got: unknown, want: unknown) => {
    if (JSON.stringify(got) !== JSON.stringify(want)) problems.push(`${what}: fiscalclock gave ${JSON.stringify(got)}, expected ${JSON.stringify(want)}`);
  };

  // NRF: "2006, 2012, 2017 and 2023 are all 53-week years."
  const cited = nrf.cases.find((c) => c.op === "period" && c.args.level === "year" && c.args.year === 2023)?.expect_subset;
  if (!cited) problems.push("the cited NRF vector no longer has a case for fiscal 2023");
  else for (const [key, want] of Object.entries(cited)) same(`fiscal 2023 ${key} (cited, NRF)`, year[key as keyof Period], want);

  // The oracle's dates hold only if it describes the same calendar.
  const o = oracle as unknown as VectorFile;
  for (const block of ["year", "week", "subdivision"]) same(`oracle definition's ${block} block`, o.definition[block], definition[block]);

  const ours = [year, ...quarters, ...periods];
  for (const c of o.cases) {
    if (c.op !== "period" || c.args.year !== 2023) continue;
    const want = c.expect as Period;
    const got = ours.find((p) => p.level === want.level && p.index === want.index);
    for (const key of ["start", "end_exclusive", "last_day", "days", "weeks", "has_extra_week"] as const) {
      same(`fiscal 2023 ${want.level} ${want.index ?? ""} ${key} (oracle)`, got?.[key], want[key]);
    }
  }

  // The page prints definitionText, so it must be the definition the dates came from.
  if (JSON.stringify(JSON.parse(definitionText)) !== JSON.stringify(definition)) problems.push("the printed definition differs from the cited vector's definition");

  // The examples read docs/examples/retail-4-5-4.json, which the page tells readers to save from the printed definition.
  if (JSON.stringify(retail454) !== JSON.stringify(definition)) problems.push("docs/examples/retail-4-5-4.json differs from the cited vector's definition");

  // The page says all three languages print the same lines, and shows them once.
  const here = cal.locate("2024-02-03");
  const live = `${cal.format(here.quarter)}\n${cal.callJson("locate", '{"date": "2024-02-03"}')}`;
  for (const [language, out] of Object.entries(askOutputs)) same(`locate 2024-02-03 (${language}'s ask_quarter.out)`, live, out.replace(/\r\n/g, "\n").replace(/\n$/, ""));

  // The 4-5-4 pattern with the extra week in period 12, as drawn by design/scripts/build_brand_assets.py.
  same("period lengths in weeks", periods.map((p) => p.weeks), [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 5]);

  if (problems.length > 0) {
    throw new Error(`The landing page's fiscal 2023 does not match its sources:\n  ${problems.join("\n  ")}`);
  }
}
