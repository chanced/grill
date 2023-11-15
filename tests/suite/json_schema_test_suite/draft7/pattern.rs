use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod pattern_validation_0 {
    #[tokio::test]
    async fn test0_a_matching_pattern_is_valid() {}
    #[tokio::test]
    async fn test1_a_non_matching_pattern_is_invalid() {}
    #[tokio::test]
    async fn test2_ignores_booleans() {}
    #[tokio::test]
    async fn test3_ignores_integers() {}
    #[tokio::test]
    async fn test4_ignores_floats() {}
    #[tokio::test]
    async fn test5_ignores_objects() {}
    #[tokio::test]
    async fn test6_ignores_arrays() {}
    #[tokio::test]
    async fn test7_ignores_null() {}
}
mod pattern_is_not_anchored_1 {
    #[tokio::test]
    async fn test0_matches_a_substring() {}
}
