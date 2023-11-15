mod property_names_validation_0 {
    #[tokio::test]
    async fn test0_all_property_names_valid() {}
    #[tokio::test]
    async fn test1_some_property_names_invalid() {}
    #[tokio::test]
    async fn test2_object_without_properties_is_valid() {}
    #[tokio::test]
    async fn test3_ignores_arrays() {}
    #[tokio::test]
    async fn test4_ignores_strings() {}
    #[tokio::test]
    async fn test5_ignores_other_non_objects() {}
}
mod property_names_with_boolean_schema_true_1 {
    #[tokio::test]
    async fn test0_object_with_any_properties_is_valid() {}
    #[tokio::test]
    async fn test1_empty_object_is_valid() {}
}
mod property_names_with_boolean_schema_false_2 {
    #[tokio::test]
    async fn test0_object_with_any_properties_is_invalid() {}
    #[tokio::test]
    async fn test1_empty_object_is_valid() {}
}
