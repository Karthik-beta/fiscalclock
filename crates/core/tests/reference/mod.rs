//! A deliberately slow, obviously correct fiscal calendar.
//!
//! It walks the calendar one day at a time with its own date counter and
//! weekday counter, finds each boundary by scanning the days around its target
//! date, and labels every day by counting forward from the year's first day. It
//! shares no arithmetic with the crate: no day numbers, no weekday modulo maths
//! on dates, no cumulative week sums.

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Civil {
    pub y: i32,
    pub m: u32,
    pub d: u32,
}

fn leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn month_len(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ if leap(y) => 29,
        _ => 28,
    }
}

fn next_day(c: Civil) -> Civil {
    if c.d < month_len(c.y, c.m) {
        Civil { d: c.d + 1, ..c }
    } else if c.m < 12 {
        Civil {
            y: c.y,
            m: c.m + 1,
            d: 1,
        }
    } else {
        Civil {
            y: c.y + 1,
            m: 1,
            d: 1,
        }
    }
}

const NAMES: [&str; 7] = [
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
];

/// Every day from 1890-01-01 to 2210-12-31 with its weekday name.
pub struct Walk {
    days: Vec<Civil>,
    weekday: Vec<&'static str>,
    at: HashMap<Civil, usize>,
}

impl Walk {
    pub fn new() -> Walk {
        let mut days = Vec::new();
        let mut c = Civil {
            y: 1890,
            m: 1,
            d: 1,
        };
        while c.y <= 2210 {
            days.push(c);
            c = next_day(c);
        }
        let monday = days
            .iter()
            .position(|&c| {
                c == Civil {
                    y: 1900,
                    m: 1,
                    d: 1,
                }
            })
            .unwrap();
        // 1900-01-01 was a Monday. Count back to the first day, then forward.
        let mut k = 0usize;
        for _ in 0..monday {
            k = if k == 0 { 6 } else { k - 1 };
        }
        let mut weekday = Vec::with_capacity(days.len());
        for _ in 0..days.len() {
            weekday.push(NAMES[k]);
            k = if k == 6 { 0 } else { k + 1 };
        }
        let at = days.iter().enumerate().map(|(i, &c)| (c, i)).collect();
        Walk { days, weekday, at }
    }
}

