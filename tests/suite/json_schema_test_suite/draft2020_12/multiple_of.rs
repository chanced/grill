mod by_int_0 {
    #[tokio::test]
    async fn test0_int_by_int() {}
    #[tokio::test]
    async fn test1_int_by_int_fail() {}
    #[tokio::test]
    async fn test2_ignores_non_numbers() {}
}
mod by_number_1 {
    #[tokio::test]
    async fn test0_zero_is_multiple_of_anything() {}
    #[tokio::test]
    async fn test1_4_5_is_multiple_of_1_5() {}
    #[tokio::test]
    async fn test2_35_is_not_multiple_of_1_5() {}
}
mod by_small_number_2 {
    #[tokio::test]
    async fn test0_0_0075_is_multiple_of_0_0001() {}
    #[tokio::test]
    async fn test1_0_00751_is_not_multiple_of_0_0001() {}
}
mod float_division_inf_3 {
    #[tokio::test]
    async fn test0_always_invalid_but_naive_implementations_may_raise_an_overflow_error() {}
}
mod small_multiple_of_large_integer_4 {
    #[tokio::test]
    async fn test0_any_integer_is_a_multiple_of_1e_8() {}
}
