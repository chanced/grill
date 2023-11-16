use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod uuid_format_0 {
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
    fn test6_all_upper_case() {}
    #[test]
    fn test7_all_lower_case() {}
    #[test]
    fn test8_mixed_case() {}
    #[test]
    fn test9_all_zeroes_is_valid() {}
    #[test]
    fn test10_wrong_length() {}
    #[test]
    fn test11_missing_section() {}
    #[test]
    fn test12_bad_characters_not_hex() {}
    #[test]
    fn test13_no_dashes() {}
    #[test]
    fn test14_too_few_dashes() {}
    #[test]
    fn test15_too_many_dashes() {}
    #[test]
    fn test16_dashes_in_the_wrong_spot() {}
    #[test]
    fn test17_valid_version_4() {}
    #[test]
    fn test18_valid_version_5() {}
    #[test]
    fn test19_hypothetical_version_6() {}
    #[test]
    fn test20_hypothetical_version_15() {}
}
