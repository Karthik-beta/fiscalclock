import sys; sys.stdout.reconfigure(encoding="utf-8")
from pathlib import Path
d = Path(r"C:\Users\Admin\AppData\Local\Temp\claude\C--Users-Admin-Desktop-fiscalclock\4e154076-abdf-4126-ab33-630145647bdc\scratchpad\pages")
w = 220
for arg in sys.argv[1:]:
    name, needle = arg.split("|", 1)
    t = (d / f"{name}.txt").read_text(encoding="utf-8")
    i = t.find(needle); n = 0
    print(f"== {name}: {needle!r} count={t.count(needle)}")
    while i >= 0 and n < 3:
        print("   ..." + t[max(0, i - w): i + len(needle) + w] + "...")
        i = t.find(needle, i + 1); n += 1
