/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Compile                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

use crate::{
    dialect::Dialects,
    schema::CompiledSchema,
    spec::{self, Specification},
    JsonSchema,
};
use grill_core::{cache::Cache, schema::Schemas, state::State, Resolve};
use serde_json::Value;
use slotmap::Key;

/// Context for [`Keyword::compile`].
pub struct Compile<'int, 'txn, 'res, R, S, K>
where
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
{
    pub context: grill_core::lang::context::Compile<'int, 'txn, 'res, JsonSchema<K, S>, R, K>,
    pub dialects: &'int Dialects<S, K>,
}

impl<'int, 'txn, 'res, R, S, K> spec::Compile<'int, 'txn, 'res, R, S, K>
    for Compile<'int, 'txn, 'res, R, S, K>
where
    R: 'static + Resolve + Send + Sync,
    K: 'static + Key + Send + Sync,
    S: Specification<K>,
{
    fn core(
        &mut self,
    ) -> &mut grill_core::lang::context::Compile<'int, 'txn, 'res, JsonSchema<K, S>, R, K> {
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
