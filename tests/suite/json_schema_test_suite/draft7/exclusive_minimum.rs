use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod exclusive_minimum_validation_0 {
    #[tokio::test]
    async fn test0_above_the_exclusive_minimum_is_valid() {}
    #[tokio::test]
    async fn test1_boundary_point_is_invalid() {}
    #[tokio::test]
    async fn test2_below_the_exclusive_minimum_is_invalid() {}
    #[tokio::test]
    async fn test3_ignores_non_numbers() {}
}
