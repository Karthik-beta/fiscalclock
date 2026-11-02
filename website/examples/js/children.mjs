import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const q4 = cal.period("quarter", 2023, 4);
for (const period of cal.children(q4, "period")) {
  console.log(cal.format(period), period.weeks, period.has_extra_week);
}
