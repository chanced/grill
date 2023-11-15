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
mod validation_of_relative_json_pointers_rjp_0 {
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
    async fn test6_a_valid_upwards_rjp() {}
    #[tokio::test]
    async fn test7_a_valid_downwards_rjp() {}
    #[tokio::test]
    async fn test8_a_valid_up_and_then_down_rjp_with_array_index() {}
    #[tokio::test]
    async fn test9_a_valid_rjp_taking_the_member_or_index_name() {}
    #[tokio::test]
    async fn test10_an_invalid_rjp_that_is_a_valid_json_pointer() {}
    #[tokio::test]
    async fn test11_negative_prefix() {}
    #[tokio::test]
    async fn test12_explicit_positive_prefix() {}
    #[tokio::test]
    async fn test13_is_not_a_valid_json_pointer() {}
    #[tokio::test]
    async fn test14_zero_cannot_be_followed_by_other_digits_plus_json_pointer() {}
    #[tokio::test]
    async fn test15_zero_cannot_be_followed_by_other_digits_plus_octothorpe() {}
    #[tokio::test]
    async fn test16_empty_string() {}
    #[tokio::test]
    async fn test17_multi_digit_integer_prefix() {}
}
