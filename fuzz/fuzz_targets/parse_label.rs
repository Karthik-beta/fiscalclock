//! `parse` and `format` with arbitrary templates and labels, on the year and
//! subdivision of a conformance vector's definition.
#![no_main]

use libfuzzer_sys::arbitrary::Unstructured;
use libfuzzer_sys::fuzz_target;
use serde_json::{Value, json};

// Reading the vectors takes seconds in an instrumented build; outside the timed run.
fuzz_target!(init: { fiscalclock_fuzz::definitions(); }, |data: &[u8]| {
    let mut u = Unstructured::new(data);
    let defs = fiscalclock_fuzz::definitions();
    let mut def = defs[u.choose_index(defs.len()).unwrap_or(0)].clone();
    let mut templates = serde_json::Map::new();
    for level in ["year", "quarter", "period", "week"] {
        if u.ratio(3u8, 4).unwrap_or(true) {
            let t: String = u.arbitrary().unwrap_or_default();
            templates.insert(level.into(), Value::from(t));
        }
    }
    let pivot: Option<u16> = u.arbitrary().unwrap_or(None);
    def["naming"] = json!({
        "default": "s",
        "schemes": {"s": {"year_number": "start", "century_pivot": pivot, "templates": templates}}
    });
    let text = def.to_string();
    let out = fiscalclock::json::validate(&text);
    fiscalclock_fuzz::check(&out);
    if out.starts_with(r#"{"ok":true"#) {
        fiscalclock_fuzz::exercise(&text, &mut u);
    }
});
