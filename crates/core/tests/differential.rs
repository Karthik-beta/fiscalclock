//! Compares the crate with the day-walking reference across the generated
//! definitions under every year-numbering rule: both must accept or refuse the
//! rule alike, and where both accept, `locate` must agree for every date from
//! 1900-01-01 to 2200-12-31.

mod common;
mod reference;

use std::sync::atomic::{AtomicUsize, Ordering};

use common::*;
use fiscalclock::{Date, ErrorCode, FiscalCalendar, Period};
use reference::{Civil, RefPeriod, Refused, Shape, Walk};
use serde_json::{Value, json};

struct Case {
    definition: Value,
    shape: Shape,
}

const RULES: [&str; 3] = ["start", "end", "majority"];

fn patterns() -> Vec<(Vec<u32>, Vec<u32>)> {
    vec![
        (P454.to_vec(), vec![3, 3, 3, 3]),
        (P445.to_vec(), vec![3, 3, 3, 3]),
        (vec![5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4], vec![3, 3, 3, 3]),
        (vec![4; 13], vec![3, 3, 3, 4]),
        (vec![4; 13], vec![4, 3, 3, 3]),
        (vec![4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5], vec![2, 4, 6]),
    ]
}

fn cases() -> Vec<Case> {
    let mut out = Vec::new();
    let patterns = patterns();
    let mut i = 0usize;
    for (wi, starts_on) in WEEKDAYS.into_iter().enumerate() {
        for month in 1..=12u32 {
            for rule in ["last", "first", "nearest"] {
                for edge_end in [true, false] {
                    let day = match i % 4 {
                        0 => None,
                        1 => Some(1),
                        2 => Some(15),
                        _ => Some(28),
                    };
                    let (periods, quarters) = patterns[i % patterns.len()].clone();
                    let extra = (i * 5 + wi) as u32 % periods.len() as u32 + 1;
                    let definition = week_based(
                        starts_on,
                        if edge_end { "end" } else { "start" },
                        month,
                        day.map_or(json!("last"), |d| json!(d)),
                        rule,
                        &periods,
                        &quarters,
                        extra,
                        "start",
                    );
                    out.push(Case {
                        definition,
                        shape: Shape::Week {
                            starts_on,
                            edge_end,
                            month,
                            day,
                            rule,
                            periods,
                            quarters,
                            extra,
                        },
                    });
                    i += 1;
                }
            }
        }
    }
    // Anchors next to 1 January put dates two key years away from their
    // calendar year, so January and December get every day choice.
    for (wi, starts_on) in WEEKDAYS.into_iter().enumerate() {
        for month in [1u32, 12] {
            for rule in ["last", "first", "nearest"] {
                for edge_end in [true, false] {
                    for day in [None, Some(1), Some(15), Some(28)] {
                        let (periods, quarters) = patterns[(i + wi) % patterns.len()].clone();
                        let extra = (i * 3 + wi) as u32 % periods.len() as u32 + 1;
                        out.push(Case {
                            definition: week_based(
                                starts_on,
                                if edge_end { "end" } else { "start" },
                                month,
                                day.map_or(json!("last"), |d| json!(d)),
                                rule,
                                &periods,
                                &quarters,
                                extra,
                                "start",
                            ),
                            shape: Shape::Week {
                                starts_on,
                                edge_end,
                                month,
                                day,
                                rule,
                                periods,
                                quarters,
                                extra,
                            },
                        });
                        i += 1;
                    }
                }
            }
        }
    }
    let quarter_sets: [&[u32]; 3] = [&[3, 3, 3, 3], &[4, 4, 4], &[1, 2, 3, 6]];
    for month in 1..=12u32 {
        for (di, day) in [1u32, 2, 15, 28, 29, 30, 31].into_iter().enumerate() {
            let policies: &[Option<&str>] = if day <= 28 {
                &[None]
            } else {
                &[Some("clamp"), Some("next")]
            };
            for &policy in policies {
                let starts_on = WEEKDAYS[(i + di) % 7];
                let quarters = quarter_sets[i % 3];
                out.push(Case {
                    definition: fixed_date(month, day, policy, starts_on, quarters, "start"),
                    shape: Shape::Fixed {
                        starts_on,
                        month,
                        day,
                        next: policy == Some("next"),
                        quarters: quarters.to_vec(),
                    },
                });
                i += 1;
            }
        }
    }
    out
}

