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
    criterion::{Assessment, Criterion, Report as CriterionReport},
    schema::{Dialects, Schemas},
    source::{Deserializers, Resolvers, Sources},
    uri::AbsoluteUri,
    Key,
};
use keyword::Keyword;
use serde::{Deserialize, Serialize};

pub mod keyword;

// /// Set of keywords to check which disable short-circuiting
// pub const DISABLING_KEYWORDS: [&'static str; 2] = [UNEVALUATED_PROPERTIES, UNEVALUATED_ITEMS];

// if Self::ENABLING_STRUCTURES.contains(ctx.structure().into()) {
//     ctx.enable_short_circuiting();
// }
// pub const ENABLING_STRUCTURES: Structures = Structures::FLAG;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Output {}
impl grill_core::criterion::Output for Output {
    fn verbose() -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report<'v> {
    marker: PhantomData<&'v i32>,
}

impl std::fmt::Display for Report<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for Report<'_> {}

#[derive(Debug, Clone)]
pub struct JsonSchema {}

impl<'i, C, K> grill_core::criterion::Context<'i, C, K> for Context<'i, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
}
#[derive(Debug, Clone)]
pub struct Context<'i, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    marker: PhantomData<&'i C>,
    marker2: PhantomData<&'i K>,
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

impl<'i, C, K> grill_core::criterion::Compile<'i> for Compile<'i, C, K>
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Annotation<'v> {
    X(Cow<'v, str>),
}
impl Default for Annotation<'_> {
    fn default() -> Self {
        todo!()
    }
}

impl<'v> CriterionReport<'v> for Report<'v> {
    type Error<'e> = Error<'e>;
    type Annotation<'a> = Annotation<'a>;
    type Output = Output;
    type Owned = Report<'static>;

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn append(&mut self, nodes: impl Iterator<Item = Self>) {
        todo!()
    }

    fn push(&mut self, output: Self) {
        todo!()
    }

    fn into_owned(self) -> Self::Owned {
        todo!()
    }

    fn new(
        output: Self::Output,
        absolute_keyword_location: &AbsoluteUri,
        keyword_location: jsonptr::Pointer,
        instance_location: jsonptr::Pointer,
        assessment: Assessment<Self::Annotation<'v>, Self::Error<'v>>,
    ) -> Self {
        todo!()
    }
}

impl<K> Criterion<K> for JsonSchema
where
    K: 'static + Key + 'static,
{
    type Context<'i> = Context<'i, Self, K>;

    type Compile<'i> = Compile<'i, Self, K>;

    type Keyword = Keyword;

    type OwnedReport = Report<'static>;
    type Report<'v> = Report<'v>;

    fn new_compile<'i>(&mut self, params: NewCompile<'i, Self, K>) -> Self::Compile<'i> {
        todo!()
    }

    fn new_context<'i, 'v, 'r>(
        &self,
        params: NewContext<'i, 'v, 'r, Self, K>,
    ) -> Self::Context<'i> {
        todo!()
    }
}
pub(crate) mod alias {
    use crate::JsonSchema;
    use grill_core::criterion::Criterion;

    pub(crate) type Compile<'i, K> = <JsonSchema as Criterion<K>>::Compile<'i>;
    pub(crate) type Context<'i, K> = <JsonSchema as Criterion<K>>::Context<'i>;
    pub(crate) type Report<'v, K> = <JsonSchema as Criterion<K>>::Report<'v>;
}
