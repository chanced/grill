pub mod consts;

use crate::{
    dialect::Dialects,
    schema::CompiledSchema,
    spec::{self, Specification},
    JsonSchema,
};
use grill_core::{cache::Cache, lang::Schemas, state::State, Key, Resolve};
use serde_json::Value;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Keyword                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A JSON Schema keyword.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {}

impl<S, K> spec::keyword::Keyword<S, K> for Keyword
where
    S: Specification<K>,
    K: 'static + Key + Sync + Send,
{
    async fn compile<R>(
        &self,
        compile: S::Compile<'_, '_, '_, R>,
    ) -> Option<Result<(), S::CompileError<R>>>
    where
        R: 'static + Resolve + Send + Sync,
    {
        todo!()
    }

    fn evaluate<'int, 'val, 'req>(
        &self,
        eval: S::Evaluate<'int, 'val, 'req>,
    ) -> Result<(), S::EvaluateError> {
        _ = eval;
        todo!()
    }

    fn reference(&self, _schema: &Value) -> Option<spec::found::Reference> {
        None
    }

    fn anchor(&self, _schema: &Value) -> Option<spec::found::Anchor> {
        None
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Keywords                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A slice of [`Keyword`]s belonging to a schema.
pub struct Keywords<'int>(pub &'int [Keyword]);

impl<'int> From<&'int [Keyword]> for Keywords<'int> {
    fn from(keywords: &'int [Keyword]) -> Self {
        Self(keywords)
    }
}
impl<'int> IntoIterator for Keywords<'int> {
    type Item = &'int Keyword;
    type IntoIter = std::slice::Iter<'int, Keyword>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Keywords<'_> {
    /// Returns the slice of keywords.
    pub fn as_slice(&self) -> &[Keyword] {
        self.0
    }
}
impl AsRef<[Keyword]> for Keywords<'_> {
    fn as_ref(&self) -> &[Keyword] {
        self.0
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Compile                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Context for [`Keyword::compile`].
pub struct Compile<'int, 'txn, 'res, R, S, K>
where
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
{
    pub context: grill_core::lang::compile::Context<'int, 'txn, 'res, JsonSchema<K, S>, R, K>,
    pub dialects: &'int Dialects<S, K>,
}

impl<'int, 'txn, 'res, R, S, K> spec::compile::Context<'int, 'txn, 'res, R, S, K>
    for Compile<'int, 'txn, 'res, R, S, K>
where
    R: 'static + Resolve + Send + Sync,
    K: 'static + Key + Send + Sync,
    S: Specification<K>,
{
    fn core_ctx(
        &mut self,
    ) -> &mut grill_core::lang::compile::Context<'int, 'txn, 'res, JsonSchema<K, S>, R, K> {
        &mut self.context
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Evaluate                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Context for [`Keyword::evaluate`].
pub struct Evaluate<'int, 'val, 'req, S, K>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
    K: 'static + Send,
{
    pub target: K,
    pub eval: &'req mut Cache,
    pub state: &'int State<JsonSchema<K, S>, K>,
    pub schemas: &'int Schemas<CompiledSchema<S, K>, K>,
    pub assess:
        <S::Report<'val> as spec::Report<'val, S::Annotation<'val>, S::Error<'val>>>::Assess<'val>,
    pub dialects: &'int Dialects<S, K>,
    pub value: &'val Value,
}

impl<'int, 'val, 'req, S, K> spec::Evaluate<'int, 'val, S, K> for Evaluate<'int, 'val, 'req, S, K>
where
    K: 'static + Key + Send + Sync,
    S: Specification<K>,
{
    fn dialects(&self) -> &Dialects<S, K> {
        self.dialects
    }

    fn assess(
        &mut self,
    ) -> &mut <S::Report<'val> as spec::Report<'val, S::Annotation<'val>, S::Error<'val>>>::Assess<'val>
    {
        &mut self.assess
    }
}
