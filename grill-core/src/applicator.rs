use crate::{Context, Injectable, Interrogator};
use paste::paste;
use std::marker::PhantomData;

pub trait Applicator<T> {
    fn call(self, interrogator: Interrogator);
}

impl<F> Applicator<()> for F
where
    F: FnOnce(),
{
    fn call(self, _: Interrogator) {
        self();
    }
}

macro_rules! impl_applicator {
    ( $($ty:ident),* $(,)? ) => {
        paste! {
            impl<F, $($ty,)* $([<V $ty>],)*> Applicator<($($ty,)*)> for F
            where
                $([<V $ty>]: Clone + Send + Sync + 'static, )*
                $( $ty: Injectable<Value = [< V $ty >]>, )*
                F: FnOnce($($ty,)*),
            {
                fn call(self, interrogator: Interrogator) {
                    $(
                        let [< inj_ $ty:lower >] = interrogator.resolve::<$ty, [< V $ty >]>();
                    )*
                    self($([< inj_ $ty:lower >],)*);
                }
            }
        }
    };
}
macro_rules! tuplize {
    ($name:ident) => {
        $name!(I1);
        $name!(I1, I2);
        $name!(I1, I2, I3);
        $name!(I1, I2, I3, I4);
        $name!(I1, I2, I3, I4, I5);
        $name!(I1, I2, I3, I4, I5, I6);
        $name!(I1, I2, I3, I4, I5, I6, I7);
        $name!(I1, I2, I3, I4, I5, I6, I7, I8);
    };
}

tuplize!(impl_applicator);

#[cfg(test)]
mod test {
    use crate::{Context, Interrogator};

    fn spike(Context(str): Context<String>) {
        println!("{}", str);
    }

    fn spike2(Context(str): Context<String>, Context(i): Context<i8>) {
        println!("{} {}", str, i);
    }

    #[test]
    fn test_injection_of_single_param() {
        let mut i = Interrogator::new();
        i.context(String::from("this is context"));
        i.context(3i8);
        i.call(spike);
        i.call(spike2);
        i.call(spike);
        i.call(spike2);
    }
}
