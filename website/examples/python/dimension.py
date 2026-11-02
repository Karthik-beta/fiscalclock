from pathlib import Path

from fiscalclock import FiscalCalendar

cal = FiscalCalendar(Path("retail-4-5-4.json").read_text(encoding="utf-8"))

for row in cal.dimension("2024-02-02", "2024-02-05"):
    print(row["date"], row["fiscal_year"], row["quarter"], row["period"], row["week"], row["label_nrf_week"])
