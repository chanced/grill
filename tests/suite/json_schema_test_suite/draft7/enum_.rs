use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod simple_enum_validation_0 {
    #[tokio::test]
    async fn test0_one_of_the_enum_is_valid() {}
    #[tokio::test]
    async fn test1_something_else_is_invalid() {}
}
mod heterogeneous_enum_validation_1 {
    #[tokio::test]
    async fn test0_one_of_the_enum_is_valid() {}
    #[tokio::test]
    async fn test1_something_else_is_invalid() {}
    #[tokio::test]
    async fn test2_objects_are_deep_compared() {}
    #[tokio::test]
    async fn test3_valid_object_matches() {}
    #[tokio::test]
    async fn test4_extra_properties_in_object_is_invalid() {}
}
mod heterogeneous_enum_with_null_validation_2 {
    #[tokio::test]
    async fn test0_null_is_valid() {}
    #[tokio::test]
    async fn test1_number_is_valid() {}
    #[tokio::test]
    async fn test2_something_else_is_invalid() {}
}
mod enums_in_properties_3 {
    #[tokio::test]
    async fn test0_both_properties_are_valid() {}
    #[tokio::test]
    async fn test1_wrong_foo_value() {}
    #[tokio::test]
    async fn test2_wrong_bar_value() {}
    #[tokio::test]
    async fn test3_missing_optional_property_is_valid() {}
    #[tokio::test]
    async fn test4_missing_required_property_is_invalid() {}
    #[tokio::test]
    async fn test5_missing_all_properties_is_invalid() {}
}
mod enum_with_escaped_characters_4 {
    #[tokio::test]
    async fn test0_member_1_is_valid() {}
    #[tokio::test]
    async fn test1_member_2_is_valid() {}
    #[tokio::test]
    async fn test2_another_string_is_invalid() {}
}
mod enum_with_false_does_not_match_0_5 {
    #[tokio::test]
    async fn test0_false_is_valid() {}
    #[tokio::test]
    async fn test1_integer_zero_is_invalid() {}
    #[tokio::test]
    async fn test2_float_zero_is_invalid() {}
}
mod enum_with_true_does_not_match_1_6 {
    #[tokio::test]
    async fn test0_true_is_valid() {}
    #[tokio::test]
    async fn test1_integer_one_is_invalid() {}
    #[tokio::test]
    async fn test2_float_one_is_invalid() {}
}
mod enum_with_0_does_not_match_false_7 {
    #[tokio::test]
    async fn test0_false_is_invalid() {}
    #[tokio::test]
    async fn test1_integer_zero_is_valid() {}
    #[tokio::test]
    async fn test2_float_zero_is_valid() {}
}
mod enum_with_1_does_not_match_true_8 {
    #[tokio::test]
    async fn test0_true_is_invalid() {}
    #[tokio::test]
    async fn test1_integer_one_is_valid() {}
    #[tokio::test]
    async fn test2_float_one_is_valid() {}
}
mod nul_characters_in_strings_9 {
    #[tokio::test]
    async fn test0_match_string_with_nul() {}
    #[tokio::test]
    async fn test1_do_not_match_string_lacking_nul() {}
}
