//! Compares the crate with the day-walking reference for rules with a `table`
//! subdivision, inside `table`, `fixed_date` and `week_based` years, under
//! every year-numbering rule: both must accept or refuse a rule alike, and
//! where both accept, `locate` must agree on every day the table covers and
//! return `OUT_OF_RANGE` on the days just outside it.
//!
//! The generator draws the listed dates with the crate's `Date` (they are
//! inputs, not expected values); the reference labels the days by walking.

mod common;
// Only the table reference is used here.
#[allow(dead_code)]
mod reference;

use common::WEEKDAYS;
use fiscalclock::{Date, ErrorCode, FiscalCalendar, Period};
use reference::table::Table;
use reference::{Civil, RefPeriod, Refused, Shape, Walk};
use serde_json::{Value, json};

const RULES: [&str; 3] = ["start", "end", "majority"];

/// A small deterministic generator (xorshift), so the cases are the same on every run.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// A number in `lo..=hi`.
    fn int(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
}

fn civil(d: Date) -> Civil {
    let (y, m, d) = d.ymd();
    Civil { y, m, d }
}

fn iso(d: Date) -> String {
    let (y, m, day) = d.ymd();
    format!("{y:04}-{m:02}-{day:02}")
}

/// Period boundaries for consecutive years `[years[i], years[i + 1])`, each
/// split into `count` periods at distinct random days.
fn periods_for(rng: &mut Rng, years: &[Date], count: usize) -> Vec<Date> {
    let mut out = Vec::new();
    for w in years.windows(2) {
        let len = w[0].days_until(w[1]) as i64;
        let mut cuts = vec![0i64];
        while cuts.len() < count {
            let c = rng.int(1, len - 1);
            if !cuts.contains(&c) {
                cuts.push(c);
            }
        }
        cuts.sort();
        out.extend(cuts.into_iter().map(|c| w[0].checked_add_days(c).unwrap()));
    }
    out.push(*years.last().unwrap());
    out
}

/// A random split of `count` periods into one to four quarters.
fn quarters_for(rng: &mut Rng, count: usize) -> Vec<u32> {
    let parts = rng.int(1, count.min(4) as i64) as usize;
    let mut sizes = vec![1u32; parts];
    for _ in parts..count {
        let i = rng.int(0, parts as i64 - 1) as usize;
        sizes[i] += 1;
    }
    sizes
}

struct Case {
    definition: Value,
    table: Table,
}

fn case(
    year: Value,
    starts_on: &'static str,
    years: &[Date],
    periods: &[Date],
    quarters: Vec<u32>,
) -> Case {
    let definition = json!({
        "spec_version": 1,
        "year": year,
        "week": {"starts_on": starts_on, "numbering": "contains_start"},
        "subdivision": {
            "type": "table",
            "boundaries": periods.iter().map(|&d| iso(d)).collect::<Vec<_>>(),
            "quarters": quarters,
        },
        "naming": common::naming("start"),
    });
    Case {
        definition,
        table: Table {
            starts_on,
            years: years.iter().map(|&d| civil(d)).collect(),
            periods: periods.iter().map(|&d| civil(d)).collect(),
            quarters,
        },
    }
}

