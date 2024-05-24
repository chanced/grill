//! # grill-core
//!

#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::implicit_hasher, clippy::wildcard_imports)]
#![cfg_attr(test, allow(clippy::too_many_lines))]

pub mod big;
pub mod iter;
pub mod lang;

use async_trait::async_trait;
pub use lang::{schema::DefaultKey, Language};
pub use slotmap::{new_key_type, Key};

use grill_uri::AbsoluteUri;
use lang::{source, Compile, Evaluate, Numbers, Schemas, Sources, Values};
use serde_json::Value;
use snafu::{Backtrace, Snafu};

/// A trait for resolving and deserializing a [`Value`] at a given [`AbsoluteUri`].
#[async_trait]
pub trait Resolve {
    /// The error type that can be returned when resolving a [`Value`].
    type Error;

    /// Resolves and deserializes a [`Value`] at the supplied [`AbsoluteUri`].
    ///
    /// # Errors
    /// Returns [`Self::Error`] if an error occurs during resolution.
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Value, Self::Error>;
}

#[async_trait]
impl Resolve for () {
    type Error = source::NotFoundError;
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Value, Self::Error> {
        Err(source::NotFoundError::new(uri.clone()))
    }
}

/// Evaluates the integrity of data through a schema language.
pub struct Interrogator<L: Language<K>, R = (), K: 'static + Key = DefaultKey> {
    lang: L,
    schemas: Schemas<L::CompiledSchema, K>,
    sources: Sources,
    values: Values,
    numbers: Numbers,
    resolve: R,
}

impl<L, R, K> Interrogator<L, R, K>
where
    L: Language<K>,
    R: Resolve,
    K: Key,
{
    /// Creates a new `Interrogator` with the given language and `Resolve`
    pub fn new_with_resolve(lang: L, resolve: R) -> Self {
        Self {
            lang,
            schemas: Schemas::new(),
            sources: Sources::new(),
            values: Values::new(),
            numbers: Numbers::new(),
            resolve,
        }
    }
}

impl<L: Language<K>, R, K: Key> Interrogator<L, R, K> {
    fn init(mut self) -> Result<Self, L::InitError> {
        self.lang.init(lang::Init {
            schemas: &mut self.schemas,
            sources: &mut self.sources,
            numbers: &mut self.numbers,
            values: &mut self.values,
        })?;
        Ok(self)
    }
}

impl<L: Language<K>, K: Key> Interrogator<L, (), K> {
    /// Creates a new `Interrogator`.
    pub fn new(lang: L) -> Result<Self, L::InitError> {
        Self {
            lang,
            schemas: Schemas::new(),
            sources: Sources::new(),
            values: Values::new(),
            numbers: Numbers::new(),
            resolve: (),
        }
        .init()
    }
    /// Sets the [`Resolve`] implementation for the `Interrogator` using a
    /// builder fashion (consume and return).
    pub fn with_resolve<R: Resolve>(self, resolve: R) -> Interrogator<L, R, K> {
        Interrogator {
            lang: self.lang,
            schemas: self.schemas,
            sources: self.sources,
            values: self.values,
            numbers: self.numbers,
            resolve,
        }
    }
}

impl<L, R, K> Interrogator<L, R, K>
where
    L: Language<K>,
    R: Resolve + Send + Sync,
    K: 'static + Key,
{
    /// Compiles a schema for the given [`Compile`] request and returns the key,
    /// if successful.
    ///
    /// This method is `async` to allow for languages that need to fetch schemas
    /// during compilation.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    pub async fn compile(&mut self, schema_uri: AbsoluteUri) -> Result<K, L::CompileError> {
        let mut sources = self.sources.clone();
        let mut schemas = self.schemas.clone();
        let c = Compile {
            uri: schema_uri,
            schemas: &mut schemas,
            sources: &mut sources,
            numbers: &mut self.numbers,
            values: &mut self.values,
            resolve: &self.resolve,
        };

        let key = self.lang.compile(c).await?;
        self.schemas = schemas;
        self.sources = sources;
        Ok(key)
    }

    /// Evaluates a schema for the given [`Evaluate`] request.
    pub fn evaluate<'i, 'v>(
        &'i self,
        schema: K,
        context: L::Context,
        value: &'v Value,
    ) -> L::EvaluateResult<'v> {
        let mut numbers = Numbers::new();
        let mut values = Values::new();
        self.lang.evaluate(Evaluate {
            context,
            key: schema,
            value,
            schemas: &self.schemas,
            sources: &self.sources,
            numbers_cache: &self.numbers,
            values_cache: &self.values,
            numbers: &mut numbers,
            values: &mut values,
        })
    }
}
