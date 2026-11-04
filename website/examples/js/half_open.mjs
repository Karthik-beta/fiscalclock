import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const quarters = cal.children(cal.period("year", 2023), "quarter");
for (const quarter of quarters) {
  console.log(quarter.start, quarter.end_exclusive, quarter.days);
}

console.log(quarters.slice(1).every((quarter, i) => quarters[i].end_exclusive === quarter.start));
