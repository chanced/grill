mod a_dynamic_ref_to_a_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_an_anchor_0 {
    #[test]
    fn test0_an_array_of_strings_is_valid() {}
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {}
}
mod a_dynamic_ref_to_an_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_an_anchor_1 {
    #[test]
    fn test0_an_array_of_strings_is_valid() {}
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {}
}
mod a_ref_to_a_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_an_anchor_2 {
    #[test]
    fn test0_an_array_of_strings_is_valid() {}
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {}
}
mod a_dynamic_ref_resolves_to_the_first_dynamic_anchor_still_in_scope_that_is_encountered_when_the_schema_is_evaluated_3 {
    #[test]
    fn test0_an_array_of_strings_is_valid() {}
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {}
}
mod a_dynamic_ref_without_anchor_in_fragment_behaves_identical_to_ref_4 {
    #[test]
    fn test0_an_array_of_strings_is_invalid() {}
    #[test]
    fn test1_an_array_of_numbers_is_valid() {}
}
mod a_dynamic_ref_with_intermediate_scopes_that_don_t_include_a_matching_dynamic_anchor_does_not_affect_dynamic_scope_resolution_5 {
    #[test]
    fn test0_an_array_of_strings_is_valid() {}
    #[test]
    fn test1_an_array_containing_non_strings_is_invalid() {}
}
mod an_anchor_with_the_same_name_as_a_dynamic_anchor_is_not_used_for_dynamic_scope_resolution_6 {
    #[test]
    fn test0_any_array_is_valid() {}
}
mod a_dynamic_ref_without_a_matching_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_anchor_7 {
    #[test]
    fn test0_any_array_is_valid() {}
}
mod a_dynamic_ref_with_a_non_matching_dynamic_anchor_in_the_same_schema_resource_behaves_like_a_normal_ref_to_anchor_8 {
    #[test]
    fn test0_any_array_is_valid() {}
}
mod a_dynamic_ref_that_initially_resolves_to_a_schema_with_a_matching_dynamic_anchor_resolves_to_the_first_dynamic_anchor_in_the_dynamic_scope_9 {
    #[test]
    fn test0_the_recursive_part_is_valid_against_the_root() {}
    #[test]
    fn test1_the_recursive_part_is_not_valid_against_the_root() {}
}
mod a_dynamic_ref_that_initially_resolves_to_a_schema_without_a_matching_dynamic_anchor_behaves_like_a_normal_ref_to_anchor_10 {
    #[test]
    fn test0_the_recursive_part_doesn_t_need_to_validate_against_the_root() {}
}
mod multiple_dynamic_paths_to_the_dynamic_ref_keyword_11 {
    #[test]
    fn test0_recurse_to_any_leaf_node_floats_are_allowed() {}
    #[test]
    fn test1_recurse_to_integer_node_floats_are_not_allowed() {}
}
mod after_leaving_a_dynamic_scope_it_is_not_used_by_a_dynamic_ref_12 {
    #[test]
    fn test0_string_matches_defs_thingy_but_the_dynamic_ref_does_not_stop_here() {}
    #[test]
    fn test1_first_scope_is_not_in_dynamic_scope_for_the_dynamic_ref() {}
    #[test]
    fn test2_then_defs_thingy_is_the_final_stop_for_the_dynamic_ref() {}
}
mod strict_tree_schema_guards_against_misspelled_properties_13 {
    #[test]
    fn test0_instance_with_misspelled_field() {}
    #[test]
    fn test1_instance_with_correct_field() {}
}
mod tests_for_implementation_dynamic_anchor_and_reference_link_14 {
    #[test]
    fn test0_incorrect_parent_schema() {}
    #[test]
    fn test1_incorrect_extended_schema() {}
    #[test]
    fn test2_correct_extended_schema() {}
}
mod ref_and_dynamic_anchor_are_independent_of_order_defs_first_15 {
    #[test]
    fn test0_incorrect_parent_schema() {}
    #[test]
    fn test1_incorrect_extended_schema() {}
    #[test]
    fn test2_correct_extended_schema() {}
}
mod ref_and_dynamic_anchor_are_independent_of_order_ref_first_16 {
    #[test]
    fn test0_incorrect_parent_schema() {}
    #[test]
    fn test1_incorrect_extended_schema() {}
    #[test]
    fn test2_correct_extended_schema() {}
}
