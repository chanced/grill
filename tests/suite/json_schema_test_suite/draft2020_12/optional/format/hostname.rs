use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_host_names_0 {
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
    fn test6_a_valid_host_name() {}
    #[test]
    fn test7_a_valid_punycoded_idn_hostname() {}
    #[test]
    fn test8_a_host_name_starting_with_an_illegal_character() {}
    #[test]
    fn test9_a_host_name_containing_illegal_characters() {}
    #[test]
    fn test10_a_host_name_with_a_component_too_long() {}
    #[test]
    fn test11_starts_with_hyphen() {}
    #[test]
    fn test12_ends_with_hyphen() {}
    #[test]
    fn test13_starts_with_underscore() {}
    #[test]
    fn test14_ends_with_underscore() {}
    #[test]
    fn test15_contains_underscore() {}
    #[test]
    fn test16_maximum_label_length() {}
    #[test]
    fn test17_exceeds_maximum_label_length() {}
}
