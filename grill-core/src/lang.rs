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

pub mod error;
pub mod schema;
pub mod source;

pub use schema::Schemas;
use serde_json::Value;
pub use source::Sources;

use crate::{
    cache::Cache,
    state::{State, Transaction},
    Resolve,
};
use grill_uri::AbsoluteUri;
use slotmap::Key;
use std::fmt;

/// A trait which defines how to compile and evaluate a schema against a
/// [`Value`].
///
/// See the [`mod`](crate::lang) for more information.
#[trait_variant::make(Send)]
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
    type EvaluateResult<'v>;

    /// Context type supplied to `evaluate`.
    type Context;

    /// Initializes the language with the given [`Init`] request.
    fn init(&mut self, init: &mut State<Self, K>);

    /// Compiles schemas specified by the [`Compile`] request and returns an
    /// iterator of keys, if successful.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    async fn compile<'x, 'int, 'res, R>(
        &'int mut self,
        compile: Compile<'x, 'int, 'res, Self, R, K>,
    ) -> Result<Vec<K>, Self::CompileError<R>>
    where
        R: 'static + Resolve + Send + Sync;

    /// Evaluates a schema for the given [`Evaluae`] request.
    fn evaluate<'int, 'v>(
        &'int self,
        eval: Evaluate<'int, 'v, Self, K>,
    ) -> Self::EvaluateResult<'v>;
}

pub mod alias {
    use super::Language;
    /// Alias for [`Language::Context`].
    pub type Context<L, K> = <L as Language<K>>::Context;

    /// Alias for [`Language::CompiledSchema`].
    pub type CompiledSchema<L, K> = <L as Language<K>>::CompiledSchema;

    /// Alias for [`CompiledSchema::Schema`]
    /// [CompiledSchema::Schema]: super::schema::CompiledSchema::Schema.
    pub type Schema<'int, L, K> =
        <CompiledSchema<L, K> as super::schema::CompiledSchema<K>>::Schema<'int>;

    /// Alias for [`Language::CompileError`].
    pub type CompileError<L, K, R> = <L as Language<K>>::CompileError<R>;

    /// Alias for [`Language::EvaluateResult`].
    pub type EvaluateResult<'v, L, K> = <L as Language<K>>::EvaluateResult<'v>;
}

/// Request to compile a schema.
pub struct Compile<'txn, 'int, 'res, L, R, K>
where
    L: Language<K>,
    K: 'static + Key + Send + Sync,
{
    /// Uris to compile
    pub uris: Vec<AbsoluteUri>,

    /// Current state of the [`Interrogator`], including schemas, sources, and
    /// cache. Upon successful compilation, the data will become to new state.
    pub txn: Transaction<'txn, 'int, L, K>,

    /// Implementation of [`Resolve`]
    pub resolve: &'res R,

    /// Whether or not to validate the schemas during compilation
    pub validate: bool,
}

impl<'x, 'int, 'res, L, R, K> Compile<'x, 'int, 'res, L, R, K>
where
    L: Language<K>,
    K: 'static + Key + Send + Sync,
{
    pub(crate) fn new(
        uris: Vec<AbsoluteUri>,
        transaction: Transaction<'x, 'int, L, K>,
        resolve: &'res R,
        validate: bool,
    ) -> Self
    where
        L: Language<K>,
        K: 'static + Key + Send + Sync,
        R: 'static + Resolve + Send + Sync,
    {
        Self {
            uris,
            txn: transaction,
            resolve,
            validate,
        }
    }
}

/// Request to evaluate a schema.
pub struct Evaluate<'int, 'v, L, K>
where
    L: Language<K>,
    K: 'static + Key + Send + Sync,
{
    /// Evaluation context `S::Context`
    pub context: L::Context,

    /// The current, immutable state of the [`Interrogator`]
    pub state: &'int State<L, K>,

    pub eval: &'int mut Cache,

    /// The key of the schema to evaluate
    pub key: K,

    /// The value to evaluate
    pub value: &'v Value,
}
