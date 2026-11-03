use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::date::{Date, days_in_month};
use crate::definition::{
    AnchorDay, Definition, Edge, FixedDate, Rule, Scheme, ShortMonth, Structure, WeekBased,
    YearNumber,
};
use crate::error::{Candidate, Details, Error, ErrorCode, Errors};

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
    /// The number of weeks in a `week_based` year; `None` in a `fixed_date` year.
    pub weeks: Option<u32>,
    /// For a year, quarter or period: whether it contains the extra week. For a
    /// week: whether it is the extra week. Always false in `fixed_date` years.
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

/// The internal identity of a fiscal year: the calendar year `Y` in the spec's
/// boundary rules. Kept distinct from year numbers, which add scheme offsets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Key(pub i32);

/// A fiscal calendar built from a validated [`Definition`].
#[derive(Clone, Debug)]
pub struct FiscalCalendar {
    pub(crate) def: Definition,
    canonical: String,
    id: String,
    min_key: Key,
    max_key: Key,
}

const MAX_YEARS: i64 = 10_000;
const MAX_DIMENSION_DAYS: i32 = 146_097;

impl FiscalCalendar {
    /// Validates a JSON definition and builds the calendar.
    pub fn from_json(definition: &str) -> Result<FiscalCalendar, Errors> {
        Definition::from_json(definition).map(FiscalCalendar::new)
    }

