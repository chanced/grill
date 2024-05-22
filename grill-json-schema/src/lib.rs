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

use core::language::{NewCompile, NewContext};

pub(crate) use grill_core as core;

use grill_core::{language::Language, Key};
use integration::{Compile, Context};
use keyword::Keyword;

pub mod integration;
pub mod keyword;
pub mod report;

pub use report::{Annotation, Error, Output, Report};

impl std::fmt::Display for Report<'_> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct JsonSchema {}

impl<K> Language<K> for JsonSchema
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
        params: NewContext<'i, 'v, 'r, Self, K>,
    ) -> Self::Context<'i, 'v, 'r> {
        Context {
            eval_numbers: params.eval_numbers,
            global_numbers: params.global_numbers,
            report: params.report,
            schemas: params.schemas,
            sources: params.sources,
            dialects: params.dialects,
        }
    }
}

/// Returns a static reference to [`Value::Bool`] with the given value.
#[must_use]
pub const fn boolean(value: bool) -> &'static Value {
    if value {
        TRUE
    } else {
        FALSE
    }
}
