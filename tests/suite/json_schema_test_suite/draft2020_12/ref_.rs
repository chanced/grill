mod root_pointer_ref_0 {
    #[tokio::test]
    async fn test0_match_() {}
    #[tokio::test]
    async fn test1_recursive_match() {}
    #[tokio::test]
    async fn test2_mismatch() {}
    #[tokio::test]
    async fn test3_recursive_mismatch() {}
}
mod relative_pointer_ref_to_object_1 {
    #[tokio::test]
    async fn test0_match_() {}
    #[tokio::test]
    async fn test1_mismatch() {}
}
mod relative_pointer_ref_to_array_2 {
    #[tokio::test]
    async fn test0_match_array() {}
    #[tokio::test]
    async fn test1_mismatch_array() {}
}
mod escaped_pointer_ref_3 {
    #[tokio::test]
    async fn test0_slash_invalid() {}
    #[tokio::test]
    async fn test1_tilde_invalid() {}
    #[tokio::test]
    async fn test2_percent_invalid() {}
    #[tokio::test]
    async fn test3_slash_valid() {}
    #[tokio::test]
    async fn test4_tilde_valid() {}
    #[tokio::test]
    async fn test5_percent_valid() {}
}
mod nested_refs_4 {
    #[tokio::test]
    async fn test0_nested_ref_valid() {}
    #[tokio::test]
    async fn test1_nested_ref_invalid() {}
}
mod ref_applies_alongside_sibling_keywords_5 {
    #[tokio::test]
    async fn test0_ref_valid_max_items_valid() {}
    #[tokio::test]
    async fn test1_ref_valid_max_items_invalid() {}
    #[tokio::test]
    async fn test2_ref_invalid() {}
}
mod remote_ref_containing_refs_itself_6 {
    #[tokio::test]
    async fn test0_remote_ref_valid() {}
    #[tokio::test]
    async fn test1_remote_ref_invalid() {}
}
mod property_named_ref_that_is_not_a_reference_7 {
    #[tokio::test]
    async fn test0_property_named_ref_valid() {}
    #[tokio::test]
    async fn test1_property_named_ref_invalid() {}
}
mod property_named_ref_containing_an_actual_ref_8 {
    #[tokio::test]
    async fn test0_property_named_ref_valid() {}
    #[tokio::test]
    async fn test1_property_named_ref_invalid() {}
}
mod ref_to_boolean_schema_true_9 {
    #[tokio::test]
    async fn test0_any_value_is_valid() {}
}
mod ref_to_boolean_schema_false_10 {
    #[tokio::test]
    async fn test0_any_value_is_invalid() {}
}
mod recursive_references_between_schemas_11 {
    #[tokio::test]
    async fn test0_valid_tree() {}
    #[tokio::test]
    async fn test1_invalid_tree() {}
}
mod refs_with_quote_12 {
    #[tokio::test]
    async fn test0_object_with_numbers_is_valid() {}
    #[tokio::test]
    async fn test1_object_with_strings_is_invalid() {}
}
mod ref_creates_new_scope_when_adjacent_to_keywords_13 {
    #[tokio::test]
    async fn test0_referenced_subschema_doesn_t_see_annotations_from_properties() {}
}
mod naive_replacement_of_ref_with_its_destination_is_not_correct_14 {
    #[tokio::test]
    async fn test0_do_not_evaluate_the_ref_inside_the_enum_matching_any_string() {}
    #[tokio::test]
    async fn test1_do_not_evaluate_the_ref_inside_the_enum_definition_exact_match() {}
    #[tokio::test]
    async fn test2_match_the_enum_exactly() {}
}
mod refs_with_relative_uris_and_defs_15 {
    #[tokio::test]
    async fn test0_invalid_on_inner_field() {}
    #[tokio::test]
    async fn test1_invalid_on_outer_field() {}
    #[tokio::test]
    async fn test2_valid_on_both_fields() {}
}
mod relative_refs_with_absolute_uris_and_defs_16 {
    #[tokio::test]
    async fn test0_invalid_on_inner_field() {}
    #[tokio::test]
    async fn test1_invalid_on_outer_field() {}
    #[tokio::test]
    async fn test2_valid_on_both_fields() {}
}
mod id_must_be_resolved_against_nearest_parent_not_just_immediate_parent_17 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_non_number_is_invalid() {}
}
mod order_of_evaluation_id_and_ref_18 {
    #[tokio::test]
    async fn test0_data_is_valid_against_first_definition() {}
    #[tokio::test]
    async fn test1_data_is_invalid_against_first_definition() {}
}
mod order_of_evaluation_id_and_anchor_and_ref_19 {
    #[tokio::test]
    async fn test0_data_is_valid_against_first_definition() {}
    #[tokio::test]
    async fn test1_data_is_invalid_against_first_definition() {}
}
mod simple_urn_base_uri_with_ref_via_the_urn_20 {
    #[tokio::test]
    async fn test0_valid_under_the_urn_i_ded_schema() {}
    #[tokio::test]
    async fn test1_invalid_under_the_urn_i_ded_schema() {}
}
mod simple_urn_base_uri_with_json_pointer_21 {
    #[tokio::test]
    async fn test0_a_string_is_valid() {}
    #[tokio::test]
    async fn test1_a_non_string_is_invalid() {}
}
mod urn_base_uri_with_nss_22 {
    #[tokio::test]
    async fn test0_a_string_is_valid() {}
    #[tokio::test]
    async fn test1_a_non_string_is_invalid() {}
}
mod urn_base_uri_with_r_component_23 {
    #[tokio::test]
    async fn test0_a_string_is_valid() {}
    #[tokio::test]
    async fn test1_a_non_string_is_invalid() {}
}
mod urn_base_uri_with_q_component_24 {
    #[tokio::test]
    async fn test0_a_string_is_valid() {}
    #[tokio::test]
    async fn test1_a_non_string_is_invalid() {}
}
mod urn_base_uri_with_f_component_25 {
    #[tokio::test]
    async fn test0_is_invalid() {}
}
mod urn_base_uri_with_urn_and_json_pointer_ref_26 {
    #[tokio::test]
    async fn test0_a_string_is_valid() {}
    #[tokio::test]
    async fn test1_a_non_string_is_invalid() {}
}
mod urn_base_uri_with_urn_and_anchor_ref_27 {
    #[tokio::test]
    async fn test0_a_string_is_valid() {}
    #[tokio::test]
    async fn test1_a_non_string_is_invalid() {}
}
mod urn_ref_with_nested_pointer_ref_28 {
    #[tokio::test]
    async fn test0_a_string_is_valid() {}
    #[tokio::test]
    async fn test1_a_non_string_is_invalid() {}
}
mod ref_to_if_29 {
    #[tokio::test]
    async fn test0_a_non_integer_is_invalid_due_to_the_ref() {}
    #[tokio::test]
    async fn test1_an_integer_is_valid() {}
}
mod ref_to_then_30 {
    #[tokio::test]
    async fn test0_a_non_integer_is_invalid_due_to_the_ref() {}
    #[tokio::test]
    async fn test1_an_integer_is_valid() {}
}
mod ref_to_else_31 {
    #[tokio::test]
    async fn test0_a_non_integer_is_invalid_due_to_the_ref() {}
    #[tokio::test]
    async fn test1_an_integer_is_valid() {}
}
mod ref_with_absolute_path_reference_32 {
    #[tokio::test]
    async fn test0_a_string_is_valid() {}
    #[tokio::test]
    async fn test1_an_integer_is_invalid() {}
}
mod id_with_file_uri_still_resolves_pointers_nix_33 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_non_number_is_invalid() {}
}
mod id_with_file_uri_still_resolves_pointers_windows_34 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_non_number_is_invalid() {}
}
mod empty_tokens_in_ref_json_pointer_35 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_non_number_is_invalid() {}
}
