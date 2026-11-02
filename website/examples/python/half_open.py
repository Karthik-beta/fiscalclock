from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

quarters = cal.children(cal.period("year", 2023), "quarter")
for quarter in quarters:
    print(quarter.start, quarter.end_exclusive, quarter.days)

print(all(a.end_exclusive == b.start for a, b in zip(quarters, quarters[1:])))
