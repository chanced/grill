mod max_length_validation_0 {
    #[test]
    fn test0_shorter_is_valid() {}
    #[test]
    fn test1_exact_length_is_valid() {}
    #[test]
    fn test2_too_long_is_invalid() {}
    #[test]
    fn test3_ignores_non_strings() {}
    #[test]
    fn test4_two_supplementary_unicode_code_points_is_long_enough() {}
}
mod max_length_validation_with_a_decimal_1 {
    #[test]
    fn test0_shorter_is_valid() {}
    #[test]
    fn test1_too_long_is_invalid() {}
}
