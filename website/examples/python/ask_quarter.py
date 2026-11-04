from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

here = cal.locate("2024-02-03")
print(cal.format(here.quarter))
print(cal.call_json("locate", '{"date": "2024-02-03"}'))
