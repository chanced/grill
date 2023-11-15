use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
async fn interrogator() {
    todo!()
}
async fn interrogator() {
    todo!()
}
mod validation_of_date_time_strings_0 {
    #[tokio::test]
    async fn test0_all_string_formats_ignore_integers() {}
    #[tokio::test]
    async fn test1_all_string_formats_ignore_floats() {}
    #[tokio::test]
    async fn test2_all_string_formats_ignore_objects() {}
    #[tokio::test]
    async fn test3_all_string_formats_ignore_arrays() {}
    #[tokio::test]
    async fn test4_all_string_formats_ignore_booleans() {}
    #[tokio::test]
    async fn test5_all_string_formats_ignore_nulls() {}
    #[tokio::test]
    async fn test6_a_valid_date_time_string() {}
    #[tokio::test]
    async fn test7_a_valid_date_time_string_without_second_fraction() {}
    #[tokio::test]
    async fn test8_a_valid_date_time_string_with_plus_offset() {}
    #[tokio::test]
    async fn test9_a_valid_date_time_string_with_minus_offset() {}
    #[tokio::test]
    async fn test10_a_valid_date_time_with_a_leap_second_utc() {}
    #[tokio::test]
    async fn test11_a_valid_date_time_with_a_leap_second_with_minus_offset() {}
    #[tokio::test]
    async fn test12_an_invalid_date_time_past_leap_second_utc() {}
    #[tokio::test]
    async fn test13_an_invalid_date_time_with_leap_second_on_a_wrong_minute_utc() {}
    #[tokio::test]
    async fn test14_an_invalid_date_time_with_leap_second_on_a_wrong_hour_utc() {}
    #[tokio::test]
    async fn test15_an_invalid_day_in_date_time_string() {}
    #[tokio::test]
    async fn test16_an_invalid_offset_in_date_time_string() {}
    #[tokio::test]
    async fn test17_an_invalid_closing_z_after_time_zone_offset() {}
    #[tokio::test]
    async fn test18_an_invalid_date_time_string() {}
    #[tokio::test]
    async fn test19_case_insensitive_t_and_z() {}
    #[tokio::test]
    async fn test20_only_rfc3339_not_all_of_iso_8601_are_valid() {}
    #[tokio::test]
    async fn test21_invalid_non_padded_month_dates() {}
    #[tokio::test]
    async fn test22_invalid_non_padded_day_dates() {}
    #[tokio::test]
    async fn test23_invalid_non_ascii_৪_a_bengali_4_in_date_portion() {}
    #[tokio::test]
    async fn test24_invalid_non_ascii_৪_a_bengali_4_in_time_portion() {}
}
