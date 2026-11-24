"""Write each conformance vector's definition to corpus/<target>/ as a seed.

Usage, from the repository root: python -I fuzz/seed_corpus.py
validate_json reads the files as JSON text. The other targets read their input
as choices, so any bytes seed them; the same files give them a start.
"""

import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parent
VECTORS = ROOT.parent / "conformance" / "vectors"

for target in ("validate_json", "validate_structured", "parse_label"):
    out = ROOT / "corpus" / target
    out.mkdir(parents=True, exist_ok=True)
    count = 0
    for path in sorted(VECTORS.rglob("*.json")):
        definition = json.loads(path.read_text(encoding="utf-8")).get("definition")
        if isinstance(definition, dict):
            text = json.dumps(definition, ensure_ascii=False, separators=(",", ":"))
            (out / f"{path.parent.name}-{path.stem}.json").write_text(text, encoding="utf-8")
            count += 1
    print(f"{target}: {count} seeds")
