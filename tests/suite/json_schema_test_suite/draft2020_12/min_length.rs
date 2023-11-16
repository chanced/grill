mod min_length_validation_0 {
    #[test]
    fn test0_longer_is_valid() {}
    #[test]
    fn test1_exact_length_is_valid() {}
    #[test]
    fn test2_too_short_is_invalid() {}
    #[test]
    fn test3_ignores_non_strings() {}
    #[test]
    fn test4_one_supplementary_unicode_code_point_is_not_long_enough() {}
}
mod min_length_validation_with_a_decimal_1 {
    #[test]
    fn test0_longer_is_valid() {}
    #[test]
    fn test1_too_short_is_invalid() {}
}
