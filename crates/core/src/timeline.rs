//! The sequence of fiscal years a definition describes.
//!
//! A rule (a `year` block with its `subdivision`) produces one fiscal year for
//! every calendar year `Y`, its *rule key*. Without `exceptions` one rule
//! produces every fiscal year. With `exceptions` the years form segments in
//! date order: a run of years of one regime's rule, or a single transition
//! year. A fiscal year's identity is its [`Key`], an ordinal that increases by
//! one from each fiscal year to the next.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::calendar::Level;
use crate::date::{Date, Weekday, days_in_month};
use crate::definition::{
    AnchorDay, Definition, Edge, Rule, ShortMonth, Structure, Tabled, TransitionSubdivision,
    YearRule,
};

/// The internal identity of a fiscal year: an ordinal that increases by one
/// per fiscal year along the calendar's sequence of fiscal years. Kept
/// distinct from year numbers, which depend on the naming scheme and can skip
/// at a change of rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Key(pub i32);

/// A month counted from January of year 0, so that consecutive months differ by one.
pub(crate) const fn month_index(year: i32, month: u32) -> i32 {
    year * 12 + month as i32 - 1
}

/// The first day of a fiscal month that nominally starts on `day` of month
/// `index` (see [`month_index`]), with `short_month` applied when that month
/// is too short.
fn month_start(day: u32, short_month: Option<ShortMonth>, index: i32) -> Date {
    let year = index.div_euclid(12);
    let month = index.rem_euclid(12) as u32 + 1;
    let len = days_in_month(year, month);
    if day <= len {
        Date::civil(year, month, day)
    } else {
        match short_month {
            Some(ShortMonth::Next) => Date::civil(year, month, len).add(1),
            _ => Date::civil(year, month, len),
        }
    }
}

/// The boundary date `B(y)` of a `week_based` rule.
fn week_boundary(
    (edge, month, day, rule): (Edge, u32, AnchorDay, Rule),
    week_start: Weekday,
    y: i32,
) -> Date {
    let day = match day {
        AnchorDay::Day(d) => d,
        AnchorDay::Last => days_in_month(y, month),
    };
    let target = Date::civil(y, month, day);
    let qualifying = match edge {
        Edge::End => week_start.plus(-1),
        Edge::Start => week_start,
    };
    let forward = (qualifying.index() as i32 - target.weekday().index() as i32).rem_euclid(7);
    let back = if forward == 0 { 0 } else { forward - 7 };
    match rule {
        Rule::First => target.add(forward),
        Rule::Last => target.add(back),
        Rule::Nearest if forward <= 3 => target.add(forward),
        Rule::Nearest => target.add(back),
    }
}

/// `[start, end_exclusive)` of the fixed-date year with rule key `y`.
fn fixed_bounds(month: u32, day: u32, short_month: Option<ShortMonth>, y: i32) -> (Date, Date) {
    (
        month_start(day, short_month, month_index(y, month)),
        month_start(day, short_month, month_index(y + 1, month)),
    )
}

/// `[start, end_exclusive)` of the week-based year with rule key `y`.
fn week_bounds(anchor: (Edge, u32, AnchorDay, Rule), week_start: Weekday, y: i32) -> (Date, Date) {
    match anchor.0 {
        Edge::End => (
            week_boundary(anchor, week_start, y - 1).add(1),
            week_boundary(anchor, week_start, y).add(1),
        ),
        Edge::Start => (
            week_boundary(anchor, week_start, y),
            week_boundary(anchor, week_start, y + 1),
        ),
    }
}

/// The rule key of the year containing `date`, for a rule keyed by calendar year.
fn search_key(bounds: impl Fn(i32) -> (Date, Date), date: Date) -> i32 {
    let y = date.year();
    // A boundary lies within six days of its target, so a late-December
    // target with edge "start" can put a date in the year keyed y - 2, and
    // an early-January target with edge "end" in the year keyed y + 2.
    [y, y - 1, y + 1, y - 2, y + 2]
        .into_iter()
        .find(|&k| {
            let (s, e) = bounds(k);
            s <= date && date < e
        })
        .unwrap_or(y)
}

