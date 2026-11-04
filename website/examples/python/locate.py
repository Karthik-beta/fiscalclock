from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

here = cal.locate("2024-02-03")
print(here.year.year, here.quarter.index, here.period.index, here.week.index)
print(here.day_of_year, here.day_of_quarter, here.day_of_period, here.day_of_week)
