mod min_properties_validation_0 {
    #[tokio::test]
    async fn test0_longer_is_valid() {}
    #[tokio::test]
    async fn test1_exact_length_is_valid() {}
    #[tokio::test]
    async fn test2_too_short_is_invalid() {}
    #[tokio::test]
    async fn test3_ignores_arrays() {}
    #[tokio::test]
    async fn test4_ignores_strings() {}
    #[tokio::test]
    async fn test5_ignores_other_non_objects() {}
}
mod min_properties_validation_with_a_decimal_1 {
    #[tokio::test]
    async fn test0_longer_is_valid() {}
    #[tokio::test]
    async fn test1_too_short_is_invalid() {}
}
