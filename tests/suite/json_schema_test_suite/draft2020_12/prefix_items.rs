mod a_schema_given_for_prefix_items_0 {
    #[test]
    fn test0_correct_types() {}
    #[test]
    fn test1_wrong_types() {}
    #[test]
    fn test2_incomplete_array_of_items() {}
    #[test]
    fn test3_array_with_additional_items() {}
    #[test]
    fn test4_empty_array() {}
    #[test]
    fn test5_java_script_pseudo_array_is_valid() {}
}
mod prefix_items_with_boolean_schemas_1 {
    #[test]
    fn test0_array_with_one_item_is_valid() {}
    #[test]
    fn test1_array_with_two_items_is_invalid() {}
    #[test]
    fn test2_empty_array_is_valid() {}
}
mod additional_items_are_allowed_by_default_2 {
    #[test]
    fn test0_only_the_first_item_is_validated() {}
}
mod prefix_items_with_null_instance_elements_3 {
    #[test]
    fn test0_allows_null_elements() {}
}
