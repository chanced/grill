use crate::Language;

/// The associated type `Keyword` of
pub type Keyword<L, Key> = <L as Language<Key>>::Keyword;
pub type Context<L, Key> = <Keyword<L, Key> as crate::Keyword<L, Key>>::Context;
pub type Compile<L, Key> = <Keyword<L, Key> as crate::Keyword<L, Key>>::Compile;
pub type Output<L, Key> = <Keyword<L, Key> as crate::Keyword<L, Key>>::Output;
pub type Structure<L, Key> = <Output<L, Key> as crate::Output>::Structure;
pub type ValidationError<L, Key> = <Output<L, Key> as crate::Output>::Error;
