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
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::needless_pass_by_value
)]
#![cfg_attr(test, allow(clippy::redundant_clone, clippy::too_many_lines))]
#![recursion_limit = "256"]

pub mod compile;
pub mod dialect;
pub mod draft;
pub mod invalid_type;
pub mod keyword;
pub mod report;
pub mod schema;
pub mod spec;

use core::fmt;
use std::marker::PhantomData;

use compile::compile;
use grill_core::{lang::context, state::State, Key, Language, Resolve};
use schema::CompiledSchema;
use spec::Specification;
pub use spec::Standard;
pub use {
    compile::Error,
    report::{Output, Report},
};

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  IntoOwned                                   ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯                                  ║
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
pub struct JsonSchema<K, S>
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
    type EvaluateResult<'val> =
        Result<Report<S::Annotation<'val>, S::Error<'val>>, S::EvaluateError>;

    /// Context type supplied to `evaluate`.
    type Context = Output;

    /// Initializes the language with the given [`Init`] request.
    fn init(&mut self, init: &mut State<Self, K>) {
        self.spec.init(init);
    }

    /// Compiles a schema or set of schemas for the given [`Compile`] request
    /// and returns an iterator of keys, if successful.
    ///
    /// This method is `async` in order to allow for fetching of schemas during
    /// compilation and if implementations need to perform io in order to setup.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    async fn compile<'int, 'txn, 'res, R>(
        &'int mut self,
        ctx: context::Compile<'int, 'txn, 'res, Self, R, K>,
    ) -> Result<Vec<K>, Self::CompileError<R>>
    where
        R: 'static + Resolve + Send + Sync,
    {
        compile::<R, S, K>(self.spec.compile(ctx).await?).await
    }

    /// Evaluates a schema for the given [`Evaluate`] request.
    fn evaluate<'int, 'val>(
        &'int self,
        eval: context::Evaluate<'int, '_, 'val, Self, K>,
    ) -> Self::EvaluateResult<'val> {
        _ = eval;
        todo!()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                EvaluateError                                 ║
║                               ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
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
