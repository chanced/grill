use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod ignore_if_without_then_or_else_0 {
    #[tokio::test]
    async fn test0_valid_when_valid_against_lone_if() {}
    #[tokio::test]
    async fn test1_valid_when_invalid_against_lone_if() {}
}
mod ignore_then_without_if_1 {
    #[tokio::test]
    async fn test0_valid_when_valid_against_lone_then() {}
    #[tokio::test]
    async fn test1_valid_when_invalid_against_lone_then() {}
}
mod ignore_else_without_if_2 {
    #[tokio::test]
    async fn test0_valid_when_valid_against_lone_else() {}
    #[tokio::test]
    async fn test1_valid_when_invalid_against_lone_else() {}
}
mod if_and_then_without_else_3 {
    #[tokio::test]
    async fn test0_valid_through_then() {}
    #[tokio::test]
    async fn test1_invalid_through_then() {}
    #[tokio::test]
    async fn test2_valid_when_if_test_fails() {}
}
mod if_and_else_without_then_4 {
    #[tokio::test]
    async fn test0_valid_when_if_test_passes() {}
    #[tokio::test]
    async fn test1_valid_through_else() {}
    #[tokio::test]
    async fn test2_invalid_through_else() {}
}
mod validate_against_correct_branch_then_vs_else_5 {
    #[tokio::test]
    async fn test0_valid_through_then() {}
    #[tokio::test]
    async fn test1_invalid_through_then() {}
    #[tokio::test]
    async fn test2_valid_through_else() {}
    #[tokio::test]
    async fn test3_invalid_through_else() {}
}
mod non_interference_across_combined_schemas_6 {
    #[tokio::test]
    async fn test0_valid_but_would_have_been_invalid_through_then() {}
    #[tokio::test]
    async fn test1_valid_but_would_have_been_invalid_through_else() {}
}
mod if_with_boolean_schema_true_7 {
    #[tokio::test]
    async fn test0_boolean_schema_true_in_if_always_chooses_the_then_path_valid() {}
    #[tokio::test]
    async fn test1_boolean_schema_true_in_if_always_chooses_the_then_path_invalid() {}
}
mod if_with_boolean_schema_false_8 {
    #[tokio::test]
    async fn test0_boolean_schema_false_in_if_always_chooses_the_else_path_invalid() {}
    #[tokio::test]
    async fn test1_boolean_schema_false_in_if_always_chooses_the_else_path_valid() {}
}
mod if_appears_at_the_end_when_serialized_keyword_processing_sequence_9 {
    #[tokio::test]
    async fn test0_yes_redirects_to_then_and_passes() {}
    #[tokio::test]
    async fn test1_other_redirects_to_else_and_passes() {}
    #[tokio::test]
    async fn test2_no_redirects_to_then_and_fails() {}
    #[tokio::test]
    async fn test3_invalid_redirects_to_else_and_fails() {}
}
