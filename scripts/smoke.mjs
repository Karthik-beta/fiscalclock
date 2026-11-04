// Smoke test for an installed fiscalclock npm package. Run from a directory
// where the package is installed: node path/to/smoke.mjs
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";

const require = createRequire(pathToFileURL(`${process.cwd()}/`));
const { FiscalCalendar, FiscalclockError, coreVersion } = await import(pathToFileURL(require.resolve("fiscalclock")).href);

const cal = new FiscalCalendar({
  spec_version: 1,
  year: { type: "week_based", anchor: { edge: "end", month: 1, day: "last", rule: "nearest" } },
  week: { starts_on: "sunday", numbering: "contains_start" },
  subdivision: { type: "week_pattern", periods: [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4], quarters: [3, 3, 3, 3], extra_week: { period: 12 } },
  naming: { default: "fy", schemes: { fy: { year_number: "start", templates: { year: "FY{yyyy}", quarter: "Q{q} FY{yyyy}" } } } },
});
const fy = cal.period("year", 2023);
assert.deepEqual([fy.start, fy.last_day, fy.weeks], ["2023-01-29", "2024-02-03", 53]);
assert.equal(cal.format(cal.locate("2024-02-03").quarter), "Q4 FY2023");
assert.throws(() => cal.locate(new Date()), (e) => e instanceof FiscalclockError && e.code === "TIMESTAMP_REJECTED");
console.log(`fiscalclock (core ${coreVersion()}): ok`);
