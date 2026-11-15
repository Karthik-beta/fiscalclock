use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use crate::calendar::{FiscalCalendar, Level, Period};
use crate::date::{Date, month_name};
use crate::definition::{AnchorDay, Edge, Rule, ShortMonth, Structure, YearNumber};
use crate::error::Error;
use crate::timeline::{Division, Source, nth_size};

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
        let year_rule = match fy.source {
            Source::Transition(i) => format!(
                "exceptions.history[{i}].transition makes {} to {} one fiscal year, between two rules{naming}",
                fy.start,
                fy.end.add(-1)
            ),
            Source::TopLevel => {
                let rule = self.year_rule(&self.def.structure);
                match self.def.history.first() {
                    None => format!("{rule}{naming}"),
                    Some(change) => format!(
                        "until {}, the top-level rule applies: {rule}{naming}",
                        change.change_point().add(-1)
                    ),
                }
            }
            Source::History(i) => {
                let change = &self.def.history[i];
                format!(
                    "from {}, exceptions.history[{i}] applies: {}{naming}",
                    change.from,
                    self.year_rule(&change.structure)
                )
            }
        };
        let k = loc.period.index.unwrap_or(1);
        let (period_rule, week_rule) = match fy.division {
            Division::Months {
                first,
                day,
                short_month,
            } => {
                let policy = short_month_policy(short_month);
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
                week_totals,
                extra_period,
                has_extra,
            } => {
                let base = nth_size(week_totals, k);
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

    /// How a regime's rule bounds its fiscal years.
    fn year_rule(&self, rule: &Structure) -> String {
        let ws = self.def.week_start;
        match rule {
            Structure::FixedDate(f) => format!(
                "the fiscal year starts on {} {}{}",
                f.day,
                month_name(f.month),
                short_month_policy(f.short_month)
            ),
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
        }
    }
}

/// How a `short_month` policy moves a start day that a month does not have.
fn short_month_policy(short_month: Option<ShortMonth>) -> &'static str {
    match short_month {
        Some(ShortMonth::Clamp) => ", or the month's last day when the month is shorter",
        Some(ShortMonth::Next) => ", or the first day of the next month when the month is shorter",
        None => "",
    }
}
