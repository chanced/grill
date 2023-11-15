use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod id_inside_an_unknown_keyword_is_not_a_real_identifier_0 {
    #[tokio::test]
    async fn test0_type_matches_second_any_of_which_has_a_real_schema_in_it() {}
    #[tokio::test]
    async fn test1_type_matches_non_schema_in_first_any_of() {}
    #[tokio::test]
    async fn test2_type_matches_non_schema_in_third_any_of() {}
}
