mod one_of_0 {
    #[tokio::test]
    async fn test0_first_one_of_valid() {}
    #[tokio::test]
    async fn test1_second_one_of_valid() {}
    #[tokio::test]
    async fn test2_both_one_of_valid() {}
    #[tokio::test]
    async fn test3_neither_one_of_valid() {}
}
mod one_of_with_base_schema_1 {
    #[tokio::test]
    async fn test0_mismatch_base_schema() {}
    #[tokio::test]
    async fn test1_one_one_of_valid() {}
    #[tokio::test]
    async fn test2_both_one_of_valid() {}
}
mod one_of_with_boolean_schemas_all_true_2 {
    #[tokio::test]
    async fn test0_any_value_is_invalid() {}
}
mod one_of_with_boolean_schemas_one_true_3 {
    #[tokio::test]
    async fn test0_any_value_is_valid() {}
}
mod one_of_with_boolean_schemas_more_than_one_true_4 {
    #[tokio::test]
    async fn test0_any_value_is_invalid() {}
}
mod one_of_with_boolean_schemas_all_false_5 {
    #[tokio::test]
    async fn test0_any_value_is_invalid() {}
}
mod one_of_complex_types_6 {
    #[tokio::test]
    async fn test0_first_one_of_valid_complex() {}
    #[tokio::test]
    async fn test1_second_one_of_valid_complex() {}
    #[tokio::test]
    async fn test2_both_one_of_valid_complex() {}
    #[tokio::test]
    async fn test3_neither_one_of_valid_complex() {}
}
mod one_of_with_empty_schema_7 {
    #[tokio::test]
    async fn test0_one_valid_valid() {}
    #[tokio::test]
    async fn test1_both_valid_invalid() {}
}
mod one_of_with_required_8 {
    #[tokio::test]
    async fn test0_both_invalid_invalid() {}
    #[tokio::test]
    async fn test1_first_valid_valid() {}
    #[tokio::test]
    async fn test2_second_valid_valid() {}
    #[tokio::test]
    async fn test3_both_valid_invalid() {}
}
mod one_of_with_missing_optional_property_9 {
    #[tokio::test]
    async fn test0_first_one_of_valid() {}
    #[tokio::test]
    async fn test1_second_one_of_valid() {}
    #[tokio::test]
    async fn test2_both_one_of_valid() {}
    #[tokio::test]
    async fn test3_neither_one_of_valid() {}
}
mod nested_one_of_to_check_validation_semantics_10 {
    #[tokio::test]
    async fn test0_null_is_valid() {}
    #[tokio::test]
    async fn test1_anything_non_null_is_invalid() {}
}