impl YearRule {
    /// `[start, end_exclusive)` of the fiscal year with rule key `y`. A table
    /// year's key is its index in the table; keys outside it are clamped.
    pub(crate) fn bounds(&self, week_start: Weekday, y: i32) -> (Date, Date) {
        match self {
            YearRule::Fixed {
                month,
                day,
                short_month,
            } => fixed_bounds(*month, *day, *short_month, y),
            YearRule::Week {
                edge,
                month,
                day,
                rule,
            } => week_bounds((*edge, *month, *day, *rule), week_start, y),
            YearRule::Table(b) => {
                let k = y.clamp(0, b.len() as i32 - 2) as usize;
                (b[k], b[k + 1])
            }
        }
    }

    /// The rule key of the fiscal year containing `date`. For a table year,
    /// −1 before the table and the number of listed years from its end on.
    pub(crate) fn key_of(&self, week_start: Weekday, date: Date) -> i32 {
        match self {
            YearRule::Table(b) => b.partition_point(|&d| d <= date) as i32 - 1,
            _ => search_key(|k| self.bounds(week_start, k), date),
        }
    }

    /// Whether `date` is the first day of a fiscal year under this rule; for
    /// a table year, of one of its listed years.
    pub(crate) fn is_year_start(&self, week_start: Weekday, date: Date) -> bool {
        match self {
            YearRule::Table(b) => b[..b.len() - 1].binary_search(&date).is_ok(),
            _ => self.bounds(week_start, self.key_of(week_start, date)).0 == date,
        }
    }

    /// A hint naming the fiscal year boundaries around `date`.
    pub(crate) fn boundary_hint(&self, week_start: Weekday, date: Date) -> String {
        match self {
            YearRule::Table(b) if date < b[0] || date >= b[b.len() - 1] => format!(
                "the year table lists fiscal years from {} to {}",
                b[0],
                b[b.len() - 1].add(-1)
            ),
            _ => {
                let (start, end) = self.bounds(week_start, self.key_of(week_start, date));
                format!("the fiscal years around {date} start on {start} and {end}")
            }
        }
    }

    /// `(start, end_exclusive)` of every fiscal year from the one starting on
    /// `first` to the one ending the day before `end`; both must be boundaries.
    pub(crate) fn years_between(
        &self,
        week_start: Weekday,
        first: Date,
        end: Date,
    ) -> Vec<(Date, Date)> {
        let lo = self.key_of(week_start, first);
        let hi = self.key_of(week_start, end);
        (lo..hi).map(|k| self.bounds(week_start, k)).collect()
    }
}

impl Structure {
    /// `[start, end_exclusive)` of the fiscal year with rule key `y`, the
    /// calendar year `Y` of the boundary rules in `spec/definition-v1.md`, or
    /// the index of a table year.
    pub(crate) fn bounds(&self, week_start: Weekday, y: i32) -> (Date, Date) {
        match self {
            Structure::FixedDate(f) => fixed_bounds(f.month, f.day, f.short_month, y),
            Structure::WeekBased(w) => week_bounds((w.edge, w.month, w.day, w.rule), week_start, y),
            Structure::Tabled(t) => t.year.bounds(week_start, y),
        }
    }

    /// The first day of the first year and the day after the last year that
    /// a rule with a table produces; `None` for a rule without one.
    pub(crate) fn table_range(&self) -> Option<(Date, Date)> {
        match self {
            Structure::Tabled(t) => Some((t.boundaries[0], t.boundaries[t.boundaries.len() - 1])),
            _ => None,
        }
    }

    /// The rule keys of the first and last years a rule with a table produces.
    fn key_range(&self, week_start: Weekday) -> Option<(i32, i32)> {
        let (first, end) = self.table_range()?;
        Some((
            self.key_of(week_start, first),
            self.key_of(week_start, end) - 1,
        ))
    }

    /// The rule key of the fiscal year containing `date`.
    pub(crate) fn key_of(&self, week_start: Weekday, date: Date) -> i32 {
        match self {
            Structure::Tabled(t) => t.year.key_of(week_start, date),
            _ => search_key(|k| self.bounds(week_start, k), date),
        }
    }

