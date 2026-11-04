from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

here = cal.locate("2024-02-03")
print(cal.format(here.year), "|", cal.format(here.quarter), "|", cal.format(here.week))

fy2023 = cal.period("year", 2023)
print(fy2023.start, fy2023.end_exclusive, fy2023.weeks)
