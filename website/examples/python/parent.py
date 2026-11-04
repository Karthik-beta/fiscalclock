from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

week = cal.period("week", 2023, 53)
period = cal.parent(week)
quarter = cal.parent(period)
year = cal.parent(quarter)
print(cal.format(week), "->", cal.format(period), "->", cal.format(quarter), "->", cal.format(year))
