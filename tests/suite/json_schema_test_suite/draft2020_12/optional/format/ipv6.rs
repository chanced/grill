use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_i_pv6_addresses_0 {
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
    fn test6_a_valid_i_pv6_address() {}
    #[test]
    fn test7_an_i_pv6_address_with_out_of_range_values() {}
    #[test]
    fn test8_trailing_4_hex_symbols_is_valid() {}
    #[test]
    fn test9_trailing_5_hex_symbols_is_invalid() {}
    #[test]
    fn test10_an_i_pv6_address_with_too_many_components() {}
    #[test]
    fn test11_an_i_pv6_address_containing_illegal_characters() {}
    #[test]
    fn test12_no_digits_is_valid() {}
    #[test]
    fn test13_leading_colons_is_valid() {}
    #[test]
    fn test14_trailing_colons_is_valid() {}
    #[test]
    fn test15_missing_leading_octet_is_invalid() {}
    #[test]
    fn test16_missing_trailing_octet_is_invalid() {}
    #[test]
    fn test17_missing_leading_octet_with_omitted_octets_later() {}
    #[test]
    fn test18_single_set_of_double_colons_in_the_middle_is_valid() {}
    #[test]
    fn test19_two_sets_of_double_colons_is_invalid() {}
    #[test]
    fn test20_mixed_format_with_the_ipv4_section_as_decimal_octets() {}
    #[test]
    fn test21_mixed_format_with_double_colons_between_the_sections() {}
    #[test]
    fn test22_mixed_format_with_ipv4_section_with_octet_out_of_range() {}
    #[test]
    fn test23_mixed_format_with_ipv4_section_with_a_hex_octet() {}
    #[test]
    fn test24_mixed_format_with_leading_double_colons_ipv4_mapped_ipv6_address() {}
    #[test]
    fn test25_triple_colons_is_invalid() {}
    #[test]
    fn test26_8_octets() {}
    #[test]
    fn test27_insufficient_octets_without_double_colons() {}
    #[test]
    fn test28_no_colons_is_invalid() {}
    #[test]
    fn test29_ipv4_is_not_ipv6() {}
    #[test]
    fn test30_ipv4_segment_must_have_4_octets() {}
    #[test]
    fn test31_leading_whitespace_is_invalid() {}
    #[test]
    fn test32_trailing_whitespace_is_invalid() {}
    #[test]
    fn test33_netmask_is_not_a_part_of_ipv6_address() {}
    #[test]
    fn test34_zone_id_is_not_a_part_of_ipv6_address() {}
    #[test]
    fn test35_a_long_valid_ipv6() {}
    #[test]
    fn test36_a_long_invalid_ipv6_below_length_limit_first() {}
    #[test]
    fn test37_a_long_invalid_ipv6_below_length_limit_second() {}
    #[test]
    fn test38_invalid_non_ascii_৪_a_bengali_4() {}
    #[test]
    fn test39_invalid_non_ascii_৪_a_bengali_4_in_the_i_pv4_portion() {}
}
