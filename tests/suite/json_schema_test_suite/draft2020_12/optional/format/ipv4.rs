use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_ip_addresses_0 {
    #[test]
    fn test0_all_string_formats_ignore_integers() {}
    #[test]
    fn test1_all_string_formats_ignore_floats() {}
    #[test]
    fn test2_all_string_formats_ignore_objects() {}
    #[test]
    fn test3_all_string_formats_ignore_arrays() {}
    #[test]
    fn test4_all_string_formats_ignore_booleans() {}
    #[test]
    fn test5_all_string_formats_ignore_nulls() {}
    #[test]
    fn test6_a_valid_ip_address() {}
    #[test]
    fn test7_an_ip_address_with_too_many_components() {}
    #[test]
    fn test8_an_ip_address_with_out_of_range_values() {}
    #[test]
    fn test9_an_ip_address_without_4_components() {}
    #[test]
    fn test10_an_ip_address_as_an_integer() {}
    #[test]
    fn test11_an_ip_address_as_an_integer_decimal() {}
    #[test]
    fn test12_invalid_leading_zeroes_as_they_are_treated_as_octals() {}
    #[test]
    fn test13_value_without_leading_zero_is_valid() {}
    #[test]
    fn test14_invalid_non_ascii_২_a_bengali_2() {}
}
