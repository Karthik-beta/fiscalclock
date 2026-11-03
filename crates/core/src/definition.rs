use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::calendar::{FiscalCalendar, Level};
use crate::date::Weekday;
use crate::error::{Error, ErrorCode, Errors};
use crate::json::{write_list, write_str};
use crate::label::Template;

/// Declares a fieldless enum for a definition field that takes one of a fixed
/// set of strings. Each variant is listed once with its string, so the parser,
/// the canonical writer and the error hints cannot disagree.
macro_rules! named_enum {
    ($name:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(crate) enum $name {
            $($variant),+
        }

        impl $name {
            /// Every variant, in the order error hints list them.
            pub(crate) const ALL: &'static [$name] = &[$($name::$variant),+];

            /// The string a definition uses for this variant.
            pub(crate) const fn name(self) -> &'static str {
                match self {
                    $($name::$variant => $text),+
                }
            }
        }
    };
}

named_enum!(ShortMonth { Clamp => "clamp", Next => "next" });
named_enum!(Edge { End => "end", Start => "start" });
named_enum!(Rule { Last => "last", First => "first", Nearest => "nearest" });
named_enum!(YearNumber { Start => "start", End => "end", Majority => "majority" });
named_enum!(YearType { FixedDate => "fixed_date", WeekBased => "week_based" });
named_enum!(SubdivisionType { CalendarMonths => "calendar_months", WeekPattern => "week_pattern" });
named_enum!(Numbering { ContainsStart => "contains_start" });

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AnchorDay {
    Day(u32),
    Last,
}

/// A `fixed_date` year with its `calendar_months` subdivision; the only pairing
/// 0.1.0 allows, so `month_start_day` is `day` and is not stored twice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FixedDate {
    pub month: u32,
    pub day: u32,
    /// `Some` exactly when `day` is 29 or more.
    pub short_month: Option<ShortMonth>,
    pub quarters: Vec<u32>,
}

/// A `week_based` year with its `week_pattern` subdivision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WeekBased {
    pub edge: Edge,
    pub month: u32,
    pub day: AnchorDay,
    pub rule: Rule,
    pub periods: Vec<u32>,
    pub quarters: Vec<u32>,
    pub extra_period: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Structure {
    FixedDate(FixedDate),
    WeekBased(WeekBased),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Scheme {
    pub name: String,
    pub year_number: YearNumber,
    pub offset: i32,
    pub century_pivot: Option<i32>,
    /// Indexed by [`Level::index`].
    pub templates: [Option<Template>; 4],
    /// Year number minus fiscal year key, before `offset`. Constant because
    /// validation rejects schemes whose numbers are not consecutive.
    pub shift: i64,
}

/// A parsed and validated calendar definition.
///
/// The only way to obtain one is [`Definition::from_json`], so every value of
/// this type satisfies the whole of `spec/definition-v1.md`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definition {
    pub(crate) structure: Structure,
    pub(crate) week_start: Weekday,
    /// Sorted by name.
    pub(crate) schemes: Vec<Scheme>,
    pub(crate) default_scheme: usize,
}

const OFFSET_LIMIT: i64 = 1_000_000_000;

impl Definition {
    /// Parses and validates a definition, reporting every error found.
    ///
    /// # Errors
    ///
    /// Returns every validation error, sorted by path and then code. Any code
    /// the error catalogue lists as raised by `validate` can appear, from
    /// `INVALID_JSON` to `MAJORITY_TIE`.
    pub fn from_json(text: &str) -> Result<Definition, Errors> {
        FiscalCalendar::numbered(Definition::parse(text)?).map(|cal| cal.def)
    }

