import datetime as dt
from pathlib import Path

from fiscalclock import FiscalCalendar, FiscalclockError

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

try:
    cal.locate(dt.datetime(2024, 2, 3, 23, 30))
except FiscalclockError as error:
    print(error.code, error.path)

print(cal.format(cal.locate(dt.date(2024, 2, 3)).week))
