from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

explanation = cal.explain("2024-02-03")
for step in explanation["steps"]:
    print(f"{step['level']}: {step['result']}")
