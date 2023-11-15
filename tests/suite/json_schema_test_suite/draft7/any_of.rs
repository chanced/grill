use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod any_of_0 {
    #[tokio::test]
    async fn test0_first_any_of_valid() {}
    #[tokio::test]
    async fn test1_second_any_of_valid() {}
    #[tokio::test]
    async fn test2_both_any_of_valid() {}
    #[tokio::test]
    async fn test3_neither_any_of_valid() {}
}
mod any_of_with_base_schema_1 {
    #[tokio::test]
    async fn test0_mismatch_base_schema() {}
    #[tokio::test]
    async fn test1_one_any_of_valid() {}
    #[tokio::test]
    async fn test2_both_any_of_invalid() {}
}
mod any_of_with_boolean_schemas_all_true_2 {
    #[tokio::test]
    async fn test0_any_value_is_valid() {}
}
mod any_of_with_boolean_schemas_some_true_3 {
    #[tokio::test]
    async fn test0_any_value_is_valid() {}
}
mod any_of_with_boolean_schemas_all_false_4 {
    #[tokio::test]
    async fn test0_any_value_is_invalid() {}
}
mod any_of_complex_types_5 {
    #[tokio::test]
    async fn test0_first_any_of_valid_complex() {}
    #[tokio::test]
    async fn test1_second_any_of_valid_complex() {}
    #[tokio::test]
    async fn test2_both_any_of_valid_complex() {}
    #[tokio::test]
    async fn test3_neither_any_of_valid_complex() {}
}
mod any_of_with_one_empty_schema_6 {
    #[tokio::test]
    async fn test0_string_is_valid() {}
    #[tokio::test]
    async fn test1_number_is_valid() {}
}
mod nested_any_of_to_check_validation_semantics_7 {
    #[tokio::test]
    async fn test0_null_is_valid() {}
    #[tokio::test]
    async fn test1_anything_non_null_is_invalid() {}
}
