use crate::circuit_json::SerialCircuit;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};

// #[pymethods]
impl SerialCircuit {
    pub fn _from_tket1(c: Py<PyAny>) -> Self {
        Python::with_gil(|py| depythonize(c.call_method0(py, "to_dict").unwrap().as_ref(py)))
            .unwrap()
    }

    pub fn to_tket1(&self) -> PyResult<Py<PyAny>> {
        Python::with_gil(|py| {
            let dict = pythonize(py, self).unwrap();
            let circ_module = PyModule::import(py, "pytket.circuit")?;

            Ok(circ_module
                .getattr("Circuit")?
                .call_method1("from_dict", (dict,))?
                .into())
        })
    }
}
