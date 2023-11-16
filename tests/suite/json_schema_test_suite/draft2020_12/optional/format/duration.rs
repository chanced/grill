use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_duration_strings_0 {
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
    fn test6_a_valid_duration_string() {}
    #[test]
    fn test7_an_invalid_duration_string() {}
    #[test]
    fn test8_no_elements_present() {}
    #[test]
    fn test9_no_time_elements_present() {}
    #[test]
    fn test10_no_date_or_time_elements_present() {}
    #[test]
    fn test11_elements_out_of_order() {}
    #[test]
    fn test12_missing_time_separator() {}
    #[test]
    fn test13_time_element_in_the_date_position() {}
    #[test]
    fn test14_four_years_duration() {}
    #[test]
    fn test15_zero_time_in_seconds() {}
    #[test]
    fn test16_zero_time_in_days() {}
    #[test]
    fn test17_one_month_duration() {}
    #[test]
    fn test18_one_minute_duration() {}
    #[test]
    fn test19_one_and_a_half_days_in_hours() {}
    #[test]
    fn test20_one_and_a_half_days_in_days_and_hours() {}
    #[test]
    fn test21_two_weeks() {}
    #[test]
    fn test22_weeks_cannot_be_combined_with_other_units() {}
    #[test]
    fn test23_invalid_non_ascii_২_a_bengali_2() {}
    #[test]
    fn test24_element_without_unit() {}
}
