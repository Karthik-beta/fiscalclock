use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::date::Date;
use crate::definition::{Definition, Scheme, YearNumber};
use crate::error::{Candidate, Details, Error, ErrorCode, Errors};
use crate::label::Captures;
use crate::timeline::{FiscalYear, Key, Source, Timeline};

/// A level of the fiscal hierarchy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    /// The fiscal year.
    Year,
    /// A quarter: a run of periods.
    Quarter,
    /// A period: a fiscal month, or a run of weeks in a week-based year.
    Period,
    /// A week.
    Week,
}

impl Level {
    /// Every level, from the year down to the week.
    pub const ALL: [Level; 4] = [Level::Year, Level::Quarter, Level::Period, Level::Week];

    /// The name used in JSON: `"year"`, `"quarter"`, `"period"` or `"week"`.
    pub const fn name(self) -> &'static str {
        match self {
            Level::Year => "year",
            Level::Quarter => "quarter",
            Level::Period => "period",
            Level::Week => "week",
        }
    }

    /// Parses a level name as used in JSON.
    pub fn from_name(s: &str) -> Option<Level> {
        Level::ALL.into_iter().find(|l| l.name() == s)
    }

    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

/// A fiscal period: the half-open interval `[start, end_exclusive)` at one level.
///
/// A plain value. Operations that take a `Period` recompute it from
/// `(level, year, index)` and ignore the other fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Period {
    /// The level.
    pub level: Level,
    /// The fiscal year number under the default naming scheme, including its offset.
    pub year: i32,
    /// 1-based position within the fiscal year; `None` for a year.
    pub index: Option<u32>,
    /// The first day.
    pub start: Date,
    /// The day after the last day.
    pub end_exclusive: Date,
    /// The number of days.
    pub days: u32,
    /// The number of weeks when the fiscal year is subdivided by a week
    /// pattern (a `week_based` year or a `week_pattern` transition); `None`
    /// otherwise.
    pub weeks: Option<u32>,
    /// For a year, quarter or period: whether it contains the extra week. For a
    /// week: whether it is the extra week. Only a 371-day `week_based` year has
    /// an extra week.
    pub has_extra_week: bool,
}

impl Period {
    /// The last day, `end_exclusive` minus one day.
    pub fn last_day(&self) -> Date {
        self.end_exclusive.add(-1)
    }
}

/// Where a date falls at every level, returned by [`FiscalCalendar::locate`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Location {
    /// The date located.
    pub date: Date,
    /// The fiscal year containing the date.
    pub year: Period,
    /// The quarter containing the date.
    pub quarter: Period,
    /// The period containing the date.
    pub period: Period,
    /// The week containing the date.
    pub week: Period,
    /// 1-based day within the year.
    pub day_of_year: u32,
    /// 1-based day within the quarter.
    pub day_of_quarter: u32,
    /// 1-based day within the period.
    pub day_of_period: u32,
    /// 1 on `week.starts_on` up to 7 on the day before it.
    pub day_of_week: u32,
}

/// A fiscal calendar built from a validated [`Definition`].
#[derive(Clone, Debug)]
pub struct FiscalCalendar {
    pub(crate) def: Definition,
    canonical: String,
    id: String,
    timeline: Timeline,
}

/// What a year number names under a scheme.
enum Numbered<'a> {
    /// The fiscal year with that number.
    Year(FiscalYear<'a>),
    /// No fiscal year: the number is skipped at a change of rule. `next` is
    /// the first fiscal year numbered above it; `before` and `after` are the
    /// numbers either side of the gap.
    Skipped { next: Key, before: i64, after: i64 },
    /// Below the first or above the last supported number.
    OutOfRange,
}

const MAX_YEARS: i64 = 10_000;
const MAX_DIMENSION_DAYS: i32 = 146_097;

impl FiscalCalendar {
    /// Validates a JSON definition and builds the calendar.
    ///
    /// # Errors
    ///
    /// Returns every validation error found, sorted by path and then code. Any
    /// code the error catalogue lists as raised by `validate` can appear, from
    /// `INVALID_JSON` to `CHANGE_NOT_ON_BOUNDARY`.
    pub fn from_json(definition: &str) -> Result<FiscalCalendar, Errors> {
        Definition::parse(definition)
            .and_then(FiscalCalendar::numbered)
            .map(FiscalCalendar::with_identity)
    }

