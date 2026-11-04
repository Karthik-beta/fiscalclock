import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

process.stdout.write(cal.dimensionCsv("2024-02-03", "2024-02-05"));
