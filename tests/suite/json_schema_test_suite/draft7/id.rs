use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod id_inside_an_enum_is_not_a_real_identifier_0 {
    #[tokio::test]
    async fn test0_exact_match_to_enum_and_type_matches() {}
    #[tokio::test]
    async fn test1_match_ref_to_id() {}
    #[tokio::test]
    async fn test2_no_match_on_enum_or_ref_to_id() {}
}
mod non_schema_object_containing_a_plain_name_id_property_1 {
    #[tokio::test]
    async fn test0_skip_traversing_definition_for_a_valid_result() {}
    #[tokio::test]
    async fn test1_const_at_const_not_anchor_does_not_match() {}
}
mod non_schema_object_containing_an_id_property_2 {
    #[tokio::test]
    async fn test0_skip_traversing_definition_for_a_valid_result() {}
    #[tokio::test]
    async fn test1_const_at_const_not_id_does_not_match() {}
}
