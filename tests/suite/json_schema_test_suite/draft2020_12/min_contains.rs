mod min_contains_without_contains_is_ignored_0 {
    #[test]
    fn test0_one_item_valid_against_lone_min_contains() {}
    #[test]
    fn test1_zero_items_still_valid_against_lone_min_contains() {}
}
mod min_contains_1_with_contains_1 {
    #[test]
    fn test0_empty_data() {}
    #[test]
    fn test1_no_elements_match() {}
    #[test]
    fn test2_single_element_matches_valid_min_contains() {}
    #[test]
    fn test3_some_elements_match_valid_min_contains() {}
    #[test]
    fn test4_all_elements_match_valid_min_contains() {}
}
mod min_contains_2_with_contains_2 {
    #[test]
    fn test0_empty_data() {}
    #[test]
    fn test1_all_elements_match_invalid_min_contains() {}
    #[test]
    fn test2_some_elements_match_invalid_min_contains() {}
    #[test]
    fn test3_all_elements_match_valid_min_contains_exactly_as_needed() {}
    #[test]
    fn test4_all_elements_match_valid_min_contains_more_than_needed() {}
    #[test]
    fn test5_some_elements_match_valid_min_contains() {}
}
mod min_contains_2_with_contains_with_a_decimal_value_3 {
    #[test]
    fn test0_one_element_matches_invalid_min_contains() {}
    #[test]
    fn test1_both_elements_match_valid_min_contains() {}
}
mod max_contains_min_contains_4 {
    #[test]
    fn test0_empty_data() {}
    #[test]
    fn test1_all_elements_match_invalid_min_contains() {}
    #[test]
    fn test2_all_elements_match_invalid_max_contains() {}
    #[test]
    fn test3_all_elements_match_valid_max_contains_and_min_contains() {}
}
mod max_contains_min_contains_5 {
    #[test]
    fn test0_empty_data() {}
    #[test]
    fn test1_invalid_min_contains() {}
    #[test]
    fn test2_invalid_max_contains() {}
    #[test]
    fn test3_invalid_max_contains_and_min_contains() {}
}
mod min_contains_0_6 {
    #[test]
    fn test0_empty_data() {}
    #[test]
    fn test1_min_contains_eq_0_makes_contains_always_pass() {}
}
mod min_contains_0_with_max_contains_7 {
    #[test]
    fn test0_empty_data() {}
    #[test]
    fn test1_not_more_than_max_contains() {}
    #[test]
    fn test2_too_many() {}
}
