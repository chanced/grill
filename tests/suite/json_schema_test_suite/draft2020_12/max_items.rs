mod max_items_validation_0 {
    #[tokio::test]
    async fn test0_shorter_is_valid() {}
    #[tokio::test]
    async fn test1_exact_length_is_valid() {}
    #[tokio::test]
    async fn test2_too_long_is_invalid() {}
    #[tokio::test]
    async fn test3_ignores_non_arrays() {}
}
mod max_items_validation_with_a_decimal_1 {
    #[tokio::test]
    async fn test0_shorter_is_valid() {}
    #[tokio::test]
    async fn test1_too_long_is_invalid() {}
}
