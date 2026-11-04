import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

try {
  cal.locate(new Date(2024, 1, 3, 23, 30));
} catch (error) {
  console.log(error.code, error.path);
}

console.log(cal.format(cal.locate("2024-02-03").week));
