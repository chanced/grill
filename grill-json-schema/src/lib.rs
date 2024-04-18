//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.

#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
// #![warn(missing_docs)]
#![allow(
    clippy::module_name_repetitions,
    clippy::result_large_err,
    clippy::enum_glob_use,
    clippy::implicit_hasher,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::missing_panics_doc, // TODO: remove after todo!()s are removed
    clippy::missing_errors_doc, // TODO: remove when I get around to documenting
    clippy::wildcard_imports,
    clippy::module_inception,
    clippy::unreadable_literal
)]
#![cfg_attr(test, allow(clippy::redundant_clone, clippy::too_many_lines))]
#![recursion_limit = "256"]

use core::criterion::{NewCompile, NewContext};
use std::{borrow::Cow, marker::PhantomData};

pub(crate) use grill_core as core;

use grill_core::{
    cache::{Numbers, Values},
    criterion::{self, Criterion},
    schema::{Dialects, Schemas},
    source::{Deserializers, Resolvers, Sources},
    uri::AbsoluteUri,
    Key,
};
use keyword::Keyword;
use serde::{Deserialize, Serialize};

pub mod keyword;
pub mod report;

pub use report::Report;

pub enum Annotation<'v> {
    Schema(Cow<'v, str>),
}

impl std::fmt::Display for Report<'_> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for Report<'_> {}

#[derive(Debug, Clone)]
pub struct JsonSchema {}

impl<'i, 'v, 'r, C, K> criterion::Context<'i, 'v, 'r, C, K> for Context<'i, 'v, 'r, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
}
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

impl<'i, C, K> std::fmt::Debug for Compile<'i, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error<'v> {
    X(Cow<'v, str>),
}

impl Default for Annotation<'_> {
    fn default() -> Self {
        todo!()
    }
}

impl<K> Criterion<K> for JsonSchema
where
    K: 'static + Key + 'static,
{
    type Context<'i, 'v, 'r> = Context<'i, 'v, 'r, Self, K> where 'v: 'r;

    type Compile<'i> = Compile<'i, Self, K>;

    type Keyword = Keyword;

    type OwnedReport = Report<'static>;
    type Report<'v> = Report<'v>;

    fn new_compile<'i>(&mut self, _params: NewCompile<'i, Self, K>) -> Self::Compile<'i> {
        todo!()
    }

    fn new_context<'i, 'v, 'r>(
        &self,
        _params: NewContext<'i, 'v, 'r, Self, K>,
    ) -> Self::Context<'i, 'v, 'r> {
        todo!()
    }
}
pub(crate) mod alias {
    use crate::JsonSchema;
    use grill_core::criterion::Criterion;

    pub(crate) type Compile<'i, K> = <JsonSchema as Criterion<K>>::Compile<'i>;
    pub(crate) type Context<'i, 'v, 'r, K> = <JsonSchema as Criterion<K>>::Context<'i, 'v, 'r>;
    pub(crate) type Report<'v, K> = <JsonSchema as Criterion<K>>::Report<'v>;
}
