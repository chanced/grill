use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod validation_of_json_pointers_json_string_representation_0 {
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
    fn test6_a_valid_json_pointer() {}
    #[test]
    fn test7_not_a_valid_json_pointer_not_escaped() {}
    #[test]
    fn test8_valid_json_pointer_with_empty_segment() {}
    #[test]
    fn test9_valid_json_pointer_with_the_last_empty_segment() {}
    #[test]
    fn test10_valid_json_pointer_as_stated_in_rfc_6901_1() {}
    #[test]
    fn test11_valid_json_pointer_as_stated_in_rfc_6901_2() {}
    #[test]
    fn test12_valid_json_pointer_as_stated_in_rfc_6901_3() {}
    #[test]
    fn test13_valid_json_pointer_as_stated_in_rfc_6901_4() {}
    #[test]
    fn test14_valid_json_pointer_as_stated_in_rfc_6901_5() {}
    #[test]
    fn test15_valid_json_pointer_as_stated_in_rfc_6901_6() {}
    #[test]
    fn test16_valid_json_pointer_as_stated_in_rfc_6901_7() {}
    #[test]
    fn test17_valid_json_pointer_as_stated_in_rfc_6901_8() {}
    #[test]
    fn test18_valid_json_pointer_as_stated_in_rfc_6901_9() {}
    #[test]
    fn test19_valid_json_pointer_as_stated_in_rfc_6901_10() {}
    #[test]
    fn test20_valid_json_pointer_as_stated_in_rfc_6901_11() {}
    #[test]
    fn test21_valid_json_pointer_as_stated_in_rfc_6901_12() {}
    #[test]
    fn test22_valid_json_pointer_used_adding_to_the_last_array_position() {}
    #[test]
    fn test23_valid_json_pointer_used_as_object_member_name() {}
    #[test]
    fn test24_valid_json_pointer_multiple_escaped_characters() {}
    #[test]
    fn test25_valid_json_pointer_escaped_with_fraction_part_1() {}
    #[test]
    fn test26_valid_json_pointer_escaped_with_fraction_part_2() {}
    #[test]
    fn test27_not_a_valid_json_pointer_uri_fragment_identifier_1() {}
    #[test]
    fn test28_not_a_valid_json_pointer_uri_fragment_identifier_2() {}
    #[test]
    fn test29_not_a_valid_json_pointer_uri_fragment_identifier_3() {}
    #[test]
    fn test30_not_a_valid_json_pointer_some_escaped_but_not_all_1() {}
    #[test]
    fn test31_not_a_valid_json_pointer_some_escaped_but_not_all_2() {}
    #[test]
    fn test32_not_a_valid_json_pointer_wrong_escape_character_1() {}
    #[test]
    fn test33_not_a_valid_json_pointer_wrong_escape_character_2() {}
    #[test]
    fn test34_not_a_valid_json_pointer_multiple_characters_not_escaped() {}
    #[test]
    fn test35_not_a_valid_json_pointer_isn_t_empty_nor_starts_with_1() {}
    #[test]
    fn test36_not_a_valid_json_pointer_isn_t_empty_nor_starts_with_2() {}
    #[test]
    fn test37_not_a_valid_json_pointer_isn_t_empty_nor_starts_with_3() {}
}
