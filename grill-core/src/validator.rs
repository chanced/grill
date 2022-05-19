// pub trait Handler<T>: Clone + Send + Sized + 'static {}

// macro_rules! impl_validator {
//     ( $($ty:ident),* $(,)? ) => {
//         impl<F, X, $($ty,)*> Handler<($($ty,)*)> for F
//         where
//             F: FnOnce($($ty,)*) ->  X + Clone + Send + 'static,
//         {
//         }
//     };
// }
// all_the_tuples!(impl_handler);

// pub type ValidatorFn = dyn Fn(Value) -> Result<(), Error> + Send + Sync + 'static;
