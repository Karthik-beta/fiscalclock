from pathlib import Path

from fiscalclock import validate

result = validate(Path("missing-choices.json").read_text(encoding="utf-8"))
for error in result["errors"]:
    print(error["path"], error["code"])
