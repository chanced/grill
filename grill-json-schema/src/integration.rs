use std::fmt;

use grill_core::{
    cache::{Numbers, Values},
    criterion::{self, Criterion, NewContext},
    error::CompileError,
    schema::{Dialects, Schemas},
    source::{Deserializers, Resolvers, Sources},
    Key,
};
use grill_uri::{AbsoluteUri, Uri};

use crate::{Annotation, Error, JsonSchema, Report};

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
    pub fn evaluate(&mut self, key: K) {
        // let mut instance_location = self.report.clone();
        // if let Some(instance) = instance {
        //     instance_location.push_back(instance.into());
        // }
        // self.evaluated.insert(instance_location.to_string());
        // let mut keyword_location = self.keyword_location.clone();
        // keyword_location.append(keyword);

        // self.structure,
        // key,
        // value,
        // instance_location,
        // keyword_location,
        // self.sources,
        // self.evaluated,
        // self.global_state,
        // self.eval_state,
        // self.global_numbers,
        // self.eval_numbers,
        // self.schemas.evaluate(Evaluate{
        //     output: ,
        //     key,
        //     value: todo!(),
        //     instance_location: todo!(),
        //     keyword_location,
        //     sources: todo!(),
        //     dialects: todo!(),
        //     global_numbers: todo!(),
        //     eval_numbers: todo!(),
        //     criterion: todo!(),
        // }
        // )
        todo!()
    }
    pub fn annotate(&mut self, annotation: Annotation) {
        self.report.push_annotation(annotation);
    }
    pub fn error(&mut self, error: Error) {
        self.report.push_error(error);
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
    pub schema_uri: &'i AbsoluteUri,
    pub global_numbers: &'i mut Numbers,
    pub schemas: &'i Schemas<C, K>,
    pub sources: &'i Sources,
    pub dialects: &'i Dialects<C, K>,
    pub resolvers: &'i Resolvers,
    pub deserializers: &'i Deserializers,
    pub values: &'i mut Values,
}
impl<'i, C, K> Compile<'i, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    pub fn schema_key_for_uri<U>(&self, uri: U) -> Result<K, CompileError<C, K>>
    where
        U: TryInto<Uri, Error = grill_uri::Error>,
    {
        let uri = uri.try_into()?; // Uri
        let uri: AbsoluteUri = uri.try_into()?; // AbsoluteUri
        let uri = self.schema_uri.with_fragment(None)?.resolve(&uri)?; // resolved AbsoluteUri
        self.schemas
            .get_key(&uri)
            .ok_or(CompileError::schema_not_found(uri))
    }
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
            .field("absolute_uri", &self.schema_uri)
            .field("global_numbers", &self.global_numbers)
            .field("schemas", &self.schemas)
            .field("sources", &self.sources)
            .field("dialects", &self.dialects)
            .field("deserializers", &self.deserializers)
            .field("values", &self.values)
            .finish_non_exhaustive()
    }
}
