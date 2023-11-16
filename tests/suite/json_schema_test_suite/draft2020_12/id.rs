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
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$ref\": \"https://json-schema.org/draft/2020-12/schema\"\n        }" ;
        const URI: &str = "http://localhost:1234/id.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$ref\": \"#foo\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"#foo\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
    fn test1_identifier_name_and_no_ref() {
        let description = "Identifier name and no ref";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$defs\": {\n                        \"A\": { \"$id\": \"#foo\" }\n                    }\n                }" ;
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
    fn test2_identifier_path() {
        let description = "Identifier path";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$ref\": \"#/a/b\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"#/a/b\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
    fn test3_identifier_name_with_absolute_uri() {
        let description = "Identifier name with absolute URI";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/bar#foo\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/bar#foo\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
    fn test4_identifier_path_with_absolute_uri() {
        let description = "Identifier path with absolute URI";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/bar#/a/b\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/bar#/a/b\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
    fn test5_identifier_name_with_base_uri_change_in_subschema() {
        let description = "Identifier name with base URI change in subschema";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$id\": \"http://localhost:1234/draft2020-12/root\",\n                    \"$ref\": \"http://localhost:1234/draft2020-12/nested.json#foo\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"nested.json\",\n                            \"$defs\": {\n                                \"B\": {\n                                    \"$id\": \"#foo\",\n                                    \"type\": \"integer\"\n                                }\n                            }\n                        }\n                    }\n                }" ;
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
    fn test6_identifier_path_with_base_uri_change_in_subschema() {
        let description = "Identifier path with base URI change in subschema";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$id\": \"http://localhost:1234/draft2020-12/root\",\n                    \"$ref\": \"http://localhost:1234/draft2020-12/nested.json#/a/b\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"nested.json\",\n                            \"$defs\": {\n                                \"B\": {\n                                    \"$id\": \"#/a/b\",\n                                    \"type\": \"integer\"\n                                }\n                            }\n                        }\n                    }\n                }" ;
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
mod valid_use_of_empty_fragments_in_location_independent_id_1 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$ref\": \"https://json-schema.org/draft/2020-12/schema\"\n        }" ;
        const URI: &str = "http://localhost:1234/id.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/bar\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/bar#\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
    fn test1_identifier_name_with_base_uri_change_in_subschema() {
        let description = "Identifier name with base URI change in subschema";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$id\": \"http://localhost:1234/draft2020-12/root\",\n                    \"$ref\": \"http://localhost:1234/draft2020-12/nested.json#/$defs/B\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"nested.json\",\n                            \"$defs\": {\n                                \"B\": {\n                                    \"$id\": \"#\",\n                                    \"type\": \"integer\"\n                                }\n                            }\n                        }\n                    }\n                }" ;
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
mod unnormalized_ids_are_allowed_but_discouraged_2 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$ref\": \"https://json-schema.org/draft/2020-12/schema\"\n        }" ;
        const URI: &str = "http://localhost:1234/id.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/foo/baz\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/foo/bar/../baz\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
    fn test1_unnormalized_identifier_and_no_ref() {
        let description = "Unnormalized identifier and no ref";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/foo/bar/../baz\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
    fn test2_unnormalized_identifier_with_empty_fragment() {
        let description = "Unnormalized identifier with empty fragment";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$ref\": \"http://localhost:1234/draft2020-12/foo/baz\",\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/foo/bar/../baz#\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
    fn test3_unnormalized_identifier_with_empty_fragment_and_no_ref() {
        let description = "Unnormalized identifier with empty fragment and no ref";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$defs\": {\n                        \"A\": {\n                            \"$id\": \"http://localhost:1234/draft2020-12/foo/bar/../baz#\",\n                            \"type\": \"integer\"\n                        }\n                    }\n                }" ;
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
mod id_inside_an_enum_is_not_a_real_identifier_3 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$defs\": {\n                \"id_in_enum\": {\n                    \"enum\": [\n                        {\n                          \"$id\": \"https://localhost:1234/draft2020-12/id/my_identifier.json\",\n                          \"type\": \"null\"\n                        }\n                    ]\n                },\n                \"real_id_in_schema\": {\n                    \"$id\": \"https://localhost:1234/draft2020-12/id/my_identifier.json\",\n                    \"type\": \"string\"\n                },\n                \"zzz_id_in_const\": {\n                    \"const\": {\n                        \"$id\": \"https://localhost:1234/draft2020-12/id/my_identifier.json\",\n                        \"type\": \"null\"\n                    }\n                }\n            },\n            \"anyOf\": [\n                { \"$ref\": \"#/$defs/id_in_enum\" },\n                { \"$ref\": \"https://localhost:1234/draft2020-12/id/my_identifier.json\" }\n            ]\n        }" ;
        const URI: &str = "http://localhost:1234/id.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{\n                    \"$id\": \"https://localhost:1234/draft2020-12/id/my_identifier.json\",\n                    \"type\": \"null\"\n                }" ;
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
    fn test1_match_ref_to_id() {
        let description = "match $ref to $id";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "\"a string to match #/$defs/id_in_enum\"";
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
    fn test2_no_match_on_enum_or_ref_to_id() {
        let description = "no match on enum or $ref to $id";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "1";
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
mod non_schema_object_containing_an_id_property_4 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$defs\": {\n                \"const_not_id\": {\n                    \"const\": {\n                        \"$id\": \"not_a_real_id\"\n                    }\n                }\n            },\n            \"if\": {\n                \"const\": \"skip not_a_real_id\"\n            },\n            \"then\": true,\n            \"else\" : {\n                \"$ref\": \"#/$defs/const_not_id\"\n            }\n        }" ;
        const URI: &str = "http://localhost:1234/id.json";
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
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "\"skip not_a_real_id\"";
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
    fn test1_const_at_const_not_id_does_not_match() {
        let description = "const at const_not_id does not match";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "1";
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
