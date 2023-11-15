use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod minimum_validation_0 {
    #[tokio::test]
    async fn test0_above_the_minimum_is_valid() {}
    #[tokio::test]
    async fn test1_boundary_point_is_valid() {}
    #[tokio::test]
    async fn test2_below_the_minimum_is_invalid() {}
    #[tokio::test]
    async fn test3_ignores_non_numbers() {}
}
mod minimum_validation_with_signed_integer_1 {
    #[tokio::test]
    async fn test0_negative_above_the_minimum_is_valid() {}
    #[tokio::test]
    async fn test1_positive_above_the_minimum_is_valid() {}
    #[tokio::test]
    async fn test2_boundary_point_is_valid() {}
    #[tokio::test]
    async fn test3_boundary_point_with_float_is_valid() {}
    #[tokio::test]
    async fn test4_float_below_the_minimum_is_invalid() {}
    #[tokio::test]
    async fn test5_int_below_the_minimum_is_invalid() {}
    #[tokio::test]
    async fn test6_ignores_non_numbers() {}
}
