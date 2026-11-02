from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

long_years = [year.year for year in cal.years(2006, 2023) if year.weeks == 53]
print(*long_years)

extra = cal.period("week", 2023, 53)
print(extra.start, extra.last_day, extra.has_extra_week)

for year in (2022, 2023):
    print(year, cal.period("period", year, 12).weeks)
