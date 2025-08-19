//! Roundtrip tests
use assert_json_diff::assert_json_eq;
use rstest::rstest;
use serde_json::Value;
use tket_json_rs::SerialCircuit;

const SIMPLE: &str = include_str!("data/simple.json");
const CLASSICAL: &str = include_str!("data/classical.json");
const DIAGONAL: &str = include_str!("data/diagonal-box.json");
const QASM: &str = include_str!("data/qasm.json");
const WASM: &str = include_str!("data/wasm.json");
const RNG: &str = include_str!("data/rng.json");

/// Cleanup some fields in the JSON so that we can compare them.
fn normalize_json(json: &mut Value) {
    if let Value::Object(obj) = json {
        // Some versions of pytket include the `created_qubits` and `discarded_qubits` fields
        // even if they are empty. Some other versions do not include them at all.
        //
        // We remove them here.
        if let Some(Value::Array(registers)) = obj.get_mut("created_qubits") {
            if registers.is_empty() {
                obj.remove("created_qubits");
            }
        }
        if let Some(Value::Array(registers)) = obj.get_mut("discarded_qubits") {
            if registers.is_empty() {
                obj.remove("discarded_qubits");
            }
        }
    }
}

#[rstest]
#[case::simple(SIMPLE, 4)]
#[case::classical(CLASSICAL, 3)]
#[case::diagonal_box(DIAGONAL, 1)]
#[case::qasm_box(QASM, 4)]
#[case::wasm_box(WASM, 1)]
#[case::rng(RNG, 8)]
fn roundtrip(#[case] json: &str, #[case] num_commands: usize) {
    let mut initial_json: Value = serde_json::from_str(json).unwrap();
    normalize_json(&mut initial_json);
    let ser: SerialCircuit = serde_json::from_value(initial_json.clone()).unwrap();

    assert_eq!(ser.commands.len(), num_commands);

    let mut reencoded_json = serde_json::to_value(&ser).unwrap();
    normalize_json(&mut reencoded_json);
    assert_json_eq!(reencoded_json, initial_json);

    let reser: SerialCircuit = serde_json::from_value(reencoded_json).unwrap();

    assert_eq!(ser, reser);
}
