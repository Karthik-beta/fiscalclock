from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

label = cal.format(cal.period("period", 2023, 12))
print(label)

parsed = cal.parse(label)
print(parsed.level, parsed.year, parsed.index, parsed.start, parsed.last_day)
