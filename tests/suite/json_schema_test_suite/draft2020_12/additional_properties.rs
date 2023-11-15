mod additional_properties_being_false_does_not_allow_other_properties_0 {
    #[tokio::test]
    async fn test0_no_additional_properties_is_valid() {}
    #[tokio::test]
    async fn test1_an_additional_property_is_invalid() {}
    #[tokio::test]
    async fn test2_ignores_arrays() {}
    #[tokio::test]
    async fn test3_ignores_strings() {}
    #[tokio::test]
    async fn test4_ignores_other_non_objects() {}
    #[tokio::test]
    async fn test5_pattern_properties_are_not_additional_properties() {}
}
mod non_ascii_pattern_with_additional_properties_1 {
    #[tokio::test]
    async fn test0_matching_the_pattern_is_valid() {}
    #[tokio::test]
    async fn test1_not_matching_the_pattern_is_invalid() {}
}
mod additional_properties_with_schema_2 {
    #[tokio::test]
    async fn test0_no_additional_properties_is_valid() {}
    #[tokio::test]
    async fn test1_an_additional_valid_property_is_valid() {}
    #[tokio::test]
    async fn test2_an_additional_invalid_property_is_invalid() {}
}
mod additional_properties_can_exist_by_itself_3 {
    #[tokio::test]
    async fn test0_an_additional_valid_property_is_valid() {}
    #[tokio::test]
    async fn test1_an_additional_invalid_property_is_invalid() {}
}
mod additional_properties_are_allowed_by_default_4 {
    #[tokio::test]
    async fn test0_additional_properties_are_allowed() {}
}
mod additional_properties_does_not_look_in_applicators_5 {
    #[tokio::test]
    async fn test0_properties_defined_in_all_of_are_not_examined() {}
}
mod additional_properties_with_null_valued_instance_properties_6 {
    #[tokio::test]
    async fn test0_allows_null_values() {}
}
