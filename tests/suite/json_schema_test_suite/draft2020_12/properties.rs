mod object_properties_validation_0 {
    #[test]
    fn test0_both_properties_present_and_valid_is_valid() {}
    #[test]
    fn test1_one_property_invalid_is_invalid() {}
    #[test]
    fn test2_both_properties_invalid_is_invalid() {}
    #[test]
    fn test3_doesn_t_invalidate_other_properties() {}
    #[test]
    fn test4_ignores_arrays() {}
    #[test]
    fn test5_ignores_other_non_objects() {}
}
mod properties_pattern_properties_additional_properties_interaction_1 {
    #[test]
    fn test0_property_validates_property() {}
    #[test]
    fn test1_property_invalidates_property() {}
    #[test]
    fn test2_pattern_property_invalidates_property() {}
    #[test]
    fn test3_pattern_property_validates_nonproperty() {}
    #[test]
    fn test4_pattern_property_invalidates_nonproperty() {}
    #[test]
    fn test5_additional_property_ignores_property() {}
    #[test]
    fn test6_additional_property_validates_others() {}
    #[test]
    fn test7_additional_property_invalidates_others() {}
}
mod properties_with_boolean_schema_2 {
    #[test]
    fn test0_no_property_present_is_valid() {}
    #[test]
    fn test1_only_true_property_present_is_valid() {}
    #[test]
    fn test2_only_false_property_present_is_invalid() {}
    #[test]
    fn test3_both_properties_present_is_invalid() {}
}
mod properties_with_escaped_characters_3 {
    #[test]
    fn test0_object_with_all_numbers_is_valid() {}
    #[test]
    fn test1_object_with_strings_is_invalid() {}
}
mod properties_with_null_valued_instance_properties_4 {
    #[test]
    fn test0_allows_null_values() {}
}
mod properties_whose_names_are_javascript_object_property_names_5 {
    #[test]
    fn test0_ignores_arrays() {}
    #[test]
    fn test1_ignores_other_non_objects() {}
    #[test]
    fn test2_none_of_the_properties_mentioned() {}
    #[test]
    fn test3_proto_not_valid() {}
    #[test]
    fn test4_to_string_not_valid() {}
    #[test]
    fn test5_constructor_not_valid() {}
    #[test]
    fn test6_all_present_and_valid() {}
}
