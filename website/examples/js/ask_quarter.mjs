import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const here = cal.locate("2024-02-03");
console.log(cal.format(here.quarter));
console.log(cal.callJson("locate", '{"date": "2024-02-03"}'));
