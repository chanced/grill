//! # grill-core
//!

#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
// #![warn(missing_docs)]
#![allow(clippy::implicit_hasher, clippy::wildcard_imports)]
#![cfg_attr(test, allow(clippy::too_many_lines))]

pub mod big;
pub mod iter;
pub mod lang;

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

pub use lang::{schema::DefaultKey, Language};
pub use slotmap::{new_key_type, Key};

use grill_uri::AbsoluteUri;
use lang::{
    schema::{CompiledSchema, InvalidKeyError},
    source::{self, NotFoundError},
    Compile, Evaluate, Numbers, Schemas, Sources, Values,
};
use serde_json::Value;

/// A trait for resolving and deserializing a [`Value`] at a given [`AbsoluteUri`].
#[trait_variant::make(Send)]
pub trait Resolve {
    /// The error type that can be returned when resolving a [`Value`].
    type Error;

    /// Resolves and deserializes a [`Value`] at the supplied [`AbsoluteUri`].
    ///
    /// # Errors
    /// Returns [`Self::Error`] if an error occurs during resolution.
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error>;
}

macro_rules! resolve_maps {
    ($($map:ident),*) => {
        $(
            impl Resolve for $map<AbsoluteUri, Arc<Value>> {
                type Error = NotFoundError;
                async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error> {
                    self.get(uri)
                        .cloned()
                        .ok_or_else(|| NotFoundError::new(uri.clone()))
                }
            }
            impl Resolve for $map<AbsoluteUri, Value> {
                type Error = NotFoundError;
                async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error> {
                    self.get(uri)
                        .cloned()
                        .map(Arc::new)
                        .ok_or_else(|| NotFoundError::new(uri.clone()))
                }
            }
        )*
    };
}
resolve_maps!(HashMap, BTreeMap);

impl Resolve for () {
    type Error = source::NotFoundError;
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error> {
        Err(source::NotFoundError::new(uri.clone()))
    }
}

/// Type alias for `()` which implements [`Resolve`] by always returning
/// [`NotFoundError`], thus relying entirely on documents added as sources
/// to the [`Interrogator`].
pub type Internal = ();

/// Evaluates the integrity of data through a schema language.
pub struct Interrogator<L: Language<K>, K: 'static + Key + Send = DefaultKey> {
    lang: L,
    schemas: Schemas<L::CompiledSchema, K>,
    sources: Sources,
    values: Values,
    numbers: Numbers,
}

impl<L: Language<K>, K: Key + Send> Interrogator<L, K> {
    fn init(mut self) -> Result<Self, L::InitError> {
        self.lang.init(lang::Init {
            schemas: &mut self.schemas,
            sources: &mut self.sources,
            numbers: &mut self.numbers,
            values: &mut self.values,
        })?;
        Ok(self)
    }

    // fn iter(&self) -> (){
    //     self.schemas.
    // }
    /// Creates a new `Interrogator`.
    pub fn new(lang: L) -> Result<Self, L::InitError> {
        Self {
            lang,
            schemas: Schemas::new(),
            sources: Sources::new(),
            values: Values::new(),
            numbers: Numbers::new(),
        }
        .init()
    }

    /// Compiles a schema for the given [`Compile`] request and returns the key,
    /// if successful.
    ///
    /// This method is `async` to allow for languages that need to fetch schemas
    /// during compilation.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    pub async fn compile<R>(
        &mut self,
        schema_uri: AbsoluteUri,
        resolve: &R,
    ) -> Result<K, L::CompileError>
    where
        R: Resolve + Sync,
    {
        let mut sources = self.sources.clone();
        let mut schemas = self.schemas.clone();
        let c = Compile {
            uri: schema_uri,
            schemas: &mut schemas,
            sources: &mut sources,
            numbers: &mut self.numbers,
            values: &mut self.values,
            resolve,
        };
        let key = self.lang.compile(c).await?;
        self.schemas = schemas;
        self.sources = sources;
        Ok(key)
    }

    pub async fn compile_all<R, U>(
        &mut self,
        schema_uri: U,
        resolve: &R,
    ) -> Result<Vec<K>, L::CompileError>
    where
        R: Resolve + Sync,
        U: IntoIterator<Item = AbsoluteUri>,
    {
        let mut sources = self.sources.clone();
        let mut schemas = self.schemas.clone();
        let c = Compile {
            uri: schema_uri.into_iter().collect(),
            schemas: &mut schemas,
            sources: &mut sources,
            numbers: &mut self.numbers,
            values: &mut self.values,
            resolve,
        };
        let keys = self.lang.compile_all(c).await?;
        self.schemas = schemas;
        self.sources = sources;
        Ok(keys)
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
    pub fn schemas<T>(
        &self,
        keys: T,
    ) -> Result<Vec<lang::alias::Schema<'_, L, K>>, InvalidKeyError<K>>
    where
        T: IntoIterator<Item = K>,
    {
        todo!()
        // self.schemas.get_all(keys.into_iter())
    }
}
