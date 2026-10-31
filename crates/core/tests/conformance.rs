//! Runs every shared conformance vector under `conformance/vectors/` through
//! the JSON API that the bindings wrap.

use std::fs;
use std::path::{Path, PathBuf};

use fiscalclock::json;
use serde_json::Value;

/// Vectors this crate believes are wrong against the current spec. Each entry
/// must still fail, so the list cannot go stale once a vector is regenerated.
const DISPUTED: &[(&str, &str)] = &[];

fn vector_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            vector_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

fn subset(want: &Value, got: &Value) -> bool {
    match (want, got) {
        (Value::Object(w), Value::Object(g)) => w
            .iter()
            .all(|(k, v)| g.get(k).is_some_and(|gv| subset(v, gv))),
        _ => want == got,
    }
}

fn error_matches(want: &Value, got: &Value) -> bool {
    want["code"] == got["code"]
        && want["path"] == got["path"]
        && want.get("details").is_none_or(|d| *d == got["details"])
}

fn check_case(definition: &str, case: &Value) -> Result<(), String> {
    let op = case["op"].as_str().unwrap();
    let out = if op == "validate" {
        json::validate(definition)
    } else {
        json::call(definition, op, &case["args"].to_string())
    };
    let got: Value = serde_json::from_str(&out).map_err(|e| format!("output is not JSON: {e}"))?;
    let ok = if let Some(want) = case.get("expect") {
        *want == got
    } else if let Some(want) = case.get("expect_subset") {
        subset(want, &got)
    } else {
        let errors = got["errors"].as_array().cloned().unwrap_or_default();
        match &case["expect_error"] {
            Value::Array(want) => {
                want.len() == errors.len()
                    && want.iter().zip(&errors).all(|(w, g)| error_matches(w, g))
            }
            want => errors.first().is_some_and(|g| error_matches(want, g)),
        }
    };
    if ok {
        Ok(())
    } else {
        let mut shown = out;
        shown.truncate(600);
        Err(format!("{op} {} -> {shown}", case["args"]))
    }
}

#[test]
fn every_conformance_vector_passes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/vectors");
    let mut files = Vec::new();
    vector_files(&root, &mut files);
    files.sort();
    assert!(!files.is_empty(), "no vectors under {}", root.display());
    let (mut cases, mut failures, mut disputed_failing) = (0, Vec::new(), Vec::new());
    for file in &files {
        let rel = file
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let vector: Value = serde_json::from_str(&fs::read_to_string(file).unwrap()).unwrap();
        let definition = vector["definition"].to_string();
        let mut file_failures = Vec::new();
        for case in vector["cases"].as_array().unwrap() {
            cases += 1;
            if let Err(e) = check_case(&definition, case) {
                file_failures.push(e);
            }
        }
        let disputed = DISPUTED.iter().any(|(f, _)| *f == rel);
        match (disputed, file_failures.is_empty()) {
            (true, false) => disputed_failing.push(rel),
            (true, true) => failures.push(format!(
                "{rel}: listed as disputed but now passes; remove it from DISPUTED"
            )),
            (false, false) => failures.push(format!(
                "{rel}: {} failing case(s), first: {}",
                file_failures.len(),
                file_failures[0]
            )),
            (false, true) => {}
        }
    }
    eprintln!(
        "conformance: {} files, {cases} cases; disputed and still failing: {disputed_failing:?}",
        files.len()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
