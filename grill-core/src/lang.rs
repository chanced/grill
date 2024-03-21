use crate::Criterion;

/// The associated type `Keyword` of
pub type Keyword<C, K> = <C as Criterion<K>>::Keyword;
// pub type Context<C, K> = <Keyword<C, K> as crate::Keyword<C, K>>::Context;
// pub type Compile<C, K> = <Keyword<C, K> as crate::Keyword<C, K>>::Compile;
// pub type Output<C, K> = <Keyword<C, K> as crate::Keyword<C, K>>::Evaluation;
// pub type Structure<C, K> = <Output<C, K> as crate::Evaluation>::Output;
// pub type ValidationError<C, K> = <Output<C, K> as crate::Evaluation>::Error;
