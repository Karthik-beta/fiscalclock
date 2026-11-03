import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("two-schemes.json", "utf8"));

const year = cal.period("year", 2023);
console.log(cal.format(year));
console.log(cal.format(year, "by_end"));
