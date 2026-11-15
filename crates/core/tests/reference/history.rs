//! The day-walking reference for calendars that change on a date
//! (`exceptions.history`, with an optional transition before each change).
//!
//! Each rule's year starts come from [`rule_starts`], which scans the days
//! around each target date. Every day is then checked on its own: which regime
//! or transition is in force on it, and whether it opens a fiscal year under
//! that regime's rule. Periods and weeks are labelled by walking each fiscal
//! year's days and counting. Nothing is shared with the crate's arithmetic.

use std::collections::HashMap;

use super::{Civil, Numbering, RefDay, RefPeriod, Refused, Shape, Walk, rule_starts};

/// The subdivision of a transition year.
#[derive(Clone, Debug)]
pub enum TransitionSub {
    /// `calendar_months`: a period starts on day `day` of every month.
    Months { day: u32, quarters: Vec<u32> },
    /// `week_pattern`: runs of whole weeks, with no extra week.
    Weeks {
        periods: Vec<u32>,
        quarters: Vec<u32>,
    },
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub start: Civil,
    pub sub: TransitionSub,
}

/// One entry of `exceptions.history`.
#[derive(Clone, Debug)]
pub struct Entry {
    pub from: Civil,
    pub transition: Option<Transition>,
    pub shape: Shape,
}

/// The top-level rule (regime 0) and the history entries (regime `i + 1`).
/// Every shape carries the same global `week.starts_on`.
#[derive(Clone, Debug)]
pub struct History {
    pub base: Shape,
    pub entries: Vec<Entry>,
}

impl History {
    pub fn starts_on(&self) -> &'static str {
        starts_on(&self.base)
    }

    /// The regime's rule: 0 is the top level, `i + 1` is entry `i`.
    pub fn shape(&self, regime: usize) -> &Shape {
        if regime == 0 {
            &self.base
        } else {
            &self.entries[regime - 1].shape
        }
    }

    /// The change point of entry `i`: its transition's start, or its `from`.
    pub fn change_point(&self, i: usize) -> Civil {
        let e = &self.entries[i];
        e.transition.as_ref().map_or(e.from, |t| t.start)
    }
}

pub fn starts_on(shape: &Shape) -> &'static str {
    match shape {
        Shape::Week { starts_on, .. } | Shape::Fixed { starts_on, .. } => starts_on,
    }
}

/// What a fiscal year is: a year of a regime's rule, or the transition of a
/// history entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Regime(usize),
    Transition(usize),
}

/// A fiscal year as indexes into the walk: `[first, end)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub first: usize,
    pub end: usize,
    pub kind: Kind,
}

/// A numbered fiscal year.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RefYear {
    pub start: Civil,
    pub end_exclusive: Civil,
    pub number: i32,
    pub kind: Kind,
    /// The calendar year of the fiscal year's last day, for `{yy_end}`.
    pub last_day_year: i32,
}

/// Each regime's year starts and month starts, as one flag per walked day.
fn regime_flags(walk: &Walk, h: &History) -> Vec<(Vec<bool>, Vec<bool>)> {
    (0..=h.entries.len())
        .map(|r| {
            let (firsts, month_starts) = rule_starts(walk, h.shape(r));
            let mut opens = vec![false; walk.days.len()];
            for i in firsts {
                opens[i] = true;
            }
            (opens, month_starts)
        })
        .collect()
}

/// The number of calendar months from `start` to `end`, counted by walking
/// the days and counting each day that has `start`'s day of the month.
fn months_between(walk: &Walk, start: Civil, end: Civil) -> u32 {
    let (a, b) = (walk.at[&start], walk.at[&end]);
    (a..b).filter(|&j| walk.days[j].d == start.d).count() as u32
}

