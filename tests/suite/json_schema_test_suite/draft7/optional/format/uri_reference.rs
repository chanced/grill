use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
async fn interrogator() {
    todo!()
}
async fn interrogator() {
    todo!()
}
mod validation_of_uri_references_0 {
    #[tokio::test]
    async fn test0_all_string_formats_ignore_integers() {}
    #[tokio::test]
    async fn test1_all_string_formats_ignore_floats() {}
    #[tokio::test]
    async fn test2_all_string_formats_ignore_objects() {}
    #[tokio::test]
    async fn test3_all_string_formats_ignore_arrays() {}
    #[tokio::test]
    async fn test4_all_string_formats_ignore_booleans() {}
    #[tokio::test]
    async fn test5_all_string_formats_ignore_nulls() {}
    #[tokio::test]
    async fn test6_a_valid_uri() {}
    #[tokio::test]
    async fn test7_a_valid_protocol_relative_uri_reference() {}
    #[tokio::test]
    async fn test8_a_valid_relative_uri_reference() {}
    #[tokio::test]
    async fn test9_an_invalid_uri_reference() {}
    #[tokio::test]
    async fn test10_a_valid_uri_reference() {}
    #[tokio::test]
    async fn test11_a_valid_uri_fragment() {}
    #[tokio::test]
    async fn test12_an_invalid_uri_fragment() {}
}
