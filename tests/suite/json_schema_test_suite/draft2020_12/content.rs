mod validation_of_string_encoded_content_based_on_media_type_0 {
    #[tokio::test]
    async fn test0_a_valid_json_document() {}
    #[tokio::test]
    async fn test1_an_invalid_json_document_validates_true() {}
    #[tokio::test]
    async fn test2_ignores_non_strings() {}
}
mod validation_of_binary_string_encoding_1 {
    #[tokio::test]
    async fn test0_a_valid_base64_string() {}
    #[tokio::test]
    async fn test1_an_invalid_base64_string_is_not_a_valid_character_validates_true() {}
    #[tokio::test]
    async fn test2_ignores_non_strings() {}
}
mod validation_of_binary_encoded_media_type_documents_2 {
    #[tokio::test]
    async fn test0_a_valid_base64_encoded_json_document() {}
    #[tokio::test]
    async fn test1_a_validly_encoded_invalid_json_document_validates_true() {}
    #[tokio::test]
    async fn test2_an_invalid_base64_string_that_is_valid_json_validates_true() {}
    #[tokio::test]
    async fn test3_ignores_non_strings() {}
}
mod validation_of_binary_encoded_media_type_documents_with_schema_3 {
    #[tokio::test]
    async fn test0_a_valid_base64_encoded_json_document() {}
    #[tokio::test]
    async fn test1_another_valid_base64_encoded_json_document() {}
    #[tokio::test]
    async fn test2_an_invalid_base64_encoded_json_document_validates_true() {}
    #[tokio::test]
    async fn test3_an_empty_object_as_a_base64_encoded_json_document_validates_true() {}
    #[tokio::test]
    async fn test4_an_empty_array_as_a_base64_encoded_json_document() {}
    #[tokio::test]
    async fn test5_a_validly_encoded_invalid_json_document_validates_true() {}
    #[tokio::test]
    async fn test6_an_invalid_base64_string_that_is_valid_json_validates_true() {}
    #[tokio::test]
    async fn test7_ignores_non_strings() {}
}
