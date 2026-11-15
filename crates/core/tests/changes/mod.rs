//! Definitions whose year rule changes (`exceptions.history`), shared by the
//! tests of 0.2.0 behaviour.
//!
//! The year boundaries before, during and after each change, and the transition
//! labels, are stated in the filings quoted in `research/filers/`:
//!
//! - Procter & Gamble Hygiene and Health Care (`research/filers/pg-hygiene.md`):
//!   July to June years, then the nine-month financial year "2024-25" from
//!   1 July 2024 to 31 March 2025, then April to March years.
//! - Siemens Limited (`research/filers/siemens-india.md`): October to
//!   September years, then the eighteen-month "FY 2024-26" from 1 October 2024
//!   to 31 March 2026, which "comprised of six quarters", then April to March.
//! - Linde India (`research/filers/linde-india.md`): calendar years, then the
//!   fifteen-month period "2022-23" from 1 January 2022 to 31 March 2023, then
//!   April to March.
//!
//! Illustrative, not from the filings: the `week` blocks, the quarters of the
//! regular years and of the P&G and Linde transitions, and the week-based
//! calendar in [`retail_to_september`].

#![allow(dead_code)]

use serde_json::{Value, json};

pub fn months(month: u32) -> (Value, Value) {
    (
        json!({"type": "fixed_date", "start": {"month": month, "day": 1}}),
        json!({"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]}),
    )
}

/// A `fixed_date` calendar starting on the first of `month`, with Monday weeks.
pub fn fixed(month: u32, year_number: &str, template: &str) -> Value {
    let (year, subdivision) = months(month);
    json!({
        "spec_version": 1,
        "year": year,
        "week": {"starts_on": "monday", "numbering": "contains_start"},
        "subdivision": subdivision,
        "naming": {"default": "fy", "schemes": {"fy": {
            "year_number": year_number,
            "templates": {
                "year": template,
                "quarter": format!("Q{{q}} {template}"),
                "period": format!("P{{pp}} {template}"),
                "week": format!("W{{ww}} {template}")
            }
        }}}
    })
}

/// A history entry moving to years starting on the first of `month` from
/// `from`, after a `calendar_months` transition from `start` with `quarters`.
pub fn entry(from: &str, month: u32, transition: Option<(&str, &[u32])>) -> Value {
    let (year, subdivision) = months(month);
    let transition = transition.map_or(Value::Null, |(start, quarters)| {
        json!({
            "start": start,
            "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": quarters}
        })
    });
    json!({"from": from, "transition": transition, "year": year, "subdivision": subdivision})
}

pub fn with_history(mut def: Value, entries: Vec<Value>) -> Value {
    def["exceptions"] = json!({ "history": entries });
    def
}

/// P&G Hygiene and Health Care, numbered by start year.
pub fn pg() -> Value {
    with_history(
        fixed(7, "start", "{yyyy}-{yy_end}"),
        vec![entry("2025-04-01", 4, Some(("2024-07-01", &[3, 3, 3])))],
    )
}

/// Siemens Limited, numbered by start year, so that 2025 is skipped.
pub fn siemens() -> Value {
    with_history(
        fixed(10, "start", "FY {yyyy}-{yy_end}"),
        vec![entry(
            "2026-04-01",
            4,
            Some(("2024-10-01", &[3, 3, 3, 3, 3, 3])),
        )],
    )
}

/// Linde India, numbered by start year.
pub fn linde() -> Value {
    with_history(
        fixed(1, "start", "{yyyy}-{yy_end}"),
        vec![entry(
            "2023-04-01",
            4,
            Some(("2022-01-01", &[3, 3, 3, 3, 3])),
        )],
    )
}

/// Illustrative: a 4-5-4 calendar ending on the Saturday nearest 31 January,
/// then a 34-week `week_pattern` transition from Sunday 4 February 2024, then
/// years ending on the Saturday nearest 30 September, from Sunday
/// 29 September 2024. Numbered by majority, which gives 2023, 2024, 2025.
pub fn retail_to_september() -> Value {
    json!({
        "spec_version": 1,
        "year": {"type": "week_based", "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
        "week": {"starts_on": "sunday", "numbering": "contains_start"},
        "subdivision": {"type": "week_pattern", "periods": [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4],
                        "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}},
        "naming": {"default": "fy", "schemes": {"fy": {"year_number": "majority", "templates": {
            "year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}", "period": "P{pp} FY{yyyy}", "week": "W{ww} FY{yyyy}"
        }}}},
        "exceptions": {"history": [{
            "from": "2024-09-29",
            "transition": {"start": "2024-02-04", "subdivision": {
                "type": "week_pattern", "periods": [4, 5, 4, 4, 5, 4, 4, 4], "quarters": [3, 3, 2]
            }},
            "year": {"type": "week_based", "anchor": {"edge": "end", "month": 9, "day": "last", "rule": "nearest"}},
            "subdivision": {"type": "week_pattern", "periods": [4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5],
                            "quarters": [3, 3, 3, 3], "extra_week": {"period": 12}}
        }]}
    })
}

/// Every definition above.
pub fn all() -> Vec<(&'static str, Value)> {
    vec![
        ("pg", pg()),
        ("siemens", siemens()),
        ("linde", linde()),
        ("retail_to_september", retail_to_september()),
    ]
}
