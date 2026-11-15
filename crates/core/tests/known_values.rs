mod changes;
mod common;

use common::*;
use fiscalclock::{Date, ErrorCode, Level};
use serde_json::json;

#[test]
fn nrf_fiscal_2023_is_the_53_week_year_ending_2024_02_03() {
    let c = cal(&nrf());
    let y = c.period(Level::Year, 2023, None).unwrap();
    assert_eq!(y.start, d("2023-01-29"));
    assert_eq!(y.end_exclusive, d("2024-02-04"));
    assert_eq!(y.last_day(), d("2024-02-03"));
    assert_eq!((y.days, y.weeks, y.has_extra_week), (371, Some(53), true));
    assert_eq!(
        c.period(Level::Year, 2024, None).unwrap().start,
        d("2024-02-04")
    );
}

#[test]
fn nrf_quarter_4_of_2023_matches_the_api_example() {
    let c = cal(&nrf());
    let q = c.period(Level::Quarter, 2023, Some(4)).unwrap();
    assert_eq!(q.start, d("2023-10-29"));
    assert_eq!(q.end_exclusive, d("2024-02-04"));
    assert_eq!((q.days, q.weeks, q.has_extra_week), (98, Some(14), true));
}

fn long_years(def: &serde_json::Value) -> Vec<i32> {
    let c = cal(def);
    c.years(2016, 2030)
        .unwrap()
        .into_iter()
        .filter(|y| y.weeks == Some(53))
        .map(|y| y.year)
        .collect()
}

#[test]
fn nrf_and_apple_53_week_years_2016_to_2030() {
    assert_eq!(long_years(&nrf()), [2017, 2023, 2028]);
    assert_eq!(long_years(&apple()), [2017, 2023, 2028]);
}

#[test]
fn apple_fiscal_2023_ends_on_the_last_saturday_of_september() {
    let c = cal(&apple());
    let y = c.period(Level::Year, 2023, None).unwrap();
    assert_eq!(y.start, d("2022-09-25"));
    assert_eq!(y.last_day(), d("2023-09-30"));
}

#[test]
fn iso_week_year_matches_python_isocalendar() {
    // Values from Python's datetime.date.isocalendar().
    let known = [
        ("2020-12-31", 2020, 53, 4),
        ("2021-01-03", 2020, 53, 7),
        ("2021-01-04", 2021, 1, 1),
        ("2008-12-29", 2009, 1, 1),
        ("2010-01-03", 2009, 53, 7),
        ("2015-12-31", 2015, 53, 4),
        ("2016-01-03", 2015, 53, 7),
        ("2012-01-01", 2011, 52, 7),
        ("2012-01-02", 2012, 1, 1),
        ("2026-10-31", 2026, 44, 6),
        ("1900-01-01", 1900, 1, 1),
        ("2199-12-31", 2200, 1, 2),
    ];
    let c = cal(&iso());
    for (date, year, week, weekday) in known {
        let l = c.locate(d(date)).unwrap();
        assert_eq!(
            (l.year.year, l.week.index, l.day_of_week),
            (year, Some(week), weekday),
            "{date}"
        );
    }
}

/// ISO week of a date by the Thursday rule, written independently of the crate.
fn iso_by_thursday(date: Date) -> (i32, u32) {
    let thursday = date
        .checked_add_days(3 - i64::from(date.weekday().index()))
        .unwrap();
    let jan1 = Date::from_ymd(thursday.year(), 1, 1).unwrap();
    (thursday.year(), (jan1.days_until(thursday) / 7 + 1) as u32)
}

#[test]
fn iso_week_year_matches_the_thursday_rule_for_every_day_1900_to_2200() {
    let c = cal(&iso());
    let mut date = d("1900-01-01");
    while date <= d("2200-12-31") {
        let l = c.locate(date).unwrap();
        let (y, w) = iso_by_thursday(date);
        assert_eq!((l.year.year, l.week.index), (y, Some(w)), "{date}");
        assert_eq!(l.day_of_week, date.weekday().index() + 1);
        date = date.checked_add_days(1).unwrap();
    }
}

