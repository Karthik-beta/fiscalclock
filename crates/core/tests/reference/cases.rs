//! Definitions with `exceptions.history` that are valid by construction.
//!
//! Each change date is picked from the year starts the reference computes for
//! the rules on either side of it: a change point on a year start of the old
//! rule, a `from` on a year start of the new rule. A transition's subdivision
//! is then derived to fit the dates. A seeded generator (SplitMix64) makes the
//! cases the same on every run.

use serde_json::{Value, json};

use super::history::{Entry, History, Transition, TransitionSub, problems, starts_on};
use super::{Civil, NAMES, Shape, Walk, rule_starts};

/// SplitMix64: small, seedable and good enough to spread test cases.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed)
    }

    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A number in `0..n`.
    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    /// A number in `lo..=hi`.
    pub fn range(&mut self, lo: u32, hi: u32) -> u32 {
        lo + self.below((hi - lo + 1) as usize) as u32
    }

    pub fn pick<T: Clone>(&mut self, xs: &[T]) -> T {
        xs[self.below(xs.len())].clone()
    }
}

/// The kinds of rule on either side of a change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    FixedToFixed,
    WeekToWeek,
    FixedToWeek,
    WeekToFixed,
}

/// The kind of change: none (straight from one rule to the next), or a
/// transition subdivided by calendar months or by whole weeks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Direct,
    Months,
    Weeks,
}

pub struct HistoryCase {
    pub definition: Value,
    pub history: History,
    /// The family and mode of each change, for messages and tallies.
    pub changes: Vec<(Family, Mode)>,
}

const WEEK_PATTERNS: [(&[u32], &[u32]); 6] = [
    (&[4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4], &[3, 3, 3, 3]),
    (&[4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5], &[3, 3, 3, 3]),
    (&[5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4], &[3, 3, 3, 3]),
    (&[4; 13], &[3, 3, 3, 4]),
    (&[4; 13], &[4, 3, 3, 3]),
    (&[4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5], &[2, 4, 6]),
];

const MONTH_QUARTERS: [&[u32]; 6] = [
    &[3, 3, 3, 3],
    &[4, 4, 4],
    &[1, 2, 3, 6],
    &[12],
    &[6, 6],
    &[2, 2, 2, 2, 2, 2],
];

pub fn week_shape(rng: &mut Rng, starts_on: &'static str) -> Shape {
    let (periods, quarters) = rng.pick(&WEEK_PATTERNS);
    Shape::Week {
        starts_on,
        edge_end: rng.below(2) == 0,
        month: rng.range(1, 12),
        day: match rng.below(4) {
            0 => None,
            _ => Some(rng.range(1, 28)),
        },
        rule: rng.pick(&["last", "first", "nearest"]),
        periods: periods.to_vec(),
        quarters: quarters.to_vec(),
        extra: rng.range(1, periods.len() as u32),
    }
}

/// A `fixed_date` rule; `day` is drawn when not given, mostly 1 to 28 and
/// sometimes 29 to 31 with a `short_month` policy.
pub fn fixed_shape(rng: &mut Rng, starts_on: &'static str, day: Option<u32>) -> Shape {
    let day = day.unwrap_or_else(|| {
        if rng.below(6) == 0 {
            rng.range(29, 31)
        } else {
            rng.range(1, 28)
        }
    });
    Shape::Fixed {
        starts_on,
        month: rng.range(1, 12),
        day,
        next: rng.below(2) == 0,
        quarters: rng.pick(&MONTH_QUARTERS).to_vec(),
    }
}

/// The same rule with a different subdivision.
fn resubdivided(rng: &mut Rng, shape: &Shape) -> Shape {
    let mut out = shape.clone();
    match &mut out {
        Shape::Week {
            periods,
            quarters,
            extra,
            ..
        } => {
            let (p, q) = rng.pick(&WEEK_PATTERNS);
            *periods = p.to_vec();
            *quarters = q.to_vec();
            *extra = rng.range(1, p.len() as u32);
        }
        Shape::Fixed { quarters, .. } => *quarters = rng.pick(&MONTH_QUARTERS).to_vec(),
    }
    out
}