    /// `None` when `date` is the first day of a fiscal year under this rule,
    /// or the day after a table year's last listed year, otherwise the first
    /// days of the years that contain it and follow it.
    pub(crate) fn off_boundary(&self, week_start: Weekday, date: Date) -> Option<(Date, Date)> {
        if let Structure::Tabled(Tabled {
            year: YearRule::Table(b),
            ..
        }) = self
        {
            if b.last() == Some(&date) {
                return None;
            }
        }
        let (start, end) = self.bounds(week_start, self.key_of(week_start, date));
        (start != date).then_some((start, end))
    }
}

/// Where the years of a segment get their rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Source {
    /// The top-level `year` and `subdivision` (regime 0).
    TopLevel,
    /// The rule of `exceptions.history[i]`.
    History(usize),
    /// The transition of `exceptions.history[i]`.
    Transition(usize),
}

#[derive(Clone, Debug)]
enum Shape {
    /// Years produced by a rule; `first_rule_key` is the first year's rule key.
    Rule {
        rule: Structure,
        first_rule_key: i32,
    },
    /// One transition year.
    Transition(TransitionSubdivision),
}

impl Shape {
    /// The running totals of the subdivision shared by every year of the shape.
    fn totals(&self) -> Totals {
        let (quarters, periods): (&[u32], &[u32]) = match self {
            Shape::Rule {
                rule: Structure::FixedDate(f),
                ..
            } => (&f.quarters, &[]),
            Shape::Rule {
                rule: Structure::WeekBased(w),
                ..
            } => (&w.quarters, &w.periods),
            Shape::Rule {
                rule: Structure::Tabled(t),
                ..
            } => (&t.quarters, &[]),
            Shape::Transition(TransitionSubdivision::CalendarMonths { quarters, .. }) => {
                (quarters, &[])
            }
            Shape::Transition(TransitionSubdivision::WeekPattern { periods, quarters }) => {
                (quarters, periods)
            }
        };
        Totals {
            quarters: running_totals(quarters),
            weeks: running_totals(periods),
        }
    }
}

/// Running totals of a subdivision, worked out once per segment so that
/// finding a quarter's periods or a period's weeks costs one lookup, and
/// finding the quarter of a period or the period of a week a binary search,
/// however many quarters or periods a transition has.
#[derive(Clone, Debug)]
struct Totals {
    /// Entry `q` is the number of periods in quarters 1 to `q`.
    quarters: Vec<u32>,
    /// For runs of weeks, entry `k` is the number of weeks in periods 1 to
    /// `k`, without the extra week; just `[0]` for fiscal months.
    weeks: Vec<u32>,
}

/// `[0, a, a + b, …]` for sizes `[a, b, …]`: entry `n` is the sum of the
/// first `n` sizes. Validation makes every sum a number of periods or weeks
/// of one fiscal year, so none overflows.
fn running_totals(sizes: &[u32]) -> Vec<u32> {
    core::iter::once(0)
        .chain(sizes.iter().scan(0, |total, &n| {
            *total += n;
            Some(*total)
        }))
        .collect()
}

/// A run of consecutive fiscal years from one source.
#[derive(Clone, Debug)]
pub(crate) struct Segment {
    pub source: Source,
    pub first: Key,
    pub last: Key,
    /// The first day of the first year.
    pub start: Date,
    /// The day after the last day of the last year.
    pub end: Date,
    shape: Shape,
    totals: Totals,
}

/// The supported fiscal years of a calendar, as segments in date order that
/// tile `[segments[0].start, segments[last].end)`.
#[derive(Clone, Debug)]
pub(crate) struct Timeline {
    week_start: Weekday,
    segments: Vec<Segment>,
}

