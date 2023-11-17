use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_property_names(interrogator)
    }
    interrogator
}
mod property_names_validation_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "propertyNames": {"maxLength": 3}
        }"##;
    const URI: &str = "http://localhost:1234/propertyNames.json";
    const DESCRIPTION: &str = "propertyNames validation";
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
                    block_on(interrogator.compile("http://localhost:1234/propertyNames.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_all_property_names_valid() {
        use super::DESCRIPTION;
        let description = "all property names valid";
        let data =
            "{\n                    \"f\": {},\n                    \"foo\": {}\n                }";
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
    fn test1_some_property_names_invalid() {
        use super::DESCRIPTION;
        let description = "some property names invalid";
        let data = "{\n                    \"foo\": {},\n                    \"foobar\": {}\n                }" ;
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
    fn test2_object_without_properties_is_valid() {
        use super::DESCRIPTION;
        let description = "object without properties is valid";
        let data = "{}";
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
    fn test3_ignores_arrays() {
        use super::DESCRIPTION;
        let description = "ignores arrays";
        let data = "[1, 2, 3, 4]";
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
    fn test4_ignores_strings() {
        use super::DESCRIPTION;
        let description = "ignores strings";
        let data = "\"foobar\"";
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
    fn test5_ignores_other_non_objects() {
        use super::DESCRIPTION;
        let description = "ignores other non-objects";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
}
mod property_names_with_boolean_schema_true_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "propertyNames": true
        }"##;
    const URI: &str = "http://localhost:1234/propertyNames.json";
    const DESCRIPTION: &str = "propertyNames with boolean schema true";
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
                    block_on(interrogator.compile("http://localhost:1234/propertyNames.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_object_with_any_properties_is_valid() {
        use super::DESCRIPTION;
        let description = "object with any properties is valid";
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
    fn test1_empty_object_is_valid() {
        use super::DESCRIPTION;
        let description = "empty object is valid";
        let data = "{}";
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
mod property_names_with_boolean_schema_false_2 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "propertyNames": false
        }"##;
    const URI: &str = "http://localhost:1234/propertyNames.json";
    const DESCRIPTION: &str = "propertyNames with boolean schema false";
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
                    block_on(interrogator.compile("http://localhost:1234/propertyNames.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_object_with_any_properties_is_invalid() {
        use super::DESCRIPTION;
        let description = "object with any properties is invalid";
        let data = "{\"foo\": 1}";
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
    fn test1_empty_object_is_valid() {
        use super::DESCRIPTION;
        let description = "empty object is valid";
        let data = "{}";
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