    /// Everything `from_json` checks except year numbering, which needs the
    /// year boundaries of a calendar. Every scheme's `shift` is still 0.
    pub(crate) fn parse(text: &str) -> Result<Definition, Errors> {
        let value: Value = match crate::strict::parse(text) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::new(
                    ErrorCode::InvalidJson,
                    "",
                    format!("the definition is not valid JSON: {e}"),
                )
                .into());
            }
        };
        let Some(top) = value.as_object() else {
            return Err(Error::new(
                ErrorCode::InvalidJson,
                "",
                "the definition must be a JSON object",
            )
            .into());
        };
        Validator::default().definition(top)
    }

    /// The canonical JSON form defined in `spec/definition-v1.md`.
    pub fn canonical_json(&self) -> String {
        let mut out = String::new();
        write_sorted(&mut out, &self.canonical_value());
        out
    }

    /// The first 16 hexadecimal characters of the SHA-256 of the canonical form.
    pub fn calendar_id(&self) -> String {
        calendar_id_of(&self.canonical_json())
    }

    fn canonical_value(&self) -> Value {
        let (year, subdivision) = match &self.structure {
            Structure::FixedDate(f) => (
                obj([
                    (
                        "short_month",
                        f.short_month.map_or(Value::Null, |s| s.name().into()),
                    ),
                    (
                        "start",
                        obj([("day", f.day.into()), ("month", f.month.into())]),
                    ),
                    ("type", YearType::FixedDate.name().into()),
                ]),
                obj([
                    ("month_start_day", f.day.into()),
                    ("quarters", f.quarters.clone().into()),
                    ("type", SubdivisionType::CalendarMonths.name().into()),
                ]),
            ),
            Structure::WeekBased(w) => (
                obj([
                    (
                        "anchor",
                        obj([
                            (
                                "day",
                                match w.day {
                                    AnchorDay::Day(d) => d.into(),
                                    AnchorDay::Last => "last".into(),
                                },
                            ),
                            ("edge", w.edge.name().into()),
                            ("month", w.month.into()),
                            ("rule", w.rule.name().into()),
                        ]),
                    ),
                    ("type", YearType::WeekBased.name().into()),
                ]),
                obj([
                    ("extra_week", obj([("period", w.extra_period.into())])),
                    ("periods", w.periods.clone().into()),
                    ("quarters", w.quarters.clone().into()),
                    ("type", SubdivisionType::WeekPattern.name().into()),
                ]),
            ),
        };
        let mut schemes = Map::new();
        for s in &self.schemes {
            let mut templates = Map::new();
            for level in Level::ALL {
                if let Some(t) = &s.templates[level.index()] {
                    templates.insert(level.name().to_owned(), t.source.clone().into());
                }
            }
            schemes.insert(
                s.name.clone(),
                obj([
                    (
                        "century_pivot",
                        s.century_pivot.map_or(Value::Null, Value::from),
                    ),
                    ("offset", s.offset.into()),
                    ("templates", Value::Object(templates)),
                    ("year_number", s.year_number.name().into()),
                ]),
            );
        }
        obj([
            (
                "naming",
                obj([
                    (
                        "default",
                        self.schemes[self.default_scheme].name.clone().into(),
                    ),
                    ("schemes", Value::Object(schemes)),
                ]),
            ),
            ("spec_version", 1.into()),
            ("subdivision", subdivision),
            (
                "week",
                obj([
                    ("numbering", Numbering::ContainsStart.name().into()),
                    ("starts_on", self.week_start.name().into()),
                ]),
            ),
            ("year", year),
        ])
    }
}

pub(crate) fn calendar_id_of(canonical: &str) -> String {
    let digest = Sha256::digest(canonical.as_bytes());
    let mut id = String::with_capacity(16);
    for byte in &digest[..8] {
        let _ = write!(id, "{byte:02x}");
    }
    id
}

fn obj<const N: usize>(entries: [(&str, Value); N]) -> Value {
    Value::Object(
        entries
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v))
            .collect(),
    )
}

// Sorts keys explicitly instead of relying on serde_json's map ordering, which a
// `preserve_order` feature enabled anywhere in a dependent's build would change.
fn write_sorted(out: &mut String, value: &Value) {
    match value {
        Value::Object(map) => {
            let sorted: BTreeMap<&[u8], (&String, &Value)> =
                map.iter().map(|(k, v)| (k.as_bytes(), (k, v))).collect();
            out.push('{');
            write_list(out, sorted.values(), |out, &(k, v)| {
                write_str(out, k);
                out.push(':');
                write_sorted(out, v);
            });
            out.push('}');
        }
        Value::Array(items) => {
            out.push('[');
            write_list(out, items, write_sorted);
            out.push(']');
        }
        Value::String(s) => write_str(out, s),
        Value::Number(n) => {
            let _ = write!(out, "{n}");
        }
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Null => out.push_str("null"),
    }
}

fn join(path: &str, key: &str) -> String {
    if path.is_empty() {
        key.to_owned()
    } else {
        format!("{path}.{key}")
    }
}

const PLANNED: &str = "planned for fiscalclock 0.2";

enum YearKind {
    Fixed { day: Option<u32> },
    Week,
}

