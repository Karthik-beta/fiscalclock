//! Compares the crate with the day-walking reference on calendars that change
//! (`exceptions.history`, 0.2.0). For every generated definition and every
//! year-numbering rule, both must accept or refuse alike. Where both accept,
//! `locate` must agree for every date from 1900-01-01 to 2200-12-31 (for the
//! first rule accepted), and `period`, `years`, `shift` and `format`/`parse`
//! with `{yy_end}` must agree year by year (for every rule accepted).
//!
//! The first tests check the reference itself: with no change, or a change to
//! the same rule, it must agree with the 0.1.0 reference, and it must
//! reproduce the transitions quoted in `research/filers/`.

mod reference;

use std::sync::atomic::{AtomicUsize, Ordering};

use fiscalclock::{Date, Details, ErrorCode, FiscalCalendar, Level, Period};
use reference::cases::{self, Family, HistoryCase, Mode, Rng};
use reference::history::{self, Entry, History, Kind, RefYear, Transition, TransitionSub};
use reference::{Civil, RefDay, RefPeriod, Refused, Shape, Walk};
use serde_json::{Value, json};

const RULES: [&str; 3] = ["start", "end", "majority"];

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

fn day_tuple(d: &RefDay) -> impl PartialEq + std::fmt::Debug {
    (
        d.date,
        d.year_number,
        d.year,
        d.quarter,
        d.period,
        d.week,
        (
            d.day_of_year,
            d.day_of_quarter,
            d.day_of_period,
            d.day_of_week,
        ),
    )
}

// ---------------------------------------------------------------------------
// The reference checked against itself and against the filings.
// ---------------------------------------------------------------------------

fn sample_shapes() -> Vec<Shape> {
    let mut rng = Rng::new(2026);
    // Unoptimised builds check fewer shapes so `cargo test` stays quick.
    let count = if cfg!(debug_assertions) { 6 } else { 24 };
    (0..count)
        .map(|i| {
            let starts_on = reference_weekday(i);
            if i % 2 == 0 {
                cases::week_shape(&mut rng, starts_on)
            } else {
                cases::fixed_shape(&mut rng, starts_on, None)
            }
        })
        .collect()
}

fn reference_weekday(i: usize) -> &'static str {
    [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ][i % 7]
}

fn same_as_plain(walk: &Walk, shape: &Shape, h: &History) {
    for rule in RULES {
        let plain = reference::compute(walk, shape, rule);
        let changed = history::compute(walk, h, rule);
        match (plain, changed) {
            (Ok(want), Ok((_, got))) => {
                assert_eq!(want.len(), got.len());
                for (w, g) in want.iter().zip(&got) {
                    assert_eq!(day_tuple(g), day_tuple(w), "{rule} under {h:?}");
                }
                let want: Vec<(Civil, i32)> = reference::numbers(walk, shape, rule).unwrap();
                let got: Vec<(Civil, i32)> = history::numbers(walk, h, rule)
                    .unwrap()
                    .iter()
                    .map(|y| (y.start, y.number))
                    .collect();
                assert_eq!(got, want, "{rule} under {h:?}");
            }
            (Err(a), Err(b)) => assert_eq!(a, b, "{rule} under {h:?}"),
            (a, b) => panic!(
                "{rule} under {h:?}: plain {:?}, with history {:?}",
                a.err(),
                b.err()
            ),
        }
    }
}

/// Runs `f` on each item on all cores.
fn in_parallel<T: Sync>(items: &[T], f: impl Fn(usize, &T) + Sync) {
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let chunk = items.len().div_ceil(threads).max(1);
    std::thread::scope(|s| {
        for (c, part) in items.chunks(chunk).enumerate() {
            let f = &f;
            s.spawn(move || {
                for (k, item) in part.iter().enumerate() {
                    f(c * chunk + k, item);
                }
            });
        }
    });
}

#[test]
fn history_reference_without_a_change_matches_the_plain_reference() {
    let walk = Walk::new();
    in_parallel(&sample_shapes(), |_, shape| {
        let h = History {
            base: shape.clone(),
            entries: Vec::new(),
        };
        same_as_plain(&walk, shape, &h);
    });
}

#[test]
fn history_reference_with_a_change_to_the_same_rule_matches_the_plain_reference() {
    let walk = Walk::new();
    in_parallel(&sample_shapes(), |i, shape| {
        let starts = reference::rule_starts(&walk, shape).0;
        // A year start between 1950 and 2150, a different one per shape.
        let from = starts[60 + 8 * i];
        let h = History {
            base: shape.clone(),
            entries: vec![Entry {
                from: walk.day(from),
                transition: None,
                shape: shape.clone(),
            }],
        };
        same_as_plain(&walk, shape, &h);
    });
}

