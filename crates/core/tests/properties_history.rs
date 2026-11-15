//! Properties of calendars that change (`exceptions.history`, 0.2.0): tiling,
//! nesting, `shift` round trips, `parse(format(p))` with `{yy_end}` templates,
//! and `locate` agreeing with `children`, across changes of rule.
//!
//! Definitions come from the generator in `reference/cases.rs`, which places
//! every change on year starts of the rules on either side, so each is valid
//! by construction. The 364/371-day and 400-year properties of rule-only
//! week-based calendars do not apply here and are not tested.

// The 0.1.0 reference functions are not used here.
#[allow(dead_code)]
mod reference;

use fiscalclock::{Date, ErrorCode, FiscalCalendar, Level, Period};
use proptest::prelude::*;
use proptest::test_runner::{Config, TestCaseError, TestRunner};
use reference::{Civil, Walk, cases, history};
use serde_json::json;

struct Built {
    cal: FiscalCalendar,
    /// Every transition start and `from`, to draw dates near the changes.
    changes: Vec<Date>,
}

fn to_date(c: Civil) -> Date {
    Date::from_ymd(c.y, c.m, c.d).unwrap()
}

/// Builds a generated definition under a numbering rule the reference
/// accepts. Rejects the case when no change fits near the drawn dates, when
/// no rule numbers the years, or when the crate refuses the numbering for
/// years outside the reference's window; fails on any other refusal.
fn build(walk: &Walk, seed: u64) -> Result<Built, TestCaseError> {
    let case = cases::history_case(walk, seed, None, None, None)
        .ok_or_else(|| TestCaseError::reject("no valid change near the drawn dates"))?;
    let rule = ["start", "end", "majority"]
        .into_iter()
        .find(|r| history::numbers(walk, &case.history, r).is_ok())
        .ok_or_else(|| TestCaseError::reject("no numbering rule numbers these years"))?;
    let mut definition = case.definition.clone();
    definition["naming"]["schemes"]["fy"]["year_number"] = json!(rule);
    let cal = match FiscalCalendar::from_json(&definition.to_string()) {
        Ok(cal) => cal,
        Err(errors)
            if errors.as_slice().iter().all(|e| {
                matches!(
                    e.code,
                    ErrorCode::YearNumberCollision | ErrorCode::MajorityTie
                )
            }) =>
        {
            return Err(TestCaseError::reject(
                "numbering refused beyond the reference's window",
            ));
        }
        Err(errors) => {
            return Err(TestCaseError::fail(format!("{errors}\n{definition}")));
        }
    };
    let mut changes = Vec::new();
    for e in &case.history.entries {
        if let Some(t) = &e.transition {
            changes.push(to_date(t.start));
        }
        changes.push(to_date(e.from));
    }
    Ok(Built { cal, changes })
}

/// A date: near a change (within 1,000 days) or anywhere in 1900-2200.
fn pick_date(b: &Built, near: bool, offset: i64, anywhere: u32) -> Date {
    if near {
        let c = b.changes[anywhere as usize % b.changes.len()];
        c.checked_add_days(offset).unwrap()
    } else {
        Date::from_ymd(1900, 1, 1)
            .unwrap()
            .checked_add_days(i64::from(anywhere))
            .unwrap()
    }
}

/// Runs `property` on generated calendars and dates; `n` is a shift amount.
fn run(cases: u32, property: impl Fn(&Built, Date, i64) -> Result<(), TestCaseError>) {
    let walk = Walk::new();
    // Each case generates a definition with the reference, which is slow
    // without optimisation: unoptimised builds run an eighth of the cases.
    let cases = if cfg!(debug_assertions) {
        cases / 8
    } else {
        cases
    };
    let mut runner = TestRunner::new(Config {
        cases,
        max_shrink_iters: 256,
        ..Config::default()
    });
    let strategy = (
        any::<u64>(),
        any::<bool>(),
        -1000i64..=1000,
        0u32..=109_937,
        -120i64..=120,
    );
    let result = runner.run(&strategy, |(seed, near, offset, anywhere, n)| {
        let built = build(&walk, seed)?;
        let date = pick_date(&built, near, offset, anywhere);
        property(&built, date, n)
    });
    if let Err(e) = result {
        panic!("{e}");
    }
}

fn contains(outer: &Period, inner: &Period) -> bool {
    outer.start <= inner.start && inner.end_exclusive <= outer.end_exclusive
}

fn assert_tiles(parent: &Period, children: &[Period]) -> Result<(), TestCaseError> {
    prop_assert!(!children.is_empty(), "{parent:?} has no children");
    prop_assert_eq!(children[0].start, parent.start);
    prop_assert_eq!(
        children[children.len() - 1].end_exclusive,
        parent.end_exclusive
    );
    if parent.level == Level::Year {
        prop_assert_eq!(children[0].index, Some(1));
    }
    for pair in children.windows(2) {
        prop_assert_eq!(
            pair[0].end_exclusive,
            pair[1].start,
            "gap or overlap in {:?}",
            parent
        );
        prop_assert_eq!(pair[0].index.map(|i| i + 1), pair[1].index);
    }
    for c in children {
        prop_assert_eq!(c.year, parent.year);
        prop_assert!(c.days > 0);
        prop_assert_eq!(c.days as i32, c.start.days_until(c.end_exclusive));
        // `weeks` is set exactly in week_pattern years, as days / 7.
        prop_assert_eq!(c.weeks, parent.weeks.map(|_| c.days / 7), "{:?}", c);
    }
    Ok(())
}

