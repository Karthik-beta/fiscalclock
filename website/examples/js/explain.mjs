import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

const explanation = cal.explain("2024-02-03");
for (const step of explanation.steps) {
  console.log(`${step.level}: ${step.result}`);
}