fn ymd(y: i32, m: u32, d: u32) -> Civil {
    Civil { y, m, d }
}

/// A `fixed_date` rule on day 1 of `month` with quarters of three months.
/// The week start is illustrative: the filings do not state one.
fn first_of(month: u32) -> Shape {
    Shape::Fixed {
        starts_on: "monday",
        month,
        day: 1,
        next: false,
        quarters: vec![3, 3, 3, 3],
    }
}

/// A change from one first-of-month year start to another through a
/// calendar-months transition of `quarters` (illustrative: only the dates and
/// labels come from the filings).
fn filed_change(old: u32, new: u32, start: Civil, from: Civil, quarters: Vec<u32>) -> History {
    History {
        base: first_of(old),
        entries: vec![Entry {
            from,
            transition: Some(Transition {
                start,
                sub: TransitionSub::Months { day: 1, quarters },
            }),
            shape: first_of(new),
        }],
    }
}

/// The transitions in the 0.2.0 brief, from `research/filers/pg-hygiene.md`,
/// `siemens-india.md` and `linde-india.md`: dates, lengths and labels.
#[test]
fn history_reference_reproduces_the_filed_transitions() {
    let walk = Walk::new();
    // (history, transition start, from, days, label with {yyyy}-{yy_end},
    // numbers under "start" of the years before, during and after).
    let filed = [
        (
            filed_change(7, 4, ymd(2024, 7, 1), ymd(2025, 4, 1), vec![3, 3, 3]),
            274,
            "2024-25",
            [2023, 2024, 2025],
        ),
        (
            filed_change(10, 4, ymd(2024, 10, 1), ymd(2026, 4, 1), vec![3; 6]),
            547,
            "2024-26",
            [2023, 2024, 2026],
        ),
        (
            filed_change(1, 4, ymd(2022, 1, 1), ymd(2023, 4, 1), vec![3; 5]),
            455,
            "2022-23",
            [2021, 2022, 2023],
        ),
    ];
    for (h, days, label, numbers) in filed {
        let t = h.entries[0].transition.as_ref().unwrap().start;
        let years = history::numbers(&walk, &h, "start").unwrap();
        let k = years.iter().position(|y| y.start == t).unwrap();
        let around: Vec<i32> = years[k - 1..=k + 1].iter().map(|y| y.number).collect();
        assert_eq!(around, numbers);
        let y = years[k];
        assert_eq!(y.kind, Kind::Transition(0));
        assert_eq!(y.end_exclusive, h.entries[0].from);
        assert_eq!(
            format!("{:04}-{:02}", y.number, y.last_day_year % 100),
            label
        );
        let (_, located) = history::compute(&walk, &h, "start").unwrap();
        let last = located
            .iter()
            .find(|d| d.date == h.entries[0].from)
            .map(|d| d.date)
            .unwrap();
        let last_of_transition = located.iter().rev().find(|d| d.date < last).unwrap();
        assert_eq!(last_of_transition.year.days, days);
        // Siemens: "comprised of six quarters"; eighteen months.
        let quarters = h.entries[0]
            .transition
            .as_ref()
            .map(|t| match &t.sub {
                TransitionSub::Months { quarters, .. } => quarters.len() as u32,
                TransitionSub::Weeks { .. } => unreachable!(),
            })
            .unwrap();
        assert_eq!(last_of_transition.quarter.index, Some(quarters));
        assert_eq!(last_of_transition.period.index, Some(quarters * 3));
    }
}

// ---------------------------------------------------------------------------
// The crate against the reference.
// ---------------------------------------------------------------------------

#[derive(Default)]
struct Tally {
    compared: AtomicUsize,
    numbering_only: AtomicUsize,
    refused_alike: AtomicUsize,
    refused_beyond_window: AtomicUsize,
    unnumberable: AtomicUsize,
    skipped_numbers: AtomicUsize,
}

fn compare_days(cal: &FiscalCalendar, expected: &[RefDay], definition: &Value) {
    let mut d = Date::from_ymd(1900, 1, 1).unwrap();
    for want in expected {
        let l = cal
            .locate(d)
            .unwrap_or_else(|e| panic!("{e} for {d} under {definition}"));
        let got = RefDay {
            date: civil(l.date),
            year_number: l.year.year,
            year: as_ref(&l.year),
            quarter: as_ref(&l.quarter),
            period: as_ref(&l.period),
            week: as_ref(&l.week),
            day_of_year: l.day_of_year,
            day_of_quarter: l.day_of_quarter,
            day_of_period: l.day_of_period,
            day_of_week: l.day_of_week,
        };
        assert_eq!(day_tuple(&got), day_tuple(want), "{d} under {definition}");
        d = d.checked_add_days(1).unwrap();
    }
    assert_eq!(d, Date::from_ymd(2201, 1, 1).unwrap());
}

