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

pub mod compile;
pub mod error;
pub mod evaluate;
pub mod schema;
pub mod source;

pub use schema::Schemas;
pub use source::Sources;

use crate::{state::State, Resolve};
use slotmap::Key;
use std::{fmt, future::Future};

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Language                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait which defines how to compile and evaluate a schema against a
/// [`Value`].
///
/// See the [`mod`](crate::lang) for more information.
pub trait Language<K>: Sized + Clone + fmt::Debug
where
    K: 'static + Key + Send + Sync,
{
    /// The [`CompiledSchema`](schema::CompiledSchema) of this language.
    type CompiledSchema: schema::CompiledSchema<K>;

    /// The error type possibly returned from [`compile`](Language::compile).
    type CompileError<R>: 'static + Send + std::error::Error
    where
        R: 'static + Resolve;

    /// The result type returned from [`evaluate`](Language::evaluate).
    type EvaluateResult<'val>
    where
        Self: 'val;

    /// Context type supplied to `evaluate`.
    type Context;

    /// Initializes the language with the given [`Init`] request.
    fn init(&mut self, init: &mut State<Self, K>);

    /// Compiles schemas specified by the [`Compile`] request and returns an
    /// iterator of keys, if successful.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    fn compile<'int, 'txn, 'res, R>(
        &'int mut self,
        compile: compile::Context<'int, 'txn, 'res, Self, R, K>,
    ) -> impl Future<Output = Result<Vec<K>, Self::CompileError<R>>>
    where
        R: 'static + Resolve + Send + Sync;

    /// Evaluates a schema for the given [`Evaluae`] request.
    fn evaluate<'int, 'val>(
        &'int self,
        eval: evaluate::Context<'int, '_, 'val, Self, K>,
    ) -> Self::EvaluateResult<'val>;
}
