use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use crate::calendar::{FiscalCalendar, Level, Period};
use crate::date::{Date, month_name};
use crate::definition::{AnchorDay, Edge, Rule, ShortMonth, Structure, YearNumber};
use crate::error::Error;
use crate::timeline::{Division, FiscalYear, Source};

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
        let _ = write!(s, ", {w} weeks");
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
        let (fy, loc) = self.locate_in(date, "date")?;
        let ws = self.def.week_start;
        let scheme = self.default_scheme();
        let numbering = match scheme.year_number {
            YearNumber::Start => "the calendar year of its first day",
            YearNumber::End => "the calendar year of its last day",
            YearNumber::Majority => "the calendar year that holds more than half of its days",
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
        let year_rule = match (fy.source, self.year_rule(&fy)) {
            (Source::Transition(i), _) => format!(
                "exceptions.history[{i}].transition makes {} to {} one fiscal year, between two rules{naming}",
                fy.start,
                fy.end.add(-1)
            ),
            (_, None) => String::new(),
            (Source::TopLevel, Some(rule)) => match self.def.history.first() {
                None => format!("{rule}{naming}"),
                Some(change) => format!(
                    "until {}, the top-level rule applies: {rule}{naming}",
                    change.change_point().add(-1)
                ),
            },
            (Source::History(i), Some(rule)) => format!(
                "from {}, exceptions.history[{i}] applies: {rule}{naming}",
                self.def.history[i].from
            ),
        };
        let k = loc.period.index.unwrap_or(1);
        let (period_rule, week_rule) = match fy.division {
            Division::Months {
                first,
                day,
                short_month,
                ..
            } => {
                let policy = match short_month {
                    Some(ShortMonth::Clamp) => {
                        ", or the month's last day when the month is shorter"
                    }
                    Some(ShortMonth::Next) => {
                        ", or the first day of the next month when the month is shorter"
                    }
                    None => "",
                };
                let month = (first + k as i32 - 1).rem_euclid(12) as u32 + 1;
                (
                    format!(
                        "period {k} starts on day {day} of {}{policy} and ends the day before the next period starts",
                        month_name(month)
                    ),
                    format!(
                        "weeks start on {}; week 1 runs from the fiscal year's first day to the day before the next {}",
                        ws.title(),
                        ws.title()
                    ),
                )
            }
            Division::Weeks {
                periods,
                extra_period,
                has_extra,
            } => {
                let base = periods[(k - 1) as usize];
                let extra = match (extra_period == Some(k), has_extra) {
                    (true, true) => " plus the extra week as its last week",
                    (true, false) => ", and gains the extra week in a 53-week year",
                    (false, _) => "",
                };
                (
                    format!("period {k} has {base} weeks{extra}"),
                    format!(
                        "weeks start on {}; week 1 starts on the fiscal year's first day",
                        ws.title()
                    ),
                )
            }
        };
        let q = loc.quarter.index.unwrap_or(1);
        let (first, last) = fy.quarter_periods(q);
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
                result: format!("period {k} runs {}", span(&loc.period)),
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

    /// How the rule of a regime's fiscal year bounds it; `None` for a transition.
    fn year_rule(&self, fy: &FiscalYear<'_>) -> Option<String> {
        let rule = match fy.source {
            Source::TopLevel => &self.def.structure,
            Source::History(i) => &self.def.history.get(i)?.structure,
            Source::Transition(_) => return None,
        };
        let ws = self.def.week_start;
        Some(match rule {
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
                format!(
                    "the fiscal year starts on {} {}{policy}",
                    f.day,
                    month_name(f.month)
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
                format!("the fiscal year {verb} on {choice}")
            }
        })
    }
}
