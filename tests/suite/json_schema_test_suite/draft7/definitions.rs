use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod validate_definition_against_metaschema_0 {
    #[tokio::test]
    async fn test0_valid_definition_schema() {}
    #[tokio::test]
    async fn test1_invalid_definition_schema() {}
}
