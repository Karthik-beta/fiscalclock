//! Fiscal calendar engine: compute any fiscal calendar from a JSON definition.
//!
//! A definition (see `spec/definition-v1.md`) describes how a fiscal year is
//! bounded, subdivided, divided into weeks and named. [`FiscalCalendar`]
//! validates it once and then answers every question from it: which period a
//! date falls in, the bounds of a period, labels, a date dimension and a trace
//! of the rules applied.
//!
//! ```
//! use fiscalclock::{Date, FiscalCalendar, Level};
//!
//! let nrf = FiscalCalendar::from_json(r#"{
//!   "spec_version": 1,
//!   "year": {"type": "week_based",
//!            "anchor": {"edge": "end", "month": 1, "day": "last", "rule": "nearest"}},
//!   "week": {"starts_on": "sunday", "numbering": "contains_start"},
//!   "subdivision": {"type": "week_pattern", "periods": [4,5,4,4,5,4,4,5,4,4,5,4],
//!                   "quarters": [3,3,3,3], "extra_week": {"period": 12}},
//!   "naming": {"default": "fy", "schemes": {"fy": {"year_number": "start",
//!              "templates": {"year": "FY{yyyy}", "quarter": "Q{q} FY{yyyy}"}}}}
//! }"#).unwrap();
//!
//! let fy2023 = nrf.period(Level::Year, 2023, None).unwrap();
//! assert_eq!(fy2023.start, Date::from_ymd(2023, 1, 29).unwrap());
//! assert_eq!(fy2023.last_day(), Date::from_ymd(2024, 2, 3).unwrap());
//! assert_eq!(fy2023.weeks, Some(53));
//!
//! let here = nrf.locate(Date::from_ymd(2024, 2, 3).unwrap()).unwrap();
//! assert_eq!(nrf.format(&here.quarter, None).unwrap(), "Q4 FY2023");
//! ```
//!
//! Bindings for other languages call [`json::validate`], [`json::call`] and
//! [`FiscalCalendar::call_json`], which take and return JSON text, and build
//! their own failures with [`json::errors_json`] and
//! [`json::timestamp_rejected`]. Every language therefore gives byte-identical
//! results.
//!
//! The crate is `no_std` with `alloc`, reads no clock, environment or global
//! state, and never panics on any input: every failure is an [`Error`] value.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

mod calendar;
mod date;
mod definition;
mod dimension;
mod error;
mod explain;
pub mod json;
mod label;
mod strict;

pub use calendar::{FiscalCalendar, Level, Location, Period};
pub use date::{Date, ParseDateError, Weekday};
pub use definition::Definition;
pub use dimension::{DimensionRow, FIXED_COLUMNS};
pub use error::{Candidate, Details, Error, ErrorCode, Errors};
pub use explain::{Explanation, Step};
