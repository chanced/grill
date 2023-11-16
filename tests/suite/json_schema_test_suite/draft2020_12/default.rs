mod invalid_type_for_default_0 {
    #[test]
    fn test0_valid_when_property_is_specified() {}
    #[test]
    fn test1_still_valid_when_the_invalid_default_is_used() {}
}
mod invalid_string_value_for_default_1 {
    #[test]
    fn test0_valid_when_property_is_specified() {}
    #[test]
    fn test1_still_valid_when_the_invalid_default_is_used() {}
}
mod the_default_keyword_does_not_do_anything_if_the_property_is_missing_2 {
    #[test]
    fn test0_an_explicit_property_value_is_checked_against_maximum_passing() {}
    #[test]
    fn test1_an_explicit_property_value_is_checked_against_maximum_failing() {}
    #[test]
    fn test2_missing_properties_are_not_filled_in_with_the_default() {}
}