impl Timeline {
    /// The timeline of a definition that has passed the cross-block checks,
    /// keeping only the fiscal years that lie wholly between 0001-01-02 and
    /// 9999-12-30.
    pub(crate) fn new(def: &Definition) -> Timeline {
        let week_start = def.week_start;
        let first_day = Date::civil(1, 1, 2);
        // Year boundaries move a few days either side of the target date, so
        // the first and last supported rule keys are searched for.
        let top = &def.structure;
        let lowest = top.key_range(week_start).map_or_else(
            || {
                (-3..=5)
                    .find(|&k| top.bounds(week_start, k).0 >= first_day)
                    .unwrap_or(5)
            },
            |(lo, _)| lo,
        );
        let mut timeline = Timeline {
            week_start,
            segments: Vec::with_capacity(2 * def.history.len() + 1),
        };
        let mut next = Key(lowest);
        for regime in 0..=def.history.len() {
            let (source, rule, lo) = match regime.checked_sub(1) {
                None => (Source::TopLevel, top, lowest),
                Some(i) => {
                    let change = &def.history[i];
                    if let Some(t) = &change.transition {
                        next = timeline.push(
                            next,
                            Source::Transition(i),
                            Shape::Transition(t.subdivision.clone()),
                            (t.start, change.from),
                            1,
                        );
                    }
                    let rule = &change.structure;
                    (
                        Source::History(i),
                        rule,
                        rule.key_of(week_start, change.from),
                    )
                }
            };
            let hi = match def.history.get(regime) {
                Some(change) => rule.key_of(week_start, change.change_point()) - 1,
                None => rule.key_range(week_start).map_or_else(
                    || {
                        (9995..=10_003)
                            .rev()
                            .find(|&k| rule.bounds(week_start, k).1 <= Date::MAX)
                            .unwrap_or(9995)
                    },
                    |(_, hi)| hi,
                ),
            };
            if lo <= hi {
                next = timeline.push(
                    next,
                    source,
                    Shape::Rule {
                        rule: rule.clone(),
                        first_rule_key: lo,
                    },
                    (rule.bounds(week_start, lo).0, rule.bounds(week_start, hi).1),
                    hi - lo + 1,
                );
            }
        }
        timeline
    }

    /// Appends a segment of `years` years and returns the key after its last.
    fn push(
        &mut self,
        first: Key,
        source: Source,
        shape: Shape,
        (start, end): (Date, Date),
        years: i32,
    ) -> Key {
        let last = Key(first.0 + years - 1);
        self.segments.push(Segment {
            source,
            first,
            last,
            start,
            end,
            totals: shape.totals(),
            shape,
        });
        Key(last.0 + 1)
    }

