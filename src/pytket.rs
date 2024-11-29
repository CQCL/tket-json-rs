//! Python bindings for the serializable circuits.

use crate::circuit_json::SerialCircuit;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};

impl SerialCircuit {
    /// Create a new `SerialCircuit` from a `pytket.Circuit`.
    pub fn from_tket1(circ: &Bound<PyAny>) -> PyResult<Self> {
        let circ = depythonize(&circ.call_method0("to_dict").unwrap())?;
        Ok(circ)
    }

    /// Create a new `SerialCircuit` from a `pytket.Circuit`.
    ///
    /// Utility function that calls [`SerialCircuit::from_tket1`] after acquiring the GIL.
    pub fn from_tket1_with_gil(circ: Py<PyAny>) -> PyResult<Self> {
        Python::with_gil(|py| Self::from_tket1(circ.bind(py)))
    }

    /// Convert a `SerialCircuit` to a `pytket.Circuit`.
    pub fn to_tket1<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dict = pythonize(py, self).unwrap();
        let circ_module = PyModule::import(py, "pytket.circuit")?;

        circ_module
            .getattr("Circuit")?
            .call_method1("from_dict", (dict,))
    }

    /// Convert a `SerialCircuit` to a `pytket.Circuit`.
    ///
    /// Utility function that calls [`SerialCircuit::to_tket1`] after acquiring the GIL.
    pub fn to_tket1_with_gil(&self) -> PyResult<Py<PyAny>> {
        Python::with_gil(|py| self.to_tket1(py).map(|x| x.into()))
    }
}
