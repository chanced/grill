use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_date_time_strings_0 {
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
    fn test6_a_valid_date_time_string() {}
    #[test]
    fn test7_a_valid_date_time_string_without_second_fraction() {}
    #[test]
    fn test8_a_valid_date_time_string_with_plus_offset() {}
    #[test]
    fn test9_a_valid_date_time_string_with_minus_offset() {}
    #[test]
    fn test10_a_valid_date_time_with_a_leap_second_utc() {}
    #[test]
    fn test11_a_valid_date_time_with_a_leap_second_with_minus_offset() {}
    #[test]
    fn test12_an_invalid_date_time_past_leap_second_utc() {}
    #[test]
    fn test13_an_invalid_date_time_with_leap_second_on_a_wrong_minute_utc() {}
    #[test]
    fn test14_an_invalid_date_time_with_leap_second_on_a_wrong_hour_utc() {}
    #[test]
    fn test15_an_invalid_day_in_date_time_string() {}
    #[test]
    fn test16_an_invalid_offset_in_date_time_string() {}
    #[test]
    fn test17_an_invalid_closing_z_after_time_zone_offset() {}
    #[test]
    fn test18_an_invalid_date_time_string() {}
    #[test]
    fn test19_case_insensitive_t_and_z() {}
    #[test]
    fn test20_only_rfc3339_not_all_of_iso_8601_are_valid() {}
    #[test]
    fn test21_invalid_non_padded_month_dates() {}
    #[test]
    fn test22_invalid_non_padded_day_dates() {}
    #[test]
    fn test23_invalid_non_ascii_৪_a_bengali_4_in_date_portion() {}
    #[test]
    fn test24_invalid_non_ascii_৪_a_bengali_4_in_time_portion() {}
}
