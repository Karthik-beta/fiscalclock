//! The day-walking reference for rules with a `table` subdivision.
//!
//! The listed year starts and period starts are taken as given. Every walked
//! day is then labelled by scanning: its fiscal year is the last listed year
//! that starts on or before it, its period the number of listed periods of
//! that year that start on or before it, and its week one more than the number
//! of `week.starts_on` days after the year's first day up to it. Nothing is
//! shared with the crate's arithmetic.

use super::{Civil, Numbering, RefDay, RefPeriod, Refused, Walk};

/// A rule with a table subdivision, as lists of dates.
#[derive(Clone, Debug)]
pub struct Table {
    pub starts_on: &'static str,
    /// The first day of every fiscal year, then the day after the last one.
    pub years: Vec<Civil>,
    /// The first day of every period, then the day after the last one.
    pub periods: Vec<Civil>,
    pub quarters: Vec<u32>,
}

/// The number of every listed fiscal year under `numbering`, or why it fails.
pub fn numbers(walk: &Walk, t: &Table, numbering: Numbering) -> Result<Vec<i32>, Refused> {
    let mut out: Vec<i32> = Vec::new();
    for w in t.years.windows(2) {
        let (first, end) = (walk.at[&w[0]], walk.at[&w[1]]);
        let days = &walk.days[first..end];
        let number = match numbering {
            "start" => days[0].y,
            "end" => days[days.len() - 1].y,
            _ => {
                let mut held: Vec<(i32, usize)> = Vec::new();
                for c in days {
                    match held.iter_mut().find(|(y, _)| *y == c.y) {
                        Some((_, n)) => *n += 1,
                        None => held.push((c.y, 1)),
                    }
                }
                match held.iter().find(|&&(_, n)| n * 2 > days.len()) {
                    Some(&(y, _)) => y,
                    None => return Err(Refused::Tie),
                }
            }
        };
        if out.last().is_some_and(|&last| number != last + 1) {
            return Err(Refused::Collision);
        }
        out.push(number);
    }
    Ok(out)
}

/// Labels every day from the first listed year's first day to the last
/// listed year's last day.
pub fn compute(walk: &Walk, t: &Table, numbering: Numbering) -> Result<Vec<RefDay>, Refused> {
    let numbers = numbers(walk, t, numbering)?;
    let at = |c: &Civil| walk.at[c];
    let year_starts: Vec<usize> = t.years.iter().map(at).collect();
    let period_starts: Vec<usize> = t.periods.iter().map(at).collect();
    let mut out = Vec::new();
    for (yi, w) in year_starts.windows(2).enumerate() {
        let (ys, ye) = (w[0], w[1]);
        // This year's period starts, then its end.
        let mut ps: Vec<usize> = period_starts
            .iter()
            .copied()
            .filter(|&p| p >= ys && p < ye)
            .collect();
        ps.push(ye);
        for i in ys..ye {
            let k = ps.iter().filter(|&&p| p <= i).count();
            let mut q = 0;
            let mut seen = 0;
            while seen < k {
                seen += t.quarters[q] as usize;
                q += 1;
            }
            let q_first = t.quarters[..q - 1].iter().sum::<u32>() as usize;
            let q_last = q_first + t.quarters[q - 1] as usize;
            let mut week = 1;
            let mut week_start = ys;
            for (j, &name) in walk.weekday.iter().enumerate().take(i + 1).skip(ys + 1) {
                if name == t.starts_on {
                    week += 1;
                    week_start = j;
                }
            }
            let mut week_end = i + 1;
            while week_end < ye && walk.weekday[week_end] != t.starts_on {
                week_end += 1;
            }
            let mk = |index: Option<u32>, s: usize, e: usize| RefPeriod {
                index,
                start: walk.days[s],
                end_exclusive: walk.days[e],
                days: (e - s) as u32,
                weeks: None,
                has_extra_week: false,
            };
            let mut day_of_week = 1;
            let mut j = i;
            while walk.weekday[j] != t.starts_on {
                j -= 1;
                day_of_week += 1;
            }
            out.push(RefDay {
                date: walk.days[i],
                year_number: numbers[yi],
                year: mk(None, ys, ye),
                quarter: mk(Some(q as u32), ps[q_first], ps[q_last]),
                period: mk(Some(k as u32), ps[k - 1], ps[k]),
                week: mk(Some(week), week_start, week_end),
                day_of_year: (i - ys + 1) as u32,
                day_of_quarter: (i - ps[q_first] + 1) as u32,
                day_of_period: (i - ps[k - 1] + 1) as u32,
                day_of_week,
            });
        }
    }
    Ok(out)
}
