pub mod reference;

mod consts;
pub use consts::*;

use crate::{
    report::{Assess, Location},
    schema::{dialect::Dialects, CompiledSchema},
    spec::{self, Specification},
    JsonSchema,
};
use grill_core::{
    lang::{Schemas, Sources},
    state::{State, Transaction},
    Key, Resolve,
};
use grill_uri::AbsoluteUri;
use serde_json::Value;
use std::{mem, sync::Arc};

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Keyword                                ║
║                               ¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A JSON Schema keyword.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {}

impl<S, K> spec::Keyword<S, K> for Keyword
where
    S: spec::Specification<K>,
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

    fn evaluate<'rpt, 'int>(
        &self,
        eval: S::Evaluate<'rpt, 'int>,
    ) -> Result<(), spec::alias::EvaluateError<S, K>> {
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
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Keywords                               ║
║                               ¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
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
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Compile                                ║
║                               ¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Context for [`Keyword::compile`].
pub struct Compile<'txn, 'int, 'res, R, S, K>
where
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
{
    pub targets: Vec<AbsoluteUri>,
    pub txn: &'txn mut Transaction<'txn, 'int, JsonSchema<K, S>, K>,
    pub resolve: &'res R,
    pub dialects: &'int Dialects<S, K>,
    pub validate: bool,
}

impl<'txn, 'int, 'res, R, S, K> spec::Compile<'txn, 'int, 'res, R, S, K>
    for Compile<'txn, 'int, 'res, R, S, K>
where
    R: 'static + Resolve + Send + Sync,
    K: 'static + Key + Send + Sync,
    S: Specification<K>,
{
    fn dialects(&self) -> &Dialects<S, K> {
        self.dialects
    }

    fn should_validate(&self) -> bool {
        self.validate
    }

    fn targets(&mut self) -> &[AbsoluteUri] {
        &self.targets
    }

    fn txn(&mut self) -> &mut Transaction<'_, 'int, JsonSchema<K>, K> {
        todo!()
    }

    fn resolve(&self) -> &'res R {
        todo!()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               Evaluate                                ║
║                              ¯¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Context for [`Keyword::evaluate`].
pub struct Evaluate<'rpt, 'int, S, K>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    pub target: K,
    pub state: &'int State<JsonSchema<K>, K>,
    pub schemas: &'int Schemas<CompiledSchema<S, K>, K>,
    pub assess:
        <S::Report<'rpt> as spec::Report<'rpt, S::Annotation<'rpt>, S::Error<'rpt>>>::Assess<'rpt>,
    pub dialects: &'int Dialects<S, K>,
}

impl<'rpt, 'int, S, K> spec::Evaluate<'rpt, 'int, S, K> for Evaluate<'rpt, 'int, S, K>
where
    K: 'static + Key + Send + Sync,
    S: Specification<K>,
{
    fn dialects(&self) -> &Dialects<S, K> {
        self.dialects
    }

    fn assess(
        &mut self,
    ) -> &mut <S::Report<'rpt> as spec::Report<'rpt, S::Annotation<'rpt>, S::Error<'rpt>>>::Assess<'rpt>
    {
        &mut self.assess
    }
}
