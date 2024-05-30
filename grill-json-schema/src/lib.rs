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
pub mod keyword;
pub mod report;
pub mod schema;
pub mod spec;

use grill_core::{lang::Init, Key, Language, Resolve};
use report::{Annotation, Error};
use schema::{dialect::Dialect, CompiledSchema};
use spec::{alias, Specification};

pub use {
    compile::CompileError,
    report::{Output, Report},
};

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               IntoOwned                               ║
║                              ¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait implemented by types that can be converted into an owned type.
pub trait IntoOwned {
    /// The owned type.
    type Owned: 'static;
    /// Consumes `self`, returning `Self::Owned`.
    fn into_owned(self) -> Self::Owned;
}

#[derive(Debug, Clone)]
pub struct JsonSchema<S = Spec>(pub S);

impl<S> JsonSchema<S> {
    pub fn new(spec: S) -> Self {
        Self(spec)
    }
}

/// Std JSON Schema specification.
#[derive(Clone, Debug)]
pub struct Spec {
    dialects: Vec<Dialect<keyword::Keyword>>,
    primary_dialect_idx: usize,
}

impl<K: Key + Send> Specification<K> for Spec {
    type InitError = ();

    type CompileError = CompileError<Error<'static>>;

    type EvaluateError = EvaluateError<K>;

    type Evaluate = keyword::Evaluate;

    type Compile = keyword::Compile<Self, K>;

    type Keyword = keyword::Keyword;

    type Annotation<'v> = report::Annotation<'v>;

    type Error<'v> = report::Error<'v>;

    type Report<'v> = report::Report<Self::Annotation<'v>, Self::Error<'v>>;

    async fn compile<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile: grill_core::lang::Compile<'i, CompiledSchema<Self, K>, R, K>,
    ) -> Result<Self::Compile, Self::CompileError> {
        todo!()
    }

    fn evaluate<'i, 'v>(
        &'i self,
        eval: grill_core::lang::Evaluate<'i, 'v, CompiledSchema<Self, K>, Output, K>,
    ) -> Result<Self::Evaluate, Self::EvaluateError> {
        todo!()
    }
}

impl<S, K> Language<K> for JsonSchema<S>
where
    S: Specification<K> + Send,
    K: 'static + Key + Send,
{
    /// The [`CompiledSchema`](schema::CompiledSchema) of this language.
    type CompiledSchema = CompiledSchema<S, K>;

    /// The error type possibly returned from [`compile`](Language::compile).
    type CompileError = alias::CompileError<S, K>;

    /// The result type returned from [`evaluate`](Language::evaluate).
    type EvaluateResult<'v> = alias::EvaluateResult<'v, S, K>;

    /// Context type supplied to `evaluate`.
    type Context = alias::Evaluate<S, K>;

    /// The error type that can be returned when initializing the language.
    type InitError = alias::InitError<S, K>;

    /// Initializes the language with the given [`Init`] request.
    fn init(&mut self, init: Init<'_, Self::CompiledSchema, K>) -> Result<(), Self::InitError> {
        Specification::init(self, init)
    }

    /// Compiles a schema for the given [`Compile`] request and returns the key,
    /// if successful.
    ///
    /// This method is `async` to allow for languages that need to fetch schemas
    /// during compilation.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    async fn compile<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile: grill_core::lang::Compile<'i, Self::CompiledSchema, R, K>,
    ) -> Result<K, Self::CompileError> {
        todo!()
    }

    /// Compiles all schemas for the given [`CompileAll`] request and returns the
    /// keys, if successful.
    async fn compile_all<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile_all: grill_core::lang::CompileAll<'i, Self::CompiledSchema, R, K>,
    ) -> Result<Vec<K>, Self::CompileError> {
        todo!()
    }

    /// Evaluates a schema for the given [`Evaluate`] request.
    fn evaluate<'i, 'v>(
        &'i self,
        eval: grill_core::lang::Evaluate<'i, 'v, Self::CompiledSchema, Self::Context, K>,
    ) -> Self::EvaluateResult<'v> {
        todo!()
    }
}

pub struct EvaluateError<K> {
    pub key: K,
}
