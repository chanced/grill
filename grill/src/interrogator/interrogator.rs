use std::{borrow::Borrow, fmt::Debug, ops::Deref};

use num::BigInt;
use num_rational::BigRational;
use serde_json::Value;
use tap::TapFallible;

use crate::{
    error::{
        CompileError, DeserializeError, DialectUnknownError, EvaluateError, SourceError,
        UnknownKeyError,
    },
    json_schema, keyword,
    keyword::{IntKey, Numbers, RationalKey, Values},
    output::{Output, Structure},
    schema::{
        iter::{Iter, IterUnchecked},
        traverse::{
            Ancestors, Descendants, DirectDependencies, DirectDependents, TransitiveDependencies,
        },
        Compiler, Dialect, Dialects, Key, Schema, Schemas,
    },
    source::{Deserializers, Resolvers, Sources, Src},
    uri::{AbsoluteUri, TryIntoAbsoluteUri},
    Builder,
};

use super::state::State;

/// Compiles and evaluates JSON Schemas.
#[derive(Clone)]
pub struct Interrogator {
    pub(crate) dialects: Dialects<'static>,
    pub(crate) sources: Sources,
    pub(crate) resolvers: Resolvers,
    pub(crate) schemas: Schemas,
    pub(crate) deserializers: Deserializers,
    pub(crate) rationals: Numbers<RationalKey, BigRational>,
    pub(crate) ints: Numbers<IntKey, BigInt>,
    pub(crate) values: Values,
    pub(crate) state: State,
}

impl Debug for Interrogator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interrogator")
            .field("dialects", &self.dialects)
            .field("sources", &self.sources)
            .field("schemas", &self.schemas)
            .field("deserializers", &self.deserializers)
            .finish_non_exhaustive()
    }
}

