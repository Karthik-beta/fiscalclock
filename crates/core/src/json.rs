//! The JSON API that every binding wraps.
//!
//! [`validate`], [`call`] and [`FiscalCalendar::call_json`] take and return JSON
//! text exactly as documented in `spec/api-v1.md`: keys in the documented
//! order, no insignificant whitespace, and either the operation's result or
//! `{"errors":[...]}`. [`errors_json`] and [`timestamp_rejected`] build that
//! error form for failures a binding detects itself. Because every language
//! calls these same functions, the same inputs give byte-identical output
//! everywhere.

use alloc::format;
use alloc::string::String;
use core::fmt::Write;

use serde_json::{Map, Value};

use crate::calendar::{FiscalCalendar, Level, Location, Period};
use crate::date::Date;
use crate::error::{Details, Error, ErrorCode};

/// Validates a definition: `{"ok":true,"calendar_id":"…","canonical":"…"}` or errors.
pub fn validate(definition: &str) -> String {
    match FiscalCalendar::from_json(definition) {
        Ok(cal) => validate_ok(&cal),
        Err(errors) => errors_json(errors.as_slice()),
    }
}

/// Validates `definition`, then runs operation `op` with the JSON object `args`.
///
/// Prefer [`FiscalCalendar::call_json`] when making many calls with one
/// definition, so it is validated once.
pub fn call(definition: &str, op: &str, args: &str) -> String {
    match FiscalCalendar::from_json(definition) {
        Ok(cal) => cal.call_json(op, args),
        Err(errors) => errors_json(errors.as_slice()),
    }
}

impl FiscalCalendar {
    /// Runs operation `op` with the JSON object `args` and returns the result
    /// JSON, or `{"errors":[...]}`, exactly as [`call`] does.
    pub fn call_json(&self, op: &str, args: &str) -> String {
        match self.dispatch(op, args) {
            Ok(out) => out,
            Err(e) => errors_json(core::slice::from_ref(&e)),
        }
    }

    fn dispatch(&self, op: &str, args: &str) -> Result<String, Error> {
        let parsed: Value = crate::strict::parse(args).map_err(|e| {
            Error::new(
                ErrorCode::InvalidArgument,
                "",
                format!("the arguments are not valid JSON: {e}"),
            )
        })?;
        let Some(a) = parsed.as_object() else {
            return Err(Error::new(
                ErrorCode::InvalidArgument,
                "",
                "the arguments must be a JSON object",
            ));
        };
        let mut out = String::new();
        match op {
            "validate" => return Ok(validate_ok(self)),
            "locate" => write_location(&mut out, &self.locate(arg_date(a, "date")?)?),
            "period" => {
                let level = arg_level(a, "level")?;
                let p = match arg_year(a, "year")? {
                    Some(year) => self.period(level, year, arg_index(a, "index", level)?)?,
                    None => return Err(year_out_of_range("year")),
                };
                write_period(&mut out, &p);
            }
            "shift" => {
                let p = self.arg_period(a)?;
                let n = arg_n(a)?;
                write_period(&mut out, &self.shift(&p, n)?);
            }
            "parent" => write_period(&mut out, &self.parent(&self.arg_period(a)?)?),
            "children" => {
                let p = self.arg_period(a)?;
                let level = arg_level(a, "level")?;
                write_periods(&mut out, &self.children(&p, level)?);
            }
            "years" => {
                let from = arg_year(a, "from")?.ok_or_else(|| year_out_of_range("from"))?;
                let to = arg_year(a, "to")?.ok_or_else(|| year_out_of_range("to"))?;
                write_periods(&mut out, &self.years(from, to)?);
            }
            "format" => {
                let p = self.arg_period(a)?;
                let scheme = match a.get("scheme") {
                    None | Some(Value::Null) => None,
                    Some(Value::String(s)) => Some(s.as_str()),
                    Some(_) => return Err(wrong_type("scheme", "a string or null")),
                };
                let (label, scheme_name) = self.format_labeled(&p, scheme)?;
                out.push_str("{\"label\":");
                write_str(&mut out, &label);
                out.push_str(",\"scheme\":");
                write_str(&mut out, scheme_name);
                out.push('}');
            }
            "parse" => {
                let label = match required(a, "label")? {
                    Value::String(s) => s,
                    _ => return Err(wrong_type("label", "a string")),
                };
                write_period(&mut out, &self.parse(label)?);
            }
            "dimension" | "dimension_csv" => {
                let start = arg_date(a, "start")?;
                let end = arg_date(a, "end_exclusive")?;
                if op == "dimension" {
                    let rows = self.dimension(start, end)?;
                    self.write_rows_json(&mut out, &rows);
                } else {
                    write_str(&mut out, &self.dimension_csv(start, end)?);
                }
            }
            "explain" => {
                let e = self.explain(arg_date(a, "date")?)?;
                out.push_str("{\"date\":");
                write_date(&mut out, e.date);
                out.push_str(",\"calendar_id\":");
                write_str(&mut out, self.calendar_id());
                out.push_str(",\"steps\":[");
                write_list(&mut out, &e.steps, |out, s| {
                    out.push_str("{\"level\":");
                    write_str(out, s.level.name());
                    out.push_str(",\"rule\":");
                    write_str(out, &s.rule);
                    out.push_str(",\"result\":");
                    write_str(out, &s.result);
                    out.push('}');
                });
                out.push_str("]}");
            }
            _ => {
                return Err(Error::new(
                    ErrorCode::InvalidArgument,
                    "op",
                    format!("\"{op}\" is not an operation"),
                )
                .hint("operations: validate, locate, period, shift, parent, children, years, format, parse, dimension, dimension_csv, explain"));
            }
        }
        Ok(out)
    }

