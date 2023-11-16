mod a_schema_given_for_items_0 {
    #[test]
    fn test0_valid_items() {}
    #[test]
    fn test1_wrong_type_of_items() {}
    #[test]
    fn test2_ignores_non_arrays() {}
    #[test]
    fn test3_java_script_pseudo_array_is_valid() {}
}
mod items_with_boolean_schema_true_1 {
    #[test]
    fn test0_any_array_is_valid() {}
    #[test]
    fn test1_empty_array_is_valid() {}
}
mod items_with_boolean_schema_false_2 {
    #[test]
    fn test0_any_non_empty_array_is_invalid() {}
    #[test]
    fn test1_empty_array_is_valid() {}
}
mod items_and_subitems_3 {
    #[test]
    fn test0_valid_items() {}
    #[test]
    fn test1_too_many_items() {}
    #[test]
    fn test2_too_many_sub_items() {}
    #[test]
    fn test3_wrong_item() {}
    #[test]
    fn test4_wrong_sub_item() {}
    #[test]
    fn test5_fewer_items_is_valid() {}
}
mod nested_items_4 {
    #[test]
    fn test0_valid_nested_array() {}
    #[test]
    fn test1_nested_array_with_invalid_type() {}
    #[test]
    fn test2_not_deep_enough() {}
}
mod prefix_items_with_no_additional_items_allowed_5 {
    #[test]
    fn test0_empty_array() {}
    #[test]
    fn test1_fewer_number_of_items_present_1() {}
    #[test]
    fn test2_fewer_number_of_items_present_2() {}
    #[test]
    fn test3_equal_number_of_items_present() {}
    #[test]
    fn test4_additional_items_are_not_permitted() {}
}
mod items_does_not_look_in_applicators_valid_case_6 {
    #[test]
    fn test0_prefix_items_in_all_of_does_not_constrain_items_invalid_case() {}
    #[test]
    fn test1_prefix_items_in_all_of_does_not_constrain_items_valid_case() {}
}
mod prefix_items_validation_adjusts_the_starting_index_for_items_7 {
    #[test]
    fn test0_valid_items() {}
    #[test]
    fn test1_wrong_type_of_second_item() {}
}
mod items_with_null_instance_elements_8 {
    #[test]
    fn test0_allows_null_elements() {}
}
