use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_date_strings_0 {
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
    fn test6_a_valid_date_string() {}
    #[test]
    fn test7_a_valid_date_string_with_31_days_in_january() {}
    #[test]
    fn test8_a_invalid_date_string_with_32_days_in_january() {}
    #[test]
    fn test9_a_valid_date_string_with_28_days_in_february_normal() {}
    #[test]
    fn test10_a_invalid_date_string_with_29_days_in_february_normal() {}
    #[test]
    fn test11_a_valid_date_string_with_29_days_in_february_leap() {}
    #[test]
    fn test12_a_invalid_date_string_with_30_days_in_february_leap() {}
    #[test]
    fn test13_a_valid_date_string_with_31_days_in_march() {}
    #[test]
    fn test14_a_invalid_date_string_with_32_days_in_march() {}
    #[test]
    fn test15_a_valid_date_string_with_30_days_in_april() {}
    #[test]
    fn test16_a_invalid_date_string_with_31_days_in_april() {}
    #[test]
    fn test17_a_valid_date_string_with_31_days_in_may() {}
    #[test]
    fn test18_a_invalid_date_string_with_32_days_in_may() {}
    #[test]
    fn test19_a_valid_date_string_with_30_days_in_june() {}
    #[test]
    fn test20_a_invalid_date_string_with_31_days_in_june() {}
    #[test]
    fn test21_a_valid_date_string_with_31_days_in_july() {}
    #[test]
    fn test22_a_invalid_date_string_with_32_days_in_july() {}
    #[test]
    fn test23_a_valid_date_string_with_31_days_in_august() {}
    #[test]
    fn test24_a_invalid_date_string_with_32_days_in_august() {}
    #[test]
    fn test25_a_valid_date_string_with_30_days_in_september() {}
    #[test]
    fn test26_a_invalid_date_string_with_31_days_in_september() {}
    #[test]
    fn test27_a_valid_date_string_with_31_days_in_october() {}
    #[test]
    fn test28_a_invalid_date_string_with_32_days_in_october() {}
    #[test]
    fn test29_a_valid_date_string_with_30_days_in_november() {}
    #[test]
    fn test30_a_invalid_date_string_with_31_days_in_november() {}
    #[test]
    fn test31_a_valid_date_string_with_31_days_in_december() {}
    #[test]
    fn test32_a_invalid_date_string_with_32_days_in_december() {}
    #[test]
    fn test33_a_invalid_date_string_with_invalid_month() {}
    #[test]
    fn test34_an_invalid_date_string() {}
    #[test]
    fn test35_only_rfc3339_not_all_of_iso_8601_are_valid() {}
    #[test]
    fn test36_non_padded_month_dates_are_not_valid() {}
    #[test]
    fn test37_non_padded_day_dates_are_not_valid() {}
    #[test]
    fn test38_invalid_month() {}
    #[test]
    fn test39_invalid_month_day_combination() {}
    #[test]
    fn test40_2021_is_not_a_leap_year() {}
    #[test]
    fn test41_2020_is_a_leap_year() {}
    #[test]
    fn test42_invalid_non_ascii_৪_a_bengali_4() {}
    #[test]
    fn test43_iso8601_non_rfc3339_yyyymmdd_without_dashes_2023_03_28() {}
    #[test]
    fn test44_iso8601_non_rfc3339_week_number_implicit_day_of_week_2023_01_02() {}
    #[test]
    fn test45_iso8601_non_rfc3339_week_number_with_day_of_week_2023_03_28() {}
    #[test]
    fn test46_iso8601_non_rfc3339_week_number_rollover_to_next_year_2023_01_01() {}
}