/// The errors `spec/definition-v1.md` "Cross-block checks" gives `h`, as
/// `(code, path)` pairs sorted by path and then code. Of the block-level
/// checks it models only the two on a transition's own fields that the
/// generator could break: `transition.start` on or after `from`, and a
/// `week_pattern` transition's `quarters` not summing to the number of its
/// `periods`. Any block-level error suppresses every cross-block check, as
/// the spec says. The generated definitions must produce no error at all.
pub fn problems(walk: &Walk, h: &History) -> Vec<(&'static str, String)> {
    let mut out = block_problems(h);
    if out.is_empty() {
        out = cross_block_problems(walk, h);
    }
    out.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(b.0)));
    out
}

fn block_problems(h: &History) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    for (i, e) in h.entries.iter().enumerate() {
        let Some(t) = &e.transition else { continue };
        if t.start >= e.from {
            out.push((
                "INVALID_VALUE",
                format!("exceptions.history[{i}].transition.start"),
            ));
        }
        if let TransitionSub::Weeks { periods, quarters } = &t.sub {
            if !periods.is_empty() && quarters.iter().sum::<u32>() as usize != periods.len() {
                out.push((
                    "QUARTERS_SUM_MISMATCH",
                    format!("exceptions.history[{i}].transition.subdivision.quarters"),
                ));
            }
        }
    }
    out
}

/// The rows of the spec's cross-block table, for every history entry.
fn cross_block_problems(walk: &Walk, h: &History) -> Vec<(&'static str, String)> {
    let flags = regime_flags(walk, h);
    let starts_on = h.starts_on();
    let mut out = Vec::new();
    for (i, e) in h.entries.iter().enumerate() {
        let c = h.change_point(i);
        let c_path = if e.transition.is_some() {
            format!("exceptions.history[{i}].transition.start")
        } else {
            format!("exceptions.history[{i}].from")
        };
        if i >= 1 && c <= h.entries[i - 1].from {
            out.push(("EXCEPTIONS_OVERLAP", c_path.clone()));
        }
        if !flags[i].0[walk.at[&c]] {
            out.push(("CHANGE_NOT_ON_BOUNDARY", c_path));
        }
        if !flags[i + 1].0[walk.at[&e.from]] {
            out.push((
                "CHANGE_NOT_ON_BOUNDARY",
                format!("exceptions.history[{i}].from"),
            ));
        }
        let Some(t) = &e.transition else { continue };
        let sub = format!("exceptions.history[{i}].transition.subdivision");
        match &t.sub {
            TransitionSub::Months { day, quarters } => {
                if *day != t.start.d || *day != e.from.d {
                    out.push((
                        "SUBDIVISION_YEAR_MISMATCH",
                        format!("{sub}.month_start_day"),
                    ));
                } else if quarters.iter().sum::<u32>() != months_between(walk, t.start, e.from) {
                    out.push(("QUARTERS_SUM_MISMATCH", format!("{sub}.quarters")));
                }
            }
            TransitionSub::Weeks { periods, .. } => {
                let days = walk.at[&e.from] - walk.at[&t.start];
                if walk.weekday[walk.at[&t.start]] != starts_on || days % 7 != 0 {
                    out.push(("SUBDIVISION_YEAR_MISMATCH", format!("{sub}.type")));
                }
                // Checked whenever the length is whole weeks, even when the
                // start is on the wrong weekday.
                if days % 7 == 0 && periods.iter().sum::<u32>() as usize != days / 7 {
                    out.push(("WEEKS_SUM_MISMATCH", format!("{sub}.periods")));
                }
            }
        }
    }
    out
}

