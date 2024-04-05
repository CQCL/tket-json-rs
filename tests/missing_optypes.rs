//! Integration test to detect missing optypes in the optype module.
//!
//! Requires the `pyo3` feature to be enabled.

use std::str::FromStr;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tket_json_rs::OpType;

#[test]
#[ignore = "Requires a python environment with `pytket` installed."]
fn missing_optypes() -> PyResult<()> {
    println!("Checking missing optypes");

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let Ok(pytket) = PyModule::import_bound(py, "pytket") else {
            panic!("Failed to import `pytket`. Make sure the python library is installed.");
        };
        let py_enum = pytket.getattr("OpType")?;
        let py_members = py_enum.getattr("__members__")?;
        let py_members = py_members.downcast::<PyDict>()?;

        let missing: Vec<String> = py_members
            .into_iter()
            .filter_map(|(name, _class)| {
                let name = name.extract::<String>().unwrap();
                match OpType::from_str(&name) {
                    Err(_) => Some(name),
                    Ok(_) => None,
                }
            })
            .collect();

        if !missing.is_empty() {
            let msg = "\nMissing optypes in `tket_json_rs`:\n".to_string();
            let msg = missing
                .into_iter()
                .fold(msg, |msg, s| msg + "  - " + &s + "\n");
            let msg =
                msg + "Please add them to the `OpType` enum in `tket_json_rs/src/optype.rs`.\n";
            panic!("{msg}");
        }

        Ok(())
    })
}
