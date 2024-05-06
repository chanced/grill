use std::fmt;

use grill_core::{
    cache::{Numbers, Values},
    criterion::{self, Criterion, NewContext},
    schema::{Dialects, Schemas},
    source::{Deserializers, Resolvers, Sources},
    Key,
};
use grill_uri::AbsoluteUri;

use crate::{JsonSchema, Report};

/// Types only needed for integration with [`Criterion`].

#[derive(Debug)]
pub struct Context<'i, 'v, 'r, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    pub report: &'r mut Report<'v>,
    pub eval_numbers: &'i mut Numbers,
    pub global_numbers: &'i Numbers,
    pub schemas: &'i Schemas<C, K>,
    pub sources: &'i Sources,
    pub dialects: &'i Dialects<JsonSchema, K>,
}

impl<'i, 'v, 'r, K> Context<'i, 'v, 'r, JsonSchema, K>
where
    K: 'static + Key,
{
    pub fn new(ctx: NewContext<'i, 'v, 'r, JsonSchema, K>) -> Self {
        Context {
            report: ctx.report,
            eval_numbers: ctx.eval_numbers,
            global_numbers: ctx.global_numbers,
            schemas: ctx.schemas,
            sources: ctx.sources,
            dialects: ctx.dialects,
        }
    }
}

impl<'i, 'v, 'r, C, K> criterion::Context<'i, 'v, 'r, C, K> for Context<'i, 'v, 'r, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
}

pub struct Compile<'i, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    pub absolute_uri: &'i AbsoluteUri,
    pub global_numbers: &'i mut Numbers,
    pub schemas: &'i Schemas<C, K>,
    pub sources: &'i Sources,
    pub dialects: &'i Dialects<C, K>,
    pub resolvers: &'i Resolvers,
    pub deserializers: &'i Deserializers,
    pub values: &'i mut Values,
}

impl<'i, C, K> criterion::Compile<'i> for Compile<'i, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
}

impl<'i, C, K> fmt::Debug for Compile<'i, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Compile")
            .field("absolute_uri", &self.absolute_uri)
            .field("global_numbers", &self.global_numbers)
            .field("schemas", &self.schemas)
            .field("sources", &self.sources)
            .field("dialects", &self.dialects)
            .field("deserializers", &self.deserializers)
            .field("values", &self.values)
            .finish_non_exhaustive()
    }
}
