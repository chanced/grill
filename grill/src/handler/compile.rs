use num::{BigInt, BigRational};
use serde_json::{Number, Value};

use crate::{
    error::{CompileError, NumberError},
    schema::Schemas,
    AbsoluteUri, Key, Uri,
};

use super::{IntKey, Numbers, RationalKey, ValueKey, Values};

#[derive(Debug)]
pub struct Compile<'i> {
    base_uri: &'i AbsoluteUri,
    schemas: &'i Schemas,

    rationals: &'i mut Numbers<RationalKey, BigRational>,
    ints: &'i mut Numbers<IntKey, BigInt>,
    values: &'i mut Values,
}

impl<'i> Compile<'i> {
    /// Parses a [`Number`] into a [`BigRational`], stores it and returns the
    /// [`RationalKey`].
    ///
    /// # Errors
    /// Returns `NumberError` if the number fails to parse
    pub fn rational(&mut self, value: &Number) -> Result<RationalKey, NumberError> {
        self.rationals.insert(value)
    }
    /// Parses a [`Number`] into a [`BigInt`], stores it and returns the
    /// [`IntKey`].
    ///
    /// # Errors
    /// Returns `NumberError` if the number fails to parse
    pub fn int(&mut self, num: &Number) -> Result<IntKey, NumberError> {
        self.ints.insert(num)
    }
    /// Stores a [`Value`] and returns the [`ValueKey`].
    pub fn value(&mut self, value: &Value) -> ValueKey {
        self.values.insert(value)
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
