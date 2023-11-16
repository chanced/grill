mod not_0 {
    #[test]
    fn test0_allowed() {}
    #[test]
    fn test1_disallowed() {}
}
mod not_multiple_types_1 {
    #[test]
    fn test0_valid() {}
    #[test]
    fn test1_mismatch() {}
    #[test]
    fn test2_other_mismatch() {}
}
mod not_more_complex_schema_2 {
    #[test]
    fn test0_match_() {}
    #[test]
    fn test1_other_match() {}
    #[test]
    fn test2_mismatch() {}
}
mod forbidden_property_3 {
    #[test]
    fn test0_property_present() {}
    #[test]
    fn test1_property_absent() {}
}
mod not_with_boolean_schema_true_4 {
    #[test]
    fn test0_any_value_is_invalid() {}
}
mod not_with_boolean_schema_false_5 {
    #[test]
    fn test0_any_value_is_valid() {}
}
mod collect_annotations_inside_a_not_even_if_collection_is_disabled_6 {
    #[test]
    fn test0_unevaluated_property() {}
    #[test]
    fn test1_annotations_are_still_collected_inside_a_not() {}
}