enum YearSpec {
    Fixed {
        month: u32,
        day: u32,
        short_month: Option<ShortMonth>,
    },
    Week {
        edge: Edge,
        month: u32,
        day: AnchorDay,
        rule: Rule,
    },
}

enum SubKind {
    CalendarMonths { month_start_day: Option<u32> },
    WeekPattern,
}

enum SubSpec {
    CalendarMonths {
        quarters: Vec<u32>,
    },
    WeekPattern {
        periods: Vec<u32>,
        quarters: Vec<u32>,
        extra_period: u32,
    },
}

#[derive(Default)]
struct Validator {
    errors: Vec<Error>,
}

impl Validator {
    fn push(&mut self, e: Error) {
        self.errors.push(e);
    }

    fn definition(mut self, top: &Map<String, Value>) -> Result<Definition, Errors> {
        if let Some(v) = top.get("spec_version") {
            if let Some(n) = v.as_i64().or_else(|| v.as_u64().map(|_| i64::MAX)) {
                if n != 1 {
                    return Err(Error::new(
                        ErrorCode::UnsupportedSpecVersion,
                        "spec_version",
                        format!("spec_version {} is not supported", v),
                    )
                    .hint("this version of fiscalclock reads spec_version 1")
                    .into());
                }
            }
        }
        self.fields(
            top,
            "",
            &[
                "spec_version",
                "$schema",
                "year",
                "week",
                "subdivision",
                "naming",
            ],
            &["exceptions"],
        );
        if let Some(v) = self.required(top, "", "spec_version") {
            self.integer(v, "spec_version");
        }
        if let Some(v) = top.get("$schema") {
            self.string(v, "$schema");
        }
        let year = self.required(top, "", "year").map(|v| self.year(v));
        let (year_kind, year_spec) = year.unwrap_or((None, None));
        let week_start = self.required(top, "", "week").and_then(|v| self.week(v));
        let sub = self
            .required(top, "", "subdivision")
            .map(|v| self.subdivision(v));
        let (sub_kind, sub_spec) = sub.unwrap_or((None, None));
        let naming = self
            .required(top, "", "naming")
            .and_then(|v| self.naming(v));

        match (&year_kind, &sub_kind) {
            (Some(YearKind::Fixed { .. }), Some(SubKind::WeekPattern)) => self.push(
                Error::new(
                    ErrorCode::UnsupportedCombination,
                    "subdivision.type",
                    "a week_pattern subdivision inside a fixed_date year needs subdivision.stub, which is not supported yet",
                )
                .hint("use a week_based year, or calendar_months for a fixed_date year; stubs are planned for fiscalclock 0.2"),
            ),
            (Some(YearKind::Week), Some(SubKind::CalendarMonths { .. })) => self.push(
                Error::new(
                    ErrorCode::UnsupportedCombination,
                    "subdivision.type",
                    "a calendar_months subdivision is valid only inside a fixed_date year",
                )
                .hint("use a week_pattern subdivision for a week_based year"),
            ),
            (
                Some(YearKind::Fixed { day: Some(day) }),
                Some(SubKind::CalendarMonths {
                    month_start_day: Some(msd),
                }),
            ) if day != msd => self.push(
                Error::new(
                    ErrorCode::SubdivisionYearMismatch,
                    "subdivision.month_start_day",
                    format!("month_start_day is {msd} but year.start.day is {day}"),
                )
                .hint(format!("set month_start_day to {day}")),
            ),
            _ => {}
        }

        if !self.errors.is_empty() {
            return Err(Errors::sorted(self.errors));
        }
        let structure = match (year_spec, sub_spec) {
            (
                Some(YearSpec::Fixed {
                    month,
                    day,
                    short_month,
                }),
                Some(SubSpec::CalendarMonths { quarters }),
            ) => Structure::FixedDate(FixedDate {
                month,
                day,
                short_month,
                quarters,
            }),
            (
                Some(YearSpec::Week {
                    edge,
                    month,
                    day,
                    rule,
                }),
                Some(SubSpec::WeekPattern {
                    periods,
                    quarters,
                    extra_period,
                }),
            ) => Structure::WeekBased(WeekBased {
                edge,
                month,
                day,
                rule,
                periods,
                quarters,
                extra_period,
            }),
            _ => return Err(Errors::sorted(self.errors)),
        };
        match (week_start, naming) {
            (Some(week_start), Some((schemes, default_scheme))) => Ok(Definition {
                structure,
                week_start,
                schemes,
                default_scheme,
            }),
            _ => Err(Errors::sorted(self.errors)),
        }
    }

