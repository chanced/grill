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
mod validation_of_i_pv6_addresses_0 {
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
    async fn test6_a_valid_i_pv6_address() {}
    #[tokio::test]
    async fn test7_an_i_pv6_address_with_out_of_range_values() {}
    #[tokio::test]
    async fn test8_trailing_4_hex_symbols_is_valid() {}
    #[tokio::test]
    async fn test9_trailing_5_hex_symbols_is_invalid() {}
    #[tokio::test]
    async fn test10_an_i_pv6_address_with_too_many_components() {}
    #[tokio::test]
    async fn test11_an_i_pv6_address_containing_illegal_characters() {}
    #[tokio::test]
    async fn test12_no_digits_is_valid() {}
    #[tokio::test]
    async fn test13_leading_colons_is_valid() {}
    #[tokio::test]
    async fn test14_trailing_colons_is_valid() {}
    #[tokio::test]
    async fn test15_missing_leading_octet_is_invalid() {}
    #[tokio::test]
    async fn test16_missing_trailing_octet_is_invalid() {}
    #[tokio::test]
    async fn test17_missing_leading_octet_with_omitted_octets_later() {}
    #[tokio::test]
    async fn test18_single_set_of_double_colons_in_the_middle_is_valid() {}
    #[tokio::test]
    async fn test19_two_sets_of_double_colons_is_invalid() {}
    #[tokio::test]
    async fn test20_mixed_format_with_the_ipv4_section_as_decimal_octets() {}
    #[tokio::test]
    async fn test21_mixed_format_with_double_colons_between_the_sections() {}
    #[tokio::test]
    async fn test22_mixed_format_with_ipv4_section_with_octet_out_of_range() {}
    #[tokio::test]
    async fn test23_mixed_format_with_ipv4_section_with_a_hex_octet() {}
    #[tokio::test]
    async fn test24_mixed_format_with_leading_double_colons_ipv4_mapped_ipv6_address() {}
    #[tokio::test]
    async fn test25_triple_colons_is_invalid() {}
    #[tokio::test]
    async fn test26_8_octets() {}
    #[tokio::test]
    async fn test27_insufficient_octets_without_double_colons() {}
    #[tokio::test]
    async fn test28_no_colons_is_invalid() {}
    #[tokio::test]
    async fn test29_ipv4_is_not_ipv6() {}
    #[tokio::test]
    async fn test30_ipv4_segment_must_have_4_octets() {}
    #[tokio::test]
    async fn test31_leading_whitespace_is_invalid() {}
    #[tokio::test]
    async fn test32_trailing_whitespace_is_invalid() {}
    #[tokio::test]
    async fn test33_netmask_is_not_a_part_of_ipv6_address() {}
    #[tokio::test]
    async fn test34_zone_id_is_not_a_part_of_ipv6_address() {}
    #[tokio::test]
    async fn test35_a_long_valid_ipv6() {}
    #[tokio::test]
    async fn test36_a_long_invalid_ipv6_below_length_limit_first() {}
    #[tokio::test]
    async fn test37_a_long_invalid_ipv6_below_length_limit_second() {}
    #[tokio::test]
    async fn test38_invalid_non_ascii_৪_a_bengali_4() {}
    #[tokio::test]
    async fn test39_invalid_non_ascii_৪_a_bengali_4_in_the_i_pv4_portion() {}
}