#[test]
fn years_touching_0001_01_01_or_9999_12_31_are_out_of_range() {
    let jan1 = cal(&fixed_date(1, 1, None, "monday", &[3, 3, 3, 3], "start"));
    for date in ["9999-12-31", "9999-06-01", "0001-01-01", "0001-12-31"] {
        assert_eq!(
            jan1.locate(d(date)).unwrap_err().code,
            ErrorCode::OutOfRange,
            "{date}"
        );
    }
    assert_eq!(jan1.locate(d("9998-12-31")).unwrap().year.year, 9998);
    assert_eq!(jan1.locate(d("0002-01-01")).unwrap().year.year, 2);
    assert_eq!(
        jan1.period(Level::Year, 9999, None).unwrap_err().code,
        ErrorCode::OutOfRange
    );

    let jan2 = cal(&fixed_date(1, 2, None, "monday", &[3, 3, 3, 3], "start"));
    assert_eq!(jan2.locate(d("0001-01-02")).unwrap().year.year, 1);
    assert_eq!(
        jan2.locate(d("0001-01-01")).unwrap_err().code,
        ErrorCode::OutOfRange
    );
    assert_eq!(jan2.locate(d("9999-01-01")).unwrap().year.year, 9998);
    assert_eq!(
        jan2.locate(d("9999-01-02")).unwrap_err().code,
        ErrorCode::OutOfRange
    );

    let n = cal(&nrf());
    assert_eq!(
        n.locate(d("9999-12-31")).unwrap_err().code,
        ErrorCode::OutOfRange
    );
}

#[test]
fn indian_fiscal_year_with_assessment_year_scheme() {
    let def = json!({
        "spec_version": 1,
        "year": {"type": "fixed_date", "start": {"month": 4, "day": 1}},
        "week": {"starts_on": "monday", "numbering": "contains_start"},
        "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]},
        "naming": {"default": "fy", "schemes": {
            "fy": {"year_number": "start", "templates": {"year": "FY {yyyy}-{yy_next}", "quarter": "Q{q} FY {yyyy}-{yy_next}"}},
            "ay": {"year_number": "start", "offset": 1, "templates": {"year": "AY {yyyy}-{yy_next}"}}
        }}
    });
    let c = cal(&def);
    let l = c.locate(d("2024-02-29")).unwrap();
    assert_eq!(l.year.year, 2023);
    assert_eq!(
        (l.year.start, l.year.last_day()),
        (d("2023-04-01"), d("2024-03-31"))
    );
    assert_eq!(l.quarter.index, Some(4));
    assert_eq!(l.period.index, Some(11));
    assert_eq!(c.format(&l.year, None).unwrap(), "FY 2023-24");
    assert_eq!(c.format(&l.quarter, None).unwrap(), "Q4 FY 2023-24");
    assert_eq!(c.format(&l.year, Some("ay")).unwrap(), "AY 2024-25");
    assert_eq!(c.parse("AY 2024-25").unwrap(), l.year);
    assert_eq!(c.parse("FY 2023-24").unwrap(), l.year);
    assert_eq!(
        c.parse("FY 2023-25").unwrap_err().code,
        ErrorCode::LabelNotRecognised
    );
}

#[test]
fn fixed_date_weeks_follow_contains_start() {
    // The year starts on Saturday 2024-06-01 and weeks start on Monday, so week 1 has two days.
    let c = cal(&fixed_date(6, 1, None, "monday", &[3, 3, 3, 3], "end"));
    let y = c.locate(d("2024-06-01")).unwrap().year;
    assert_eq!(y.year, 2025);
    let weeks = c.children(&y, Level::Week).unwrap();
    assert_eq!((weeks[0].start, weeks[0].days), (d("2024-06-01"), 2));
    assert_eq!((weeks[1].start, weeks[1].days), (d("2024-06-03"), 7));
    let last = weeks.last().unwrap();
    assert_eq!(last.last_day(), d("2025-05-31"));
    assert_eq!(weeks.len(), 53);
    assert!(weeks.iter().all(|w| w.weeks.is_none() && !w.has_extra_week));
}

