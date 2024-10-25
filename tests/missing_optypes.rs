//! Integration test to detect missing optypes in the optype module.
//!
//! The `pyo3` dependency is included as a dev-dependency in the `Cargo.toml` file.
//! The `pyo3` feature enables the `extension-module` feature in `pyo3`, which is useful
//! for distributing rust crates with pyo3 bindings without having it link to libpython,
//! but prevents us from running these tests.
//! Hence why this tests are not compiled when the `pyo3` feature is enabled.
#![cfg(not(feature = "pyo3"))]

use std::str::FromStr;

use itertools::Itertools;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tket_json_rs::clexpr::op::ClOp;
use tket_json_rs::OpType;

/// Given a python enum, lists the enum variants that cannot be converted into a `T` using `FromStr`.
fn find_missing_variants<'py, T>(py_enum: &Bound<'py, PyAny>) -> impl Iterator<Item = String> + 'py
where
    T: FromStr,
{
    let py_members = py_enum.getattr("__members__").unwrap();
    let py_members = py_members.downcast::<PyDict>().unwrap();

    py_members.into_iter().filter_map(|(name, _class)| {
        let name = name.extract::<String>().unwrap();
        match T::from_str(&name) {
            Err(_) => Some(name),
            Ok(_) => None,
        }
    })
}

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
        let missing = find_missing_variants::<OpType>(&py_enum).collect_vec();

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

#[test]
#[ignore = "Requires a python environment with `pytket` installed."]
fn missing_classical_optypes() -> PyResult<()> {
    println!("Checking missing classical ops");

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let Ok(pytket) = PyModule::import_bound(py, "pytket") else {
            panic!("Failed to import `pytket`. Make sure the python library is installed.");
        };
        let py_enum = pytket.getattr("circuit")?.getattr("ClOp")?;
        let missing = find_missing_variants::<ClOp>(&py_enum).collect_vec();

        if !missing.is_empty() {
            let msg = "\nMissing classical ops in `tket_json_rs`:\n".to_string();
            let msg = missing
                .into_iter()
                .fold(msg, |msg, s| msg + "  - " + &s + "\n");
            let msg =
                msg + "Please add them to the `ClOp` enum in `tket_json_rs/src/clexpr/op.rs`.\n";
            panic!("{msg}");
        }

        Ok(())
    })
}