    /// Builds the calendar from a validated definition.
    pub fn new(def: Definition) -> FiscalCalendar {
        FiscalCalendar::with_timeline(def).with_identity()
    }

    /// Builds the calendar from a parsed definition, checking that every scheme
    /// numbers the supported fiscal years by the rules of the spec and
    /// recording each scheme's numbers. The canonical form and `calendar_id`
    /// are not computed yet.
    pub(crate) fn numbered(def: Definition) -> Result<FiscalCalendar, Errors> {
        let mut cal = FiscalCalendar::with_timeline(def);
        let numbers = cal.scheme_numbers().map_err(Errors::sorted)?;
        for (scheme, numbers) in cal.def.schemes.iter_mut().zip(numbers) {
            scheme.numbers = numbers;
        }
        Ok(cal)
    }

    fn with_identity(mut self) -> FiscalCalendar {
        self.canonical = self.def.canonical_json();
        self.id = crate::definition::calendar_id_of(&self.canonical);
        self
    }

    /// The calendar with its timeline worked out and no identity yet.
    fn with_timeline(def: Definition) -> FiscalCalendar {
        FiscalCalendar {
            timeline: Timeline::new(&def),
            def,
            canonical: String::new(),
            id: String::new(),
        }
    }

    /// The definition this calendar was built from.
    pub fn definition(&self) -> &Definition {
        &self.def
    }

    /// The calendar's `calendar_id`.
    pub fn calendar_id(&self) -> &str {
        &self.id
    }

    /// The canonical JSON form of the definition.
    pub fn canonical_json(&self) -> &str {
        &self.canonical
    }

    /// The calendar year that numbers `fy` under `rule`, before any offset;
    /// `None` when a majority rule finds no calendar year with more than half
    /// of the days.
    fn raw_number(fy: &FiscalYear<'_>, rule: YearNumber) -> Option<i64> {
        let (start, end) = (fy.start, fy.end);
        match rule {
            YearNumber::Start => Some(i64::from(start.year())),
            YearNumber::End => Some(i64::from(end.add(-1).year())),
            YearNumber::Majority => {
                let total = start.days_until(end);
                let mut from = start;
                while from < end {
                    let y = from.year();
                    let next = Date::civil(y + 1, 1, 1).min(end);
                    let held = from.days_until(next);
                    if 2 * held > total {
                        return Some(i64::from(y));
                    }
                    if 2 * held == total {
                        return None;
                    }
                    from = next;
                }
                None
            }
        }
    }

    /// Numbers every supported fiscal year under `rule` and checks the
    /// numbering rules: within a regime each year is numbered one more than
    /// the one before it, and across a change of rule the number increases.
    /// Returns the number of each segment's first year, before any offset, or
    /// each problem found with its first affected years (before offset).
    fn rule_numbers(&self, rule: YearNumber) -> Result<Vec<i64>, Vec<(ErrorCode, Vec<i64>)>> {
        const SHOWN: usize = 5;
        let mut ties = Vec::new();
        let mut clashes: Vec<i64> = Vec::new();
        let mut firsts = Vec::with_capacity(self.timeline.segments().len());
        // The previous year's number and segment.
        let mut prev: Option<(i64, usize)> = None;
        for fy in self.timeline.years() {
            let n = Self::raw_number(&fy, rule);
            if fy.position == 0 {
                firsts.push(n.unwrap_or(0));
            }
            let Some(n) = n else {
                if ties.len() < SHOWN {
                    ties.push(i64::from(fy.start.year()));
                }
                prev = None;
                continue;
            };
            let affected = match prev {
                // Two years of one regime: consecutive numbers.
                Some((p, segment)) if segment == fy.segment => {
                    if n == p {
                        Some(n)
                    } else if n != p + 1 {
                        Some(p + 1)
                    } else {
                        None
                    }
                }
                // A change of rule lies between them: any increase.
                Some((p, _)) if n <= p => Some(n),
                _ => None,
            };
            if let Some(a) = affected {
                if clashes.len() < SHOWN && !clashes.contains(&a) {
                    clashes.push(a);
                }
            }
            prev = Some((n, fy.segment));
        }
        // A tie leaves years unnumbered, so it supersedes any collision report.
        if !ties.is_empty() {
            return Err(alloc::vec![(ErrorCode::MajorityTie, ties)]);
        }
        if !clashes.is_empty() {
            return Err(alloc::vec![(ErrorCode::YearNumberCollision, clashes)]);
        }
        Ok(firsts)
    }

