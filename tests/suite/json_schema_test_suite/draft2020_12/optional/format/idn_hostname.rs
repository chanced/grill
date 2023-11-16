use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_internationalized_host_names_0 {
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
    fn test6_a_valid_host_name_example_test_in_hangul() {}
    #[test]
    fn test7_illegal_first_char_u_302e_hangul_single_dot_tone_mark() {}
    #[test]
    fn test8_contains_illegal_char_u_302e_hangul_single_dot_tone_mark() {}
    #[test]
    fn test9_a_host_name_with_a_component_too_long() {}
    #[test]
    fn test10_invalid_label_correct_punycode() {}
    #[test]
    fn test11_valid_chinese_punycode() {}
    #[test]
    fn test12_invalid_punycode() {}
    #[test]
    fn test13_u_label_contains_in_the_3rd_and_4th_position() {}
    #[test]
    fn test14_u_label_starts_with_a_dash() {}
    #[test]
    fn test15_u_label_ends_with_a_dash() {}
    #[test]
    fn test16_u_label_starts_and_ends_with_a_dash() {}
    #[test]
    fn test17_begins_with_a_spacing_combining_mark() {}
    #[test]
    fn test18_begins_with_a_nonspacing_mark() {}
    #[test]
    fn test19_begins_with_an_enclosing_mark() {}
    #[test]
    fn test20_exceptions_that_are_pvalid_left_to_right_chars() {}
    #[test]
    fn test21_exceptions_that_are_pvalid_right_to_left_chars() {}
    #[test]
    fn test22_exceptions_that_are_disallowed_right_to_left_chars() {}
    #[test]
    fn test23_exceptions_that_are_disallowed_left_to_right_chars() {}
    #[test]
    fn test24_middle_dot_with_no_preceding_l() {}
    #[test]
    fn test25_middle_dot_with_nothing_preceding() {}
    #[test]
    fn test26_middle_dot_with_no_following_l() {}
    #[test]
    fn test27_middle_dot_with_nothing_following() {}
    #[test]
    fn test28_middle_dot_with_surrounding_l_s() {}
    #[test]
    fn test29_greek_keraia_not_followed_by_greek() {}
    #[test]
    fn test30_greek_keraia_not_followed_by_anything() {}
    #[test]
    fn test31_greek_keraia_followed_by_greek() {}
    #[test]
    fn test32_hebrew_geresh_not_preceded_by_hebrew() {}
    #[test]
    fn test33_hebrew_geresh_not_preceded_by_anything() {}
    #[test]
    fn test34_hebrew_geresh_preceded_by_hebrew() {}
    #[test]
    fn test35_hebrew_gershayim_not_preceded_by_hebrew() {}
    #[test]
    fn test36_hebrew_gershayim_not_preceded_by_anything() {}
    #[test]
    fn test37_hebrew_gershayim_preceded_by_hebrew() {}
    #[test]
    fn test38_katakana_middle_dot_with_no_hiragana_katakana_or_han() {}
    #[test]
    fn test39_katakana_middle_dot_with_no_other_characters() {}
    #[test]
    fn test40_katakana_middle_dot_with_hiragana() {}
    #[test]
    fn test41_katakana_middle_dot_with_katakana() {}
    #[test]
    fn test42_katakana_middle_dot_with_han() {}
    #[test]
    fn test43_arabic_indic_digits_mixed_with_extended_arabic_indic_digits() {}
    #[test]
    fn test44_arabic_indic_digits_not_mixed_with_extended_arabic_indic_digits() {}
    #[test]
    fn test45_extended_arabic_indic_digits_not_mixed_with_arabic_indic_digits() {}
    #[test]
    fn test46_zero_width_joiner_not_preceded_by_virama() {}
    #[test]
    fn test47_zero_width_joiner_not_preceded_by_anything() {}
    #[test]
    fn test48_zero_width_joiner_preceded_by_virama() {}
    #[test]
    fn test49_zero_width_non_joiner_preceded_by_virama() {}
    #[test]
    fn test50_zero_width_non_joiner_not_preceded_by_virama_but_matches_regexp() {}
}
