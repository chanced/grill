mod max_length_validation_0 {
    #[tokio::test]
    async fn test0_shorter_is_valid() {}
    #[tokio::test]
    async fn test1_exact_length_is_valid() {}
    #[tokio::test]
    async fn test2_too_long_is_invalid() {}
    #[tokio::test]
    async fn test3_ignores_non_strings() {}
    #[tokio::test]
    async fn test4_two_supplementary_unicode_code_points_is_long_enough() {}
}
mod max_length_validation_with_a_decimal_1 {
    #[tokio::test]
    async fn test0_shorter_is_valid() {}
    #[tokio::test]
    async fn test1_too_long_is_invalid() {}
}
