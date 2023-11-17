use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_ref(interrogator)
    }
    interrogator
}
mod root_pointer_ref_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo": {"$ref": "#"}
            },
            "additionalProperties": false
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "root pointer ref";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_match_() {
        let description = "match";
        let data = "{\"foo\": false}";
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
    fn test1_recursive_match() {
        let description = "recursive match";
        let data = "{\"foo\": {\"foo\": false}}";
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
    fn test2_mismatch() {
        let description = "mismatch";
        let data = "{\"bar\": false}";
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
    fn test3_recursive_mismatch() {
        let description = "recursive mismatch";
        let data = "{\"foo\": {\"bar\": false}}";
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
mod relative_pointer_ref_to_object_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo": {"type": "integer"},
                "bar": {"$ref": "#/properties/foo"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "relative pointer ref to object";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_match_() {
        let description = "match";
        let data = "{\"bar\": 3}";
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
    fn test1_mismatch() {
        let description = "mismatch";
        let data = "{\"bar\": true}";
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
mod relative_pointer_ref_to_array_2 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "prefixItems": [
                {"type": "integer"},
                {"$ref": "#/prefixItems/0"}
            ]
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "relative pointer ref to array";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_match_array() {
        let description = "match array";
        let data = "[1, 2]";
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
    fn test1_mismatch_array() {
        let description = "mismatch array";
        let data = "[1, \"foo\"]";
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
mod escaped_pointer_ref_3 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "tilde~field": {"type": "integer"},
                "slash/field": {"type": "integer"},
                "percent%field": {"type": "integer"}
            },
            "properties": {
                "tilde": {"$ref": "#/$defs/tilde~0field"},
                "slash": {"$ref": "#/$defs/slash~1field"},
                "percent": {"$ref": "#/$defs/percent%25field"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "escaped pointer ref";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_slash_invalid() {
        let description = "slash invalid";
        let data = "{\"slash\": \"aoeu\"}";
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
    fn test1_tilde_invalid() {
        let description = "tilde invalid";
        let data = "{\"tilde\": \"aoeu\"}";
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
    fn test2_percent_invalid() {
        let description = "percent invalid";
        let data = "{\"percent\": \"aoeu\"}";
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
    fn test3_slash_valid() {
        let description = "slash valid";
        let data = "{\"slash\": 123}";
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
    fn test4_tilde_valid() {
        let description = "tilde valid";
        let data = "{\"tilde\": 123}";
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
    fn test5_percent_valid() {
        let description = "percent valid";
        let data = "{\"percent\": 123}";
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
mod nested_refs_4 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "a": {"type": "integer"},
                "b": {"$ref": "#/$defs/a"},
                "c": {"$ref": "#/$defs/b"}
            },
            "$ref": "#/$defs/c"
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "nested refs";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_nested_ref_valid() {
        let description = "nested ref valid";
        let data = "5";
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
    fn test1_nested_ref_invalid() {
        let description = "nested ref invalid";
        let data = "\"a\"";
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
mod ref_applies_alongside_sibling_keywords_5 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "reffed": {
                    "type": "array"
                }
            },
            "properties": {
                "foo": {
                    "$ref": "#/$defs/reffed",
                    "maxItems": 2
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "ref applies alongside sibling keywords";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_ref_valid_max_items_valid() {
        let description = "ref valid, maxItems valid";
        let data = "{ \"foo\": [] }";
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
    fn test1_ref_valid_max_items_invalid() {
        let description = "ref valid, maxItems invalid";
        let data = "{ \"foo\": [1, 2, 3] }";
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
    fn test2_ref_invalid() {
        let description = "ref invalid";
        let data = "{ \"foo\": \"string\" }";
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
mod remote_ref_containing_refs_itself_6 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$ref": "https://json-schema.org/draft/2020-12/schema"
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "remote ref, containing refs itself";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_remote_ref_valid() {
        let description = "remote ref valid";
        let data = "{\"minLength\": 1}";
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
    fn test1_remote_ref_invalid() {
        let description = "remote ref invalid";
        let data = "{\"minLength\": -1}";
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
mod property_named_ref_that_is_not_a_reference_7 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "$ref": {"type": "string"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "property named $ref that is not a reference";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_property_named_ref_valid() {
        let description = "property named $ref valid";
        let data = "{\"$ref\": \"a\"}";
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
    fn test1_property_named_ref_invalid() {
        let description = "property named $ref invalid";
        let data = "{\"$ref\": 2}";
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
mod property_named_ref_containing_an_actual_ref_8 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "$ref": {"$ref": "#/$defs/is-string"}
            },
            "$defs": {
                "is-string": {
                    "type": "string"
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "property named $ref, containing an actual $ref";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_property_named_ref_valid() {
        let description = "property named $ref valid";
        let data = "{\"$ref\": \"a\"}";
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
    fn test1_property_named_ref_invalid() {
        let description = "property named $ref invalid";
        let data = "{\"$ref\": 2}";
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
mod ref_to_boolean_schema_true_9 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$ref": "#/$defs/bool",
            "$defs": {
                "bool": true
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "$ref to boolean schema true";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_any_value_is_valid() {
        let description = "any value is valid";
        let data = "\"foo\"";
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
mod ref_to_boolean_schema_false_10 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$ref": "#/$defs/bool",
            "$defs": {
                "bool": false
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "$ref to boolean schema false";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_any_value_is_invalid() {
        let description = "any value is invalid";
        let data = "\"foo\"";
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
mod recursive_references_between_schemas_11 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://localhost:1234/draft2020-12/tree",
            "description": "tree of nodes",
            "type": "object",
            "properties": {
                "meta": {"type": "string"},
                "nodes": {
                    "type": "array",
                    "items": {"$ref": "node"}
                }
            },
            "required": ["meta", "nodes"],
            "$defs": {
                "node": {
                    "$id": "http://localhost:1234/draft2020-12/node",
                    "description": "node",
                    "type": "object",
                    "properties": {
                        "value": {"type": "number"},
                        "subtree": {"$ref": "tree"}
                    },
                    "required": ["value"]
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "Recursive references between schemas";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid_tree() {
        let description = "valid tree";
        let data = "{\n                    \"meta\": \"root\",\n                    \"nodes\": [\n                        {\n                            \"value\": 1,\n                            \"subtree\": {\n                                \"meta\": \"child\",\n                                \"nodes\": [\n                                    {\"value\": 1.1},\n                                    {\"value\": 1.2}\n                                ]\n                            }\n                        },\n                        {\n                            \"value\": 2,\n                            \"subtree\": {\n                                \"meta\": \"child\",\n                                \"nodes\": [\n                                    {\"value\": 2.1},\n                                    {\"value\": 2.2}\n                                ]\n                            }\n                        }\n                    ]\n                }" ;
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
    fn test1_invalid_tree() {
        let description = "invalid tree";
        let data = "{\n                    \"meta\": \"root\",\n                    \"nodes\": [\n                        {\n                            \"value\": 1,\n                            \"subtree\": {\n                                \"meta\": \"child\",\n                                \"nodes\": [\n                                    {\"value\": \"string is invalid\"},\n                                    {\"value\": 1.2}\n                                ]\n                            }\n                        },\n                        {\n                            \"value\": 2,\n                            \"subtree\": {\n                                \"meta\": \"child\",\n                                \"nodes\": [\n                                    {\"value\": 2.1},\n                                    {\"value\": 2.2}\n                                ]\n                            }\n                        }\n                    ]\n                }" ;
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
mod refs_with_quote_12 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo\"bar": {"$ref": "#/$defs/foo%22bar"}
            },
            "$defs": {
                "foo\"bar": {"type": "number"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "refs with quote";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_object_with_numbers_is_valid() {
        let description = "object with numbers is valid";
        let data = "{\n                    \"foo\\\"bar\": 1\n                }";
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
    fn test1_object_with_strings_is_invalid() {
        let description = "object with strings is invalid";
        let data = "{\n                    \"foo\\\"bar\": \"1\"\n                }";
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
mod ref_creates_new_scope_when_adjacent_to_keywords_13 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "A": {
                    "unevaluatedProperties": false
                }
            },
            "properties": {
                "prop1": {
                    "type": "string"
                }
            },
            "$ref": "#/$defs/A"
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "ref creates new scope when adjacent to keywords";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_referenced_subschema_doesn_t_see_annotations_from_properties() {
        let description = "referenced subschema doesn't see annotations from properties";
        let data = "{\n                    \"prop1\": \"match\"\n                }";
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
mod naive_replacement_of_ref_with_its_destination_is_not_correct_14 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "a_string": { "type": "string" }
            },
            "enum": [
                { "$ref": "#/$defs/a_string" }
            ]
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "naive replacement of $ref with its destination is not correct";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_do_not_evaluate_the_ref_inside_the_enum_matching_any_string() {
        let description = "do not evaluate the $ref inside the enum, matching any string";
        let data = "\"this is a string\"";
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
    fn test1_do_not_evaluate_the_ref_inside_the_enum_definition_exact_match() {
        let description = "do not evaluate the $ref inside the enum, definition exact match";
        let data = "{ \"type\": \"string\" }";
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
    fn test2_match_the_enum_exactly() {
        let description = "match the enum exactly";
        let data = "{ \"$ref\": \"#/$defs/a_string\" }";
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
mod refs_with_relative_uris_and_defs_15 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://example.com/schema-relative-uri-defs1.json",
            "properties": {
                "foo": {
                    "$id": "schema-relative-uri-defs2.json",
                    "$defs": {
                        "inner": {
                            "properties": {
                                "bar": { "type": "string" }
                            }
                        }
                    },
                    "$ref": "#/$defs/inner"
                }
            },
            "$ref": "schema-relative-uri-defs2.json"
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "refs with relative uris and defs";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_invalid_on_inner_field() {
        let description = "invalid on inner field";
        let data = "{\n                    \"foo\": {\n                        \"bar\": 1\n                    },\n                    \"bar\": \"a\"\n                }" ;
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
    fn test1_invalid_on_outer_field() {
        let description = "invalid on outer field";
        let data = "{\n                    \"foo\": {\n                        \"bar\": \"a\"\n                    },\n                    \"bar\": 1\n                }" ;
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
    fn test2_valid_on_both_fields() {
        let description = "valid on both fields";
        let data = "{\n                    \"foo\": {\n                        \"bar\": \"a\"\n                    },\n                    \"bar\": \"a\"\n                }" ;
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
mod relative_refs_with_absolute_uris_and_defs_16 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://example.com/schema-refs-absolute-uris-defs1.json",
            "properties": {
                "foo": {
                    "$id": "http://example.com/schema-refs-absolute-uris-defs2.json",
                    "$defs": {
                        "inner": {
                            "properties": {
                                "bar": { "type": "string" }
                            }
                        }
                    },
                    "$ref": "#/$defs/inner"
                }
            },
            "$ref": "schema-refs-absolute-uris-defs2.json"
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "relative refs with absolute uris and defs";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_invalid_on_inner_field() {
        let description = "invalid on inner field";
        let data = "{\n                    \"foo\": {\n                        \"bar\": 1\n                    },\n                    \"bar\": \"a\"\n                }" ;
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
    fn test1_invalid_on_outer_field() {
        let description = "invalid on outer field";
        let data = "{\n                    \"foo\": {\n                        \"bar\": \"a\"\n                    },\n                    \"bar\": 1\n                }" ;
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
    fn test2_valid_on_both_fields() {
        let description = "valid on both fields";
        let data = "{\n                    \"foo\": {\n                        \"bar\": \"a\"\n                    },\n                    \"bar\": \"a\"\n                }" ;
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
mod id_must_be_resolved_against_nearest_parent_not_just_immediate_parent_17 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://example.com/a.json",
            "$defs": {
                "x": {
                    "$id": "http://example.com/b/c.json",
                    "not": {
                        "$defs": {
                            "y": {
                                "$id": "d.json",
                                "type": "number"
                            }
                        }
                    }
                }
            },
            "allOf": [
                {
                    "$ref": "http://example.com/b/d.json"
                }
            ]
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str =
        "$id must be resolved against nearest parent, not just immediate parent";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_number_is_valid() {
        let description = "number is valid";
        let data = "1";
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
    fn test1_non_number_is_invalid() {
        let description = "non-number is invalid";
        let data = "\"a\"";
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
mod order_of_evaluation_id_and_ref_18 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$comment": "$id must be evaluated before $ref to get the proper $ref destination",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com/draft2020-12/ref-and-id1/base.json",
            "$ref": "int.json",
            "$defs": {
                "bigint": {
                    "$comment": "canonical uri: https://example.com/ref-and-id1/int.json",
                    "$id": "int.json",
                    "maximum": 10
                },
                "smallint": {
                    "$comment": "canonical uri: https://example.com/ref-and-id1-int.json",
                    "$id": "/draft2020-12/ref-and-id1-int.json",
                    "maximum": 2
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "order of evaluation: $id and $ref";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_data_is_valid_against_first_definition() {
        let description = "data is valid against first definition";
        let data = "5";
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
    fn test1_data_is_invalid_against_first_definition() {
        let description = "data is invalid against first definition";
        let data = "50";
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
mod order_of_evaluation_id_and_anchor_and_ref_19 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$comment": "$id must be evaluated before $ref to get the proper $ref destination",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com/draft2020-12/ref-and-id2/base.json",
            "$ref": "#bigint",
            "$defs": {
                "bigint": {
                    "$comment": "canonical uri: /ref-and-id2/base.json#/$defs/bigint; another valid uri for this location: /ref-and-id2/base.json#bigint",
                    "$anchor": "bigint",
                    "maximum": 10
                },
                "smallint": {
                    "$comment": "canonical uri: https://example.com/ref-and-id2#/$defs/smallint; another valid uri for this location: https://example.com/ref-and-id2/#bigint",
                    "$id": "https://example.com/draft2020-12/ref-and-id2/",
                    "$anchor": "bigint",
                    "maximum": 2
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "order of evaluation: $id and $anchor and $ref";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_data_is_valid_against_first_definition() {
        let description = "data is valid against first definition";
        let data = "5";
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
    fn test1_data_is_invalid_against_first_definition() {
        let description = "data is invalid against first definition";
        let data = "50";
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
mod simple_urn_base_uri_with_ref_via_the_urn_20 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$comment": "URIs do not have to have HTTP(s) schemes",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "urn:uuid:deadbeef-1234-ffff-ffff-4321feebdaed",
            "minimum": 30,
            "properties": {
                "foo": {"$ref": "urn:uuid:deadbeef-1234-ffff-ffff-4321feebdaed"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "simple URN base URI with $ref via the URN";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid_under_the_urn_i_ded_schema() {
        let description = "valid under the URN IDed schema";
        let data = "{\"foo\": 37}";
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
    fn test1_invalid_under_the_urn_i_ded_schema() {
        let description = "invalid under the URN IDed schema";
        let data = "{\"foo\": 12}";
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
mod simple_urn_base_uri_with_json_pointer_21 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$comment": "URIs do not have to have HTTP(s) schemes",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "urn:uuid:deadbeef-1234-00ff-ff00-4321feebdaed",
            "properties": {
                "foo": {"$ref": "#/$defs/bar"}
            },
            "$defs": {
                "bar": {"type": "string"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "simple URN base URI with JSON pointer";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_string_is_valid() {
        let description = "a string is valid";
        let data = "{\"foo\": \"bar\"}";
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
    fn test1_a_non_string_is_invalid() {
        let description = "a non-string is invalid";
        let data = "{\"foo\": 12}";
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
mod urn_base_uri_with_nss_22 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$comment": "RFC 8141 §2.2",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "urn:example:1/406/47452/2",
            "properties": {
                "foo": {"$ref": "#/$defs/bar"}
            },
            "$defs": {
                "bar": {"type": "string"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "URN base URI with NSS";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_string_is_valid() {
        let description = "a string is valid";
        let data = "{\"foo\": \"bar\"}";
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
    fn test1_a_non_string_is_invalid() {
        let description = "a non-string is invalid";
        let data = "{\"foo\": 12}";
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
mod urn_base_uri_with_r_component_23 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$comment": "RFC 8141 §2.3.1",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "urn:example:foo-bar-baz-qux?+CCResolve:cc=uk",
            "properties": {
                "foo": {"$ref": "#/$defs/bar"}
            },
            "$defs": {
                "bar": {"type": "string"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "URN base URI with r-component";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_string_is_valid() {
        let description = "a string is valid";
        let data = "{\"foo\": \"bar\"}";
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
    fn test1_a_non_string_is_invalid() {
        let description = "a non-string is invalid";
        let data = "{\"foo\": 12}";
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
mod urn_base_uri_with_q_component_24 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$comment": "RFC 8141 §2.3.2",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "urn:example:weather?=op=map&lat=39.56&lon=-104.85&datetime=1969-07-21T02:56:15Z",
            "properties": {
                "foo": {"$ref": "#/$defs/bar"}
            },
            "$defs": {
                "bar": {"type": "string"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "URN base URI with q-component";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_string_is_valid() {
        let description = "a string is valid";
        let data = "{\"foo\": \"bar\"}";
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
    fn test1_a_non_string_is_invalid() {
        let description = "a non-string is invalid";
        let data = "{\"foo\": 12}";
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
mod urn_base_uri_with_f_component_25 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$comment": "RFC 8141 §2.3.3, but we don't allow fragments",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$ref": "https://json-schema.org/draft/2020-12/schema"
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "URN base URI with f-component";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_is_invalid() {
        let description = "is invalid";
        let data = "{\"$id\": \"urn:example:foo-bar-baz-qux#somepart\"}";
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
mod urn_base_uri_with_urn_and_json_pointer_ref_26 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "urn:uuid:deadbeef-1234-0000-0000-4321feebdaed",
            "properties": {
                "foo": {"$ref": "urn:uuid:deadbeef-1234-0000-0000-4321feebdaed#/$defs/bar"}
            },
            "$defs": {
                "bar": {"type": "string"}
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "URN base URI with URN and JSON pointer ref";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_string_is_valid() {
        let description = "a string is valid";
        let data = "{\"foo\": \"bar\"}";
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
    fn test1_a_non_string_is_invalid() {
        let description = "a non-string is invalid";
        let data = "{\"foo\": 12}";
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
mod urn_base_uri_with_urn_and_anchor_ref_27 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "urn:uuid:deadbeef-1234-ff00-00ff-4321feebdaed",
            "properties": {
                "foo": {"$ref": "urn:uuid:deadbeef-1234-ff00-00ff-4321feebdaed#something"}
            },
            "$defs": {
                "bar": {
                    "$anchor": "something",
                    "type": "string"
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "URN base URI with URN and anchor ref";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_string_is_valid() {
        let description = "a string is valid";
        let data = "{\"foo\": \"bar\"}";
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
    fn test1_a_non_string_is_invalid() {
        let description = "a non-string is invalid";
        let data = "{\"foo\": 12}";
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
mod urn_ref_with_nested_pointer_ref_28 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$ref": "urn:uuid:deadbeef-4321-ffff-ffff-1234feebdaed",
            "$defs": {
                "foo": {
                    "$id": "urn:uuid:deadbeef-4321-ffff-ffff-1234feebdaed",
                    "$defs": {"bar": {"type": "string"}},
                    "$ref": "#/$defs/bar"
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "URN ref with nested pointer ref";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_string_is_valid() {
        let description = "a string is valid";
        let data = "\"bar\"";
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
    fn test1_a_non_string_is_invalid() {
        let description = "a non-string is invalid";
        let data = "12";
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
mod ref_to_if_29 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$ref": "http://example.com/ref/if",
            "if": {
                "$id": "http://example.com/ref/if",
                "type": "integer"
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "ref to if";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_non_integer_is_invalid_due_to_the_ref() {
        let description = "a non-integer is invalid due to the $ref";
        let data = "\"foo\"";
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
    fn test1_an_integer_is_valid() {
        let description = "an integer is valid";
        let data = "12";
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
mod ref_to_then_30 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$ref": "http://example.com/ref/then",
            "then": {
                "$id": "http://example.com/ref/then",
                "type": "integer"
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "ref to then";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_non_integer_is_invalid_due_to_the_ref() {
        let description = "a non-integer is invalid due to the $ref";
        let data = "\"foo\"";
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
    fn test1_an_integer_is_valid() {
        let description = "an integer is valid";
        let data = "12";
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
mod ref_to_else_31 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$ref": "http://example.com/ref/else",
            "else": {
                "$id": "http://example.com/ref/else",
                "type": "integer"
            }
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "ref to else";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_non_integer_is_invalid_due_to_the_ref() {
        let description = "a non-integer is invalid due to the $ref";
        let data = "\"foo\"";
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
    fn test1_an_integer_is_valid() {
        let description = "an integer is valid";
        let data = "12";
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
mod ref_with_absolute_path_reference_32 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$id": "http://example.com/ref/absref.json",
            "$defs": {
                "a": {
                    "$id": "http://example.com/ref/absref/foobar.json",
                    "type": "number"
                },
                "b": {
                    "$id": "http://example.com/absref/foobar.json",
                    "type": "string"
                }
            },
            "$ref": "/absref/foobar.json"
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "ref with absolute-path-reference";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_string_is_valid() {
        let description = "a string is valid";
        let data = "\"foo\"";
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
    fn test1_an_integer_is_invalid() {
        let description = "an integer is invalid";
        let data = "12";
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
mod id_with_file_uri_still_resolves_pointers_nix_33 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$id": "file:///folder/file.json",
            "$defs": {
                "foo": {
                    "type": "number"
                }
            },
            "$ref": "#/$defs/foo"
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "$id with file URI still resolves pointers - *nix";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_number_is_valid() {
        let description = "number is valid";
        let data = "1";
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
    fn test1_non_number_is_invalid() {
        let description = "non-number is invalid";
        let data = "\"a\"";
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
mod id_with_file_uri_still_resolves_pointers_windows_34 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$id": "file:///c:/folder/file.json",
            "$defs": {
                "foo": {
                    "type": "number"
                }
            },
            "$ref": "#/$defs/foo"
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "$id with file URI still resolves pointers - windows";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_number_is_valid() {
        let description = "number is valid";
        let data = "1";
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
    fn test1_non_number_is_invalid() {
        let description = "non-number is invalid";
        let data = "\"a\"";
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
mod empty_tokens_in_ref_json_pointer_35 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$defs": {
                "": {
                    "$defs": {
                        "": { "type": "number" }
                    }
                } 
            },
            "allOf": [
                {
                    "$ref": "#/$defs//$defs/"
                }
            ]
        }"##;
    const URI: &str = "http://localhost:1234/ref.json";
    const DESCRIPTION: &str = "empty tokens in $ref json-pointer";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ref.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_number_is_valid() {
        let description = "number is valid";
        let data = "1";
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
    fn test1_non_number_is_invalid() {
        let description = "non-number is invalid";
        let data = "\"a\"";
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
