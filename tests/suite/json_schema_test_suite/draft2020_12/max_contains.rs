mod max_contains_without_contains_is_ignored_0 {
    #[tokio::test]
    async fn test0_one_item_valid_against_lone_max_contains() {}
    #[tokio::test]
    async fn test1_two_items_still_valid_against_lone_max_contains() {}
}
mod max_contains_with_contains_1 {
    #[tokio::test]
    async fn test0_empty_data() {}
    #[tokio::test]
    async fn test1_all_elements_match_valid_max_contains() {}
    #[tokio::test]
    async fn test2_all_elements_match_invalid_max_contains() {}
    #[tokio::test]
    async fn test3_some_elements_match_valid_max_contains() {}
    #[tokio::test]
    async fn test4_some_elements_match_invalid_max_contains() {}
}
mod max_contains_with_contains_value_with_a_decimal_2 {
    #[tokio::test]
    async fn test0_one_element_matches_valid_max_contains() {}
    #[tokio::test]
    async fn test1_too_many_elements_match_invalid_max_contains() {}
}
mod min_contains_max_contains_3 {
    #[tokio::test]
    async fn test0_actual_lt_min_contains_lt_max_contains() {}
    #[tokio::test]
    async fn test1_min_contains_lt_actual_lt_max_contains() {}
    #[tokio::test]
    async fn test2_min_contains_lt_max_contains_lt_actual() {}
}
