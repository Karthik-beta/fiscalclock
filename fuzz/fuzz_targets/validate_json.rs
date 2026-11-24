//! `validate` on arbitrary text: no panic, and the output meets the JSON API contract.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        fiscalclock_fuzz::check(&fiscalclock::json::validate(text));
    }
});
