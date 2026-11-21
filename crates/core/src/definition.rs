use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::calendar::{FiscalCalendar, Level};
use crate::date::{Date, Weekday};
use crate::error::{Error, ErrorCode, Errors};
use crate::json::{write_list, write_str};
use crate::label::Template;
use crate::timeline::month_index;

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

/// How a transition year is split into periods.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum TransitionSubdivision {
    /// Fiscal months starting on `month_start_day`, which equals the day of the
    /// transition's first day and of its entry's `from`.
    CalendarMonths {
        month_start_day: u32,
        quarters: Vec<u32>,
    },
    /// Runs of whole weeks; the transition starts on `week.starts_on`.
    WeekPattern {
        periods: Vec<u32>,
        quarters: Vec<u32>,
    },
}

/// A transition: one fiscal year from `start` to the day before its entry's `from`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Transition {
    pub start: Date,
    pub subdivision: TransitionSubdivision,
}

/// One entry of `exceptions.history`: a rule in force from `from`, optionally
/// after a transition year.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Change {
    pub from: Date,
    pub transition: Option<Transition>,
    pub structure: Structure,
}

impl Change {
    /// The change point: the first day that the previous rule no longer covers.
    pub(crate) fn change_point(&self) -> Date {
        self.transition.as_ref().map_or(self.from, |t| t.start)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Scheme {
    pub name: String,
    pub year_number: YearNumber,
    pub offset: i32,
    pub century_pivot: Option<i32>,
    /// Indexed by [`Level::index`].
    pub templates: [Option<Template>; 4],
    /// The number, before `offset`, of the first fiscal year of each segment
    /// of the calendar's timeline; numbers are consecutive within a segment.
    /// Empty until numbering has run.
    pub numbers: Vec<i64>,
}

/// A parsed and validated calendar definition.
///
/// The only way to obtain one is [`Definition::from_json`], so every value of
/// this type satisfies the whole of `spec/definition-v1.md`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definition {
    /// The top-level rule, regime 0.
    pub(crate) structure: Structure,
    /// `exceptions.history` in order; empty when `exceptions` is absent.
    pub(crate) history: Vec<Change>,
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
    /// `INVALID_JSON` to `CHANGE_NOT_ON_BOUNDARY`.
    pub fn from_json(text: &str) -> Result<Definition, Errors> {
        FiscalCalendar::numbered(Definition::parse(text)?).map(|cal| cal.def)
    }

    /// Everything `from_json` checks except year numbering, which needs the
    /// timeline of a calendar. Every scheme's `numbers` is still empty.
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
        let (year, subdivision) = rule_values(&self.structure);
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
        let mut top = obj([
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
        ]);
        // Absent exceptions are left out, so that a definition without them
        // keeps its 0.1.0 canonical form and calendar_id.
        if let (false, Value::Object(map)) = (self.history.is_empty(), &mut top) {
            let history = self.history.iter().map(change_value).collect();
            map.insert(
                String::from("exceptions"),
                obj([("history", Value::Array(history))]),
            );
        }
        top
    }

    /// The checks that compare blocks with each other. They run only when
    /// every block is valid on its own, and report every error they find.
    fn cross_block_errors(&self) -> Vec<Error> {
        let ws = self.week_start;
        let mut errors = Vec::new();
        // The previous regime's rule, its `from` (none for regime 0) and its name.
        let mut previous = (&self.structure, None, String::from("the top-level rule"));
        for (i, change) in self.history.iter().enumerate() {
            let path = format!("exceptions.history[{i}]");
            let point = change.change_point();
            let point_path = if change.transition.is_some() {
                format!("{path}.transition.start")
            } else {
                format!("{path}.from")
            };
            let (rule, previous_from, name) = &previous;
            if let Some(pf) = previous_from.filter(|&pf: &Date| point <= pf) {
                errors.push(
                    Error::new(
                        ErrorCode::ExceptionsOverlap,
                        point_path.clone(),
                        format!(
                            "the change on {point} is not after exceptions.history[{}].from ({pf}), so that entry's rule would have no fiscal year",
                            i - 1
                        ),
                    )
                    .hint("list history entries in date order, each change after the previous entry's from"),
                );
            }
            if let Some((before, after)) = rule.off_boundary(ws, point) {
                errors.push(
                    Error::new(
                        ErrorCode::ChangeNotOnBoundary,
                        point_path,
                        format!("{point} is not the first day of a fiscal year under {name}"),
                    )
                    .hint(format!(
                        "under {name}, the fiscal years around {point} start on {before} and {after}"
                    )),
                );
            }
            if let Some((before, after)) = change.structure.off_boundary(ws, change.from) {
                errors.push(
                    Error::new(
                        ErrorCode::ChangeNotOnBoundary,
                        format!("{path}.from"),
                        format!(
                            "{} is not the first day of a fiscal year under this entry's rule",
                            change.from
                        ),
                    )
                    .hint(format!(
                        "under this entry's rule, the fiscal years around {} start on {before} and {after}",
                        change.from
                    )),
                );
            }
            if let Some(t) = &change.transition {
                transition_errors(t, change.from, ws, &path, &mut errors);
            }
            previous = (
                &change.structure,
                Some(change.from),
                format!("the rule of {path}"),
            );
        }
        errors
    }
}

