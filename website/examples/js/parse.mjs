import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const week = cal.parse("FY2023 W53");
console.log(week.level, week.year, week.index, week.start, week.last_day, week.has_extra_week);