/// Every complete fiscal year in the walk, in date order. Panics if
/// [`problems`] finds an error in the history.
pub fn timeline(walk: &Walk, h: &History) -> Vec<Span> {
    let found = problems(walk, h);
    assert!(found.is_empty(), "{found:?}");
    let n = walk.days.len();
    let flags = regime_flags(walk, h);

    // What is in force on each day: later entries overwrite earlier ones from
    // their change point on.
    let mut in_force = vec![Kind::Regime(0); n];
    for (i, e) in h.entries.iter().enumerate() {
        let from = walk.at[&e.from];
        if let Some(t) = &e.transition {
            for k in in_force.iter_mut().take(from).skip(walk.at[&t.start]) {
                *k = Kind::Transition(i);
            }
        }
        for k in in_force.iter_mut().skip(from) {
            *k = Kind::Regime(i + 1);
        }
    }

    // A day opens a fiscal year when it is a year start under the rule of the
    // regime in force, or the first day of a transition.
    let opens: Vec<bool> = (0..n)
        .map(|j| match in_force[j] {
            Kind::Regime(r) => flags[r].0[j],
            Kind::Transition(i) => j == walk.at[&h.entries[i].transition.as_ref().unwrap().start],
        })
        .collect();
    for j in 1..n {
        if in_force[j] != in_force[j - 1] {
            assert!(opens[j], "a change on {:?} opens no year", walk.days[j]);
        }
    }

    let firsts: Vec<usize> = (0..n).filter(|&j| opens[j]).collect();
    firsts
        .windows(2)
        .map(|w| Span {
            first: w[0],
            end: w[1],
            kind: in_force[w[0]],
        })
        .collect()
}

/// The number `numbering` gives one fiscal year, or `None` when "majority"
/// finds no calendar year holding more than half of its days: an even split,
/// or a year spread over three calendar years with none holding a majority
/// (both `MAJORITY_TIE`).
fn raw_number(walk: &Walk, s: &Span, numbering: Numbering) -> Option<i32> {
    match numbering {
        "start" => Some(walk.days[s.first].y),
        "end" => Some(walk.days[s.end - 1].y),
        _ => {
            let mut held: HashMap<i32, usize> = HashMap::new();
            for c in &walk.days[s.first..s.end] {
                *held.entry(c.y).or_default() += 1;
            }
            let total = s.end - s.first;
            held.iter().find(|&(_, &c)| c * 2 > total).map(|(&y, _)| y)
        }
    }
}

/// The number of each span, or why `numbering` cannot number them with the
/// years the error concerns, in date order and without repeats: for a tie,
/// the calendar year in which each tied fiscal year starts; for a collision
/// within one regime's run, the repeated number or the first skipped one; for
/// a collision across a change, the later year's number. Ties are reported
/// before any collision.
fn number_spans(
    walk: &Walk,
    spans: &[Span],
    numbering: Numbering,
) -> Result<Vec<i32>, (Refused, Vec<i64>)> {
    let raw: Vec<Option<i32>> = spans
        .iter()
        .map(|s| raw_number(walk, s, numbering))
        .collect();
    let ties: Vec<i64> = spans
        .iter()
        .zip(&raw)
        .filter(|(_, n)| n.is_none())
        .map(|(s, _)| i64::from(walk.days[s.first].y))
        .collect();
    if !ties.is_empty() {
        return Err((Refused::Tie, ties));
    }
    let numbers: Vec<i32> = raw.into_iter().map(Option::unwrap).collect();
    let mut clashes: Vec<i64> = Vec::new();
    for k in 1..spans.len() {
        let (p, n) = (numbers[k - 1], numbers[k]);
        let same_run =
            matches!(spans[k].kind, Kind::Regime(_)) && spans[k].kind == spans[k - 1].kind;
        let affected = if same_run {
            // Within one regime's run: consecutive.
            if n == p {
                Some(n)
            } else if n != p + 1 {
                Some(p + 1)
            } else {
                None
            }
        } else if n <= p {
            // Across a change: strictly increasing, a skip allowed.
            Some(n)
        } else {
            None
        };
        if let Some(a) = affected.map(i64::from) {
            if !clashes.contains(&a) {
                clashes.push(a);
            }
        }
    }
    if clashes.is_empty() {
        Ok(numbers)
    } else {
        Err((Refused::Collision, clashes))
    }
}

/// Why `numbering` cannot number the years, and the years the error
/// concerns (see [`number_spans`]); `(None, [])` when it can.
pub fn affected(walk: &Walk, h: &History, numbering: Numbering) -> (Option<Refused>, Vec<i64>) {
    match number_spans(walk, &timeline(walk, h), numbering) {
        Ok(_) => (None, Vec::new()),
        Err((why, years)) => (Some(why), years),
    }
}

