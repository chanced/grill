use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_unknown_keyword(interrogator)
    }
    interrogator
}
mod id_inside_an_unknown_keyword_is_not_a_real_identifier_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "id_in_unknown0": {
                    "not": {
                        "array_of_schemas": [
                            {
                              "$id": "https://localhost:1234/draft2020-12/unknownKeyword/my_identifier.json",
                              "type": "null"
                            }
                        ]
                    }
                },
                "real_id_in_schema": {
                    "$id": "https://localhost:1234/draft2020-12/unknownKeyword/my_identifier.json",
                    "type": "string"
                },
                "id_in_unknown1": {
                    "not": {
                        "object_of_schemas": {
                            "foo": {
                              "$id": "https://localhost:1234/draft2020-12/unknownKeyword/my_identifier.json",
                              "type": "integer"
                            }
                        }
                    }
                }
            },
            "anyOf": [
                { "$ref": "#/$defs/id_in_unknown0" },
                { "$ref": "#/$defs/id_in_unknown1" },
                { "$ref": "https://localhost:1234/draft2020-12/unknownKeyword/my_identifier.json" }
            ]
        }"##;
    const URI: &str = "http://localhost:1234/unknownKeyword.json";
    const DESCRIPTION: &str = "$id inside an unknown keyword is not a real identifier";
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        static INTERROGATOR: OnceLock<Result<(Key, Interrogator), CompileError>> = OnceLock::new();
        INTERROGATOR
            .get_or_init(|| {
                let mut interrogator = super::interrogator()
                    .map_err(|err| panic!("failed to build interrogator:\n{}", err))
                    .unwrap();
                interrogator
                    .source_str(URI, SCHEMA)
                    .map_err(|err| panic!("failed to source schema:\n: {err}"))
                    .unwrap();
                let key =
                    block_on(interrogator.compile("http://localhost:1234/unknownKeyword.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_type_matches_second_any_of_which_has_a_real_schema_in_it() {
        let description = "type matches second anyOf, which has a real schema in it";
        let data = "\"a string\"";
        let expected_valid = true;
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = match serde_json::from_str(data) {
            Ok(data) => data,
            Err(err) => {
                panic!("failed to parse data as json:\n{}", err);
            }
        };
        let output = match interrogator.evaluate(Structure::Flag, key, &data) {
            Ok(output) => output,
            Err(err) => {
                panic!("failed to evaluate schema:\n{}", err);
            }
        };
        let valid_msg = if expected_valid { "valid" } else { "invalid" };
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  expected: {valid_msg}\n  schema:{SCHEMA}\n  data: {data}")
    }
    #[test]
    fn test1_type_matches_non_schema_in_first_any_of() {
        let description = "type matches non-schema in first anyOf";
        let data = "null";
        let expected_valid = false;
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = match serde_json::from_str(data) {
            Ok(data) => data,
            Err(err) => {
                panic!("failed to parse data as json:\n{}", err);
            }
        };
        let output = match interrogator.evaluate(Structure::Flag, key, &data) {
            Ok(output) => output,
            Err(err) => {
                panic!("failed to evaluate schema:\n{}", err);
            }
        };
        let valid_msg = if expected_valid { "valid" } else { "invalid" };
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  expected: {valid_msg}\n  schema:{SCHEMA}\n  data: {data}")
    }
    #[test]
    fn test2_type_matches_non_schema_in_third_any_of() {
        let description = "type matches non-schema in third anyOf";
        let data = "1";
        let expected_valid = false;
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = match serde_json::from_str(data) {
            Ok(data) => data,
            Err(err) => {
                panic!("failed to parse data as json:\n{}", err);
            }
        };
        let output = match interrogator.evaluate(Structure::Flag, key, &data) {
            Ok(output) => output,
            Err(err) => {
                panic!("failed to evaluate schema:\n{}", err);
            }
        };
        let valid_msg = if expected_valid { "valid" } else { "invalid" };
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  expected: {valid_msg}\n  schema:{SCHEMA}\n  data: {data}")
    }
}
