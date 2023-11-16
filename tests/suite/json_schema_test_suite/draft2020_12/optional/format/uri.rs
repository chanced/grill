use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_ur_is_0 {
    #[test]
    fn test0_all_string_formats_ignore_integers() {}
    #[test]
    fn test1_all_string_formats_ignore_floats() {}
    #[test]
    fn test2_all_string_formats_ignore_objects() {}
    #[test]
    fn test3_all_string_formats_ignore_arrays() {}
    #[test]
    fn test4_all_string_formats_ignore_booleans() {}
    #[test]
    fn test5_all_string_formats_ignore_nulls() {}
    #[test]
    fn test6_a_valid_url_with_anchor_tag() {}
    #[test]
    fn test7_a_valid_url_with_anchor_tag_and_parentheses() {}
    #[test]
    fn test8_a_valid_url_with_url_encoded_stuff() {}
    #[test]
    fn test9_a_valid_puny_coded_url() {}
    #[test]
    fn test10_a_valid_url_with_many_special_characters() {}
    #[test]
    fn test11_a_valid_url_based_on_i_pv4() {}
    #[test]
    fn test12_a_valid_url_with_ftp_scheme() {}
    #[test]
    fn test13_a_valid_url_for_a_simple_text_file() {}
    #[test]
    fn test14_a_valid_url() {}
    #[test]
    fn test15_a_valid_mailto_uri() {}
    #[test]
    fn test16_a_valid_newsgroup_uri() {}
    #[test]
    fn test17_a_valid_tel_uri() {}
    #[test]
    fn test18_a_valid_urn() {}
    #[test]
    fn test19_an_invalid_protocol_relative_uri_reference() {}
    #[test]
    fn test20_an_invalid_relative_uri_reference() {}
    #[test]
    fn test21_an_invalid_uri() {}
    #[test]
    fn test22_an_invalid_uri_though_valid_uri_reference() {}
    #[test]
    fn test23_an_invalid_uri_with_spaces() {}
    #[test]
    fn test24_an_invalid_uri_with_spaces_and_missing_scheme() {}
    #[test]
    fn test25_an_invalid_uri_with_comma_in_scheme() {}
}