/// `period`, `years`, `shift` and `format`/`parse` with `{yy_end}`, year by
/// year, for the fiscal years that touch 1900-2200.
fn compare_years(cal: &FiscalCalendar, years: &[RefYear], definition: &Value, tally: &Tally) {
    let (lo, hi) = (ymd(1900, 1, 1), ymd(2200, 12, 31));
    let window: Vec<&RefYear> = years
        .iter()
        .filter(|y| y.end_exclusive > lo && y.start <= hi)
        .collect();
    let bounds = |p: &Period| (civil(p.start), civil(p.end_exclusive), p.year);
    let mut periods = Vec::new();
    for y in &window {
        let p = cal
            .period(Level::Year, y.number, None)
            .unwrap_or_else(|e| panic!("{e} for {} under {definition}", y.number));
        assert_eq!(
            bounds(&p),
            (y.start, y.end_exclusive, y.number),
            "{} under {definition}",
            y.number
        );
        let label = cal.format(&p, None).unwrap();
        assert_eq!(
            label,
            format!("FY{:04}-{:02}", y.number, y.last_day_year % 100),
            "{definition}"
        );
        assert_eq!(cal.parse(&label).unwrap(), p, "{label} under {definition}");
        let q1 = cal.period(Level::Quarter, y.number, Some(1)).unwrap();
        assert_eq!(
            cal.format(&q1, None).unwrap(),
            format!("Q1 FY{:04}-{:02}", y.number, y.last_day_year % 100),
            "{definition}"
        );
        periods.push(p);
    }
    for pair in periods.windows(2) {
        assert_eq!(cal.shift(&pair[0], 1).unwrap(), pair[1], "{definition}");
        assert_eq!(cal.shift(&pair[1], -1).unwrap(), pair[0], "{definition}");
    }
    let (first, last) = (window[0].number, window[window.len() - 1].number);
    let got: Vec<_> = cal
        .years(first, last)
        .unwrap_or_else(|e| panic!("{e} under {definition}"))
        .iter()
        .map(bounds)
        .collect();
    let want: Vec<_> = window
        .iter()
        .map(|y| (y.start, y.end_exclusive, y.number))
        .collect();
    assert_eq!(got, want, "years({first}, {last}) under {definition}");
    // Numbers skipped across a change name no fiscal year.
    for n in first..=last {
        if window.iter().any(|y| y.number == n) {
            continue;
        }
        let e = cal.period(Level::Year, n, None).unwrap_err();
        assert_eq!(
            (e.code, e.path.as_str()),
            (ErrorCode::PeriodNotFound, "year"),
            "{n} under {definition}"
        );
        assert_eq!(
            cal.years(n, n).unwrap(),
            Vec::new(),
            "{n} under {definition}"
        );
        // A label naming it matches no fiscal year, and a period argument
        // naming it is not found.
        let e = cal.parse(&format!("FY{n:04}-{:02}", n % 100)).unwrap_err();
        assert_eq!(
            e.code,
            ErrorCode::LabelNotRecognised,
            "{n} under {definition}"
        );
        let ghost = Period {
            year: n,
            ..periods[0]
        };
        for e in [
            cal.shift(&ghost, 1).unwrap_err(),
            cal.format(&ghost, None).unwrap_err(),
            cal.children(&ghost, Level::Quarter).unwrap_err(),
        ] {
            assert_eq!(
                (e.code, e.path.as_str()),
                (ErrorCode::PeriodNotFound, "period"),
                "{n} under {definition}"
            );
        }
        tally.skipped_numbers.fetch_add(1, Ordering::Relaxed);
    }
}

