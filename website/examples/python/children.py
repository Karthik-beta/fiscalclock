from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

q4 = cal.period("quarter", 2023, 4)
for period in cal.children(q4, "period"):
    print(cal.format(period), period.weeks, period.has_extra_week)
