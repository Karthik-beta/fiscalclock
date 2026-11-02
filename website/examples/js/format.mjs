import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const week = cal.period("week", 2023, 53);
console.log(cal.format(week));
console.log(cal.format(cal.parent(week), "nrf"));
