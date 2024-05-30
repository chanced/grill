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

use std::fmt::Display;

use grill_core::{lang::Init, Key, Language, Resolve};
use keyword::{eval, spec::Spec};
use report::{Annotation, Error, IntoOwned};
use schema::CompiledSchema;
use serde::{de::DeserializeOwned, Serialize};

pub use {
    compile::CompileError,
    report::{Output, Report},
};
/// A trait implemented by types which are capable of evaluating a specification
/// of JSON Schema.
pub trait Specification<K: Key> {
    /// The error type that can be returned when initializing the dialect.
    type InitError;

    /// The error type that can be returned when compiling a schema.
    type CompileError: for<'v> From<CompileError<<Self::Error<'v> as IntoOwned>::Owned>>;

    type EvaluateError: for<'v> From<EvaluateError<K>>;

    /// The context type supplied to `evaluate`.
    type Evaluate: keyword::Evaluate<K>;

    type Compile: keyword::Compile<K>;

    type Keyword: keyword::Keyword<Self, K>;

    /// The annotation type to be used in [`Report`s](report::Report).
    ///
    /// Even if an annotation is not used for a keyword, it is helpful to have
    /// unit struct as an annotation for analysis pre-serialization.
    ///
    ///[`ShouldSerialize`] is used by the `Report` to determine which annotations
    /// should be serialized.
    type Annotation<'v>: From<Annotation<'v>> + Serialize + ShouldSerialize + DeserializeOwned;

    /// The error type to be used in [`Report`s](report::Report).
    type Error<'v>: From<Error<'v>> + IntoOwned + Display;

    /// Initializes the specification.
    fn init(
        &mut self,
        init: Init<'_, CompiledSchema<Self::Keyword, K>, K>,
    ) -> Result<(), Self::InitError> {
        Ok(())
    }

    async fn compile<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile: grill_core::lang::Compile<'i, CompiledSchema<Self::Keyword, K>, R, K>,
    ) -> Result<Self::Compile, Self::CompileError>;

    fn evaluate<'i, 'v>(
        &'i self,
        eval: grill_core::lang::Evaluate<'i, 'v, CompiledSchema<Self::Keyword, K>, Output, K>,
    ) -> Result<Report<Self::Annotation<'v>, Self::Error<'v>>, Self::EvaluateError>;
}

pub(crate) mod alias {
    use super::{Report, Specification};
    pub(super) type InitError<S, K> = <S as Specification<K>>::InitError;
    pub(super) type CompileError<S, K> = <S as Specification<K>>::CompileError;
    pub(super) type EvaluateError<S, K> = <S as Specification<K>>::EvaluateError;
    pub(super) type Evaluate<S, K> = <S as Specification<K>>::Evaluate;
    pub(super) type Compile<S, K> = <S as Specification<K>>::Compile;
    pub(super) type Annotation<'v, S, K> = <S as Specification<K>>::Annotation<'v>;
    pub(super) type Error<'v, S, K> = <S as Specification<K>>::Error<'v>;
    pub(super) type TypedReport<'v, S, K> = Report<Annotation<'v, S, K>, Error<'v, S, K>>;
    pub(super) type EvaluateResult<'v, S, K> = Result<TypedReport<'v, S, K>, EvaluateError<S, K>>;
}

pub trait ShouldSerialize {
    fn should_serialize(&self) -> bool;
}

pub struct JsonSchema {}

impl<K: Key + Send> Specification<K> for JsonSchema {
    type InitError = ();

    type CompileError = CompileError<Error<'static>>;

    type EvaluateError = EvaluateError<K>;

    type Evaluate = eval::Context;

    type Compile = keyword::compile::Context<Spec<Self, K>, K>;

    type Keyword = keyword::spec::Spec<Self, K>;

    type Annotation<'v> = report::Annotation<'v>;

    type Error<'v> = report::Error<'v>;

    async fn compile<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile: grill_core::lang::Compile<'i, CompiledSchema<Self::Keyword, K>, R, K>,
    ) -> Result<Self::Compile, Self::CompileError> {
        todo!()
    }

    fn evaluate<'i, 'v>(
        &'i self,
        eval: grill_core::lang::Evaluate<'i, 'v, CompiledSchema<Self::Keyword, K>, Output, K>,
    ) -> Result<Report<Self::Annotation<'v>, Self::Error<'v>>, Self::EvaluateError> {
        todo!()
    }
}

impl<S, K> Language<K> for S
where
    S: Specification<K> + Send,
    K: Key + Send,
{
    /// The [`CompiledSchema`](schema::CompiledSchema) of this language.
    type CompiledSchema = CompiledSchema<K, S::Keyword>;

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
