//! Shared helpers for the fuzz targets in `fuzz_targets/`.

use std::path::Path;
use std::sync::OnceLock;

use libfuzzer_sys::arbitrary::Unstructured;
use serde_json::{Map, Value};

/// Every `definition` object in `conformance/vectors/`, read once per process.
pub fn definitions() -> &'static [Value] {
    static DEFS: OnceLock<Vec<Value>> = OnceLock::new();
    DEFS.get_or_init(|| {
        let mut out = Vec::new();
        collect(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance/vectors"),
            &mut out,
        );
        assert!(
            !out.is_empty(),
            "no definitions found under conformance/vectors"
        );
        out
    })
}

fn collect(dir: &Path, out: &mut Vec<Value>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect(&p, out);
        } else if p.extension().is_some_and(|e| e == "json") {
            let v: Value = serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
            if let Some(d @ Value::Object(_)) = v.get("definition") {
                out.push(d.clone());
            }
        }
    }
}

/// The contract every JSON API output must meet: valid JSON, and an error
/// result is a non-empty list of objects with a code, a path and a message.
pub fn check(out: &str) {
    let v: Value = serde_json::from_str(out).unwrap_or_else(|e| panic!("not JSON ({e}): {out}"));
    if let Some(errors) = v.get("errors") {
        let list = errors.as_array().expect("errors is an array");
        assert!(!list.is_empty(), "empty errors: {out}");
        for e in list {
            for key in ["code", "path", "message"] {
                assert!(
                    e.get(key).is_some_and(Value::is_string),
                    "error without {key}: {out}"
                );
            }
            assert!(!e["code"].as_str().unwrap().is_empty(), "empty code: {out}");
        }
    }
}

/// Runs a few operations on a valid definition with arguments taken from `u`,
/// and checks each output.
pub fn exercise(definition: &str, u: &mut Unstructured) {
    let Ok(cal) = fiscalclock::FiscalCalendar::from_json(definition) else {
        panic!("validate said ok but from_json failed: {definition}");
    };
    let year: i64 = u.int_in_range(-2..=10_001).unwrap_or(2025);
    let date = format!(
        "{:04}-{:02}-{:02}",
        u.int_in_range(0..=10_000).unwrap_or(2025),
        u.int_in_range(0..=13).unwrap_or(1),
        u.int_in_range(0..=32).unwrap_or(1)
    );
    let level = *u
        .choose(&["year", "quarter", "period", "week"])
        .unwrap_or(&"year");
    let index: i64 = u.int_in_range(-1..=60).unwrap_or(1);
    let label: String = u.arbitrary().unwrap_or_default();
    let period = serde_json::json!({"level": level, "year": year, "index": if level == "year" { Value::Null } else { index.into() }});
    let calls = [
        ("locate", serde_json::json!({"date": date})),
        ("explain", serde_json::json!({"date": date})),
        ("period", period.clone()),
        ("format", serde_json::json!({"period": period})),
        (
            "shift",
            serde_json::json!({"period": period, "n": u.int_in_range(-100..=100).unwrap_or(1)}),
        ),
        (
            "children",
            serde_json::json!({"period": period, "level": level}),
        ),
        ("years", serde_json::json!({"from": year, "to": year + 3})),
        ("parse", serde_json::json!({"label": label})),
    ];
    for (op, args) in calls {
        check(&cal.call_json(op, &args.to_string()));
    }
}

const FIELDS: &[&str] = &[
    "spec_version",
    "year",
    "type",
    "start",
    "month",
    "day",
    "short_month",
    "anchor",
    "edge",
    "rule",
    "week",
    "starts_on",
    "numbering",
    "subdivision",
    "month_start_day",
    "quarters",
    "periods",
    "extra_week",
    "period",
    "standalone",
    "by_year",
    "stub",
    "absorb",
    "special_periods",
    "naming",
    "default",
    "schemes",
    "year_number",
    "offset",
    "century_pivot",
    "templates",
    "quarter",
    "eras",
    "exceptions",
    "history",
    "overrides",
    "transitions",
    "from",
    "transition",
    "fy",
];

const STRINGS: &[&str] = &[
    "fixed_date",
    "week_based",
    "table",
    "calendar_months",
    "week_pattern",
    "anchored",
    "clamp",
    "next",
    "start",
    "end",
    "majority",
    "last",
    "first",
    "nearest",
    "contains_start",
    "first_full",
    "first_and_last",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
    "{yyyy}",
    "FY{yy}",
    "{yyyy}-{yy_next}",
    "{yyyy}-{yy_end}",
    "Q{q} {yyyy}",
    "P{pp} {yyyy}",
    "W{ww} {yyyy}",
    "{yy}{yy}",
    "{",
    "}",
    "2024-04-01",
    "0001-01-02",
    "9999-12-30",
    "2024-02-29",
    "2023-02-29",
    "",
];

const NUMBERS: &[i64] = &[
    -1,
    0,
    1,
    2,
    3,
    4,
    5,
    12,
    13,
    28,
    29,
    30,
    31,
    52,
    53,
    99,
    100,
    2025,
    9999,
    10_000,
    1_000_000_000,
    1_000_000_001,
    i64::MAX,
    i64::MIN,
];

fn pool(u: &mut Unstructured) -> Value {
    match u.int_in_range(0..=6u8).unwrap_or(0) {
        0 => Value::from(*u.choose(STRINGS).unwrap_or(&"")),
        1 => Value::from(*u.choose(NUMBERS).unwrap_or(&0)),
        2 => Value::Null,
        3 => Value::Bool(u.arbitrary().unwrap_or(false)),
        4 => Value::Array(vec![
            Value::from(3),
            Value::from(3),
            Value::from(3),
            Value::from(3),
        ]),
        5 => Value::Object(Map::new()),
        _ => Value::from(u.arbitrary::<f64>().unwrap_or(0.5)),
    }
}

/// Changes one node of `v`: descends at random, then replaces the node, or
/// adds, removes or replaces a member of an object or array.
pub fn mutate(v: &mut Value, u: &mut Unstructured) {
    let descend = u.ratio(3u8, 4).unwrap_or(false);
    match v {
        Value::Object(m) if descend && !m.is_empty() => {
            let i = u.choose_index(m.len()).unwrap_or(0);
            let key = m.keys().nth(i).unwrap().clone();
            mutate(m.get_mut(&key).unwrap(), u);
        }
        Value::Array(a) if descend && !a.is_empty() => {
            let i = u.choose_index(a.len()).unwrap_or(0);
            mutate(&mut a[i], u);
        }
        Value::Object(m) => match u.int_in_range(0..=2u8).unwrap_or(0) {
            0 => {
                let key = (*u.choose(FIELDS).unwrap_or(&"year")).to_owned();
                let value = pool(u);
                m.insert(key, value);
            }
            1 if !m.is_empty() => {
                let i = u.choose_index(m.len()).unwrap_or(0);
                let key = m.keys().nth(i).unwrap().clone();
                m.remove(&key);
            }
            _ => *v = pool(u),
        },
        Value::Array(a) => match u.int_in_range(0..=2u8).unwrap_or(0) {
            0 => a.push(pool(u)),
            1 if !a.is_empty() => {
                a.pop();
            }
            _ => *v = pool(u),
        },
        _ => *v = pool(u),
    }
}
