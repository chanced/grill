mod all_of_0 {
    #[tokio::test]
    async fn test0_all_of() {}
    #[tokio::test]
    async fn test1_mismatch_second() {}
    #[tokio::test]
    async fn test2_mismatch_first() {}
    #[tokio::test]
    async fn test3_wrong_type() {}
}
mod all_of_with_base_schema_1 {
    #[tokio::test]
    async fn test0_valid() {}
    #[tokio::test]
    async fn test1_mismatch_base_schema() {}
    #[tokio::test]
    async fn test2_mismatch_first_all_of() {}
    #[tokio::test]
    async fn test3_mismatch_second_all_of() {}
    #[tokio::test]
    async fn test4_mismatch_both() {}
}
mod all_of_simple_types_2 {
    #[tokio::test]
    async fn test0_valid() {}
    #[tokio::test]
    async fn test1_mismatch_one() {}
}
mod all_of_with_boolean_schemas_all_true_3 {
    #[tokio::test]
    async fn test0_any_value_is_valid() {}
}
mod all_of_with_boolean_schemas_some_false_4 {
    #[tokio::test]
    async fn test0_any_value_is_invalid() {}
}
mod all_of_with_boolean_schemas_all_false_5 {
    #[tokio::test]
    async fn test0_any_value_is_invalid() {}
}
mod all_of_with_one_empty_schema_6 {
    #[tokio::test]
    async fn test0_any_data_is_valid() {}
}
mod all_of_with_two_empty_schemas_7 {
    #[tokio::test]
    async fn test0_any_data_is_valid() {}
}
mod all_of_with_the_first_empty_schema_8 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_string_is_invalid() {}
}
mod all_of_with_the_last_empty_schema_9 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_string_is_invalid() {}
}
mod nested_all_of_to_check_validation_semantics_10 {
    #[tokio::test]
    async fn test0_null_is_valid() {}
    #[tokio::test]
    async fn test1_anything_non_null_is_invalid() {}
}
mod all_of_combined_with_any_of_one_of_11 {
    #[tokio::test]
    async fn test0_all_of_false_any_of_false_one_of_false() {}
    #[tokio::test]
    async fn test1_all_of_false_any_of_false_one_of_true() {}
    #[tokio::test]
    async fn test2_all_of_false_any_of_true_one_of_false() {}
    #[tokio::test]
    async fn test3_all_of_false_any_of_true_one_of_true() {}
    #[tokio::test]
    async fn test4_all_of_true_any_of_false_one_of_false() {}
    #[tokio::test]
    async fn test5_all_of_true_any_of_false_one_of_true() {}
    #[tokio::test]
    async fn test6_all_of_true_any_of_true_one_of_false() {}
    #[tokio::test]
    async fn test7_all_of_true_any_of_true_one_of_true() {}
}