/// The cross-block checks of a transition's subdivision against its dates.
fn transition_errors(
    t: &Transition,
    from: Date,
    week_start: Weekday,
    entry: &str,
    errors: &mut Vec<Error>,
) {
    let path = format!("{entry}.transition.subdivision");
    let (start, last) = (t.start, from.add(-1));
    match &t.subdivision {
        TransitionSubdivision::CalendarMonths {
            month_start_day: day,
            quarters,
        } => {
            if start.day() != *day || from.day() != *day {
                let hint = if start.day() == from.day() {
                    format!("set month_start_day to {}", start.day())
                } else {
                    String::from(
                        "a calendar_months transition needs transition.start and from on the same day of the month",
                    )
                };
                errors.push(
                    Error::new(
                        ErrorCode::SubdivisionYearMismatch,
                        format!("{path}.month_start_day"),
                        format!(
                            "month_start_day is {day}, but the transition runs from {start} to {last}"
                        ),
                    )
                    .hint(hint),
                );
                // The number of months is defined only when the days agree.
                return;
            }
            let months =
                month_index(from.year(), from.month()) - month_index(start.year(), start.month());
            let sum: u64 = quarters.iter().map(|&q| u64::from(q)).sum();
            if u64::try_from(months).ok() != Some(sum) {
                errors.push(
                    Error::new(
                        ErrorCode::QuartersSumMismatch,
                        format!("{path}.quarters"),
                        format!(
                            "quarters sum to {sum} periods, but the transition from {start} to {last} has {months} months"
                        ),
                    )
                    .hint(format!("make the quarter sizes add up to {months}")),
                );
            }
        }
        TransitionSubdivision::WeekPattern { periods, .. } => {
            let days = start.days_until(from);
            if start.weekday() != week_start || days % 7 != 0 {
                errors.push(
                    Error::new(
                        ErrorCode::SubdivisionYearMismatch,
                        format!("{path}.type"),
                        format!(
                            "a week_pattern transition must start on a {} and last whole weeks, but this one starts on a {} and lasts {days} days",
                            week_start.title(),
                            start.weekday().title()
                        ),
                    )
                    .hint("use a calendar_months transition, or dates that span whole weeks"),
                );
            }
            if days % 7 == 0 {
                let weeks = days / 7;
                let sum: u64 = periods.iter().map(|&p| u64::from(p)).sum();
                if u64::try_from(weeks).ok() != Some(sum) {
                    errors.push(
                        Error::new(
                            ErrorCode::WeeksSumMismatch,
                            format!("{path}.periods"),
                            format!(
                                "periods sum to {sum} weeks, but the transition from {start} to {last} has {weeks} weeks"
                            ),
                        )
                        .hint(format!("make the periods add up to {weeks} weeks")),
                    );
                }
            }
        }
    }
}

/// The canonical `year` and `subdivision` blocks of a rule.
fn rule_values(structure: &Structure) -> (Value, Value) {
    match structure {
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
    }
}

