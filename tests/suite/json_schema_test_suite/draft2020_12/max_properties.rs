mod max_properties_validation_0 {
    #[tokio::test]
    async fn test0_shorter_is_valid() {}
    #[tokio::test]
    async fn test1_exact_length_is_valid() {}
    #[tokio::test]
    async fn test2_too_long_is_invalid() {}
    #[tokio::test]
    async fn test3_ignores_arrays() {}
    #[tokio::test]
    async fn test4_ignores_strings() {}
    #[tokio::test]
    async fn test5_ignores_other_non_objects() {}
}
mod max_properties_validation_with_a_decimal_1 {
    #[tokio::test]
    async fn test0_shorter_is_valid() {}
    #[tokio::test]
    async fn test1_too_long_is_invalid() {}
}
mod max_properties_0_means_the_object_is_empty_2 {
    #[tokio::test]
    async fn test0_no_properties_is_valid() {}
    #[tokio::test]
    async fn test1_one_property_is_invalid() {}
}
