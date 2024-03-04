use crate::Language;

/// The associated type `Keyword` of
pub type Keyword<L, K> = <L as Language<K>>::Keyword;
pub type Context<L, K> = <Keyword<L, K> as crate::Keyword<L, K>>::Context;
pub type Compile<L, K> = <Keyword<L, K> as crate::Keyword<L, K>>::Compile;
pub type Output<L, K> = <Keyword<L, K> as crate::Keyword<L, K>>::Evaluation;
pub type Structure<L, K> = <Output<L, K> as crate::Evaluation>::Output;
pub type ValidationError<L, K> = <Output<L, K> as crate::Evaluation>::Error;
