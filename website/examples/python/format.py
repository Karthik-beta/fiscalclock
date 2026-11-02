from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

week = cal.period("week", 2023, 53)
print(cal.format(week))
print(cal.format(cal.parent(week), "nrf"))
