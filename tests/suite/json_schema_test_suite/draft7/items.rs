use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod a_schema_given_for_items_0 {
    #[tokio::test]
    async fn test0_valid_items() {}
    #[tokio::test]
    async fn test1_wrong_type_of_items() {}
    #[tokio::test]
    async fn test2_ignores_non_arrays() {}
    #[tokio::test]
    async fn test3_java_script_pseudo_array_is_valid() {}
}
mod an_array_of_schemas_for_items_1 {
    #[tokio::test]
    async fn test0_correct_types() {}
    #[tokio::test]
    async fn test1_wrong_types() {}
    #[tokio::test]
    async fn test2_incomplete_array_of_items() {}
    #[tokio::test]
    async fn test3_array_with_additional_items() {}
    #[tokio::test]
    async fn test4_empty_array() {}
    #[tokio::test]
    async fn test5_java_script_pseudo_array_is_valid() {}
}
mod items_with_boolean_schema_true_2 {
    #[tokio::test]
    async fn test0_any_array_is_valid() {}
    #[tokio::test]
    async fn test1_empty_array_is_valid() {}
}
mod items_with_boolean_schema_false_3 {
    #[tokio::test]
    async fn test0_any_non_empty_array_is_invalid() {}
    #[tokio::test]
    async fn test1_empty_array_is_valid() {}
}
mod items_with_boolean_schemas_4 {
    #[tokio::test]
    async fn test0_array_with_one_item_is_valid() {}
    #[tokio::test]
    async fn test1_array_with_two_items_is_invalid() {}
    #[tokio::test]
    async fn test2_empty_array_is_valid() {}
}
mod items_and_subitems_5 {
    #[tokio::test]
    async fn test0_valid_items() {}
    #[tokio::test]
    async fn test1_too_many_items() {}
    #[tokio::test]
    async fn test2_too_many_sub_items() {}
    #[tokio::test]
    async fn test3_wrong_item() {}
    #[tokio::test]
    async fn test4_wrong_sub_item() {}
    #[tokio::test]
    async fn test5_fewer_items_is_valid() {}
}
mod nested_items_6 {
    #[tokio::test]
    async fn test0_valid_nested_array() {}
    #[tokio::test]
    async fn test1_nested_array_with_invalid_type() {}
    #[tokio::test]
    async fn test2_not_deep_enough() {}
}
mod single_form_items_with_null_instance_elements_7 {
    #[tokio::test]
    async fn test0_allows_null_elements() {}
}
mod array_form_items_with_null_instance_elements_8 {
    #[tokio::test]
    async fn test0_allows_null_elements() {}
}
