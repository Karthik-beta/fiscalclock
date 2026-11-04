import { readFileSync } from "node:fs";
import { FiscalCalendar } from "fiscalclock";

const cal = new FiscalCalendar(readFileSync("retail-4-5-4.json", "utf8"));

for (const row of cal.dimension("2024-02-02", "2024-02-05")) {
  console.log(row.date, row.fiscal_year, row.quarter, row.period, row.week, row.label_nrf_week);
}