fn cases(walk: &Walk) -> Vec<Case> {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let mut out = Vec::new();
    let first_day = Date::from_ymd(1901, 1, 1).unwrap();
    for i in 0..120usize {
        let starts_on = WEEKDAYS[i % 7];
        let count = rng.int(1, 13) as usize;
        let quarters = quarters_for(&mut rng, count);
        let n = rng.int(1, 8) as usize;
        match i % 3 {
            // A table year: listed years of any length from 300 to 420 days.
            0 => {
                let mut years = vec![first_day.checked_add_days(rng.int(0, 90_000)).unwrap()];
                for _ in 0..n {
                    let last = *years.last().unwrap();
                    years.push(last.checked_add_days(rng.int(300, 420)).unwrap());
                }
                let periods = periods_for(&mut rng, &years, count);
                let year = json!({"type": "table", "boundaries": years.iter().map(|&d| iso(d)).collect::<Vec<_>>()});
                out.push(case(year, starts_on, &years, &periods, quarters));
            }
            // A fixed_date year: its years are the listed ones in a window.
            1 => {
                let (month, day) = (rng.int(1, 12) as u32, rng.int(1, 28) as u32);
                let y0 = rng.int(1901, 2180) as i32;
                let years: Vec<Date> = (0..=n as i32)
                    .map(|k| Date::from_ymd(y0 + k, month, day).unwrap())
                    .collect();
                let periods = periods_for(&mut rng, &years, count);
                let year = json!({"type": "fixed_date", "start": {"month": month, "day": day}});
                out.push(case(year, starts_on, &years, &periods, quarters));
            }
            // A week_based year: the reference finds its year starts by scanning.
            _ => {
                let (month, edge_end) = (rng.int(1, 12) as u32, rng.next() % 2 == 0);
                let day = if rng.next() % 2 == 0 {
                    None
                } else {
                    Some(rng.int(1, 28) as u32)
                };
                let rule = ["last", "first", "nearest"][rng.int(0, 2) as usize];
                let shape = Shape::Week {
                    starts_on,
                    edge_end,
                    month,
                    day,
                    rule,
                    periods: vec![],
                    quarters: vec![],
                    extra: 1,
                };
                let y0 = rng.int(1902, 2180) as i32;
                let years: Vec<Date> = (0..=n as i32)
                    .map(|k| {
                        let start = reference::rule_year_start(walk, &shape, y0 + k).unwrap();
                        let c = walk_day(walk, start);
                        Date::from_ymd(c.y, c.m, c.d).unwrap()
                    })
                    .collect();
                let periods = periods_for(&mut rng, &years, count);
                let year = json!({
                    "type": "week_based",
                    "anchor": {
                        "edge": if edge_end { "end" } else { "start" },
                        "month": month,
                        "day": day.map_or(json!("last"), |d| json!(d)),
                        "rule": rule,
                    },
                });
                out.push(case(year, starts_on, &years, &periods, quarters));
            }
        }
    }
    out
}

fn walk_day(walk: &Walk, i: usize) -> Civil {
    walk.day(i)
}

fn as_ref(p: &Period) -> RefPeriod {
    RefPeriod {
        index: p.index,
        start: civil(p.start),
        end_exclusive: civil(p.end_exclusive),
        days: p.days,
        weeks: p.weeks,
        has_extra_week: p.has_extra_week,
    }
}

fn compare(cal: &FiscalCalendar, walk: &Walk, case: &Case, rule: &'static str, definition: &Value) {
    let expected = reference::table::compute(walk, &case.table, rule).unwrap();
    for want in &expected {
        let date = Date::from_ymd(want.date.y, want.date.m, want.date.d).unwrap();
        let l = cal.locate(date).unwrap();
        let got = (
            l.year.year,
            as_ref(&l.year),
            as_ref(&l.quarter),
            as_ref(&l.period),
            as_ref(&l.week),
            (
                l.day_of_year,
                l.day_of_quarter,
                l.day_of_period,
                l.day_of_week,
            ),
        );
        let exp = (
            want.year_number,
            want.year,
            want.quarter,
            want.period,
            want.week,
            (
                want.day_of_year,
                want.day_of_quarter,
                want.day_of_period,
                want.day_of_week,
            ),
        );
        assert_eq!(got, exp, "{date} under {definition}");
    }
    let first = case.table.periods[0];
    let end = *case.table.periods.last().unwrap();
    let before = Date::from_ymd(first.y, first.m, first.d)
        .unwrap()
        .checked_add_days(-1)
        .unwrap();
    for date in [before, Date::from_ymd(end.y, end.m, end.d).unwrap()] {
        let e = cal.locate(date).unwrap_err();
        assert_eq!(e.code, ErrorCode::OutOfRange, "{date} under {definition}");
    }
}

#[test]
fn table_calendars_match_the_day_walking_reference() {
    let walk = Walk::new();
    let all = cases(&walk);
    let (mut compared, mut refused) = (0, 0);
    for case in &all {
        for rule in RULES {
            let mut definition = case.definition.clone();
            definition["naming"]["schemes"]["fy"]["year_number"] = json!(rule);
            let got = FiscalCalendar::from_json(&definition.to_string());
            match (reference::table::numbers(&walk, &case.table, rule), got) {
                (Ok(_), Ok(cal)) => {
                    compare(&cal, &walk, case, rule, &definition);
                    compared += 1;
                }
                (Err(why), Err(errors)) => {
                    let code = match why {
                        Refused::Tie => ErrorCode::MajorityTie,
                        Refused::Collision => ErrorCode::YearNumberCollision,
                    };
                    assert_eq!(errors.as_slice()[0].code, code, "{definition}");
                    refused += 1;
                }
                (want, got) => panic!("reference {want:?}, crate {got:?}, under {definition}"),
            }
        }
    }
    eprintln!(
        "differential_table: {} definitions x 3 rules; {compared} compared day by day, {refused} refused alike",
        all.len()
    );
    assert!(compared >= 200, "{compared} compared");
}
