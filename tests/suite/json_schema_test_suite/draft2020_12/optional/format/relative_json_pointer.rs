use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_relative_json_pointers_rjp_0 {
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
    fn test6_a_valid_upwards_rjp() {}
    #[test]
    fn test7_a_valid_downwards_rjp() {}
    #[test]
    fn test8_a_valid_up_and_then_down_rjp_with_array_index() {}
    #[test]
    fn test9_a_valid_rjp_taking_the_member_or_index_name() {}
    #[test]
    fn test10_an_invalid_rjp_that_is_a_valid_json_pointer() {}
    #[test]
    fn test11_negative_prefix() {}
    #[test]
    fn test12_explicit_positive_prefix() {}
    #[test]
    fn test13_is_not_a_valid_json_pointer() {}
    #[test]
    fn test14_zero_cannot_be_followed_by_other_digits_plus_json_pointer() {}
    #[test]
    fn test15_zero_cannot_be_followed_by_other_digits_plus_octothorpe() {}
    #[test]
    fn test16_empty_string() {}
    #[test]
    fn test17_multi_digit_integer_prefix() {}
}
