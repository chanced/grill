use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod invalid_type_for_default_0 {
    #[tokio::test]
    async fn test0_valid_when_property_is_specified() {}
    #[tokio::test]
    async fn test1_still_valid_when_the_invalid_default_is_used() {}
}
mod invalid_string_value_for_default_1 {
    #[tokio::test]
    async fn test0_valid_when_property_is_specified() {}
    #[tokio::test]
    async fn test1_still_valid_when_the_invalid_default_is_used() {}
}
mod the_default_keyword_does_not_do_anything_if_the_property_is_missing_2 {
    #[tokio::test]
    async fn test0_an_explicit_property_value_is_checked_against_maximum_passing() {}
    #[tokio::test]
    async fn test1_an_explicit_property_value_is_checked_against_maximum_failing() {}
    #[tokio::test]
    async fn test2_missing_properties_are_not_filled_in_with_the_default() {}
}
