use grill_core::Key;

use crate::{CompileError, EvaluateError, Report, Specification};

use crate::keyword::{Compile, Evaluate, Keyword};

pub struct Schema<S, K: Key> {
    _marker: std::marker::PhantomData<(S, K)>,
}

impl<S, K> Keyword<S, K> for Schema<S, K>
where
    S: Specification<K>,
    K: Key,
{
    fn compile<'int>(
        &self,
        compile: alias::Compile<S, K>,
    ) -> Option<Result<(), alias::CompileError<S, K>>> {
        todo!()
    }

    fn evaluate<'v>(&self, eval: alias::Evaluate<S, K>) -> Result<(), alias::EvaluateError<S, K>> {
        todo!()
    }
}
