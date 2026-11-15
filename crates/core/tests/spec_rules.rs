//! One test per rule that the spec states in prose and no conformance vector
//! pins, so that a change to the rule fails here as well as in review.

mod changes;
mod common;

use common::*;
use fiscalclock::json;
use serde_json::{Value, json};

fn errors(out: &str) -> Vec<(String, String)> {
    let v: Value = serde_json::from_str(out).unwrap();
    v["errors"]
        .as_array()
        .unwrap_or_else(|| panic!("expected errors, got {out}"))
        .iter()
        .map(|e| {
            (
                e["code"].as_str().unwrap().into(),
                e["path"].as_str().unwrap().into(),
            )
        })
        .collect()
}

fn validate(def: &Value) -> Vec<(String, String)> {
    errors(&json::validate(&def.to_string()))
}

fn call(def: &Value, op: &str, args: &str) -> Vec<(String, String)> {
    errors(&json::call(&def.to_string(), op, args))
}

fn one(code: &str, path: &str) -> Vec<(String, String)> {
    vec![(code.into(), path.into())]
}

#[test]
fn another_spec_version_is_the_only_error_reported() {
    let mut def = nrf();
    def["spec_version"] = json!(2);
    def["week"]["colour"] = json!("blue");
    assert_eq!(
        validate(&def),
        one("UNSUPPORTED_SPEC_VERSION", "spec_version")
    );
}

#[test]
fn null_means_absent_for_optional_fields_without_a_default() {
    let mut def = fixed_date(4, 1, None, "monday", &[3, 3, 3, 3], "start");
    def["year"]["short_month"] = Value::Null;
    def["naming"]["schemes"]["fy"]["century_pivot"] = Value::Null;
    let out: Value = serde_json::from_str(&json::validate(&def.to_string())).unwrap();
    assert_eq!(out["ok"], true, "{out}");
    let canonical = out["canonical"].as_str().unwrap();
    let again: Value = serde_json::from_str(&json::validate(canonical)).unwrap();
    assert_eq!(again["calendar_id"], out["calendar_id"]);

    // `exceptions: null` is absent, and absent exceptions leave the canonical
    // form, and so the calendar_id, as they were in 0.1.0.
    let mut def = nrf();
    let plain = json::validate(&def.to_string());
    def["exceptions"] = Value::Null;
    assert_eq!(json::validate(&def.to_string()), plain);
    assert!(!plain.contains("exceptions"), "{plain}");
}

#[test]
fn offset_is_limited_to_one_billion_either_way() {
    for (offset, ok) in [
        (1_000_000_000i64, true),
        (-1_000_000_000, true),
        (1_000_000_001, false),
        (-1_000_000_001, false),
    ] {
        let mut def = nrf();
        def["naming"]["schemes"]["fy"]["offset"] = json!(offset);
        let out: Value = serde_json::from_str(&json::validate(&def.to_string())).unwrap();
        if ok {
            assert_eq!(out["ok"], true, "{offset}: {out}");
        } else {
            assert_eq!(
                validate(&def),
                one("INVALID_VALUE", "naming.schemes.fy.offset"),
                "{offset}"
            );
        }
    }
}

#[test]
fn sub_year_templates_need_their_index_placeholder() {
    for (level, template) in [
        ("quarter", "FY{yyyy}"),
        ("period", "FY{yyyy} Q"),
        ("week", "W FY{yyyy}"),
    ] {
        let mut def = nrf();
        def["naming"]["schemes"]["fy"]["templates"][level] = json!(template);
        let path = format!("naming.schemes.fy.templates.{level}");
        assert_eq!(validate(&def), one("INVALID_TEMPLATE", &path), "{level}");
    }
}

