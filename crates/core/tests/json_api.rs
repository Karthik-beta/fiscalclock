mod common;

use common::*;
use fiscalclock::{FiscalCalendar, json};
use serde_json::{Value, json};

fn call(def: &Value, op: &str, args: Value) -> String {
    json::call(&def.to_string(), op, &args.to_string())
}

fn err(def: &Value, op: &str, args: Value) -> (String, String) {
    let out: Value = serde_json::from_str(&call(def, op, args.clone())).unwrap();
    let errors = out["errors"]
        .as_array()
        .unwrap_or_else(|| panic!("{op} {args} succeeded: {out}"));
    assert_eq!(errors.len(), 1, "{out}");
    (
        errors[0]["code"].as_str().unwrap().to_string(),
        errors[0]["path"].as_str().unwrap().to_string(),
    )
}

fn assert_err(def: &Value, op: &str, args: Value, code: &str, path: &str) {
    assert_eq!(
        err(def, op, args.clone()),
        (code.to_string(), path.to_string()),
        "{op} {args}"
    );
}

#[test]
fn period_output_matches_the_api_example_byte_for_byte() {
    assert_eq!(
        call(
            &nrf(),
            "period",
            json!({"level": "quarter", "year": 2023, "index": 4})
        ),
        r#"{"level":"quarter","year":2023,"index":4,"start":"2023-10-29","end_exclusive":"2024-02-04","last_day":"2024-02-03","days":98,"weeks":14,"has_extra_week":true}"#
    );
}

