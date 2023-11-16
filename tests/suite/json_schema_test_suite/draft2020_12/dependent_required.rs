mod single_dependency_0 {
    #[test]
    fn test0_neither() {}
    #[test]
    fn test1_nondependant() {}
    #[test]
    fn test2_with_dependency() {}
    #[test]
    fn test3_missing_dependency() {}
    #[test]
    fn test4_ignores_arrays() {}
    #[test]
    fn test5_ignores_strings() {}
    #[test]
    fn test6_ignores_other_non_objects() {}
}
mod empty_dependents_1 {
    #[test]
    fn test0_empty_object() {}
    #[test]
    fn test1_object_with_one_property() {}
    #[test]
    fn test2_non_object_is_valid() {}
}
mod multiple_dependents_required_2 {
    #[test]
    fn test0_neither() {}
    #[test]
    fn test1_nondependants() {}
    #[test]
    fn test2_with_dependencies() {}
    #[test]
    fn test3_missing_dependency() {}
    #[test]
    fn test4_missing_other_dependency() {}
    #[test]
    fn test5_missing_both_dependencies() {}
}
mod dependencies_with_escaped_characters_3 {
    #[test]
    fn test0_crlf() {}
    #[test]
    fn test1_quoted_quotes() {}
    #[test]
    fn test2_crlf_missing_dependent() {}
    #[test]
    fn test3_quoted_quotes_missing_dependent() {}
}