fn years_from(walk: &Walk, spans: &[Span], numbers: &[i32]) -> Vec<RefYear> {
    spans
        .iter()
        .zip(numbers)
        .map(|(s, &number)| RefYear {
            start: walk.days[s.first],
            end_exclusive: walk.days[s.end],
            number,
            kind: s.kind,
            last_day_year: walk.days[s.end - 1].y,
        })
        .collect()
}

/// Every complete fiscal year in the walk with its number, or why `numbering`
/// cannot number them.
pub fn numbers(walk: &Walk, h: &History, numbering: Numbering) -> Result<Vec<RefYear>, Refused> {
    let spans = timeline(walk, h);
    let numbers = number_spans(walk, &spans, numbering).map_err(|(why, _)| why)?;
    Ok(years_from(walk, &spans, &numbers))
}

/// Whether every year in `listed` lies outside the reference's window, given
/// its numbered `years` (which it found no fault with). The crate checks the
/// numbering over its whole supported range, so it can refuse a numbering
/// because of years beyond the walk; then the years it lists (see
/// [`number_spans`] for what they are) must be at or beyond the edges of the
/// window. The edges are included because a fault between the first or last
/// year in the window and its neighbour outside names the edge year's number
/// (a repeat, or a skip after it) or the calendar year in which the
/// neighbour starts, which can equal the edge year's.
pub fn beyond_window(years: &[RefYear], why: Refused, listed: &[i64]) -> bool {
    let (first, last) = (years[0], years[years.len() - 1]);
    let (lo, hi) = match why {
        Refused::Collision => (first.number, last.number),
        Refused::Tie => (first.start.y, last.start.y),
    };
    !listed.is_empty()
        && listed
            .iter()
            .all(|&y| y <= i64::from(lo) || y >= i64::from(hi))
}

/// A fiscal year's subdivision: its quarters, and either a week pattern
/// (periods and extra-week period, 0 for none) or a test for whether a walked
/// day starts a fiscal month.
type Subdivision<'a> = (
    &'a [u32],
    Option<(&'a [u32], u32)>,
    Box<dyn Fn(usize) -> bool + 'a>,
);

#[derive(Clone, Copy)]
struct Label {
    quarter: u32,
    period: u32,
    week: u32,
    extra_week: bool,
}

