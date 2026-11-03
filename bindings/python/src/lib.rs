//! The compiled half of the `fiscalclock` Python package.
//!
//! It passes JSON text between Python and the Rust core and nothing else;
//! `python/fiscalclock/__init__.py` converts Python values to and from that
//! JSON. Keeping all logic in the core is what makes every language return
//! byte-identical results.

#![forbid(unsafe_code)]

use fiscalclock::{FiscalCalendar, json};
use pyo3::prelude::*;

/// `fiscalclock::json::validate`.
#[pyfunction]
fn validate(py: Python<'_>, definition: &str) -> String {
    py.detach(|| json::validate(definition))
}

/// `fiscalclock::json::call`.
#[pyfunction]
fn call(py: Python<'_>, definition: &str, op: &str, args: &str) -> String {
    py.detach(|| json::call(definition, op, args))
}

/// `fiscalclock::json::timestamp_rejected`.
#[pyfunction]
fn timestamp_rejected(path: &str) -> String {
    json::timestamp_rejected(path)
}

/// A validated calendar, so repeated calls skip validation.
#[pyclass(frozen, module = "fiscalclock._fiscalclock")]
struct Calendar(FiscalCalendar);

#[pymethods]
impl Calendar {
    /// Returns `(calendar, None)`, or `(None, errors)` with the core's errors JSON.
    #[staticmethod]
    fn build(py: Python<'_>, definition: &str) -> (Option<Calendar>, Option<String>) {
        match py.detach(|| FiscalCalendar::from_json(definition)) {
            Ok(cal) => (Some(Calendar(cal)), None),
            Err(errors) => (None, Some(json::errors_json(errors.as_slice()))),
        }
    }

    fn call_json(&self, py: Python<'_>, op: &str, args: &str) -> String {
        py.detach(|| self.0.call_json(op, args))
    }

    #[getter]
    fn calendar_id(&self) -> &str {
        self.0.calendar_id()
    }

    #[getter]
    fn canonical_json(&self) -> &str {
        self.0.canonical_json()
    }
}

#[pymodule(gil_used = false)]
fn _fiscalclock(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(call, m)?)?;
    m.add_function(wrap_pyfunction!(timestamp_rejected, m)?)?;
    m.add_class::<Calendar>()?;
    m.add("__core_version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
