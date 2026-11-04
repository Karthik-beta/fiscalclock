mod common;

use common::*;
use fiscalclock::{Definition, Details, ErrorCode, FiscalCalendar, json};
use serde_json::{Value, json};

fn errors_of(def: &Value) -> Vec<(ErrorCode, String)> {
    errors_of_text(&def.to_string())
}

fn errors_of_text(text: &str) -> Vec<(ErrorCode, String)> {
    Definition::from_json(text)
        .expect_err("definition should be rejected")
        .into_vec()
        .into_iter()
        .map(|e| (e.code, e.path))
        .collect()
}

fn only(def: &Value, code: ErrorCode, path: &str) {
    assert_eq!(errors_of(def), [(code, path.to_string())], "{def}");
}

#[test]
fn invalid_json() {
    for text in ["{", "[1, 2]", "\"x\"", "", "null"] {
        assert_eq!(
            errors_of_text(text),
            [(ErrorCode::InvalidJson, String::new())],
            "{text}"
        );
    }
}

#[test]
fn unknown_field_at_every_level() {
    let mut d = nrf();
    d["colour"] = json!("red");
    only(&d, ErrorCode::UnknownField, "colour");
    let mut d = nrf();
    d["year"]["anchor"]["weekday"] = json!("saturday");
    only(&d, ErrorCode::UnknownField, "year.anchor.weekday");
    let mut d = nrf();
    d["naming"]["schemes"]["fy"]["templates"]["day"] = json!("D{yyyy}");
    only(
        &d,
        ErrorCode::UnknownField,
        "naming.schemes.fy.templates.day",
    );
}

#[test]
fn missing_field() {
    let mut d = nrf();
    d["week"].as_object_mut().unwrap().remove("numbering");
    only(&d, ErrorCode::MissingField, "week.numbering");
    let mut d = nrf();
    d.as_object_mut().unwrap().remove("naming");
    only(&d, ErrorCode::MissingField, "naming");
}

#[test]
fn invalid_type() {
    let mut d = nrf();
    d["spec_version"] = json!("1");
    only(&d, ErrorCode::InvalidType, "spec_version");
    let mut d = nrf();
    d["year"]["anchor"]["month"] = json!(1.5);
    only(&d, ErrorCode::InvalidType, "year.anchor.month");
    let mut d = nrf();
    d["subdivision"]["periods"][3] = json!("4");
    only(&d, ErrorCode::InvalidType, "subdivision.periods[3]");
}

#[test]
fn invalid_value() {
    let cases: Vec<(Value, &str)> = vec![
        (
            {
                let mut d = nrf();
                d["year"]["anchor"]["day"] = json!(29);
                d
            },
            "year.anchor.day",
        ),
        (
            {
                let mut d = nrf();
                d["year"]["anchor"]["month"] = json!(13);
                d
            },
            "year.anchor.month",
        ),
        (
            {
                let mut d = nrf();
                d["week"]["starts_on"] = json!("Sunday");
                d
            },
            "week.starts_on",
        ),
        (
            {
                let mut d = nrf();
                d["subdivision"]["periods"] = json!(vec![4; 11]);
                d
            },
            "subdivision.periods",
        ),
        (
            {
                let mut d = nrf();
                d["subdivision"]["periods"][0] = json!(0);
                d
            },
            "subdivision.periods[0]",
        ),
        (
            {
                let mut d = fixed_date(4, 1, None, "monday", &[3, 3, 3, 3], "start");
                d["year"]["short_month"] = json!("clamp");
                d
            },
            "year.short_month",
        ),
        (
            {
                let mut d = nrf();
                d["naming"]["schemes"]["fy"]["century_pivot"] = json!(99);
                d
            },
            "naming.schemes.fy.century_pivot",
        ),
    ];
    for (d, path) in cases {
        only(&d, ErrorCode::InvalidValue, path);
    }
}

#[test]
fn anchor_day_above_28_hints_last() {
    let mut d = nrf();
    d["year"]["anchor"]["day"] = json!(31);
    let e = &Definition::from_json(&d.to_string())
        .unwrap_err()
        .into_vec()[0];
    assert!(e.hint.as_deref().unwrap().contains("\"last\""));
}

