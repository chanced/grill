mod schema_that_uses_custom_metaschema_with_with_no_validation_vocabulary_0 {
    #[tokio::test]
    async fn test0_applicator_vocabulary_still_works() {}
    #[tokio::test]
    async fn test1_no_validation_valid_number() {}
    #[tokio::test]
    async fn test2_no_validation_invalid_number_but_it_still_validates() {}
}
mod ignore_unrecognized_optional_vocabulary_1 {
    #[tokio::test]
    async fn test0_string_value() {}
    #[tokio::test]
    async fn test1_number_value() {}
}