    pub(crate) fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// The fiscal year `key`, or `None` outside the supported range.
    pub(crate) fn year(&self, key: Key) -> Option<FiscalYear<'_>> {
        let i = self
            .segments
            .partition_point(|s| s.first <= key)
            .checked_sub(1)?;
        (key <= self.segments[i].last).then(|| self.year_in(i, key))
    }

    /// The fiscal year containing `date`, or `None` outside the supported range.
    pub(crate) fn year_at(&self, date: Date) -> Option<FiscalYear<'_>> {
        let i = self
            .segments
            .partition_point(|s| s.start <= date)
            .checked_sub(1)?;
        let seg = &self.segments[i];
        if date >= seg.end {
            return None;
        }
        let key = match &seg.shape {
            Shape::Rule {
                rule,
                first_rule_key,
            } => Key(seg.first.0 + rule.key_of(self.week_start, date) - first_rule_key),
            Shape::Transition(_) => seg.first,
        };
        Some(self.year_in(i, key))
    }

    /// Every supported fiscal year, in order.
    pub(crate) fn years(&self) -> impl Iterator<Item = FiscalYear<'_>> {
        self.segments
            .iter()
            .enumerate()
            .flat_map(move |(i, s)| (s.first.0..=s.last.0).map(move |k| self.year_in(i, Key(k))))
    }

    /// An upper bound on the number of periods at `level` in the supported
    /// range, so that `shift` can reject a step that must leave it.
    pub(crate) fn capacity(&self, level: Level) -> u64 {
        self.segments
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let years = u64::try_from(s.last.0 - s.first.0 + 1).unwrap_or(0);
                let per_year = match (&s.shape, level) {
                    // Listed years can have any length, so their weeks are counted.
                    (
                        Shape::Rule {
                            rule:
                                Structure::Tabled(Tabled {
                                    year: YearRule::Table(_),
                                    ..
                                }),
                            ..
                        },
                        Level::Week,
                    ) => {
                        return (s.first.0..=s.last.0)
                            .map(|k| u64::from(self.year_in(i, Key(k)).count(level)))
                            .sum();
                    }
                    // 53 or 54 weeks in a fixed_date year, 52 or 53 in a week_based one.
                    (Shape::Rule { .. }, Level::Week) => 54,
                    // Constant within a segment for the other levels.
                    _ => u64::from(self.year_in(i, s.first).count(level)),
                };
                years * per_year
            })
            .sum()
    }

    /// Fiscal year `key` of segment `i`; `key` must lie in that segment.
    pub(crate) fn year_in(&self, i: usize, key: Key) -> FiscalYear<'_> {
        let seg = &self.segments[i];
        let position = key.0 - seg.first.0;
        let week_totals = &seg.totals.weeks[..];
        let (start, end, division) = match &seg.shape {
            Shape::Rule {
                rule,
                first_rule_key,
            } => {
                let y = first_rule_key + position;
                let (start, end) = rule.bounds(self.week_start, y);
                let division = match rule {
                    Structure::FixedDate(f) => Division::Months {
                        first: month_index(y, f.month),
                        day: f.day,
                        short_month: f.short_month,
                    },
                    Structure::WeekBased(w) => Division::Weeks {
                        week_totals,
                        extra_period: Some(w.extra_period),
                        has_extra: start.days_until(end) == 371,
                    },
                    Structure::Tabled(t) => {
                        // Validation lists every year's first day and gives
                        // every year the same number of periods.
                        let first = t.boundaries.binary_search(&start).unwrap_or(0);
                        let periods = seg.totals.quarters[seg.totals.quarters.len() - 1] as usize;
                        Division::Table {
                            starts: &t.boundaries[first..=first + periods],
                        }
                    }
                };
                (start, end, division)
            }
            Shape::Transition(sub) => {
                let division = match sub {
                    TransitionSubdivision::CalendarMonths {
                        month_start_day, ..
                    } => Division::Months {
                        first: month_index(seg.start.year(), seg.start.month()),
                        day: *month_start_day,
                        short_month: None,
                    },
                    TransitionSubdivision::WeekPattern { .. } => Division::Weeks {
                        week_totals,
                        extra_period: None,
                        has_extra: false,
                    },
                };
                (seg.start, seg.end, division)
            }
        };
        let back = (start.weekday().index() as i32 - self.week_start.index() as i32).rem_euclid(7);
        FiscalYear {
            key,
            segment: i,
            position,
            source: seg.source,
            start,
            end,
            origin: start.add(-back),
            quarter_totals: &seg.totals.quarters,
            division,
        }
    }
}

/// How a fiscal year is split into periods.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Division<'a> {
    /// Fiscal months: period `k` starts on `day` of month `first + k - 1` (see
    /// [`month_index`]), adjusted by `short_month`; the last ends with the year.
    Months {
        first: i32,
        day: u32,
        short_month: Option<ShortMonth>,
    },
    /// Runs of whole weeks from the year's first day.
    Weeks {
        /// Entry `k` is the number of weeks the pattern gives periods 1 to
        /// `k`, without the extra week (see [`running_totals`]).
        week_totals: &'a [u32],
        /// The rule's `extra_week.period`; `None` in a transition.
        extra_period: Option<u32>,
        /// Whether this year is a 371-day year of a `week_based` rule.
        has_extra: bool,
    },
    /// Listed periods: period `k` runs from `starts[k - 1]` to the day before
    /// `starts[k]`; the last entry is the year's `end_exclusive`.
    Table { starts: &'a [Date] },
}

/// Size `k` (from 1) of the sizes whose running totals are `totals` (see
/// [`running_totals`]).
pub(crate) fn nth_size(totals: &[u32], k: u32) -> u32 {
    totals[k as usize] - totals[k as usize - 1]
}

