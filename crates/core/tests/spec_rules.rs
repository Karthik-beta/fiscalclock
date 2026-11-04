//! One test per rule that the spec states in prose and no conformance vector
//! pins, so that a change to the rule fails here as well as in review.

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

    let mut def = nrf();
    def["exceptions"] = Value::Null;
    assert_eq!(validate(&def), one("UNSUPPORTED_FEATURE", "exceptions"));
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
