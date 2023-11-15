mod single_dependency_0 {
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
mod empty_dependents_1 {
    #[tokio::test]
    async fn test0_empty_object() {}
    #[tokio::test]
    async fn test1_object_with_one_property() {}
    #[tokio::test]
    async fn test2_non_object_is_valid() {}
}
mod multiple_dependents_required_2 {
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
mod dependencies_with_escaped_characters_3 {
    #[tokio::test]
    async fn test0_crlf() {}
    #[tokio::test]
    async fn test1_quoted_quotes() {}
    #[tokio::test]
    async fn test2_crlf_missing_dependent() {}
    #[tokio::test]
    async fn test3_quoted_quotes_missing_dependent() {}
}
