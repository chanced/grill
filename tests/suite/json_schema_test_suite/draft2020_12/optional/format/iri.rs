use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_ir_is_0 {
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
    fn test6_a_valid_iri_with_anchor_tag() {}
    #[test]
    fn test7_a_valid_iri_with_anchor_tag_and_parentheses() {}
    #[test]
    fn test8_a_valid_iri_with_url_encoded_stuff() {}
    #[test]
    fn test9_a_valid_iri_with_many_special_characters() {}
    #[test]
    fn test10_a_valid_iri_based_on_i_pv6() {}
    #[test]
    fn test11_an_invalid_iri_based_on_i_pv6() {}
    #[test]
    fn test12_an_invalid_relative_iri_reference() {}
    #[test]
    fn test13_an_invalid_iri() {}
    #[test]
    fn test14_an_invalid_iri_though_valid_iri_reference() {}
}
