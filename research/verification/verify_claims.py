"""Reproduce the date-arithmetic claims in the fiscalclock research documents.

Standard library only. Run with: python verify_claims.py
Exit status is 0 when every check passes and 1 otherwise.
"""

from __future__ import annotations

import sys
from datetime import date, timedelta

WEEKDAYS = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]

results: list[tuple[str, bool, str]] = []


def check(claim_id: str, ok: bool, detail: str) -> None:
    results.append((claim_id, ok, detail))


def last_day_of_month(year: int, month: int) -> date:
    if month == 12:
        return date(year, 12, 31)
    return date(year, month + 1, 1) - timedelta(days=1)


def weekday_on_or_before(d: date, weekday: int) -> date:
    return d - timedelta(days=(d.weekday() - weekday) % 7)


def weekday_nearest(d: date, weekday: int) -> date:
    before = weekday_on_or_before(d, weekday)
    return before if (d - before).days <= 3 else before + timedelta(days=7)


def year_end(year: int, month: int, weekday: int, rule: str) -> date:
    """End date of a 52/53-week year anchored on the last day of `month` in `year`."""
    target = last_day_of_month(year, month)
    if rule == "last":
        return weekday_on_or_before(target, weekday)
    if rule == "nearest":
        return weekday_nearest(target, weekday)
    raise ValueError(rule)


def long_years(month: int, weekday: int, rule: str, first: int, last: int) -> list[int]:
    """Years in [first, last] whose year (ending in that calendar year) has 371 days."""
    out = []
    for y in range(first, last + 1):
        length = (year_end(y, month, weekday, rule) - year_end(y - 1, month, weekday, rule)).days
        assert length in (364, 371), (month, weekday, rule, y, length)
        if length == 371:
            out.append(y)
    return out


def check_400_year_rule() -> None:
    windows = [(1601, 2000), (2001, 2400)]
    failures = []
    count = 0
    for rule in ("last", "nearest"):
        for month in range(1, 13):
            for weekday in range(7):
                count += 1
                for first, last in windows:
                    n = len(long_years(month, weekday, rule, first, last))
                    if n != 71:
                        failures.append((rule, MONTHS[month - 1], WEEKDAYS[weekday], first, last, n))
    check(
        "C01 400-year rule",
        count == 168 and not failures,
        f"{count} rules x {len(windows)} windows {windows}; every rule has 71 long years in each window"
        if not failures
        else f"failures: {failures[:5]}",
    )
    check(
        "C02 400 Gregorian years hold 20,871 weeks",
        (date(2400, 1, 1) - date(2000, 1, 1)).days == 20871 * 7,
        f"{(date(2400, 1, 1) - date(2000, 1, 1)).days} days = {(date(2400, 1, 1) - date(2000, 1, 1)).days / 7} weeks",
    )


