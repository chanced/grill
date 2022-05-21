// use crate::{Error, Evaluation, Interrogator, Next, Result, Value};
// use paste::paste;

// pub trait Applicator<T> {
//     fn setup(
//         self,
//         interrogator: Interrogator,
//     ) -> Box<dyn FnOnce(Value, Next) -> Result<Evaluation>>;
// }

// impl<F> Applicator<()> for F
// where
//     F: FnOnce() -> Box<dyn FnOnce(Value) -> Result<Evaluation>>,
// {
//     fn setup(self, _: Interrogator) -> Box<dyn FnOnce(Value, Next) -> Result<Evaluation>> {
//         let f = self();
//         Box::new(move |value, next| {
//             match f(value.clone()) {
//                 Ok(eval) => {
//                     let sub_eval = next.call(value);
//                     // todo: merge sub_eval with eval
//                     sub_eval
//                 }
//                 Err(err) => Err(err),
//             }
//         })
//     }
// }

// // macro_rules! impl_applicator {
// //     ( $($ty:ident),* $(,)? ) => {
// //         paste! {
// //             impl<F, N, R, $($ty,)* $([<V $ty>],)*> Applicator<($($ty,)*), N, R> for F
// //             where
// //                 $([<V $ty>]: Clone + Send + Sync + 'static, )*
// //                 $( $ty: Injectable<Value = [< V $ty >]>, )*
// //                 F: FnOnce($($ty,)*),
// //                 N: Clone + Send + Sync + 'static + FnOnce(Value) -> Result<Annotations, Error>,
// //                 R: FnOnce(Value, N) -> Result<Annotations, Error>,
// //             {
// //                 fn setup(self, interrogator: Interrogator) -> R {
// //                     $(
// //                         let [< inj_ $ty:lower >] = interrogator.resolve::<$ty, [< V $ty >]>();
// //                     )*
// //                     self($([< inj_ $ty:lower >],)*);
// //                     todo!()
// //                 }
// //             }
// //         }
// //     };
// // }
// // tuplize!(impl_applicator);

// #[cfg(test)]
// mod test {
//     use crate::{Error, Evaluation, Result, Value};

//     use crate::{Context, Interrogator};

//     fn spike() -> Box<dyn FnOnce(Value) -> Result<Evaluation>> {
//         Box::new(move |v: Value| Ok(Evaluation::new()))
//     }
//     fn spike1(Context(str): Context<String>) -> Box<dyn FnOnce(Value) -> Result<Evaluation>> {
//         Box::new(move |_: Value| -> Result<Evaluation> {
//             println!("inside closure");
//             Ok(Evaluation::new())
//         })
//     }

//     fn spike2(Context(str): Context<String>, Context(i): Context<i8>) {
//         println!("{} {}", str, i);
//     }

//     #[test]
//     /// temp tests to get the API nailed down.
//     fn test_injection_of_single_param() {
//         let mut i = Interrogator::new();
//         i.call(spike);
//         // i.context(String::from("this is context"));
//         // i.context(3i8);
//         // i.call(spike2);
//         // i.call(spike);
//         // i.call(spike2);
//     }
// }
