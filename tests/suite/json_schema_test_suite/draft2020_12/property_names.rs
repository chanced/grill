mod property_names_validation_0 {
    #[test]
    fn test0_all_property_names_valid() {}
    #[test]
    fn test1_some_property_names_invalid() {}
    #[test]
    fn test2_object_without_properties_is_valid() {}
    #[test]
    fn test3_ignores_arrays() {}
    #[test]
    fn test4_ignores_strings() {}
    #[test]
    fn test5_ignores_other_non_objects() {}
}
mod property_names_with_boolean_schema_true_1 {
    #[test]
    fn test0_object_with_any_properties_is_valid() {}
    #[test]
    fn test1_empty_object_is_valid() {}
}
mod property_names_with_boolean_schema_false_2 {
    #[test]
    fn test0_object_with_any_properties_is_invalid() {}
    #[test]
    fn test1_empty_object_is_valid() {}
}
