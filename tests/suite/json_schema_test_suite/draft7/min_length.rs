use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod min_length_validation_0 {
    #[tokio::test]
    async fn test0_longer_is_valid() {}
    #[tokio::test]
    async fn test1_exact_length_is_valid() {}
    #[tokio::test]
    async fn test2_too_short_is_invalid() {}
    #[tokio::test]
    async fn test3_ignores_non_strings() {}
    #[tokio::test]
    async fn test4_one_supplementary_unicode_code_point_is_not_long_enough() {}
}
mod min_length_validation_with_a_decimal_1 {
    #[tokio::test]
    async fn test0_longer_is_valid() {}
    #[tokio::test]
    async fn test1_too_short_is_invalid() {}
}
