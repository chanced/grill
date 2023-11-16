mod one_of_0 {
    #[test]
    fn test0_first_one_of_valid() {}
    #[test]
    fn test1_second_one_of_valid() {}
    #[test]
    fn test2_both_one_of_valid() {}
    #[test]
    fn test3_neither_one_of_valid() {}
}
mod one_of_with_base_schema_1 {
    #[test]
    fn test0_mismatch_base_schema() {}
    #[test]
    fn test1_one_one_of_valid() {}
    #[test]
    fn test2_both_one_of_valid() {}
}
mod one_of_with_boolean_schemas_all_true_2 {
    #[test]
    fn test0_any_value_is_invalid() {}
}
mod one_of_with_boolean_schemas_one_true_3 {
    #[test]
    fn test0_any_value_is_valid() {}
}
mod one_of_with_boolean_schemas_more_than_one_true_4 {
    #[test]
    fn test0_any_value_is_invalid() {}
}
mod one_of_with_boolean_schemas_all_false_5 {
    #[test]
    fn test0_any_value_is_invalid() {}
}
mod one_of_complex_types_6 {
    #[test]
    fn test0_first_one_of_valid_complex() {}
    #[test]
    fn test1_second_one_of_valid_complex() {}
    #[test]
    fn test2_both_one_of_valid_complex() {}
    #[test]
    fn test3_neither_one_of_valid_complex() {}
}
mod one_of_with_empty_schema_7 {
    #[test]
    fn test0_one_valid_valid() {}
    #[test]
    fn test1_both_valid_invalid() {}
}
mod one_of_with_required_8 {
    #[test]
    fn test0_both_invalid_invalid() {}
    #[test]
    fn test1_first_valid_valid() {}
    #[test]
    fn test2_second_valid_valid() {}
    #[test]
    fn test3_both_valid_invalid() {}
}
mod one_of_with_missing_optional_property_9 {
    #[test]
    fn test0_first_one_of_valid() {}
    #[test]
    fn test1_second_one_of_valid() {}
    #[test]
    fn test2_both_one_of_valid() {}
    #[test]
    fn test3_neither_one_of_valid() {}
}
mod nested_one_of_to_check_validation_semantics_10 {
    #[test]
    fn test0_null_is_valid() {}
    #[test]
    fn test1_anything_non_null_is_invalid() {}
}