    /// The numbers of every scheme, or the numbering errors of the schemes that fail.
    pub(crate) fn scheme_numbers(&self) -> Result<Vec<Vec<i64>>, Vec<Error>> {
        let mut numbers = Vec::new();
        let mut errors = Vec::new();
        for scheme in &self.def.schemes {
            match self.rule_numbers(scheme.year_number) {
                Ok(firsts) => numbers.push(firsts),
                Err(problems) => {
                    let alternative = YearNumber::ALL
                        .iter()
                        .copied()
                        .find(|&r| r != scheme.year_number && self.rule_numbers(r).is_ok());
                    let hint = match alternative {
                        Some(alt) => format!(
                            "use \"year_number\": \"{}\", which numbers every fiscal year in the range uniquely",
                            alt.name()
                        ),
                        None => String::from(
                            "no year_number rule numbers this calendar uniquely; table numbering is planned for fiscalclock 0.3",
                        ),
                    };
                    for (code, years) in problems {
                        let (message, years) = if code == ErrorCode::MajorityTie {
                            // A transition can span three or more calendar years.
                            let reason = if self.def.history.is_empty() {
                                "that has exactly half of its days in each of two calendar years"
                            } else {
                                "in which no calendar year holds more than half of its days"
                            };
                            (
                                format!(
                                    "year_number \"majority\" cannot number a fiscal year {reason}"
                                ),
                                years,
                            )
                        } else {
                            (
                                format!(
                                    "year_number \"{}\" gives two fiscal years the same number, or skips a number, within the supported range",
                                    scheme.year_number.name()
                                ),
                                years
                                    .into_iter()
                                    .map(|y| y + i64::from(scheme.offset))
                                    .collect(),
                            )
                        };
                        let mut e = Error::new(
                            code,
                            format!("naming.schemes.{}.year_number", scheme.name),
                            message,
                        )
                        .hint(hint.clone());
                        e.details = Some(Details::Years(years));
                        errors.push(e);
                    }
                }
            }
        }
        if errors.is_empty() {
            Ok(numbers)
        } else {
            Err(errors)
        }
    }

    /// The number of `fy` under `scheme`, including its offset.
    pub(crate) fn number(&self, fy: &FiscalYear<'_>, scheme: &Scheme) -> i64 {
        // `numbered` records each scheme's first number for every segment of
        // the timeline, and every `Definition` has been through it, since
        // `Definition::from_json` runs it too.
        debug_assert_eq!(scheme.numbers.len(), self.timeline.segments().len());
        scheme.numbers[fy.segment] + i64::from(fy.position) + i64::from(scheme.offset)
    }

    pub(crate) fn default_scheme(&self) -> &Scheme {
        &self.def.schemes[self.def.default_scheme]
    }

    fn default_number(&self, fy: &FiscalYear<'_>) -> i32 {
        // Offsets are limited to +-1e9 at validation, so this fits.
        self.number(fy, self.default_scheme()) as i32
    }