#[test]
fn invalid_scheme_name_and_empty_schemes() {
    let mut d = nrf();
    let body = d["naming"]["schemes"]["fy"].clone();
    d["naming"]["schemes"] = json!({"FY": body});
    d["naming"]["default"] = json!("FY");
    only(&d, ErrorCode::InvalidValue, "naming.schemes.FY");

    let mut d = nrf();
    d["naming"]["schemes"] = json!({});
    assert_eq!(
        errors_of(&d),
        [
            (ErrorCode::DefaultSchemeMissing, "naming.default".into()),
            (ErrorCode::InvalidValue, "naming.schemes".into()),
        ]
    );
}

#[test]
fn unsupported_spec_version() {
    for v in [2, 0, -1] {
        let mut d = nrf();
        d["spec_version"] = json!(v);
        d["future_field"] = json!(true);
        only(&d, ErrorCode::UnsupportedSpecVersion, "spec_version");
    }
}

#[test]
fn unsupported_feature_for_every_reserved_part() {
    let mut d = nrf();
    d["exceptions"] = json!({});
    only(&d, ErrorCode::UnsupportedFeature, "exceptions");

    let mut d = nrf();
    d["year"] = json!({"type": "table"});
    only(&d, ErrorCode::UnsupportedFeature, "year.type");

    let mut d = nrf();
    d["subdivision"]["type"] = json!("anchored");
    only(&d, ErrorCode::UnsupportedFeature, "subdivision.type");

    let mut d = nrf();
    d["subdivision"]["stub"] = json!({});
    only(&d, ErrorCode::UnsupportedFeature, "subdivision.stub");

    let mut d = nrf();
    d["subdivision"]["special_periods"] = json!([]);
    only(
        &d,
        ErrorCode::UnsupportedFeature,
        "subdivision.special_periods",
    );

    let mut d = nrf();
    d["subdivision"]["extra_week"]["standalone"] = json!(true);
    only(
        &d,
        ErrorCode::UnsupportedFeature,
        "subdivision.extra_week.standalone",
    );

    let mut d = nrf();
    d["subdivision"]["extra_week"]["by_year"] = json!({});
    only(
        &d,
        ErrorCode::UnsupportedFeature,
        "subdivision.extra_week.by_year",
    );

    let mut d = nrf();
    d["week"]["numbering"] = json!("first_full");
    only(&d, ErrorCode::UnsupportedFeature, "week.numbering");

    let mut d = nrf();
    d["naming"]["schemes"]["fy"]["year_number"] = json!("table");
    only(
        &d,
        ErrorCode::UnsupportedFeature,
        "naming.schemes.fy.year_number",
    );

    let mut d = nrf();
    d["naming"]["schemes"]["fy"]["eras"] = json!(null);
    only(&d, ErrorCode::UnsupportedFeature, "naming.schemes.fy.eras");

    let e = &Definition::from_json(&d.to_string())
        .unwrap_err()
        .into_vec()[0];
    assert!(e.hint.as_deref().unwrap().contains("0.2"));
}

#[test]
fn unsupported_combination_both_ways() {
    let mut d = nrf();
    d["subdivision"] =
        json!({"type": "calendar_months", "month_start_day": 1, "quarters": [3, 3, 3, 3]});
    only(&d, ErrorCode::UnsupportedCombination, "subdivision.type");

    let mut d = fixed_date(1, 1, None, "monday", &[3, 3, 3, 3], "start");
    d["subdivision"] = nrf()["subdivision"].clone();
    only(&d, ErrorCode::UnsupportedCombination, "subdivision.type");
}

#[test]
fn weeks_sum_mismatch_matches_the_api_example() {
    let mut d = nrf();
    d["subdivision"]["periods"][1] = json!(4);
    let out = json::validate(&d.to_string());
    assert_eq!(
        out,
        r#"{"errors":[{"code":"WEEKS_SUM_MISMATCH","path":"subdivision.periods","message":"periods sum to 51 weeks, expected 52","hint":"add one week to a period","details":null}]}"#
    );
}