impl Interrogator {
    /// Returns the [`Schema`] with the given `key` if it exists.
    ///
    /// # Errors
    /// Returns [`UnknownKeyError`] if the `key` does not belong to this `Interrgator`.
    pub fn schema(&self, key: Key) -> Result<Schema<'_>, UnknownKeyError> {
        self.schemas.get(key, &self.sources)
    }

    #[must_use]
    /// Returns the [`Schema`] with the given `key` if it exists.
    ///
    /// # Panics
    /// Panics if the `key` does not belong to this `Interrgator`.
    pub fn schema_unchecked(&self, key: Key) -> Schema<'_> {
        self.schemas.get_unchecked(key, &self.sources)
    }

    /// Returns the [`Schema`] with the given `id` if it exists.
    #[must_use]
    pub fn schema_by_id(&self, id: &AbsoluteUri) -> Option<Schema<'_>> {
        self.schemas.get_by_uri(id, &self.sources)
    }

    /// Returns `true` if `key` belongs to this `Interrogator`
    #[must_use]
    pub fn contains_key(&self, key: Key) -> bool {
        self.schemas.contains_key(key)
    }

    /// Returns [`Ancestors`] which is an [`Iterator`] over the descendants,
    /// i.e. embedded schemas, of a given [`Schema`].
    ///
    ///
    /// Note that the JSON Schema specification states that if a schema is
    /// identified (by having either an `$id` field for Draft 07 and beyond or
    /// an `id` field for Draft 04 and earlier), then it must be the document
    /// root. As such, embedded schemas with an id will not have a parent, even
    /// if the [`Schema`] is embedded.
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn ancestors(&self, key: Key) -> Result<Ancestors<'_>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.ancestors(key, &self.sources))
    }
    /// Returns [`Ancestors`] which is an [`Iterator`] over the descendants,
    /// i.e. embedded schemas, of a given [`Schema`].
    ///
    ///
    /// Note that the JSON Schema specification states that if a schema is
    /// identified (by having either an `$id` field for Draft 07 and beyond or
    /// an `id` field for Draft 04 and earlier), then it must be the document
    /// root. As such, embedded schemas with an id will not have a parent, even
    /// if the [`Schema`] is embedded.
    ///
    /// # Panics
    /// Panics if `key` does not belong to this `Interrogator`
    #[must_use]
    pub fn ancestors_unchecked(&self, key: Key) -> Ancestors<'_> {
        self.schemas.ancestors(key, &self.sources)
    }

    /// Returns [`Descendants`] which is an [`Iterator`] over the hiearchy of a
    /// given [`Schema`].
    ///
    /// Note that the JSON Schema specification states that if a schema is
    /// identified (by having either an `$id` field for Draft 07 and beyond or
    /// an `id` field for Draft 04 and earlier), then it must be the document
    /// root. As such, embedded schemas with an id  will not have a parent, even
    /// if the [`Schema`] is embedded.
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn descendants(&self, key: Key) -> Result<Descendants<'_>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.descendants(key, &self.sources))
    }

    /// Returns [`Descendants`] which is an [`Iterator`] over the hiearchy of a
    /// given [`Schema`].
    ///
    /// Note that the JSON Schema specification states that if a schema is
    /// identified (by having either an `$id` field for Draft 07 and beyond or
    /// an `id` field for Draft 04 and earlier), then it must be the document
    /// root. As such, embedded schemas with an id  will not have a parent, even
    /// if the [`Schema`] is embedded.
    ///
    /// # Panics
    /// Panics if `key` does not belong to this `Interrogator`
    pub fn descendants_unchecked(&self, key: Key) -> Result<Descendants<'_>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.descendants(key, &self.sources))
    }

    /// Returns [`DirectDependencies`] which is an [`Iterator`] over the direct
    /// dependencies of a [`Schema`]
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn direct_dependencies(&self, key: Key) -> Result<DirectDependencies<'_>, UnknownKeyError> {
        self.schemas
            .ensure_key_exists(key, || self.schemas.direct_dependencies(key, &self.sources))
    }

    /// Returns [`DirectDependencies`] which is an [`Iterator`] over the direct
    /// dependencies of a [`Schema`]
    ///
    /// # Panics
    /// Panics if `key` does not belong to this `Interrogator`
    #[must_use]
    pub fn direct_dependencies_unchecked(&self, key: Key) -> DirectDependencies<'_> {
        self.schemas.direct_dependencies(key, &self.sources)
    }

    /// Returns [`TransitiveDependencies`] which is a
    /// [depth-first](https://en.wikipedia.org/wiki/Depth-first_search)
    /// [`Iterator`] that traverses both direct and indirect dependencies of a
    /// [`Schema`].
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn transitive_dependencies(
        &self,
        key: Key,
    ) -> Result<TransitiveDependencies<'_>, UnknownKeyError> {
        self.ensure_key_exists(key, || {
            self.schemas.transitive_dependencies(key, &self.sources)
        })
    }

    /// Returns [`TransitiveDependencies`] which is a
    /// [depth-first](https://en.wikipedia.org/wiki/Depth-first_search)
    /// [`Iterator`] that traverses both direct and indirect dependencies of a
    /// [`Schema`].
    ///
    /// # Panics
    /// Panics if `key` does not belong to this `Interrogator`
    #[must_use]
    pub fn transitive_dependencies_unchecked(&self, key: Key) -> TransitiveDependencies<'_> {
        self.schemas.transitive_dependencies(key, &self.sources)
    }

    /// Return [`Schema`](crate::schema::Schema)s which is an [`Iterator`] over
    /// [`Schema`]s which directly depend on a specified
    /// [`Schema`](crate::schema::Schema)
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn direct_dependents(&self, key: Key) -> Result<DirectDependents<'_>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.direct_dependents(key, &self.sources))
    }

    /// Return [`Schema`](crate::schema::Schema)s which is an [`Iterator`] over
    /// [`Schema`]s which directly depend on a specified
    /// [`Schema`](crate::schema::Schema)
    ///
    /// # Panics
    /// Panics if `key` does not belong to this `Interrogator`
    pub fn direct_dependents_unchecked(
        &self,
        key: Key,
    ) -> Result<DirectDependents<'_>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.direct_dependents(key, &self.sources))
    }

    /// A helper method that returns `UnknownKeyError` if `key` does not belong
    /// to this `Interrogator` and executes `f` if it does.
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn ensure_key_exists<T, F>(&self, key: Key, f: F) -> Result<T, UnknownKeyError>
    where
        F: FnOnce() -> T,
    {
        self.schemas.ensure_key_exists(key, f)
    }

    /// Compiles all schemas at the given URIs if not already compiled, returning
    /// a [`Vec`] of either the freshly or previously compiled [`Schema`]s
    ///
    /// # Errors
    /// Returns [`CompileError`] if any of the schemas fail to compile.
    ///
    /// # Example
    /// ```rust
    /// use grill::{ Interrogator };
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut interrogator = Interrogator::json_schema_2020_12().build().unwrap();
    ///     interrogator.source_str("https://example.com/string.json", r#"{"type": "string"}"#).unwrap();
    ///     interrogator.source_str("https://example.com/number.json", r#"{"type": "number"}"#).unwrap();
    ///     let schemas = interrogator.compile_all(vec![
    ///        "https://example.com/string.json",
    ///        "https://example.com/number.json",
    ///     ]).unwrap();
    ///     assert_eq!(schemas.len(), 2);
    /// }
    /// ```
    ///
    #[allow(clippy::unused_async)]
    pub async fn compile_all<I>(&mut self, uris: I) -> Result<Vec<(AbsoluteUri, Key)>, CompileError>
    where
        I: IntoIterator,
        I::Item: TryIntoAbsoluteUri,
    {
        let uris = uris.into_iter();
        let (lower, upper) = uris.size_hint();
        let mut keys = Vec::with_capacity(upper.unwrap_or(lower));
        self.start_txn();
        for uri in uris {
            let uri = uri
                .try_into_absolute_uri()
                .tap_err(|_| self.rollback_txn())?;
            let key = self
                .compile_schema(uri.clone())
                .await
                .tap_err(|_| self.rollback_txn())?;
            keys.push((uri, key));
        }
        self.commit_txn();
        Ok(keys)
    }

    /// Attempts to compile the schema at the given URI if not already compiled,
    /// returning the freshly or previously compiled [`Schema`].
    ///
    /// # Errors
    /// Returns [`CompileError`] if:
    ///   - the schema fails to compile.
    ///   - a dependent schema fails to compile.
    ///   - the uri fails to convert to an [`AbsoluteUri`].
    ///   - the schema fails to validate with the determined [`Dialect`]'s metaschema
    #[allow(clippy::unused_async)]
    pub async fn compile(&mut self, uri: impl TryIntoAbsoluteUri) -> Result<Key, CompileError> {
        // TODO: use the txn method once async closures are available: https://github.com/rust-lang/rust/issues/62290
        self.start_txn();
        let uri = uri.try_into_absolute_uri()?;
        match self.compile_schema(uri).await {
            Ok(key) => {
                self.commit_txn();
                Ok(key)
            }
            Err(err) => {
                self.rollback_txn();
                Err(err)
            }
        }
    }

    #[must_use]
    pub fn iter<'i>(&'i self, keys: &'i [Key]) -> Iter<'i> {
        Iter::new(keys, &self.schemas, &self.sources)
    }

    #[must_use]
    pub fn iter_unchecked<'i>(&'i self, keys: &'i [Key]) -> IterUnchecked<'i> {
        self.iter(keys).unchecked()
    }

    async fn compile_schema(&mut self, uri: AbsoluteUri) -> Result<Key, CompileError> {
        Compiler::new(self).compile(uri).await
    }

    #[must_use]
    pub fn dialects(&self) -> Dialects<'_> {
        self.dialects.as_borrowed()
    }

    /// Returns the default [`Dialect`] for the `Interrogator`.
    #[must_use]
    pub fn default_dialect(&self) -> &Dialect {
        self.dialects.primary()
    }

    /// Returns the [`Dialect`] for the given schema, if any.
    pub fn determine_dialect(
        &self,
        schema: &Value,
    ) -> Result<Option<&Dialect>, DialectUnknownError> {
        if let Some(schema) = self.dialects.pertinent_to(schema) {
            return Ok(Some(schema));
        }
        // TODO: this is the only place outside of a Keyword that a specific
        // json schema keyword is used. This should be refactored.
        match schema
            .get(keyword::SCHEMA)
            .and_then(Value::as_str)
            .map(ToString::to_string)
        {
            Some(metaschema_id) => Err(DialectUnknownError { metaschema_id }),
            None => Ok(None),
        }
    }

    pub fn evaluate(
        &self,
        _key: Key,
        _value: &Value,
        _structure: Structure,
    ) -> Result<Output, EvaluateError> {
        todo!()
    }

    /// Returns the schema's `Key` if it exists
    #[must_use]
    pub fn schema_key_by_id(&self, id: &AbsoluteUri) -> Option<Key> {
        self.schemas.get_by_uri(id, &self.sources)?.key.into()
    }
    /// Returns the attached `Deserializers`.
    #[must_use]
    pub fn deserializers(&self) -> &Deserializers {
        &self.deserializers
    }

    /// Attempts to deserialize the given string into a [`Value`] using
    /// available [`Deserializer`]s.
    pub fn deserialize(&self, data: &str) -> Result<Value, DeserializeError> {
        self.deserializers.deserialize(data)
    }

    /// Adds a schema source from a slice of bytes that will be deserialized with
    /// avaialble [`Deserializer`] at the time of [`build`](`Builder::build`).
    ///
    /// # Example
    /// ```rust
    /// use grill::Interrogator;
    /// let mut interrogator = Interrogator::json_schema_2020_12().build().unwrap();
    /// let source = br#"{"type": "string"}"#;
    /// interrogator.source_slice("https://example.com/schema.json", source).unwrap();
    /// ```
    /// # Errors
    /// Returns [`SourceSliceError`] if:
    /// - the `uri` fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    pub fn source_slice(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &[u8],
    ) -> Result<&Value, SourceError> {
        let source = Src::String(
            uri.try_into_absolute_uri()?,
            String::from_utf8(source.to_vec())?,
        );

        self.source(source)
    }
    fn source(&mut self, source: Src) -> Result<&Value, SourceError> {
        // self.sources.insert(source, &self.deserializers)
        todo!()
    }

    /// Adds a schema source from a `&str`
    /// # Example
    /// ```rust
    /// let mut interrogator = grill::Interrogator::json_schema_2020_12().build().unwrap();
    /// interrogator.source_str("https://example.com/schema.json", r#"{"type": "string"}"#).unwrap();
    /// ```
    /// # Errors
    /// Returns [`UriError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    pub fn source_str(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &str,
    ) -> Result<&Value, SourceError> {
        self.source(Src::String(
            uri.try_into_absolute_uri()?,
            source.to_string(),
        ))
    }

    /// Adds a source schema from a [`Value`]
    /// # Example
    /// ```rust
    /// use grill::Interrogator;
    /// use serde_json::json;
    ///
    /// let mut interrogator = Interrogator::json_schema().build().unwrap();
    /// let source = json!({"type": "string"});
    /// interrogator.source_value("https://example.com/schema.json", source).unwrap();
    /// ```
    /// # Errors
    /// Returns [`UriError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    ///
    pub fn source_value(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
        source: impl Borrow<Value>,
    ) -> Result<&Value, SourceError> {
        self.source(Src::Value(
            uri.try_into_absolute_uri()?,
            source.borrow().clone(),
        ))
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, Deref<Target=str>)`
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use grill::Interrogator;
    ///
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", r#"{"type": "string"}"#);
    /// let mut interrogator = Interrogator::json_schema().build().unwrap();
    /// interrogator.source_strs(sources).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`UriError`] if a URI fails to convert to an
    /// [`AbsoluteUri`]
    pub fn source_strs<I, K, V>(&mut self, sources: I) -> Result<(), SourceError>
    where
        K: TryIntoAbsoluteUri,
        V: Deref<Target = str>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.source(Src::String(k.try_into_absolute_uri()?, v.to_string()))?;
        }
        Ok(())
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, AsRef<[u8]>)`
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use grill::Interrogator;
    ///
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", br#"{"type": "string"}"#);
    /// let mut interrogator = Interrogator::json_schema().build().unwrap();
    /// interrogator.source_slices(sources).unwrap();
    /// ```
    /// # Errors
    /// Returns [`SourceSliceError`] if:
    /// - an Absolute URI fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    ///
    pub fn source_slices<I, K, V>(&mut self, sources: I) -> Result<(), SourceError>
    where
        K: TryIntoAbsoluteUri,
        V: AsRef<[u8]>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.source(Src::String(
                k.try_into_absolute_uri()?,
                String::from_utf8(v.as_ref().to_vec())?,
            ))?;
        }
        Ok(())
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, Borrow<serde_json::Value>>)`
    ///
    /// # Example
    /// ```
    /// use grill::Interrogator;
    /// use std::collections::HashMap;
    /// use serde_json::json;
    ///
    /// let mut sources = HashMap::new();
    /// let source = json!({"type": "string"});
    /// sources.insert("https://example.com/schema.json", source);
    /// let mut interrogator = Interrogator::json_schema().build().unwrap();
    /// interrogator.source_values(sources).unwrap();
    /// ```
    /// # Errors
    /// Returns [`SourceSliceError`] if:
    /// - an Absolute URI fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    ///
    pub fn source_values<I, K, V>(&mut self, sources: I) -> Result<(), SourceError>
    where
        K: TryIntoAbsoluteUri,
        V: Borrow<Value>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.source(Src::Value(k.try_into_absolute_uri()?, v.borrow().clone()))?;
        }
        Ok(())
    }

    /// Returns a new, empty [`Builder`].
    #[must_use]
    #[allow(unused_must_use)]
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 2020-12 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_2020_12() -> Builder {
        Builder::default()
            .json_schema_2020_12()
            .default_dialect(json_schema::draft_2020_12::dialect())
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 2019-09 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_2019_09() -> Builder {
        // Builder::default()
        //     .with_json_schema_2019_09()
        //     .with_default_dialect(
        //         json_schema::draft_2019_09::JSON_SCHEMA_2019_09_ABSOLUTE_URI.clone(),
        //     )
        todo!()
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 07 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_07() -> Builder {
        // Builder::default()
        //     .with_json_schema_07()
        //     .with_default_dialect(json_schema::draft_07::JSON_SCHEMA_07_ABSOLUTE_URI.clone())
        //     .unwrap()
        todo!()
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 04 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_04() -> Builder {
        // Builder::default()
        //     .with_json_schema_04()
        //     .with_default_dialect(json_schema::draft_04::JSON_SCHEMA_04_ABSOLUTE_URI.clone())
        //     .unwrap()
        todo!()
    }
    /// Starts a new transaction.
    fn start_txn(&mut self) {
        self.schemas.start_txn();
        self.sources.start_txn();
    }

    /// Acccepts the current transaction, committing all changes.
    fn commit_txn(&mut self) {
        self.schemas.commit_txn();
        self.sources.commit_txn();
    }

    /// Rejects the current transaction, discarding all changes.
    pub(crate) fn rollback_txn(&mut self) {
        self.schemas.rollback_txn();
        self.sources.rollback_txn();
    }

    // /// requires <https://github.com/rust-lang/rust/issues/62290> be made stable.
    // fn txn<F, O, E>(&mut self, f: F) -> Result<O, E>
    // where
    //     F: FnOnce(&mut Self) -> Result<O, E>,
    // {
    //     self.start_txn();
    //     let result = f(self);
    //     match result {
    //         Ok(res) => {
    //             self.commit_txn();
    //             Ok(res)
    //         }
    //         Err(err) => {
    //             self.rollback_txn();
    //             Err(err)
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    #[tokio::test]
    async fn test_build() {
        let interrogator = Builder::default()
            .json_schema_2020_12()
            .source_str("https://example.com/schema.json", r#"{"type": "string"}"#)
            .unwrap()
            .build()
            .await
            .unwrap();

        let mut file = File::create("foo.txt").unwrap();
        file.write_all(format!("{interrogator:#?}").as_bytes())
            .unwrap();
    }
}
