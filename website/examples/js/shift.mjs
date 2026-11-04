import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const q4 = cal.period("quarter", 2023, 4);
for (const n of [1, -4]) {
  const moved = cal.shift(q4, n);
  console.log(n, cal.format(moved), moved.start);
}