def check_nrf() -> None:
    # NRF: year ends on the Saturday nearest 31 January; fiscal Y ends in January of Y + 1.
    def nrf_end(fy: int) -> date:
        return year_end(fy + 1, 1, 5, "nearest")

    long_fy = [fy for fy in range(2016, 2031) if (nrf_end(fy) - nrf_end(fy - 1)).days == 371]
    check("C03 NRF 53-week years 2016-2030", long_fy == [2017, 2023, 2028], f"53-week fiscal years: {long_fy}")

    start_2023 = nrf_end(2022) + timedelta(days=1)
    end_2023 = nrf_end(2023)
    check(
        "C04 NRF fiscal 2023 bounds",
        (start_2023, end_2023) == (date(2023, 1, 29), date(2024, 2, 3)),
        f"fiscal 2023 runs {start_2023} to {end_2023} ({(end_2023 - start_2023).days + 1} days)",
    )
    start_2024 = nrf_end(2023) + timedelta(days=1)
    check("C05 NRF fiscal 2024 start", start_2024 == date(2024, 2, 4), f"fiscal 2024 starts {start_2024}")

    back = start_2024 - timedelta(days=364)
    week = (back - start_2023).days // 7 + 1
    check(
        "C06 same_weekday example",
        back == date(2023, 2, 5) and week == 2,
        f"{start_2024} minus 364 days = {back}, fiscal week {week} of fiscal 2023",
    )
    pos = start_2024 - timedelta(days=371)
    check(
        "C07 same_position is 371 days after a 53-week year",
        pos == start_2023,
        f"week 1 day 1 of fiscal 2024 minus 371 days = {pos} (fiscal 2023 week 1 day 1 = {start_2023})",
    )

    start_2017 = nrf_end(2016) + timedelta(days=1)
    restated = start_2017 + timedelta(days=7)
    check(
        "C08 NRF 2017 restated first week",
        (start_2017, start_2017 + timedelta(days=6), restated, restated + timedelta(days=6))
        == (date(2017, 1, 29), date(2017, 2, 4), date(2017, 2, 5), date(2017, 2, 11)),
        f"fiscal week 1 = {start_2017}..{start_2017 + timedelta(days=6)}; "
        f"restated week 1 = {restated}..{restated + timedelta(days=6)}",
    )

    # NRF's 4-5-4 calendar page (https://nrf.com/resources/4-5-4-calendar, read 31 Oct 2026) says
    # "2006, 2012, 2017 and 2023 are all 53-week years." and "This anomaly has most recently
    # occurred in FY12, FY17 and FY23." The page lists no 53-week years after FY23.
    published = [2006, 2012, 2017, 2023]
    rule = [fy for fy in range(2006, 2024) if (nrf_end(fy) - nrf_end(fy - 1)).days == 371]
    check(
        "C19 NRF's published 53-week years 2006-2023 match the rule",
        rule == published,
        f"rule gives {rule}; NRF publishes {published}",
    )


def check_apple() -> None:
    # Apple: year ends on the last Saturday of September; named by end year.
    long_fy = long_years(9, 5, "last", 2016, 2030)
    check("C09 Apple 53-week years 2016-2030", long_fy == [2017, 2023, 2028], f"53-week fiscal years: {long_fy}")


def broadcast_month_end(year: int, month: int) -> date:
    return weekday_on_or_before(last_day_of_month(year, month), 6)


