import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const here = cal.locate("2024-02-03");
console.log(here.year.year, here.quarter.index, here.period.index, here.week.index);
console.log(here.day_of_year, here.day_of_quarter, here.day_of_period, here.day_of_week);
