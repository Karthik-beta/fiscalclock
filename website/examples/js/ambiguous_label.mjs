import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("two-schemes.json", "utf8"));

try {
  cal.parse("FY2024");
} catch (error) {
  console.log(error.code);
  for (const { scheme, period } of error.details.candidates) {
    console.log(scheme, period.year, period.start, period.last_day);
  }
}