#[test]
fn short_month_policies_shift_period_starts() {
    let clamp = cal(&fixed_date(
        1,
        31,
        Some("clamp"),
        "monday",
        &[3, 3, 3, 3],
        "start",
    ));
    let next = cal(&fixed_date(
        1,
        31,
        Some("next"),
        "monday",
        &[3, 3, 3, 3],
        "start",
    ));
    let p2c = clamp.period(Level::Period, 2023, Some(2)).unwrap();
    let p2n = next.period(Level::Period, 2023, Some(2)).unwrap();
    assert_eq!(
        (p2c.start, p2c.last_day()),
        (d("2023-02-28"), d("2023-03-30"))
    );
    assert_eq!(
        (p2n.start, p2n.last_day()),
        (d("2023-03-01"), d("2023-03-30"))
    );
    let p4n = next.period(Level::Period, 2024, Some(4)).unwrap();
    assert_eq!(p4n.start, d("2024-05-01"));
}

#[test]
fn extra_week_is_the_last_week_of_its_period() {
    let mut def = nrf();
    def["subdivision"]["extra_week"]["period"] = json!(3);
    let c = cal(&def);
    let p3 = c.period(Level::Period, 2023, Some(3)).unwrap();
    assert_eq!((p3.weeks, p3.has_extra_week), (Some(5), true));
    let weeks = c.children(&p3, Level::Week).unwrap();
    let flags: Vec<bool> = weeks.iter().map(|w| w.has_extra_week).collect();
    assert_eq!(flags, [false, false, false, false, true]);
    assert_eq!(weeks[4].index, Some(14));
    assert!(
        !c.period(Level::Period, 2023, Some(12))
            .unwrap()
            .has_extra_week
    );
    assert_eq!(
        c.period(Level::Week, 2023, Some(53)).unwrap().last_day(),
        d("2024-02-03")
    );
}

// ---- Calendars whose rule changes (0.2.0) ----
//
// Dates, lengths and transition labels below are stated in the filings quoted
// in research/filers/ (see the `changes` module for which file says what).
// Day counts are calendar arithmetic on those dates.

