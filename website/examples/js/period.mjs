import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const q4 = cal.period("quarter", 2023, 4);
console.log(q4.start, q4.end_exclusive, q4.last_day, q4.days, q4.weeks, q4.has_extra_week);