    /// Reads the `period` argument: a Period value or its coordinates. Only
    /// `level`, `year` and `index` are read.
    fn arg_period(&self, a: &Map<String, Value>) -> Result<Period, Error> {
        let Value::Object(p) = required(a, "period")? else {
            return Err(wrong_type("period", "an object"));
        };
        let level = arg_level(p, "level").map_err(at_period)?;
        let year = arg_year(p, "year")
            .map_err(at_period)?
            .ok_or_else(|| year_out_of_range("period"))?;
        let index = arg_index(p, "index", level).map_err(at_period)?;
        // Only the coordinates matter: every operation recomputes the bounds.
        Ok(Period {
            level,
            year,
            index,
            start: Date::MIN,
            end_exclusive: Date::MIN,
            days: 0,
            weeks: None,
            has_extra_week: false,
        })
    }
}

fn at_period(mut e: Error) -> Error {
    e.path = String::from("period");
    e
}

fn required<'a>(a: &'a Map<String, Value>, name: &str) -> Result<&'a Value, Error> {
    a.get(name).ok_or_else(|| {
        Error::new(
            ErrorCode::InvalidArgument,
            name,
            format!("{name} is required"),
        )
    })
}

fn wrong_type(name: &str, expected: &str) -> Error {
    Error::new(
        ErrorCode::InvalidArgument,
        name,
        format!("{name} must be {expected}"),
    )
}

fn year_out_of_range(path: &str) -> Error {
    crate::calendar::out_of_range(path, String::from("that fiscal year"))
}

fn arg_date(a: &Map<String, Value>, name: &str) -> Result<Date, Error> {
    match required(a, name)? {
        Value::String(s) => Date::parse_iso(s).ok_or_else(|| {
            Error::new(
                ErrorCode::InvalidDate,
                name,
                format!("\"{s}\" is not a YYYY-MM-DD civil date in the years 0001-9999"),
            )
            .hint("pass a plain date such as \"2024-02-03\", without a time or zone")
        }),
        _ => Err(wrong_type(name, "a YYYY-MM-DD string")),
    }
}

fn arg_level(a: &Map<String, Value>, name: &str) -> Result<Level, Error> {
    match required(a, name)? {
        Value::String(s) => Level::from_name(s).ok_or_else(|| {
            Error::new(
                ErrorCode::InvalidArgument,
                name,
                format!("\"{s}\" is not a level"),
            )
            .hint("levels: year, quarter, period, week")
        }),
        _ => Err(wrong_type(name, "a level name")),
    }
}

/// A year number; `Ok(None)` for an integer that cannot be a supported year.
fn arg_year(a: &Map<String, Value>, name: &str) -> Result<Option<i32>, Error> {
    let v = required(a, name)?;
    if v.is_i64() || v.is_u64() {
        Ok(v.as_i64().and_then(|n| i32::try_from(n).ok()))
    } else {
        Err(wrong_type(name, "an integer"))
    }
}

fn arg_index(a: &Map<String, Value>, name: &str, level: Level) -> Result<Option<u32>, Error> {
    match a.get(name) {
        None | Some(Value::Null) => Ok(None),
        Some(v) if v.is_i64() || v.is_u64() => match v.as_i64().and_then(|n| u32::try_from(n).ok())
        {
            Some(i) => Ok(Some(i)),
            // Not a valid index for any level. A year takes no index, so the
            // sentinel 0 reaches `resolve`, which rejects it as INVALID_ARGUMENT
            // ("a year has no index") instead of PERIOD_NOT_FOUND.
            None if level == Level::Year => Ok(Some(0)),
            None => Err(Error::new(
                ErrorCode::PeriodNotFound,
                name,
                format!("there is no {} {v}", level.name()),
            )),
        },
        Some(_) => Err(wrong_type(name, "an integer or null")),
    }
}

