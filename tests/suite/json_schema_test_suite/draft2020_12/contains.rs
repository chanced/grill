mod contains_keyword_validation_0 {
    #[tokio::test]
    async fn test0_array_with_item_matching_schema_5_is_valid() {}
    #[tokio::test]
    async fn test1_array_with_item_matching_schema_6_is_valid() {}
    #[tokio::test]
    async fn test2_array_with_two_items_matching_schema_5_6_is_valid() {}
    #[tokio::test]
    async fn test3_array_without_items_matching_schema_is_invalid() {}
    #[tokio::test]
    async fn test4_empty_array_is_invalid() {}
    #[tokio::test]
    async fn test5_not_array_is_valid() {}
}
mod contains_keyword_with_const_keyword_1 {
    #[tokio::test]
    async fn test0_array_with_item_5_is_valid() {}
    #[tokio::test]
    async fn test1_array_with_two_items_5_is_valid() {}
    #[tokio::test]
    async fn test2_array_without_item_5_is_invalid() {}
}
mod contains_keyword_with_boolean_schema_true_2 {
    #[tokio::test]
    async fn test0_any_non_empty_array_is_valid() {}
    #[tokio::test]
    async fn test1_empty_array_is_invalid() {}
}
mod contains_keyword_with_boolean_schema_false_3 {
    #[tokio::test]
    async fn test0_any_non_empty_array_is_invalid() {}
    #[tokio::test]
    async fn test1_empty_array_is_invalid() {}
    #[tokio::test]
    async fn test2_non_arrays_are_valid() {}
}
mod items_contains_4 {
    #[tokio::test]
    async fn test0_matches_items_does_not_match_contains() {}
    #[tokio::test]
    async fn test1_does_not_match_items_matches_contains() {}
    #[tokio::test]
    async fn test2_matches_both_items_and_contains() {}
    #[tokio::test]
    async fn test3_matches_neither_items_nor_contains() {}
}
mod contains_with_false_if_subschema_5 {
    #[tokio::test]
    async fn test0_any_non_empty_array_is_valid() {}
    #[tokio::test]
    async fn test1_empty_array_is_invalid() {}
}
mod contains_with_null_instance_elements_6 {
    #[tokio::test]
    async fn test0_allows_null_items() {}
}
