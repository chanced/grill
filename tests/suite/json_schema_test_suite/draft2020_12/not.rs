mod not_0 {
    #[tokio::test]
    async fn test0_allowed() {}
    #[tokio::test]
    async fn test1_disallowed() {}
}
mod not_multiple_types_1 {
    #[tokio::test]
    async fn test0_valid() {}
    #[tokio::test]
    async fn test1_mismatch() {}
    #[tokio::test]
    async fn test2_other_mismatch() {}
}
mod not_more_complex_schema_2 {
    #[tokio::test]
    async fn test0_match_() {}
    #[tokio::test]
    async fn test1_other_match() {}
    #[tokio::test]
    async fn test2_mismatch() {}
}
mod forbidden_property_3 {
    #[tokio::test]
    async fn test0_property_present() {}
    #[tokio::test]
    async fn test1_property_absent() {}
}
mod not_with_boolean_schema_true_4 {
    #[tokio::test]
    async fn test0_any_value_is_invalid() {}
}
mod not_with_boolean_schema_false_5 {
    #[tokio::test]
    async fn test0_any_value_is_valid() {}
}
mod collect_annotations_inside_a_not_even_if_collection_is_disabled_6 {
    #[tokio::test]
    async fn test0_unevaluated_property() {}
    #[tokio::test]
    async fn test1_annotations_are_still_collected_inside_a_not() {}
}
