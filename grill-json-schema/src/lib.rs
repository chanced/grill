//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.

#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::implicit_hasher, clippy::wildcard_imports)]
#![cfg_attr(test, allow(clippy::redundant_clone, clippy::too_many_lines))]
#![recursion_limit = "256"]

pub mod compile;
pub mod report;
pub mod schema;

pub use {
    compile::CompileError,
    report::{Output, Report},
};

use async_trait::async_trait;
use grill_core::{
    lang::{Compile, CompileAll, Evaluate, Init},
    Key, Language, Resolve,
};

#[derive(Debug, Clone)]
/// JSON Schema [`Language`] implementation.
pub struct JsonSchema<K: Key> {
    _marker: std::marker::PhantomData<K>,
}

#[async_trait]
impl<K> Language<K> for JsonSchema<K>
where
    K: 'static + Key + Send + Sync,
{
    type CompiledSchema = schema::CompiledSchema<K>;
    type CompileError = CompileError;
    type EvaluateResult<'v> = Result<Report<'v>, EvaluateError<K>>;
    type Context = Output;
    type InitError = ();

    fn init(&mut self, init: Init<'_, Self::CompiledSchema, K>) -> Result<(), Self::InitError> {
        todo!()
    }

    async fn compile<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile: Compile<'i, Self::CompiledSchema, R, K>,
    ) -> Result<K, Self::CompileError> {
        todo!()
    }

    async fn compile_all<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile_all: CompileAll<'i, Self::CompiledSchema, R, K>,
    ) -> Result<Vec<K>, Self::CompileError> {
        todo!()
    }

    fn evaluate<'i, 'v>(
        &'i self,
        eval: Evaluate<'i, 'v, Self::CompiledSchema, Self::Context, K>,
    ) -> Self::EvaluateResult<'v> {
        todo!()
    }
}

pub struct EvaluateError<K> {
    pub key: K,
}

pub struct InitError {}
