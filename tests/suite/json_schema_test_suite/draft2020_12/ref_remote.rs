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
mod anchor_within_remote_ref_2 {
    #[tokio::test]
    async fn test0_remote_anchor_valid() {}
    #[tokio::test]
    async fn test1_remote_anchor_invalid() {}
}
mod ref_within_remote_ref_3 {
    #[tokio::test]
    async fn test0_ref_within_ref_valid() {}
    #[tokio::test]
    async fn test1_ref_within_ref_invalid() {}
}
mod base_uri_change_4 {
    #[tokio::test]
    async fn test0_base_uri_change_ref_valid() {}
    #[tokio::test]
    async fn test1_base_uri_change_ref_invalid() {}
}
mod base_uri_change_change_folder_5 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_string_is_invalid() {}
}
mod base_uri_change_change_folder_in_subschema_6 {
    #[tokio::test]
    async fn test0_number_is_valid() {}
    #[tokio::test]
    async fn test1_string_is_invalid() {}
}
mod root_ref_in_remote_ref_7 {
    #[tokio::test]
    async fn test0_string_is_valid() {}
    #[tokio::test]
    async fn test1_null_is_valid() {}
    #[tokio::test]
    async fn test2_object_is_invalid() {}
}
mod remote_ref_with_ref_to_defs_8 {
    #[tokio::test]
    async fn test0_invalid() {}
    #[tokio::test]
    async fn test1_valid() {}
}
mod location_independent_identifier_in_remote_ref_9 {
    #[tokio::test]
    async fn test0_integer_is_valid() {}
    #[tokio::test]
    async fn test1_string_is_invalid() {}
}
mod retrieved_nested_refs_resolve_relative_to_their_uri_not_id_10 {
    #[tokio::test]
    async fn test0_number_is_invalid() {}
    #[tokio::test]
    async fn test1_string_is_valid() {}
}
mod remote_http_ref_with_different_id_11 {
    #[tokio::test]
    async fn test0_number_is_invalid() {}
    #[tokio::test]
    async fn test1_string_is_valid() {}
}
mod remote_http_ref_with_different_urn_id_12 {
    #[tokio::test]
    async fn test0_number_is_invalid() {}
    #[tokio::test]
    async fn test1_string_is_valid() {}
}
mod remote_http_ref_with_nested_absolute_ref_13 {
    #[tokio::test]
    async fn test0_number_is_invalid() {}
    #[tokio::test]
    async fn test1_string_is_valid() {}
}
