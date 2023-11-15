use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod evaluating_the_same_schema_location_against_the_same_data_location_twice_is_not_a_sign_of_an_infinite_loop_0 {
    #[tokio::test]
    async fn test0_passing_case() {}
    #[tokio::test]
    async fn test1_failing_case() {}
}
