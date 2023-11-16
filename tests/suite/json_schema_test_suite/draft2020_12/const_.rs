mod const_validation_0 {
    #[test]
    fn test0_same_value_is_valid() {}
    #[test]
    fn test1_another_value_is_invalid() {}
    #[test]
    fn test2_another_type_is_invalid() {}
}
mod const_with_object_1 {
    #[test]
    fn test0_same_object_is_valid() {}
    #[test]
    fn test1_same_object_with_different_property_order_is_valid() {}
    #[test]
    fn test2_another_object_is_invalid() {}
    #[test]
    fn test3_another_type_is_invalid() {}
}
mod const_with_array_2 {
    #[test]
    fn test0_same_array_is_valid() {}
    #[test]
    fn test1_another_array_item_is_invalid() {}
    #[test]
    fn test2_array_with_additional_items_is_invalid() {}
}
mod const_with_null_3 {
    #[test]
    fn test0_null_is_valid() {}
    #[test]
    fn test1_not_null_is_invalid() {}
}
mod const_with_false_does_not_match_0_4 {
    #[test]
    fn test0_false_is_valid() {}
    #[test]
    fn test1_integer_zero_is_invalid() {}
    #[test]
    fn test2_float_zero_is_invalid() {}
}
mod const_with_true_does_not_match_1_5 {
    #[test]
    fn test0_true_is_valid() {}
    #[test]
    fn test1_integer_one_is_invalid() {}
    #[test]
    fn test2_float_one_is_invalid() {}
}
mod const_with_false_does_not_match_0_6 {
    #[test]
    fn test0_false_is_valid() {}
    #[test]
    fn test1_0_is_invalid() {}
    #[test]
    fn test2_0_0_is_invalid() {}
}
mod const_with_true_does_not_match_1_7 {
    #[test]
    fn test0_true_is_valid() {}
    #[test]
    fn test1_1_is_invalid() {}
    #[test]
    fn test2_1_0_is_invalid() {}
}
mod const_with_a_false_does_not_match_a_0_8 {
    #[test]
    fn test0_a_false_is_valid() {}
    #[test]
    fn test1_a_0_is_invalid() {}
    #[test]
    fn test2_a_0_0_is_invalid() {}
}
mod const_with_a_true_does_not_match_a_1_9 {
    #[test]
    fn test0_a_true_is_valid() {}
    #[test]
    fn test1_a_1_is_invalid() {}
    #[test]
    fn test2_a_1_0_is_invalid() {}
}
mod const_with_0_does_not_match_other_zero_like_types_10 {
    #[test]
    fn test0_false_is_invalid() {}
    #[test]
    fn test1_integer_zero_is_valid() {}
    #[test]
    fn test2_float_zero_is_valid() {}
    #[test]
    fn test3_empty_object_is_invalid() {}
    #[test]
    fn test4_empty_array_is_invalid() {}
    #[test]
    fn test5_empty_string_is_invalid() {}
}
mod const_with_1_does_not_match_true_11 {
    #[test]
    fn test0_true_is_invalid() {}
    #[test]
    fn test1_integer_one_is_valid() {}
    #[test]
    fn test2_float_one_is_valid() {}
}
mod const_with_2_0_matches_integer_and_float_types_12 {
    #[test]
    fn test0_integer_2_is_valid() {}
    #[test]
    fn test1_integer_2_is_invalid() {}
    #[test]
    fn test2_float_2_0_is_valid() {}
    #[test]
    fn test3_float_2_0_is_invalid() {}
    #[test]
    fn test4_float_2_00001_is_invalid() {}
}
mod float_and_integers_are_equal_up_to_64_bit_representation_limits_13 {
    #[test]
    fn test0_integer_is_valid() {}
    #[test]
    fn test1_integer_minus_one_is_invalid() {}
    #[test]
    fn test2_float_is_valid() {}
    #[test]
    fn test3_float_minus_one_is_invalid() {}
}
mod nul_characters_in_strings_14 {
    #[test]
    fn test0_match_string_with_nul() {}
    #[test]
    fn test1_do_not_match_string_lacking_nul() {}
}