#[test]
fn quarters_sum_mismatch_in_both_subdivisions() {
    let mut d = nrf();
    d["subdivision"]["quarters"] = json!([3, 3, 3]);
    only(&d, ErrorCode::QuartersSumMismatch, "subdivision.quarters");
    let d = fixed_date(1, 1, None, "monday", &[4, 4, 3], "start");
    only(&d, ErrorCode::QuartersSumMismatch, "subdivision.quarters");
}

#[test]
fn extra_week_missing_and_out_of_range() {
    let mut d = nrf();
    d["subdivision"]
        .as_object_mut()
        .unwrap()
        .remove("extra_week");
    only(&d, ErrorCode::ExtraWeekMissing, "subdivision.extra_week");
    for p in [0, 13] {
        let mut d = nrf();
        d["subdivision"]["extra_week"]["period"] = json!(p);
        only(
            &d,
            ErrorCode::ExtraWeekOutOfRange,
            "subdivision.extra_week.period",
        );
    }
}

#[test]
fn short_month_policy_missing() {
    let d = fixed_date(1, 29, None, "monday", &[3, 3, 3, 3], "start");
    only(&d, ErrorCode::ShortMonthPolicyMissing, "year.short_month");
}

#[test]
fn subdivision_year_mismatch() {
    let mut d = fixed_date(4, 1, None, "monday", &[3, 3, 3, 3], "start");
    d["subdivision"]["month_start_day"] = json!(2);
    only(
        &d,
        ErrorCode::SubdivisionYearMismatch,
        "subdivision.month_start_day",
    );
}

#[test]
fn default_scheme_missing() {
    let mut d = nrf();
    d["naming"]["default"] = json!("retail");
    only(&d, ErrorCode::DefaultSchemeMissing, "naming.default");
}

#[test]
fn invalid_template_cases() {
    let path = "naming.schemes.fy.templates.";
    for (level, template) in [
        ("year", "Fiscal year"),
        ("year", "FY{yy_next}"),
        ("year", "FY{YYYY}"),
        ("year", "FY{yyyy"),
        ("year", "FY}{yyyy}"),
        ("year", "FY{{yyyy}}"),
        ("year", "Q{q} {yyyy}"),
        ("quarter", "Q{pp} {yyyy}"),
        ("quarter", "FY{yyyy}"),
        ("week", "W{q} {yyyy}"),
    ] {
        let mut d = nrf();
        d["naming"]["schemes"]["fy"]["templates"] = json!({ level: template });
        only(&d, ErrorCode::InvalidTemplate, &format!("{path}{level}"));
    }
}

#[test]
fn century_pivot_needed_only_for_yy_without_yyyy() {
    let mut d = nrf();
    d["naming"]["schemes"]["fy"]["templates"] = json!({"year": "FY{yy}"});
    only(
        &d,
        ErrorCode::CenturyPivotMissing,
        "naming.schemes.fy.century_pivot",
    );
    d["naming"]["schemes"]["fy"]["century_pivot"] = json!(2069);
    assert!(Definition::from_json(&d.to_string()).is_ok());

    let mut d = nrf();
    d["naming"]["schemes"]["fy"]["templates"] = json!({"year": "FY {yyyy}-{yy_next}"});
    assert!(Definition::from_json(&d.to_string()).is_ok());
}

#[test]
fn year_number_collision_for_iso_weeks_numbered_by_start() {
    for rule in ["start", "end"] {
        let mut d = iso();
        d["naming"]["schemes"]["fy"]["year_number"] = json!(rule);
        let errors = Definition::from_json(&d.to_string())
            .unwrap_err()
            .into_vec();
        assert_eq!(errors.len(), 1, "{rule}");
        let e = &errors[0];
        assert_eq!(e.code, ErrorCode::YearNumberCollision);
        assert_eq!(e.path, "naming.schemes.fy.year_number");
        assert!(e.hint.as_deref().unwrap().contains("\"majority\""));
        let Some(Details::Years(years)) = &e.details else {
            panic!("no years")
        };
        assert!(!years.is_empty() && years.len() <= 5, "{years:?}");
    }
}