/// `total` split into `parts` positive integers.
fn compose(rng: &mut Rng, total: u32, parts: u32) -> Vec<u32> {
    let mut cuts: Vec<u32> = Vec::new();
    while cuts.len() + 1 < parts as usize {
        let c = rng.range(1, total - 1);
        if !cuts.contains(&c) {
            cuts.push(c);
        }
    }
    cuts.sort_unstable();
    cuts.push(total);
    let mut prev = 0;
    cuts.iter()
        .map(|&c| {
            let part = c - prev;
            prev = c;
            part
        })
        .collect()
}

/// Picks a change from `prev`'s rule to `new`'s with its change point after
/// walk index `after`, preferring `mode`. Returns the `from` date and the
/// transition, if any.
fn change(
    walk: &Walk,
    rng: &mut Rng,
    prev: &Shape,
    new: &Shape,
    after: usize,
    mode: Mode,
) -> Option<(Civil, Option<Transition>, Mode)> {
    let starts_on = starts_on(prev);
    let old_starts: Vec<usize> = rule_starts(walk, prev)
        .0
        .into_iter()
        .filter(|&i| i > after && i < after + 12 * 366)
        .collect();
    let new_starts = rule_starts(walk, new).0;
    let is_new: Vec<bool> = {
        let mut v = vec![false; walk.days.len()];
        for &i in &new_starts {
            v[i] = true;
        }
        v
    };
    let modes = match mode {
        Mode::Direct => [Mode::Direct, Mode::Months, Mode::Weeks],
        Mode::Months => [Mode::Months, Mode::Weeks, Mode::Direct],
        Mode::Weeks => [Mode::Weeks, Mode::Months, Mode::Direct],
    };
    for mode in modes {
        // Candidate (change point, from) pairs for the first old-rule years.
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        for &t in &old_starts {
            let tc = walk.days[t];
            let later = new_starts.iter().copied().filter(|&f| f > t);
            match mode {
                Mode::Direct => {
                    if is_new[t] {
                        pairs.push((t, t));
                    }
                }
                Mode::Months => {
                    if tc.d <= 28 {
                        // Up to the third matching new-rule start, at most
                        // three years on: transitions of 1 to 36 months.
                        pairs.extend(
                            later
                                .filter(|&f| walk.days[f].d == tc.d && f - t <= 3 * 366)
                                .take(3)
                                .map(|f| (t, f)),
                        );
                    }
                }
                Mode::Weeks => {
                    if walk.weekday[t] == starts_on {
                        pairs.extend(
                            later
                                .filter(|&f| (f - t) % 7 == 0 && f - t <= 6 * 371)
                                .take(2)
                                .map(|f| (t, f)),
                        );
                    }
                }
            }
            if pairs.len() >= 6 {
                break;
            }
        }
        if pairs.is_empty() {
            continue;
        }
        let (t, f) = rng.pick(&pairs);
        let (start, from) = (walk.days[t], walk.days[f]);
        let transition = match mode {
            Mode::Direct => None,
            Mode::Months => {
                let months = (from.y - start.y) as u32 * 12 + from.m - start.m;
                let n_quarters = rng.range(1, months.min(8));
                let quarters = compose(rng, months, n_quarters);
                Some(Transition {
                    start,
                    sub: TransitionSub::Months {
                        day: start.d,
                        quarters,
                    },
                })
            }
            Mode::Weeks => {
                let weeks = ((f - t) / 7) as u32;
                let n_periods = rng.range(1, weeks.min(20));
                let periods = compose(rng, weeks, n_periods);
                let n_quarters = rng.range(1, n_periods.min(8));
                let quarters = compose(rng, n_periods, n_quarters);
                Some(Transition {
                    start,
                    sub: TransitionSub::Weeks { periods, quarters },
                })
            }
        };
        return Some((from, transition, mode));
    }
    None
}