fn check(walk: &Walk, case: &HistoryCase, tally: &Tally) {
    let mut compared = false;
    let mut accepted = 0;
    for rule in RULES {
        let mut definition = case.definition.clone();
        definition["naming"]["schemes"]["fy"]["year_number"] = json!(rule);
        let crate_result = FiscalCalendar::from_json(&definition.to_string());
        match (history::numbers(walk, &case.history, rule), crate_result) {
            (Ok(years), Ok(cal)) => {
                accepted += 1;
                if !compared {
                    let (_, days) = history::compute(walk, &case.history, rule).unwrap();
                    compare_days(&cal, &days, &definition);
                    tally.compared.fetch_add(1, Ordering::Relaxed);
                    compared = true;
                } else {
                    tally.numbering_only.fetch_add(1, Ordering::Relaxed);
                }
                compare_years(&cal, &years, &definition, tally);
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
                // The crate lists up to five affected years over its whole
                // range; with fewer than five it has listed them all, so
                // every one the reference finds in its window is there.
                let (why, want) = history::affected(walk, &case.history, rule);
                assert_eq!(why, Some(refused));
                let Some(Details::Years(got)) = &errors.as_slice()[0].details else {
                    panic!("no details.years: {errors} under {definition}")
                };
                if got.len() < 5 {
                    for y in &want {
                        assert!(got.contains(y), "{y} not in {got:?} under {definition}");
                    }
                }
                tally.refused_alike.fetch_add(1, Ordering::Relaxed);
            }
            (Ok(_), Err(errors)) => {
                // The crate checks the whole supported range; the reference
                // only the years that open between 1892 and 2209.
                let code = errors.as_slice()[0].code;
                assert!(
                    errors.as_slice().len() == 1
                        && matches!(
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
fn calendars_that_change_match_the_day_walking_reference_1900_to_2200() {
    let walk = Walk::new();
    let all = cases::history_cases(&walk, 240);
    // Coverage of the generated set: every family, each kind of change, and
    // one and two history entries.
    let count = |f: &dyn Fn(&HistoryCase) -> bool| all.iter().filter(|c| f(c)).count();
    for family in [
        Family::FixedToFixed,
        Family::WeekToWeek,
        Family::FixedToWeek,
        Family::WeekToFixed,
    ] {
        for mode in [Mode::Direct, Mode::Months, Mode::Weeks] {
            let n = count(&|c| c.changes.contains(&(family, mode)));
            assert!(n > 0, "no {family:?} change of kind {mode:?}");
        }
    }
    assert!(count(&|c| c.history.entries.len() == 2) >= 100);
    let month_lengths: std::collections::BTreeSet<u32> = all
        .iter()
        .flat_map(|c| &c.history.entries)
        .filter_map(|e| match &e.transition {
            Some(Transition {
                sub: TransitionSub::Months { quarters, .. },
                ..
            }) => Some(quarters.iter().sum()),
            _ => None,
        })
        .collect();
    assert!(month_lengths.len() >= 12, "{month_lengths:?}");

    // Unoptimised builds check every 8th definition so `cargo test` stays
    // quick; `cargo test --release` checks all of them.
    let step = if cfg!(debug_assertions) { 8 } else { 1 };
    let selected: Vec<&HistoryCase> = all.iter().step_by(step).collect();
    let tally = Tally::default();
    in_parallel(&selected, |_, c| check(&walk, c, &tally));
    let compared = tally.compared.load(Ordering::Relaxed);
    let unnumberable = tally.unnumberable.load(Ordering::Relaxed);
    eprintln!(
        "differential_history: {} definitions x 3 rules; {} compared day by day over \
         109,938 days, {} compared by years only, {} refused by both, {} refused by the \
         crate beyond the reference window, {} definitions with no usable rule, \
         {} skipped year numbers checked",
        selected.len(),
        compared,
        tally.numbering_only.load(Ordering::Relaxed),
        tally.refused_alike.load(Ordering::Relaxed),
        tally.refused_beyond_window.load(Ordering::Relaxed),
        unnumberable,
        tally.skipped_numbers.load(Ordering::Relaxed),
    );
    assert_eq!(
        compared + unnumberable,
        selected.len(),
        "every numberable definition is compared day by day"
    );
    assert!(
        compared * 2 >= selected.len(),
        "fewer than half of the definitions can be numbered"
    );
}

/// The canonical form of a definition with `"transition": null` equals the
/// form without the field (both mean no transition).
#[test]
fn transition_null_and_absent_are_the_same_definition() {
    let walk = Walk::new();
    let case = (1..)
        .find_map(|seed| {
            cases::history_case(
                &walk,
                seed,
                Some(Family::FixedToFixed),
                Some(Mode::Direct),
                Some(1),
            )
        })
        .unwrap();
    let a = cases::definition(&case.history, "start", true);
    let b = cases::definition(&case.history, "start", false);
    assert_ne!(a, b);
    let (a, b) = (
        FiscalCalendar::from_json(&a.to_string()).unwrap_or_else(|e| panic!("{e}\n{a}")),
        FiscalCalendar::from_json(&b.to_string()).unwrap_or_else(|e| panic!("{e}\n{b}")),
    );
    assert_eq!(a.canonical_json(), b.canonical_json());
    assert_eq!(a.calendar_id(), b.calendar_id());
}