fn civil(d: Date) -> Civil {
    let (y, m, d) = d.ymd();
    Civil { y, m, d }
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

#[derive(Default)]
struct Tally {
    compared: AtomicUsize,
    numbering_only: AtomicUsize,
    refused_alike: AtomicUsize,
    refused_beyond_window: AtomicUsize,
    unnumberable: AtomicUsize,
}

fn compare_days(cal: &FiscalCalendar, expected: &[reference::RefDay], definition: &Value) {
    let mut date = Date::from_ymd(1900, 1, 1).unwrap();
    for want in expected {
        let l = cal.locate(date).unwrap();
        let got = (
            civil(l.date),
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
            want.date,
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
        date = date.checked_add_days(1).unwrap();
    }
    assert_eq!(date, Date::from_ymd(2201, 1, 1).unwrap());
}

fn check(walk: &Walk, case: &Case, tally: &Tally) {
    let mut compared = false;
    let mut accepted = 0;
    for rule in RULES {
        let mut definition = case.definition.clone();
        definition["naming"]["schemes"]["fy"]["year_number"] = json!(rule);
        let crate_result = FiscalCalendar::from_json(&definition.to_string());
        match (reference::numbers(walk, &case.shape, rule), crate_result) {
            (Ok(years), Ok(cal)) => {
                accepted += 1;
                if !compared {
                    let expected = reference::compute(walk, &case.shape, rule).unwrap();
                    compare_days(&cal, &expected, &definition);
                    tally.compared.fetch_add(1, Ordering::Relaxed);
                    compared = true;
                } else {
                    for (first, number) in years.iter().filter(|(c, _)| c.y >= 1900 && c.y <= 2200)
                    {
                        let p = cal
                            .period(fiscalclock::Level::Year, *number, None)
                            .unwrap_or_else(|e| panic!("{e} for {number} under {definition}"));
                        assert_eq!(civil(p.start), *first, "{number} under {definition}");
                    }
                    tally.numbering_only.fetch_add(1, Ordering::Relaxed);
                }
            }
            (Err(refused), Err(errors)) => {
                let code = match refused {
                    Refused::Tie => ErrorCode::MajorityTie,
                    Refused::Collision => ErrorCode::YearNumberCollision,
                };
                let found: Vec<_> = errors
                    .as_slice()
                    .iter()
                    .map(|e| (e.code, e.path.as_str()))
                    .collect();
                assert_eq!(
                    found,
                    [(code, "naming.schemes.fy.year_number")],
                    "{definition}"
                );
                tally.refused_alike.fetch_add(1, Ordering::Relaxed);
            }
            (Ok(_), Err(errors)) => {
                // The crate checks years 1-9999; the reference only 1892-2209.
                let code = errors.as_slice()[0].code;
                assert!(
                    matches!(
                        code,
                        ErrorCode::YearNumberCollision | ErrorCode::MajorityTie
                    ),
                    "{errors} under {definition}"
                );
                tally.refused_beyond_window.fetch_add(1, Ordering::Relaxed);
            }
            (Err(refused), Ok(_)) => {
                panic!("the reference refuses ({refused:?}) what the crate accepts: {definition}")
            }
        }
    }
    if accepted == 0 {
        tally.unnumberable.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn locate_matches_the_day_walking_reference_1900_to_2200() {
    let walk = Walk::new();
    let all = cases();
    assert!(all.len() >= 600, "{} definitions", all.len());
    // Unoptimised builds check every 8th definition so `cargo test` stays
    // quick; `cargo test --release` checks all of them.
    let step = if cfg!(debug_assertions) { 8 } else { 1 };
    let selected: Vec<&Case> = all.iter().step_by(step).collect();
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let chunk = selected.len().div_ceil(threads);
    let tally = Tally::default();
    std::thread::scope(|s| {
        for part in selected.chunks(chunk) {
            let (walk, tally) = (&walk, &tally);
            s.spawn(move || part.iter().for_each(|c| check(walk, c, tally)));
        }
    });
    let compared = tally.compared.load(Ordering::Relaxed);
    eprintln!(
        "differential: {} definitions x 3 rules; {} compared day by day over 109,938 days, \
         {} compared by year bounds and numbers, {} refused by both, \
         {} refused by the crate beyond the reference window, \
         {} definitions with no usable rule",
        selected.len(),
        compared,
        tally.numbering_only.load(Ordering::Relaxed),
        tally.refused_alike.load(Ordering::Relaxed),
        tally.refused_beyond_window.load(Ordering::Relaxed),
        tally.unnumberable.load(Ordering::Relaxed),
    );
    assert_eq!(
        compared + tally.unnumberable.load(Ordering::Relaxed),
        selected.len(),
        "every numberable definition is compared day by day"
    );
}
