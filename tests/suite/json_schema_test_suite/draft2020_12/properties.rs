mod object_properties_validation_0 {
    #[tokio::test]
    async fn test0_both_properties_present_and_valid_is_valid() {}
    #[tokio::test]
    async fn test1_one_property_invalid_is_invalid() {}
    #[tokio::test]
    async fn test2_both_properties_invalid_is_invalid() {}
    #[tokio::test]
    async fn test3_doesn_t_invalidate_other_properties() {}
    #[tokio::test]
    async fn test4_ignores_arrays() {}
    #[tokio::test]
    async fn test5_ignores_other_non_objects() {}
}
mod properties_pattern_properties_additional_properties_interaction_1 {
    #[tokio::test]
    async fn test0_property_validates_property() {}
    #[tokio::test]
    async fn test1_property_invalidates_property() {}
    #[tokio::test]
    async fn test2_pattern_property_invalidates_property() {}
    #[tokio::test]
    async fn test3_pattern_property_validates_nonproperty() {}
    #[tokio::test]
    async fn test4_pattern_property_invalidates_nonproperty() {}
    #[tokio::test]
    async fn test5_additional_property_ignores_property() {}
    #[tokio::test]
    async fn test6_additional_property_validates_others() {}
    #[tokio::test]
    async fn test7_additional_property_invalidates_others() {}
}
mod properties_with_boolean_schema_2 {
    #[tokio::test]
    async fn test0_no_property_present_is_valid() {}
    #[tokio::test]
    async fn test1_only_true_property_present_is_valid() {}
    #[tokio::test]
    async fn test2_only_false_property_present_is_invalid() {}
    #[tokio::test]
    async fn test3_both_properties_present_is_invalid() {}
}
mod properties_with_escaped_characters_3 {
    #[tokio::test]
    async fn test0_object_with_all_numbers_is_valid() {}
    #[tokio::test]
    async fn test1_object_with_strings_is_invalid() {}
}
mod properties_with_null_valued_instance_properties_4 {
    #[tokio::test]
    async fn test0_allows_null_values() {}
}
mod properties_whose_names_are_javascript_object_property_names_5 {
    #[tokio::test]
    async fn test0_ignores_arrays() {}
    #[tokio::test]
    async fn test1_ignores_other_non_objects() {}
    #[tokio::test]
    async fn test2_none_of_the_properties_mentioned() {}
    #[tokio::test]
    async fn test3_proto_not_valid() {}
    #[tokio::test]
    async fn test4_to_string_not_valid() {}
    #[tokio::test]
    async fn test5_constructor_not_valid() {}
    #[tokio::test]
    async fn test6_all_present_and_valid() {}
}
