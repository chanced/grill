pub trait Applicator {
    fn execute(&self, interrogator: Interrogator);
}

// pub(crate) struct Executor<A> {
//     applicator: A,
// }

// borrowed from axum
//
// see: https://docs.rs/axum/0.5.5/src/axum/macros.rs.html#181
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!(T1);
        $name!(T1, T2);
        $name!(T1, T2, T3);
        $name!(T1, T2, T3, T4);
        $name!(T1, T2, T3, T4, T5);
        $name!(T1, T2, T3, T4, T5, T6);
        $name!(T1, T2, T3, T4, T5, T6, T7);
        $name!(T1, T2, T3, T4, T5, T6, T7, T8);
    };
}

macro_rules! impl_applicator {
    ( $($ty:ident),* $(,)? ) => {
        impl<F, $($ty,)*> Applicator for F
        where
            F: FnOnce($($ty,)*) -> Option<dyn FnOnce() -> Result<(), crate::Error>>,
            $( $ty: Injectable,)*
        {
        }
    };
}
all_the_tuples!(impl_applicator);
