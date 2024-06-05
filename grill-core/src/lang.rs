//! Traits and resources for integrating a schema language.
//!
//! [`Interrogator`](crate::Interrogator) relies upon implementations of
//! [`Language`] to compile and evaluate schemas. This `mod` contains the traits
//! to satisfy that contract.
//!
//! ## What's provided
//! An [`Interrogator`](crate::Interrogator) contains a number of data
//! structures to facilitate implementing [`language`]:
//! - [`Schemas`] is a [`SlotMap`](`slotmap::SlotMap`)-backed graph of schemas.
//! - [`Sources`] is a repository of [`Arc<Value>`](`serde_json::Value`) indexed
//!   by [`AbsoluteUri`].
//! - [`Values`] is a cache of [`Arc<Value>`](`serde_json::Value`) indexed by
//!   [`Value`].
//! - [`Numbers`] is a cache of [`Arc<BigRational>`](num::BigRational) that will
//!   also parse [`serde_json::Number`]s.
//!
//! ## Compiling a schema

pub mod cache;
pub mod schema;
pub mod source;

pub use {
    cache::{Numbers, Values},
    schema::Schemas,
    source::Sources,
};

use crate::Resolve;
use grill_uri::AbsoluteUri;
use serde_json::Value;
use slotmap::Key;
use std::fmt::Debug;

/// A trait which defines how to compile and evaluate a schema against a
/// [`Value`].
///
/// See the [`mod`](crate::lang) for more information.
#[trait_variant::make(Send)]
pub trait Language<K>: Sized + Clone + Debug
where
    K: 'static + Key,
{
    /// The [`CompiledSchema`](schema::CompiledSchema) of this language.
    type CompiledSchema: schema::CompiledSchema<K>;

    /// The error type possibly returned from [`compile`](Language::compile).
    type CompileError: Send + std::error::Error;

    /// The result type returned from [`evaluate`](Language::evaluate).
    type EvaluateResult<'v>;

    /// Context type supplied to `evaluate`.
    ///
    /// For example, `grill-json-schema` uses an `enum` to represent the desired
    /// format of the output.
    type Context;

    /// The error type that can be returned when initializing the language.
    type InitError;

    /// Initializes the language with the given [`Init`] request.
    fn init(&mut self, init: Init<'_, Self::CompiledSchema, K>) -> Result<(), Self::InitError>;

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
        compile: Compile<'i, AbsoluteUri, Self::CompiledSchema, R, K>,
    ) -> Result<K, Self::CompileError>;

    /// Compiles all schemas for the given [`CompileAll`] request and returns the
    /// keys, if successful.
    async fn compile_all<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile_all: Compile<'i, Vec<AbsoluteUri>, Self::CompiledSchema, R, K>,
    ) -> Result<Vec<K>, Self::CompileError>;

    /// Evaluates a schema for the given [`Evaluae`] request.
    fn evaluate<'i, 'v>(
        &'i self,
        eval: Evaluate<'i, 'v, Self::CompiledSchema, Self::Context, K>,
    ) -> Self::EvaluateResult<'v>;
}

pub mod alias {
    use super::Language;
    /// Alias for [`Language::Context`].
    pub type Context<L, K> = <L as Language<K>>::Context;
    /// Alias for [`Language::CompiledSchema`].
    pub type CompiledSchema<L, K> = <L as Language<K>>::CompiledSchema;
    /// Alias for [`Language::CompiledSchema`].
    pub type Schema<'i, L, K> =
        <CompiledSchema<L, K> as super::schema::CompiledSchema<K>>::Schema<'i>;
    /// Alias for [`Language::InitError`].
    pub type InitError<L, K> = <L as Language<K>>::InitError;
    /// Alias for [`Language::CompileError`].
    pub type CompileError<L, K> = <L as Language<K>>::CompileError;
    /// Alias for [`Language::EvaluateResult`].
    pub type EvaluateResult<'v, L, K> = <L as Language<K>>::EvaluateResult<'v>;
}
/// Request to initialize a language.
pub struct Init<'i, S, K: Key> {
    /// Schema graph
    pub schemas: &'i mut Schemas<S, K>,
    /// Source repository
    pub sources: &'i mut Sources,
    /// Number cache
    pub numbers: &'i mut Numbers,
    /// Values cache
    pub values: &'i mut Values,
}

/// Request to compile a schema.
pub struct Compile<'i, U, S, R, K: Key> {
    /// Either the [`AbsoluteUri`] of the schema to `compile` in the case of
    /// `compile` or a `Vec<AbsoluteUri>` in the case of `compile_all`
    pub uri: U,
    /// Schema graph
    pub schemas: &'i mut Schemas<S, K>,
    /// Source repository
    pub sources: &'i mut Sources,
    /// Number cache
    pub numbers: &'i mut Numbers,
    /// Values cache
    pub values: &'i mut Values,
    /// Implementation of [`Resolve`]
    pub resolve: &'i R,
}

/// Request to evaluate a schema.
pub struct Evaluate<'i, 'v, S, X, K: Key> {
    /// Evaluation context `S::Context`
    pub context: X,
    /// The key of the schema to evaluate
    pub key: K,
    /// The value to evaluate
    pub value: &'v Value,
    /// Schema graph
    pub schemas: &'i Schemas<S, K>,
    /// Source repository
    pub sources: &'i Sources,
    /// Read-only access to global (to the `Interrogator`) cache of [`Numbers`]     
    pub numbers_cache: &'i Numbers,
    /// Read-only access to global (to the `Interrogator`) cache of [`Values`]
    pub values_cache: &'i Values,
    /// [`Numbers`] local to this evaluation
    pub numbers: &'i mut Numbers,
    /// [`Values`] local to this evaluation
    pub values: &'i mut Values,
}