/// One fiscal year with everything needed to subdivide it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FiscalYear<'a> {
    pub key: Key,
    /// Index of its segment in the timeline.
    pub segment: usize,
    /// Position within its segment, from 0.
    pub position: i32,
    pub source: Source,
    pub start: Date,
    pub end: Date,
    /// The `week.starts_on` day on or before `start`: week `w` ends the day
    /// before `origin + 7w`.
    origin: Date,
    /// Entry `q` is the number of periods in quarters 1 to `q` (see
    /// [`running_totals`]).
    quarter_totals: &'a [u32],
    pub division: Division<'a>,
}

impl FiscalYear<'_> {
    /// The calendar year of the last day, for `{yy_end}`.
    pub(crate) fn last_day_year(&self) -> i32 {
        self.end.add(-1).year()
    }

    /// Whether the year is split into runs of weeks (`week_pattern`).
    pub(crate) fn is_week_pattern(&self) -> bool {
        matches!(self.division, Division::Weeks { .. })
    }

    /// The period that holds this year's extra week, if it has one.
    pub(crate) fn extra_period(&self) -> Option<u32> {
        match self.division {
            Division::Weeks {
                extra_period,
                has_extra: true,
                ..
            } => extra_period,
            _ => None,
        }
    }

    /// The number of quarters, periods or weeks; 1 for the year itself.
    pub(crate) fn count(&self, level: Level) -> u32 {
        // `running_totals` starts with 0, so the list is never empty.
        let quarters = self.quarter_totals.len() - 1;
        match level {
            Level::Year => 1,
            Level::Quarter => quarters as u32,
            // Validation checks that the quarters partition the periods, so the
            // last quarter ends with the last period.
            Level::Period => self.quarter_totals[quarters],
            Level::Week => (self.origin.days_until(self.end) as u32).div_ceil(7),
        }
    }

    /// First and last period of quarter `q`.
    pub(crate) fn quarter_periods(&self, q: u32) -> (u32, u32) {
        let q = q as usize;
        (self.quarter_totals[q - 1] + 1, self.quarter_totals[q])
    }

    /// The quarter holding period `k`: the first whose periods reach `k`.
    pub(crate) fn quarter_of_period(&self, k: u32) -> u32 {
        self.quarter_totals.partition_point(|&total| total < k) as u32
    }

    /// The number of weeks in periods 1 to `k` of a year of runs of weeks,
    /// including the extra week when one of those periods holds it; 0 for
    /// fiscal months. Every week count of such a year comes from here.
    fn weeks_through(&self, k: u32) -> u32 {
        match self.division {
            Division::Weeks { week_totals, .. } => {
                week_totals[k as usize] + u32::from(self.extra_period().is_some_and(|e| e <= k))
            }
            Division::Months { .. } | Division::Table { .. } => 0,
        }
    }

    pub(crate) fn period_bounds(&self, k: u32) -> (Date, Date) {
        match self.division {
            Division::Months {
                first,
                day,
                short_month,
            } => {
                let at = |i: u32| month_start(day, short_month, first + i as i32);
                let end = if k >= self.count(Level::Period) {
                    self.end
                } else {
                    at(k)
                };
                (at(k - 1), end)
            }
            Division::Weeks { .. } => {
                let after_weeks = |n: u32| self.start.add(7 * n as i32);
                (
                    after_weeks(self.weeks_through(k - 1)),
                    after_weeks(self.weeks_through(k)),
                )
            }
            Division::Table { starts } => (starts[k as usize - 1], starts[k as usize]),
        }
    }

    /// The period containing `date`, which must lie in the year.
    pub(crate) fn period_of(&self, date: Date) -> u32 {
        let periods = self.count(Level::Period);
        match self.division {
            Division::Months {
                first,
                day,
                short_month,
            } => {
                // The fiscal month that starts in the date's own month begins
                // on or after that month's first day, and the one before it on
                // or before, so the date is in one of those two.
                let m = month_index(date.year(), date.month()) - first;
                let k = if date >= month_start(day, short_month, first + m) {
                    m + 1
                } else {
                    m
                };
                k.clamp(1, periods as i32) as u32
            }
            Division::Weeks { .. } => {
                // A binary search for the first period whose weeks run past
                // the date's week, counted from 0.
                let week = self.start.days_until(date) as u32 / 7;
                let (mut lo, mut hi) = (1, periods);
                while lo < hi {
                    let mid = lo + (hi - lo) / 2;
                    if self.weeks_through(mid) > week {
                        hi = mid;
                    } else {
                        lo = mid + 1;
                    }
                }
                lo
            }
            // The number of listed periods that start on or before the date.
            Division::Table { starts } => {
                starts[..periods as usize].partition_point(|&d| d <= date) as u32
            }
        }
    }

    pub(crate) fn week_bounds(&self, w: u32) -> (Date, Date) {
        (
            self.origin.add(7 * (w as i32 - 1)).max(self.start),
            self.origin.add(7 * w as i32).min(self.end),
        )
    }

    /// The week containing `date`, which must lie in the year.
    pub(crate) fn week_of(&self, date: Date) -> u32 {
        (self.origin.days_until(date) / 7 + 1) as u32
    }

    /// The number of the extra week, if the year has one: the last week of
    /// the period that holds it.
    pub(crate) fn extra_week(&self) -> Option<u32> {
        self.extra_period().map(|p| self.weeks_through(p))
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;
    use crate::calendar::FiscalCalendar;

    fn timeline(def: &str) -> Timeline {
        Timeline::new(FiscalCalendar::from_json(def).unwrap().definition())
    }

    fn fixed(day: u32, short_month: &str) -> Timeline {
        timeline(&format!(
            r#"{{"spec_version":1,
            "year":{{"type":"fixed_date","start":{{"month":3,"day":{day}}},"short_month":{short_month}}},
            "week":{{"starts_on":"wednesday","numbering":"contains_start"}},
            "subdivision":{{"type":"calendar_months","month_start_day":{day},"quarters":[3,3,3,3]}},
            "naming":{{"default":"fy","schemes":{{"fy":{{"year_number":"start","templates":{{"year":"FY{{yyyy}}"}}}}}}}}}}"#
        ))
    }

    /// `period_of` and `week_of` agree with a search through the bounds, for
    /// every day of several years and every short-month policy.
    #[test]
    fn locating_a_day_agrees_with_searching_the_bounds() {
        for t in [
            fixed(1, "null"),
            fixed(15, "null"),
            fixed(29, "\"clamp\""),
            fixed(31, "\"clamp\""),
            fixed(30, "\"next\""),
            fixed(31, "\"next\""),
        ] {
            let mut date = Date::civil(1999, 1, 1);
            while date < Date::civil(2005, 1, 1) {
                let fy = t.year_at(date).unwrap();
                let k = fy.period_of(date);
                let found = (1..=fy.count(Level::Period))
                    .find(|&k| fy.period_bounds(k).1 > date)
                    .unwrap();
                assert_eq!(k, found, "{date}");
                let (s, e) = fy.period_bounds(k);
                assert!(s <= date && date < e, "{date}");
                let (s, e) = fy.week_bounds(fy.week_of(date));
                assert!(s <= date && date < e, "{date}");
                date = date.add(1);
            }
        }
    }

    /// Week 1 ends the day before the first `week.starts_on` after the year's
    /// first day; later weeks have seven days except the last.
    #[test]
    fn weeks_tile_the_year() {
        let t = fixed(1, "null");
        for fy in t
            .years()
            .filter(|fy| (1990..2010).contains(&fy.start.year()))
        {
            let n = fy.count(Level::Week);
            let mut at = fy.start;
            for w in 1..=n {
                let (s, e) = fy.week_bounds(w);
                assert_eq!(s, at);
                assert!(s < e && s.days_until(e) <= 7);
                if w > 1 {
                    assert_eq!(s.weekday(), Weekday::Wednesday);
                }
                if w < n {
                    assert_eq!(e.weekday(), Weekday::Wednesday);
                }
                at = e;
            }
            assert_eq!(at, fy.end);
        }
    }
}
