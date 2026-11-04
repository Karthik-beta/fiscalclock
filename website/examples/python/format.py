from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("two-schemes.json").read_text(encoding="utf-8"))

year = cal.period("year", 2023)
print(cal.format(year))
print(cal.format(year, "by_end"))
