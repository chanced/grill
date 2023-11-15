use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
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
mod property_names_validation_with_pattern_1 {
    #[tokio::test]
    async fn test0_matching_property_names_valid() {}
    #[tokio::test]
    async fn test1_non_matching_property_name_is_invalid() {}
    #[tokio::test]
    async fn test2_object_without_properties_is_valid() {}
}
mod property_names_with_boolean_schema_true_2 {
    #[tokio::test]
    async fn test0_object_with_any_properties_is_valid() {}
    #[tokio::test]
    async fn test1_empty_object_is_valid() {}
}
mod property_names_with_boolean_schema_false_3 {
    #[tokio::test]
    async fn test0_object_with_any_properties_is_invalid() {}
    #[tokio::test]
    async fn test1_empty_object_is_valid() {}
}
