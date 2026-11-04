from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

for year in cal.years(2021, 2024):
    print(year.year, year.start, year.weeks)
