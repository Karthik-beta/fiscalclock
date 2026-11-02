import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const week = cal.period("week", 2023, 53);
const period = cal.parent(week);
const quarter = cal.parent(period);
const year = cal.parent(quarter);
console.log(cal.format(week), "->", cal.format(period), "->", cal.format(quarter), "->", cal.format(year));
