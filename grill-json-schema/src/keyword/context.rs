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
    JsonSchema,
    dialect::Dialects,
    schema::CompiledSchema,
    spec::{self, Specification},
};
use grill_core::{Key, Resolve, cache::Cache, schema::Schemas, state::State};
use serde_json::Value;

/// Context for [`Keyword::compile`].
pub struct Compile<'int, 'txn, 'res, R, S>
where
    S: 'static + Specification,
{
    pub instance: grill_core::lang::context::Compile<'int, 'txn, 'res, JsonSchema<S>, R>,
    pub dialects: &'int Dialects<S>,
}

impl<'int, 'txn, 'res, R, S> spec::Compile<'int, 'txn, 'res, R, S>
    for Compile<'int, 'txn, 'res, R, S>
where
    R: 'static + Resolve + Send + Sync,
    S: Specification,
{
    fn instance(
        &mut self,
    ) -> &mut grill_core::lang::context::Compile<'int, 'txn, 'res, JsonSchema<S>, R> {
        &mut self.instance
    }
    fn language(&mut self) -> &mut Compile<'int, 'txn, 'res, R, S> {
        self
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
pub struct Evaluate<'int, 'val, 'req, S>
where
    S: Specification,
{
    pub target: Key,
    pub eval: &'req mut Cache,
    pub state: &'int State<JsonSchema<S>>,
    pub schemas: &'int Schemas<CompiledSchema<S>>,
    pub assess:
        <S::Report<'val> as spec::Report<'val, S::Annotation<'val>, S::Error<'val>>>::Assess<'val>,
    pub dialects: &'int Dialects<S>,
    pub value: &'val Value,
}

impl<'int, 'val, 'req, S> spec::Evaluate<'int, 'val, S> for Evaluate<'int, 'val, 'req, S>
where
    S: Specification,
{
    fn dialects(&self) -> &Dialects<S> {
        self.dialects
    }

    fn assess(
        &mut self,
    ) -> &mut <S::Report<'val> as spec::Report<'val, S::Annotation<'val>, S::Error<'val>>>::Assess<'val>
    {
        &mut self.assess
    }
}
