mod unevaluated_items_true_0 {
    #[test]
    fn test0_with_no_unevaluated_items() {}
    #[test]
    fn test1_with_unevaluated_items() {}
}
mod unevaluated_items_false_1 {
    #[test]
    fn test0_with_no_unevaluated_items() {}
    #[test]
    fn test1_with_unevaluated_items() {}
}
mod unevaluated_items_as_schema_2 {
    #[test]
    fn test0_with_no_unevaluated_items() {}
    #[test]
    fn test1_with_valid_unevaluated_items() {}
    #[test]
    fn test2_with_invalid_unevaluated_items() {}
}
mod unevaluated_items_with_uniform_items_3 {
    #[test]
    fn test0_unevaluated_items_doesn_t_apply() {}
}
mod unevaluated_items_with_tuple_4 {
    #[test]
    fn test0_with_no_unevaluated_items() {}
    #[test]
    fn test1_with_unevaluated_items() {}
}
mod unevaluated_items_with_items_and_prefix_items_5 {
    #[test]
    fn test0_unevaluated_items_doesn_t_apply() {}
}
mod unevaluated_items_with_items_6 {
    #[test]
    fn test0_valid_under_items() {}
    #[test]
    fn test1_invalid_under_items() {}
}
mod unevaluated_items_with_nested_tuple_7 {
    #[test]
    fn test0_with_no_unevaluated_items() {}
    #[test]
    fn test1_with_unevaluated_items() {}
}
mod unevaluated_items_with_nested_items_8 {
    #[test]
    fn test0_with_only_valid_additional_items() {}
    #[test]
    fn test1_with_no_additional_items() {}
    #[test]
    fn test2_with_invalid_additional_item() {}
}
mod unevaluated_items_with_nested_prefix_items_and_items_9 {
    #[test]
    fn test0_with_no_additional_items() {}
    #[test]
    fn test1_with_additional_items() {}
}
mod unevaluated_items_with_nested_unevaluated_items_10 {
    #[test]
    fn test0_with_no_additional_items() {}
    #[test]
    fn test1_with_additional_items() {}
}
mod unevaluated_items_with_any_of_11 {
    #[test]
    fn test0_when_one_schema_matches_and_has_no_unevaluated_items() {}
    #[test]
    fn test1_when_one_schema_matches_and_has_unevaluated_items() {}
    #[test]
    fn test2_when_two_schemas_match_and_has_no_unevaluated_items() {}
    #[test]
    fn test3_when_two_schemas_match_and_has_unevaluated_items() {}
}
mod unevaluated_items_with_one_of_12 {
    #[test]
    fn test0_with_no_unevaluated_items() {}
    #[test]
    fn test1_with_unevaluated_items() {}
}
mod unevaluated_items_with_not_13 {
    #[test]
    fn test0_with_unevaluated_items() {}
}
mod unevaluated_items_with_if_then_else_14 {
    #[test]
    fn test0_when_if_matches_and_it_has_no_unevaluated_items() {}
    #[test]
    fn test1_when_if_matches_and_it_has_unevaluated_items() {}
    #[test]
    fn test2_when_if_doesn_t_match_and_it_has_no_unevaluated_items() {}
    #[test]
    fn test3_when_if_doesn_t_match_and_it_has_unevaluated_items() {}
}
mod unevaluated_items_with_boolean_schemas_15 {
    #[test]
    fn test0_with_no_unevaluated_items() {}
    #[test]
    fn test1_with_unevaluated_items() {}
}
mod unevaluated_items_with_ref_16 {
    #[test]
    fn test0_with_no_unevaluated_items() {}
    #[test]
    fn test1_with_unevaluated_items() {}
}
mod unevaluated_items_can_t_see_inside_cousins_17 {
    #[test]
    fn test0_always_fails() {}
}
mod item_is_evaluated_in_an_uncle_schema_to_unevaluated_items_18 {
    #[test]
    fn test0_no_extra_items() {}
    #[test]
    fn test1_uncle_keyword_evaluation_is_not_significant() {}
}
mod unevaluated_items_depends_on_adjacent_contains_19 {
    #[test]
    fn test0_second_item_is_evaluated_by_contains() {}
    #[test]
    fn test1_contains_fails_second_item_is_not_evaluated() {}
    #[test]
    fn test2_contains_passes_second_item_is_not_evaluated() {}
}
mod unevaluated_items_depends_on_multiple_nested_contains_20 {
    #[test]
    fn test0_5_not_evaluated_passes_unevaluated_items() {}
    #[test]
    fn test1_7_not_evaluated_fails_unevaluated_items() {}
}
mod unevaluated_items_and_contains_interact_to_control_item_dependency_relationship_21 {
    #[test]
    fn test0_empty_array_is_valid() {}
    #[test]
    fn test1_only_a_s_are_valid() {}
    #[test]
    fn test2_a_s_and_b_s_are_valid() {}
    #[test]
    fn test3_a_s_b_s_and_c_s_are_valid() {}
    #[test]
    fn test4_only_b_s_are_invalid() {}
    #[test]
    fn test5_only_c_s_are_invalid() {}
    #[test]
    fn test6_only_b_s_and_c_s_are_invalid() {}
    #[test]
    fn test7_only_a_s_and_c_s_are_invalid() {}
}
mod non_array_instances_are_valid_22 {
    #[test]
    fn test0_ignores_booleans() {}
    #[test]
    fn test1_ignores_integers() {}
    #[test]
    fn test2_ignores_floats() {}
    #[test]
    fn test3_ignores_objects() {}
    #[test]
    fn test4_ignores_strings() {}
    #[test]
    fn test5_ignores_null() {}
}
mod unevaluated_items_with_null_instance_elements_23 {
    #[test]
    fn test0_allows_null_elements() {}
}
mod unevaluated_items_can_see_annotations_from_if_without_then_and_else_24 {
    #[test]
    fn test0_valid_in_case_if_is_evaluated() {}
    #[test]
    fn test1_invalid_in_case_if_is_evaluated() {}
}
