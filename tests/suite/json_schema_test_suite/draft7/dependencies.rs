use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod dependencies_0 {
    #[tokio::test]
    async fn test0_neither() {}
    #[tokio::test]
    async fn test1_nondependant() {}
    #[tokio::test]
    async fn test2_with_dependency() {}
    #[tokio::test]
    async fn test3_missing_dependency() {}
    #[tokio::test]
    async fn test4_ignores_arrays() {}
    #[tokio::test]
    async fn test5_ignores_strings() {}
    #[tokio::test]
    async fn test6_ignores_other_non_objects() {}
}
mod dependencies_with_empty_array_1 {
    #[tokio::test]
    async fn test0_empty_object() {}
    #[tokio::test]
    async fn test1_object_with_one_property() {}
    #[tokio::test]
    async fn test2_non_object_is_valid() {}
}
mod multiple_dependencies_2 {
    #[tokio::test]
    async fn test0_neither() {}
    #[tokio::test]
    async fn test1_nondependants() {}
    #[tokio::test]
    async fn test2_with_dependencies() {}
    #[tokio::test]
    async fn test3_missing_dependency() {}
    #[tokio::test]
    async fn test4_missing_other_dependency() {}
    #[tokio::test]
    async fn test5_missing_both_dependencies() {}
}
mod multiple_dependencies_subschema_3 {
    #[tokio::test]
    async fn test0_valid() {}
    #[tokio::test]
    async fn test1_no_dependency() {}
    #[tokio::test]
    async fn test2_wrong_type() {}
    #[tokio::test]
    async fn test3_wrong_type_other() {}
    #[tokio::test]
    async fn test4_wrong_type_both() {}
}
mod dependencies_with_boolean_subschemas_4 {
    #[tokio::test]
    async fn test0_object_with_property_having_schema_true_is_valid() {}
    #[tokio::test]
    async fn test1_object_with_property_having_schema_false_is_invalid() {}
    #[tokio::test]
    async fn test2_object_with_both_properties_is_invalid() {}
    #[tokio::test]
    async fn test3_empty_object_is_valid() {}
}
mod dependencies_with_escaped_characters_5 {
    #[tokio::test]
    async fn test0_valid_object_1() {}
    #[tokio::test]
    async fn test1_valid_object_2() {}
    #[tokio::test]
    async fn test2_valid_object_3() {}
    #[tokio::test]
    async fn test3_invalid_object_1() {}
    #[tokio::test]
    async fn test4_invalid_object_2() {}
    #[tokio::test]
    async fn test5_invalid_object_3() {}
    #[tokio::test]
    async fn test6_invalid_object_4() {}
}
mod dependent_subschema_incompatible_with_root_6 {
    #[tokio::test]
    async fn test0_matches_root() {}
    #[tokio::test]
    async fn test1_matches_dependency() {}
    #[tokio::test]
    async fn test2_matches_both() {}
    #[tokio::test]
    async fn test3_no_dependency() {}
}
