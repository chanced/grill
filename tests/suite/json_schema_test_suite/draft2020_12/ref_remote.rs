mod remote_ref_0 {
    #[test]
    fn test0_remote_ref_valid() {}
    #[test]
    fn test1_remote_ref_invalid() {}
}
mod fragment_within_remote_ref_1 {
    #[test]
    fn test0_remote_fragment_valid() {}
    #[test]
    fn test1_remote_fragment_invalid() {}
}
mod anchor_within_remote_ref_2 {
    #[test]
    fn test0_remote_anchor_valid() {}
    #[test]
    fn test1_remote_anchor_invalid() {}
}
mod ref_within_remote_ref_3 {
    #[test]
    fn test0_ref_within_ref_valid() {}
    #[test]
    fn test1_ref_within_ref_invalid() {}
}
mod base_uri_change_4 {
    #[test]
    fn test0_base_uri_change_ref_valid() {}
    #[test]
    fn test1_base_uri_change_ref_invalid() {}
}
mod base_uri_change_change_folder_5 {
    #[test]
    fn test0_number_is_valid() {}
    #[test]
    fn test1_string_is_invalid() {}
}
mod base_uri_change_change_folder_in_subschema_6 {
    #[test]
    fn test0_number_is_valid() {}
    #[test]
    fn test1_string_is_invalid() {}
}
mod root_ref_in_remote_ref_7 {
    #[test]
    fn test0_string_is_valid() {}
    #[test]
    fn test1_null_is_valid() {}
    #[test]
    fn test2_object_is_invalid() {}
}
mod remote_ref_with_ref_to_defs_8 {
    #[test]
    fn test0_invalid() {}
    #[test]
    fn test1_valid() {}
}
mod location_independent_identifier_in_remote_ref_9 {
    #[test]
    fn test0_integer_is_valid() {}
    #[test]
    fn test1_string_is_invalid() {}
}
mod retrieved_nested_refs_resolve_relative_to_their_uri_not_id_10 {
    #[test]
    fn test0_number_is_invalid() {}
    #[test]
    fn test1_string_is_valid() {}
}
mod remote_http_ref_with_different_id_11 {
    #[test]
    fn test0_number_is_invalid() {}
    #[test]
    fn test1_string_is_valid() {}
}
mod remote_http_ref_with_different_urn_id_12 {
    #[test]
    fn test0_number_is_invalid() {}
    #[test]
    fn test1_string_is_valid() {}
}
mod remote_http_ref_with_nested_absolute_ref_13 {
    #[test]
    fn test0_number_is_invalid() {}
    #[test]
    fn test1_string_is_valid() {}
}
