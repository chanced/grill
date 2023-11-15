use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod integer_type_matches_integers_0 {
    #[tokio::test]
    async fn test0_an_integer_is_an_integer() {}
    #[tokio::test]
    async fn test1_a_float_with_zero_fractional_part_is_an_integer() {}
    #[tokio::test]
    async fn test2_a_float_is_not_an_integer() {}
    #[tokio::test]
    async fn test3_a_string_is_not_an_integer() {}
    #[tokio::test]
    async fn test4_a_string_is_still_not_an_integer_even_if_it_looks_like_one() {}
    #[tokio::test]
    async fn test5_an_object_is_not_an_integer() {}
    #[tokio::test]
    async fn test6_an_array_is_not_an_integer() {}
    #[tokio::test]
    async fn test7_a_boolean_is_not_an_integer() {}
    #[tokio::test]
    async fn test8_null_is_not_an_integer() {}
}
mod number_type_matches_numbers_1 {
    #[tokio::test]
    async fn test0_an_integer_is_a_number() {}
    #[tokio::test]
    async fn test1_a_float_with_zero_fractional_part_is_a_number_and_an_integer() {}
    #[tokio::test]
    async fn test2_a_float_is_a_number() {}
    #[tokio::test]
    async fn test3_a_string_is_not_a_number() {}
    #[tokio::test]
    async fn test4_a_string_is_still_not_a_number_even_if_it_looks_like_one() {}
    #[tokio::test]
    async fn test5_an_object_is_not_a_number() {}
    #[tokio::test]
    async fn test6_an_array_is_not_a_number() {}
    #[tokio::test]
    async fn test7_a_boolean_is_not_a_number() {}
    #[tokio::test]
    async fn test8_null_is_not_a_number() {}
}
mod string_type_matches_strings_2 {
    #[tokio::test]
    async fn test0_1_is_not_a_string() {}
    #[tokio::test]
    async fn test1_a_float_is_not_a_string() {}
    #[tokio::test]
    async fn test2_a_string_is_a_string() {}
    #[tokio::test]
    async fn test3_a_string_is_still_a_string_even_if_it_looks_like_a_number() {}
    #[tokio::test]
    async fn test4_an_empty_string_is_still_a_string() {}
    #[tokio::test]
    async fn test5_an_object_is_not_a_string() {}
    #[tokio::test]
    async fn test6_an_array_is_not_a_string() {}
    #[tokio::test]
    async fn test7_a_boolean_is_not_a_string() {}
    #[tokio::test]
    async fn test8_null_is_not_a_string() {}
}
mod object_type_matches_objects_3 {
    #[tokio::test]
    async fn test0_an_integer_is_not_an_object() {}
    #[tokio::test]
    async fn test1_a_float_is_not_an_object() {}
    #[tokio::test]
    async fn test2_a_string_is_not_an_object() {}
    #[tokio::test]
    async fn test3_an_object_is_an_object() {}
    #[tokio::test]
    async fn test4_an_array_is_not_an_object() {}
    #[tokio::test]
    async fn test5_a_boolean_is_not_an_object() {}
    #[tokio::test]
    async fn test6_null_is_not_an_object() {}
}
mod array_type_matches_arrays_4 {
    #[tokio::test]
    async fn test0_an_integer_is_not_an_array() {}
    #[tokio::test]
    async fn test1_a_float_is_not_an_array() {}
    #[tokio::test]
    async fn test2_a_string_is_not_an_array() {}
    #[tokio::test]
    async fn test3_an_object_is_not_an_array() {}
    #[tokio::test]
    async fn test4_an_array_is_an_array() {}
    #[tokio::test]
    async fn test5_a_boolean_is_not_an_array() {}
    #[tokio::test]
    async fn test6_null_is_not_an_array() {}
}
mod boolean_type_matches_booleans_5 {
    #[tokio::test]
    async fn test0_an_integer_is_not_a_boolean() {}
    #[tokio::test]
    async fn test1_zero_is_not_a_boolean() {}
    #[tokio::test]
    async fn test2_a_float_is_not_a_boolean() {}
    #[tokio::test]
    async fn test3_a_string_is_not_a_boolean() {}
    #[tokio::test]
    async fn test4_an_empty_string_is_not_a_boolean() {}
    #[tokio::test]
    async fn test5_an_object_is_not_a_boolean() {}
    #[tokio::test]
    async fn test6_an_array_is_not_a_boolean() {}
    #[tokio::test]
    async fn test7_true_is_a_boolean() {}
    #[tokio::test]
    async fn test8_false_is_a_boolean() {}
    #[tokio::test]
    async fn test9_null_is_not_a_boolean() {}
}
mod null_type_matches_only_the_null_object_6 {
    #[tokio::test]
    async fn test0_an_integer_is_not_null() {}
    #[tokio::test]
    async fn test1_a_float_is_not_null() {}
    #[tokio::test]
    async fn test2_zero_is_not_null() {}
    #[tokio::test]
    async fn test3_a_string_is_not_null() {}
    #[tokio::test]
    async fn test4_an_empty_string_is_not_null() {}
    #[tokio::test]
    async fn test5_an_object_is_not_null() {}
    #[tokio::test]
    async fn test6_an_array_is_not_null() {}
    #[tokio::test]
    async fn test7_true_is_not_null() {}
    #[tokio::test]
    async fn test8_false_is_not_null() {}
    #[tokio::test]
    async fn test9_null_is_null() {}
}
mod multiple_types_can_be_specified_in_an_array_7 {
    #[tokio::test]
    async fn test0_an_integer_is_valid() {}
    #[tokio::test]
    async fn test1_a_string_is_valid() {}
    #[tokio::test]
    async fn test2_a_float_is_invalid() {}
    #[tokio::test]
    async fn test3_an_object_is_invalid() {}
    #[tokio::test]
    async fn test4_an_array_is_invalid() {}
    #[tokio::test]
    async fn test5_a_boolean_is_invalid() {}
    #[tokio::test]
    async fn test6_null_is_invalid() {}
}
mod type_as_array_with_one_item_8 {
    #[tokio::test]
    async fn test0_string_is_valid() {}
    #[tokio::test]
    async fn test1_number_is_invalid() {}
}
mod type_array_or_object_9 {
    #[tokio::test]
    async fn test0_array_is_valid() {}
    #[tokio::test]
    async fn test1_object_is_valid() {}
    #[tokio::test]
    async fn test2_number_is_invalid() {}
    #[tokio::test]
    async fn test3_string_is_invalid() {}
    #[tokio::test]
    async fn test4_null_is_invalid() {}
}
mod type_array_object_or_null_10 {
    #[tokio::test]
    async fn test0_array_is_valid() {}
    #[tokio::test]
    async fn test1_object_is_valid() {}
    #[tokio::test]
    async fn test2_null_is_valid() {}
    #[tokio::test]
    async fn test3_number_is_invalid() {}
    #[tokio::test]
    async fn test4_string_is_invalid() {}
}