    /// Builds the calendar from a validated definition.
    pub fn new(def: Definition) -> FiscalCalendar {
        let canonical = def.canonical_json();
        let id = crate::definition::calendar_id_of(&canonical);
        let mut cal = FiscalCalendar {
            def,
            canonical,
            id,
            min_key: Key(0),
            max_key: Key(0),
        };
        let first_day = Date::civil(1, 1, 2);
        let end_limit = Date::civil(9999, 12, 31);
        cal.min_key = (-3..=5)
            .map(Key)
            .find(|&k| cal.year_bounds(k).0 >= first_day)
            .unwrap_or(Key(5));
        cal.max_key = (9995..=10_003)
            .rev()
            .map(Key)
            .find(|&k| cal.year_bounds(k).1 <= end_limit)
            .unwrap_or(Key(9995));
        cal
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

    fn week_boundary(&self, w: &WeekBased, y: i32) -> Date {
        let day = match w.day {
            AnchorDay::Day(d) => d,
            AnchorDay::Last => days_in_month(y, w.month),
        };
        let target = Date::civil(y, w.month, day);
        let qualifying = match w.edge {
            Edge::End => self.def.week_start.plus(-1),
            Edge::Start => self.def.week_start,
        };
        let forward = (qualifying.index() as i32 - target.weekday().index() as i32).rem_euclid(7);
        let back = if forward == 0 { 0 } else { forward - 7 };
        match w.rule {
            Rule::First => target.add(forward),
            Rule::Last => target.add(back),
            Rule::Nearest if forward <= 3 => target.add(forward),
            Rule::Nearest => target.add(back),
        }
    }

    /// The first day of the fiscal month that nominally starts in `(year, month)`.
    fn month_start(f: &FixedDate, year: i32, month: u32) -> Date {
        let len = days_in_month(year, month);
        if f.day <= len {
            Date::civil(year, month, f.day)
        } else {
            match f.short_month {
                Some(ShortMonth::Next) => Date::civil(year, month, len).add(1),
                _ => Date::civil(year, month, len),
            }
        }
    }

    /// Start of fiscal month `k` (1-based; 13 is the next year's first day).
    fn fixed_period_start(f: &FixedDate, key: Key, k: u32) -> Date {
        let m0 = f.month - 1 + (k - 1);
        Self::month_start(f, key.0 + (m0 / 12) as i32, m0 % 12 + 1)
    }

    pub(crate) fn year_bounds(&self, key: Key) -> (Date, Date) {
        match &self.def.structure {
            Structure::FixedDate(f) => (
                Self::fixed_period_start(f, key, 1),
                Self::fixed_period_start(f, key, 13),
            ),
            Structure::WeekBased(w) => match w.edge {
                Edge::End => (
                    self.week_boundary(w, key.0 - 1).add(1),
                    self.week_boundary(w, key.0).add(1),
                ),
                Edge::Start => (
                    self.week_boundary(w, key.0),
                    self.week_boundary(w, key.0 + 1),
                ),
            },
        }
    }

    fn key_of(&self, date: Date) -> Key {
        let y = date.year();
        // A boundary lies within six days of its target, so a late-December
        // target with edge "start" can put a date in the year keyed y - 2, and
        // an early-January target with edge "end" in the year keyed y + 2.
        [y, y - 1, y + 1, y - 2, y + 2]
            .into_iter()
            .map(Key)
            .find(|&k| {
                let (s, e) = self.year_bounds(k);
                s <= date && date < e
            })
            .unwrap_or(Key(y))
    }

    fn in_range(&self, key: Key) -> bool {
        self.min_key <= key && key <= self.max_key
    }

    /// The calendar year that numbers fiscal year `key` under `rule`, before
    /// any offset; `None` when a majority rule meets an even split.
    fn raw_number(&self, key: Key, rule: YearNumber) -> Option<i64> {
        let (start, end) = self.year_bounds(key);
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

    /// Checks that `rule` gives consecutive numbers to consecutive fiscal years
    /// across the supported range, returning the constant shift from key to
    /// number, or each problem found with its first affected years (before offset).
    fn rule_shift(&self, rule: YearNumber) -> Result<i64, Vec<(ErrorCode, Vec<i64>)>> {
        const SHOWN: usize = 5;
        let mut ties = Vec::new();
        let mut clashes: Vec<i64> = Vec::new();
        let mut prev: Option<i64> = None;
        for k in self.min_key.0..=self.max_key.0 {
            let Some(n) = self.raw_number(Key(k), rule) else {
                if ties.len() < SHOWN {
                    ties.push(i64::from(self.year_bounds(Key(k)).0.year()));
                }
                prev = None;
                continue;
            };
            if let Some(p) = prev {
                let affected = if n == p {
                    Some(n)
                } else if n != p + 1 {
                    Some(p + 1)
                } else {
                    None
                };
                if let Some(a) = affected {
                    if clashes.len() < SHOWN && !clashes.contains(&a) {
                        clashes.push(a);
                    }
                }
            }
            prev = Some(n);
        }
        // A tie leaves years unnumbered, so it supersedes any collision report.
        if !ties.is_empty() {
            return Err(alloc::vec![(ErrorCode::MajorityTie, ties)]);
        }
        if !clashes.is_empty() {
            return Err(alloc::vec![(ErrorCode::YearNumberCollision, clashes)]);
        }
        let first = self.min_key;
        Ok(self.raw_number(first, rule).unwrap_or(0) - i64::from(first.0))
    }

    /// The shift of every scheme, or the numbering errors of the schemes that fail.
    pub(crate) fn scheme_shifts(&self) -> Result<Vec<i64>, Vec<Error>> {
        let mut shifts = Vec::new();
        let mut errors = Vec::new();
        for scheme in &self.def.schemes {
            match self.rule_shift(scheme.year_number) {
                Ok(shift) => shifts.push(shift),
                Err(problems) => {
                    let alternative = [YearNumber::Start, YearNumber::End, YearNumber::Majority]
                        .into_iter()
                        .find(|&r| r != scheme.year_number && self.rule_shift(r).is_ok());
                    let hint = match alternative {
                        Some(alt) => format!(
                            "use \"year_number\": \"{}\", which numbers every fiscal year in the range uniquely",
                            year_number_name(alt)
                        ),
                        None => String::from(
                            "no year_number rule numbers this calendar uniquely; table numbering is planned for fiscalclock 0.2",
                        ),
                    };
                    for (code, years) in problems {
                        let (message, years) = if code == ErrorCode::MajorityTie {
                            (
                                String::from(
                                    "year_number \"majority\" cannot number a fiscal year that has exactly half of its days in each of two calendar years",
                                ),
                                years,
                            )
                        } else {
                            (
                                format!(
                                    "year_number \"{}\" gives two fiscal years the same number, or skips a number, within the supported range",
                                    year_number_name(scheme.year_number)
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
            Ok(shifts)
        } else {
            Err(errors)
        }
    }

    pub(crate) fn number(&self, key: Key, scheme: &Scheme) -> i64 {
        i64::from(key.0) + scheme.shift + i64::from(scheme.offset)
    }

    fn default_scheme(&self) -> &Scheme {
        &self.def.schemes[self.def.default_scheme]
    }

    fn default_number(&self, key: Key) -> i32 {
        // Offsets are limited to +-1e9 at validation, so this fits.
        self.number(key, self.default_scheme()) as i32
    }

    fn key_from_number(&self, number: i64, scheme: &Scheme) -> Option<Key> {
        let k = number - scheme.shift - i64::from(scheme.offset);
        let key = Key(i32::try_from(k).ok()?);
        self.in_range(key).then_some(key)
    }

    fn quarters(&self) -> &[u32] {
        match &self.def.structure {
            Structure::FixedDate(f) => &f.quarters,
            Structure::WeekBased(w) => &w.quarters,
        }
    }

    fn n_periods(&self) -> u32 {
        match &self.def.structure {
            Structure::FixedDate(_) => 12,
            Structure::WeekBased(w) => w.periods.len() as u32,
        }
    }

    fn n_quarters(&self) -> u32 {
        self.quarters().len() as u32
    }

    /// First and last period of quarter `q`.
    pub(crate) fn quarter_periods(&self, q: u32) -> (u32, u32) {
        let qs = self.quarters();
        let first = qs[..(q - 1) as usize].iter().sum::<u32>() + 1;
        (first, first + qs[(q - 1) as usize] - 1)
    }

    fn quarter_of_period(&self, k: u32) -> u32 {
        let mut last = 0;
        for (i, n) in self.quarters().iter().enumerate() {
            last += n;
            if k <= last {
                return i as u32 + 1;
            }
        }
        self.n_quarters()
    }

    fn has_extra(&self, key: Key) -> bool {
        let (s, e) = self.year_bounds(key);
        matches!(self.def.structure, Structure::WeekBased(_)) && s.days_until(e) == 371
    }

    /// Weeks before period `k` and weeks in it, in a week-based year.
    fn pattern_weeks(w: &WeekBased, extra: bool, k: u32) -> (u32, u32) {
        let before: u32 = w.periods[..(k - 1) as usize].iter().sum();
        let before = before + u32::from(extra && k > w.extra_period);
        let own = w.periods[(k - 1) as usize] + u32::from(extra && k == w.extra_period);
        (before, own)
    }

    fn period_bounds(&self, key: Key, k: u32) -> (Date, Date) {
        match &self.def.structure {
            Structure::FixedDate(f) => (
                Self::fixed_period_start(f, key, k),
                Self::fixed_period_start(f, key, k + 1),
            ),
            Structure::WeekBased(w) => {
                let start = self.year_bounds(key).0;
                let (before, own) = Self::pattern_weeks(w, self.has_extra(key), k);
                (
                    start.add(7 * before as i32),
                    start.add(7 * (before + own) as i32),
                )
            }
        }
    }

    /// The Monday-or-whatever on or before the year's first day, from which
    /// `contains_start` weeks are counted in a fixed-date year.
    fn week_origin(&self, year_start: Date) -> Date {
        let back = (year_start.weekday().index() as i32 - self.def.week_start.index() as i32)
            .rem_euclid(7);
        year_start.add(-back)
    }

    fn n_weeks(&self, key: Key) -> u32 {
        let (s, e) = self.year_bounds(key);
        match self.def.structure {
            Structure::WeekBased(_) => (s.days_until(e) / 7) as u32,
            Structure::FixedDate(_) => (self.week_origin(s).days_until(e) as u32).div_ceil(7),
        }
    }

    fn week_bounds(&self, key: Key, w: u32) -> (Date, Date) {
        let (s, e) = self.year_bounds(key);
        match self.def.structure {
            Structure::WeekBased(_) => (s.add(7 * (w as i32 - 1)), s.add(7 * w as i32)),
            Structure::FixedDate(_) => {
                let origin = self.week_origin(s);
                (
                    origin.add(7 * (w as i32 - 1)).max(s),
                    origin.add(7 * w as i32).min(e),
                )
            }
        }
    }

    fn extra_week_number(&self, key: Key) -> Option<u32> {
        match &self.def.structure {
            Structure::WeekBased(w) if self.has_extra(key) => {
                let (before, own) = Self::pattern_weeks(w, true, w.extra_period);
                Some(before + own)
            }
            _ => None,
        }
    }

    fn count(&self, level: Level, key: Key) -> u32 {
        match level {
            Level::Year => 1,
            Level::Quarter => self.n_quarters(),
            Level::Period => self.n_periods(),
            Level::Week => self.n_weeks(key),
        }
    }

    /// Builds a period whose coordinates are known to exist.
    fn make(&self, level: Level, key: Key, index: Option<u32>) -> Period {
        let i = index.unwrap_or(1);
        let week_based = matches!(self.def.structure, Structure::WeekBased(_));
        let extra = self.has_extra(key);
        let ((start, end), has_extra_week) = match level {
            Level::Year => (self.year_bounds(key), extra),
            Level::Quarter => {
                let (first, last) = self.quarter_periods(i);
                let contains = match &self.def.structure {
                    Structure::WeekBased(w) => extra && (first..=last).contains(&w.extra_period),
                    Structure::FixedDate(_) => false,
                };
                (
                    (
                        self.period_bounds(key, first).0,
                        self.period_bounds(key, last).1,
                    ),
                    contains,
                )
            }
            Level::Period => {
                let contains = match &self.def.structure {
                    Structure::WeekBased(w) => extra && i == w.extra_period,
                    Structure::FixedDate(_) => false,
                };
                (self.period_bounds(key, i), contains)
            }
            Level::Week => (
                self.week_bounds(key, i),
                self.extra_week_number(key) == Some(i),
            ),
        };
        let days = start.days_until(end) as u32;
        Period {
            level,
            year: self.default_number(key),
            index: if level == Level::Year { None } else { Some(i) },
            start,
            end_exclusive: end,
            days,
            weeks: week_based.then_some(days / 7),
            has_extra_week,
        }
    }

    /// Resolves coordinates to a key and index, with errors under `prefix`.
    fn resolve(
        &self,
        level: Level,
        year: i32,
        index: Option<u32>,
    ) -> Result<(Key, Option<u32>), Error> {
        let path = String::from;
        let key = self
            .key_from_number(i64::from(year), self.default_scheme())
            .ok_or_else(|| out_of_range(path("year"), format!("fiscal year {year}")))?;
        match (level, index) {
            (Level::Year, None) => Ok((key, None)),
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
                let n = self.count(level, key);
                if (1..=n).contains(&i) {
                    Ok((key, Some(i)))
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

    fn reresolve(&self, p: &Period) -> Result<(Key, Option<u32>), Error> {
        self.resolve(p.level, p.year, p.index).map_err(|mut e| {
            e.path = String::from("period");
            e
        })
    }

    /// Where `date` falls at every level.
    pub fn locate(&self, date: Date) -> Result<Location, Error> {
        self.locate_at(date, "date")
    }

    pub(crate) fn locate_at(&self, date: Date, path: &str) -> Result<Location, Error> {
        let key = self.key_of(date);
        if !self.in_range(key) {
            return Err(out_of_range(
                path,
                format!("the fiscal year containing {date}"),
            ));
        }
        let year = self.make(Level::Year, key, None);
        let pi = match &self.def.structure {
            Structure::FixedDate(f) => (1..=12)
                .find(|&k| date < Self::fixed_period_start(f, key, k + 1))
                .unwrap_or(12),
            Structure::WeekBased(w) => {
                let week0 = year.start.days_until(date) as u32 / 7;
                let extra = year.has_extra_week;
                (1..=w.periods.len() as u32)
                    .find(|&k| {
                        let (before, own) = Self::pattern_weeks(w, extra, k);
                        week0 < before + own
                    })
                    .unwrap_or(w.periods.len() as u32)
            }
        };
        let wi = match self.def.structure {
            Structure::WeekBased(_) => year.start.days_until(date) / 7 + 1,
            Structure::FixedDate(_) => self.week_origin(year.start).days_until(date) / 7 + 1,
        } as u32;
        let quarter = self.make(Level::Quarter, key, Some(self.quarter_of_period(pi)));
        let period = self.make(Level::Period, key, Some(pi));
        let week = self.make(Level::Week, key, Some(wi));
        let day_of = |p: &Period| p.start.days_until(date) as u32 + 1;
        Ok(Location {
            date,
            day_of_year: day_of(&year),
            day_of_quarter: day_of(&quarter),
            day_of_period: day_of(&period),
            day_of_week: (date.weekday().index() + 7 - self.def.week_start.index()) % 7 + 1,
            year,
            quarter,
            period,
            week,
        })
    }

    /// The period with the given coordinates. `index` is `None` for a year.
    pub fn period(&self, level: Level, year: i32, index: Option<u32>) -> Result<Period, Error> {
        let (key, index) = self.resolve(level, year, index)?;
        Ok(self.make(level, key, index))
    }

    /// The period `n` steps away at the same level, crossing years as needed.
    pub fn shift(&self, period: &Period, n: i64) -> Result<Period, Error> {
        let (key, index) = self.reresolve(period)?;
        let level = period.level;
        let oor = || out_of_range("n", format!("shifting by {n}"));
        match level {
            Level::Year => {
                let k = i64::from(key.0).checked_add(n).ok_or_else(oor)?;
                let key = i32::try_from(k).ok().map(Key).filter(|&k| self.in_range(k));
                Ok(self.make(level, key.ok_or_else(oor)?, None))
            }
            Level::Quarter | Level::Period => {
                let c = i64::from(self.count(level, key));
                let lin = i64::from(key.0)
                    .checked_mul(c)
                    .and_then(|x| x.checked_add(i64::from(index.unwrap_or(1)) - 1))
                    .and_then(|x| x.checked_add(n))
                    .ok_or_else(oor)?;
                let key = i32::try_from(lin.div_euclid(c))
                    .ok()
                    .map(Key)
                    .filter(|&k| self.in_range(k))
                    .ok_or_else(oor)?;
                Ok(self.make(level, key, Some(lin.rem_euclid(c) as u32 + 1)))
            }
            Level::Week => match self.def.structure {
                Structure::WeekBased(_) => {
                    let (start, _) = self.week_bounds(key, index.unwrap_or(1));
                    let target = n
                        .checked_mul(7)
                        .and_then(|d| start.checked_add_days(d))
                        .ok_or_else(oor)?;
                    let loc = self.locate_at(target, "n")?;
                    Ok(loc.week)
                }
                Structure::FixedDate(_) => {
                    let span = i64::from(self.max_key.0 - self.min_key.0 + 1) * 54;
                    if n.unsigned_abs() > span.unsigned_abs() {
                        return Err(oor());
                    }
                    let mut key = key;
                    let mut i = i64::from(index.unwrap_or(1)) + n;
                    while i > i64::from(self.n_weeks(key)) {
                        i -= i64::from(self.n_weeks(key));
                        key = Key(key.0 + 1);
                        if !self.in_range(key) {
                            return Err(oor());
                        }
                    }
                    while i < 1 {
                        key = Key(key.0 - 1);
                        if !self.in_range(key) {
                            return Err(oor());
                        }
                        i += i64::from(self.n_weeks(key));
                    }
                    Ok(self.make(level, key, Some(i as u32)))
                }
            },
        }
    }

    /// The enclosing period one level up. A year has no parent.
    pub fn parent(&self, period: &Period) -> Result<Period, Error> {
        let (key, index) = self.reresolve(period)?;
        let i = index.unwrap_or(1);
        match period.level {
            Level::Year => Err(Error::new(
                ErrorCode::InvalidArgument,
                "period",
                "a year has no parent",
            )),
            Level::Quarter => Ok(self.make(Level::Year, key, None)),
            Level::Period => Ok(self.make(Level::Quarter, key, Some(self.quarter_of_period(i)))),
            Level::Week => match self.def.structure {
                Structure::WeekBased(_) => {
                    let start = self.week_bounds(key, i).0;
                    Ok(self.locate_at(start, "period")?.period)
                }
                Structure::FixedDate(_) => Ok(self.make(Level::Year, key, None)),
            },
        }
    }

    /// The periods at `level` inside `period`, in order.
    pub fn children(&self, period: &Period, level: Level) -> Result<Vec<Period>, Error> {
        let (key, index) = self.reresolve(period)?;
        let fixed = matches!(self.def.structure, Structure::FixedDate(_));
        if level <= period.level {
            return Err(Error::new(
                ErrorCode::LevelNotNested,
                "level",
                format!("{} is not below {}", level.name(), period.level.name()),
            )
            .hint("ask for a level below the period's level"));
        }
        if fixed && level == Level::Week && period.level != Level::Year {
            return Err(Error::new(
                ErrorCode::LevelNotNested,
                "level",
                format!(
                    "weeks cross month boundaries in a fixed_date year, so they do not nest inside a {}",
                    period.level.name()
                ),
            )
            .hint("ask for the weeks of the year instead"));
        }
        let i = index.unwrap_or(1);
        let range = match (period.level, level) {
            (Level::Year, l) => 1..=self.count(l, key),
            (Level::Quarter, Level::Period) => {
                let (a, b) = self.quarter_periods(i);
                a..=b
            }
            (Level::Quarter | Level::Period, _) => {
                let (s, e) = if period.level == Level::Quarter {
                    let (a, b) = self.quarter_periods(i);
                    (self.period_bounds(key, a).0, self.period_bounds(key, b).1)
                } else {
                    self.period_bounds(key, i)
                };
                let ys = self.year_bounds(key).0;
                let first = ys.days_until(s) as u32 / 7 + 1;
                let last = ys.days_until(e) as u32 / 7;
                first..=last
            }
            // Rejected above: nothing is below a week.
            (Level::Week, _) => return Ok(Vec::new()),
        };
        Ok(range.map(|j| self.make(level, key, Some(j))).collect())
    }

    /// The year periods with year numbers `from` to `to` inclusive, at most 10,000.
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
        let first = self
            .key_from_number(i64::from(from), scheme)
            .ok_or_else(|| out_of_range("from", format!("fiscal year {from}")))?;
        self.key_from_number(i64::from(to), scheme)
            .ok_or_else(|| out_of_range("to", format!("fiscal year {to}")))?;
        Ok((0..=(to - from))
            .map(|d| self.make(Level::Year, Key(first.0 + d), None))
            .collect())
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
                    let mut names = String::new();
                    for s in &self.def.schemes {
                        if !names.is_empty() {
                            names.push_str(", ");
                        }
                        names.push_str(&s.name);
                    }
                    Error::new(
                        ErrorCode::SchemeNotFound,
                        "scheme",
                        format!("no scheme is named \"{n}\""),
                    )
                    .hint(format!("schemes: {names}"))
                }),
        }
    }

    /// The name of the scheme `format` uses for `scheme`, checking that it exists.
    pub(crate) fn scheme_name(&self, name: Option<&str>) -> Result<&str, Error> {
        self.scheme(name).map(|s| s.name.as_str())
    }

    /// The label of `period` under `scheme`, or the default scheme when `None`.
    pub fn format(&self, period: &Period, scheme: Option<&str>) -> Result<String, Error> {
        let (key, index) = self.reresolve(period)?;
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
        let number = self.number(key, s);
        t.render(number, index).ok_or_else(|| {
            out_of_range(
                "period",
                format!("year number {number} under scheme \"{}\"", s.name),
            )
            .hint("{yyyy} formats year numbers from 1 to 9999")
        })
    }

    pub(crate) fn label_for(
        &self,
        scheme: &Scheme,
        level: Level,
        loc: &Location,
    ) -> Option<String> {
        let t = scheme.templates[level.index()].as_ref()?;
        let p = match level {
            Level::Year => &loc.year,
            Level::Quarter => &loc.quarter,
            Level::Period => &loc.period,
            Level::Week => &loc.week,
        };
        let key = self.key_of(loc.date);
        t.render(self.number(key, scheme), p.index)
    }

    /// The period a label denotes. Reports every match rather than choosing one.
    pub fn parse(&self, label: &str) -> Result<Period, Error> {
        let mut candidates: Vec<Candidate> = Vec::new();
        let mut out_of_range_year = false;
        for scheme in &self.def.schemes {
            for level in Level::ALL {
                let Some(t) = &scheme.templates[level.index()] else {
                    continue;
                };
                for c in t.captures(label) {
                    let number = if let Some(yyyy) = c.yyyy {
                        let n = i64::from(yyyy);
                        if c.yy.is_some_and(|yy| i64::from(yy) != n % 100)
                            || c.yy_next.is_some_and(|yn| i64::from(yn) != (n + 1) % 100)
                        {
                            continue;
                        }
                        n
                    } else {
                        let Some(pivot) = scheme.century_pivot else {
                            continue;
                        };
                        let pivot = i64::from(pivot);
                        let Some(yy) = c.yy else {
                            continue;
                        };
                        let n = pivot - (pivot - i64::from(yy)).rem_euclid(100);
                        if c.yy_next
                            .is_some_and(|yn| i64::from(yn) != (n + 1).rem_euclid(100))
                        {
                            continue;
                        }
                        n
                    };
                    let Some(key) = self.key_from_number(number, scheme) else {
                        out_of_range_year = true;
                        continue;
                    };
                    let index = if level == Level::Year {
                        None
                    } else {
                        match c.index {
                            Some(i) if (1..=self.count(level, key)).contains(&i) => Some(i),
                            _ => continue,
                        }
                    };
                    candidates.push(Candidate {
                        period: self.make(level, key, index),
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
            0 if out_of_range_year => Err(out_of_range(
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
            if !self.in_range(self.key_of(start)) {
                return Err(out_of_range(
                    "start",
                    format!("the fiscal year containing {start}"),
                ));
            }
            let last = end_exclusive.add(-1);
            if !self.in_range(self.key_of(last)) {
                return Err(out_of_range(
                    "end_exclusive",
                    format!("the fiscal year containing {last}"),
                ));
            }
        }
        Ok(days)
    }
}

pub(crate) fn out_of_range(path: impl Into<String>, what: String) -> Error {
    Error::new(
        ErrorCode::OutOfRange,
        path,
        format!("{what} is outside the supported range"),
    )
    .hint("fiscalclock supports fiscal years that lie wholly between 0001-01-02 and 9999-12-30")
}

fn year_number_name(rule: YearNumber) -> &'static str {
    match rule {
        YearNumber::Start => "start",
        YearNumber::End => "end",
        YearNumber::Majority => "majority",
    }
}