    fn year(&mut self, v: &Value) -> (Option<YearKind>, Option<YearSpec>) {
        let path = "year";
        let Some(m) = self.object(v, path) else {
            return (None, None);
        };
        let Some(t) = self.required(m, path, "type") else {
            return (None, None);
        };
        let Some(kind) = self.choice(t, "year.type", YearType::ALL, YearType::name, &["table"])
        else {
            return (None, None);
        };
        if kind == YearType::FixedDate {
            self.fields(m, path, &["type", "start", "short_month"], &[]);
            let mut month = None;
            let mut day = None;
            if let Some(s) = self.required(m, path, "start") {
                if let Some(sm) = self.object(s, "year.start") {
                    self.fields(sm, "year.start", &["month", "day"], &[]);
                    month = self
                        .required(sm, "year.start", "month")
                        .and_then(|v| self.int_in(v, "year.start.month", 1, 12).map(|n| n as u32));
                    day = self
                        .required(sm, "year.start", "day")
                        .and_then(|v| self.int_in(v, "year.start.day", 1, 31).map(|n| n as u32));
                }
            }
            let short = match m.get("short_month") {
                None | Some(Value::Null) => Some(None),
                Some(v) => self
                    .choice(
                        v,
                        "year.short_month",
                        ShortMonth::ALL,
                        ShortMonth::name,
                        &[],
                    )
                    .map(Some),
            };
            let mut short_ok = short;
            match (day, short) {
                (Some(d), Some(None)) if d >= 29 => {
                    self.push(
                        Error::new(
                            ErrorCode::ShortMonthPolicyMissing,
                            "year.short_month",
                            format!("start.day is {d}, which some months do not have, and short_month is absent"),
                        )
                        .hint("add \"short_month\": \"clamp\" to use the month's last day, or \"next\" to use the first day of the following month"),
                    );
                    short_ok = None;
                }
                (Some(d), Some(Some(_))) if d <= 28 => {
                    self.push(
                        Error::new(
                            ErrorCode::InvalidValue,
                            "year.short_month",
                            format!("short_month never applies because start.day {d} exists in every month"),
                        )
                        .hint("remove short_month"),
                    );
                    short_ok = None;
                }
                _ => {}
            }
            let spec = match (month, day, short_ok) {
                (Some(month), Some(day), Some(short_month)) => Some(YearSpec::Fixed {
                    month,
                    day,
                    short_month,
                }),
                _ => None,
            };
            (Some(YearKind::Fixed { day }), spec)
        } else {
            self.fields(m, path, &["type", "anchor"], &[]);
            let Some(a) = self.required(m, path, "anchor") else {
                return (Some(YearKind::Week), None);
            };
            let ap = "year.anchor";
            let Some(am) = self.object(a, ap) else {
                return (Some(YearKind::Week), None);
            };
            self.fields(am, ap, &["edge", "month", "day", "rule"], &[]);
            let edge = self
                .required(am, ap, "edge")
                .and_then(|v| self.choice(v, "year.anchor.edge", Edge::ALL, Edge::name, &[]));
            let month = self
                .required(am, ap, "month")
                .and_then(|v| self.int_in(v, "year.anchor.month", 1, 12).map(|n| n as u32));
            let day = self
                .required(am, ap, "day")
                .and_then(|v| self.anchor_day(v));
            let rule = self
                .required(am, ap, "rule")
                .and_then(|v| self.choice(v, "year.anchor.rule", Rule::ALL, Rule::name, &[]));
            let spec = match (edge, month, day, rule) {
                (Some(edge), Some(month), Some(day), Some(rule)) => Some(YearSpec::Week {
                    edge,
                    month,
                    day,
                    rule,
                }),
                _ => None,
            };
            (Some(YearKind::Week), spec)
        }
    }

