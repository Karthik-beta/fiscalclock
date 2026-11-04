from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

print(cal.dimension_csv("2024-02-03", "2024-02-05"), end="")
