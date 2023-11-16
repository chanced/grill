mod pattern_properties_validates_properties_matching_a_regex_0 {
    #[test]
    fn test0_a_single_valid_match_is_valid() {}
    #[test]
    fn test1_multiple_valid_matches_is_valid() {}
    #[test]
    fn test2_a_single_invalid_match_is_invalid() {}
    #[test]
    fn test3_multiple_invalid_matches_is_invalid() {}
    #[test]
    fn test4_ignores_arrays() {}
    #[test]
    fn test5_ignores_strings() {}
    #[test]
    fn test6_ignores_other_non_objects() {}
}
mod multiple_simultaneous_pattern_properties_are_validated_1 {
    #[test]
    fn test0_a_single_valid_match_is_valid() {}
    #[test]
    fn test1_a_simultaneous_match_is_valid() {}
    #[test]
    fn test2_multiple_matches_is_valid() {}
    #[test]
    fn test3_an_invalid_due_to_one_is_invalid() {}
    #[test]
    fn test4_an_invalid_due_to_the_other_is_invalid() {}
    #[test]
    fn test5_an_invalid_due_to_both_is_invalid() {}
}
mod regexes_are_not_anchored_by_default_and_are_case_sensitive_2 {
    #[test]
    fn test0_non_recognized_members_are_ignored() {}
    #[test]
    fn test1_recognized_members_are_accounted_for() {}
    #[test]
    fn test2_regexes_are_case_sensitive() {}
    #[test]
    fn test3_regexes_are_case_sensitive_2() {}
}
mod pattern_properties_with_boolean_schemas_3 {
    #[test]
    fn test0_object_with_property_matching_schema_true_is_valid() {}
    #[test]
    fn test1_object_with_property_matching_schema_false_is_invalid() {}
    #[test]
    fn test2_object_with_both_properties_is_invalid() {}
    #[test]
    fn test3_object_with_a_property_matching_both_true_and_false_is_invalid() {}
    #[test]
    fn test4_empty_object_is_valid() {}
}
mod pattern_properties_with_null_valued_instance_properties_4 {
    #[test]
    fn test0_allows_null_values() {}
}
