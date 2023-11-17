use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_dynamic_ref(interrogator)
    }
    interrogator
}
mod a_dynamic_ref_to_a_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_an_anchor_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/dynamicRef-dynamicAnchor-same-schema/root",
            "type": "array",
            "items": { "$dynamicRef": "#items" },
            "$defs": {
                "foo": {
                    "$dynamicAnchor": "items",
                    "type": "string"
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_an_array_of_strings_is_valid() {
        let description = "An array of strings is valid";
        let data = "[\"foo\", \"bar\"]";
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
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let data = "[\"foo\", 42]";
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
mod a_dynamic_ref_to_an_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_an_anchor_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/dynamicRef-anchor-same-schema/root",
            "type": "array",
            "items": { "$dynamicRef": "#items" },
            "$defs": {
                "foo": {
                    "$anchor": "items",
                    "type": "string"
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "A $dynamicRef to an $anchor in the same schema resource behaves like a normal $ref to an $anchor" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_an_array_of_strings_is_valid() {
        let description = "An array of strings is valid";
        let data = "[\"foo\", \"bar\"]";
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
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let data = "[\"foo\", 42]";
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
mod a_ref_to_a_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_an_anchor_2 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/ref-dynamicAnchor-same-schema/root",
            "type": "array",
            "items": { "$ref": "#items" },
            "$defs": {
                "foo": {
                    "$dynamicAnchor": "items",
                    "type": "string"
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "A $ref to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_an_array_of_strings_is_valid() {
        let description = "An array of strings is valid";
        let data = "[\"foo\", \"bar\"]";
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
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let data = "[\"foo\", 42]";
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
mod a_dynamic_ref_resolves_to_the_first_dynamic_anchor_still_in_scope_that_is_encountered_when_the_schema_is_evaluated_3 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/typical-dynamic-resolution/root",
            "$ref": "list",
            "$defs": {
                "foo": {
                    "$dynamicAnchor": "items",
                    "type": "string"
                },
                "list": {
                    "$id": "list",
                    "type": "array",
                    "items": { "$dynamicRef": "#items" },
                    "$defs": {
                      "items": {
                          "$comment": "This is only needed to satisfy the bookending requirement",
                          "$dynamicAnchor": "items"
                      }
                    }
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_an_array_of_strings_is_valid() {
        let description = "An array of strings is valid";
        let data = "[\"foo\", \"bar\"]";
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
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let data = "[\"foo\", 42]";
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
mod a_dynamic_ref_without_anchor_in_fragment_behaves_identical_to_ref_4 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/dynamicRef-without-anchor/root",
            "$ref": "list",
            "$defs": {
                "foo": {
                    "$dynamicAnchor": "items",
                    "type": "string"
                },
                "list": {
                    "$id": "list",
                    "type": "array",
                    "items": { "$dynamicRef": "#/$defs/items" },
                    "$defs": {
                      "items": {
                          "$comment": "This is only needed to satisfy the bookending requirement",
                          "$dynamicAnchor": "items",
                          "type": "number"
                      }
                    }
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION: &str = "A $dynamicRef without anchor in fragment behaves identical to $ref";
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_an_array_of_strings_is_invalid() {
        let description = "An array of strings is invalid";
        let data = "[\"foo\", \"bar\"]";
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
    fn test1_an_array_of_numbers_is_valid() {
        let description = "An array of numbers is valid";
        let data = "[24, 42]";
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
mod a_dynamic_ref_with_intermediate_scopes_that_don_t_include_a_matching_dynamic_anchor_does_not_affect_dynamic_scope_resolution_5 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/dynamic-resolution-with-intermediate-scopes/root",
            "$ref": "intermediate-scope",
            "$defs": {
                "foo": {
                    "$dynamicAnchor": "items",
                    "type": "string"
                },
                "intermediate-scope": {
                    "$id": "intermediate-scope",
                    "$ref": "list"
                },
                "list": {
                    "$id": "list",
                    "type": "array",
                    "items": { "$dynamicRef": "#items" },
                    "$defs": {
                      "items": {
                          "$comment": "This is only needed to satisfy the bookending requirement",
                          "$dynamicAnchor": "items"
                      }
                    }
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_an_array_of_strings_is_valid() {
        let description = "An array of strings is valid";
        let data = "[\"foo\", \"bar\"]";
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
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let data = "[\"foo\", 42]";
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
mod an_anchor_with_the_same_name_as_a_dynamic_anchor_is_not_used_for_dynamic_scope_resolution_6 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/dynamic-resolution-ignores-anchors/root",
            "$ref": "list",
            "$defs": {
                "foo": {
                    "$anchor": "items",
                    "type": "string"
                },
                "list": {
                    "$id": "list",
                    "type": "array",
                    "items": { "$dynamicRef": "#items" },
                    "$defs": {
                      "items": {
                          "$comment": "This is only needed to satisfy the bookending requirement",
                          "$dynamicAnchor": "items"
                      }
                    }
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_any_array_is_valid() {
        let description = "Any array is valid";
        let data = "[\"foo\", 42]";
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
mod a_dynamic_ref_without_a_matching_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_anchor_7 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/dynamic-resolution-without-bookend/root",
            "$ref": "list",
            "$defs": {
                "foo": {
                    "$dynamicAnchor": "items",
                    "type": "string"
                },
                "list": {
                    "$id": "list",
                    "type": "array",
                    "items": { "$dynamicRef": "#items" },
                    "$defs": {
                        "items": {
                            "$comment": "This is only needed to give the reference somewhere to resolve to when it behaves like $ref",
                            "$anchor": "items"
                        }
                    }
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "A $dynamicRef without a matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_any_array_is_valid() {
        let description = "Any array is valid";
        let data = "[\"foo\", 42]";
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
mod a_dynamic_ref_with_a_non_matching_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_anchor_8 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/unmatched-dynamic-anchor/root",
            "$ref": "list",
            "$defs": {
                "foo": {
                    "$dynamicAnchor": "items",
                    "type": "string"
                },
                "list": {
                    "$id": "list",
                    "type": "array",
                    "items": { "$dynamicRef": "#items" },
                    "$defs": {
                        "items": {
                            "$comment": "This is only needed to give the reference somewhere to resolve to when it behaves like $ref",
                            "$anchor": "items",
                            "$dynamicAnchor": "foo"
                        }
                    }
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "A $dynamicRef with a non-matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_any_array_is_valid() {
        let description = "Any array is valid";
        let data = "[\"foo\", 42]";
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
mod a_dynamic_ref_that_initially_resolves_to_a_schema_with_a_matching_dynamic_anchor_resolves_to_the_first_dynamic_anchor_in_the_dynamic_scope_9 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/relative-dynamic-reference/root",
            "$dynamicAnchor": "meta",
            "type": "object",
            "properties": {
                "foo": { "const": "pass" }
            },
            "$ref": "extended",
            "$defs": {
                "extended": {
                    "$id": "extended",
                    "$dynamicAnchor": "meta",
                    "type": "object",
                    "properties": {
                        "bar": { "$ref": "bar" }
                    }
                },
                "bar": {
                    "$id": "bar",
                    "type": "object",
                    "properties": {
                        "baz": { "$dynamicRef": "extended#meta" }
                    }
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "A $dynamicRef that initially resolves to a schema with a matching $dynamicAnchor resolves to the first $dynamicAnchor in the dynamic scope" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_the_recursive_part_is_valid_against_the_root() {
        let description = "The recursive part is valid against the root";
        let data = "{\n                    \"foo\": \"pass\",\n                    \"bar\": {\n                        \"baz\": { \"foo\": \"pass\" }\n                    }\n                }" ;
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
    fn test1_the_recursive_part_is_not_valid_against_the_root() {
        let description = "The recursive part is not valid against the root";
        let data = "{\n                    \"foo\": \"pass\",\n                    \"bar\": {\n                        \"baz\": { \"foo\": \"fail\" }\n                    }\n                }" ;
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
mod a_dynamic_ref_that_initially_resolves_to_a_schema_without_a_matching_dynamic_anchor_behaves_like_a_normal_ref_to_anchor_10 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/relative-dynamic-reference-without-bookend/root",
            "$dynamicAnchor": "meta",
            "type": "object",
            "properties": {
                "foo": { "const": "pass" }
            },
            "$ref": "extended",
            "$defs": {
                "extended": {
                    "$id": "extended",
                    "$anchor": "meta",
                    "type": "object",
                    "properties": {
                        "bar": { "$ref": "bar" }
                    }
                },
                "bar": {
                    "$id": "bar",
                    "type": "object",
                    "properties": {
                        "baz": { "$dynamicRef": "extended#meta" }
                    }
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION : & str = "A $dynamicRef that initially resolves to a schema without a matching $dynamicAnchor behaves like a normal $ref to $anchor" ;
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_the_recursive_part_doesn_t_need_to_validate_against_the_root() {
        let description = "The recursive part doesn't need to validate against the root";
        let data = "{\n                    \"foo\": \"pass\",\n                    \"bar\": {\n                        \"baz\": { \"foo\": \"fail\" }\n                    }\n                }" ;
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
mod multiple_dynamic_paths_to_the_dynamic_ref_keyword_11 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/dynamic-ref-with-multiple-paths/main",
            "$defs": {
                "inner": {
                    "$id": "inner",
                    "$dynamicAnchor": "foo",
                    "title": "inner",
                    "additionalProperties": {
                        "$dynamicRef": "#foo"
                    }
                }
            },
            "if": {
                "propertyNames": {
                    "pattern": "^[a-m]"
                }
            },
            "then": {
                "title": "any type of node",
                "$id": "anyLeafNode",
                "$dynamicAnchor": "foo",
                "$ref": "inner"
            },
            "else": {
                "title": "integer node",
                "$id": "integerNode",
                "$dynamicAnchor": "foo",
                "type": [ "object", "integer" ],
                "$ref": "inner"
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION: &str = "multiple dynamic paths to the $dynamicRef keyword";
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_recurse_to_any_leaf_node_floats_are_allowed() {
        let description = "recurse to anyLeafNode - floats are allowed";
        let data = "{ \"alpha\": 1.1 }";
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
    fn test1_recurse_to_integer_node_floats_are_not_allowed() {
        let description = "recurse to integerNode - floats are not allowed";
        let data = "{ \"november\": 1.1 }";
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
mod after_leaving_a_dynamic_scope_it_is_not_used_by_a_dynamic_ref_12 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://test.json-schema.org/dynamic-ref-leaving-dynamic-scope/main",
            "if": {
                "$id": "first_scope",
                "$defs": {
                    "thingy": {
                        "$comment": "this is first_scope#thingy",
                        "$dynamicAnchor": "thingy",
                        "type": "number"
                    }
                }
            },
            "then": {
                "$id": "second_scope",
                "$ref": "start",
                "$defs": {
                    "thingy": {
                        "$comment": "this is second_scope#thingy, the final destination of the $dynamicRef",
                        "$dynamicAnchor": "thingy",
                        "type": "null"
                    }
                }
            },
            "$defs": {
                "start": {
                    "$comment": "this is the landing spot from $ref",
                    "$id": "start",
                    "$dynamicRef": "inner_scope#thingy"
                },
                "thingy": {
                    "$comment": "this is the first stop for the $dynamicRef",
                    "$id": "inner_scope",
                    "$dynamicAnchor": "thingy",
                    "type": "string"
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION: &str = "after leaving a dynamic scope, it is not used by a $dynamicRef";
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_string_matches_defs_thingy_but_the_dynamic_ref_does_not_stop_here() {
        let description = "string matches /$defs/thingy, but the $dynamicRef does not stop here";
        let data = "\"a string\"";
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
    fn test1_first_scope_is_not_in_dynamic_scope_for_the_dynamic_ref() {
        let description = "first_scope is not in dynamic scope for the $dynamicRef";
        let data = "42";
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
    fn test2_then_defs_thingy_is_the_final_stop_for_the_dynamic_ref() {
        let description = "/then/$defs/thingy is the final stop for the $dynamicRef";
        let data = "null";
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
mod strict_tree_schema_guards_against_misspelled_properties_13 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://localhost:1234/draft2020-12/strict-tree.json",
            "$dynamicAnchor": "node",

            "$ref": "tree.json",
            "unevaluatedProperties": false
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION: &str = "strict-tree schema, guards against misspelled properties";
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_instance_with_misspelled_field() {
        let description = "instance with misspelled field";
        let data = "{\n                    \"children\": [{\n                            \"daat\": 1\n                        }]\n                }" ;
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
    fn test1_instance_with_correct_field() {
        let description = "instance with correct field";
        let data = "{\n                    \"children\": [{\n                            \"data\": 1\n                        }]\n                }" ;
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
mod tests_for_implementation_dynamic_anchor_and_reference_link_14 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://localhost:1234/draft2020-12/strict-extendible.json",
            "$ref": "extendible-dynamic-ref.json",
            "$defs": {
                "elements": {
                    "$dynamicAnchor": "elements",
                    "properties": {
                        "a": true
                    },
                    "required": ["a"],
                    "additionalProperties": false
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION: &str = "tests for implementation dynamic anchor and reference link";
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_incorrect_parent_schema() {
        let description = "incorrect parent schema";
        let data = "{\n                    \"a\": true\n                }";
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
    fn test1_incorrect_extended_schema() {
        let description = "incorrect extended schema";
        let data = "{\n                    \"elements\": [\n                        { \"b\": 1 }\n                    ]\n                }" ;
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
    fn test2_correct_extended_schema() {
        let description = "correct extended schema";
        let data = "{\n                    \"elements\": [\n                        { \"a\": 1 }\n                    ]\n                }" ;
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
mod ref_and_dynamic_anchor_are_independent_of_order_defs_first_15 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://localhost:1234/draft2020-12/strict-extendible-allof-defs-first.json",
            "allOf": [
                {
                    "$ref": "extendible-dynamic-ref.json"
                },
                {
                    "$defs": {
                        "elements": {
                            "$dynamicAnchor": "elements",
                            "properties": {
                                "a": true
                            },
                            "required": ["a"],
                            "additionalProperties": false
                        }
                    }
                }
            ]
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION: &str = "$ref and $dynamicAnchor are independent of order - $defs first";
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_incorrect_parent_schema() {
        let description = "incorrect parent schema";
        let data = "{\n                    \"a\": true\n                }";
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
    fn test1_incorrect_extended_schema() {
        let description = "incorrect extended schema";
        let data = "{\n                    \"elements\": [\n                        { \"b\": 1 }\n                    ]\n                }" ;
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
    fn test2_correct_extended_schema() {
        let description = "correct extended schema";
        let data = "{\n                    \"elements\": [\n                        { \"a\": 1 }\n                    ]\n                }" ;
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
mod ref_and_dynamic_anchor_are_independent_of_order_ref_first_16 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://localhost:1234/draft2020-12/strict-extendible-allof-ref-first.json",
            "allOf": [
                {
                    "$defs": {
                        "elements": {
                            "$dynamicAnchor": "elements",
                            "properties": {
                                "a": true
                            },
                            "required": ["a"],
                            "additionalProperties": false
                        }
                    }
                },
                {
                    "$ref": "extendible-dynamic-ref.json"
                }
            ]
        }"##;
    const URI: &str = "http://localhost:1234/dynamicRef.json";
    const DESCRIPTION: &str = "$ref and $dynamicAnchor are independent of order - $ref first";
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
                let key = block_on(interrogator.compile("http://localhost:1234/dynamicRef.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_incorrect_parent_schema() {
        let description = "incorrect parent schema";
        let data = "{\n                    \"a\": true\n                }";
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
    fn test1_incorrect_extended_schema() {
        let description = "incorrect extended schema";
        let data = "{\n                    \"elements\": [\n                        { \"b\": 1 }\n                    ]\n                }" ;
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
    fn test2_correct_extended_schema() {
        let description = "correct extended schema";
        let data = "{\n                    \"elements\": [\n                        { \"a\": 1 }\n                    ]\n                }" ;
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
