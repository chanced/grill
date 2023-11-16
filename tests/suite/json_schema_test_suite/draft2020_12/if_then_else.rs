mod ignore_if_without_then_or_else_0 {
    #[test]
    fn test0_valid_when_valid_against_lone_if() {}
    #[test]
    fn test1_valid_when_invalid_against_lone_if() {}
}
mod ignore_then_without_if_1 {
    #[test]
    fn test0_valid_when_valid_against_lone_then() {}
    #[test]
    fn test1_valid_when_invalid_against_lone_then() {}
}
mod ignore_else_without_if_2 {
    #[test]
    fn test0_valid_when_valid_against_lone_else() {}
    #[test]
    fn test1_valid_when_invalid_against_lone_else() {}
}
mod if_and_then_without_else_3 {
    #[test]
    fn test0_valid_through_then() {}
    #[test]
    fn test1_invalid_through_then() {}
    #[test]
    fn test2_valid_when_if_test_fails() {}
}
mod if_and_else_without_then_4 {
    #[test]
    fn test0_valid_when_if_test_passes() {}
    #[test]
    fn test1_valid_through_else() {}
    #[test]
    fn test2_invalid_through_else() {}
}
mod validate_against_correct_branch_then_vs_else_5 {
    #[test]
    fn test0_valid_through_then() {}
    #[test]
    fn test1_invalid_through_then() {}
    #[test]
    fn test2_valid_through_else() {}
    #[test]
    fn test3_invalid_through_else() {}
}
mod non_interference_across_combined_schemas_6 {
    #[test]
    fn test0_valid_but_would_have_been_invalid_through_then() {}
    #[test]
    fn test1_valid_but_would_have_been_invalid_through_else() {}
}
mod if_with_boolean_schema_true_7 {
    #[test]
    fn test0_boolean_schema_true_in_if_always_chooses_the_then_path_valid() {}
    #[test]
    fn test1_boolean_schema_true_in_if_always_chooses_the_then_path_invalid() {}
}
mod if_with_boolean_schema_false_8 {
    #[test]
    fn test0_boolean_schema_false_in_if_always_chooses_the_else_path_invalid() {}
    #[test]
    fn test1_boolean_schema_false_in_if_always_chooses_the_else_path_valid() {}
}
mod if_appears_at_the_end_when_serialized_keyword_processing_sequence_9 {
    #[test]
    fn test0_yes_redirects_to_then_and_passes() {}
    #[test]
    fn test1_other_redirects_to_else_and_passes() {}
    #[test]
    fn test2_no_redirects_to_then_and_fails() {}
    #[test]
    fn test3_invalid_redirects_to_else_and_fails() {}
}
