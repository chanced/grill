use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod not_0 {
    #[tokio::test]
    async fn test0_allowed() {}
    #[tokio::test]
    async fn test1_disallowed() {}
}
mod not_multiple_types_1 {
    #[tokio::test]
    async fn test0_valid() {}
    #[tokio::test]
    async fn test1_mismatch() {}
    #[tokio::test]
    async fn test2_other_mismatch() {}
}
mod not_more_complex_schema_2 {
    #[tokio::test]
    async fn test0_match_() {}
    #[tokio::test]
    async fn test1_other_match() {}
    #[tokio::test]
    async fn test2_mismatch() {}
}
mod forbidden_property_3 {
    #[tokio::test]
    async fn test0_property_present() {}
    #[tokio::test]
    async fn test1_property_absent() {}
}
mod not_with_boolean_schema_true_4 {
    #[tokio::test]
    async fn test0_any_value_is_invalid() {}
}
mod not_with_boolean_schema_false_5 {
    #[tokio::test]
    async fn test0_any_value_is_valid() {}
}
