mod common;

use common::{WEEKDAYS, fixed_date, week_based};
use fiscalclock::{Date, FiscalCalendar, Level, Period};
use proptest::prelude::*;
use proptest::test_runner::TestCaseError;
use serde_json::{Value, json};

const RULES: [&str; 3] = ["start", "end", "majority"];

fn patterns() -> Vec<(Vec<u32>, Vec<u32>)> {
    vec![
        (vec![4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4], vec![3, 3, 3, 3]),
        (vec![4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5], vec![3, 3, 3, 3]),
        (vec![5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4], vec![3, 3, 3, 3]),
        (vec![4; 13], vec![3, 3, 3, 4]),
        (vec![4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5], vec![2, 4, 6]),
    ]
}

fn week_based_definition() -> impl Strategy<Value = Value> {
    (
        0..7usize,
        any::<bool>(),
        1..=12u32,
        prop_oneof![Just(json!("last")), (1..=28u32).prop_map(|d| json!(d))],
        prop::sample::select(vec!["last", "first", "nearest"]),
        prop::sample::select(patterns()),
        any::<prop::sample::Index>(),
        prop::sample::select(RULES.to_vec()),
    )
        .prop_map(
            |(w, edge_end, month, day, rule, (periods, quarters), extra, numbering)| {
                let extra = extra.index(periods.len()) as u32 + 1;
                week_based(
                    WEEKDAYS[w],
                    if edge_end { "end" } else { "start" },
                    month,
                    day,
                    rule,
                    &periods,
                    &quarters,
                    extra,
                    numbering,
                )
            },
        )
}

fn fixed_date_definition() -> impl Strategy<Value = Value> {
    (
        1..=12u32,
        1..=31u32,
        any::<bool>(),
        0..7usize,
        prop::sample::select(vec![vec![3, 3, 3, 3], vec![1, 2, 3, 6], vec![12]]),
        prop::sample::select(RULES.to_vec()),
    )
        .prop_map(|(month, day, clamp, w, quarters, numbering)| {
            let policy = (day >= 29).then_some(if clamp { "clamp" } else { "next" });
            fixed_date(month, day, policy, WEEKDAYS[w], &quarters, numbering)
        })
}

fn any_definition() -> impl Strategy<Value = Value> {
    prop_oneof![week_based_definition(), fixed_date_definition()]
}

fn date_1900_to_2200() -> impl Strategy<Value = Date> {
    let first = Date::from_ymd(1900, 1, 1).unwrap();
    let span = first.days_until(Date::from_ymd(2200, 12, 31).unwrap());
    (0..=span).prop_map(move |n| first.checked_add_days(i64::from(n)).unwrap())
}

/// Builds the calendar, or rejects the generated case when validation refuses
/// it (for example a numbering rule that would skip a year number).
fn build(definition: &Value) -> Result<FiscalCalendar, TestCaseError> {
    FiscalCalendar::from_json(&definition.to_string())
        .map_err(|_| TestCaseError::reject("definition refused by validation"))
}

fn is_week_based(definition: &Value) -> bool {
    definition["year"]["type"] == "week_based"
}

fn assert_tiles(parent: &Period, children: &[Period]) -> Result<(), TestCaseError> {
    prop_assert!(!children.is_empty(), "{parent:?} has no children");
    prop_assert_eq!(children[0].start, parent.start);
    prop_assert_eq!(
        children[children.len() - 1].end_exclusive,
        parent.end_exclusive
    );
    // Indexes count within the fiscal year, so they run on without gaps but
    // start at 1 only for the children of a year.
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
        prop_assert_eq!(c.days as i32, c.start.days_until(c.end_exclusive));
        prop_assert!(c.days > 0);
    }
    Ok(())
}

