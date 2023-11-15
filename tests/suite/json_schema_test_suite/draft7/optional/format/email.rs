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
mod validation_of_e_mail_addresses_0 {
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
    async fn test6_a_valid_e_mail_address() {}
    #[tokio::test]
    async fn test7_an_invalid_e_mail_address() {}
    #[tokio::test]
    async fn test8_tilde_in_local_part_is_valid() {}
    #[tokio::test]
    async fn test9_tilde_before_local_part_is_valid() {}
    #[tokio::test]
    async fn test10_tilde_after_local_part_is_valid() {}
    #[tokio::test]
    async fn test11_dot_before_local_part_is_not_valid() {}
    #[tokio::test]
    async fn test12_dot_after_local_part_is_not_valid() {}
    #[tokio::test]
    async fn test13_two_separated_dots_inside_local_part_are_valid() {}
    #[tokio::test]
    async fn test14_two_subsequent_dots_inside_local_part_are_not_valid() {}
}
