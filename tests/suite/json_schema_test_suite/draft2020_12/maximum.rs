mod maximum_validation_0 {
    #[tokio::test]
    async fn test0_below_the_maximum_is_valid() {}
    #[tokio::test]
    async fn test1_boundary_point_is_valid() {}
    #[tokio::test]
    async fn test2_above_the_maximum_is_invalid() {}
    #[tokio::test]
    async fn test3_ignores_non_numbers() {}
}
mod maximum_validation_with_unsigned_integer_1 {
    #[tokio::test]
    async fn test0_below_the_maximum_is_invalid() {}
    #[tokio::test]
    async fn test1_boundary_point_integer_is_valid() {}
    #[tokio::test]
    async fn test2_boundary_point_float_is_valid() {}
    #[tokio::test]
    async fn test3_above_the_maximum_is_invalid() {}
}
