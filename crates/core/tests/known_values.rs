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
