#![warn(missing_docs)]
//! Serializable Rust definition for circuits and operations of the
//! [TKET](https://github.com/CQCL/tket) quantum compiler.

pub mod circuit_json;
pub mod opbox;
pub mod optype;
#[cfg(feature = "pyo3")]
pub mod pytket;

pub use circuit_json::SerialCircuit;
pub use optype::OpType;

#[cfg(test)]
mod tests {
    use crate::circuit_json::SerialCircuit;
    use serde_json::to_string;
    #[test]
    fn read_json() {
        let circ_s = r#"{"bits": [["c", [0]], ["c", [1]]], "commands": [{"args": [["q", [0]]], "op": {"type": "H"}}, {"args": [["q", [0]], ["q", [1]]], "op": {"type": "CX"}}, {"args": [["q", [0]], ["c", [0]]], "op": {"type": "Measure"}}, {"args": [["q", [1]], ["c", [1]]], "op": {"type": "Measure"}}], "implicit_permutation": [[["q", [0]], ["q", [0]]], [["q", [1]], ["q", [1]]]], "phase": "0.0", "qubits": [["q", [0]], ["q", [1]]]}"#;
        let ser: SerialCircuit = serde_json::from_str(circ_s).unwrap();

        assert_eq!(ser.commands.len(), 4);

        let reser: SerialCircuit = serde_json::from_str(&to_string(&ser).unwrap()[..]).unwrap();
        assert_eq!(ser, reser);
    }
}
