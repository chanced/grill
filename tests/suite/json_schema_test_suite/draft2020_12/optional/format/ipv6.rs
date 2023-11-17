use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_optional_format_ipv6(interrogator)
    }
    interrogator
}
mod validation_of_i_pv6_addresses_0 {
    use super::*;
    use grill::{error::CompileError, Key, Structure};
    const SCHEMA: &str = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "format": "ipv6"
        }"##;
    const URI: &str = "http://localhost:1234/ipv6.json";
    const DESCRIPTION: &str = "validation of IPv6 addresses";
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
                let key = block_on(interrogator.compile("http://localhost:1234/ipv6.json"))?;
                Ok((key, interrogator))
            })
            .as_ref()
            .map(Clone::clone)
    }
    #[test]
    fn test0_all_string_formats_ignore_integers() {
        use super::DESCRIPTION;
        let description = "all string formats ignore integers";
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
    #[test]
    fn test1_all_string_formats_ignore_floats() {
        use super::DESCRIPTION;
        let description = "all string formats ignore floats";
        let data = "13.7";
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
    fn test2_all_string_formats_ignore_objects() {
        use super::DESCRIPTION;
        let description = "all string formats ignore objects";
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
    fn test3_all_string_formats_ignore_arrays() {
        use super::DESCRIPTION;
        let description = "all string formats ignore arrays";
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
    fn test4_all_string_formats_ignore_booleans() {
        use super::DESCRIPTION;
        let description = "all string formats ignore booleans";
        let data = "false";
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
    fn test5_all_string_formats_ignore_nulls() {
        use super::DESCRIPTION;
        let description = "all string formats ignore nulls";
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
    #[test]
    fn test6_a_valid_i_pv6_address() {
        use super::DESCRIPTION;
        let description = "a valid IPv6 address";
        let data = "\"::1\"";
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
    fn test7_an_i_pv6_address_with_out_of_range_values() {
        use super::DESCRIPTION;
        let description = "an IPv6 address with out-of-range values";
        let data = "\"12345::\"";
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
    fn test8_trailing_4_hex_symbols_is_valid() {
        use super::DESCRIPTION;
        let description = "trailing 4 hex symbols is valid";
        let data = "\"::abef\"";
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
    fn test9_trailing_5_hex_symbols_is_invalid() {
        use super::DESCRIPTION;
        let description = "trailing 5 hex symbols is invalid";
        let data = "\"::abcef\"";
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
    fn test10_an_i_pv6_address_with_too_many_components() {
        use super::DESCRIPTION;
        let description = "an IPv6 address with too many components";
        let data = "\"1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1\"";
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
    fn test11_an_i_pv6_address_containing_illegal_characters() {
        use super::DESCRIPTION;
        let description = "an IPv6 address containing illegal characters";
        let data = "\"::laptop\"";
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
    fn test12_no_digits_is_valid() {
        use super::DESCRIPTION;
        let description = "no digits is valid";
        let data = "\"::\"";
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
    fn test13_leading_colons_is_valid() {
        use super::DESCRIPTION;
        let description = "leading colons is valid";
        let data = "\"::42:ff:1\"";
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
    fn test14_trailing_colons_is_valid() {
        use super::DESCRIPTION;
        let description = "trailing colons is valid";
        let data = "\"d6::\"";
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
    fn test15_missing_leading_octet_is_invalid() {
        use super::DESCRIPTION;
        let description = "missing leading octet is invalid";
        let data = "\":2:3:4:5:6:7:8\"";
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
    fn test16_missing_trailing_octet_is_invalid() {
        use super::DESCRIPTION;
        let description = "missing trailing octet is invalid";
        let data = "\"1:2:3:4:5:6:7:\"";
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
    fn test17_missing_leading_octet_with_omitted_octets_later() {
        use super::DESCRIPTION;
        let description = "missing leading octet with omitted octets later";
        let data = "\":2:3:4::8\"";
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
    fn test18_single_set_of_double_colons_in_the_middle_is_valid() {
        use super::DESCRIPTION;
        let description = "single set of double colons in the middle is valid";
        let data = "\"1:d6::42\"";
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
    fn test19_two_sets_of_double_colons_is_invalid() {
        use super::DESCRIPTION;
        let description = "two sets of double colons is invalid";
        let data = "\"1::d6::42\"";
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
    fn test20_mixed_format_with_the_ipv4_section_as_decimal_octets() {
        use super::DESCRIPTION;
        let description = "mixed format with the ipv4 section as decimal octets";
        let data = "\"1::d6:192.168.0.1\"";
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
    fn test21_mixed_format_with_double_colons_between_the_sections() {
        use super::DESCRIPTION;
        let description = "mixed format with double colons between the sections";
        let data = "\"1:2::192.168.0.1\"";
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
    fn test22_mixed_format_with_ipv4_section_with_octet_out_of_range() {
        use super::DESCRIPTION;
        let description = "mixed format with ipv4 section with octet out of range";
        let data = "\"1::2:192.168.256.1\"";
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
    fn test23_mixed_format_with_ipv4_section_with_a_hex_octet() {
        use super::DESCRIPTION;
        let description = "mixed format with ipv4 section with a hex octet";
        let data = "\"1::2:192.168.ff.1\"";
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
    fn test24_mixed_format_with_leading_double_colons_ipv4_mapped_ipv6_address() {
        use super::DESCRIPTION;
        let description = "mixed format with leading double colons (ipv4-mapped ipv6 address)";
        let data = "\"::ffff:192.168.0.1\"";
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
    fn test25_triple_colons_is_invalid() {
        use super::DESCRIPTION;
        let description = "triple colons is invalid";
        let data = "\"1:2:3:4:5:::8\"";
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
    fn test26_8_octets() {
        use super::DESCRIPTION;
        let description = "8 octets";
        let data = "\"1:2:3:4:5:6:7:8\"";
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
    fn test27_insufficient_octets_without_double_colons() {
        use super::DESCRIPTION;
        let description = "insufficient octets without double colons";
        let data = "\"1:2:3:4:5:6:7\"";
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
    fn test28_no_colons_is_invalid() {
        use super::DESCRIPTION;
        let description = "no colons is invalid";
        let data = "\"1\"";
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
    fn test29_ipv4_is_not_ipv6() {
        use super::DESCRIPTION;
        let description = "ipv4 is not ipv6";
        let data = "\"127.0.0.1\"";
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
    fn test30_ipv4_segment_must_have_4_octets() {
        use super::DESCRIPTION;
        let description = "ipv4 segment must have 4 octets";
        let data = "\"1:2:3:4:1.2.3\"";
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
    fn test31_leading_whitespace_is_invalid() {
        use super::DESCRIPTION;
        let description = "leading whitespace is invalid";
        let data = "\"  ::1\"";
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
    fn test32_trailing_whitespace_is_invalid() {
        use super::DESCRIPTION;
        let description = "trailing whitespace is invalid";
        let data = "\"::1  \"";
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
    fn test33_netmask_is_not_a_part_of_ipv6_address() {
        use super::DESCRIPTION;
        let description = "netmask is not a part of ipv6 address";
        let data = "\"fe80::/64\"";
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
    fn test34_zone_id_is_not_a_part_of_ipv6_address() {
        use super::DESCRIPTION;
        let description = "zone id is not a part of ipv6 address";
        let data = "\"fe80::a%eth1\"";
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
    fn test35_a_long_valid_ipv6() {
        use super::DESCRIPTION;
        let description = "a long valid ipv6";
        let data = "\"1000:1000:1000:1000:1000:1000:255.255.255.255\"";
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
    fn test36_a_long_invalid_ipv6_below_length_limit_first() {
        use super::DESCRIPTION;
        let description = "a long invalid ipv6, below length limit, first";
        let data = "\"100:100:100:100:100:100:255.255.255.255.255\"";
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
    fn test37_a_long_invalid_ipv6_below_length_limit_second() {
        use super::DESCRIPTION;
        let description = "a long invalid ipv6, below length limit, second";
        let data = "\"100:100:100:100:100:100:100:255.255.255.255\"";
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
    fn test38_invalid_non_ascii_৪_a_bengali_4() {
        use super::DESCRIPTION;
        let description = "invalid non-ASCII '৪' (a Bengali 4)";
        let data = "\"1:2:3:4:5:6:7:৪\"";
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
    fn test39_invalid_non_ascii_৪_a_bengali_4_in_the_i_pv4_portion() {
        use super::DESCRIPTION;
        let description = "invalid non-ASCII '৪' (a Bengali 4) in the IPv4 portion";
        let data = "\"1:2::192.16৪.0.1\"";
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