#[test]
fn parse_of_an_out_of_range_year_is_out_of_range_at_label() {
    assert_eq!(
        call(&nrf(), "parse", r#"{"label":"FY9999"}"#),
        one("OUT_OF_RANGE", "label")
    );
    assert_eq!(
        call(&nrf(), "parse", r#"{"label":"FY0000"}"#),
        one("OUT_OF_RANGE", "label")
    );
}

#[test]
fn arguments_unknown_keys_ignored_unknown_op_and_non_object_rejected() {
    let out: Value = serde_json::from_str(&json::call(
        &nrf().to_string(),
        "locate",
        r#"{"date":"2024-02-03","note":1}"#,
    ))
    .unwrap();
    assert!(out.get("errors").is_none(), "{out}");
    assert_eq!(
        call(&nrf(), "frobnicate", "{}"),
        one("INVALID_ARGUMENT", "op")
    );
    assert_eq!(call(&nrf(), "locate", "[]"), one("INVALID_ARGUMENT", ""));
}

#[test]
fn index_rules() {
    assert_eq!(
        call(
            &nrf(),
            "period",
            r#"{"level":"quarter","year":2023,"index":-1}"#
        ),
        one("PERIOD_NOT_FOUND", "index")
    );
    assert_eq!(
        call(
            &nrf(),
            "period",
            r#"{"level":"quarter","year":2023,"index":99999999999}"#
        ),
        one("PERIOD_NOT_FOUND", "index")
    );
    assert_eq!(
        call(
            &nrf(),
            "period",
            r#"{"level":"year","year":2023,"index":1}"#
        ),
        one("INVALID_ARGUMENT", "index")
    );
}

#[test]
fn a_year_beyond_i32_is_out_of_range() {
    assert_eq!(
        call(&nrf(), "period", r#"{"level":"year","year":99999999999}"#),
        one("OUT_OF_RANGE", "year")
    );
}

#[test]
fn an_unformattable_dimension_label_is_null_in_json_and_empty_in_csv() {
    let mut def = nrf();
    def["naming"]["schemes"]["fy"]["offset"] = json!(9000);
    let args = r#"{"start":"2024-02-03","end_exclusive":"2024-02-04"}"#;
    let rows: Value =
        serde_json::from_str(&json::call(&def.to_string(), "dimension", args)).unwrap();
    assert_eq!(rows[0]["fiscal_year"], 11023, "{rows}");
    assert_eq!(rows[0]["label_fy_year"], Value::Null, "{rows}");
    let csv: Value =
        serde_json::from_str(&json::call(&def.to_string(), "dimension_csv", args)).unwrap();
    let text = csv.as_str().unwrap_or_else(|| panic!("{csv}"));
    let header: Vec<&str> = text.lines().next().unwrap().split(',').collect();
    let row: Vec<&str> = text.lines().nth(1).unwrap().split(',').collect();
    let col = header.iter().position(|h| *h == "label_fy_year").unwrap();
    assert_eq!(row[col], "");
}

// ---- exceptions.history (0.2.0) ----

fn ok(out: &str) -> Value {
    let v: Value = serde_json::from_str(out).unwrap();
    assert!(v.get("errors").is_none(), "{out}");
    v
}

fn vector_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            vector_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

#[test]
fn definitions_without_exceptions_keep_their_canonical_form_and_calendar_id() {
    // Every conformance vector that pins a calendar_id was generated against
    // 0.1.0, where exceptions did not exist.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/vectors");
    let mut files = Vec::new();
    vector_files(&root, &mut files);
    let mut checked = 0;
    for file in files {
        let vector: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
        let definition = &vector["definition"];
        if definition.get("exceptions").is_some() {
            continue;
        }
        for case in vector["cases"].as_array().unwrap() {
            let want = case.get("expect").or_else(|| case.get("expect_subset"));
            let Some(id) = want.and_then(|w| w.get("calendar_id")) else {
                continue;
            };
            let got = ok(&json::call(&definition.to_string(), "validate", "{}"));
            assert_eq!(&got["calendar_id"], id, "{}", file.display());
            if let Some(canonical) = want.and_then(|w| w.get("canonical")) {
                assert_eq!(&got["canonical"], canonical, "{}", file.display());
            }
            assert!(!got["canonical"].as_str().unwrap().contains("exceptions"));
            checked += 1;
        }
    }
    assert!(checked > 10, "only {checked} calendar_id cases found");
}

#[test]
fn canonical_form_with_exceptions() {
    let out = ok(&json::validate(&changes::pg().to_string()));
    let canonical = out["canonical"].as_str().unwrap();
    assert!(
        canonical.contains(r#""exceptions":{"history":[{"from":"2025-04-01","subdivision":{"month_start_day":1,"quarters":[3,3,3,3],"type":"calendar_months"},"transition":{"start":"2024-07-01","subdivision":{"month_start_day":1,"quarters":[3,3,3],"type":"calendar_months"}},"year":{"short_month":null,"start":{"day":1,"month":4},"type":"fixed_date"}}]}"#),
        "{canonical}"
    );
    // The canonical form validates to itself.
    let again = ok(&json::validate(canonical));
    assert_eq!(again["canonical"], out["canonical"]);
    assert_eq!(again["calendar_id"], out["calendar_id"]);

    // An absent transition is written as null, and null means absent.
    // Here only the quarters change, from three to four months long.
    let mut def = changes::fixed(4, "start", "FY{yyyy}");
    def["exceptions"] = json!({"history": [{
        "year": {"type": "fixed_date", "start": {"month": 4, "day": 1}},
        "subdivision": {"type": "calendar_months", "month_start_day": 1, "quarters": [4, 4, 4]},
        "from": "2025-04-01"
    }]});
    let absent = ok(&json::validate(&def.to_string()));
    assert!(
        absent["canonical"]
            .as_str()
            .unwrap()
            .contains(r#""transition":null"#)
    );
    def["exceptions"]["history"][0]["transition"] = Value::Null;
    assert_eq!(ok(&json::validate(&def.to_string())), absent);

    // The transition is part of the identity.
    let mut other = changes::pg();
    other["exceptions"]["history"][0]["transition"]["subdivision"]["quarters"] = json!([4, 5]);
    assert_ne!(
        ok(&json::validate(&other.to_string()))["calendar_id"],
        out["calendar_id"]
    );
}

#[test]
fn weeks_follow_one_rule_in_a_calendar_months_transition() {
    // The spec's P&G example: 1 July 2024 is a Monday, so the transition has
    // 39 seven-day weeks and week 40 is Monday 31 March 2025 alone.
    let def = changes::pg().to_string();
    let weeks = ok(&json::call(
        &def,
        "children",
        r#"{"period":{"level":"year","year":2024},"level":"week"}"#,
    ));
    let weeks = weeks.as_array().unwrap();
    assert_eq!(weeks.len(), 40);
    assert!(weeks[..39].iter().all(|w| w["days"] == 7));
    assert_eq!(
        (weeks[0]["start"].as_str(), weeks[0]["last_day"].as_str()),
        (Some("2024-07-01"), Some("2024-07-07"))
    );
    assert_eq!(
        (weeks[39]["start"].as_str(), weeks[39]["days"].as_i64()),
        (Some("2025-03-31"), Some(1))
    );
    assert!(
        weeks
            .iter()
            .all(|w| w["weeks"].is_null() && w["has_extra_week"] == false)
    );
    // The following year starts on a Tuesday, so its week 1 has six days.
    let w1 = ok(&json::call(
        &def,
        "period",
        r#"{"level":"week","year":2025,"index":1}"#,
    ));
    assert_eq!(
        (w1["start"].as_str(), w1["days"].as_i64()),
        (Some("2025-04-01"), Some(6))
    );
}

#[test]
fn weeks_nest_only_in_week_pattern_years() {
    // A calendar_months transition: a week's parent is the year, and the
    // weeks of a quarter are not nested.
    let pg = changes::pg();
    let week = r#"{"period":{"level":"week","year":2024,"index":3}}"#;
    let parent = ok(&json::call(&pg.to_string(), "parent", week));
    assert_eq!(
        (parent["level"].as_str(), parent["year"].as_i64()),
        (Some("year"), Some(2024))
    );
    assert_eq!(
        call(
            &pg,
            "children",
            r#"{"period":{"level":"quarter","year":2024,"index":1},"level":"week"}"#
        ),
        one("LEVEL_NOT_NESTED", "level")
    );

    // A week_pattern transition: a week's parent is its period, and quarters
    // and periods have weeks.
    let retail = changes::retail_to_september();
    let week = r#"{"period":{"level":"week","year":2024,"index":5}}"#;
    let parent = ok(&json::call(&retail.to_string(), "parent", week));
    assert_eq!(
        (parent["level"].as_str(), parent["index"].as_i64()),
        (Some("period"), Some(2))
    );
    let weeks = ok(&json::call(
        &retail.to_string(),
        "children",
        r#"{"period":{"level":"quarter","year":2024,"index":3},"level":"week"}"#,
    ));
    let numbers: Vec<_> = weeks
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w["index"].as_i64().unwrap())
        .collect();
    assert_eq!(numbers, (27..=34).collect::<Vec<_>>());
}

#[test]
fn explain_names_the_regime_or_transition() {
    let def = changes::siemens().to_string();
    let rule = |date: &str| -> String {
        let e = ok(&json::call(
            &def,
            "explain",
            &format!(r#"{{"date":"{date}"}}"#),
        ));
        e["steps"][0]["rule"].as_str().unwrap().to_string()
    };
    assert!(rule("2024-09-30").contains("top-level rule"));
    assert!(rule("2025-06-15").contains("exceptions.history[0].transition"));
    assert!(rule("2026-04-01").contains("exceptions.history[0] applies"));
}