fn contains(outer: &Period, inner: &Period) -> bool {
    outer.start <= inner.start && inner.end_exclusive <= outer.end_exclusive
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 512, ..ProptestConfig::default() })]

    #[test]
    fn located_periods_nest_and_contain_the_date(definition in any_definition(), date in date_1900_to_2200()) {
        let cal = build(&definition)?;
        let loc = cal.locate(date).unwrap();
        for p in [&loc.year, &loc.quarter, &loc.period, &loc.week] {
            prop_assert!(p.start <= date && date < p.end_exclusive, "{:?} does not contain {:?}", p, date);
        }
        prop_assert!(contains(&loc.year, &loc.quarter));
        prop_assert!(contains(&loc.quarter, &loc.period));
        if is_week_based(&definition) {
            prop_assert!(contains(&loc.period, &loc.week));
            prop_assert_eq!(cal.parent(&loc.week).unwrap(), loc.period);
        } else {
            prop_assert!(contains(&loc.year, &loc.week));
            prop_assert_eq!(cal.parent(&loc.week).unwrap(), loc.year);
        }
        prop_assert_eq!(cal.parent(&loc.period).unwrap(), loc.quarter);
        prop_assert_eq!(cal.parent(&loc.quarter).unwrap(), loc.year);
        prop_assert_eq!(loc.day_of_year as i32, loc.year.start.days_until(date) + 1);
        prop_assert_eq!(loc.day_of_quarter as i32, loc.quarter.start.days_until(date) + 1);
        prop_assert_eq!(loc.day_of_period as i32, loc.period.start.days_until(date) + 1);
    }

    #[test]
    fn children_tile_their_parent(definition in any_definition(), date in date_1900_to_2200()) {
        let cal = build(&definition)?;
        let loc = cal.locate(date).unwrap();
        for level in [Level::Quarter, Level::Period, Level::Week] {
            assert_tiles(&loc.year, &cal.children(&loc.year, level).unwrap())?;
        }
        assert_tiles(&loc.quarter, &cal.children(&loc.quarter, Level::Period).unwrap())?;
        if is_week_based(&definition) {
            assert_tiles(&loc.quarter, &cal.children(&loc.quarter, Level::Week).unwrap())?;
            assert_tiles(&loc.period, &cal.children(&loc.period, Level::Week).unwrap())?;
        }
    }

    #[test]
    fn consecutive_years_are_contiguous(definition in any_definition(), date in date_1900_to_2200()) {
        let cal = build(&definition)?;
        let year = cal.locate(date).unwrap().year.year;
        let years = cal.years(year - 2, year + 2).unwrap();
        prop_assert_eq!(years.len(), 5);
        for pair in years.windows(2) {
            prop_assert_eq!(pair[0].end_exclusive, pair[1].start);
            prop_assert_eq!(pair[0].year + 1, pair[1].year);
        }
    }

    #[test]
    fn shift_round_trips(definition in any_definition(), date in date_1900_to_2200(), n in -120i64..=120) {
        let cal = build(&definition)?;
        let loc = cal.locate(date).unwrap();
        for p in [loc.year, loc.quarter, loc.period, loc.week] {
            let moved = cal.shift(&p, n).unwrap();
            prop_assert_eq!(moved.level, p.level);
            prop_assert_eq!(cal.shift(&moved, -n).unwrap(), p);
            prop_assert_eq!(cal.shift(&p, 1).unwrap().start, p.end_exclusive);
        }
    }

    #[test]
    fn parse_inverts_format(definition in any_definition(), date in date_1900_to_2200()) {
        let cal = build(&definition)?;
        let loc = cal.locate(date).unwrap();
        for p in [loc.year, loc.quarter, loc.period, loc.week] {
            let label = cal.format(&p, None).unwrap();
            prop_assert_eq!(cal.parse(&label).unwrap(), p, "label {}", label);
        }
    }

    // Rule-only week-based years only: 400 Gregorian years are exactly 20,871
    // weeks, so any 400 consecutive years hold 400 x 52 + 71 weeks.
    #[test]
    fn week_based_years_have_364_or_371_days_and_71_long_years_in_400(
        definition in week_based_definition(),
        first in 1700i32..=1900,
    ) {
        let cal = build(&definition)?;
        let year = cal.locate(Date::from_ymd(first, 7, 1).unwrap()).unwrap().year.year;
        let years = cal.years(year, year + 399).unwrap();
        prop_assert_eq!(years.len(), 400);
        for y in &years {
            prop_assert!(y.days == 364 || y.days == 371, "{:?}", y);
            prop_assert_eq!(y.has_extra_week, y.days == 371);
            prop_assert_eq!(y.weeks, Some(y.days / 7));
        }
        prop_assert_eq!(years.iter().filter(|y| y.days == 371).count(), 71);
    }
}
