use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_iri_references_0 {
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
    fn test6_a_valid_iri() {}
    #[test]
    fn test7_a_valid_protocol_relative_iri_reference() {}
    #[test]
    fn test8_a_valid_relative_iri_reference() {}
    #[test]
    fn test9_an_invalid_iri_reference() {}
    #[test]
    fn test10_a_valid_iri_reference() {}
    #[test]
    fn test11_a_valid_iri_fragment() {}
    #[test]
    fn test12_an_invalid_iri_fragment() {}
}
