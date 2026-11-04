from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

week = cal.parse("FY2023 W53")
print(week.level, week.year, week.index, week.start, week.last_day, week.has_extra_week)
