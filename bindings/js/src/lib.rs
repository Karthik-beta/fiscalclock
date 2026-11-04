//! The compiled half of the `fiscalclock` npm package.
//!
//! It passes JSON text between JavaScript and the Rust core and nothing else;
//! `ts/index.ts` converts JavaScript values to and from that JSON. Keeping all
//! logic in the core is what makes every language return byte-identical
//! results.

#![forbid(unsafe_code)]

use fiscalclock::{FiscalCalendar, json};
use wasm_bindgen::prelude::*;

/// `fiscalclock::json::validate`.
#[wasm_bindgen]
pub fn validate(definition: &str) -> String {
    json::validate(definition)
}

/// `fiscalclock::json::call`.
#[wasm_bindgen]
pub fn call(definition: &str, op: &str, args: &str) -> String {
    json::call(definition, op, args)
}

/// `fiscalclock::json::timestamp_rejected`.
#[wasm_bindgen(js_name = timestampRejected)]
pub fn timestamp_rejected(path: &str) -> String {
    json::timestamp_rejected(path)
}

/// The version of the Rust core compiled into this module.
#[wasm_bindgen(js_name = coreVersion)]
pub fn core_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

/// A validated calendar, so repeated calls skip validation.
#[wasm_bindgen]
pub struct Calendar(FiscalCalendar);

#[wasm_bindgen]
impl Calendar {
    /// Throws the core's errors JSON, as a string, when the definition is invalid.
    #[wasm_bindgen(constructor)]
    pub fn new(definition: &str) -> Result<Calendar, JsValue> {
        FiscalCalendar::from_json(definition)
            .map(Calendar)
            .map_err(|errors| JsValue::from_str(&json::errors_json(errors.as_slice())))
    }

    /// `FiscalCalendar::call_json`.
    #[wasm_bindgen(js_name = callJson)]
    pub fn call_json(&self, op: &str, args: &str) -> String {
        self.0.call_json(op, args)
    }

    /// The calendar's `calendar_id`.
    #[wasm_bindgen(getter, js_name = calendarId)]
    pub fn calendar_id(&self) -> String {
        self.0.calendar_id().into()
    }

    /// The definition's canonical form.
    #[wasm_bindgen(getter, js_name = canonicalJson)]
    pub fn canonical_json(&self) -> String {
        self.0.canonical_json().into()
    }
}
