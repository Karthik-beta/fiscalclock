import sys
sys.path.insert(0, r"C:\Users\Admin\Desktop\fiscalclock\research\evidence")
import check_quotes as c
from pathlib import Path
out = Path(sys.argv[1])
for line in sys.stdin:
    line=line.strip()
    if not line: continue
    name, url = line.split(" ",1)
    text, prob = c.fetch(url)
    if text is None:
        print(name, "FAIL", prob); continue
    t = c.normalise(text)
    (out/f"{name}.txt").write_text(t, encoding="utf-8")
    blocked = [m for m in c.BLOCK_MARKERS if m in t]
    print(name, len(t), blocked)
