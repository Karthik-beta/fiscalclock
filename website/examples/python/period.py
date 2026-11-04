from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

q4 = cal.period("quarter", 2023, 4)
print(q4.start, q4.end_exclusive, q4.last_day, q4.days, q4.weeks, q4.has_extra_week)
