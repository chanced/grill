//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.

#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// #![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::implicit_hasher,
    clippy::wildcard_imports,
    clippy::module_name_repetitions
)]
#![cfg_attr(test, allow(clippy::redundant_clone, clippy::too_many_lines))]
#![recursion_limit = "256"]

pub mod compile;
pub mod invalid_type;
pub mod keyword;
pub mod report;
pub mod schema;
pub mod spec;

use core::fmt;
use std::marker::PhantomData;

use grill_core::{lang, Key, Language, Resolve};
use schema::CompiledSchema;
use slotmap::DefaultKey;
pub use spec::Spec;
use spec::Specification;
pub use {
    compile::CompileError,
    report::{Output, Report},
};

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   IntoOwned                                  ║
║                                  ¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait implemented by types that can be converted into an owned type.
pub trait IntoOwned {
    /// The owned type.
    type Owned: 'static;
    /// Consumes `self`, returning `Self::Owned`.
    fn into_owned(self) -> Self::Owned;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  JsonSchema                                  ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// JSON Schema with support for drafts 2020-12, 2019-09, 07, and 04.
#[derive(Debug, Clone)]
pub struct JsonSchema<K = DefaultKey, S = Spec<K>>
where
    K: 'static + Key + Send + Sync,
    S: Specification<K>,
{
    spec: S,
    _marker: PhantomData<K>,
}

impl<K, S> JsonSchema<K, S>
where
    K: 'static + Key + Send + Sync,
    S: Specification<K> + Send + Sync,
{
    /// Creates a new JSON Schema language for the given [`Specification`].
    pub fn new(spec: S) -> Self {
        Self {
            spec,
            _marker: PhantomData,
        }
    }
    pub fn spec(&self) -> &S {
        &self.spec
    }
    pub fn spec_mut(&mut self) -> &mut S {
        &mut self.spec
    }
}

impl<K, S> Language<K> for JsonSchema<K, S>
where
    K: 'static + Key + Send + Sync,
    S: Specification<K> + Send + 'static,
{
    /// The [`CompiledSchema`](schema::CompiledSchema) of this language.
    type CompiledSchema = CompiledSchema<S, K>;

    /// The error type possibly returned from [`compile`](Language::compile).
    type CompileError<R> = S::CompileError<R>
    where
    R: 'static + Resolve;

    /// The result type returned from [`evaluate`](Language::evaluate).
    type EvaluateResult<'rpt> =
        Result<Report<S::Annotation<'rpt>, S::Error<'rpt>>, S::EvaluateError>;

    /// Context type supplied to `evaluate`.
    type Context = Output;

    /// Initializes the language with the given [`Init`] request.
    fn init(&mut self, init: lang::Init<'_, Self::CompiledSchema, K>) {
        self.spec.init(init)
    }

    /// Compiles a schema or set of schemas for the given [`Compile`] request
    /// and returns an iterator of keys, if successful.
    ///
    /// This method is `async` in order to allow for fetching of schemas during
    /// compilation and if implementations need to perform io in order to setup.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    async fn compile<'int, R>(
        &'int mut self,
        ctx: lang::Compile<'int, Self, R, K>,
    ) -> Result<Vec<K>, Self::CompileError>
    where
        R: 'static + Resolve + Send + Sync,
    {
        compile::compile::<R, S, K>(self.spec.init_compile(ctx).await?).await
    }

    /// Evaluates a schema for the given [`Evaluate`] request.
    fn evaluate<'int, 'v>(
        &'int self,
        eval: lang::Evaluate<'int, 'v, Self::CompiledSchema, Self::Context, K>,
    ) -> Self::EvaluateResult<'v> {
        _ = eval;
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EvaluateError<K: Key> {
    X(K),
}

impl<K: Key + Send> fmt::Display for EvaluateError<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to evaluate value")
    }
}

impl<K: Key + Send> std::error::Error for EvaluateError<K> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EvaluateError::X(_) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct UriError {
    pub actul: String,
    pub source: grill_uri::Error,
}

impl fmt::Display for UriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse uri: \"{}\"", self.actul)
    }
}

impl std::error::Error for UriError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
