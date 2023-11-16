use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_an_internationalized_e_mail_addresses_0 {
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
    fn test6_a_valid_idn_e_mail_example_example_test_in_hangul() {}
    #[test]
    fn test7_an_invalid_idn_e_mail_address() {}
    #[test]
    fn test8_a_valid_e_mail_address() {}
    #[test]
    fn test9_an_invalid_e_mail_address() {}
}
