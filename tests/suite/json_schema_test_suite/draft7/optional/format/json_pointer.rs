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
mod validation_of_json_pointers_json_string_representation_0 {
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
    async fn test6_a_valid_json_pointer() {}
    #[tokio::test]
    async fn test7_not_a_valid_json_pointer_not_escaped() {}
    #[tokio::test]
    async fn test8_valid_json_pointer_with_empty_segment() {}
    #[tokio::test]
    async fn test9_valid_json_pointer_with_the_last_empty_segment() {}
    #[tokio::test]
    async fn test10_valid_json_pointer_as_stated_in_rfc_6901_1() {}
    #[tokio::test]
    async fn test11_valid_json_pointer_as_stated_in_rfc_6901_2() {}
    #[tokio::test]
    async fn test12_valid_json_pointer_as_stated_in_rfc_6901_3() {}
    #[tokio::test]
    async fn test13_valid_json_pointer_as_stated_in_rfc_6901_4() {}
    #[tokio::test]
    async fn test14_valid_json_pointer_as_stated_in_rfc_6901_5() {}
    #[tokio::test]
    async fn test15_valid_json_pointer_as_stated_in_rfc_6901_6() {}
    #[tokio::test]
    async fn test16_valid_json_pointer_as_stated_in_rfc_6901_7() {}
    #[tokio::test]
    async fn test17_valid_json_pointer_as_stated_in_rfc_6901_8() {}
    #[tokio::test]
    async fn test18_valid_json_pointer_as_stated_in_rfc_6901_9() {}
    #[tokio::test]
    async fn test19_valid_json_pointer_as_stated_in_rfc_6901_10() {}
    #[tokio::test]
    async fn test20_valid_json_pointer_as_stated_in_rfc_6901_11() {}
    #[tokio::test]
    async fn test21_valid_json_pointer_as_stated_in_rfc_6901_12() {}
    #[tokio::test]
    async fn test22_valid_json_pointer_used_adding_to_the_last_array_position() {}
    #[tokio::test]
    async fn test23_valid_json_pointer_used_as_object_member_name() {}
    #[tokio::test]
    async fn test24_valid_json_pointer_multiple_escaped_characters() {}
    #[tokio::test]
    async fn test25_valid_json_pointer_escaped_with_fraction_part_1() {}
    #[tokio::test]
    async fn test26_valid_json_pointer_escaped_with_fraction_part_2() {}
    #[tokio::test]
    async fn test27_not_a_valid_json_pointer_uri_fragment_identifier_1() {}
    #[tokio::test]
    async fn test28_not_a_valid_json_pointer_uri_fragment_identifier_2() {}
    #[tokio::test]
    async fn test29_not_a_valid_json_pointer_uri_fragment_identifier_3() {}
    #[tokio::test]
    async fn test30_not_a_valid_json_pointer_some_escaped_but_not_all_1() {}
    #[tokio::test]
    async fn test31_not_a_valid_json_pointer_some_escaped_but_not_all_2() {}
    #[tokio::test]
    async fn test32_not_a_valid_json_pointer_wrong_escape_character_1() {}
    #[tokio::test]
    async fn test33_not_a_valid_json_pointer_wrong_escape_character_2() {}
    #[tokio::test]
    async fn test34_not_a_valid_json_pointer_multiple_characters_not_escaped() {}
    #[tokio::test]
    async fn test35_not_a_valid_json_pointer_isn_t_empty_nor_starts_with_1() {}
    #[tokio::test]
    async fn test36_not_a_valid_json_pointer_isn_t_empty_nor_starts_with_2() {}
    #[tokio::test]
    async fn test37_not_a_valid_json_pointer_isn_t_empty_nor_starts_with_3() {}
}
