use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_pattern(interrogator)
    }
    interrogator
}
mod pattern_validation_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "pattern": "^a*$"
        }"##;
    const URI: &str = "http://localhost:1234/pattern.json";
    const DESCRIPTION: &str = "pattern validation";
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
                let key = block_on(interrogator.compile("http://localhost:1234/pattern.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_a_matching_pattern_is_valid() {
        use super::DESCRIPTION;
        let description = "a matching pattern is valid";
        let data = "\"aaa\"";
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
    fn test1_a_non_matching_pattern_is_invalid() {
        use super::DESCRIPTION;
        let description = "a non-matching pattern is invalid";
        let data = "\"abc\"";
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
    fn test2_ignores_booleans() {
        use super::DESCRIPTION;
        let description = "ignores booleans";
        let data = "true";
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
    fn test3_ignores_integers() {
        use super::DESCRIPTION;
        let description = "ignores integers";
        let data = "123";
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
    fn test4_ignores_floats() {
        use super::DESCRIPTION;
        let description = "ignores floats";
        let data = "1.0";
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
    fn test5_ignores_objects() {
        use super::DESCRIPTION;
        let description = "ignores objects";
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
    fn test6_ignores_arrays() {
        use super::DESCRIPTION;
        let description = "ignores arrays";
        let data = "[]";
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
    fn test7_ignores_null() {
        use super::DESCRIPTION;
        let description = "ignores null";
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
        assert_eq ! (output . valid () , expected_valid , "expected {expected_valid} for: \n\tcase: {DESCRIPTION}\n\ttest: {description}\n\tschema:\n{SCHEMA}\n\tdata:\n{data}")
    }
}
mod pattern_is_not_anchored_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "pattern": "a+"
        }"##;
    const URI: &str = "http://localhost:1234/pattern.json";
    const DESCRIPTION: &str = "pattern is not anchored";
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
                let key = block_on(interrogator.compile("http://localhost:1234/pattern.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_matches_a_substring() {
        use super::DESCRIPTION;
        let description = "matches a substring";
        let data = "\"xxaayy\"";
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
