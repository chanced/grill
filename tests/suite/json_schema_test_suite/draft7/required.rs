use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod required_validation_0 {
    #[tokio::test]
    async fn test0_present_required_property_is_valid() {}
    #[tokio::test]
    async fn test1_non_present_required_property_is_invalid() {}
    #[tokio::test]
    async fn test2_ignores_arrays() {}
    #[tokio::test]
    async fn test3_ignores_strings() {}
    #[tokio::test]
    async fn test4_ignores_other_non_objects() {}
}
mod required_default_validation_1 {
    #[tokio::test]
    async fn test0_not_required_by_default() {}
}
mod required_with_empty_array_2 {
    #[tokio::test]
    async fn test0_property_not_required() {}
}
mod required_with_escaped_characters_3 {
    #[tokio::test]
    async fn test0_object_with_all_properties_present_is_valid() {}
    #[tokio::test]
    async fn test1_object_with_some_properties_missing_is_invalid() {}
}
mod required_properties_whose_names_are_javascript_object_property_names_4 {
    #[tokio::test]
    async fn test0_ignores_arrays() {}
    #[tokio::test]
    async fn test1_ignores_other_non_objects() {}
    #[tokio::test]
    async fn test2_none_of_the_properties_mentioned() {}
    #[tokio::test]
    async fn test3_proto_present() {}
    #[tokio::test]
    async fn test4_to_string_present() {}
    #[tokio::test]
    async fn test5_constructor_present() {}
    #[tokio::test]
    async fn test6_all_present() {}
}
