from pathlib import Path

from fiscalclock import FiscalCalendar, FiscalclockError

cal = FiscalCalendar(Path("two-schemes.json").read_text(encoding="utf-8"))

try:
    cal.parse("FY2024")
except FiscalclockError as error:
    print(error.code)
    for candidate in error.details["candidates"]:
        period = candidate["period"]
        print(candidate["scheme"], period["year"], period["start"], period["last_day"])