#[test]
fn years_tile_time_across_changes() {
    run(256, |b, date, _| {
        let cal = &b.cal;
        let year = cal.locate(date).unwrap().year;
        let seq: Vec<Period> = (-3..=3).map(|k| cal.shift(&year, k).unwrap()).collect();
        for pair in seq.windows(2) {
            prop_assert_eq!(pair[0].end_exclusive, pair[1].start);
            // Numbers strictly increase; a skip is allowed at a change.
            prop_assert!(pair[0].year < pair[1].year, "{:?}", pair);
        }
        for y in &seq {
            prop_assert_eq!(cal.locate(y.start).unwrap().year, *y);
            prop_assert_eq!(cal.locate(y.last_day()).unwrap().year, *y);
            prop_assert_eq!(cal.period(Level::Year, y.year, None).unwrap(), *y);
            // Only a 371-day year of a week-based regime has an extra week.
            if y.has_extra_week {
                prop_assert_eq!(y.days, 371);
            }
        }
        let listed = cal.years(seq[0].year, seq[6].year).unwrap();
        prop_assert_eq!(listed, seq);
        Ok(())
    });
}

#[test]
fn children_tile_their_parent_across_changes() {
    run(256, |b, date, _| {
        let cal = &b.cal;
        let year = cal.locate(date).unwrap().year;
        for level in [Level::Quarter, Level::Period, Level::Week] {
            assert_tiles(&year, &cal.children(&year, level).unwrap())?;
        }
        for q in cal.children(&year, Level::Quarter).unwrap() {
            assert_tiles(&q, &cal.children(&q, Level::Period).unwrap())?;
            let weeks = cal.children(&q, Level::Week);
            if year.weeks.is_some() {
                assert_tiles(&q, &weeks.unwrap())?;
                for p in cal.children(&q, Level::Period).unwrap() {
                    assert_tiles(&p, &cal.children(&p, Level::Week).unwrap())?;
                }
            } else {
                prop_assert_eq!(weeks.unwrap_err().code, ErrorCode::LevelNotNested);
            }
        }
        Ok(())
    });
}

#[test]
fn located_periods_nest_across_changes() {
    run(256, |b, date, _| {
        let cal = &b.cal;
        let loc = cal.locate(date).unwrap();
        for p in [&loc.year, &loc.quarter, &loc.period, &loc.week] {
            prop_assert!(
                p.start <= date && date < p.end_exclusive,
                "{:?} does not contain {:?}",
                p,
                date
            );
        }
        prop_assert!(contains(&loc.year, &loc.quarter));
        prop_assert!(contains(&loc.quarter, &loc.period));
        if loc.year.weeks.is_some() {
            prop_assert!(contains(&loc.period, &loc.week));
            prop_assert_eq!(cal.parent(&loc.week).unwrap(), loc.period);
        } else {
            prop_assert!(contains(&loc.year, &loc.week));
            prop_assert_eq!(cal.parent(&loc.week).unwrap(), loc.year);
        }
        prop_assert_eq!(cal.parent(&loc.period).unwrap(), loc.quarter);
        prop_assert_eq!(cal.parent(&loc.quarter).unwrap(), loc.year);
        prop_assert_eq!(loc.day_of_year as i32, loc.year.start.days_until(date) + 1);
        prop_assert_eq!(
            loc.day_of_quarter as i32,
            loc.quarter.start.days_until(date) + 1
        );
        prop_assert_eq!(
            loc.day_of_period as i32,
            loc.period.start.days_until(date) + 1
        );
        Ok(())
    });
}

#[test]
fn shift_round_trips_across_changes() {
    run(256, |b, date, n| {
        let cal = &b.cal;
        let loc = cal.locate(date).unwrap();
        for p in [loc.year, loc.quarter, loc.period, loc.week] {
            let moved = cal.shift(&p, n).unwrap();
            prop_assert_eq!(moved.level, p.level);
            prop_assert_eq!(cal.shift(&moved, -n).unwrap(), p, "shift by {}", n);
            prop_assert_eq!(cal.shift(&p, 1).unwrap().start, p.end_exclusive);
            prop_assert_eq!(cal.shift(&p, -1).unwrap().end_exclusive, p.start);
        }
        Ok(())
    });
}

#[test]
fn parse_inverts_format_with_yy_end() {
    run(256, |b, date, _| {
        let cal = &b.cal;
        let loc = cal.locate(date).unwrap();
        let yy_end = loc.year.last_day().ymd().0 % 100;
        prop_assert_eq!(
            cal.format(&loc.year, None).unwrap(),
            format!("FY{:04}-{:02}", loc.year.year, yy_end)
        );
        for p in [loc.year, loc.quarter, loc.period, loc.week] {
            let label = cal.format(&p, None).unwrap();
            if p.index.is_some_and(|i| i >= 100) {
                // Written in full, but parse reads at most two digits.
                prop_assert_eq!(
                    cal.parse(&label).unwrap_err().code,
                    ErrorCode::LabelNotRecognised
                );
            } else {
                prop_assert_eq!(cal.parse(&label).unwrap(), p, "label {}", label);
            }
        }
        Ok(())
    });
}

#[test]
fn locate_agrees_with_children_across_changes() {
    run(128, |b, date, _| {
        let cal = &b.cal;
        let loc = cal.locate(date).unwrap();
        let pick = |l: &fiscalclock::Location, level: Level| match level {
            Level::Year => l.year,
            Level::Quarter => l.quarter,
            Level::Period => l.period,
            Level::Week => l.week,
        };
        for level in [Level::Quarter, Level::Period, Level::Week] {
            let children = cal.children(&loc.year, level).unwrap();
            prop_assert!(children.contains(&pick(&loc, level)));
            for c in children {
                prop_assert_eq!(pick(&cal.locate(c.start).unwrap(), level), c);
                prop_assert_eq!(pick(&cal.locate(c.last_day()).unwrap(), level), c);
            }
        }
        Ok(())
    });
}
