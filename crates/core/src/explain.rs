use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::calendar::{FiscalCalendar, Level, Period};
use crate::date::{Date, month_name};
use crate::definition::{AnchorDay, Edge, Rule, ShortMonth, Structure};
use crate::error::Error;

/// The rule trace behind [`FiscalCalendar::locate`] for one date.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Explanation {
    /// The date explained.
    pub date: Date,
    /// One step per level, from the year down to the week.
    pub steps: Vec<Step>,
}

/// One step of an [`Explanation`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// The level this step decides.
    pub level: Level,
    /// The rule from the definition that applies.
    pub rule: String,
    /// What the rule gives for the date.
    pub result: String,
}

fn span(p: &Period) -> String {
    let mut s = format!("{} to {} ({} days", p.start, p.last_day(), p.days);
    if let Some(w) = p.weeks {
        s.push_str(&format!(", {w} weeks"));
    }
    if p.has_extra_week {
        s.push_str(if p.level == Level::Week {
            ", the extra week"
        } else {
            ", including the extra week"
        });
    }
    s.push(')');
    s
}

impl FiscalCalendar {
    /// Explains, level by level, why `date` falls where [`FiscalCalendar::locate`] says.
    ///
    /// The wording of each step is for people and may change in any release.
    ///
    /// # Errors
    ///
    /// `OUT_OF_RANGE` at `date` when the fiscal year containing `date` is outside
    /// the supported range.
    pub fn explain(&self, date: Date) -> Result<Explanation, Error> {
        let loc = self.locate(date)?;
        let ws = self.def.week_start;
        let scheme = &self.def.schemes[self.def.default_scheme];
        let numbering = match scheme.year_number {
            crate::definition::YearNumber::Start => "the calendar year of its first day",
            crate::definition::YearNumber::End => "the calendar year of its last day",
            crate::definition::YearNumber::Majority => {
                "the calendar year that holds more than half of its days"
            }
        };
        let offset = if scheme.offset == 0 {
            String::new()
        } else {
            format!(" plus {}", scheme.offset)
        };
        let naming = format!(
            "; scheme \"{}\" numbers it by {numbering}{offset}",
            scheme.name
        );
        let (year_rule, period_rule, week_rule) = match &self.def.structure {
            Structure::FixedDate(f) => {
                let policy = match f.short_month {
                    Some(ShortMonth::Clamp) => {
                        ", or the month's last day when the month is shorter"
                    }
                    Some(ShortMonth::Next) => {
                        ", or the first day of the next month when the month is shorter"
                    }
                    None => "",
                };
                let k = loc.period.index.unwrap_or(1);
                let month = (f.month - 1 + k - 1) % 12 + 1;
                (
                    format!(
                        "the fiscal year starts on {} {}{policy}{naming}",
                        f.day,
                        month_name(f.month)
                    ),
                    format!(
                        "period {k} starts on day {} of {}{policy} and ends the day before the next period starts",
                        f.day,
                        month_name(month)
                    ),
                    format!(
                        "weeks start on {}; week 1 runs from the fiscal year's first day to the day before the next {}",
                        ws.title(),
                        ws.title()
                    ),
                )
            }
            Structure::WeekBased(w) => {
                let target = match w.day {
                    AnchorDay::Last => format!("the last day of {}", month_name(w.month)),
                    AnchorDay::Day(d) => format!("{d} {}", month_name(w.month)),
                };
                let (verb, weekday) = match w.edge {
                    Edge::End => ("ends", ws.plus(-1)),
                    Edge::Start => ("starts", ws),
                };
                let choice = match w.rule {
                    Rule::Last => format!("the last {} on or before {target}", weekday.title()),
                    Rule::First => format!("the first {} on or after {target}", weekday.title()),
                    Rule::Nearest => format!("the {} nearest {target}", weekday.title()),
                };
                let k = loc.period.index.unwrap_or(1);
                let base = w.periods[(k - 1) as usize];
                let extra = if k == w.extra_period {
                    if loc.year.has_extra_week {
                        " plus the extra week as its last week"
                    } else {
                        ", and gains the extra week in a 53-week year"
                    }
                } else {
                    ""
                };
                (
                    format!("the fiscal year {verb} on {choice}{naming}"),
                    format!("period {k} has {base} weeks{extra}"),
                    format!(
                        "weeks start on {}; week 1 starts on the fiscal year's first day",
                        ws.title()
                    ),
                )
            }
        };
        let q = loc.quarter.index.unwrap_or(1);
        let (first, last) = self.quarter_periods(q);
        let steps = alloc::vec![
            Step {
                level: Level::Year,
                rule: year_rule,
                result: format!("fiscal year {} runs {}", loc.year.year, span(&loc.year)),
            },
            Step {
                level: Level::Quarter,
                rule: if first == last {
                    format!("quarter {q} is period {first}")
                } else {
                    format!("quarter {q} is periods {first} to {last}")
                },
                result: format!("quarter {q} runs {}", span(&loc.quarter)),
            },
            Step {
                level: Level::Period,
                rule: period_rule,
                result: format!(
                    "period {} runs {}",
                    loc.period.index.unwrap_or(1),
                    span(&loc.period)
                ),
            },
            Step {
                level: Level::Week,
                rule: week_rule,
                result: format!(
                    "week {} runs {}; {date} is day {} of the week",
                    loc.week.index.unwrap_or(1),
                    span(&loc.week),
                    loc.day_of_week
                ),
            },
        ];
        Ok(Explanation { date, steps })
    }
}
