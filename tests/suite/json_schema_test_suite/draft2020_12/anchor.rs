mod location_independent_identifier_0 {
    #[tokio::test]
    async fn test0_match_() {}
    #[tokio::test]
    async fn test1_mismatch() {}
}
mod location_independent_identifier_with_absolute_uri_1 {
    #[tokio::test]
    async fn test0_match_() {}
    #[tokio::test]
    async fn test1_mismatch() {}
}
mod location_independent_identifier_with_base_uri_change_in_subschema_2 {
    #[tokio::test]
    async fn test0_match_() {}
    #[tokio::test]
    async fn test1_mismatch() {}
}
mod anchor_inside_an_enum_is_not_a_real_identifier_3 {
    #[tokio::test]
    async fn test0_exact_match_to_enum_and_type_matches() {}
    #[tokio::test]
    async fn test1_in_implementations_that_strip_anchor_this_may_match_either_def() {}
    #[tokio::test]
    async fn test2_match_ref_to_anchor() {}
    #[tokio::test]
    async fn test3_no_match_on_enum_or_ref_to_anchor() {}
}
mod same_anchor_with_different_base_uri_4 {
    #[tokio::test]
    async fn test0_ref_resolves_to_defs_a_all_of_1() {}
    #[tokio::test]
    async fn test1_ref_does_not_resolve_to_defs_a_all_of_0() {}
}
mod non_schema_object_containing_an_anchor_property_5 {
    #[tokio::test]
    async fn test0_skip_traversing_definition_for_a_valid_result() {}
    #[tokio::test]
    async fn test1_const_at_const_not_anchor_does_not_match() {}
}
mod invalid_anchors_6 {
    #[tokio::test]
    async fn test0_must_start_with_a_letter_and_not() {}
    #[tokio::test]
    async fn test1_json_pointers_are_not_valid() {}
    #[tokio::test]
    async fn test2_invalid_with_valid_beginning() {}
}
