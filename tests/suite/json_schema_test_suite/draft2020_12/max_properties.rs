use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_max_properties(interrogator)
    }
    interrogator
}
mod max_properties_validation_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "maxProperties": 2
        }"##;
    const URI: &str = "http://localhost:1234/maxProperties.json";
    const DESCRIPTION: &str = "maxProperties validation";
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
                    block_on(interrogator.compile("http://localhost:1234/maxProperties.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_shorter_is_valid() {
        let description = "shorter is valid";
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
        let valid_msg = if expected_valid { "valid" } else { "invalid" };
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  expected: {valid_msg}\n  schema:{SCHEMA}\n  data: {data}")
    }
    #[test]
    fn test1_exact_length_is_valid() {
        let description = "exact length is valid";
        let data = "{\"foo\": 1, \"bar\": 2}";
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
    fn test2_too_long_is_invalid() {
        let description = "too long is invalid";
        let data = "{\"foo\": 1, \"bar\": 2, \"baz\": 3}";
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
    fn test3_ignores_arrays() {
        let description = "ignores arrays";
        let data = "[1, 2, 3]";
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
    fn test4_ignores_strings() {
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
        let valid_msg = if expected_valid { "valid" } else { "invalid" };
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  expected: {valid_msg}\n  schema:{SCHEMA}\n  data: {data}")
    }
    #[test]
    fn test5_ignores_other_non_objects() {
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
        let valid_msg = if expected_valid { "valid" } else { "invalid" };
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  expected: {valid_msg}\n  schema:{SCHEMA}\n  data: {data}")
    }
}
mod max_properties_validation_with_a_decimal_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "maxProperties": 2.0
        }"##;
    const URI: &str = "http://localhost:1234/maxProperties.json";
    const DESCRIPTION: &str = "maxProperties validation with a decimal";
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
                    block_on(interrogator.compile("http://localhost:1234/maxProperties.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_shorter_is_valid() {
        let description = "shorter is valid";
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
        let valid_msg = if expected_valid { "valid" } else { "invalid" };
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  expected: {valid_msg}\n  schema:{SCHEMA}\n  data: {data}")
    }
    #[test]
    fn test1_too_long_is_invalid() {
        let description = "too long is invalid";
        let data = "{\"foo\": 1, \"bar\": 2, \"baz\": 3}";
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
mod max_properties_0_means_the_object_is_empty_2 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "maxProperties": 0
        }"##;
    const URI: &str = "http://localhost:1234/maxProperties.json";
    const DESCRIPTION: &str = "maxProperties = 0 means the object is empty";
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
                    block_on(interrogator.compile("http://localhost:1234/maxProperties.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_no_properties_is_valid() {
        let description = "no properties is valid";
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
        let valid_msg = if expected_valid { "valid" } else { "invalid" };
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  expected: {valid_msg}\n  schema:{SCHEMA}\n  data: {data}")
    }
    #[test]
    fn test1_one_property_is_invalid() {
        let description = "one property is invalid";
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
        let valid_msg = if expected_valid { "valid" } else { "invalid" };
        assert_eq ! (output . valid () , expected_valid , "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  expected: {valid_msg}\n  schema:{SCHEMA}\n  data: {data}")
    }
}
