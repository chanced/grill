use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_time_strings_0 {
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
    fn test6_a_valid_time_string() {}
    #[test]
    fn test7_invalid_time_string_with_extra_leading_zeros() {}
    #[test]
    fn test8_invalid_time_string_with_no_leading_zero_for_single_digit() {}
    #[test]
    fn test9_hour_minute_second_must_be_two_digits() {}
    #[test]
    fn test10_a_valid_time_string_with_leap_second_zulu() {}
    #[test]
    fn test11_invalid_leap_second_zulu_wrong_hour() {}
    #[test]
    fn test12_invalid_leap_second_zulu_wrong_minute() {}
    #[test]
    fn test13_valid_leap_second_zero_time_offset() {}
    #[test]
    fn test14_invalid_leap_second_zero_time_offset_wrong_hour() {}
    #[test]
    fn test15_invalid_leap_second_zero_time_offset_wrong_minute() {}
    #[test]
    fn test16_valid_leap_second_positive_time_offset() {}
    #[test]
    fn test17_valid_leap_second_large_positive_time_offset() {}
    #[test]
    fn test18_invalid_leap_second_positive_time_offset_wrong_hour() {}
    #[test]
    fn test19_invalid_leap_second_positive_time_offset_wrong_minute() {}
    #[test]
    fn test20_valid_leap_second_negative_time_offset() {}
    #[test]
    fn test21_valid_leap_second_large_negative_time_offset() {}
    #[test]
    fn test22_invalid_leap_second_negative_time_offset_wrong_hour() {}
    #[test]
    fn test23_invalid_leap_second_negative_time_offset_wrong_minute() {}
    #[test]
    fn test24_a_valid_time_string_with_second_fraction() {}
    #[test]
    fn test25_a_valid_time_string_with_precise_second_fraction() {}
    #[test]
    fn test26_a_valid_time_string_with_plus_offset() {}
    #[test]
    fn test27_a_valid_time_string_with_minus_offset() {}
    #[test]
    fn test28_hour_minute_in_time_offset_must_be_two_digits() {}
    #[test]
    fn test29_a_valid_time_string_with_case_insensitive_z() {}
    #[test]
    fn test30_an_invalid_time_string_with_invalid_hour() {}
    #[test]
    fn test31_an_invalid_time_string_with_invalid_minute() {}
    #[test]
    fn test32_an_invalid_time_string_with_invalid_second() {}
    #[test]
    fn test33_an_invalid_time_string_with_invalid_leap_second_wrong_hour() {}
    #[test]
    fn test34_an_invalid_time_string_with_invalid_leap_second_wrong_minute() {}
    #[test]
    fn test35_an_invalid_time_string_with_invalid_time_numoffset_hour() {}
    #[test]
    fn test36_an_invalid_time_string_with_invalid_time_numoffset_minute() {}
    #[test]
    fn test37_an_invalid_time_string_with_invalid_time_with_both_z_and_numoffset() {}
    #[test]
    fn test38_an_invalid_offset_indicator() {}
    #[test]
    fn test39_only_rfc3339_not_all_of_iso_8601_are_valid() {}
    #[test]
    fn test40_no_time_offset() {}
    #[test]
    fn test41_no_time_offset_with_second_fraction() {}
    #[test]
    fn test42_invalid_non_ascii_২_a_bengali_2() {}
    #[test]
    fn test43_offset_not_starting_with_plus_or_minus() {}
    #[test]
    fn test44_contains_letters() {}
}
