use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod boolean_schema_true_0 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_string_is_valid() {}
    #[tokio::test]
    async fn test2_boolean_true_is_valid() {}
    #[tokio::test]
    async fn test3_boolean_false_is_valid() {}
    #[tokio::test]
    async fn test4_null_is_valid() {}
    #[tokio::test]
    async fn test5_object_is_valid() {}
    #[tokio::test]
    async fn test6_empty_object_is_valid() {}
    #[tokio::test]
    async fn test7_array_is_valid() {}
    #[tokio::test]
    async fn test8_empty_array_is_valid() {}
}
mod boolean_schema_false_1 {
    #[tokio::test]
    async fn test0_number_is_invalid() {}
    #[tokio::test]
    async fn test1_string_is_invalid() {}
    #[tokio::test]
    async fn test2_boolean_true_is_invalid() {}
    #[tokio::test]
    async fn test3_boolean_false_is_invalid() {}
    #[tokio::test]
    async fn test4_null_is_invalid() {}
    #[tokio::test]
    async fn test5_object_is_invalid() {}
    #[tokio::test]
    async fn test6_empty_object_is_invalid() {}
    #[tokio::test]
    async fn test7_array_is_invalid() {}
    #[tokio::test]
    async fn test8_empty_array_is_invalid() {}
}