fn arg_n(a: &Map<String, Value>) -> Result<i64, Error> {
    let v = required(a, "n")?;
    if let Some(n) = v.as_i64() {
        Ok(n)
    } else if v.is_u64() {
        Err(crate::calendar::out_of_range(
            "n",
            format!("shifting by {v}"),
        ))
    } else {
        Err(wrong_type("n", "an integer"))
    }
}

fn validate_ok(cal: &FiscalCalendar) -> String {
    let mut out = String::from("{\"ok\":true,\"calendar_id\":");
    write_str(&mut out, cal.calendar_id());
    out.push_str(",\"canonical\":");
    write_str(&mut out, cal.canonical_json());
    out.push('}');
    out
}

/// Writes `s` as a JSON string with only the escapes the spec requires.
pub(crate) fn write_str(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

pub(crate) fn write_list<T>(
    out: &mut String,
    items: impl IntoIterator<Item = T>,
    mut each: impl FnMut(&mut String, T),
) {
    for (i, item) in items.into_iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        each(out, item);
    }
}

pub(crate) fn write_date(out: &mut String, d: Date) {
    let _ = write!(out, "\"{d}\"");
}

fn write_period(out: &mut String, p: &Period) {
    out.push_str("{\"level\":");
    write_str(out, p.level.name());
    let _ = write!(out, ",\"year\":{},\"index\":", p.year);
    match p.index {
        Some(i) => {
            let _ = write!(out, "{i}");
        }
        None => out.push_str("null"),
    }
    out.push_str(",\"start\":");
    write_date(out, p.start);
    out.push_str(",\"end_exclusive\":");
    write_date(out, p.end_exclusive);
    out.push_str(",\"last_day\":");
    write_date(out, p.last_day());
    let _ = write!(out, ",\"days\":{},\"weeks\":", p.days);
    match p.weeks {
        Some(w) => {
            let _ = write!(out, "{w}");
        }
        None => out.push_str("null"),
    }
    out.push_str(if p.has_extra_week {
        ",\"has_extra_week\":true}"
    } else {
        ",\"has_extra_week\":false}"
    });
}

fn write_periods(out: &mut String, ps: &[Period]) {
    out.push('[');
    write_list(out, ps, write_period);
    out.push(']');
}

fn write_location(out: &mut String, l: &Location) {
    out.push_str("{\"date\":");
    write_date(out, l.date);
    for (name, p) in [
        ("year", &l.year),
        ("quarter", &l.quarter),
        ("period", &l.period),
        ("week", &l.week),
    ] {
        let _ = write!(out, ",\"{name}\":");
        write_period(out, p);
    }
    let _ = write!(
        out,
        ",\"day_of_year\":{},\"day_of_quarter\":{},\"day_of_period\":{},\"day_of_week\":{}}}",
        l.day_of_year, l.day_of_quarter, l.day_of_period, l.day_of_week
    );
}

fn write_error(out: &mut String, e: &Error) {
    out.push_str("{\"code\":");
    write_str(out, e.code.as_str());
    out.push_str(",\"path\":");
    write_str(out, &e.path);
    out.push_str(",\"message\":");
    write_str(out, &e.message);
    out.push_str(",\"hint\":");
    match &e.hint {
        Some(h) => write_str(out, h),
        None => out.push_str("null"),
    }
    out.push_str(",\"details\":");
    match &e.details {
        None => out.push_str("null"),
        Some(Details::Years(years)) => {
            out.push_str("{\"years\":[");
            write_list(out, years, |out, y| {
                let _ = write!(out, "{y}");
            });
            out.push_str("]}");
        }
        Some(Details::Candidates(candidates)) => {
            out.push_str("{\"candidates\":[");
            write_list(out, candidates, |out, c| {
                out.push_str("{\"period\":");
                write_period(out, &c.period);
                out.push_str(",\"scheme\":");
                write_str(out, &c.scheme);
                out.push_str(",\"level\":");
                write_str(out, c.level.name());
                out.push('}');
            });
            out.push_str("]}");
        }
    }
    out.push('}');
}

/// `{"errors":[...]}` for a list of errors.
pub fn errors_json(errors: &[Error]) -> String {
    let mut out = String::from("{\"errors\":[");
    write_list(&mut out, errors, write_error);
    out.push_str("]}");
    out
}

/// The `TIMESTAMP_REJECTED` error for argument `path`, for bindings to return
/// when a caller passes a timestamp (such as a Python `datetime` or a
/// JavaScript `Date`) where a civil date is expected. Built here so that every
/// binding returns the same bytes.
pub fn timestamp_rejected(path: &str) -> String {
    errors_json(&[Error::new(
        ErrorCode::TimestampRejected,
        path,
        "a timestamp was passed where a civil date (year, month, day) is expected",
    )
    .hint("convert the timestamp to a date in the time zone you mean, then pass that date")])
}
