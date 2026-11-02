import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const fy2023 = cal.period("year", 2023);
const days = cal.dimension(fy2023.start, fy2023.end_exclusive);
console.log(days.length);
for (const [label, rows] of Object.entries(Object.groupBy(days, (day) => day.label_nrf_quarter))) {
  console.log(label, rows.length);
}
