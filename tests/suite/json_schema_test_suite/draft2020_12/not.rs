use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_not(interrogator)
    }
    interrogator
}
mod not_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "not": {"type": "integer"}
        }"##;
    const URI: &str = "http://localhost:1234/not.json";
    const DESCRIPTION: &str = "not";
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
                let key = block_on(interrogator.compile("http://localhost:1234/not.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_allowed() {
        use super::DESCRIPTION;
        let description = "allowed";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
    #[test]
    fn test1_disallowed() {
        use super::DESCRIPTION;
        let description = "disallowed";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
}
mod not_multiple_types_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "not": {"type": ["integer", "boolean"]}
        }"##;
    const URI: &str = "http://localhost:1234/not.json";
    const DESCRIPTION: &str = "not multiple types";
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
                let key = block_on(interrogator.compile("http://localhost:1234/not.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid() {
        use super::DESCRIPTION;
        let description = "valid";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
    #[test]
    fn test1_mismatch() {
        use super::DESCRIPTION;
        let description = "mismatch";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
    #[test]
    fn test2_other_mismatch() {
        use super::DESCRIPTION;
        let description = "other mismatch";
        let data = "true";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
}
mod not_more_complex_schema_2 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "not": {
                "type": "object",
                "properties": {
                    "foo": {
                        "type": "string"
                    }
                }
             }
        }"##;
    const URI: &str = "http://localhost:1234/not.json";
    const DESCRIPTION: &str = "not more complex schema";
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
                let key = block_on(interrogator.compile("http://localhost:1234/not.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_match_() {
        use super::DESCRIPTION;
        let description = "match";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
    #[test]
    fn test1_other_match() {
        use super::DESCRIPTION;
        let description = "other match";
        let data = "{\"foo\": 1}";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
    #[test]
    fn test2_mismatch() {
        use super::DESCRIPTION;
        let description = "mismatch";
        let data = "{\"foo\": \"bar\"}";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
}
mod forbidden_property_3 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "properties": {
                "foo": { 
                    "not": {}
                }
            }
        }"##;
    const URI: &str = "http://localhost:1234/not.json";
    const DESCRIPTION: &str = "forbidden property";
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
                let key = block_on(interrogator.compile("http://localhost:1234/not.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_property_present() {
        use super::DESCRIPTION;
        let description = "property present";
        let data = "{\"foo\": 1, \"bar\": 2}";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
    #[test]
    fn test1_property_absent() {
        use super::DESCRIPTION;
        let description = "property absent";
        let data = "{\"bar\": 1, \"baz\": 2}";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
}
mod not_with_boolean_schema_true_4 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "not": true
        }"##;
    const URI: &str = "http://localhost:1234/not.json";
    const DESCRIPTION: &str = "not with boolean schema true";
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
                let key = block_on(interrogator.compile("http://localhost:1234/not.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_any_value_is_invalid() {
        use super::DESCRIPTION;
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
}
mod not_with_boolean_schema_false_5 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "not": false
        }"##;
    const URI: &str = "http://localhost:1234/not.json";
    const DESCRIPTION: &str = "not with boolean schema false";
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
                let key = block_on(interrogator.compile("http://localhost:1234/not.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_any_value_is_valid() {
        use super::DESCRIPTION;
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
}
mod collect_annotations_inside_a_not_even_if_collection_is_disabled_6 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "not": {
                "$comment": "this subschema must still produce annotations internally, even though the 'not' will ultimately discard them",
                "anyOf": [
                    true,
                    { "properties": { "foo": true } }
                ],
                "unevaluatedProperties": false
            }
        }"##;
    const URI: &str = "http://localhost:1234/not.json";
    const DESCRIPTION: &str = "collect annotations inside a 'not', even if collection is disabled";
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
                let key = block_on(interrogator.compile("http://localhost:1234/not.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_unevaluated_property() {
        use super::DESCRIPTION;
        let description = "unevaluated property";
        let data = "{ \"bar\": 1 }";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
    #[test]
    fn test1_annotations_are_still_collected_inside_a_not() {
        use super::DESCRIPTION;
        let description = "annotations are still collected inside a 'not'";
        let data = "{ \"foo\": 1 }";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
}
