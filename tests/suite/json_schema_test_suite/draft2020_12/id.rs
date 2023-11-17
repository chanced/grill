use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_id(interrogator)
    }
    interrogator
}
mod invalid_use_of_fragments_in_location_independent_id_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$ref": "https://json-schema.org/draft/2020-12/schema"
        }"##;
    const URI: &str = "http://localhost:1234/id.json";
    const DESCRIPTION: &str = "Invalid use of fragments in location-independent $id";
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
                let key = block_on(interrogator.compile("http://localhost:1234/id.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_identifier_name() {
        let description = "Identifier name";
        let data = "{\n                    \"$ref\": \"#foo\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"#foo\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test1_identifier_name_and_no_ref() {
        let description = "Identifier name and no ref";
        let data = "{\n                    \"$defs\": {\n                        \"A\": { \"$id\": \"#foo\" }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test2_identifier_path() {
        let description = "Identifier path";
        let data = "{\n                    \"$ref\": \"#/a/b\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"#/a/b\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test3_identifier_name_with_absolute_uri() {
        let description = "Identifier name with absolute URI";
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/bar#foo\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/bar#foo\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test4_identifier_path_with_absolute_uri() {
        let description = "Identifier path with absolute URI";
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/bar#/a/b\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/bar#/a/b\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test5_identifier_name_with_base_uri_change_in_subschema() {
        let description = "Identifier name with base URI change in subschema";
        let data = "{\n                    \"$id\": \"http://localhost:1234/draft2020-12/root\",\n                    \"$ref\": \"http://localhost:1234/draft2020-12/nested.json#foo\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"nested.json\",\n                            \"$defs\": {\n                                \"B\": {\n                                    \"$id\": \"#foo\",\n                                    \"type\": \"integer\"\n                                }\n                            }\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test6_identifier_path_with_base_uri_change_in_subschema() {
        let description = "Identifier path with base URI change in subschema";
        let data = "{\n                    \"$id\": \"http://localhost:1234/draft2020-12/root\",\n                    \"$ref\": \"http://localhost:1234/draft2020-12/nested.json#/a/b\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"nested.json\",\n                            \"$defs\": {\n                                \"B\": {\n                                    \"$id\": \"#/a/b\",\n                                    \"type\": \"integer\"\n                                }\n                            }\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
}
mod valid_use_of_empty_fragments_in_location_independent_id_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$ref": "https://json-schema.org/draft/2020-12/schema"
        }"##;
    const URI: &str = "http://localhost:1234/id.json";
    const DESCRIPTION: &str = "Valid use of empty fragments in location-independent $id";
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
                let key = block_on(interrogator.compile("http://localhost:1234/id.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_identifier_name_with_absolute_uri() {
        let description = "Identifier name with absolute URI";
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/bar\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/bar#\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test1_identifier_name_with_base_uri_change_in_subschema() {
        let description = "Identifier name with base URI change in subschema";
        let data = "{\n                    \"$id\": \"http://localhost:1234/draft2020-12/root\",\n                    \"$ref\": \"http://localhost:1234/draft2020-12/nested.json#/$defs/B\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"nested.json\",\n                            \"$defs\": {\n                                \"B\": {\n                                    \"$id\": \"#\",\n                                    \"type\": \"integer\"\n                                }\n                            }\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
}
mod unnormalized_ids_are_allowed_but_discouraged_2 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$ref": "https://json-schema.org/draft/2020-12/schema"
        }"##;
    const URI: &str = "http://localhost:1234/id.json";
    const DESCRIPTION: &str = "Unnormalized $ids are allowed but discouraged";
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
                let key = block_on(interrogator.compile("http://localhost:1234/id.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_unnormalized_identifier() {
        let description = "Unnormalized identifier";
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/foo/baz\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/foo/bar/../baz\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test1_unnormalized_identifier_and_no_ref() {
        let description = "Unnormalized identifier and no ref";
        let data = "{\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/foo/bar/../baz\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test2_unnormalized_identifier_with_empty_fragment() {
        let description = "Unnormalized identifier with empty fragment";
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/foo/baz\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/foo/bar/../baz#\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test3_unnormalized_identifier_with_empty_fragment_and_no_ref() {
        let description = "Unnormalized identifier with empty fragment and no ref";
        let data = "{\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/foo/bar/../baz#\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
}
mod id_inside_an_enum_is_not_a_real_identifier_3 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "id_in_enum": {
                    "enum": [
                        {
                          "$id": "https://localhost:1234/draft2020-12/id/my_identifier.json",
                          "type": "null"
                        }
                    ]
                },
                "real_id_in_schema": {
                    "$id": "https://localhost:1234/draft2020-12/id/my_identifier.json",
                    "type": "string"
                },
                "zzz_id_in_const": {
                    "const": {
                        "$id": "https://localhost:1234/draft2020-12/id/my_identifier.json",
                        "type": "null"
                    }
                }
            },
            "anyOf": [
                { "$ref": "#/$defs/id_in_enum" },
                { "$ref": "https://localhost:1234/draft2020-12/id/my_identifier.json" }
            ]
        }"##;
    const URI: &str = "http://localhost:1234/id.json";
    const DESCRIPTION: &str = "$id inside an enum is not a real identifier";
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
                let key = block_on(interrogator.compile("http://localhost:1234/id.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_exact_match_to_enum_and_type_matches() {
        let description = "exact match to enum, and type matches";
        let data = "{\n                    \"$id\": \"https://localhost:1234/draft2020-12/id/my_identifier.json\",\n                    \"type\": \"null\"\n                }" ;
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test1_match_ref_to_id() {
        let description = "match $ref to $id";
        let data = "\"a string to match #/$defs/id_in_enum\"";
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test2_no_match_on_enum_or_ref_to_id() {
        let description = "no match on enum or $ref to $id";
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
}
mod non_schema_object_containing_an_id_property_4 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "const_not_id": {
                    "const": {
                        "$id": "not_a_real_id"
                    }
                }
            },
            "if": {
                "const": "skip not_a_real_id"
            },
            "then": true,
            "else" : {
                "$ref": "#/$defs/const_not_id"
            }
        }"##;
    const URI: &str = "http://localhost:1234/id.json";
    const DESCRIPTION: &str = "non-schema object containing an $id property";
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
                let key = block_on(interrogator.compile("http://localhost:1234/id.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_skip_traversing_definition_for_a_valid_result() {
        let description = "skip traversing definition for a valid result";
        let data = "\"skip not_a_real_id\"";
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
    #[test]
    fn test1_const_at_const_not_id_does_not_match() {
        let description = "const at const_not_id does not match";
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
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}")
    }
}
