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
mod validation_of_ur_is_0 {
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
    async fn test6_a_valid_url_with_anchor_tag() {}
    #[tokio::test]
    async fn test7_a_valid_url_with_anchor_tag_and_parentheses() {}
    #[tokio::test]
    async fn test8_a_valid_url_with_url_encoded_stuff() {}
    #[tokio::test]
    async fn test9_a_valid_puny_coded_url() {}
    #[tokio::test]
    async fn test10_a_valid_url_with_many_special_characters() {}
    #[tokio::test]
    async fn test11_a_valid_url_based_on_i_pv4() {}
    #[tokio::test]
    async fn test12_a_valid_url_with_ftp_scheme() {}
    #[tokio::test]
    async fn test13_a_valid_url_for_a_simple_text_file() {}
    #[tokio::test]
    async fn test14_a_valid_url() {}
    #[tokio::test]
    async fn test15_a_valid_mailto_uri() {}
    #[tokio::test]
    async fn test16_a_valid_newsgroup_uri() {}
    #[tokio::test]
    async fn test17_a_valid_tel_uri() {}
    #[tokio::test]
    async fn test18_a_valid_urn() {}
    #[tokio::test]
    async fn test19_an_invalid_protocol_relative_uri_reference() {}
    #[tokio::test]
    async fn test20_an_invalid_relative_uri_reference() {}
    #[tokio::test]
    async fn test21_an_invalid_uri() {}
    #[tokio::test]
    async fn test22_an_invalid_uri_though_valid_uri_reference() {}
    #[tokio::test]
    async fn test23_an_invalid_uri_with_spaces() {}
    #[tokio::test]
    async fn test24_an_invalid_uri_with_spaces_and_missing_scheme() {}
    #[tokio::test]
    async fn test25_an_invalid_uri_with_comma_in_scheme() {}
}
