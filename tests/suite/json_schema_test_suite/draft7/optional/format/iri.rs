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
mod validation_of_ir_is_0 {
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
    async fn test6_a_valid_iri_with_anchor_tag() {}
    #[tokio::test]
    async fn test7_a_valid_iri_with_anchor_tag_and_parentheses() {}
    #[tokio::test]
    async fn test8_a_valid_iri_with_url_encoded_stuff() {}
    #[tokio::test]
    async fn test9_a_valid_iri_with_many_special_characters() {}
    #[tokio::test]
    async fn test10_a_valid_iri_based_on_i_pv6() {}
    #[tokio::test]
    async fn test11_an_invalid_iri_based_on_i_pv6() {}
    #[tokio::test]
    async fn test12_an_invalid_relative_iri_reference() {}
    #[tokio::test]
    async fn test13_an_invalid_iri() {}
    #[tokio::test]
    async fn test14_an_invalid_iri_though_valid_iri_reference() {}
}
