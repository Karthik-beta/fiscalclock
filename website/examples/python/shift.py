from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

q4 = cal.period("quarter", 2023, 4)
for n in (1, -4):
    moved = cal.shift(q4, n)
    print(n, cal.format(moved), moved.start)
