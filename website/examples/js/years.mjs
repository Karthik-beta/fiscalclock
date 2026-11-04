import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

for (const year of cal.years(2021, 2024)) {
  console.log(year.year, year.start, year.weeks);
}