def check_broadcast() -> None:
    lines = []
    month_weeks: dict[int, list[int]] = {}
    quarter_weeks: dict[int, list[int]] = {}
    ok = True
    for y in range(2020, 2032):
        prev = broadcast_month_end(y - 1, 12)
        weeks = []
        for m in range(1, 13):
            end = broadcast_month_end(y, m)
            days = (end - prev).days
            ok &= days % 7 == 0 and days // 7 in (4, 5)
            weeks.append(days // 7)
            prev = end
        month_weeks[y] = weeks
        quarter_weeks[y] = [sum(weeks[i : i + 3]) for i in range(0, 12, 3)]
        lines.append(f"    {y}: months {'-'.join(map(str, weeks))} | quarters {quarter_weeks[y]} | year {sum(weeks)} weeks")
    check("C10 broadcast months are 4 or 5 weeks 2020-2031", ok, "\n" + "\n".join(lines))
    check(
        "C11 broadcast 2024 months run 4-4-5 in every quarter",
        all(month_weeks[2024][i : i + 3] == [4, 4, 5] for i in range(0, 12, 3)),
        f"2024 months: {month_weeks[2024]}",
    )
    check(
        "C12 broadcast 2021 Q1 months run 5-4-4",
        month_weeks[2021][:3] == [5, 4, 4],
        f"2021 months: {month_weeks[2021]}",
    )
    check(
        "C12a correction (informational): 2021 is not 5-4-4 in every quarter",
        not all(month_weeks[2021][i : i + 3] == [5, 4, 4] for i in range(0, 12, 3)),
        "the proposal's 'months run ... 5-4-4 in 2021' holds for Q1 and Q4 only; "
        f"2021 quarters run {[month_weeks[2021][i : i + 3] for i in range(0, 12, 3)]}",
    )
    check("C13 broadcast 2029 quarters 12-13-14-13", quarter_weeks[2029] == [12, 13, 14, 13], f"2029 quarters: {quarter_weeks[2029]}")
    qmin = min(min(q) for q in quarter_weeks.values())
    qmax = max(max(q) for q in quarter_weeks.values())
    check("C14 broadcast quarters run 12 to 14 weeks 2020-2031", (qmin, qmax) == (12, 14), f"quarter lengths range {qmin}..{qmax}")


def check_iso() -> None:
    mismatches = 0
    for y in range(1600, 2400):
        start = weekday_nearest(date(y, 1, 1), 0)
        next_start = weekday_nearest(date(y + 1, 1, 1), 0)
        if start.isocalendar()[:3] != (y, 1, 1):
            mismatches += 1
        if (next_start - timedelta(days=1)).isocalendar()[0] != y:
            mismatches += 1
        weeks = (next_start - start).days // 7
        if date(y, 12, 28).isocalendar()[1] != weeks:
            mismatches += 1
    check(
        "C15 ISO week year = week_based year starting Monday nearest 1 January, 1600-2399",
        mismatches == 0,
        f"800 years compared with date.isocalendar(); mismatches: {mismatches}",
    )


def check_care_com() -> None:
    # Year end: "the Saturday in December closest and prior to December 31", read as on or before 31 Dec.
    # Quarter end: "the Saturday that falls closest to the last day of the third month".
    def diverging(first: int, last: int, strict: bool) -> list[tuple[int, date, date]]:
        out = []
        for y in range(first, last + 1):
            anchor = date(y, 12, 30) if strict else date(y, 12, 31)
            year_rule = weekday_on_or_before(anchor, 5)
            quarter_rule = weekday_nearest(date(y, 12, 31), 5)
            if year_rule != quarter_rule:
                out.append((y, year_rule, quarter_rule))
        return out

    inclusive = diverging(2019, 2028, strict=False)
    years = [y for y, _, _ in inclusive]
    sample = {y: (a, b) for y, a, b in inclusive}
    check(
        "C16 Care.com Q4 ends diverge in 2020, 2021, 2025-2027 (window 2019-2028)",
        years == [2020, 2021, 2025, 2026, 2027] and sample[2025] == (date(2025, 12, 27), date(2026, 1, 3)),
        f"diverging years: {years}; 2025: year rule {sample[2025][0]}, quarter rule {sample[2025][1]}",
    )
    strict = [y for y, _, _ in diverging(2019, 2028, strict=True)]
    wide = [y for y, _, _ in diverging(2010, 2035, strict=False)]
    check(
        "C17 Care.com reading note (informational)",
        True,
        f"if 'prior to December 31' excludes 31 Dec, diverging years 2019-2028 are {strict}; "
        f"inclusive reading over 2010-2035: {wide}",
    )


def check_python_overflow() -> None:
    try:
        date(9999, 12, 31) + timedelta(days=1)
    except OverflowError as exc:
        check("C18 date(9999,12,31) + 1 day raises OverflowError", True, f"OverflowError: {exc}")
    else:
        check("C18 date(9999,12,31) + 1 day raises OverflowError", False, "no exception raised")


def main() -> int:
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    print(f"Python {sys.version.split()[0]}", file=sys.stderr)
    check_400_year_rule()
    check_nrf()
    check_apple()
    check_broadcast()
    check_iso()
    check_care_com()
    check_python_overflow()
    for claim_id, ok, detail in results:
        print(f"[{'PASS' if ok else 'FAIL'}] {claim_id}: {detail}")
    failed = [c for c, ok, _ in results if not ok]
    print(f"{len(results) - len(failed)} passed, {len(failed)} failed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
