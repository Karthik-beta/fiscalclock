import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const label = cal.format(cal.period("period", 2023, 12));
console.log(label);

const parsed = cal.parse(label);
console.log(parsed.level, parsed.year, parsed.index, parsed.start, parsed.last_day);
