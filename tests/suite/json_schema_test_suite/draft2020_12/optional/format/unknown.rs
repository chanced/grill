use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
mod unknown_format_0 {
    #[test]
    fn test0_unknown_formats_ignore_integers() {}
    #[test]
    fn test1_unknown_formats_ignore_floats() {}
    #[test]
    fn test2_unknown_formats_ignore_objects() {}
    #[test]
    fn test3_unknown_formats_ignore_arrays() {}
    #[test]
    fn test4_unknown_formats_ignore_booleans() {}
    #[test]
    fn test5_unknown_formats_ignore_nulls() {}
    #[test]
    fn test6_unknown_formats_ignore_strings() {}
}
