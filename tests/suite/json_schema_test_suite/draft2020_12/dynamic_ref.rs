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
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/dynamicRef-dynamicAnchor-same-schema/root\",\n            \"type\": \"array\",\n            \"items\": { \"$dynamicRef\": \"#items\" },\n            \"$defs\": {\n                \"foo\": {\n                    \"$dynamicAnchor\": \"items\",\n                    \"type\": \"string\"\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", \"bar\"]";
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
        assert_eq!(output.valid(), true, "expected ")
    }
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", 42]";
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
        assert_eq!(output.valid(), false, "expected ")
    }
}
mod a_dynamic_ref_to_an_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_an_anchor_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/dynamicRef-anchor-same-schema/root\",\n            \"type\": \"array\",\n            \"items\": { \"$dynamicRef\": \"#items\" },\n            \"$defs\": {\n                \"foo\": {\n                    \"$anchor\": \"items\",\n                    \"type\": \"string\"\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", \"bar\"]";
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
        assert_eq!(output.valid(), true, "expected ")
    }
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", 42]";
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
        assert_eq!(output.valid(), false, "expected ")
    }
}
mod a_ref_to_a_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_an_anchor_2 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/ref-dynamicAnchor-same-schema/root\",\n            \"type\": \"array\",\n            \"items\": { \"$ref\": \"#items\" },\n            \"$defs\": {\n                \"foo\": {\n                    \"$dynamicAnchor\": \"items\",\n                    \"type\": \"string\"\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", \"bar\"]";
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
        assert_eq!(output.valid(), true, "expected ")
    }
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", 42]";
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
        assert_eq!(output.valid(), false, "expected ")
    }
}
mod a_dynamic_ref_resolves_to_the_first_dynamic_anchor_still_in_scope_that_is_encountered_when_the_schema_is_evaluated_3 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/typical-dynamic-resolution/root\",\n            \"$ref\": \"list\",\n            \"$defs\": {\n                \"foo\": {\n                    \"$dynamicAnchor\": \"items\",\n                    \"type\": \"string\"\n                },\n                \"list\": {\n                    \"$id\": \"list\",\n                    \"type\": \"array\",\n                    \"items\": { \"$dynamicRef\": \"#items\" },\n                    \"$defs\": {\n                      \"items\": {\n                          \"$comment\": \"This is only needed to satisfy the bookending requirement\",\n                          \"$dynamicAnchor\": \"items\"\n                      }\n                    }\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", \"bar\"]";
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
        assert_eq!(output.valid(), true, "expected ")
    }
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", 42]";
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
        assert_eq!(output.valid(), false, "expected ")
    }
}
mod a_dynamic_ref_without_anchor_in_fragment_behaves_identical_to_ref_4 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/dynamicRef-without-anchor/root\",\n            \"$ref\": \"list\",\n            \"$defs\": {\n                \"foo\": {\n                    \"$dynamicAnchor\": \"items\",\n                    \"type\": \"string\"\n                },\n                \"list\": {\n                    \"$id\": \"list\",\n                    \"type\": \"array\",\n                    \"items\": { \"$dynamicRef\": \"#/$defs/items\" },\n                    \"$defs\": {\n                      \"items\": {\n                          \"$comment\": \"This is only needed to satisfy the bookending requirement\",\n                          \"$dynamicAnchor\": \"items\",\n                          \"type\": \"number\"\n                      }\n                    }\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", \"bar\"]";
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test1_an_array_of_numbers_is_valid() {
        let description = "An array of numbers is valid";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[24, 42]";
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
mod a_dynamic_ref_with_intermediate_scopes_that_don_t_include_a_matching_dynamic_anchor_does_not_affect_dynamic_scope_resolution_5 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/dynamic-resolution-with-intermediate-scopes/root\",\n            \"$ref\": \"intermediate-scope\",\n            \"$defs\": {\n                \"foo\": {\n                    \"$dynamicAnchor\": \"items\",\n                    \"type\": \"string\"\n                },\n                \"intermediate-scope\": {\n                    \"$id\": \"intermediate-scope\",\n                    \"$ref\": \"list\"\n                },\n                \"list\": {\n                    \"$id\": \"list\",\n                    \"type\": \"array\",\n                    \"items\": { \"$dynamicRef\": \"#items\" },\n                    \"$defs\": {\n                      \"items\": {\n                          \"$comment\": \"This is only needed to satisfy the bookending requirement\",\n                          \"$dynamicAnchor\": \"items\"\n                      }\n                    }\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", \"bar\"]";
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
        assert_eq!(output.valid(), true, "expected ")
    }
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {
        let description = "An array containing non-strings is invalid";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", 42]";
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
        assert_eq!(output.valid(), false, "expected ")
    }
}
mod an_anchor_with_the_same_name_as_a_dynamic_anchor_is_not_used_for_dynamic_scope_resolution_6 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/dynamic-resolution-ignores-anchors/root\",\n            \"$ref\": \"list\",\n            \"$defs\": {\n                \"foo\": {\n                    \"$anchor\": \"items\",\n                    \"type\": \"string\"\n                },\n                \"list\": {\n                    \"$id\": \"list\",\n                    \"type\": \"array\",\n                    \"items\": { \"$dynamicRef\": \"#items\" },\n                    \"$defs\": {\n                      \"items\": {\n                          \"$comment\": \"This is only needed to satisfy the bookending requirement\",\n                          \"$dynamicAnchor\": \"items\"\n                      }\n                    }\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", 42]";
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
mod a_dynamic_ref_without_a_matching_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_anchor_7 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/dynamic-resolution-without-bookend/root\",\n            \"$ref\": \"list\",\n            \"$defs\": {\n                \"foo\": {\n                    \"$dynamicAnchor\": \"items\",\n                    \"type\": \"string\"\n                },\n                \"list\": {\n                    \"$id\": \"list\",\n                    \"type\": \"array\",\n                    \"items\": { \"$dynamicRef\": \"#items\" },\n                    \"$defs\": {\n                        \"items\": {\n                            \"$comment\": \"This is only needed to give the reference somewhere to resolve to when it behaves like $ref\",\n                            \"$anchor\": \"items\"\n                        }\n                    }\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", 42]";
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
mod a_dynamic_ref_with_a_non_matching_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_anchor_8 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/unmatched-dynamic-anchor/root\",\n            \"$ref\": \"list\",\n            \"$defs\": {\n                \"foo\": {\n                    \"$dynamicAnchor\": \"items\",\n                    \"type\": \"string\"\n                },\n                \"list\": {\n                    \"$id\": \"list\",\n                    \"type\": \"array\",\n                    \"items\": { \"$dynamicRef\": \"#items\" },\n                    \"$defs\": {\n                        \"items\": {\n                            \"$comment\": \"This is only needed to give the reference somewhere to resolve to when it behaves like $ref\",\n                            \"$anchor\": \"items\",\n                            \"$dynamicAnchor\": \"foo\"\n                        }\n                    }\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "[\"foo\", 42]";
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
mod a_dynamic_ref_that_initially_resolves_to_a_schema_with_a_matching_dynamic_anchor_resolves_to_the_first_dynamic_anchor_in_the_dynamic_scope_9 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/relative-dynamic-reference/root\",\n            \"$dynamicAnchor\": \"meta\",\n            \"type\": \"object\",\n            \"properties\": {\n                \"foo\": { \"const\": \"pass\" }\n            },\n            \"$ref\": \"extended\",\n            \"$defs\": {\n                \"extended\": {\n                    \"$id\": \"extended\",\n                    \"$dynamicAnchor\": \"meta\",\n                    \"type\": \"object\",\n                    \"properties\": {\n                        \"bar\": { \"$ref\": \"bar\" }\n                    }\n                },\n                \"bar\": {\n                    \"$id\": \"bar\",\n                    \"type\": \"object\",\n                    \"properties\": {\n                        \"baz\": { \"$dynamicRef\": \"extended#meta\" }\n                    }\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"foo\": \"pass\",\n                    \"bar\": {\n                        \"baz\": { \"foo\": \"pass\" }\n                    }\n                }" ;
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
        assert_eq!(output.valid(), true, "expected ")
    }
    #[test]
    fn test1_the_recursive_part_is_not_valid_against_the_root() {
        let description = "The recursive part is not valid against the root";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"foo\": \"pass\",\n                    \"bar\": {\n                        \"baz\": { \"foo\": \"fail\" }\n                    }\n                }" ;
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
        assert_eq!(output.valid(), false, "expected ")
    }
}
mod a_dynamic_ref_that_initially_resolves_to_a_schema_without_a_matching_dynamic_anchor_behaves_like_a_normal_ref_to_anchor_10 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/relative-dynamic-reference-without-bookend/root\",\n            \"$dynamicAnchor\": \"meta\",\n            \"type\": \"object\",\n            \"properties\": {\n                \"foo\": { \"const\": \"pass\" }\n            },\n            \"$ref\": \"extended\",\n            \"$defs\": {\n                \"extended\": {\n                    \"$id\": \"extended\",\n                    \"$anchor\": \"meta\",\n                    \"type\": \"object\",\n                    \"properties\": {\n                        \"bar\": { \"$ref\": \"bar\" }\n                    }\n                },\n                \"bar\": {\n                    \"$id\": \"bar\",\n                    \"type\": \"object\",\n                    \"properties\": {\n                        \"baz\": { \"$dynamicRef\": \"extended#meta\" }\n                    }\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"foo\": \"pass\",\n                    \"bar\": {\n                        \"baz\": { \"foo\": \"fail\" }\n                    }\n                }" ;
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
mod multiple_dynamic_paths_to_the_dynamic_ref_keyword_11 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/dynamic-ref-with-multiple-paths/main\",\n            \"$defs\": {\n                \"inner\": {\n                    \"$id\": \"inner\",\n                    \"$dynamicAnchor\": \"foo\",\n                    \"title\": \"inner\",\n                    \"additionalProperties\": {\n                        \"$dynamicRef\": \"#foo\"\n                    }\n                }\n            },\n            \"if\": {\n                \"propertyNames\": {\n                    \"pattern\": \"^[a-m]\"\n                }\n            },\n            \"then\": {\n                \"title\": \"any type of node\",\n                \"$id\": \"anyLeafNode\",\n                \"$dynamicAnchor\": \"foo\",\n                \"$ref\": \"inner\"\n            },\n            \"else\": {\n                \"title\": \"integer node\",\n                \"$id\": \"integerNode\",\n                \"$dynamicAnchor\": \"foo\",\n                \"type\": [ \"object\", \"integer\" ],\n                \"$ref\": \"inner\"\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{ \"alpha\": 1.1 }";
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
        assert_eq!(output.valid(), true, "expected ")
    }
    #[test]
    fn test1_recurse_to_integer_node_floats_are_not_allowed() {
        let description = "recurse to integerNode - floats are not allowed";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{ \"november\": 1.1 }";
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
        assert_eq!(output.valid(), false, "expected ")
    }
}
mod after_leaving_a_dynamic_scope_it_is_not_used_by_a_dynamic_ref_12 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"https://test.json-schema.org/dynamic-ref-leaving-dynamic-scope/main\",\n            \"if\": {\n                \"$id\": \"first_scope\",\n                \"$defs\": {\n                    \"thingy\": {\n                        \"$comment\": \"this is first_scope#thingy\",\n                        \"$dynamicAnchor\": \"thingy\",\n                        \"type\": \"number\"\n                    }\n                }\n            },\n            \"then\": {\n                \"$id\": \"second_scope\",\n                \"$ref\": \"start\",\n                \"$defs\": {\n                    \"thingy\": {\n                        \"$comment\": \"this is second_scope#thingy, the final destination of the $dynamicRef\",\n                        \"$dynamicAnchor\": \"thingy\",\n                        \"type\": \"null\"\n                    }\n                }\n            },\n            \"$defs\": {\n                \"start\": {\n                    \"$comment\": \"this is the landing spot from $ref\",\n                    \"$id\": \"start\",\n                    \"$dynamicRef\": \"inner_scope#thingy\"\n                },\n                \"thingy\": {\n                    \"$comment\": \"this is the first stop for the $dynamicRef\",\n                    \"$id\": \"inner_scope\",\n                    \"$dynamicAnchor\": \"thingy\",\n                    \"type\": \"string\"\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "\"a string\"";
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test1_first_scope_is_not_in_dynamic_scope_for_the_dynamic_ref() {
        let description = "first_scope is not in dynamic scope for the $dynamicRef";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "42";
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test2_then_defs_thingy_is_the_final_stop_for_the_dynamic_ref() {
        let description = "/then/$defs/thingy is the final stop for the $dynamicRef";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "null";
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
mod strict_tree_schema_guards_against_misspelled_properties_13 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"http://localhost:1234/draft2020-12/strict-tree.json\",\n            \"$dynamicAnchor\": \"node\",\n\n            \"$ref\": \"tree.json\",\n            \"unevaluatedProperties\": false\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"children\": [{\n                            \"daat\": 1\n                        }]\n                }" ;
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test1_instance_with_correct_field() {
        let description = "instance with correct field";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"children\": [{\n                            \"data\": 1\n                        }]\n                }" ;
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
mod tests_for_implementation_dynamic_anchor_and_reference_link_14 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"http://localhost:1234/draft2020-12/strict-extendible.json\",\n            \"$ref\": \"extendible-dynamic-ref.json\",\n            \"$defs\": {\n                \"elements\": {\n                    \"$dynamicAnchor\": \"elements\",\n                    \"properties\": {\n                        \"a\": true\n                    },\n                    \"required\": [\"a\"],\n                    \"additionalProperties\": false\n                }\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"a\": true\n                }";
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test1_incorrect_extended_schema() {
        let description = "incorrect extended schema";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"elements\": [\n                        { \"b\": 1 }\n                    ]\n                }" ;
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test2_correct_extended_schema() {
        let description = "correct extended schema";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"elements\": [\n                        { \"a\": 1 }\n                    ]\n                }" ;
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
mod ref_and_dynamic_anchor_are_independent_of_order_defs_first_15 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"http://localhost:1234/draft2020-12/strict-extendible-allof-defs-first.json\",\n            \"allOf\": [\n                {\n                    \"$ref\": \"extendible-dynamic-ref.json\"\n                },\n                {\n                    \"$defs\": {\n                        \"elements\": {\n                            \"$dynamicAnchor\": \"elements\",\n                            \"properties\": {\n                                \"a\": true\n                            },\n                            \"required\": [\"a\"],\n                            \"additionalProperties\": false\n                        }\n                    }\n                }\n            ]\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"a\": true\n                }";
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test1_incorrect_extended_schema() {
        let description = "incorrect extended schema";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"elements\": [\n                        { \"b\": 1 }\n                    ]\n                }" ;
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test2_correct_extended_schema() {
        let description = "correct extended schema";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"elements\": [\n                        { \"a\": 1 }\n                    ]\n                }" ;
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
mod ref_and_dynamic_anchor_are_independent_of_order_ref_first_16 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$id\": \"http://localhost:1234/draft2020-12/strict-extendible-allof-ref-first.json\",\n            \"allOf\": [\n                {\n                    \"$defs\": {\n                        \"elements\": {\n                            \"$dynamicAnchor\": \"elements\",\n                            \"properties\": {\n                                \"a\": true\n                            },\n                            \"required\": [\"a\"],\n                            \"additionalProperties\": false\n                        }\n                    }\n                },\n                {\n                    \"$ref\": \"extendible-dynamic-ref.json\"\n                }\n            ]\n        }" ;
        const URI: &str = "http://localhost:1234/dynamicRef.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"a\": true\n                }";
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test1_incorrect_extended_schema() {
        let description = "incorrect extended schema";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"elements\": [\n                        { \"b\": 1 }\n                    ]\n                }" ;
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
        assert_eq!(output.valid(), false, "expected ")
    }
    #[test]
    fn test2_correct_extended_schema() {
        let description = "correct extended schema";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"elements\": [\n                        { \"a\": 1 }\n                    ]\n                }" ;
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
        assert_eq!(output.valid(), true, "expected ")
    }
}
