mod single_dependency_0 {
    #[test]
    fn test0_valid() {}
    #[test]
    fn test1_no_dependency() {}
    #[test]
    fn test2_wrong_type() {}
    #[test]
    fn test3_wrong_type_other() {}
    #[test]
    fn test4_wrong_type_both() {}
    #[test]
    fn test5_ignores_arrays() {}
    #[test]
    fn test6_ignores_strings() {}
    #[test]
    fn test7_ignores_other_non_objects() {}
}
mod boolean_subschemas_1 {
    #[test]
    fn test0_object_with_property_having_schema_true_is_valid() {}
    #[test]
    fn test1_object_with_property_having_schema_false_is_invalid() {}
    #[test]
    fn test2_object_with_both_properties_is_invalid() {}
    #[test]
    fn test3_empty_object_is_valid() {}
}
mod dependencies_with_escaped_characters_2 {
    #[test]
    fn test0_quoted_tab() {}
    #[test]
    fn test1_quoted_quote() {}
    #[test]
    fn test2_quoted_tab_invalid_under_dependent_schema() {}
    #[test]
    fn test3_quoted_quote_invalid_under_dependent_schema() {}
}
mod dependent_subschema_incompatible_with_root_3 {
    #[test]
    fn test0_matches_root() {}
    #[test]
    fn test1_matches_dependency() {}
    #[test]
    fn test2_matches_both() {}
    #[test]
    fn test3_no_dependency() {}
}
