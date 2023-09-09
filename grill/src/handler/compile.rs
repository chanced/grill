use num::{BigInt, BigRational};
use serde_json::{Number, Value};

use crate::{
    error::{CompileError, NumberError},
    schema::Schemas,
    AbsoluteUri, Key, Uri,
};

use super::{BigInts, BigRationals, IntKey, Numbers, RationalKey, ValueKey, Values};

#[derive(Debug)]
pub struct Compile<'i> {
    pub(crate) base_uri: &'i AbsoluteUri,
    pub(crate) schemas: &'i Schemas,
    pub(crate) rationals: &'i mut BigRationals,
    pub(crate) ints: &'i mut BigInts,
    pub(crate) values: &'i mut Values,
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