#[test]
fn collision_years_name_a_real_collision() {
    // ISO 2018 starts on Monday 1 January 2018 and ISO 2019 on Monday 31
    // December 2018, so "start" numbers both 2018.
    let mut d = iso();
    d["naming"]["schemes"]["fy"]["year_number"] = json!("start");
    let out: Value = serde_json::from_str(&json::validate(&d.to_string())).unwrap();
    let years = out["errors"][0]["details"]["years"].as_array().unwrap();
    let start = cal(&iso());
    for y in years {
        let n = y.as_i64().unwrap();
        // Under "start" a number is given twice or skipped; either way the
        // fiscal years around it start on both sides of 1 January.
        let a = start
            .period(fiscalclock::Level::Year, n as i32, None)
            .unwrap();
        let b = start
            .period(fiscalclock::Level::Year, n as i32 + 1, None)
            .unwrap();
        assert!(
            a.start.year() == b.start.year() || b.start.year() - a.start.year() == 2,
            "{a:?} {b:?}"
        );
    }
}

#[test]
fn majority_tie_for_a_fixed_year_starting_2_july() {
    // 2 July to 31 December holds 183 days, and 1 January to 1 July of a leap
    // year holds 183 more.
    let d = fixed_date(7, 2, None, "monday", &[3, 3, 3, 3], "majority");
    let errors = Definition::from_json(&d.to_string())
        .unwrap_err()
        .into_vec();
    assert_eq!(errors[0].code, ErrorCode::MajorityTie);
    assert_eq!(errors[0].path, "naming.schemes.fy.year_number");
    assert!(matches!(&errors[0].details, Some(Details::Years(y)) if y.len() <= 5));
    let ok = fixed_date(7, 2, None, "monday", &[3, 3, 3, 3], "start");
    assert!(Definition::from_json(&ok.to_string()).is_ok());
}

#[test]
fn errors_are_collected_and_sorted_by_path_then_code() {
    let mut d = nrf();
    d["week"]["starts_on"] = json!("funday");
    d["subdivision"]["periods"][0] = json!(5);
    d["naming"]["default"] = json!("zz");
    d["aaa"] = json!(1);
    d["year"]["anchor"].as_object_mut().unwrap().remove("rule");
    assert_eq!(
        errors_of(&d),
        [
            (ErrorCode::UnknownField, "aaa".into()),
            (ErrorCode::DefaultSchemeMissing, "naming.default".into()),
            (ErrorCode::WeeksSumMismatch, "subdivision.periods".into()),
            (ErrorCode::InvalidValue, "week.starts_on".into()),
            (ErrorCode::MissingField, "year.anchor.rule".into()),
        ]
    );
}

#[test]
fn canonical_form_ignores_order_whitespace_schema_and_defaults() {
    let a = cal(&nrf());
    let mut b = nrf();
    b["$schema"] = json!("https://example.invalid/schema.json");
    b["naming"]["schemes"]["fy"]["offset"] = json!(0);
    b["naming"]["schemes"]["fy"]["century_pivot"] = json!(null);
    let pretty = serde_json::to_string_pretty(&b).unwrap();
    let b = FiscalCalendar::from_json(&pretty).unwrap();
    assert_eq!(a.calendar_id(), b.calendar_id());
    assert_eq!(a.canonical_json(), b.canonical_json());
    let again = FiscalCalendar::from_json(a.canonical_json()).unwrap();
    assert_eq!(again.canonical_json(), a.canonical_json());
    assert_eq!(a.calendar_id().len(), 16);
    assert!(!a.canonical_json().contains("short_month"));

    let mut c = nrf();
    c["subdivision"]["extra_week"]["period"] = json!(11);
    assert_ne!(cal(&c).calendar_id(), a.calendar_id());
}

#[test]
fn canonical_form_escapes_control_characters_and_keeps_utf8() {
    let mut d = fixed_date(4, 1, None, "monday", &[3, 3, 3, 3], "start");
    d["naming"]["schemes"]["fy"]["templates"] = json!({"year": "\u{1}é\t\"\\{yyyy}\u{7f}"});
    let c = cal(&d);
    assert!(
        c.canonical_json()
            .contains(r#""year":"\u0001é\t\"\\{yyyy}"#),
        "{}",
        c.canonical_json()
    );
    assert!(c.canonical_json().contains("\"short_month\":null"));
}
