use std::sync::Arc;

use num_rational::BigRational;
use serde_json::{Number, Value};

use crate::{
    anymap::AnyMap,
    error::{CompileError, NumberError},
    schema::Schemas,
    AbsoluteUri, Key, Uri,
};

use super::{NumberCache, ValueCache};

#[derive(Debug)]
pub struct Compile<'i> {
    pub(crate) base_uri: &'i AbsoluteUri,
    pub(crate) schemas: &'i Schemas,
    pub(crate) numbers: &'i mut NumberCache,
    pub(crate) value_cache: &'i mut ValueCache,
    pub(crate) state: &'i mut AnyMap,
}

impl<'i> Compile<'i> {
    /// Parses a [`Number`] into a [`BigRational`], stores it and returns an
    /// `Arc` to it.
    ///
    /// # Errors
    /// Returns `NumberError` if the number fails to parse
    pub fn number(&mut self, num: &Number) -> Result<Arc<BigRational>, NumberError> {
        self.numbers.number(num)
    }
    /// Caches a [`Value`] and returns an `Arc` to it.
    pub fn value(&mut self, value: &Value) -> Arc<Value> {
        self.value_cache.value(value)
    }

    /// Returns an immutable reference to the global state [`AnyMap`].
    #[must_use]
    pub fn state(&self) -> &AnyMap {
        self.state
    }

    /// Returns a mutable reference to the global state [`AnyMap`].
    #[must_use]
    pub fn state_mut(&mut self) -> &mut AnyMap {
        self.state
    }

    /// Resolves a schema `Key` by URI
    ///
    /// # Errors
    /// - `CompileError::SchemaNotFound` if the schema is not found
    /// - `CompileError::UriParsingFailed` if the URI is invalid
    pub fn schema(&self, uri: &str) -> Result<Key, CompileError> {
        let uri: Uri = uri.parse()?;
        let uri = self.base_uri.resolve(&uri)?;
        self.schemas
            .get_key_by_id(&uri)
            .ok_or(CompileError::SchemaNotFound(uri))
    }
}
