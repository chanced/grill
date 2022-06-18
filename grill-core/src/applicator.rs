use crate::{Evaluation, Implementation, Interrogator, Next, Result, Value};

pub trait Applicator<T, I>
where
    I: Implementation,
{
    fn setup(
        self,
        interrogator: Interrogator<I>,
    ) -> Box<dyn FnOnce(I, Value, Evaluation<I>, Next<I>) -> Result<Evaluation<I>>>;
}

impl<F, I> Applicator<(), I> for F
where
    F: FnOnce() -> Box<dyn FnOnce(I, Value, Evaluation<I>) -> Result<Evaluation<I>>>,
    I: Implementation + 'static,
{
    fn setup(
        self,
        _: Interrogator<I>,
    ) -> Box<dyn FnOnce(I, Value, Evaluation<I>, Next<I>) -> Result<Evaluation<I>>> {
        let f = self();
        Box::new(move |imp, value, eval, next| -> Result<Evaluation<I>> {
            match f(imp, value.clone(), eval) {
                Ok(eval) => {
                    let sub_eval: Result<Evaluation<I>> = next.call(value);
                    // todo: merge sub_eval with eval
                    sub_eval
                }
                Err(err) => Err(err),
            }
        })
    }
}

// macro_rules! impl_applicator {
//     ( $($ty:ident),* $(,)? ) => {
//         paste! {
//             impl<F, N, R, $($ty,)* $([<V $ty>],)*> Applicator<($($ty,)*), N, R> for F
//             where
//                 $([<V $ty>]: Clone + Send + Sync + 'static, )*
//                 $( $ty: Injectable<Value = [< V $ty >]>, )*
//                 F: FnOnce($($ty,)*),
//                 N: Clone + Send + Sync + 'static + FnOnce(Value) -> Result<Annotations, Error>,
//                 R: FnOnce(Value, N) -> Result<Annotations, Error>,
//             {
//                 fn setup(self, interrogator: Interrogator) -> R {
//                     $(
//                         let [< inj_ $ty:lower >] = interrogator.resolve::<$ty, [< V $ty >]>();
//                     )*
//                     self($([< inj_ $ty:lower >],)*);
//                     todo!()
//                 }
//             }
//         }
//     };
// }
// tuplize!(impl_applicator);

#[cfg(test)]
mod test {
    use jsonptr::Pointer;

    use crate::Output;
    use crate::{Context, Implementation, Interrogator};
    use crate::{Evaluation, Result, Value};

    #[derive(Clone)]
    struct TestImpl {}

    impl Implementation for TestImpl {
        fn keyword_location_field() -> &'static str {
            "keywordLocation"
        }
        fn instance_location_field() -> &'static str {
            "instanceLocation"
        }
        fn error_field() -> &'static str {
            "error"
        }
    }

    fn spike<I: Implementation>() -> Box<dyn FnOnce(I, Value) -> Result<Evaluation<I>>> {
        Box::new(move |i: I, v: Value| {
            Ok(i.evaluation(
                Pointer::new(&["example"]),
                Pointer::new(&["example"]),
                Output::Basic,
            ))
        })
    }
    fn spike1<I: Implementation>(
        Context(str): Context<String>,
    ) -> Box<dyn FnOnce(I, Value) -> Result<Evaluation<I>>> {
        Box::new(move |imp: I, _: Value| -> Result<Evaluation<I>> {
            println!("inside closure");
            todo!()
        })
    }

    fn spike2(Context(str): Context<String>, Context(i): Context<i8>) {
        println!("{} {}", str, i);
    }

    #[test]
    /// temp tests to get the API nailed down.
    fn test_injection_of_single_param() {
        let mut i = Interrogator::new();
        i.call(spike);
        // i.context(String::from("this is context"));
        // i.context(3i8);
        // i.call(spike2);
        // i.call(spike);
        // i.call(spike2);
    }
}