/// The canonical form of a history entry.
fn change_value(change: &Change) -> Value {
    let (year, subdivision) = rule_values(&change.structure);
    let transition = change.transition.as_ref().map_or(Value::Null, |t| {
        let subdivision = match &t.subdivision {
            TransitionSubdivision::CalendarMonths {
                month_start_day,
                quarters,
            } => obj([
                ("month_start_day", (*month_start_day).into()),
                ("quarters", quarters.clone().into()),
                ("type", SubdivisionType::CalendarMonths.name().into()),
            ]),
            TransitionSubdivision::WeekPattern { periods, quarters } => obj([
                ("periods", periods.clone().into()),
                ("quarters", quarters.clone().into()),
                ("type", SubdivisionType::WeekPattern.name().into()),
            ]),
        };
        obj([
            ("start", format!("{}", t.start).into()),
            ("subdivision", subdivision),
        ])
    });
    obj([
        ("from", format!("{}", change.from).into()),
        ("subdivision", subdivision),
        ("transition", transition),
        ("year", year),
    ])
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

const PLANNED: &str = "planned for fiscalclock 0.3";

/// The hint for a reserved field or value: its planned release, or none for
/// the parts that ADRs 0035, 0036 and 0038 leave unscheduled.
fn planned(name: &str) -> &'static str {
    match name {
        "standalone" | "by_year" | "first_full" | "overrides" => {
            "not scheduled for any release; see the reserved table in spec/definition-v1.md"
        }
        _ => PLANNED,
    }
}