/// The years and the label of every day from 1900-01-01 to 2200-12-31, or why
/// `numbering` cannot number the years.
pub fn compute(
    walk: &Walk,
    h: &History,
    numbering: Numbering,
) -> Result<(Vec<RefYear>, Vec<RefDay>), Refused> {
    let spans = timeline(walk, h);
    let numbers = number_spans(walk, &spans, numbering).map_err(|(why, _)| why)?;
    let years = years_from(walk, &spans, &numbers);
    let flags = regime_flags(walk, h);
    let starts_on = h.starts_on();

    let from = walk.at[&Civil {
        y: 1900,
        m: 1,
        d: 1,
    }];
    let to = walk.at[&Civil {
        y: 2200,
        m: 12,
        d: 31,
    }];
    let mut out = Vec::with_capacity(to - from + 1);

    for (si, s) in spans.iter().enumerate() {
        if s.end <= from || s.first > to {
            continue;
        }
        // The year's subdivision: its quarters, and either a week pattern
        // (periods and extra-week period, 0 for none) or a test for the first
        // day of a fiscal month.
        let (quarters, pattern, month_start): Subdivision = match s.kind {
            Kind::Regime(r) => match h.shape(r) {
                Shape::Week {
                    periods,
                    quarters,
                    extra,
                    ..
                } => (quarters, Some((periods, *extra)), Box::new(|_| false)),
                Shape::Fixed { quarters, .. } => {
                    let starts = &flags[r].1;
                    (quarters, None, Box::new(move |j: usize| starts[j]))
                }
            },
            Kind::Transition(i) => match &h.entries[i].transition.as_ref().unwrap().sub {
                TransitionSub::Months { day, quarters } => {
                    let day = *day;
                    (
                        quarters,
                        None,
                        Box::new(move |j: usize| walk.days[j].d == day),
                    )
                }
                TransitionSub::Weeks { periods, quarters } => {
                    (quarters, Some((periods, 0)), Box::new(|_| false))
                }
            },
        };
        let days = s.end - s.first;
        // Only a week-based regime has an extra week, in its 371-day years.
        let long = matches!(pattern, Some((_, extra)) if extra > 0) && days == 371;
        let extra = pattern.map_or(0, |(_, e)| e);
        if pattern.is_some() {
            assert_eq!(walk.weekday[s.first], starts_on, "{:?}", walk.days[s.first]);
            assert_eq!(days % 7, 0);
        }
        let quarter_of = |p: u32| {
            let mut seen = 0;
            for (qi, size) in quarters.iter().enumerate() {
                seen += size;
                if p <= seen {
                    return qi as u32 + 1;
                }
            }
            panic!("period {p} is beyond the quarters {quarters:?}")
        };

        // Label each day by counting from the year's first day.
        let mut labels: Vec<Label> = Vec::with_capacity(days);
        let (mut period, mut week, mut weeks_done) = (1u32, 1u32, 0u32);
        #[allow(clippy::needless_range_loop)] // `j` is also the day passed to `month_start`.
        for j in s.first..s.end {
            if j != s.first && walk.weekday[j] == starts_on {
                week += 1;
                if let Some((periods, _)) = pattern {
                    weeks_done += 1;
                    let mut len = periods[period as usize - 1];
                    if long && period == extra {
                        len += 1;
                    }
                    if weeks_done == len {
                        period += 1;
                        weeks_done = 0;
                    }
                }
            }
            if pattern.is_none() && j != s.first && month_start(j) {
                period += 1;
            }
            let extra_week =
                long && period == extra && weeks_done == pattern.unwrap().0[extra as usize - 1];
            labels.push(Label {
                quarter: quarter_of(period),
                period,
                week,
                extra_week,
            });
        }
        assert_eq!(
            period,
            quarters.iter().sum::<u32>(),
            "the periods of {:?} do not fill its quarters",
            walk.days[s.first]
        );

        // The first and end index of each quarter, period and week.
        let mut bounds: HashMap<(u8, u32), (usize, usize)> = HashMap::new();
        for (k, l) in labels.iter().enumerate() {
            let j = s.first + k;
            for key in [(1u8, l.quarter), (2, l.period), (3, l.week)] {
                bounds.entry(key).or_insert((j, j)).1 = j + 1;
            }
        }
        let mk = |index: Option<u32>, (a, b): (usize, usize), flag: bool| RefPeriod {
            index,
            start: walk.days[a],
            end_exclusive: walk.days[b],
            days: (b - a) as u32,
            weeks: pattern.map(|_| ((b - a) / 7) as u32),
            has_extra_week: flag,
        };
        for j in s.first.max(from)..s.end.min(to + 1) {
            let l = labels[j - s.first];
            let qb = bounds[&(1, l.quarter)];
            let pb = bounds[&(2, l.period)];
            let wb = bounds[&(3, l.week)];
            let mut day_of_week = 1;
            let mut k = j;
            while walk.weekday[k] != starts_on {
                k -= 1;
                day_of_week += 1;
            }
            out.push(RefDay {
                date: walk.days[j],
                year_number: numbers[si],
                year: mk(None, (s.first, s.end), long),
                quarter: mk(Some(l.quarter), qb, long && quarter_of(extra) == l.quarter),
                period: mk(Some(l.period), pb, long && l.period == extra),
                week: mk(Some(l.week), wb, l.extra_week),
                day_of_year: (j - s.first + 1) as u32,
                day_of_quarter: (j - qb.0 + 1) as u32,
                day_of_period: (j - pb.0 + 1) as u32,
                day_of_week,
            });
        }
    }
    assert_eq!(out.len(), to - from + 1);
    Ok((years, out))
}
