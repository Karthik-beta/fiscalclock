import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const here = cal.locate("2024-02-03");
console.log(cal.format(here.year), "|", cal.format(here.quarter), "|", cal.format(here.week));

const fy2023 = cal.period("year", 2023);
console.log(fy2023.start, fy2023.end_exclusive, fy2023.weeks);