/// (year number, start, last day, days, quarters, periods, label) of each year.
type YearRow = (i32, &'static str, &'static str, u32, u32, u32, &'static str);

fn check_years(def: &serde_json::Value, rows: &[YearRow]) {
    let c = cal(def);
    for &(number, start, last, days, quarters, periods, label) in rows {
        let y = c.period(Level::Year, number, None).unwrap();
        assert_eq!(
            (y.start, y.last_day(), y.days),
            (d(start), d(last), days),
            "{number}"
        );
        assert_eq!(
            c.children(&y, Level::Quarter).unwrap().len() as u32,
            quarters,
            "{number}"
        );
        assert_eq!(
            c.children(&y, Level::Period).unwrap().len() as u32,
            periods,
            "{number}"
        );
        if !label.is_empty() {
            assert_eq!(c.format(&y, None).unwrap(), label, "{number}");
            assert_eq!(c.parse(label).unwrap(), y, "{label}");
        }
        assert!(!y.has_extra_week && y.weeks.is_none(), "{number}");
    }
}

#[test]
fn pg_hygiene_nine_month_year_2024_25() {
    // research/filers/pg-hygiene.md: "Financial Year 2024-25 is a 9 month
    // period from July 1, 2024 to March 31, 2025"; 2025-26 runs from
    // April 1, 2025 to March 31, 2026. The 2023-24 label is inferred there.
    check_years(
        &changes::pg(),
        &[
            (2023, "2023-07-01", "2024-06-30", 366, 4, 12, "2023-24"),
            (2024, "2024-07-01", "2025-03-31", 274, 3, 9, "2024-25"),
            (2025, "2025-04-01", "2026-03-31", 365, 4, 12, "2025-26"),
        ],
    );
    let c = cal(&changes::pg());
    let quarters: Vec<_> = c
        .children(&c.period(Level::Year, 2024, None).unwrap(), Level::Quarter)
        .unwrap()
        .into_iter()
        .map(|q| (q.start, q.last_day()))
        .collect();
    assert_eq!(
        quarters,
        [
            (d("2024-07-01"), d("2024-09-30")),
            (d("2024-10-01"), d("2024-12-31")),
            (d("2025-01-01"), d("2025-03-31")),
        ]
    );
}

#[test]
fn siemens_eighteen_month_fy_2024_26_skips_2025() {
    // research/filers/siemens-india.md: "FY 2024-26 was for a period of
    // eighteen months from October 1, 2024 to March 31, 2026, while the
    // previous financial year i.e. FY 2023-24 was for a period of twelve
    // months from October 1, 2023 to September 30, 2024", and the eighteen
    // months "comprised of six quarters". FY 2026-27 ends on March 31, 2027.
    check_years(
        &changes::siemens(),
        &[
            (2023, "2023-10-01", "2024-09-30", 366, 4, 12, "FY 2023-24"),
            (2024, "2024-10-01", "2026-03-31", 547, 6, 18, "FY 2024-26"),
            (2026, "2026-04-01", "2027-03-31", 365, 4, 12, "FY 2026-27"),
        ],
    );
    let c = cal(&changes::siemens());
    let e = c.period(Level::Year, 2025, None).unwrap_err();
    assert_eq!(
        (e.code, e.path.as_str()),
        (ErrorCode::PeriodNotFound, "year")
    );
    let q6 = c.period(Level::Quarter, 2024, Some(6)).unwrap();
    assert_eq!(
        (q6.start, q6.last_day()),
        (d("2026-01-01"), d("2026-03-31"))
    );
}

#[test]
fn linde_fifteen_month_period_2022_23() {
    // research/filers/linde-india.md: the Annual Report 2022-23 covers the
    // "15 months period from 1 January 2022 to 31 March 2023", and the
    // tentative calendar labels April 2023 to March 2024 "2023-24".
    check_years(
        &changes::linde(),
        &[
            (2021, "2021-01-01", "2021-12-31", 365, 4, 12, ""),
            (2022, "2022-01-01", "2023-03-31", 455, 5, 15, "2022-23"),
            (2023, "2023-04-01", "2024-03-31", 366, 4, 12, "2023-24"),
        ],
    );
}

#[test]
fn yy_end_agrees_with_yy_next_for_april_to_march_years() {
    let mut def = changes::fixed(4, "start", "{yyyy}-{yy_end}");
    def["naming"]["schemes"]["next"] = serde_json::json!({
        "year_number": "start", "templates": {"year": "{yyyy}-{yy_next}"}
    });
    let c = cal(&def);
    for y in c.years(1990, 2110).unwrap() {
        assert_eq!(
            c.format(&y, None).unwrap(),
            c.format(&y, Some("next")).unwrap()
        );
    }
}

#[test]
fn week_pattern_transition_between_week_based_regimes() {
    // Illustrative calendar (see `changes::retail_to_september`).
    let c = cal(&changes::retail_to_september());
    let years = c.years(2023, 2025).unwrap();
    let summary: Vec<_> = years
        .iter()
        .map(|y| (y.year, y.start, y.last_day(), y.weeks, y.has_extra_week))
        .collect();
    assert_eq!(
        summary,
        [
            (2023, d("2023-01-29"), d("2024-02-03"), Some(53), true),
            (2024, d("2024-02-04"), d("2024-09-28"), Some(34), false),
            (2025, d("2024-09-29"), d("2025-09-27"), Some(52), false),
        ]
    );
    let t = &years[1];
    let weeks = c.children(t, Level::Week).unwrap();
    assert_eq!(weeks.len(), 34);
    assert!(weeks.iter().all(|w| w.days == 7 && !w.has_extra_week));
    let periods = c.children(t, Level::Period).unwrap();
    assert_eq!(
        periods.iter().map(|p| p.weeks.unwrap()).collect::<Vec<_>>(),
        [4, 5, 4, 4, 5, 4, 4, 4]
    );
    assert_eq!(c.children(t, Level::Quarter).unwrap().len(), 3);
}

// ---- A transition of 9,994 years ----
//
// The format sets no limit on a transition's length, so a valid calendar can
// have one transition from 0003-01-01 to 9997-01-01 (both Wednesdays, 521,462
// weeks apart) between two January to December rules. Its subdivision then
// has about 120,000 fiscal months, or hundreds of thousands of runs of
// weeks. A lookup inside it must not cost more the more quarters or periods
// come before it: summing them for each of the 347,641 periods of the
// week-pattern test below would take tens of billions of steps. Most of each
// test's time is the 400-year dimension, which costs about the same on a
// calendar without exceptions.

const TRANSITION_START: &str = "0003-01-01";
const TRANSITION_MONTHS: u32 = 9994 * 12;
const TRANSITION_WEEKS: u32 = 521_462;

fn long_transition(subdivision: serde_json::Value, starts_on: &str) -> serde_json::Value {
    let mut def = changes::fixed(1, "start", "FY{yyyy}");
    def["week"]["starts_on"] = json!(starts_on);
    def["exceptions"] = json!({"history": [{
        "from": "9997-01-01",
        "transition": {"start": TRANSITION_START, "subdivision": subdivision},
        "year": def["year"].clone(),
        "subdivision": def["subdivision"].clone()
    }]});
    def
}

/// `days` days after `date`.
fn plus(date: Date, days: u32) -> Date {
    date.checked_add_days(i64::from(days)).unwrap()
}

/// The first day of calendar month `k` counted from January 0003 as 1.
fn transition_month(k: u32) -> Date {
    let m = k - 1;
    Date::from_ymd(3 + (m / 12) as i32, m % 12 + 1, 1).unwrap()
}

#[test]
fn a_calendar_months_transition_of_119_928_months() {
    let c = cal(&long_transition(
        json!({"type": "calendar_months", "month_start_day": 1,
               "quarters": vec![1; TRANSITION_MONTHS as usize]}),
        "monday",
    ));
    let year = c.period(Level::Year, 3, None).unwrap();
    assert_eq!(
        (year.start, year.end_exclusive),
        (d(TRANSITION_START), d("9997-01-01"))
    );
    // One-month quarters: quarter k and period k are both calendar month k.
    for level in [Level::Quarter, Level::Period] {
        let children = c.children(&year, level).unwrap();
        assert_eq!(children.len(), TRANSITION_MONTHS as usize);
        for k in [1, 2, 13, 24_254, TRANSITION_MONTHS - 1, TRANSITION_MONTHS] {
            let p = &children[k as usize - 1];
            assert_eq!(p.index, Some(k), "{level:?} {k}");
            assert_eq!(p.start, transition_month(k), "{level:?} {k}");
            assert_eq!(p.end_exclusive, transition_month(k + 1), "{level:?} {k}");
        }
    }
    // 400 years of the dimension inside the transition.
    let first = d("2000-01-01");
    let rows = c.dimension(first, d("2400-01-01")).unwrap();
    assert_eq!(rows.len(), 146_097);
    for (date, month, day) in [
        ("2000-01-01", (2000 - 3) * 12 + 1, 1),
        ("2024-02-29", (2024 - 3) * 12 + 2, 29),
        ("2399-12-31", (2399 - 3) * 12 + 12, 31),
    ] {
        let at = &rows[first.days_until(d(date)) as usize].location;
        assert_eq!(at.date, d(date));
        assert_eq!(at.year.year, 3, "{date}");
        assert_eq!(at.quarter.index, Some(month), "{date}");
        assert_eq!(at.period.index, Some(month), "{date}");
        assert_eq!(at.day_of_period, day, "{date}");
        assert_eq!(
            at.day_of_year,
            d(TRANSITION_START).days_until(d(date)) as u32 + 1
        );
    }
}

#[test]
fn a_week_pattern_transition_of_347_641_periods() {
    // Periods of 1 and 2 weeks in turn, then a last period of 2 weeks:
    // 173,820 pairs of 3 weeks and 2 weeks more make 521,462 weeks.
    // Quarters of three periods, then a last quarter of one.
    let pairs = 173_820;
    let periods_n = 2 * pairs + 1;
    let mut periods = [1, 2].repeat(pairs as usize);
    periods.push(2);
    let mut quarters = vec![3; pairs as usize * 2 / 3];
    quarters.push(1);
    let c = cal(&long_transition(
        json!({"type": "week_pattern", "periods": periods, "quarters": quarters}),
        "wednesday",
    ));
    // Weeks before period k and weeks in it, from the pattern above.
    let period = |k: u32| -> (u32, u32) {
        let j = (k - 1) / 2;
        match k {
            _ if k == periods_n => (3 * pairs, 2),
            _ if k % 2 == 1 => (3 * j, 1),
            _ => (3 * j + 1, 2),
        }
    };
    let start = d(TRANSITION_START);
    let week_start = |w: u32| plus(start, 7 * w);
    let year = c.period(Level::Year, 3, None).unwrap();
    assert_eq!(year.weeks, Some(TRANSITION_WEEKS));

    let children = c.children(&year, Level::Period).unwrap();
    assert_eq!(children.len(), periods_n as usize);
    for k in [1, 2, 3, 4, 200_001, periods_n - 1, periods_n] {
        let (before, own) = period(k);
        let p = &children[k as usize - 1];
        assert_eq!(
            (p.start, p.end_exclusive, p.weeks),
            (week_start(before), week_start(before + own), Some(own)),
            "period {k}"
        );
        assert_eq!(c.parent(p).unwrap().index, Some(k.div_ceil(3).min(115_881)));
    }

    let children = c.children(&year, Level::Quarter).unwrap();
    assert_eq!(children.len(), 115_881);
    for q in [1, 2, 100_000, 115_880, 115_881] {
        let (first, last) = (3 * q - 2, (3 * q).min(periods_n));
        let p = &children[q as usize - 1];
        assert_eq!(p.start, week_start(period(first).0), "quarter {q}");
        let (before, own) = period(last);
        assert_eq!(p.end_exclusive, week_start(before + own), "quarter {q}");
    }

    let weeks = c.children(&year, Level::Week).unwrap();
    assert_eq!(weeks.len(), TRANSITION_WEEKS as usize);
    assert_eq!(
        weeks[TRANSITION_WEEKS as usize - 1].start,
        week_start(TRANSITION_WEEKS - 1)
    );

    // 400 years of the dimension inside the transition.
    let first = d("2000-01-01");
    let rows = c.dimension(first, d("2400-01-01")).unwrap();
    assert_eq!(rows.len(), 146_097);
    for date in ["2000-01-01", "2024-02-29", "2399-12-31"] {
        let at = &rows[first.days_until(d(date)) as usize].location;
        let w = start.days_until(d(date)) as u32 / 7;
        // Week w (from 0) is the first week of period 2j + 1 or within
        // period 2j + 2 of its run of three.
        let k = if w % 3 == 0 {
            2 * (w / 3) + 1
        } else {
            2 * (w / 3) + 2
        };
        assert_eq!(at.week.index, Some(w + 1), "{date}");
        assert_eq!(at.period.index, Some(k), "{date}");
        assert_eq!(at.quarter.index, Some(k.div_ceil(3)), "{date}");
        assert_eq!(at.period.start, week_start(period(k).0), "{date}");
    }
}
