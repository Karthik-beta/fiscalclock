use core::fmt;
use core::str::FromStr;

/// A proleptic Gregorian civil date in the years 0001 to 9999.
///
/// Stored as a day number, so comparison, ordering and differences are cheap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(i32);

/// A day of the week.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Weekday {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

/// The error returned when a string is not a strict `YYYY-MM-DD` date in 0001–9999.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseDateError;

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("not a YYYY-MM-DD civil date in the years 0001-9999")
    }
}

impl core::error::Error for ParseDateError {}

const WEEKDAYS: [Weekday; 7] = [
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
    Weekday::Saturday,
    Weekday::Sunday,
];

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

impl Weekday {
    /// Days since Monday: 0 for Monday up to 6 for Sunday.
    pub const fn index(self) -> u32 {
        self as u32
    }

    pub(crate) fn from_index(i: u32) -> Weekday {
        WEEKDAYS[(i % 7) as usize]
    }

    /// The lower-case English name used in definitions, such as `"monday"`.
    pub const fn name(self) -> &'static str {
        match self {
            Weekday::Monday => "monday",
            Weekday::Tuesday => "tuesday",
            Weekday::Wednesday => "wednesday",
            Weekday::Thursday => "thursday",
            Weekday::Friday => "friday",
            Weekday::Saturday => "saturday",
            Weekday::Sunday => "sunday",
        }
    }

    pub(crate) fn title(self) -> &'static str {
        match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        }
    }

    pub(crate) fn from_name(s: &str) -> Option<Weekday> {
        WEEKDAYS.iter().copied().find(|w| w.name() == s)
    }

    /// The weekday `n` days later.
    pub(crate) fn plus(self, n: i32) -> Weekday {
        Weekday::from_index((self.index() as i32 + n).rem_euclid(7) as u32)
    }
}

pub(crate) fn month_name(month: u32) -> &'static str {
    MONTH_NAMES[(month - 1) as usize]
}

pub(crate) const fn is_leap(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub(crate) const fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 if is_leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

// Howard Hinnant's days_from_civil; day 0 is 1970-01-01. Valid for any year
// whose day number fits i32, which internal boundary arithmetic relies on for
// years 0 and 10000.
const fn days_from_civil(year: i32, month: u32, day: u32) -> i32 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe as i32 - 719_468
}

const fn civil_from_days(days: i32) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe as i32 + era * 400 + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

impl Date {
    /// The earliest representable date, 0001-01-01.
    pub const MIN: Date = Date(days_from_civil(1, 1, 1));
    /// The latest representable date, 9999-12-31.
    pub const MAX: Date = Date(days_from_civil(9999, 12, 31));

    /// Builds a date from a year, month and day, or `None` if it does not exist
    /// or lies outside 0001-01-01 to 9999-12-31.
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Date> {
        if !(1..=9999).contains(&year)
            || !(1..=12).contains(&month)
            || day == 0
            || day > days_in_month(year, month)
        {
            return None;
        }
        Some(Date(days_from_civil(year, month, day)))
    }

    /// Parses exactly `YYYY-MM-DD` with a year from 0001 to 9999. Anything else,
    /// including a time or zone suffix, is rejected.
    pub fn parse_iso(s: &str) -> Option<Date> {
        let b = s.as_bytes();
        if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
            return None;
        }
        let num = |range: core::ops::Range<usize>| -> Option<u32> {
            b[range].iter().try_fold(0u32, |acc, &c| {
                c.is_ascii_digit().then(|| acc * 10 + u32::from(c - b'0'))
            })
        };
        Date::from_ymd(num(0..4)? as i32, num(5..7)?, num(8..10)?)
    }

    /// Internal constructor for any year, used where boundary arithmetic needs
    /// years just outside the public range. `month` and `day` must be valid.
    pub(crate) const fn civil(year: i32, month: u32, day: u32) -> Date {
        Date(days_from_civil(year, month, day))
    }

    /// The year, month and day.
    pub const fn ymd(self) -> (i32, u32, u32) {
        civil_from_days(self.0)
    }

    /// The calendar year.
    pub const fn year(self) -> i32 {
        self.ymd().0
    }

    /// The month, 1 to 12.
    pub const fn month(self) -> u32 {
        self.ymd().1
    }

    /// The day of the month, 1 to 31.
    pub const fn day(self) -> u32 {
        self.ymd().2
    }

    /// The day of the week.
    pub fn weekday(self) -> Weekday {
        // 1970-01-01 was a Thursday.
        Weekday::from_index((self.0 + 3).rem_euclid(7) as u32)
    }

    /// The date `days` days later (earlier when negative), or `None` outside
    /// the representable range.
    pub fn checked_add_days(self, days: i64) -> Option<Date> {
        let d = i64::from(self.0).checked_add(days)?;
        let date = Date(i32::try_from(d).ok()?);
        (Date::MIN..=Date::MAX).contains(&date).then_some(date)
    }

    /// The number of days from `self` to `later`; negative when `later` is earlier.
    pub const fn days_until(self, later: Date) -> i32 {
        later.0 - self.0
    }

    pub(crate) const fn add(self, days: i32) -> Date {
        Date(self.0 + days)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d) = self.ymd();
        write!(f, "{y:04}-{m:02}-{d:02}")
    }
}

impl FromStr for Date {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Date, ParseDateError> {
        Date::parse_iso(s).ok_or(ParseDateError)
    }
}
