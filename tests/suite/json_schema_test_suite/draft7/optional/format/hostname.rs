use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
async fn interrogator() {
    todo!()
}
async fn interrogator() {
    todo!()
}
mod validation_of_host_names_0 {
    #[tokio::test]
    async fn test0_all_string_formats_ignore_integers() {}
    #[tokio::test]
    async fn test1_all_string_formats_ignore_floats() {}
    #[tokio::test]
    async fn test2_all_string_formats_ignore_objects() {}
    #[tokio::test]
    async fn test3_all_string_formats_ignore_arrays() {}
    #[tokio::test]
    async fn test4_all_string_formats_ignore_booleans() {}
    #[tokio::test]
    async fn test5_all_string_formats_ignore_nulls() {}
    #[tokio::test]
    async fn test6_a_valid_host_name() {}
    #[tokio::test]
    async fn test7_a_valid_punycoded_idn_hostname() {}
    #[tokio::test]
    async fn test8_a_host_name_starting_with_an_illegal_character() {}
    #[tokio::test]
    async fn test9_a_host_name_containing_illegal_characters() {}
    #[tokio::test]
    async fn test10_a_host_name_with_a_component_too_long() {}
    #[tokio::test]
    async fn test11_starts_with_hyphen() {}
    #[tokio::test]
    async fn test12_ends_with_hyphen() {}
    #[tokio::test]
    async fn test13_starts_with_underscore() {}
    #[tokio::test]
    async fn test14_ends_with_underscore() {}
    #[tokio::test]
    async fn test15_contains_underscore() {}
    #[tokio::test]
    async fn test16_maximum_label_length() {}
    #[tokio::test]
    async fn test17_exceeds_maximum_label_length() {}
}