    /// What `number` names under `scheme`.
    fn find_number(&self, number: i64, scheme: &Scheme) -> Numbered<'_> {
        let n = number.saturating_sub(i64::from(scheme.offset));
        // Segment numbers increase, so the segment that could hold `n` is the
        // last one whose first year is numbered `n` or less. `scheme.numbers`
        // has one entry per segment (see `number`).
        let Some(i) = scheme
            .numbers
            .partition_point(|&first| first <= n)
            .checked_sub(1)
        else {
            return Numbered::OutOfRange;
        };
        let segments = self.timeline.segments();
        let (seg, first) = (&segments[i], scheme.numbers[i]);
        let years = i64::from(seg.last.0 - seg.first.0);
        let position = n - first;
        if position <= years {
            return Numbered::Year(self.timeline.year_in(i, Key(seg.first.0 + position as i32)));
        }
        match segments.get(i + 1) {
            Some(next) => Numbered::Skipped {
                next: next.first,
                before: first + years + i64::from(scheme.offset),
                after: scheme.numbers[i + 1] + i64::from(scheme.offset),
            },
            None => Numbered::OutOfRange,
        }
    }

    /// Builds a period whose coordinates are known to exist.
    fn make(&self, level: Level, fy: &FiscalYear<'_>, index: Option<u32>) -> Period {
        let i = index.unwrap_or(1);
        let extra = fy.extra_period();
        let ((start, end), has_extra_week) = match level {
            Level::Year => ((fy.start, fy.end), extra.is_some()),
            Level::Quarter => {
                let (first, last) = fy.quarter_periods(i);
                (
                    (fy.period_bounds(first).0, fy.period_bounds(last).1),
                    extra.is_some_and(|p| (first..=last).contains(&p)),
                )
            }
            Level::Period => (fy.period_bounds(i), extra == Some(i)),
            Level::Week => (fy.week_bounds(i), fy.extra_week() == Some(i)),
        };
        let days = start.days_until(end) as u32;
        Period {
            level,
            year: self.default_number(fy),
            index: if level == Level::Year { None } else { Some(i) },
            start,
            end_exclusive: end,
            days,
            weeks: fy.is_week_pattern().then_some(days / 7),
            has_extra_week,
        }
    }

    /// Resolves coordinates to a fiscal year and index.
    fn resolve(
        &self,
        level: Level,
        year: i32,
        index: Option<u32>,
    ) -> Result<(FiscalYear<'_>, Option<u32>), Error> {
        let path = String::from;
        let fy = match self.find_number(i64::from(year), self.default_scheme()) {
            Numbered::Year(fy) => fy,
            Numbered::Skipped { before, after, .. } => {
                return Err(Error::new(
                    ErrorCode::PeriodNotFound,
                    path("year"),
                    format!("no fiscal year is numbered {year}: the numbering skips from {before} to {after} at a change of rule"),
                )
                .hint(format!("use fiscal year {before} or {after}")));
            }
            Numbered::OutOfRange => {
                return Err(out_of_range(path("year"), format!("fiscal year {year}")));
            }
        };
        match (level, index) {
            (Level::Year, None) => Ok((fy, None)),
            (Level::Year, Some(_)) => Err(Error::new(
                ErrorCode::InvalidArgument,
                path("index"),
                "a year has no index; omit index or pass null",
            )),
            (_, None) => Err(Error::new(
                ErrorCode::InvalidArgument,
                path("index"),
                format!("index is required for a {}", level.name()),
            )),
            (_, Some(i)) => {
                let n = fy.count(level);
                if (1..=n).contains(&i) {
                    Ok((fy, Some(i)))
                } else {
                    Err(Error::new(
                        ErrorCode::PeriodNotFound,
                        path("index"),
                        format!(
                            "fiscal year {year} has no {} {i}; it has {} {}s",
                            level.name(),
                            n,
                            level.name()
                        ),
                    )
                    .hint(format!("use an index from 1 to {n}")))
                }
            }
        }
    }

    fn reresolve(&self, p: &Period) -> Result<(FiscalYear<'_>, Option<u32>), Error> {
        self.resolve(p.level, p.year, p.index).map_err(|mut e| {
            e.path = String::from("period");
            e
        })
    }

    /// Where `date` falls at every level.
    ///
    /// # Errors
    ///
    /// `OUT_OF_RANGE` at `date` when the fiscal year containing `date` is
    /// outside the supported range.
    pub fn locate(&self, date: Date) -> Result<Location, Error> {
        self.locate_at(date, "date")
    }

    pub(crate) fn locate_at(&self, date: Date, path: &str) -> Result<Location, Error> {
        self.locate_in(date, path).map(|(_, loc)| loc)
    }

    /// Like [`Self::locate_at`], also returning the fiscal year.
    pub(crate) fn locate_in(
        &self,
        date: Date,
        path: &str,
    ) -> Result<(FiscalYear<'_>, Location), Error> {
        let fy = self
            .timeline
            .year_at(date)
            .ok_or_else(|| out_of_range(path, format!("the fiscal year containing {date}")))?;
        let year = self.make(Level::Year, &fy, None);
        let pi = fy.period_of(date);
        let quarter = self.make(Level::Quarter, &fy, Some(fy.quarter_of_period(pi)));
        let period = self.make(Level::Period, &fy, Some(pi));
        let week = self.make(Level::Week, &fy, Some(fy.week_of(date)));
        let day_of = |p: &Period| p.start.days_until(date) as u32 + 1;
        let location = Location {
            date,
            day_of_year: day_of(&year),
            day_of_quarter: day_of(&quarter),
            day_of_period: day_of(&period),
            day_of_week: (date.weekday().index() + 7 - self.def.week_start.index()) % 7 + 1,
            year,
            quarter,
            period,
            week,
        };
        Ok((fy, location))
    }

    /// The period with the given coordinates. `index` is `None` for a year.
    ///
    /// # Errors
    ///
    /// `OUT_OF_RANGE` at `year` when the fiscal year is outside the supported
    /// range. `PERIOD_NOT_FOUND` at `year` when no fiscal year has that number
    /// because the numbering skips it at a change of rule. `INVALID_ARGUMENT`
    /// at `index` when a year is given an index or any other level none.
    /// `PERIOD_NOT_FOUND` at `index` when the year has no such quarter, period
    /// or week.
    pub fn period(&self, level: Level, year: i32, index: Option<u32>) -> Result<Period, Error> {
        let (fy, index) = self.resolve(level, year, index)?;
        Ok(self.make(level, &fy, index))
    }

    /// The period `n` steps away at the same level, crossing fiscal years as
    /// needed. A year moves `n` fiscal years along the calendar's sequence of
    /// years, which is not the same as `n` in the year number when the
    /// numbering skips a number at a change of rule.
    ///
    /// # Errors
    ///
    /// `OUT_OF_RANGE`, `INVALID_ARGUMENT` or `PERIOD_NOT_FOUND` at `period` when
    /// its coordinates do not name a period, and `OUT_OF_RANGE` at `n` when the
    /// result lies outside the supported range.
    pub fn shift(&self, period: &Period, n: i64) -> Result<Period, Error> {
        let (fy, index) = self.reresolve(period)?;
        let level = period.level;
        let oor = || out_of_range("n", format!("shifting by {n}"));
        if level == Level::Year {
            let target = i64::from(fy.key.0)
                .checked_add(n)
                .and_then(|k| i32::try_from(k).ok())
                .and_then(|k| self.timeline.year(Key(k)))
                .ok_or_else(oor)?;
            return Ok(self.make(level, &target, None));
        }
        // A step longer than every period of the range must leave it; checking
        // first bounds the walk below by the number of fiscal years.
        if n.unsigned_abs() > self.timeline.capacity(level) {
            return Err(oor());
        }
        let mut fy = fy;
        let mut i = i64::from(index.unwrap_or(1)) + n;
        while i > i64::from(fy.count(level)) {
            i -= i64::from(fy.count(level));
            fy = self.timeline.year(Key(fy.key.0 + 1)).ok_or_else(oor)?;
        }
        while i < 1 {
            fy = self.timeline.year(Key(fy.key.0 - 1)).ok_or_else(oor)?;
            i += i64::from(fy.count(level));
        }
        Ok(self.make(level, &fy, Some(i as u32)))
    }

    /// The enclosing period one level up. A year has no parent.
    ///
    /// A week's parent is its period when the fiscal year is subdivided by a
    /// week pattern, and its year otherwise.
    ///
    /// # Errors
    ///
    /// `OUT_OF_RANGE` or `PERIOD_NOT_FOUND` at `period` when its coordinates do
    /// not name a period, and `INVALID_ARGUMENT` at `period` for a year.
    pub fn parent(&self, period: &Period) -> Result<Period, Error> {
        let (fy, index) = self.reresolve(period)?;
        let i = index.unwrap_or(1);
        match period.level {
            Level::Year => Err(Error::new(
                ErrorCode::InvalidArgument,
                "period",
                "a year has no parent",
            )),
            Level::Quarter => Ok(self.make(Level::Year, &fy, None)),
            Level::Period => Ok(self.make(Level::Quarter, &fy, Some(fy.quarter_of_period(i)))),
            Level::Week if fy.is_week_pattern() => {
                let k = fy.period_of(fy.week_bounds(i).0);
                Ok(self.make(Level::Period, &fy, Some(k)))
            }
            Level::Week => Ok(self.make(Level::Year, &fy, None)),
        }
    }

    /// The periods at `level` inside `period`, in order.
    ///
    /// # Errors
    ///
    /// `OUT_OF_RANGE`, `INVALID_ARGUMENT` or `PERIOD_NOT_FOUND` at `period` when
    /// its coordinates do not name a period. `LEVEL_NOT_NESTED` at `level` when
    /// `level` is not below the period's level, or when asking for the weeks of
    /// a quarter or period in a fiscal year subdivided into calendar months.
    pub fn children(&self, period: &Period, level: Level) -> Result<Vec<Period>, Error> {
        let (fy, index) = self.reresolve(period)?;
        if level <= period.level {
            return Err(Error::new(
                ErrorCode::LevelNotNested,
                "level",
                format!("{} is not below {}", level.name(), period.level.name()),
            )
            .hint("ask for a level below the period's level"));
        }
        if !fy.is_week_pattern() && level == Level::Week && period.level != Level::Year {
            let year = match fy.source {
                Source::Transition(_) => "a calendar_months transition",
                Source::TopLevel | Source::History(_) => "a fixed_date year",
            };
            return Err(Error::new(
                ErrorCode::LevelNotNested,
                "level",
                format!(
                    "weeks cross month boundaries in {year}, so they do not nest inside a {}",
                    period.level.name()
                ),
            )
            .hint("ask for the weeks of the year instead"));
        }
        let i = index.unwrap_or(1);
        let range = match (period.level, level) {
            (Level::Year, l) => 1..=fy.count(l),
            (Level::Quarter, Level::Period) => {
                let (a, b) = fy.quarter_periods(i);
                a..=b
            }
            // Weeks of a quarter or period, in a year of whole weeks from its first day.
            (Level::Quarter | Level::Period, _) => {
                let (s, e) = if period.level == Level::Quarter {
                    let (a, b) = fy.quarter_periods(i);
                    (fy.period_bounds(a).0, fy.period_bounds(b).1)
                } else {
                    fy.period_bounds(i)
                };
                let first = fy.start.days_until(s) as u32 / 7 + 1;
                let last = fy.start.days_until(e) as u32 / 7;
                first..=last
            }
            // Rejected above: nothing is below a week.
            (Level::Week, _) => return Ok(Vec::new()),
        };
        Ok(range.map(|j| self.make(level, &fy, Some(j))).collect())
    }

    /// The year periods whose year numbers lie in `from` to `to` inclusive, in
    /// order, for at most 10,000 numbers. The result is shorter than
    /// `to - from + 1` only when the numbering skips numbers at a change of rule.
    ///
    /// # Errors
    ///
    /// `INVALID_ARGUMENT` at `to` when `from` is after `to` or more than 10,000
    /// years are requested. `OUT_OF_RANGE` at `from` or `to` when that number
    /// is below the first or above the last supported year number.
    pub fn years(&self, from: i32, to: i32) -> Result<Vec<Period>, Error> {
        if from > to {
            return Err(Error::new(
                ErrorCode::InvalidArgument,
                "to",
                format!("to ({to}) is before from ({from})"),
            ));
        }
        if i64::from(to) - i64::from(from) + 1 > MAX_YEARS {
            return Err(Error::new(
                ErrorCode::InvalidArgument,
                "to",
                "at most 10,000 years can be requested at once",
            ));
        }
        let scheme = self.default_scheme();
        let first = match self.find_number(i64::from(from), scheme) {
            Numbered::Year(fy) => fy.key,
            Numbered::Skipped { next, .. } => next,
            Numbered::OutOfRange => {
                return Err(out_of_range("from", format!("fiscal year {from}")));
            }
        };
        if let Numbered::OutOfRange = self.find_number(i64::from(to), scheme) {
            return Err(out_of_range("to", format!("fiscal year {to}")));
        }
        let mut out = Vec::new();
        let mut key = first;
        while let Some(fy) = self.timeline.year(key) {
            if self.number(&fy, scheme) > i64::from(to) {
                break;
            }
            out.push(self.make(Level::Year, &fy, None));
            key = Key(key.0 + 1);
        }
        Ok(out)
    }

    fn scheme(&self, name: Option<&str>) -> Result<&Scheme, Error> {
        match name {
            None => Ok(self.default_scheme()),
            Some(n) => self
                .def
                .schemes
                .iter()
                .find(|s| s.name == n)
                .ok_or_else(|| {
                    let names: Vec<&str> =
                        self.def.schemes.iter().map(|s| s.name.as_str()).collect();
                    let names = names.join(", ");
                    Error::new(
                        ErrorCode::SchemeNotFound,
                        "scheme",
                        format!("no scheme is named \"{n}\""),
                    )
                    .hint(format!("schemes: {names}"))
                }),
        }
    }

    /// The label of `period` under `scheme`, or the default scheme when `None`.
    ///
    /// # Errors
    ///
    /// `OUT_OF_RANGE`, `INVALID_ARGUMENT` or `PERIOD_NOT_FOUND` at `period` when
    /// its coordinates do not name a period. `SCHEME_NOT_FOUND` at `scheme`.
    /// `TEMPLATE_MISSING` at `period` when the scheme has no template for the
    /// level. `OUT_OF_RANGE` at `period` when `{yyyy}` would need a year number
    /// outside 1 to 9999.
    pub fn format(&self, period: &Period, scheme: Option<&str>) -> Result<String, Error> {
        self.format_labeled(period, scheme).map(|(label, _)| label)
    }

    /// Like [`Self::format`], also returning the name of the scheme it used.
    pub(crate) fn format_labeled(
        &self,
        period: &Period,
        scheme: Option<&str>,
    ) -> Result<(String, &str), Error> {
        let (fy, index) = self.reresolve(period)?;
        let s = self.scheme(scheme)?;
        let Some(t) = &s.templates[period.level.index()] else {
            return Err(Error::new(
                ErrorCode::TemplateMissing,
                "period",
                format!(
                    "scheme \"{}\" has no {} template",
                    s.name,
                    period.level.name()
                ),
            ));
        };
        let number = self.number(&fy, s);
        let label = t.render(number, fy.last_day_year(), index).ok_or_else(|| {
            out_of_range(
                "period",
                format!("year number {number} under scheme \"{}\"", s.name),
            )
            .hint("{yyyy} formats year numbers from 1 to 9999")
        })?;
        Ok((label, s.name.as_str()))
    }

    pub(crate) fn label_for(
        &self,
        scheme: &Scheme,
        level: Level,
        loc: &Location,
        fy: &FiscalYear<'_>,
    ) -> Option<String> {
        let t = scheme.templates[level.index()].as_ref()?;
        let p = match level {
            Level::Year => &loc.year,
            Level::Quarter => &loc.quarter,
            Level::Period => &loc.period,
            Level::Week => &loc.week,
        };
        t.render(self.number(fy, scheme), fy.last_day_year(), p.index)
    }

    /// The period a label denotes. Reports every match rather than choosing one.
    ///
    /// # Errors
    ///
    /// `LABEL_NOT_RECOGNISED` at `label` when no template matches, or no match
    /// is consistent with a fiscal year and at least one names a year number
    /// from the first to the last supported number (a number skipped at a
    /// change of rule is one of these). `AMBIGUOUS_LABEL` at `label` when more
    /// than one period matches, with every candidate in
    /// [`Details::Candidates`]. `OUT_OF_RANGE` at `label` when every match
    /// names a year number below the first or above the last supported number.
    pub fn parse(&self, label: &str) -> Result<Period, Error> {
        let mut candidates: Vec<Candidate> = Vec::new();
        // Whether some match names a number inside, or outside, the supported numbers.
        let (mut in_range, mut out_of_range_match) = (false, false);
        for scheme in &self.def.schemes {
            for level in Level::ALL {
                let Some(t) = &scheme.templates[level.index()] else {
                    continue;
                };
                for c in t.captures(label) {
                    // Validation gives every template {yyyy} or {yy}, and a
                    // century pivot to a scheme that has {yy} without {yyyy}.
                    let Some(number) = label_number(&c, scheme) else {
                        continue;
                    };
                    let fy = match self.find_number(number, scheme) {
                        Numbered::Year(fy) => fy,
                        Numbered::Skipped { .. } => {
                            in_range = true;
                            continue;
                        }
                        Numbered::OutOfRange => {
                            out_of_range_match = true;
                            continue;
                        }
                    };
                    in_range = true;
                    if !two_digit_years_agree(&c, number, &fy) {
                        continue;
                    }
                    let index = if level == Level::Year {
                        None
                    } else {
                        match c.index {
                            Some(i) if (1..=fy.count(level)).contains(&i) => Some(i),
                            _ => continue,
                        }
                    };
                    candidates.push(Candidate {
                        period: self.make(level, &fy, index),
                        scheme: scheme.name.clone(),
                        level,
                    });
                }
            }
        }
        candidates.sort_by(|a, b| {
            a.period
                .start
                .cmp(&b.period.start)
                .then_with(|| a.scheme.as_bytes().cmp(b.scheme.as_bytes()))
                .then_with(|| a.level.cmp(&b.level))
        });
        let mut merged: Vec<Candidate> = Vec::new();
        for c in candidates {
            if !merged
                .iter()
                .any(|m| m.level == c.level && m.period.start == c.period.start)
            {
                merged.push(c);
            }
        }
        match merged.len() {
            1 => Ok(merged.remove(0).period),
            0 if out_of_range_match && !in_range => Err(out_of_range(
                "label",
                format!("the fiscal year of \"{label}\""),
            )),
            0 => Err(Error::new(
                ErrorCode::LabelNotRecognised,
                "label",
                format!("no template matches \"{label}\""),
            )
            .hint("labels are matched exactly and case-sensitively against every template")),
            n => {
                let mut e = Error::new(
                    ErrorCode::AmbiguousLabel,
                    "label",
                    format!("\"{label}\" matches {n} periods"),
                )
                .hint("pick one of the candidates, or make the templates distinct");
                e.details = Some(Details::Candidates(merged));
                Err(e)
            }
        }
    }

    /// The `label_<scheme>_<level>` columns of the date dimension, in order.
    pub fn label_columns(&self) -> Vec<String> {
        self.label_slots()
            .map(|(s, l)| format!("label_{}_{}", s.name, l.name()))
            .collect()
    }

    pub(crate) fn label_slots(&self) -> impl Iterator<Item = (&Scheme, Level)> {
        self.def.schemes.iter().flat_map(|s| {
            Level::ALL
                .into_iter()
                .filter(move |l| s.templates[l.index()].is_some())
                .map(move |l| (s, l))
        })
    }

    /// Checks a dimension range and returns its length in days.
    pub(crate) fn dimension_span(&self, start: Date, end_exclusive: Date) -> Result<i32, Error> {
        let days = start.days_until(end_exclusive);
        if days < 0 {
            return Err(Error::new(
                ErrorCode::InvalidArgument,
                "end_exclusive",
                format!("end_exclusive ({end_exclusive}) is before start ({start})"),
            ));
        }
        if days > MAX_DIMENSION_DAYS {
            return Err(Error::new(
                ErrorCode::InvalidArgument,
                "end_exclusive",
                format!("the range has {days} days; at most 146,097 (400 years) are allowed"),
            ));
        }
        if days > 0 {
            if self.timeline.year_at(start).is_none() {
                return Err(out_of_range(
                    "start",
                    format!("the fiscal year containing {start}"),
                ));
            }
            let last = end_exclusive.add(-1);
            if self.timeline.year_at(last).is_none() {
                return Err(out_of_range(
                    "end_exclusive",
                    format!("the fiscal year containing {last}"),
                ));
            }
        }
        Ok(days)
    }
}

