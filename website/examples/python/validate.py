from pathlib import Path

from fiscalclock import validate

result = validate(Path("retail-4-5-4.json").read_text(encoding="utf-8"))
print(result["ok"], result["calendar_id"])
