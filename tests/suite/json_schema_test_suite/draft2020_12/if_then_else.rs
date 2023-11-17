use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_if_then_else(interrogator)
    }
    interrogator
}
mod ignore_if_without_then_or_else_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": {
                "const": 0
            }
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "ignore if without then or else";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid_when_valid_against_lone_if() {
        use super::DESCRIPTION;
        let description = "valid when valid against lone if";
        let data = "0";
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
    fn test1_valid_when_invalid_against_lone_if() {
        use super::DESCRIPTION;
        let description = "valid when invalid against lone if";
        let data = "\"hello\"";
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
mod ignore_then_without_if_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "then": {
                "const": 0
            }
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "ignore then without if";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid_when_valid_against_lone_then() {
        use super::DESCRIPTION;
        let description = "valid when valid against lone then";
        let data = "0";
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
    fn test1_valid_when_invalid_against_lone_then() {
        use super::DESCRIPTION;
        let description = "valid when invalid against lone then";
        let data = "\"hello\"";
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
mod ignore_else_without_if_2 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "else": {
                "const": 0
            }
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "ignore else without if";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid_when_valid_against_lone_else() {
        use super::DESCRIPTION;
        let description = "valid when valid against lone else";
        let data = "0";
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
    fn test1_valid_when_invalid_against_lone_else() {
        use super::DESCRIPTION;
        let description = "valid when invalid against lone else";
        let data = "\"hello\"";
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
mod if_and_then_without_else_3 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": {
                "exclusiveMaximum": 0
            },
            "then": {
                "minimum": -10
            }
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "if and then without else";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid_through_then() {
        use super::DESCRIPTION;
        let description = "valid through then";
        let data = "-1";
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
    fn test1_invalid_through_then() {
        use super::DESCRIPTION;
        let description = "invalid through then";
        let data = "-100";
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
    fn test2_valid_when_if_test_fails() {
        use super::DESCRIPTION;
        let description = "valid when if test fails";
        let data = "3";
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
mod if_and_else_without_then_4 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": {
                "exclusiveMaximum": 0
            },
            "else": {
                "multipleOf": 2
            }
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "if and else without then";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid_when_if_test_passes() {
        use super::DESCRIPTION;
        let description = "valid when if test passes";
        let data = "-1";
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
    fn test1_valid_through_else() {
        use super::DESCRIPTION;
        let description = "valid through else";
        let data = "4";
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
    fn test2_invalid_through_else() {
        use super::DESCRIPTION;
        let description = "invalid through else";
        let data = "3";
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
mod validate_against_correct_branch_then_vs_else_5 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": {
                "exclusiveMaximum": 0
            },
            "then": {
                "minimum": -10
            },
            "else": {
                "multipleOf": 2
            }
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "validate against correct branch, then vs else";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid_through_then() {
        use super::DESCRIPTION;
        let description = "valid through then";
        let data = "-1";
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
    fn test1_invalid_through_then() {
        use super::DESCRIPTION;
        let description = "invalid through then";
        let data = "-100";
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
    fn test2_valid_through_else() {
        use super::DESCRIPTION;
        let description = "valid through else";
        let data = "4";
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
    fn test3_invalid_through_else() {
        use super::DESCRIPTION;
        let description = "invalid through else";
        let data = "3";
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
mod non_interference_across_combined_schemas_6 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "allOf": [
                {
                    "if": {
                        "exclusiveMaximum": 0
                    }
                },
                {
                    "then": {
                        "minimum": -10
                    }
                },
                {
                    "else": {
                        "multipleOf": 2
                    }
                }
            ]
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "non-interference across combined schemas";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_valid_but_would_have_been_invalid_through_then() {
        use super::DESCRIPTION;
        let description = "valid, but would have been invalid through then";
        let data = "-100";
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
    fn test1_valid_but_would_have_been_invalid_through_else() {
        use super::DESCRIPTION;
        let description = "valid, but would have been invalid through else";
        let data = "3";
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
mod if_with_boolean_schema_true_7 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": true,
            "then": { "const": "then" },
            "else": { "const": "else" }
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "if with boolean schema true";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_boolean_schema_true_in_if_always_chooses_the_then_path_valid() {
        use super::DESCRIPTION;
        let description = "boolean schema true in if always chooses the then path (valid)";
        let data = "\"then\"";
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
    fn test1_boolean_schema_true_in_if_always_chooses_the_then_path_invalid() {
        use super::DESCRIPTION;
        let description = "boolean schema true in if always chooses the then path (invalid)";
        let data = "\"else\"";
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
mod if_with_boolean_schema_false_8 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "if": false,
            "then": { "const": "then" },
            "else": { "const": "else" }
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "if with boolean schema false";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_boolean_schema_false_in_if_always_chooses_the_else_path_invalid() {
        use super::DESCRIPTION;
        let description = "boolean schema false in if always chooses the else path (invalid)";
        let data = "\"then\"";
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
    fn test1_boolean_schema_false_in_if_always_chooses_the_else_path_valid() {
        use super::DESCRIPTION;
        let description = "boolean schema false in if always chooses the else path (valid)";
        let data = "\"else\"";
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
mod if_appears_at_the_end_when_serialized_keyword_processing_sequence_9 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "then": { "const": "yes" },
            "else": { "const": "other" },
            "if": { "maxLength": 4 }
        }"##;
    const URI: &str = "http://localhost:1234/if-then-else.json";
    const DESCRIPTION: &str = "if appears at the end when serialized (keyword processing sequence)";
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
                    block_on(interrogator.compile("http://localhost:1234/if-then-else.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_yes_redirects_to_then_and_passes() {
        use super::DESCRIPTION;
        let description = "yes redirects to then and passes";
        let data = "\"yes\"";
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
    fn test1_other_redirects_to_else_and_passes() {
        use super::DESCRIPTION;
        let description = "other redirects to else and passes";
        let data = "\"other\"";
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
    fn test2_no_redirects_to_then_and_fails() {
        use super::DESCRIPTION;
        let description = "no redirects to then and fails";
        let data = "\"no\"";
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
    fn test3_invalid_redirects_to_else_and_fails() {
        use super::DESCRIPTION;
        let description = "invalid redirects to else and fails";
        let data = "\"invalid\"";
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
