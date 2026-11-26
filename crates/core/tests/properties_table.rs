//! Tiling, nesting and round-trip properties for rules with a `table`
//! subdivision. The 364/371-day and 400-year properties do not apply to them
//! (AGENTS.md, correctness rule 3).

mod common;

use common::WEEKDAYS;
use fiscalclock::{Date, FiscalCalendar, Level, Period};
use proptest::prelude::*;
use proptest::test_runner::TestCaseError;
use serde_json::{Value, json};

fn iso(d: Date) -> String {
    let (y, m, day) = d.ymd();
    format!("{y:04}-{m:02}-{day:02}")
}

/// A table calendar drawn from `seed`: a `table` year with years of 300 to
/// 420 days, or a `fixed_date` year, each with 1 to 13 listed periods a year.
fn table_definition() -> impl Strategy<Value = (Value, Vec<Date>)> {
    (any::<u64>(), any::<bool>(), 1..=6usize, 1..=13usize, 0..7usize, prop::sample::select(vec!["start", "end", "majority"]))
        .prop_map(|(seed, table_year, n, count, w, numbering)| {
            let mut state = seed | 1;
            let mut int = |lo: i64, hi: i64| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                lo + (state % (hi - lo + 1) as u64) as i64
            };
            let years: Vec<Date> = if table_year {
                let mut v = vec![Date::from_ymd(1950, 1, 1).unwrap().checked_add_days(int(0, 40_000)).unwrap()];
                for _ in 0..n {
                    let last = *v.last().unwrap();
                    v.push(last.checked_add_days(int(300, 420)).unwrap());
                }
                v
            } else {
                let (month, day, y0) = (int(1, 12) as u32, int(1, 28) as u32, int(1950, 2050) as i32);
                (0..=n as i32).map(|k| Date::from_ymd(y0 + k, month, day).unwrap()).collect()
            };
            let mut periods = Vec::new();
            for pair in years.windows(2) {
                let len = pair[0].days_until(pair[1]) as i64;
                let mut cuts = vec![0i64];
                while cuts.len() < count {
                    let c = int(1, len - 1);
                    if !cuts.contains(&c) {
                        cuts.push(c);
                    }
                }
                cuts.sort();
                periods.extend(cuts.into_iter().map(|c| pair[0].checked_add_days(c).unwrap()));
            }
            periods.push(*years.last().unwrap());
            let mut quarters = vec![1u32; count.min(4)];
            for k in quarters.len()..count {
                let i = k % quarters.len();
                quarters[i] += 1;
            }
            let year = if table_year {
                json!({"type": "table", "boundaries": years.iter().map(|&d| iso(d)).collect::<Vec<_>>()})
            } else {
                let (_, month, day) = years[0].ymd();
                json!({"type": "fixed_date", "start": {"month": month, "day": day}})
            };
            let definition = json!({
                "spec_version": 1,
                "year": year,
                "week": {"starts_on": WEEKDAYS[w], "numbering": "contains_start"},
                "subdivision": {"type": "table", "boundaries": periods.iter().map(|&d| iso(d)).collect::<Vec<_>>(), "quarters": quarters},
                "naming": common::naming(numbering),
            });
            (definition, years)
        })
}

/// A calendar and a day it covers, or a rejected case when validation refuses
/// the numbering rule.
fn build(definition: &Value) -> Result<FiscalCalendar, TestCaseError> {
    FiscalCalendar::from_json(&definition.to_string())
        .map_err(|_| TestCaseError::reject("definition refused by validation"))
}

fn day_in(years: &[Date], pick: u32) -> Date {
    let span = years[0].days_until(*years.last().unwrap()) as u32;
    years[0].checked_add_days(i64::from(pick % span)).unwrap()
}

fn assert_tiles(parent: &Period, children: &[Period]) -> Result<(), TestCaseError> {
    prop_assert!(!children.is_empty());
    prop_assert_eq!(children[0].start, parent.start);
    prop_assert_eq!(
        children[children.len() - 1].end_exclusive,
        parent.end_exclusive
    );
    for pair in children.windows(2) {
        prop_assert_eq!(pair[0].end_exclusive, pair[1].start);
        prop_assert_eq!(pair[0].index.map(|i| i + 1), pair[1].index);
    }
    for c in children {
        prop_assert!(c.days > 0);
        prop_assert_eq!(c.weeks, None);
    }
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 512, ..ProptestConfig::default() })]

    #[test]
    fn located_periods_nest_and_tile((definition, years) in table_definition(), pick in any::<u32>()) {
        let cal = build(&definition)?;
        let date = day_in(&years, pick);
        let loc = cal.locate(date).unwrap();
        for p in [&loc.year, &loc.quarter, &loc.period, &loc.week] {
            prop_assert!(p.start <= date && date < p.end_exclusive);
        }
        prop_assert!(loc.year.start <= loc.quarter.start && loc.quarter.end_exclusive <= loc.year.end_exclusive);
        prop_assert!(loc.quarter.start <= loc.period.start && loc.period.end_exclusive <= loc.quarter.end_exclusive);
        prop_assert!(years.contains(&loc.year.start) && years.contains(&loc.year.end_exclusive));
        prop_assert_eq!(cal.parent(&loc.week).unwrap(), loc.year);
        prop_assert_eq!(cal.parent(&loc.period).unwrap(), loc.quarter);
        for level in [Level::Quarter, Level::Period, Level::Week] {
            assert_tiles(&loc.year, &cal.children(&loc.year, level).unwrap())?;
        }
        assert_tiles(&loc.quarter, &cal.children(&loc.quarter, Level::Period).unwrap())?;
        prop_assert!(cal.children(&loc.period, Level::Week).is_err());
    }

    #[test]
    fn years_are_exactly_the_listed_ones((definition, years) in table_definition()) {
        let cal = build(&definition)?;
        let first = cal.locate(years[0]).unwrap().year.year;
        let all = cal.years(first, first + years.len() as i32 - 2).unwrap();
        let starts: Vec<Date> = all.iter().map(|y| y.start).chain(all.last().map(|y| y.end_exclusive)).collect();
        prop_assert_eq!(starts, years.clone());
        prop_assert!(cal.locate(years[0].checked_add_days(-1).unwrap()).is_err());
        prop_assert!(cal.locate(*years.last().unwrap()).is_err());
    }

    #[test]
    fn shift_and_parse_round_trip((definition, years) in table_definition(), pick in any::<u32>(), n in -30i64..=30) {
        let cal = build(&definition)?;
        let loc = cal.locate(day_in(&years, pick)).unwrap();
        for p in [loc.year, loc.quarter, loc.period, loc.week] {
            if let Ok(moved) = cal.shift(&p, n) {
                prop_assert_eq!(cal.shift(&moved, -n).unwrap(), p);
            }
            if let Ok(next) = cal.shift(&p, 1) {
                prop_assert_eq!(next.start, p.end_exclusive);
            }
            let label = cal.format(&p, None).unwrap();
            prop_assert_eq!(cal.parse(&label).unwrap(), p, "label {}", label);
        }
    }
}
