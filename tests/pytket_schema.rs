#[cfg(feature = "schemars")]
mod schema_tests {
    use std::fs::File;

    use assert_json_diff::assert_json_include;
    use schemars::schema_for;
    use serde_json::Value;
    use tket_json_rs::SerialCircuit;

    /// Test how similar the schemars generated schema is to the handwritten schema.
    #[ignore = "there are known differences between the schemas."]
    #[test]
    fn schema_similarity() {
        let schema = schema_for!(SerialCircuit);
        let tket_schema_file = File::open("./tests/schemas/circuit_v1.json").unwrap();
        let tket_schema: Value = serde_json::from_reader(tket_schema_file).unwrap();

        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        assert_json_include!(actual: schema, expected: tket_schema);
    }
}
