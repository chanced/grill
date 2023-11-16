mod any_of_0 {
    #[test]
    fn test0_first_any_of_valid() {}
    #[test]
    fn test1_second_any_of_valid() {}
    #[test]
    fn test2_both_any_of_valid() {}
    #[test]
    fn test3_neither_any_of_valid() {}
}
mod any_of_with_base_schema_1 {
    #[test]
    fn test0_mismatch_base_schema() {}
    #[test]
    fn test1_one_any_of_valid() {}
    #[test]
    fn test2_both_any_of_invalid() {}
}
mod any_of_with_boolean_schemas_all_true_2 {
    #[test]
    fn test0_any_value_is_valid() {}
}
mod any_of_with_boolean_schemas_some_true_3 {
    #[test]
    fn test0_any_value_is_valid() {}
}
mod any_of_with_boolean_schemas_all_false_4 {
    #[test]
    fn test0_any_value_is_invalid() {}
}
mod any_of_complex_types_5 {
    #[test]
    fn test0_first_any_of_valid_complex() {}
    #[test]
    fn test1_second_any_of_valid_complex() {}
    #[test]
    fn test2_both_any_of_valid_complex() {}
    #[test]
    fn test3_neither_any_of_valid_complex() {}
}
mod any_of_with_one_empty_schema_6 {
    #[test]
    fn test0_string_is_valid() {}
    #[test]
    fn test1_number_is_valid() {}
}
mod nested_any_of_to_check_validation_semantics_7 {
    #[test]
    fn test0_null_is_valid() {}
    #[test]
    fn test1_anything_non_null_is_invalid() {}
}
