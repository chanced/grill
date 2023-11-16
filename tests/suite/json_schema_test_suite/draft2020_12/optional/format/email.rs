use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_e_mail_addresses_0 {
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
    fn test6_a_valid_e_mail_address() {}
    #[test]
    fn test7_an_invalid_e_mail_address() {}
    #[test]
    fn test8_tilde_in_local_part_is_valid() {}
    #[test]
    fn test9_tilde_before_local_part_is_valid() {}
    #[test]
    fn test10_tilde_after_local_part_is_valid() {}
    #[test]
    fn test11_a_quoted_string_with_a_space_in_the_local_part_is_valid() {}
    #[test]
    fn test12_a_quoted_string_with_a_double_dot_in_the_local_part_is_valid() {}
    #[test]
    fn test13_a_quoted_string_with_a_in_the_local_part_is_valid() {}
    #[test]
    fn test14_an_i_pv4_address_literal_after_the_is_valid() {}
    #[test]
    fn test15_an_i_pv6_address_literal_after_the_is_valid() {}
    #[test]
    fn test16_dot_before_local_part_is_not_valid() {}
    #[test]
    fn test17_dot_after_local_part_is_not_valid() {}
    #[test]
    fn test18_two_separated_dots_inside_local_part_are_valid() {}
    #[test]
    fn test19_two_subsequent_dots_inside_local_part_are_not_valid() {}
    #[test]
    fn test20_an_invalid_domain() {}
    #[test]
    fn test21_an_invalid_i_pv4_address_literal() {}
}
