mod single_dependency_0 {
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
    #[tokio::test]
    async fn test5_ignores_arrays() {}
    #[tokio::test]
    async fn test6_ignores_strings() {}
    #[tokio::test]
    async fn test7_ignores_other_non_objects() {}
}
mod boolean_subschemas_1 {
    #[tokio::test]
    async fn test0_object_with_property_having_schema_true_is_valid() {}
    #[tokio::test]
    async fn test1_object_with_property_having_schema_false_is_invalid() {}
    #[tokio::test]
    async fn test2_object_with_both_properties_is_invalid() {}
    #[tokio::test]
    async fn test3_empty_object_is_valid() {}
}
mod dependencies_with_escaped_characters_2 {
    #[tokio::test]
    async fn test0_quoted_tab() {}
    #[tokio::test]
    async fn test1_quoted_quote() {}
    #[tokio::test]
    async fn test2_quoted_tab_invalid_under_dependent_schema() {}
    #[tokio::test]
    async fn test3_quoted_quote_invalid_under_dependent_schema() {}
}
mod dependent_subschema_incompatible_with_root_3 {
    #[tokio::test]
    async fn test0_matches_root() {}
    #[tokio::test]
    async fn test1_matches_dependency() {}
    #[tokio::test]
    async fn test2_matches_both() {}
    #[tokio::test]
    async fn test3_no_dependency() {}
}
