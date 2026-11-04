#![allow(dead_code)]

use fiscalclock::{Date, FiscalCalendar};
use serde_json::{Value, json};

pub const WEEKDAYS: [&str; 7] = [
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
];

pub fn d(s: &str) -> Date {
    Date::parse_iso(s).unwrap()
}

pub fn naming(year_number: &str) -> Value {
    json!({
        "default": "fy",
        "schemes": {
            "fy": {
                "year_number": year_number,
                "templates": {
                    "year": "FY{yyyy}",
                    "quarter": "Q{q} FY{yyyy}",
                    "period": "P{pp} FY{yyyy}",
                    "week": "W{ww} FY{yyyy}"
                }
            }
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub fn week_based(
    starts_on: &str,
    edge: &str,
    month: u32,
    day: Value,
    rule: &str,
    periods: &[u32],
    quarters: &[u32],
    extra: u32,
    year_number: &str,
) -> Value {
    json!({
        "spec_version": 1,
        "year": {"type": "week_based", "anchor": {"edge": edge, "month": month, "day": day, "rule": rule}},
        "week": {"starts_on": starts_on, "numbering": "contains_start"},
        "subdivision": {"type": "week_pattern", "periods": periods, "quarters": quarters, "extra_week": {"period": extra}},
        "naming": naming(year_number)
    })
}

pub fn fixed_date(
    month: u32,
    day: u32,
    short_month: Option<&str>,
    starts_on: &str,
    quarters: &[u32],
    year_number: &str,
) -> Value {
    let mut year = json!({"type": "fixed_date", "start": {"month": month, "day": day}});
    if let Some(p) = short_month {
        year["short_month"] = json!(p);
    }
    json!({
        "spec_version": 1,
        "year": year,
        "week": {"starts_on": starts_on, "numbering": "contains_start"},
        "subdivision": {"type": "calendar_months", "month_start_day": day, "quarters": quarters},
        "naming": naming(year_number)
    })
}

pub const P454: [u32; 12] = [4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4];
pub const P445: [u32; 12] = [4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5];

pub fn nrf() -> Value {
    week_based(
        "sunday",
        "end",
        1,
        json!("last"),
        "nearest",
        &P454,
        &[3, 3, 3, 3],
        12,
        "start",
    )
}

pub fn apple() -> Value {
    week_based(
        "sunday",
        "end",
        9,
        json!("last"),
        "last",
        &P445,
        &[3, 3, 3, 3],
        12,
        "end",
    )
}

pub fn iso() -> Value {
    week_based(
        "monday",
        "start",
        1,
        json!(1),
        "nearest",
        &P445,
        &[3, 3, 3, 3],
        12,
        "majority",
    )
}

pub fn cal(v: &Value) -> FiscalCalendar {
    FiscalCalendar::from_json(&v.to_string()).unwrap_or_else(|e| panic!("{e}\n{v}"))
}
