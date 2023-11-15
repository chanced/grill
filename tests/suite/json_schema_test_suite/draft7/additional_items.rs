use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod additional_items_as_schema_0 {
    #[tokio::test]
    async fn test0_additional_items_match_schema() {}
    #[tokio::test]
    async fn test1_additional_items_do_not_match_schema() {}
}
mod when_items_is_schema_additional_items_does_nothing_1 {
    #[tokio::test]
    async fn test0_valid_with_a_array_of_type_integers() {}
    #[tokio::test]
    async fn test1_invalid_with_a_array_of_mixed_types() {}
}
mod when_items_is_schema_boolean_additional_items_does_nothing_2 {
    #[tokio::test]
    async fn test0_all_items_match_schema() {}
}
mod array_of_items_with_no_additional_items_permitted_3 {
    #[tokio::test]
    async fn test0_empty_array() {}
    #[tokio::test]
    async fn test1_fewer_number_of_items_present_1() {}
    #[tokio::test]
    async fn test2_fewer_number_of_items_present_2() {}
    #[tokio::test]
    async fn test3_equal_number_of_items_present() {}
    #[tokio::test]
    async fn test4_additional_items_are_not_permitted() {}
}
mod additional_items_as_false_without_items_4 {
    #[tokio::test]
    async fn test0_items_defaults_to_empty_schema_so_everything_is_valid() {}
    #[tokio::test]
    async fn test1_ignores_non_arrays() {}
}
mod additional_items_are_allowed_by_default_5 {
    #[tokio::test]
    async fn test0_only_the_first_item_is_validated() {}
}
mod additional_items_does_not_look_in_applicators_valid_case_6 {
    #[tokio::test]
    async fn test0_items_defined_in_all_of_are_not_examined() {}
}
mod additional_items_does_not_look_in_applicators_invalid_case_7 {
    #[tokio::test]
    async fn test0_items_defined_in_all_of_are_not_examined() {}
}
mod items_validation_adjusts_the_starting_index_for_additional_items_8 {
    #[tokio::test]
    async fn test0_valid_items() {}
    #[tokio::test]
    async fn test1_wrong_type_of_second_item() {}
}
mod additional_items_with_null_instance_elements_9 {
    #[tokio::test]
    async fn test0_allows_null_elements() {}
}
