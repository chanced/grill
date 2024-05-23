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

use async_trait::async_trait;
use grill_uri::AbsoluteUri;
use serde_json::Value;
use slotmap::Key;
use std::fmt::Debug;

/// A trait which defines how to compile and evaluate a schema against a
/// [`Value`].
///
/// See the [`mod`](crate::lang) for more information.
#[async_trait]
pub trait Language<K>: Sized + Clone + Debug
where
    K: Key,
{
    /// The [`CompiledSchema`](schema::CompiledSchema) of this language.
    type CompiledSchema: schema::CompiledSchema<K>;
    /// The error type that can be returned when compiling a schema.
    type CompileError: std::error::Error;
    /// The result type that can be returned when evaluating a schema.
    type EvaluateResult;

    /// Context type supplied to `evaluate`.
    ///
    /// For example, `grill-json-schema` uses an `enum` to represent the desired
    /// format of the output.
    type Context;

    /// Compiles a schema for the given [`Compile`] request and returns the key,
    /// if successful.
    ///
    /// This method is `async` to allow for languages that need to fetch schemas
    /// during compilation.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    async fn compile(
        &mut self,
        compile: Compile<Self::CompiledSchema, K>,
    ) -> Result<K, Self::CompileError>;

    /// Evaluates a schema for the given [`Evaluate`] request.
    fn evaluate<'i, 'v>(
        &'i self,
        eval: Evaluate<'i, 'v, Self::CompiledSchema, Self::Context, K>,
    ) -> Self::EvaluateResult;
}

/// Request to compile a schema.
pub struct Compile<'i, S, K: Key> {
    /// The URI of the schema to compile
    pub uri: AbsoluteUri,
    /// Schema graph
    pub schemas: &'i mut Schemas<S, K>,
    /// Source repository
    pub sources: &'i mut Sources,
    /// Number cache
    pub numbers: &'i mut Numbers,
    /// Values cache
    pub values: &'i mut Values,
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
