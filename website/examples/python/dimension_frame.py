import sqlite3
from pathlib import Path

import pandas as pd
from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

fy2023 = cal.period("year", 2023)
days = pd.DataFrame(cal.dimension(fy2023.start, fy2023.end_exclusive))
print(len(days))
print(days.groupby("label_nrf_quarter").size().to_string())

with sqlite3.connect(":memory:") as db:
    days.to_sql("fiscal_day", db, index=False)
    query = "SELECT date FROM fiscal_day WHERE week_is_extra_week ORDER BY date LIMIT 1"
    print(db.execute(query).fetchone()[0])