pub enum Shape {
    Week {
        starts_on: &'static str,
        edge_end: bool,
        month: u32,
        /// `None` means the month's last day.
        day: Option<u32>,
        rule: &'static str,
        periods: Vec<u32>,
        quarters: Vec<u32>,
        extra: u32,
    },
    Fixed {
        starts_on: &'static str,
        month: u32,
        day: u32,
        next: bool,
        quarters: Vec<u32>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RefPeriod {
    pub index: Option<u32>,
    pub start: Civil,
    pub end_exclusive: Civil,
    pub days: u32,
    pub weeks: Option<u32>,
    pub has_extra_week: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RefDay {
    pub date: Civil,
    pub year_number: i32,
    pub year: RefPeriod,
    pub quarter: RefPeriod,
    pub period: RefPeriod,
    pub week: RefPeriod,
    pub day_of_year: u32,
    pub day_of_quarter: u32,
    pub day_of_period: u32,
    pub day_of_week: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Label {
    year_first: usize,
    quarter: u32,
    period: u32,
    week: u32,
    extra_week: bool,
}

/// How fiscal years are numbered: `"start"`, `"end"` or `"majority"`.
pub type Numbering = &'static str;

/// Why a numbering rule fails within the walked years.
#[derive(Debug, PartialEq, Eq)]
pub enum Refused {
    /// Two consecutive fiscal years share a number, or a number is skipped.
    Collision,
    /// A fiscal year has exactly half of its days in each of two calendar years.
    Tie,
}

/// The first day and number of every fiscal year that opens between 1892 and
/// 2208, or why `numbering` cannot number them.
pub fn numbers(
    walk: &Walk,
    shape: &Shape,
    numbering: Numbering,
) -> Result<Vec<(Civil, i32)>, Refused> {
    let (years, _) = years_of(walk, shape, numbering)?;
    Ok(years.into_iter().map(|(i, n)| (walk.days[i], n)).collect())
}

/// Labels every day from 1900-01-01 to 2200-12-31, or reports why `numbering`
/// cannot number the fiscal years that open between 1892 and 2208.
pub fn compute(walk: &Walk, shape: &Shape, numbering: Numbering) -> Result<Vec<RefDay>, Refused> {
    let n = walk.days.len();
    let (years, is_period_start) = years_of(walk, shape, numbering)?;
    let (starts_on, quarters) = match shape {
        Shape::Week {
            starts_on,
            quarters,
            ..
        }
        | Shape::Fixed {
            starts_on,
            quarters,
            ..
        } => (*starts_on, quarters),
    };
    let quarter_of = |p: u32| {
        let mut seen = 0;
        for (qi, size) in quarters.iter().enumerate() {
            seen += size;
            if p <= seen {
                return qi as u32 + 1;
            }
        }
        unreachable!()
    };

    let mut labels: Vec<Option<Label>> = vec![None; n];
    for w in years.windows(2) {
        let (first, next_first) = (w[0].0, w[1].0);
        let long = next_first - first == 371;
        let mut period = 1u32;
        let mut week = 1u32;
        let mut day_in_week = 0u32;
        let mut weeks_done_in_period = 0u32;
        for i in first..next_first {
            let mut extra_week = false;
            match shape {
                Shape::Week { periods, extra, .. } => {
                    if day_in_week == 7 {
                        day_in_week = 0;
                        week += 1;
                        weeks_done_in_period += 1;
                        let mut len = periods[period as usize - 1];
                        if long && period == *extra {
                            len += 1;
                        }
                        if weeks_done_in_period == len {
                            period += 1;
                            weeks_done_in_period = 0;
                        }
                    }
                    extra_week = long
                        && period == *extra
                        && weeks_done_in_period == periods[*extra as usize - 1];
                    day_in_week += 1;
                }
                Shape::Fixed { .. } => {
                    if i != first && walk.weekday[i] == starts_on {
                        week += 1;
                    }
                    if i != first && is_period_start[i] {
                        period += 1;
                    }
                }
            }
            labels[i] = Some(Label {
                year_first: first,
                quarter: quarter_of(period),
                period,
                week,
                extra_week,
            });
        }
    }

    let week_based = matches!(shape, Shape::Week { .. });
    let extra_period = match shape {
        Shape::Week { extra, .. } => *extra,
        Shape::Fixed { .. } => 0,
    };
    let numbers: HashMap<usize, i32> = years.iter().copied().collect();

    // The run of consecutive days around `i` that share `key`.
    let run = |i: usize, key: &dyn Fn(&Label) -> (usize, u32)| {
        let k = key(&labels[i].unwrap());
        let same = |j: usize| labels[j].map(|l| key(&l)) == Some(k);
        let mut s = i;
        while s > 0 && same(s - 1) {
            s -= 1;
        }
        let mut e = i + 1;
        while e < n && same(e) {
            e += 1;
        }
        (s, e)
    };

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
    // Every day of a period shares its bounds. Without this cache each day
    // re-scans its period, which makes the differential test much slower.
    let mut cache: HashMap<(u8, usize, u32), (usize, usize)> = HashMap::new();
    let mut out = Vec::with_capacity(to - from + 1);
    #[allow(clippy::needless_range_loop)] // `i` is also the day passed to `run`.
    for i in from..=to {
        let l = labels[i].unwrap();
        let mut bounds = |tag: u8, idx: u32, key: &dyn Fn(&Label) -> (usize, u32)| {
            *cache
                .entry((tag, l.year_first, idx))
                .or_insert_with(|| run(i, key))
        };
        let yb = bounds(0, 0, &|l| (l.year_first, 0));
        let qb = bounds(1, l.quarter, &|l| (l.year_first, l.quarter));
        let pb = bounds(2, l.period, &|l| (l.year_first, l.period));
        let wb = bounds(3, l.week, &|l| (l.year_first, l.week));
        let long = yb.1 - yb.0 == 371;
        let mk = |index: Option<u32>, (s, e): (usize, usize), extra: bool| RefPeriod {
            index,
            start: walk.days[s],
            end_exclusive: walk.days[e],
            days: (e - s) as u32,
            weeks: week_based.then_some(((e - s) / 7) as u32),
            has_extra_week: extra,
        };
        let mut day_of_week = 1;
        let mut j = i;
        while walk.weekday[j] != starts_on {
            j -= 1;
            day_of_week += 1;
        }
        out.push(RefDay {
            date: walk.days[i],
            year_number: numbers[&l.year_first],
            year: mk(None, yb, long),
            quarter: mk(
                Some(l.quarter),
                qb,
                long && quarter_of(extra_period.max(1)) == l.quarter && week_based,
            ),
            period: mk(Some(l.period), pb, long && l.period == extra_period),
            week: mk(Some(l.week), wb, l.extra_week),
            day_of_year: (i - yb.0 + 1) as u32,
            day_of_quarter: (i - qb.0 + 1) as u32,
            day_of_period: (i - pb.0 + 1) as u32,
            day_of_week,
        });
    }
    Ok(out)
}

#[allow(clippy::type_complexity)]
fn years_of(
    walk: &Walk,
    shape: &Shape,
    numbering: Numbering,
) -> Result<(Vec<(usize, i32)>, Vec<bool>), Refused> {
    let n = walk.days.len();
    // Each fiscal year as (index of its first day, year number), in order.
    let mut years: Vec<(usize, i32)> = Vec::new();
    let mut is_period_start = vec![false; n];
    let starts_on = match shape {
        Shape::Week { starts_on, .. } | Shape::Fixed { starts_on, .. } => *starts_on,
    };
    match shape {
        Shape::Week {
            edge_end,
            month,
            day,
            rule,
            ..
        } => {
            let pos = NAMES.iter().position(|&s| s == starts_on).unwrap();
            let qualifying = if *edge_end {
                NAMES[(pos + 6) % 7]
            } else {
                starts_on
            };
            let target = |y: i32| {
                walk.at[&Civil {
                    y,
                    m: *month,
                    d: day.unwrap_or(month_len(y, *month)),
                }]
            };
            let boundary = |y: i32| {
                let t = target(y);
                let hits = (t - 6..=t + 6).filter(|&i| walk.weekday[i] == qualifying);
                match *rule {
                    "last" => hits.filter(|&i| i <= t).max().unwrap(),
                    "first" => hits.filter(|&i| i >= t).min().unwrap(),
                    _ => hits.min_by_key(|&i| i.abs_diff(t)).unwrap(),
                }
            };
            for y in 1892..=2209 {
                // Year y: (B(y-1), B(y)] for edge "end", [B(y), B(y+1)) for "start".
                let first = if *edge_end {
                    boundary(y - 1) + 1
                } else {
                    boundary(y)
                };
                years.push((first, 0));
            }
        }
        Shape::Fixed {
            month, day, next, ..
        } => {
            let month_start = |y: i32, m: u32| {
                if *day <= month_len(y, m) {
                    walk.at[&Civil { y, m, d: *day }]
                } else {
                    let last = walk.at[&Civil {
                        y,
                        m,
                        d: month_len(y, m),
                    }];
                    if *next { last + 1 } else { last }
                }
            };
            for y in 1892..=2209 {
                let (mut yy, mut mm) = (y, *month);
                for _ in 0..12 {
                    is_period_start[month_start(yy, mm)] = true;
                    mm += 1;
                    if mm == 13 {
                        mm = 1;
                        yy += 1;
                    }
                }
                years.push((month_start(y, *month), 0));
            }
        }
    }

    // Number each year by counting its days per calendar year. Any even split
    // is reported before checking that consecutive years get consecutive numbers.
    if numbering == "majority" {
        for w in years.windows(2) {
            let mut held: HashMap<i32, usize> = HashMap::new();
            for j in w[0].0..w[1].0 {
                *held.entry(walk.days[j].y).or_default() += 1;
            }
            if held.values().any(|&c| c * 2 == w[1].0 - w[0].0) {
                return Err(Refused::Tie);
            }
        }
    }
    for i in 0..years.len() - 1 {
        let (first, next_first) = (years[i].0, years[i + 1].0);
        let mut held: Vec<(i32, usize)> = Vec::new();
        for j in first..next_first {
            match held.last_mut() {
                Some((y, count)) if *y == walk.days[j].y => *count += 1,
                _ => held.push((walk.days[j].y, 1)),
            }
        }
        let total = next_first - first;
        years[i].1 = match numbering {
            "start" => walk.days[first].y,
            "end" => walk.days[next_first - 1].y,
            _ => {
                if held.iter().any(|&(_, c)| c * 2 == total) {
                    return Err(Refused::Tie);
                }
                held.iter().find(|&&(_, c)| c * 2 > total).unwrap().0
            }
        };
        if i > 0 && years[i].1 != years[i - 1].1 + 1 {
            return Err(Refused::Collision);
        }
    }
    years.pop();
    Ok((years, is_period_start))
}
