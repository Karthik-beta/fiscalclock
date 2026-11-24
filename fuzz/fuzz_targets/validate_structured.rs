//! `validate` on definitions made by changing a few fields of a conformance
//! vector's definition, so inputs get past the JSON parser; a valid result also
//! runs a few operations.
#![no_main]

use libfuzzer_sys::arbitrary::Unstructured;
use libfuzzer_sys::fuzz_target;

// Reading the vectors takes seconds in an instrumented build; outside the timed run.
fuzz_target!(init: { fiscalclock_fuzz::definitions(); }, |data: &[u8]| {
    let mut u = Unstructured::new(data);
    let defs = fiscalclock_fuzz::definitions();
    let mut def = defs[u.choose_index(defs.len()).unwrap_or(0)].clone();
    for _ in 0..u.int_in_range(1..=8u8).unwrap_or(1) {
        fiscalclock_fuzz::mutate(&mut def, &mut u);
    }
    let text = def.to_string();
    let out = fiscalclock::json::validate(&text);
    fiscalclock_fuzz::check(&out);
    if out.starts_with(r#"{"ok":true"#) {
        fiscalclock_fuzz::exercise(&text, &mut u);
    }
});