/// A new rule of `family`'s target kind for a change after walk index
/// `after`, shaped so that `mode` is likely to find a change.
fn new_shape(
    walk: &Walk,
    rng: &mut Rng,
    prev: &Shape,
    family: Family,
    mode: Mode,
    after: usize,
) -> Shape {
    let starts_on = starts_on(prev);
    let to_fixed = matches!(family, Family::FixedToFixed | Family::WeekToFixed);
    // A year start of the old rule soon after `after`.
    let near = || {
        let starts = rule_starts(walk, prev).0;
        let first = starts.iter().position(|&i| i > after).unwrap();
        walk.days[starts[first]]
    };
    match (family, mode) {
        // Same start, new subdivision: a direct change is always possible.
        (Family::FixedToFixed | Family::WeekToWeek, Mode::Direct) => resubdivided(rng, prev),
        // A calendar-months transition needs the same day of the month at
        // both ends.
        (Family::FixedToFixed, Mode::Months) => {
            let day = match prev {
                Shape::Fixed { day, .. } if *day <= 28 => Some(*day),
                _ => None,
            };
            fixed_shape(rng, starts_on, day)
        }
        // A new fixed date on the old rule's year start: they coincide.
        (Family::WeekToFixed, Mode::Direct) => {
            let c = near();
            let mut s = fixed_shape(rng, starts_on, Some(c.d.min(28)));
            if let Shape::Fixed { month, .. } = &mut s {
                *month = c.m;
            }
            s
        }
        // A fixed date on the day of the month of an old year start.
        (Family::WeekToFixed, Mode::Months) => fixed_shape(rng, starts_on, Some(near().d.min(28))),
        // A week-based anchor on the fixed date's day: the years coincide
        // whenever the fixed date falls on `starts_on`.
        (Family::FixedToWeek, Mode::Direct) => {
            let (m, d) = match prev {
                Shape::Fixed { month, day, .. } => (*month, (*day).min(28)),
                Shape::Week { .. } => unreachable!(),
            };
            let mut s = week_shape(rng, starts_on);
            if let Shape::Week {
                edge_end,
                month,
                day,
                rule,
                ..
            } = &mut s
            {
                *edge_end = false;
                *month = m;
                *day = Some(d);
                *rule = rng.pick(&["first", "last", "nearest"]);
            }
            s
        }
        _ if to_fixed => fixed_shape(rng, starts_on, None),
        _ => week_shape(rng, starts_on),
    }
}

fn date(c: Civil) -> String {
    format!("{:04}-{:02}-{:02}", c.y, c.m, c.d)
}

/// The `year` and `subdivision` blocks of a rule.
pub fn blocks(shape: &Shape) -> (Value, Value) {
    match shape {
        Shape::Week {
            edge_end,
            month,
            day,
            rule,
            periods,
            quarters,
            extra,
            ..
        } => (
            json!({"type": "week_based", "anchor": {
                "edge": if *edge_end { "end" } else { "start" },
                "month": month,
                "day": day.map_or(json!("last"), |d| json!(d)),
                "rule": rule,
            }}),
            json!({"type": "week_pattern", "periods": periods, "quarters": quarters,
                   "extra_week": {"period": extra}}),
        ),
        Shape::Fixed {
            month,
            day,
            next,
            quarters,
            ..
        } => {
            let mut year = json!({"type": "fixed_date", "start": {"month": month, "day": day}});
            if *day >= 29 {
                year["short_month"] = json!(if *next { "next" } else { "clamp" });
            }
            (
                year,
                json!({"type": "calendar_months", "month_start_day": day, "quarters": quarters}),
            )
        }
    }
}

/// Templates that use `{yy_end}` at every level.
pub fn yy_end_naming(year_number: &str) -> Value {
    json!({
        "default": "fy",
        "schemes": {
            "fy": {
                "year_number": year_number,
                "templates": {
                    "year": "FY{yyyy}-{yy_end}",
                    "quarter": "Q{q} FY{yyyy}-{yy_end}",
                    "period": "P{pp} FY{yyyy}-{yy_end}",
                    "week": "W{ww} FY{yyyy}-{yy_end}"
                }
            }
        }
    })
}

