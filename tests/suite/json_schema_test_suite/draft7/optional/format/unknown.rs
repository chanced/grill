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
mod unknown_format_0 {
    #[tokio::test]
    async fn test0_unknown_formats_ignore_integers() {}
    #[tokio::test]
    async fn test1_unknown_formats_ignore_floats() {}
    #[tokio::test]
    async fn test2_unknown_formats_ignore_objects() {}
    #[tokio::test]
    async fn test3_unknown_formats_ignore_arrays() {}
    #[tokio::test]
    async fn test4_unknown_formats_ignore_booleans() {}
    #[tokio::test]
    async fn test5_unknown_formats_ignore_nulls() {}
    #[tokio::test]
    async fn test6_unknown_formats_ignore_strings() {}
}
