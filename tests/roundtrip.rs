//! Roundtrip tests
use assert_json_diff::assert_json_include;
use rstest::rstest;
use serde_json::Value;
use tket_json_rs::SerialCircuit;

const SIMPLE: &str = include_str!("data/simple.json");
const DIAGONAL: &str = include_str!("data/diagonal-box.json");

#[rstest]
#[case::simple(SIMPLE, 4)]
#[case::diagonal_box(DIAGONAL, 1)]
fn roundtrip(#[case] json: &str, #[case] num_commands: usize) {
    let initial_json: Value = serde_json::from_str(json).unwrap();
    let ser: SerialCircuit = serde_json::from_value(initial_json.clone()).unwrap();

    assert_eq!(ser.commands.len(), num_commands);

    let reencoded_json = serde_json::to_value(&ser).unwrap();
    // Do a partial comparison. The re-encoded circuit does not include "created_qubits" nor "discarded_qubits".
    assert_json_include!(actual: initial_json, expected: reencoded_json);

    let reser: SerialCircuit = serde_json::from_value(reencoded_json).unwrap();

    assert_eq!(ser, reser);
}
