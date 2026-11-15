mod changes;
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
    d["exceptions"] =
        json!({"overrides": null, "history": [changes::entry("2030-04-01", 4, None)]});
    only(&d, ErrorCode::UnsupportedFeature, "exceptions.overrides");

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
    assert!(e.hint.as_deref().unwrap().contains("0.3"));
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
fn weeks_sum_mismatch_has_the_layout_of_the_api_example() {
    // The message and hint wording is not API, only their place in the JSON.
    let mut d = nrf();
    d["subdivision"]["periods"][1] = json!(4);
    let out = json::validate(&d.to_string());
    let layout =
        r#"{"errors":[{"code":"WEEKS_SUM_MISMATCH","path":"subdivision.periods","message":""#;
    assert!(out.starts_with(layout), "{out}");
    assert!(out.ends_with(r#"","details":null}]}"#), "{out}");
    assert_eq!(out.matches(r#"","hint":""#).count(), 1, "{out}");
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

// ---- exceptions.history (0.2.0) ----

/// P&G Hygiene and Health Care's calendar (see `changes`) with `edit` applied
/// to its only history entry.
fn pg_with(edit: impl FnOnce(&mut Value)) -> Value {
    let mut d = changes::pg();
    edit(&mut d["exceptions"]["history"][0]);
    d
}

fn errors_and_hints(def: &Value) -> Vec<(ErrorCode, String, String)> {
    Definition::from_json(&def.to_string())
        .expect_err("definition should be rejected")
        .into_vec()
        .into_iter()
        .map(|e| (e.code, e.path, e.hint.unwrap_or_default()))
        .collect()
}

#[test]
fn the_filer_timelines_validate() {
    for (name, def) in changes::all() {
        let result = Definition::from_json(&def.to_string());
        assert!(result.is_ok(), "{name}: {:?}", result.err());
    }
}

#[test]
fn exceptions_block_level_errors() {
    let h = "exceptions.history";
    let cases: Vec<(Value, ErrorCode, String)> = vec![
        (
            {
                let mut d = nrf();
                d["exceptions"] = json!(5);
                d
            },
            ErrorCode::InvalidType,
            "exceptions".into(),
        ),
        (
            {
                let mut d = nrf();
                d["exceptions"] = json!({});
                d
            },
            ErrorCode::MissingField,
            h.into(),
        ),
        (
            {
                let mut d = nrf();
                d["exceptions"] = json!({"history": []});
                d
            },
            ErrorCode::InvalidValue,
            h.into(),
        ),
        (
            {
                let mut d = nrf();
                d["exceptions"] = json!({"history": null});
                d
            },
            ErrorCode::InvalidType,
            h.into(),
        ),
        (
            {
                let mut d = changes::pg();
                d["exceptions"]["transitions"] = json!([]);
                d
            },
            ErrorCode::UnknownField,
            "exceptions.transitions".into(),
        ),
        (
            {
                let mut d = changes::pg();
                d["exceptions"]["history"][0] = json!("2025-04-01");
                d
            },
            ErrorCode::InvalidType,
            format!("{h}[0]"),
        ),
        (
            pg_with(|e| e["week"] = json!({"starts_on": "sunday", "numbering": "contains_start"})),
            ErrorCode::UnknownField,
            format!("{h}[0].week"),
        ),
        (
            pg_with(|e| e["naming"] = json!({})),
            ErrorCode::UnknownField,
            format!("{h}[0].naming"),
        ),
        (
            pg_with(|e| {
                e.as_object_mut().unwrap().remove("from");
            }),
            ErrorCode::MissingField,
            format!("{h}[0].from"),
        ),
        (
            pg_with(|e| {
                e.as_object_mut().unwrap().remove("subdivision");
            }),
            ErrorCode::MissingField,
            format!("{h}[0].subdivision"),
        ),
        (
            pg_with(|e| e["from"] = json!(20250401)),
            ErrorCode::InvalidType,
            format!("{h}[0].from"),
        ),
        (
            pg_with(|e| e["transition"] = json!("2024-07-01")),
            ErrorCode::InvalidType,
            format!("{h}[0].transition"),
        ),
        (
            pg_with(|e| e["transition"]["subdivision"]["extra_week"] = json!({"period": 1})),
            ErrorCode::UnknownField,
            format!("{h}[0].transition.subdivision.extra_week"),
        ),
        (
            pg_with(|e| e["transition"]["end"] = json!("2025-03-31")),
            ErrorCode::UnknownField,
            format!("{h}[0].transition.end"),
        ),
        (
            pg_with(|e| e["transition"]["subdivision"]["type"] = json!("anchored")),
            ErrorCode::UnsupportedFeature,
            format!("{h}[0].transition.subdivision.type"),
        ),
        (
            pg_with(|e| e["transition"]["subdivision"]["type"] = json!("table")),
            ErrorCode::InvalidValue,
            format!("{h}[0].transition.subdivision.type"),
        ),
        (
            pg_with(|e| e["transition"]["subdivision"]["stub"] = json!({})),
            ErrorCode::UnsupportedFeature,
            format!("{h}[0].transition.subdivision.stub"),
        ),
        (
            pg_with(|e| e["transition"]["subdivision"]["special_periods"] = json!([])),
            ErrorCode::UnsupportedFeature,
            format!("{h}[0].transition.subdivision.special_periods"),
        ),
        (
            pg_with(|e| e["transition"]["subdivision"]["month_start_day"] = json!(29)),
            ErrorCode::InvalidValue,
            format!("{h}[0].transition.subdivision.month_start_day"),
        ),
        (
            pg_with(|e| {
                e["transition"]["subdivision"] =
                    json!({"type": "week_pattern", "periods": [], "quarters": [1]});
            }),
            ErrorCode::InvalidValue,
            format!("{h}[0].transition.subdivision.periods"),
        ),
        (
            pg_with(|e| {
                e["transition"]["subdivision"] =
                    json!({"type": "week_pattern", "periods": [13, 13, 13], "quarters": [1, 1]});
            }),
            ErrorCode::QuartersSumMismatch,
            format!("{h}[0].transition.subdivision.quarters"),
        ),
        (
            pg_with(|e| e["transition"]["start"] = json!("2025-04-01")),
            ErrorCode::InvalidValue,
            format!("{h}[0].transition.start"),
        ),
        (
            pg_with(|e| e["transition"]["start"] = json!("2026-01-01")),
            ErrorCode::InvalidValue,
            format!("{h}[0].transition.start"),
        ),
        (
            pg_with(|e| {
                e["year"]["start"]["day"] = json!(31);
                e["subdivision"]["month_start_day"] = json!(31);
            }),
            ErrorCode::ShortMonthPolicyMissing,
            format!("{h}[0].year.short_month"),
        ),
        (
            pg_with(|e| e["subdivision"]["month_start_day"] = json!(2)),
            ErrorCode::SubdivisionYearMismatch,
            format!("{h}[0].subdivision.month_start_day"),
        ),
        (
            pg_with(|e| e["subdivision"] = nrf()["subdivision"].clone()),
            ErrorCode::UnsupportedCombination,
            format!("{h}[0].subdivision.type"),
        ),
        (
            pg_with(|e| e["subdivision"]["quarters"] = json!([6, 6, 1])),
            ErrorCode::QuartersSumMismatch,
            format!("{h}[0].subdivision.quarters"),
        ),
        (
            pg_with(|e| e["subdivision"]["stub"] = json!({})),
            ErrorCode::UnsupportedFeature,
            format!("{h}[0].subdivision.stub"),
        ),
        (
            pg_with(|e| e["year"] = json!({"type": "table"})),
            ErrorCode::UnsupportedFeature,
            format!("{h}[0].year.type"),
        ),
    ];
    for (d, code, path) in cases {
        only(&d, code, &path);
    }
}

#[test]
fn exceptions_dates_are_strict_and_bounded() {
    for bad in [
        "2025-4-1",
        "2025-02-29",
        "2025-04-01T00:00:00",
        "0001-12-31",
        "9999-01-01",
        "",
    ] {
        only(
            &pg_with(|e| e["from"] = json!(bad)),
            ErrorCode::InvalidValue,
            "exceptions.history[0].from",
        );
        only(
            &pg_with(|e| e["transition"]["start"] = json!(bad)),
            ErrorCode::InvalidValue,
            "exceptions.history[0].transition.start",
        );
    }
}

#[test]
fn reserved_overrides_is_unsupported_even_when_null() {
    let mut d = changes::pg();
    d["exceptions"]["overrides"] = Value::Null;
    only(&d, ErrorCode::UnsupportedFeature, "exceptions.overrides");
    let e = &Definition::from_json(&d.to_string())
        .unwrap_err()
        .into_vec()[0];
    assert!(e.hint.as_deref().unwrap().contains("0.3"));
}

#[test]
fn exceptions_overlap_is_reported_at_the_change_point() {
    // Siemens (see `changes`) plus a second entry whose change is on or before
    // the first entry's from, 2026-04-01.
    let mut d = changes::siemens();
    d["exceptions"]["history"]
        .as_array_mut()
        .unwrap()
        .push(changes::entry("2025-04-01", 4, None));
    only(
        &d,
        ErrorCode::ExceptionsOverlap,
        "exceptions.history[1].from",
    );

    let mut d = changes::siemens();
    d["exceptions"]["history"]
        .as_array_mut()
        .unwrap()
        .push(changes::entry(
            "2027-01-01",
            1,
            Some(("2026-04-01", &[3, 3, 3])),
        ));
    only(
        &d,
        ErrorCode::ExceptionsOverlap,
        "exceptions.history[1].transition.start",
    );

    // A year later the change falls on the first day of the previous rule's
    // second year: valid.
    let mut d = changes::siemens();
    d["exceptions"]["history"]
        .as_array_mut()
        .unwrap()
        .push(changes::entry(
            "2028-01-01",
            1,
            Some(("2027-04-01", &[3, 3, 3])),
        ));
    let result = Definition::from_json(&d.to_string());
    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
fn change_not_on_boundary_for_the_previous_and_the_own_rule() {
    // The transition starts on 1 August, which no July to June year does.
    let d = pg_with(|e| {
        e["transition"]["start"] = json!("2024-08-01");
        e["transition"]["subdivision"]["quarters"] = json!([3, 3, 2]);
    });
    let errors = errors_and_hints(&d);
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(
        (errors[0].0, errors[0].1.as_str()),
        (
            ErrorCode::ChangeNotOnBoundary,
            "exceptions.history[0].transition.start"
        )
    );
    // The hint names the year starts either side and picks neither.
    assert!(errors[0].2.contains("2024-07-01") && errors[0].2.contains("2025-07-01"));

    // from is 1 May, which no April to March year starts on.
    let d = pg_with(|e| {
        e["from"] = json!("2025-05-01");
        e["transition"]["subdivision"]["quarters"] = json!([3, 3, 3, 1]);
    });
    let errors = errors_and_hints(&d);
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(
        (errors[0].0, errors[0].1.as_str()),
        (ErrorCode::ChangeNotOnBoundary, "exceptions.history[0].from")
    );
    assert!(errors[0].2.contains("2025-04-01") && errors[0].2.contains("2026-04-01"));

    // Without a transition, a from on neither rule's boundary fails both
    // checks at the same path, the previous rule's first.
    let mut d = changes::fixed(7, "start", "FY{yyyy}");
    d["exceptions"] = json!({"history": [changes::entry("2025-03-01", 4, None)]});
    let errors = errors_and_hints(&d);
    assert_eq!(
        errors
            .iter()
            .map(|(c, p, _)| (*c, p.as_str()))
            .collect::<Vec<_>>(),
        [
            (ErrorCode::ChangeNotOnBoundary, "exceptions.history[0].from"),
            (ErrorCode::ChangeNotOnBoundary, "exceptions.history[0].from"),
        ]
    );
    // The first hint names the July years around 1 March 2025, the second
    // the April years.
    assert!(errors[0].2.contains("2024-07-01") && errors[0].2.contains("2025-07-01"));
    assert!(errors[1].2.contains("2024-04-01") && errors[1].2.contains("2025-04-01"));
}

#[test]
fn calendar_months_transition_must_fit_its_dates() {
    // The quarters check needs whole months, so it is not also reported.
    let d = pg_with(|e| e["transition"]["subdivision"]["month_start_day"] = json!(2));
    only(
        &d,
        ErrorCode::SubdivisionYearMismatch,
        "exceptions.history[0].transition.subdivision.month_start_day",
    );
    let d = pg_with(|e| e["transition"]["subdivision"]["quarters"] = json!([3, 3, 3, 3]));
    only(
        &d,
        ErrorCode::QuartersSumMismatch,
        "exceptions.history[0].transition.subdivision.quarters",
    );
}

#[test]
fn calendar_months_transition_needs_month_start_day_on_both_dates() {
    // July years starting on the 15th, then April years starting on the 1st,
    // through a transition from 15 July 2024 to 31 March 2025. Each of the
    // transition's dates is a year start of its rule, but they fall on
    // different days of the month, so no month_start_day fits both.
    let mut d = changes::fixed(7, "start", "FY{yyyy}");
    d["year"]["start"]["day"] = json!(15);
    d["subdivision"]["month_start_day"] = json!(15);
    let base = changes::with_history(
        d,
        vec![changes::entry(
            "2025-04-01",
            4,
            Some(("2024-07-15", &[3, 3, 3])),
        )],
    );
    let path = "exceptions.history[0].transition.subdivision.month_start_day";
    for day in [15, 1] {
        let mut d = base.clone();
        d["exceptions"]["history"][0]["transition"]["subdivision"]["month_start_day"] = json!(day);
        only(&d, ErrorCode::SubdivisionYearMismatch, path);
    }
}

#[test]
fn extra_week_errors_in_a_history_entry() {
    let path = "exceptions.history[0].subdivision.extra_week";
    let mut d = changes::retail_to_september();
    d["exceptions"]["history"][0]["subdivision"]
        .as_object_mut()
        .unwrap()
        .remove("extra_week");
    only(&d, ErrorCode::ExtraWeekMissing, path);
    for p in [0, 13] {
        let mut d = changes::retail_to_september();
        d["exceptions"]["history"][0]["subdivision"]["extra_week"]["period"] = json!(p);
        only(
            &d,
            ErrorCode::ExtraWeekOutOfRange,
            &format!("{path}.period"),
        );
    }
}

#[test]
fn week_pattern_transition_must_fit_its_dates() {
    let base = changes::retail_to_september();
    let t = "exceptions.history[0].transition";

    let mut d = base.clone();
    d["exceptions"]["history"][0]["transition"]["subdivision"]["periods"][7] = json!(5);
    only(
        &d,
        ErrorCode::WeeksSumMismatch,
        &format!("{t}.subdivision.periods"),
    );

    // Starts on a Monday, 237 days before from: one mismatch for both faults,
    // and no weeks check because the length is not whole weeks.
    let mut d = base;
    d["exceptions"]["history"][0]["transition"]["start"] = json!("2024-02-05");
    assert_eq!(
        errors_of(&d),
        [
            (ErrorCode::ChangeNotOnBoundary, format!("{t}.start")),
            (
                ErrorCode::SubdivisionYearMismatch,
                format!("{t}.subdivision.type")
            ),
        ]
    );

    // Whole weeks from the wrong weekday: the weeks are still counted.
    // 2025-01-01 is a Wednesday and 2025-04-02 is 13 weeks later.
    let mut d = changes::fixed(1, "start", "FY{yyyy}");
    d["exceptions"] = json!({"history": [{
        "from": "2025-04-02",
        "transition": {"start": "2025-01-01", "subdivision": {
            "type": "week_pattern", "periods": [4, 4, 4], "quarters": [3]}},
        "year": {"type": "fixed_date", "start": {"month": 4, "day": 2}},
        "subdivision": {"type": "calendar_months", "month_start_day": 2, "quarters": [3, 3, 3, 3]}
    }]});
    assert_eq!(
        errors_of(&d),
        [
            (
                ErrorCode::WeeksSumMismatch,
                format!("{t}.subdivision.periods")
            ),
            (
                ErrorCode::SubdivisionYearMismatch,
                format!("{t}.subdivision.type")
            ),
        ]
    );
}

#[test]
fn cross_block_checks_wait_for_every_block_and_numbering_waits_for_them() {
    // A block-level error anywhere hides the cross-block errors.
    let mut d = pg_with(|e| e["from"] = json!("2025-05-01"));
    d["naming"]["default"] = json!("missing");
    only(&d, ErrorCode::DefaultSchemeMissing, "naming.default");

    let mut d = changes::siemens();
    d["exceptions"]["history"]
        .as_array_mut()
        .unwrap()
        .push(changes::entry("2025-04-01", 4, None));
    d["exceptions"]["history"][0]["transition"]["start"] = json!("2026-04-01");
    only(
        &d,
        ErrorCode::InvalidValue,
        "exceptions.history[0].transition.start",
    );

    // A cross-block error hides numbering errors: ISO week years numbered by
    // "start" collide, but only the misplaced change is reported.
    let mut d = iso();
    d["naming"]["schemes"]["fy"]["year_number"] = json!("start");
    d["exceptions"] = json!({"history": [{
        "from": "2030-01-02",
        "year": d["year"].clone(),
        "subdivision": d["subdivision"].clone()
    }]});
    assert_eq!(
        errors_of(&d),
        [
            (
                ErrorCode::ChangeNotOnBoundary,
                "exceptions.history[0].from".into()
            ),
            (
                ErrorCode::ChangeNotOnBoundary,
                "exceptions.history[0].from".into()
            ),
        ]
    );
}

#[test]
fn numbers_may_skip_only_at_a_change() {
    // Siemens numbered by "start" goes 2023, 2024 (transition), 2026.
    assert!(Definition::from_json(&changes::siemens().to_string()).is_ok());

    // A three-month transition in calendar year 2025 after January to
    // December years: "start" gives 2024, 2025, 2025.
    let mut d = changes::fixed(1, "start", "FY{yyyy}");
    d["exceptions"] =
        json!({"history": [changes::entry("2025-04-01", 4, Some(("2025-01-01", &[3])))]});
    let errors = Definition::from_json(&d.to_string())
        .unwrap_err()
        .into_vec();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].code, ErrorCode::YearNumberCollision);
    assert_eq!(errors[0].path, "naming.schemes.fy.year_number");
    assert_eq!(errors[0].details, Some(Details::Years(vec![2025])));
    assert!(errors[0].hint.as_deref().unwrap().contains("\"end\""));

    // The offset is included in details.years.
    d["naming"]["schemes"]["fy"]["offset"] = json!(1);
    let errors = Definition::from_json(&d.to_string())
        .unwrap_err()
        .into_vec();
    assert_eq!(errors[0].details, Some(Details::Years(vec![2026])));
}

#[test]
fn majority_tie_covers_a_transition_without_a_majority_year() {
    // 1 January 2023 to 30 June 2025 is 365 + 366 + 181 days, so no calendar
    // year holds more than half of the 912.
    let mut d = changes::fixed(1, "majority", "FY{yyyy}");
    d["exceptions"] =
        json!({"history": [changes::entry("2025-07-01", 7, Some(("2023-01-01", &[3; 10])))]});
    let errors = Definition::from_json(&d.to_string())
        .unwrap_err()
        .into_vec();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].code, ErrorCode::MajorityTie);
    assert_eq!(errors[0].details, Some(Details::Years(vec![2023])));
}