/// The year number, including the scheme's offset, that a label's captures
/// name: `{yyyy}`, or else `{yy}` resolved through the scheme's century pivot.
/// `None` when the template has neither or the pivot is missing, which
/// validation rules out.
fn label_number(c: &Captures, scheme: &Scheme) -> Option<i64> {
    match (c.yyyy, c.yy, scheme.century_pivot) {
        (Some(yyyy), _, _) => Some(i64::from(yyyy)),
        (None, Some(yy), Some(pivot)) => {
            let pivot = i64::from(pivot);
            Some(pivot - (pivot - i64::from(yy)).rem_euclid(100))
        }
        _ => None,
    }
}

/// Whether `{yy}`, `{yy_next}` and `{yy_end}`, where the label has them, are
/// the last two digits of `number`, of `number + 1` and of the calendar year
/// of `fy`'s last day.
fn two_digit_years_agree(c: &Captures, number: i64, fy: &FiscalYear<'_>) -> bool {
    let agrees =
        |two: Option<u32>, year: i64| two.is_none_or(|t| i64::from(t) == year.rem_euclid(100));
    agrees(c.yy, number)
        && agrees(c.yy_next, number + 1)
        && agrees(c.yy_end, i64::from(fy.last_day_year()))
}

pub(crate) fn out_of_range(path: impl Into<String>, what: String) -> Error {
    Error::new(
        ErrorCode::OutOfRange,
        path,
        format!("{what} is outside the supported range"),
    )
    .hint("fiscalclock supports fiscal years that lie wholly between 0001-01-02 and 9999-12-30")
}