#[test]
fn location_keys_are_in_documented_order() {
    let out = call(&nrf(), "locate", json!({"date": "2024-02-03"}));
    let tail = r#","day_of_year":371,"day_of_quarter":98,"day_of_period":35,"day_of_week":7}"#;
    assert!(
        out.starts_with(r#"{"date":"2024-02-03","year":{"level":"year","year":2023,"index":null,"#)
    );
    assert!(out.ends_with(tail), "{out}");
    let keys: Vec<usize> = ["\"year\":{", "\"quarter\":{", "\"period\":{", "\"week\":{"]
        .iter()
        .map(|k| out.find(k).unwrap())
        .collect();
    assert!(keys.windows(2).all(|w| w[0] < w[1]));
    assert!(!out.contains(' '));
}

#[test]
fn validate_returns_id_and_canonical_text() {
    let out: Value = serde_json::from_str(&json::validate(&nrf().to_string())).unwrap();
    let c = cal(&nrf());
    assert_eq!(out["ok"], json!(true));
    assert_eq!(out["calendar_id"], json!(c.calendar_id()));
    assert_eq!(out["canonical"], json!(c.canonical_json()));
    assert_eq!(
        call(&nrf(), "validate", json!({})),
        json::validate(&nrf().to_string())
    );
}

#[test]
fn call_json_equals_call() {
    let c = cal(&nrf());
    let args = json!({"date": "2023-06-15"}).to_string();
    assert_eq!(
        c.call_json("locate", &args),
        json::call(&nrf().to_string(), "locate", &args)
    );
}

#[test]
fn invalid_definition_surfaces_through_call() {
    let mut d = nrf();
    d["week"]["starts_on"] = json!("caturday");
    assert_err(
        &d,
        "locate",
        json!({"date": "2024-01-01"}),
        "INVALID_VALUE",
        "week.starts_on",
    );
}

#[test]
fn invalid_date() {
    for bad in [
        "2023-02-29",
        "2023-2-01",
        "2023-01-05T00:00:00",
        "2023-01-05Z",
        "20230105",
        "0000-06-15",
        " 2023-01-05",
    ] {
        assert_err(
            &nrf(),
            "locate",
            json!({"date": bad}),
            "INVALID_DATE",
            "date",
        );
    }
    assert_err(
        &nrf(),
        "dimension",
        json!({"start": "2024-01-01", "end_exclusive": "2024-13-01"}),
        "INVALID_DATE",
        "end_exclusive",
    );
}

#[test]
fn invalid_argument() {
    let n = nrf();
    assert_err(&n, "locate", json!({}), "INVALID_ARGUMENT", "date");
    assert_err(
        &n,
        "locate",
        json!({"date": 20240101}),
        "INVALID_ARGUMENT",
        "date",
    );
    assert_err(&n, "nonsense", json!({}), "INVALID_ARGUMENT", "op");
    assert_eq!(
        err_text(&n, "locate", "[1]"),
        ("INVALID_ARGUMENT".into(), String::new())
    );
    assert_eq!(
        err_text(&n, "locate", "{"),
        ("INVALID_ARGUMENT".into(), String::new())
    );
    assert_err(
        &n,
        "period",
        json!({"level": "month", "year": 2023, "index": 1}),
        "INVALID_ARGUMENT",
        "level",
    );
    assert_err(
        &n,
        "period",
        json!({"level": "quarter", "year": 2023}),
        "INVALID_ARGUMENT",
        "index",
    );
    assert_err(
        &n,
        "period",
        json!({"level": "year", "year": 2023, "index": 1}),
        "INVALID_ARGUMENT",
        "index",
    );
    assert_err(
        &n,
        "period",
        json!({"level": "year", "year": "2023"}),
        "INVALID_ARGUMENT",
        "year",
    );
    assert_err(
        &n,
        "shift",
        json!({"period": {"level": "year", "year": 2023}, "n": 1.5}),
        "INVALID_ARGUMENT",
        "n",
    );
    assert_err(
        &n,
        "shift",
        json!({"period": {"level": "year", "year": 2023}}),
        "INVALID_ARGUMENT",
        "n",
    );
    assert_err(
        &n,
        "parent",
        json!({"period": {"level": "year", "year": 2023}}),
        "INVALID_ARGUMENT",
        "period",
    );
    assert_err(
        &n,
        "parent",
        json!({"period": "FY2023"}),
        "INVALID_ARGUMENT",
        "period",
    );
    assert_err(
        &n,
        "parent",
        json!({"period": {"level": "decade", "year": 2023}}),
        "INVALID_ARGUMENT",
        "period",
    );
    assert_err(
        &n,
        "years",
        json!({"from": 2025, "to": 2024}),
        "INVALID_ARGUMENT",
        "to",
    );
    assert_err(
        &n,
        "years",
        json!({"from": 1000, "to": 11000}),
        "INVALID_ARGUMENT",
        "to",
    );
    assert_err(
        &n,
        "format",
        json!({"period": {"level": "year", "year": 2023}, "scheme": 7}),
        "INVALID_ARGUMENT",
        "scheme",
    );
    assert_err(
        &n,
        "parse",
        json!({"label": 2023}),
        "INVALID_ARGUMENT",
        "label",
    );
    assert_err(
        &n,
        "dimension",
        json!({"start": "2024-01-02", "end_exclusive": "2024-01-01"}),
        "INVALID_ARGUMENT",
        "end_exclusive",
    );
    assert_err(
        &n,
        "dimension",
        json!({"start": "2000-01-01", "end_exclusive": "2400-01-02"}),
        "INVALID_ARGUMENT",
        "end_exclusive",
    );
}

fn err_text(def: &Value, op: &str, args: &str) -> (String, String) {
    let out: Value = serde_json::from_str(&json::call(&def.to_string(), op, args)).unwrap();
    (
        out["errors"][0]["code"].as_str().unwrap().into(),
        out["errors"][0]["path"].as_str().unwrap().into(),
    )
}

#[test]
fn out_of_range() {
    let n = nrf();
    assert_err(
        &n,
        "locate",
        json!({"date": "9999-12-31"}),
        "OUT_OF_RANGE",
        "date",
    );
    assert_err(
        &n,
        "locate",
        json!({"date": "0001-01-01"}),
        "OUT_OF_RANGE",
        "date",
    );
    assert_err(
        &n,
        "period",
        json!({"level": "year", "year": 10000}),
        "OUT_OF_RANGE",
        "year",
    );
    assert_err(
        &n,
        "period",
        json!({"level": "year", "year": 99999999999i64}),
        "OUT_OF_RANGE",
        "year",
    );
    assert_err(
        &n,
        "shift",
        json!({"period": {"level": "year", "year": 2023}, "n": 9000}),
        "OUT_OF_RANGE",
        "n",
    );
    assert_err(
        &n,
        "shift",
        json!({"period": {"level": "week", "year": 2023, "index": 1}, "n": i64::MAX}),
        "OUT_OF_RANGE",
        "n",
    );
    assert_err(
        &n,
        "shift",
        json!({"period": {"level": "quarter", "year": 2023, "index": 1}, "n": -9_000_000}),
        "OUT_OF_RANGE",
        "n",
    );
    assert_err(
        &n,
        "shift",
        json!({"period": {"level": "year", "year": 20000}, "n": 1}),
        "OUT_OF_RANGE",
        "period",
    );
    assert_err(
        &n,
        "years",
        json!({"from": 0, "to": 5}),
        "OUT_OF_RANGE",
        "from",
    );
    assert_err(
        &n,
        "years",
        json!({"from": 9990, "to": 9999}),
        "OUT_OF_RANGE",
        "to",
    );
    assert_err(
        &n,
        "parse",
        json!({"label": "FY0000"}),
        "OUT_OF_RANGE",
        "label",
    );
    assert_err(
        &n,
        "dimension",
        json!({"start": "0001-01-01", "end_exclusive": "0001-03-01"}),
        "OUT_OF_RANGE",
        "start",
    );
    assert_err(
        &n,
        "dimension_csv",
        json!({"start": "9998-06-01", "end_exclusive": "9999-12-31"}),
        "OUT_OF_RANGE",
        "end_exclusive",
    );
    let mut be = fixed_date(1, 1, None, "monday", &[3, 3, 3, 3], "start");
    be["naming"]["schemes"]["be"] =
        json!({"year_number": "start", "offset": 9000, "templates": {"year": "BE {yyyy}"}});
    assert_err(
        &be,
        "format",
        json!({"period": {"level": "year", "year": 2000}, "scheme": "be"}),
        "OUT_OF_RANGE",
        "period",
    );
    for n in [i64::MIN, i64::MAX] {
        assert_err(
            &be,
            "shift",
            json!({"period": {"level": "week", "year": 2000, "index": 1}, "n": n}),
            "OUT_OF_RANGE",
            "n",
        );
    }
}

#[test]
fn period_not_found() {
    let n = nrf();
    assert_err(
        &n,
        "period",
        json!({"level": "quarter", "year": 2023, "index": 5}),
        "PERIOD_NOT_FOUND",
        "index",
    );
    assert_err(
        &n,
        "period",
        json!({"level": "week", "year": 2024, "index": 53}),
        "PERIOD_NOT_FOUND",
        "index",
    );
    assert_err(
        &n,
        "period",
        json!({"level": "period", "year": 2023, "index": 0}),
        "PERIOD_NOT_FOUND",
        "index",
    );
    assert_err(
        &n,
        "period",
        json!({"level": "period", "year": 2023, "index": -3}),
        "PERIOD_NOT_FOUND",
        "index",
    );
    assert_err(
        &n,
        "shift",
        json!({"period": {"level": "week", "year": 2024, "index": 53}, "n": 1}),
        "PERIOD_NOT_FOUND",
        "period",
    );
}

#[test]
fn level_not_nested() {
    let f = fixed_date(4, 1, None, "monday", &[3, 3, 3, 3], "start");
    assert_err(
        &f,
        "children",
        json!({"period": {"level": "quarter", "year": 2023, "index": 1}, "level": "week"}),
        "LEVEL_NOT_NESTED",
        "level",
    );
    assert_err(
        &f,
        "children",
        json!({"period": {"level": "period", "year": 2023, "index": 1}, "level": "week"}),
        "LEVEL_NOT_NESTED",
        "level",
    );
    assert_err(
        &nrf(),
        "children",
        json!({"period": {"level": "period", "year": 2023, "index": 1}, "level": "quarter"}),
        "LEVEL_NOT_NESTED",
        "level",
    );
    assert_err(
        &nrf(),
        "children",
        json!({"period": {"level": "week", "year": 2023, "index": 1}, "level": "week"}),
        "LEVEL_NOT_NESTED",
        "level",
    );
}

#[test]
fn scheme_not_found_and_template_missing() {
    let mut d = nrf();
    d["naming"]["schemes"]["fy"]["templates"] = json!({"year": "FY{yyyy}"});
    assert_err(
        &d,
        "format",
        json!({"period": {"level": "year", "year": 2023}, "scheme": "nope"}),
        "SCHEME_NOT_FOUND",
        "scheme",
    );
    assert_err(
        &d,
        "format",
        json!({"period": {"level": "week", "year": 2023, "index": 2}}),
        "TEMPLATE_MISSING",
        "period",
    );
    assert_eq!(
        call(
            &d,
            "format",
            json!({"period": {"level": "year", "year": 2023}, "scheme": null})
        ),
        r#"{"label":"FY2023","scheme":"fy"}"#
    );
}

#[test]
fn labels_not_recognised() {
    let n = nrf();
    for label in [
        "FY 2023",
        "fy2023",
        "FY2023 ",
        "Q01 FY2023",
        "Q5 FY2023",
        "W54 FY2023",
        "FY23",
    ] {
        assert_err(
            &n,
            "parse",
            json!({"label": label}),
            "LABEL_NOT_RECOGNISED",
            "label",
        );
    }
}

#[test]
fn ambiguous_label_lists_candidates_in_order() {
    let mut d = fixed_date(4, 1, None, "monday", &[3, 3, 3, 3], "start");
    d["naming"]["schemes"] = json!({
        "fy": {"year_number": "start", "templates": {"year": "{yyyy}-{yy_next}"}},
        "ay": {"year_number": "start", "offset": 1, "templates": {"year": "{yyyy}-{yy_next}"}}
    });
    let out: Value = serde_json::from_str(&call(&d, "parse", json!({"label": "2025-26"}))).unwrap();
    let e = &out["errors"][0];
    assert_eq!(e["code"], "AMBIGUOUS_LABEL");
    let cands = e["details"]["candidates"].as_array().unwrap();
    let summary: Vec<(i64, &str, &str)> = cands
        .iter()
        .map(|c| {
            (
                c["period"]["year"].as_i64().unwrap(),
                c["scheme"].as_str().unwrap(),
                c["level"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(summary, [(2024, "ay", "year"), (2025, "fy", "year")]);
    let text = call(&d, "parse", json!({"label": "2025-26"}));
    assert!(text.contains(r#""details":{"candidates":[{"period":{"level":"year","year":2024,"#));
}

#[test]
fn two_digit_years_resolve_through_the_pivot() {
    let mut d = fixed_date(4, 1, None, "monday", &[3, 3, 3, 3], "start");
    d["naming"]["schemes"]["fy"] = json!({"year_number": "start", "century_pivot": 2069, "templates": {"year": "FY{yy}-{yy_next}", "quarter": "Q{q}/{yy}"}});
    let c = cal(&d);
    assert_eq!(c.parse("FY69-70").unwrap().year, 2069);
    assert_eq!(c.parse("FY70-71").unwrap().year, 1970);
    assert_eq!(c.parse("FY99-00").unwrap().year, 1999);
    assert!(c.parse("FY70-72").is_err());
    let q = c.parse("Q12/23").unwrap_err();
    assert_eq!(q.code, fiscalclock::ErrorCode::LabelNotRecognised);
    assert_eq!(c.parse("Q3/23").unwrap().index, Some(3));
}

#[test]
fn dimension_rows_and_csv() {
    let mut d = nrf();
    d["naming"]["schemes"]["alt"] = json!({"year_number": "start", "templates": {"year": "Retail, {yyyy}", "week": "\"W\"{ww}-{yyyy}"}});
    let c = cal(&d);
    let csv = c.call_json(
        "dimension_csv",
        &json!({"start": "2024-02-03", "end_exclusive": "2024-02-05"}).to_string(),
    );
    let text: String = serde_json::from_str(&csv).unwrap();
    let lines: Vec<&str> = text.split('\n').collect();
    assert_eq!(
        lines[0],
        "date,calendar_id,fiscal_year,quarter,period,week,day_of_year,day_of_quarter,day_of_period,day_of_week,year_start,year_last_day,quarter_start,quarter_last_day,period_start,period_last_day,week_start,week_last_day,year_has_extra_week,period_has_extra_week,week_is_extra_week,label_alt_year,label_alt_week,label_fy_year,label_fy_quarter,label_fy_period,label_fy_week"
    );
    assert_eq!(
        lines[1],
        format!(
            "2024-02-03,{},2023,4,12,53,371,98,35,7,2023-01-29,2024-02-03,2023-10-29,2024-02-03,2023-12-31,2024-02-03,2024-01-28,2024-02-03,true,true,true,\"Retail, 2023\",\"\"\"W\"\"53-2023\",FY2023,Q4 FY2023,P12 FY2023,W53 FY2023",
            c.calendar_id()
        )
    );
    assert!(lines[2].starts_with("2024-02-04,"));
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[3], "");

    let rows: Value = serde_json::from_str(&c.call_json(
        "dimension",
        &json!({"start": "2024-02-03", "end_exclusive": "2024-02-05"}).to_string(),
    ))
    .unwrap();
    assert_eq!(rows.as_array().unwrap().len(), 2);
    assert_eq!(rows[1]["fiscal_year"], 2024);
    assert_eq!(rows[1]["label_alt_week"], "\"W\"01-2024");
    assert_eq!(
        c.call_json(
            "dimension",
            &json!({"start": "2024-02-03", "end_exclusive": "2024-02-03"}).to_string()
        ),
        "[]"
    );
}

#[test]
fn explain_has_one_step_per_level() {
    let c = cal(&nrf());
    let out: Value =
        serde_json::from_str(&c.call_json("explain", r#"{"date":"2024-02-03"}"#)).unwrap();
    assert_eq!(out["date"], "2024-02-03");
    assert_eq!(out["calendar_id"], c.calendar_id());
    let steps = out["steps"].as_array().unwrap();
    let levels: Vec<&str> = steps.iter().map(|s| s["level"].as_str().unwrap()).collect();
    assert_eq!(levels, ["year", "quarter", "period", "week"]);
    for step in steps {
        assert_eq!(step.as_object().unwrap().len(), 3, "{step}");
        for key in ["rule", "result"] {
            assert!(!step[key].as_str().unwrap().is_empty(), "{step}");
        }
    }
    let text = out.to_string();
    assert!(
        text.contains("Saturday nearest the last day of January"),
        "{text}"
    );
    assert!(text.contains("2023-01-29 to 2024-02-03"), "{text}");
}

#[test]
fn any_input_returns_json_and_never_panics() {
    let defs = [
        nrf().to_string(),
        fixed_date(4, 1, None, "monday", &[3, 3, 3, 3], "start").to_string(),
        "{".into(),
        "[]".into(),
        "null".into(),
    ];
    let ops = [
        "validate",
        "locate",
        "period",
        "shift",
        "parent",
        "children",
        "years",
        "format",
        "parse",
        "dimension",
        "dimension_csv",
        "explain",
        "",
    ];
    let args = [
        "{}",
        "",
        "null",
        r#"{"date":"2024-02-29","level":"week","year":2024,"index":1,"period":{"level":"week","year":2024,"index":1},"n":-5,"from":2020,"to":2021,"label":"W01 FY2024","start":"2024-01-01","end_exclusive":"2024-01-08","scheme":"fy"}"#,
        r#"{"period":{"level":"year","year":-2147483648},"n":-9223372036854775808,"date":"9999-12-31"}"#,
        r#"{"period":{"level":"week","year":2024,"index":1},"n":-9223372036854775808}"#,
    ];
    for d in &defs {
        for op in ops {
            for a in args {
                let out = json::call(d, op, a);
                let v: Value = serde_json::from_str(&out).unwrap();
                assert!(v.is_object() || v.is_array() || v.is_string(), "{op} {a}");
            }
        }
    }
    assert!(FiscalCalendar::from_json(&defs[0]).is_ok());
}

#[test]
fn duplicate_keys_are_rejected_not_resolved() {
    let text = nrf().to_string();
    let doubled = text.replacen("{", r#"{"spec_version":1,"#, 1);
    let out: Value = serde_json::from_str(&json::validate(&doubled)).unwrap();
    assert_eq!(out["errors"][0]["code"], "INVALID_JSON", "{out}");
    assert_eq!(out["errors"][0]["path"], "", "{out}");

    let out: Value = serde_json::from_str(&json::call(
        &text,
        "locate",
        r#"{"date":"2024-02-03","date":"2024-02-04"}"#,
    ))
    .unwrap();
    assert_eq!(out["errors"][0]["code"], "INVALID_ARGUMENT", "{out}");
    assert_eq!(out["errors"][0]["path"], "", "{out}");

    let nested = text.replacen(r#""week":{"#, r#""week":{"starts_on":"monday","#, 1);
    assert_ne!(nested, text);
    let out: Value = serde_json::from_str(&json::validate(&nested)).unwrap();
    assert_eq!(out["errors"][0]["code"], "INVALID_JSON", "{out}");
}
