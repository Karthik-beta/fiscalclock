import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const longYears = cal.years(2006, 2023).filter((year) => year.weeks === 53).map((year) => year.year);
console.log(...longYears);

const extra = cal.period("week", 2023, 53);
console.log(extra.start, extra.last_day, extra.has_extra_week);

for (const year of [2022, 2023]) {
  console.log(year, cal.period("period", year, 12).weeks);
}
