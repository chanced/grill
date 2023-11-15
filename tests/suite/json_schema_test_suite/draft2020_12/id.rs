mod invalid_use_of_fragments_in_location_independent_id_0 {
    #[tokio::test]
    async fn test0_identifier_name() {}
    #[tokio::test]
    async fn test1_identifier_name_and_no_ref() {}
    #[tokio::test]
    async fn test2_identifier_path() {}
    #[tokio::test]
    async fn test3_identifier_name_with_absolute_uri() {}
    #[tokio::test]
    async fn test4_identifier_path_with_absolute_uri() {}
    #[tokio::test]
    async fn test5_identifier_name_with_base_uri_change_in_subschema() {}
    #[tokio::test]
    async fn test6_identifier_path_with_base_uri_change_in_subschema() {}
}
mod valid_use_of_empty_fragments_in_location_independent_id_1 {
    #[tokio::test]
    async fn test0_identifier_name_with_absolute_uri() {}
    #[tokio::test]
    async fn test1_identifier_name_with_base_uri_change_in_subschema() {}
}
mod unnormalized_ids_are_allowed_but_discouraged_2 {
    #[tokio::test]
    async fn test0_unnormalized_identifier() {}
    #[tokio::test]
    async fn test1_unnormalized_identifier_and_no_ref() {}
    #[tokio::test]
    async fn test2_unnormalized_identifier_with_empty_fragment() {}
    #[tokio::test]
    async fn test3_unnormalized_identifier_with_empty_fragment_and_no_ref() {}
}
mod id_inside_an_enum_is_not_a_real_identifier_3 {
    #[tokio::test]
    async fn test0_exact_match_to_enum_and_type_matches() {}
    #[tokio::test]
    async fn test1_match_ref_to_id() {}
    #[tokio::test]
    async fn test2_no_match_on_enum_or_ref_to_id() {}
}
mod non_schema_object_containing_an_id_property_4 {
    #[tokio::test]
    async fn test0_skip_traversing_definition_for_a_valid_result() {}
    #[tokio::test]
    async fn test1_const_at_const_not_id_does_not_match() {}
}
