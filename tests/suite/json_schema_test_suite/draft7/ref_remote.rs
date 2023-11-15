use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
mod remote_ref_0 {
    #[tokio::test]
    async fn test0_remote_ref_valid() {}
    #[tokio::test]
    async fn test1_remote_ref_invalid() {}
}
mod fragment_within_remote_ref_1 {
    #[tokio::test]
    async fn test0_remote_fragment_valid() {}
    #[tokio::test]
    async fn test1_remote_fragment_invalid() {}
}
mod ref_within_remote_ref_2 {
    #[tokio::test]
    async fn test0_ref_within_ref_valid() {}
    #[tokio::test]
    async fn test1_ref_within_ref_invalid() {}
}
mod base_uri_change_3 {
    #[tokio::test]
    async fn test0_base_uri_change_ref_valid() {}
    #[tokio::test]
    async fn test1_base_uri_change_ref_invalid() {}
}
mod base_uri_change_change_folder_4 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_string_is_invalid() {}
}
mod base_uri_change_change_folder_in_subschema_5 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_string_is_invalid() {}
}
mod root_ref_in_remote_ref_6 {
    #[tokio::test]
    async fn test0_string_is_valid() {}
    #[tokio::test]
    async fn test1_null_is_valid() {}
    #[tokio::test]
    async fn test2_object_is_invalid() {}
}
mod remote_ref_with_ref_to_definitions_7 {
    #[tokio::test]
    async fn test0_invalid() {}
    #[tokio::test]
    async fn test1_valid() {}
}
mod location_independent_identifier_in_remote_ref_8 {
    #[tokio::test]
    async fn test0_integer_is_valid() {}
    #[tokio::test]
    async fn test1_string_is_invalid() {}
}
mod retrieved_nested_refs_resolve_relative_to_their_uri_not_id_9 {
    #[tokio::test]
    async fn test0_number_is_invalid() {}
    #[tokio::test]
    async fn test1_string_is_valid() {}
}
