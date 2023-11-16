use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_infinite_loop_detection(interrogator)
    }
    interrogator
}
mod evaluating_the_same_schema_location_against_the_same_data_location_twice_is_not_a_sign_of_an_infinite_loop_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
        use std::sync::OnceLock;
        const SCHEMA : & str = "{\n            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n            \"$defs\": {\n                \"int\": { \"type\": \"integer\" }\n            },\n            \"allOf\": [\n                {\n                    \"properties\": {\n                        \"foo\": {\n                            \"$ref\": \"#/$defs/int\"\n                        }\n                    }\n                },\n                {\n                    \"additionalProperties\": {\n                        \"$ref\": \"#/$defs/int\"\n                    }\n                }\n            ]\n        }" ;
        const URI: &str = "http://localhost:1234/infinite-loop-detection.json";
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
                let key = block_on(
                    interrogator.compile("http://localhost:1234/infinite-loop-detection.json"),
                )?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_passing_case() {
        let description = "passing case";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{ \"foo\": 1 }";
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
    fn test1_failing_case() {
        let description = "failing case";
        let (key, interrogator) = match setup() {
            Ok((key, interrogator)) => (key, interrogator),
            Err(err) => {
                panic!("failed to setup test for {}\n:{}", description, err);
            }
        };
        let data = "{ \"foo\": \"a string\" }";
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