    fn anchor_day(&mut self, v: &Value) -> Option<AnchorDay> {
        let path = "year.anchor.day";
        match v {
            Value::String(s) if s == "last" => Some(AnchorDay::Last),
            Value::String(s) => {
                self.push(
                    Error::new(
                        ErrorCode::InvalidValue,
                        path,
                        format!("\"{s}\" is not a day; expected an integer 1-28 or \"last\""),
                    )
                    .hint("use \"last\" for the month's last day"),
                );
                None
            }
            Value::Number(_) => {
                let n = self.integer(v, path)?;
                if (1..=28).contains(&n) {
                    Some(AnchorDay::Day(n as u32))
                } else if n > 28 {
                    self.push(
                        Error::new(
                            ErrorCode::InvalidValue,
                            path,
                            format!("anchor.day {n} does not exist in every month in every year"),
                        )
                        .hint("use \"last\" for the month's last day"),
                    );
                    None
                } else {
                    self.push(Error::new(
                        ErrorCode::InvalidValue,
                        path,
                        format!("anchor.day {n} is not a day; expected 1-28 or \"last\""),
                    ));
                    None
                }
            }
            _ => {
                self.type_error(v, path, "an integer 1-28 or \"last\"");
                None
            }
        }
    }

    fn week(&mut self, v: &Value) -> Option<Weekday> {
        let path = "week";
        let m = self.object(v, path)?;
        self.fields(m, path, &["starts_on", "numbering"], &[]);
        let starts_on = self.required(m, path, "starts_on").and_then(|v| {
            let s = self.string(v, "week.starts_on")?;
            let w = Weekday::from_name(s);
            if w.is_none() {
                self.push(
                    Error::new(
                        ErrorCode::InvalidValue,
                        "week.starts_on",
                        format!("\"{s}\" is not a weekday"),
                    )
                    .hint("use a lower-case English weekday name such as \"monday\""),
                );
            }
            w
        });
        let numbering = self.required(m, path, "numbering").and_then(|v| {
            self.choice(
                v,
                "week.numbering",
                Numbering::ALL,
                Numbering::name,
                &["first_full"],
            )
        });
        numbering.and(starts_on)
    }

    fn subdivision(&mut self, v: &Value) -> (Option<SubKind>, Option<SubSpec>) {
        let path = "subdivision";
        let Some(m) = self.object(v, path) else {
            return (None, None);
        };
        let Some(t) = self.required(m, path, "type") else {
            return (None, None);
        };
        let Some(kind) = self.choice(
            t,
            "subdivision.type",
            SubdivisionType::ALL,
            SubdivisionType::name,
            &["anchored"],
        ) else {
            return (None, None);
        };
        if kind == SubdivisionType::CalendarMonths {
            self.fields(
                m,
                path,
                &["type", "month_start_day", "quarters"],
                &["stub", "special_periods"],
            );
            let msd = self.required(m, path, "month_start_day").and_then(|v| {
                self.int_in(v, "subdivision.month_start_day", 1, 31)
                    .map(|n| n as u32)
            });
            let quarters = self
                .required(m, path, "quarters")
                .and_then(|v| self.positive_array(v, "subdivision.quarters"));
            if let Some(q) = &quarters {
                let sum: u64 = q.iter().map(|&x| u64::from(x)).sum();
                if sum != 12 {
                    self.push(
                        Error::new(
                            ErrorCode::QuartersSumMismatch,
                            "subdivision.quarters",
                            format!("quarters sum to {sum} periods, expected 12"),
                        )
                        .hint("a calendar_months year has 12 periods, for example [3, 3, 3, 3]"),
                    );
                    return (
                        Some(SubKind::CalendarMonths {
                            month_start_day: msd,
                        }),
                        None,
                    );
                }
            }
            let spec = match (msd, quarters) {
                (Some(_), Some(quarters)) => Some(SubSpec::CalendarMonths { quarters }),
                _ => None,
            };
            (
                Some(SubKind::CalendarMonths {
                    month_start_day: msd,
                }),
                spec,
            )
        } else {
            self.fields(
                m,
                path,
                &["type", "periods", "quarters", "extra_week"],
                &["stub", "special_periods"],
            );
            let mut periods = self
                .required(m, path, "periods")
                .and_then(|v| self.positive_array(v, "subdivision.periods"));
            if let Some(p) = &periods {
                if p.len() != 12 && p.len() != 13 {
                    self.push(Error::new(
                        ErrorCode::InvalidValue,
                        "subdivision.periods",
                        format!("periods has {} entries, expected 12 or 13", p.len()),
                    ));
                    periods = None;
                } else {
                    let sum: u64 = p.iter().map(|&x| u64::from(x)).sum();
                    if sum != 52 {
                        let hint = if sum < 52 {
                            let n = 52 - sum;
                            if n == 1 {
                                String::from("add one week to a period")
                            } else {
                                format!("add {n} weeks across the periods")
                            }
                        } else {
                            let n = sum - 52;
                            if n == 1 {
                                String::from("remove one week from a period")
                            } else {
                                format!("remove {n} weeks across the periods")
                            }
                        };
                        self.push(
                            Error::new(
                                ErrorCode::WeeksSumMismatch,
                                "subdivision.periods",
                                format!("periods sum to {sum} weeks, expected 52"),
                            )
                            .hint(hint),
                        );
                        periods = None;
                    }
                }
            }
            let mut quarters = self
                .required(m, path, "quarters")
                .and_then(|v| self.positive_array(v, "subdivision.quarters"));
            if let (Some(q), Some(p)) = (&quarters, &periods) {
                let sum: u64 = q.iter().map(|&x| u64::from(x)).sum();
                if sum != p.len() as u64 {
                    self.push(
                        Error::new(
                            ErrorCode::QuartersSumMismatch,
                            "subdivision.quarters",
                            format!("quarters sum to {sum} periods, expected {}", p.len()),
                        )
                        .hint("make the quarter sizes add up to the number of entries in periods"),
                    );
                    quarters = None;
                }
            }
            let extra = match m.get("extra_week") {
                None => {
                    self.push(
                        Error::new(
                            ErrorCode::ExtraWeekMissing,
                            "subdivision.extra_week",
                            "extra_week is missing, but every week-based rule produces 53-week years (71 in every 400 years)",
                        )
                        .hint("add \"extra_week\": {\"period\": N} naming the period that gains the 53rd week"),
                    );
                    None
                }
                Some(v) => self.extra_week(v, periods.as_ref().map(Vec::len)),
            };
            let spec = match (periods, quarters, extra) {
                (Some(periods), Some(quarters), Some(extra_period)) => Some(SubSpec::WeekPattern {
                    periods,
                    quarters,
                    extra_period,
                }),
                _ => None,
            };
            (Some(SubKind::WeekPattern), spec)
        }
    }

