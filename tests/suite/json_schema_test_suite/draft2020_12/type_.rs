mod integer_type_matches_integers_0 {
    #[test]
    fn test0_an_integer_is_an_integer() {}
    #[test]
    fn test1_a_float_with_zero_fractional_part_is_an_integer() {}
    #[test]
    fn test2_a_float_is_not_an_integer() {}
    #[test]
    fn test3_a_string_is_not_an_integer() {}
    #[test]
    fn test4_a_string_is_still_not_an_integer_even_if_it_looks_like_one() {}
    #[test]
    fn test5_an_object_is_not_an_integer() {}
    #[test]
    fn test6_an_array_is_not_an_integer() {}
    #[test]
    fn test7_a_boolean_is_not_an_integer() {}
    #[test]
    fn test8_null_is_not_an_integer() {}
}
mod number_type_matches_numbers_1 {
    #[test]
    fn test0_an_integer_is_a_number() {}
    #[test]
    fn test1_a_float_with_zero_fractional_part_is_a_number_and_an_integer() {}
    #[test]
    fn test2_a_float_is_a_number() {}
    #[test]
    fn test3_a_string_is_not_a_number() {}
    #[test]
    fn test4_a_string_is_still_not_a_number_even_if_it_looks_like_one() {}
    #[test]
    fn test5_an_object_is_not_a_number() {}
    #[test]
    fn test6_an_array_is_not_a_number() {}
    #[test]
    fn test7_a_boolean_is_not_a_number() {}
    #[test]
    fn test8_null_is_not_a_number() {}
}
mod string_type_matches_strings_2 {
    #[test]
    fn test0_1_is_not_a_string() {}
    #[test]
    fn test1_a_float_is_not_a_string() {}
    #[test]
    fn test2_a_string_is_a_string() {}
    #[test]
    fn test3_a_string_is_still_a_string_even_if_it_looks_like_a_number() {}
    #[test]
    fn test4_an_empty_string_is_still_a_string() {}
    #[test]
    fn test5_an_object_is_not_a_string() {}
    #[test]
    fn test6_an_array_is_not_a_string() {}
    #[test]
    fn test7_a_boolean_is_not_a_string() {}
    #[test]
    fn test8_null_is_not_a_string() {}
}
mod object_type_matches_objects_3 {
    #[test]
    fn test0_an_integer_is_not_an_object() {}
    #[test]
    fn test1_a_float_is_not_an_object() {}
    #[test]
    fn test2_a_string_is_not_an_object() {}
    #[test]
    fn test3_an_object_is_an_object() {}
    #[test]
    fn test4_an_array_is_not_an_object() {}
    #[test]
    fn test5_a_boolean_is_not_an_object() {}
    #[test]
    fn test6_null_is_not_an_object() {}
}
mod array_type_matches_arrays_4 {
    #[test]
    fn test0_an_integer_is_not_an_array() {}
    #[test]
    fn test1_a_float_is_not_an_array() {}
    #[test]
    fn test2_a_string_is_not_an_array() {}
    #[test]
    fn test3_an_object_is_not_an_array() {}
    #[test]
    fn test4_an_array_is_an_array() {}
    #[test]
    fn test5_a_boolean_is_not_an_array() {}
    #[test]
    fn test6_null_is_not_an_array() {}
}
mod boolean_type_matches_booleans_5 {
    #[test]
    fn test0_an_integer_is_not_a_boolean() {}
    #[test]
    fn test1_zero_is_not_a_boolean() {}
    #[test]
    fn test2_a_float_is_not_a_boolean() {}
    #[test]
    fn test3_a_string_is_not_a_boolean() {}
    #[test]
    fn test4_an_empty_string_is_not_a_boolean() {}
    #[test]
    fn test5_an_object_is_not_a_boolean() {}
    #[test]
    fn test6_an_array_is_not_a_boolean() {}
    #[test]
    fn test7_true_is_a_boolean() {}
    #[test]
    fn test8_false_is_a_boolean() {}
    #[test]
    fn test9_null_is_not_a_boolean() {}
}
mod null_type_matches_only_the_null_object_6 {
    #[test]
    fn test0_an_integer_is_not_null() {}
    #[test]
    fn test1_a_float_is_not_null() {}
    #[test]
    fn test2_zero_is_not_null() {}
    #[test]
    fn test3_a_string_is_not_null() {}
    #[test]
    fn test4_an_empty_string_is_not_null() {}
    #[test]
    fn test5_an_object_is_not_null() {}
    #[test]
    fn test6_an_array_is_not_null() {}
    #[test]
    fn test7_true_is_not_null() {}
    #[test]
    fn test8_false_is_not_null() {}
    #[test]
    fn test9_null_is_null() {}
}
mod multiple_types_can_be_specified_in_an_array_7 {
    #[test]
    fn test0_an_integer_is_valid() {}
    #[test]
    fn test1_a_string_is_valid() {}
    #[test]
    fn test2_a_float_is_invalid() {}
    #[test]
    fn test3_an_object_is_invalid() {}
    #[test]
    fn test4_an_array_is_invalid() {}
    #[test]
    fn test5_a_boolean_is_invalid() {}
    #[test]
    fn test6_null_is_invalid() {}
}
mod type_as_array_with_one_item_8 {
    #[test]
    fn test0_string_is_valid() {}
    #[test]
    fn test1_number_is_invalid() {}
}
mod type_array_or_object_9 {
    #[test]
    fn test0_array_is_valid() {}
    #[test]
    fn test1_object_is_valid() {}
    #[test]
    fn test2_number_is_invalid() {}
    #[test]
    fn test3_string_is_invalid() {}
    #[test]
    fn test4_null_is_invalid() {}
}
mod type_array_object_or_null_10 {
    #[test]
    fn test0_array_is_valid() {}
    #[test]
    fn test1_object_is_valid() {}
    #[test]
    fn test2_null_is_valid() {}
    #[test]
    fn test3_number_is_invalid() {}
    #[test]
    fn test4_string_is_invalid() {}
}