/// The JSON definition of `h`. `explicit_null` writes `"transition": null`
/// for a change without a transition instead of leaving the field out.
pub fn definition(h: &History, year_number: &str, explicit_null: bool) -> Value {
    let (year, subdivision) = blocks(&h.base);
    let history: Vec<Value> = h
        .entries
        .iter()
        .map(|e| {
            let (year, subdivision) = blocks(&e.shape);
            let mut entry = json!({"from": date(e.from), "year": year, "subdivision": subdivision});
            match &e.transition {
                Some(t) => {
                    let sub = match &t.sub {
                        TransitionSub::Months { day, quarters } => json!({
                            "type": "calendar_months", "month_start_day": day, "quarters": quarters
                        }),
                        TransitionSub::Weeks { periods, quarters } => json!({
                            "type": "week_pattern", "periods": periods, "quarters": quarters
                        }),
                    };
                    entry["transition"] = json!({"start": date(t.start), "subdivision": sub});
                }
                None if explicit_null => entry["transition"] = Value::Null,
                None => {}
            }
            entry
        })
        .collect();
    json!({
        "spec_version": 1,
        "year": year,
        "week": {"starts_on": h.starts_on(), "numbering": "contains_start"},
        "subdivision": subdivision,
        "naming": yy_end_naming(year_number),
        "exceptions": {"history": history}
    })
}

/// One generated case, or `None` when no valid change was found near the
/// drawn dates. `family` and `mode` steer the first change; a second change,
/// when drawn, picks its own.
pub fn history_case(
    walk: &Walk,
    seed: u64,
    family: Option<Family>,
    mode: Option<Mode>,
    entries: Option<usize>,
) -> Option<HistoryCase> {
    let mut rng = Rng::new(seed);
    let families = [
        Family::FixedToFixed,
        Family::WeekToWeek,
        Family::FixedToWeek,
        Family::WeekToFixed,
    ];
    let modes = [Mode::Direct, Mode::Months, Mode::Weeks];
    let mut family = family.unwrap_or_else(|| rng.pick(&families));
    let mut mode = mode.unwrap_or_else(|| rng.pick(&modes));
    let entries = entries.unwrap_or_else(|| rng.range(1, 2) as usize);
    let starts_on = NAMES[rng.below(7)];
    let base = match family {
        Family::FixedToFixed | Family::FixedToWeek => {
            // A calendar-months transition needs a day that every month has.
            let day = (mode == Mode::Months).then(|| rng.range(1, 28));
            fixed_shape(&mut rng, starts_on, day)
        }
        _ => week_shape(&mut rng, starts_on),
    };
    let year = rng.range(1950, 2100) as i32;
    let mut after = walk.at[&Civil {
        y: year,
        m: 1,
        d: 1,
    }] + rng.below(366);
    let mut h = History {
        base,
        entries: Vec::new(),
    };
    let mut changes = Vec::new();
    for k in 0..entries {
        if k > 0 {
            let prev_fixed = matches!(h.shape(k), Shape::Fixed { .. });
            family = match (prev_fixed, rng.below(2)) {
                (true, 0) => Family::FixedToFixed,
                (true, _) => Family::FixedToWeek,
                (false, 0) => Family::WeekToWeek,
                (false, _) => Family::WeekToFixed,
            };
            mode = rng.pick(&modes);
        }
        let prev = h.shape(k).clone();
        let new = new_shape(walk, &mut rng, &prev, family, mode, after);
        let (from, transition, found) = change(walk, &mut rng, &prev, &new, after, mode)?;
        changes.push((family, found));
        after = walk.at[&from];
        h.entries.push(Entry {
            from,
            transition,
            shape: new,
        });
    }
    let found = problems(walk, &h);
    assert!(
        found.is_empty(),
        "generated an invalid history: {found:?}\n{h:?}"
    );
    let definition = definition(&h, "start", seed % 2 == 0);
    Some(HistoryCase {
        definition,
        history: h,
        changes,
    })
}

/// `count` cases, cycling through every family and preferred mode with one or
/// two history entries.
pub fn history_cases(walk: &Walk, count: usize) -> Vec<HistoryCase> {
    let families = [
        Family::FixedToFixed,
        Family::WeekToWeek,
        Family::FixedToWeek,
        Family::WeekToFixed,
    ];
    let modes = [Mode::Direct, Mode::Months, Mode::Weeks];
    let mut out = Vec::with_capacity(count);
    let mut seed = 0u64;
    while out.len() < count {
        let i = out.len();
        let family = families[i % 4];
        let mode = modes[(i / 4) % 3];
        let entries = 1 + (i / 12) % 2;
        // Try seeds until one gives a valid change near its drawn dates.
        loop {
            seed += 1;
            if let Some(case) = history_case(walk, seed, Some(family), Some(mode), Some(entries)) {
                out.push(case);
                break;
            }
        }
    }
    out
}