    fn extra_week(&mut self, v: &Value, n_periods: Option<usize>) -> Option<u32> {
        let path = "subdivision.extra_week";
        let m = self.object(v, path)?;
        self.fields(m, path, &["period"], &["standalone", "by_year"]);
        let p = self.required(m, path, "period")?;
        let n = self.integer(p, "subdivision.extra_week.period")?;
        let n_periods = n_periods?;
        if n < 1 || n > n_periods as i64 {
            self.push(
                Error::new(
                    ErrorCode::ExtraWeekOutOfRange,
                    "subdivision.extra_week.period",
                    format!("extra_week.period is {n}, but the year has periods 1 to {n_periods}"),
                )
                .hint(format!("name a period from 1 to {n_periods}")),
            );
            return None;
        }
        Some(n as u32)
    }

    fn naming(&mut self, v: &Value) -> Option<(Vec<Scheme>, usize)> {
        let path = "naming";
        let m = self.object(v, path)?;
        self.fields(m, path, &["default", "schemes"], &[]);
        let default = self
            .required(m, path, "default")
            .and_then(|v| self.string(v, "naming.default"));
        let mut names: Option<Vec<&str>> = None;
        let mut schemes = Some(Vec::new());
        if let Some(sv) = self.required(m, path, "schemes") {
            if let Some(sm) = self.object(sv, "naming.schemes") {
                if sm.is_empty() {
                    self.push(
                        Error::new(
                            ErrorCode::InvalidValue,
                            "naming.schemes",
                            "naming.schemes must contain at least one scheme",
                        )
                        .hint("add a scheme, for example \"fy\": {\"year_number\": \"end\", \"templates\": {\"year\": \"FY{yyyy}\"}}"),
                    );
                    schemes = None;
                }
                names = Some(sm.keys().map(String::as_str).collect());
                for (name, body) in sm {
                    let parsed = self.scheme(name, body);
                    match (&mut schemes, parsed) {
                        (Some(list), Some(s)) => list.push(s),
                        _ => schemes = None,
                    }
                }
            } else {
                schemes = None;
            }
        } else {
            schemes = None;
        }
        if let (Some(d), Some(names)) = (default, &names) {
            if !names.contains(&d) {
                let hint = if names.is_empty() {
                    String::from("add the scheme to naming.schemes")
                } else {
                    format!("set naming.default to one of: {}", names.join(" "))
                };
                self.push(
                    Error::new(
                        ErrorCode::DefaultSchemeMissing,
                        "naming.default",
                        format!("naming.default \"{d}\" is not a scheme name"),
                    )
                    .hint(hint),
                );
                return None;
            }
        }
        let mut schemes = schemes?;
        let default = default?;
        schemes.sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));
        let index = schemes.iter().position(|s| s.name == default)?;
        Some((schemes, index))
    }

    fn scheme(&mut self, name: &str, v: &Value) -> Option<Scheme> {
        let path = join("naming.schemes", name);
        let valid_name = {
            let b = name.as_bytes();
            !b.is_empty()
                && b.len() <= 32
                && b[0].is_ascii_lowercase()
                && b.iter()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'_')
        };
        if !valid_name {
            self.push(
                Error::new(
                    ErrorCode::InvalidValue,
                    path.clone(),
                    format!("\"{name}\" is not a valid scheme name"),
                )
                .hint("scheme names start with a lower-case letter and use only a-z, 0-9 and _, at most 32 characters"),
            );
        }
        let m = self.object(v, &path)?;
        self.fields(
            m,
            &path,
            &["year_number", "offset", "century_pivot", "templates"],
            &["eras"],
        );
        let year_number = self.required(m, &path, "year_number").and_then(|v| {
            self.choice(
                v,
                &join(&path, "year_number"),
                YearNumber::ALL,
                YearNumber::name,
                &["table"],
            )
        });
        let offset = match m.get("offset") {
            None => Some(0),
            Some(v) => self
                .int_in(v, &join(&path, "offset"), -OFFSET_LIMIT, OFFSET_LIMIT)
                .map(|n| n as i32),
        };
        let pivot = match m.get("century_pivot") {
            None | Some(Value::Null) => Some(None),
            Some(v) => self
                .int_in(v, &join(&path, "century_pivot"), 100, 9999)
                .map(|n| Some(n as i32)),
        };
        let tpath = join(&path, "templates");
        let mut templates: Option<[Option<Template>; 4]> = None;
        if let Some(tv) = self.required(m, &path, "templates") {
            if let Some(tm) = self.object(tv, &tpath) {
                self.fields(tm, &tpath, &["year", "quarter", "period", "week"], &[]);
                if tm.is_empty() {
                    self.push(
                        Error::new(
                            ErrorCode::InvalidValue,
                            tpath.clone(),
                            "templates must contain at least one of year, quarter, period, week",
                        )
                        .hint("add a year template such as \"FY{yyyy}\""),
                    );
                } else {
                    let mut out: [Option<Template>; 4] = Default::default();
                    let mut ok = true;
                    for level in Level::ALL {
                        let Some(tv) = tm.get(level.name()) else {
                            continue;
                        };
                        let lp = join(&tpath, level.name());
                        let Some(src) = self.string(tv, &lp) else {
                            ok = false;
                            continue;
                        };
                        match Template::compile(src, level) {
                            Ok(t) => out[level.index()] = Some(t),
                            Err(reason) => {
                                ok = false;
                                self.push(
                                    Error::new(ErrorCode::InvalidTemplate, lp, reason).hint(
                                        "placeholders are {yyyy} {yy} {yy_next} everywhere, {q} in quarter, {pp} in period and {ww} in week templates",
                                    ),
                                );
                            }
                        }
                    }
                    if ok {
                        templates = Some(out);
                    }
                }
            }
        }
        if let (Some(t), Some(None)) = (&templates, pivot) {
            if t.iter().flatten().any(Template::needs_pivot) {
                self.push(
                    Error::new(
                        ErrorCode::CenturyPivotMissing,
                        join(&path, "century_pivot"),
                        "a template uses {yy} without {yyyy}, so two-digit years need a century_pivot",
                    )
                    .hint("add \"century_pivot\", for example 2069 to read 70-99 as 1970-1999 and 00-69 as 2000-2069"),
                );
                return None;
            }
        }
        if !valid_name {
            return None;
        }
        Some(Scheme {
            name: name.to_owned(),
            year_number: year_number?,
            offset: offset?,
            century_pivot: pivot?,
            templates: templates?,
            shift: 0,
        })
    }

    fn fields(&mut self, m: &Map<String, Value>, path: &str, known: &[&str], reserved: &[&str]) {
        for key in m.keys() {
            if known.contains(&key.as_str()) {
                continue;
            }
            let p = join(path, key);
            if reserved.contains(&key.as_str()) {
                self.push(
                    Error::new(
                        ErrorCode::UnsupportedFeature,
                        p.clone(),
                        format!("{p} is reserved and not supported in fiscalclock 0.1"),
                    )
                    .hint(PLANNED),
                );
            } else {
                self.push(
                    Error::new(
                        ErrorCode::UnknownField,
                        p.clone(),
                        format!("{p} is not a field of the definition format"),
                    )
                    .hint("check the spelling against spec/definition-v1.md"),
                );
            }
        }
    }

    fn required<'a>(
        &mut self,
        m: &'a Map<String, Value>,
        path: &str,
        key: &str,
    ) -> Option<&'a Value> {
        let v = m.get(key);
        if v.is_none() {
            let p = join(path, key);
            self.push(Error::new(
                ErrorCode::MissingField,
                p.clone(),
                format!("{p} is required"),
            ));
        }
        v
    }

    fn type_error(&mut self, v: &Value, path: &str, expected: &str) {
        let found = match v {
            Value::Null => "null",
            Value::Bool(_) => "a boolean",
            Value::Number(n) if n.is_f64() => "a non-integer number",
            Value::Number(_) => "an integer",
            Value::String(_) => "a string",
            Value::Array(_) => "an array",
            Value::Object(_) => "an object",
        };
        self.push(Error::new(
            ErrorCode::InvalidType,
            path,
            format!("{path} must be {expected}, found {found}"),
        ));
    }

    fn object<'a>(&mut self, v: &'a Value, path: &str) -> Option<&'a Map<String, Value>> {
        let m = v.as_object();
        if m.is_none() {
            self.type_error(v, path, "an object");
        }
        m
    }

    fn string<'a>(&mut self, v: &'a Value, path: &str) -> Option<&'a str> {
        let s = v.as_str();
        if s.is_none() {
            self.type_error(v, path, "a string");
        }
        s
    }

    /// An integer, saturated to `i64::MAX` when it only fits `u64`.
    fn integer(&mut self, v: &Value, path: &str) -> Option<i64> {
        let n = v.as_i64().or_else(|| v.as_u64().map(|_| i64::MAX));
        if n.is_none() {
            self.type_error(v, path, "an integer");
        }
        n
    }

    fn int_in(&mut self, v: &Value, path: &str, lo: i64, hi: i64) -> Option<i64> {
        let n = self.integer(v, path)?;
        if n < lo || n > hi {
            self.push(Error::new(
                ErrorCode::InvalidValue,
                path,
                format!("{path} is {v}, expected an integer from {lo} to {hi}"),
            ));
            return None;
        }
        Some(n)
    }

    fn positive_array(&mut self, v: &Value, path: &str) -> Option<Vec<u32>> {
        let Some(items) = v.as_array() else {
            self.type_error(v, path, "an array of positive integers");
            return None;
        };
        let mut out = Some(Vec::with_capacity(items.len()));
        for (i, item) in items.iter().enumerate() {
            let p = format!("{path}[{i}]");
            let n = self.integer(item, &p);
            let valid = match n {
                Some(n) if (1..=i64::from(u32::MAX)).contains(&n) => Some(n as u32),
                Some(_) => {
                    self.push(Error::new(
                        ErrorCode::InvalidValue,
                        p.clone(),
                        format!("{p} is {item}, expected a positive integer"),
                    ));
                    None
                }
                None => None,
            };
            match (&mut out, valid) {
                (Some(list), Some(n)) => list.push(n),
                _ => out = None,
            }
        }
        out
    }

    fn choice<T: Copy>(
        &mut self,
        v: &Value,
        path: &str,
        options: &[T],
        name: fn(T) -> &'static str,
        reserved: &[&str],
    ) -> Option<T> {
        let s = self.string(v, path)?;
        if let Some(&t) = options.iter().find(|&&t| name(t) == s) {
            return Some(t);
        }
        if reserved.contains(&s) {
            self.push(
                Error::new(
                    ErrorCode::UnsupportedFeature,
                    path,
                    format!("{path} \"{s}\" is reserved and not supported in fiscalclock 0.1"),
                )
                .hint(PLANNED),
            );
            return None;
        }
        let allowed = options
            .iter()
            .map(|&t| format!("\"{}\"", name(t)))
            .collect::<Vec<_>>()
            .join(", ");
        self.push(
            Error::new(
                ErrorCode::InvalidValue,
                path,
                format!("{path} \"{s}\" is not one of {allowed}"),
            )
            .hint(format!("use one of {allowed}")),
        );
        None
    }
}