/// The first and last dates allowed in `exceptions`.
///
/// They do not make every regime reach the supported range. A change near
/// the first date can leave regime 0 with no supported fiscal year, when each
/// of its years before the change starts before 0001-01-02; a change near the
/// last date can leave the last regime with none, when its first year ends
/// after 9999-12-30. The timeline then has no segment for that regime.
const CHANGE_DATES: (Date, Date) = (Date::civil(2, 1, 1), Date::civil(9998, 12, 31));

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
                "exceptions",
            ],
            &[],
        );
        if let Some(v) = self.required(top, "", "spec_version") {
            self.integer(v, "spec_version");
        }
        if let Some(v) = top.get("$schema") {
            self.string(v, "$schema");
        }
        let structure = self.rule(top, "");
        let week_start = self.required(top, "", "week").and_then(|v| self.week(v));
        let naming = self
            .required(top, "", "naming")
            .and_then(|v| self.naming(v));
        let history = match top.get("exceptions") {
            None | Some(Value::Null) => Some(Vec::new()),
            Some(v) => self.exceptions(v),
        };
        if !self.errors.is_empty() {
            return Err(Errors::sorted(self.errors));
        }
        let (Some(structure), Some(week_start), Some((schemes, default_scheme)), Some(history)) =
            (structure, week_start, naming, history)
        else {
            return Err(Errors::sorted(self.errors));
        };
        let def = Definition {
            structure,
            history,
            week_start,
            schemes,
            default_scheme,
        };
        let cross = def.cross_block_errors();
        if cross.is_empty() {
            Ok(def)
        } else {
            Err(Errors::sorted(cross))
        }
    }

    /// The `year` and `subdivision` blocks of the object at `prefix`, checked
    /// together; `None` when either is invalid.
    fn rule(&mut self, m: &Map<String, Value>, prefix: &str) -> Option<Structure> {
        let year_path = join(prefix, "year");
        let sub_path = join(prefix, "subdivision");
        let (year_kind, year_spec) = self
            .required(m, prefix, "year")
            .map_or((None, None), |v| self.year(v, &year_path));
        let (sub_kind, sub_spec) = self
            .required(m, prefix, "subdivision")
            .map_or((None, None), |v| self.subdivision(v, &sub_path));

        match (&year_kind, &sub_kind) {
            (Some(YearKind::Fixed { .. }), Some(SubKind::WeekPattern)) => {
                self.push(
                    Error::new(
                        ErrorCode::UnsupportedCombination,
                        join(&sub_path, "type"),
                        "a week_pattern subdivision inside a fixed_date year needs subdivision.stub, which is not supported yet",
                    )
                    .hint("use a week_based year, or calendar_months for a fixed_date year; stubs are planned for fiscalclock 0.3"),
                );
                return None;
            }
            (Some(YearKind::Week), Some(SubKind::CalendarMonths { .. })) => {
                self.push(
                    Error::new(
                        ErrorCode::UnsupportedCombination,
                        join(&sub_path, "type"),
                        "a calendar_months subdivision is valid only inside a fixed_date year",
                    )
                    .hint("use a week_pattern subdivision for a week_based year"),
                );
                return None;
            }
            (
                Some(YearKind::Fixed { day: Some(day) }),
                Some(SubKind::CalendarMonths {
                    month_start_day: Some(msd),
                }),
            ) if day != msd => {
                self.push(
                    Error::new(
                        ErrorCode::SubdivisionYearMismatch,
                        join(&sub_path, "month_start_day"),
                        format!("month_start_day is {msd} but year.start.day is {day}"),
                    )
                    .hint(format!("set month_start_day to {day}")),
                );
                return None;
            }
            _ => {}
        }
        match (year_spec?, sub_spec?) {
            (
                YearSpec::Fixed {
                    month,
                    day,
                    short_month,
                },
                SubSpec::CalendarMonths { quarters },
            ) => Some(Structure::FixedDate(FixedDate {
                month,
                day,
                short_month,
                quarters,
            })),
            (
                YearSpec::Week {
                    edge,
                    month,
                    day,
                    rule,
                },
                SubSpec::WeekPattern {
                    periods,
                    quarters,
                    extra_period,
                },
            ) => Some(Structure::WeekBased(WeekBased {
                edge,
                month,
                day,
                rule,
                periods,
                quarters,
                extra_period,
            })),
            _ => None,
        }
    }

    fn year(&mut self, v: &Value, path: &str) -> (Option<YearKind>, Option<YearSpec>) {
        let Some(m) = self.object(v, path) else {
            return (None, None);
        };
        let Some(t) = self.required(m, path, "type") else {
            return (None, None);
        };
        let Some(kind) = self.choice(
            t,
            &join(path, "type"),
            YearType::ALL,
            YearType::name,
            &["table"],
        ) else {
            return (None, None);
        };
        if kind == YearType::FixedDate {
            self.fields(m, path, &["type", "start", "short_month"], &[]);
            let start_path = join(path, "start");
            let mut month = None;
            let mut day = None;
            if let Some(s) = self.required(m, path, "start") {
                if let Some(sm) = self.object(s, &start_path) {
                    self.fields(sm, &start_path, &["month", "day"], &[]);
                    month = self.required(sm, &start_path, "month").and_then(|v| {
                        self.int_in(v, &join(&start_path, "month"), 1, 12)
                            .map(|n| n as u32)
                    });
                    day = self.required(sm, &start_path, "day").and_then(|v| {
                        self.int_in(v, &join(&start_path, "day"), 1, 31)
                            .map(|n| n as u32)
                    });
                }
            }
            let short_path = join(path, "short_month");
            let short = match m.get("short_month") {
                None | Some(Value::Null) => Some(None),
                Some(v) => self
                    .choice(v, &short_path, ShortMonth::ALL, ShortMonth::name, &[])
                    .map(Some),
            };
            let mut short_ok = short;
            match (day, short) {
                (Some(d), Some(None)) if d >= 29 => {
                    self.push(
                        Error::new(
                            ErrorCode::ShortMonthPolicyMissing,
                            short_path,
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
                            short_path,
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
            let ap = join(path, "anchor");
            let Some(am) = self.object(a, &ap) else {
                return (Some(YearKind::Week), None);
            };
            self.fields(am, &ap, &["edge", "month", "day", "rule"], &[]);
            let edge = self
                .required(am, &ap, "edge")
                .and_then(|v| self.choice(v, &join(&ap, "edge"), Edge::ALL, Edge::name, &[]));
            let month = self
                .required(am, &ap, "month")
                .and_then(|v| self.int_in(v, &join(&ap, "month"), 1, 12).map(|n| n as u32));
            let day = self
                .required(am, &ap, "day")
                .and_then(|v| self.anchor_day(v, &join(&ap, "day")));
            let rule = self
                .required(am, &ap, "rule")
                .and_then(|v| self.choice(v, &join(&ap, "rule"), Rule::ALL, Rule::name, &[]));
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

    fn anchor_day(&mut self, v: &Value, path: &str) -> Option<AnchorDay> {
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

    /// The subdivision `type` at `path`, with the reserved types rejected.
    fn subdivision_type(&mut self, m: &Map<String, Value>, path: &str) -> Option<SubdivisionType> {
        let t = self.required(m, path, "type")?;
        self.choice(
            t,
            &join(path, "type"),
            SubdivisionType::ALL,
            SubdivisionType::name,
            &["anchored"],
        )
    }

    fn subdivision(&mut self, v: &Value, path: &str) -> (Option<SubKind>, Option<SubSpec>) {
        let Some(m) = self.object(v, path) else {
            return (None, None);
        };
        let Some(kind) = self.subdivision_type(m, path) else {
            return (None, None);
        };
        let quarters_path = join(path, "quarters");
        if kind == SubdivisionType::CalendarMonths {
            self.fields(
                m,
                path,
                &["type", "month_start_day", "quarters"],
                &["stub", "special_periods"],
            );
            let msd = self.required(m, path, "month_start_day").and_then(|v| {
                self.int_in(v, &join(path, "month_start_day"), 1, 31)
                    .map(|n| n as u32)
            });
            let quarters = self
                .required(m, path, "quarters")
                .and_then(|v| self.positive_array(v, &quarters_path));
            if let Some(q) = &quarters {
                let sum: u64 = q.iter().map(|&x| u64::from(x)).sum();
                if sum != 12 {
                    self.push(
                        Error::new(
                            ErrorCode::QuartersSumMismatch,
                            quarters_path,
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
            let periods_path = join(path, "periods");
            let mut periods = self
                .required(m, path, "periods")
                .and_then(|v| self.positive_array(v, &periods_path));
            if let Some(p) = &periods {
                if p.len() != 12 && p.len() != 13 {
                    self.push(Error::new(
                        ErrorCode::InvalidValue,
                        periods_path,
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
                                periods_path,
                                format!("periods sum to {sum} weeks, expected 52"),
                            )
                            .hint(hint),
                        );
                        periods = None;
                    }
                }
            }
            let quarters = self.pattern_quarters(m, path, periods.as_deref());
            let extra_path = join(path, "extra_week");
            let extra = match m.get("extra_week") {
                None => {
                    self.push(
                        Error::new(
                            ErrorCode::ExtraWeekMissing,
                            extra_path,
                            "extra_week is missing, but every week-based rule produces 53-week years (71 in every 400 years)",
                        )
                        .hint("add \"extra_week\": {\"period\": N} naming the period that gains the 53rd week"),
                    );
                    None
                }
                Some(v) => self.extra_week(v, &extra_path, periods.as_ref().map(Vec::len)),
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

    /// The `quarters` of a `week_pattern` subdivision at `path`, which must
    /// sum to the number of `periods` when those are valid.
    fn pattern_quarters(
        &mut self,
        m: &Map<String, Value>,
        path: &str,
        periods: Option<&[u32]>,
    ) -> Option<Vec<u32>> {
        let quarters_path = join(path, "quarters");
        let quarters = self
            .required(m, path, "quarters")
            .and_then(|v| self.positive_array(v, &quarters_path))?;
        if let Some(p) = periods {
            let sum: u64 = quarters.iter().map(|&x| u64::from(x)).sum();
            if sum != p.len() as u64 {
                self.push(
                    Error::new(
                        ErrorCode::QuartersSumMismatch,
                        quarters_path,
                        format!("quarters sum to {sum} periods, expected {}", p.len()),
                    )
                    .hint("make the quarter sizes add up to the number of entries in periods"),
                );
                return None;
            }
        }
        Some(quarters)
    }

    fn extra_week(&mut self, v: &Value, path: &str, n_periods: Option<usize>) -> Option<u32> {
        let m = self.object(v, path)?;
        self.fields(m, path, &["period"], &["standalone", "by_year"]);
        let p = self.required(m, path, "period")?;
        let period_path = join(path, "period");
        let n = self.integer(p, &period_path)?;
        let n_periods = n_periods?;
        if n < 1 || n > n_periods as i64 {
            self.push(
                Error::new(
                    ErrorCode::ExtraWeekOutOfRange,
                    period_path,
                    format!("extra_week.period is {n}, but the year has periods 1 to {n_periods}"),
                )
                .hint(format!("name a period from 1 to {n_periods}")),
            );
            return None;
        }
        Some(n as u32)
    }

    fn exceptions(&mut self, v: &Value) -> Option<Vec<Change>> {
        let path = "exceptions";
        let m = self.object(v, path)?;
        self.fields(m, path, &["history"], &["overrides"]);
        let history = self.required(m, path, "history")?;
        let history_path = "exceptions.history";
        let Some(entries) = history.as_array() else {
            self.type_error(history, history_path, "an array of history entries");
            return None;
        };
        if entries.is_empty() {
            self.push(
                Error::new(
                    ErrorCode::InvalidValue,
                    history_path,
                    "exceptions.history must contain at least one entry",
                )
                .hint("add an entry with from, year and subdivision, or remove exceptions"),
            );
            return None;
        }
        let mut out = Some(Vec::with_capacity(entries.len()));
        for (i, entry) in entries.iter().enumerate() {
            let change = self.change(entry, &format!("{history_path}[{i}]"));
            match (&mut out, change) {
                (Some(list), Some(c)) => list.push(c),
                _ => out = None,
            }
        }
        out
    }

    fn change(&mut self, v: &Value, path: &str) -> Option<Change> {
        let m = self.object(v, path)?;
        self.fields(m, path, &["from", "year", "subdivision", "transition"], &[]);
        let from = self
            .required(m, path, "from")
            .and_then(|v| self.date(v, &join(path, "from")));
        let structure = self.rule(m, path);
        let transition_path = join(path, "transition");
        let transition = match m.get("transition") {
            None | Some(Value::Null) => Some(None),
            Some(v) => self.transition(v, &transition_path, from).map(Some),
        };
        Some(Change {
            from: from?,
            transition: transition?,
            structure: structure?,
        })
    }

    /// Parses a transition. Its start is compared with `from` here, apart from
    /// the subdivision, so that an invalid subdivision cannot hide the error.
    fn transition(&mut self, v: &Value, path: &str, from: Option<Date>) -> Option<Transition> {
        let m = self.object(v, path)?;
        self.fields(m, path, &["start", "subdivision"], &[]);
        let start_path = join(path, "start");
        let start = self
            .required(m, path, "start")
            .and_then(|v| self.date(v, &start_path))
            .filter(|&start| match from {
                Some(from) if start >= from => {
                    self.push(
                        Error::new(
                            ErrorCode::InvalidValue,
                            start_path.as_str(),
                            format!(
                                "transition.start is {start}, which is not before from ({from})"
                            ),
                        )
                        .hint(
                            "a transition ends the day before from, so it must start before from",
                        ),
                    );
                    false
                }
                _ => true,
            });
        let subdivision = self
            .required(m, path, "subdivision")
            .and_then(|v| self.transition_subdivision(v, &join(path, "subdivision")));
        Some(Transition {
            start: start?,
            subdivision: subdivision?,
        })
    }

    fn transition_subdivision(&mut self, v: &Value, path: &str) -> Option<TransitionSubdivision> {
        let m = self.object(v, path)?;
        let kind = self.subdivision_type(m, path)?;
        if kind == SubdivisionType::CalendarMonths {
            self.fields(
                m,
                path,
                &["type", "month_start_day", "quarters"],
                &["stub", "special_periods"],
            );
            let month_start_day = self.required(m, path, "month_start_day").and_then(|v| {
                self.int_in(v, &join(path, "month_start_day"), 1, 28)
                    .map(|n| n as u32)
            });
            let quarters = self
                .required(m, path, "quarters")
                .and_then(|v| self.positive_array(v, &join(path, "quarters")));
            Some(TransitionSubdivision::CalendarMonths {
                month_start_day: month_start_day?,
                quarters: quarters?,
            })
        } else {
            self.fields(
                m,
                path,
                &["type", "periods", "quarters"],
                &["stub", "special_periods"],
            );
            let periods_path = join(path, "periods");
            let periods = self
                .required(m, path, "periods")
                .and_then(|v| self.positive_array(v, &periods_path))
                .filter(|p| {
                    if p.is_empty() {
                        self.push(
                            Error::new(
                                ErrorCode::InvalidValue,
                                periods_path.clone(),
                                "periods must have at least one entry",
                            )
                            .hint("list the weeks in each period of the transition"),
                        );
                    }
                    !p.is_empty()
                });
            let quarters = self.pattern_quarters(m, path, periods.as_deref());
            Some(TransitionSubdivision::WeekPattern {
                periods: periods?,
                quarters: quarters?,
            })
        }
    }

    /// A date in `exceptions`: a `YYYY-MM-DD` string from 0002-01-01 to 9998-12-31.
    fn date(&mut self, v: &Value, path: &str) -> Option<Date> {
        let Some(s) = v.as_str() else {
            self.type_error(v, path, "a YYYY-MM-DD date string");
            return None;
        };
        let (lo, hi) = CHANGE_DATES;
        let date = Date::parse_iso(s).filter(|d| (lo..=hi).contains(d));
        if date.is_none() {
            self.push(
                Error::new(
                    ErrorCode::InvalidValue,
                    path,
                    format!("\"{s}\" is not a YYYY-MM-DD date from {lo} to {hi}"),
                )
                .hint("write the date as YYYY-MM-DD, for example \"2025-04-01\""),
            );
        }
        date
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
                                        "placeholders are {yyyy} {yy} {yy_next} {yy_end} everywhere, {q} in quarter, {pp} in period and {ww} in week templates",
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
            numbers: Vec::new(),
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
                        format!("{p} is reserved and not supported in fiscalclock 0.2"),
                    )
                    .hint(planned(key)),
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
                    format!("{path} \"{s}\" is reserved and not supported in fiscalclock 0.2"),
                )
                .hint(planned(s)),
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
