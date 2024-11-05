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

#[rstest]
#[case::simple(SIMPLE, 4)]
#[case::classical(CLASSICAL, 3)]
#[case::diagonal_box(DIAGONAL, 1)]
#[case::qasm_box(QASM, 4)]
#[case::wasm_box(WASM, 1)]
fn roundtrip(#[case] json: &str, #[case] num_commands: usize) {
    let initial_json: Value = serde_json::from_str(json).unwrap();
    let ser: SerialCircuit = serde_json::from_value(initial_json.clone()).unwrap();

    assert_eq!(ser.commands.len(), num_commands);

    let reencoded_json = serde_json::to_value(&ser).unwrap();
    assert_json_eq!(reencoded_json, initial_json);

    let reser: SerialCircuit = serde_json::from_value(reencoded_json).unwrap();

    assert_eq!(ser, reser);
}
