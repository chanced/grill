use std::{any, borrow::Borrow, fmt::Debug, ops::Deref, str::FromStr};

use jsonptr::{Pointer, Resolve as _};
use serde_json::Value;

use crate::{
    error::{
        CompileError, DeserializeError, DialectUnknownError, EvaluateError, ResolveError,
        SourceConflictError, SourceError,
    },
    json_schema,
    output::{Output, Structure},
    schema::{CompiledSchema, Dialect, Dialects, Keyword, Schema, SchemaKey, Schemas},
    source::Resolvers,
    source::Sources,
    source::{Deserializer, Deserializers, Resolve, Source},
    uri::{AbsoluteUri, TryIntoAbsoluteUri},
    Builder,
};

/// Compiles and evaluates JSON Schemas.
#[derive(Clone)]
pub struct Interrogator<Key: slotmap::Key = SchemaKey> {
    dialects: Dialects<'static>,
    sources: Sources,
    resolvers: Resolvers,
    schemas: Schemas<Key>,
    deserializers: Deserializers,
}

impl<Key: slotmap::Key> Debug for Interrogator<Key> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interrogator")
            .field("dialects", &self.dialects)
            .field("sources", &self.sources)
            .field("schemas", &self.schemas)
            .field("deserializers", &self.deserializers)
            .finish_non_exhaustive()
    }
}

impl<Key> Interrogator<Key>
where
    Key: slotmap::Key,
{
    /// Attempts to compile the schema at the given URI if not already compiled,
    /// returning the freshly or previously compiled [`Schema`].
    ///
    /// # Errors
    /// Returns [`CompileError`] if:
    ///   - the schema fails to compile.
    ///   - a dependent schema fails to compile.
    ///   - the uri fails to convert to an [`AbsoluteUri`].
    ///   - the schema fails to validate with the determined [`Dialect`]'s metaschema
    pub async fn compile(&mut self, uri: impl TryIntoAbsoluteUri) -> Result<Key, CompileError> {
        self.schemas.start_txn();

        // convert the uri to an absolute uri
        let uri = uri.try_into_absolute_uri()?;
        // resolving the uri provided.
        let (ptr, value) = self.resolve(&uri).await?;

        // self.compile_schema(uri, &value)
        //     .await
        //     .tap_ok(|_| self.schemas.accept_txn())
        //     .tap_err(|_| self.schemas.rollback_txn())
        todo!()
    }

    /// Returns the [`Schema`] with the given `key` if it exists.
    #[must_use]
    pub fn schema(&self, key: Key) -> Schema<'_, Key> {
        self.schemas.get(key, &self.sources).unwrap()
    }

    /// Returns the [`Schema`] with the given `id` if it exists.
    #[must_use]
    pub fn schema_by_id(&self, id: &AbsoluteUri) -> Option<Schema<'_, Key>> {
        self.schemas.get_by_uri(id, &self.sources)
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
    #[allow(clippy::unused_async)]
    pub async fn compile_all<I>(
        &mut self,
        uris: I,
    ) -> Result<Vec<Schema<'static, Key>>, CompileError>
    where
        I: IntoIterator,
        I::Item: TryIntoAbsoluteUri,
    {
        self.schemas.start_txn();
        let mut keys = Vec::new();
        for uri in uris {
            // let schema = self
            //     .compile_schema(uri)
            //     .await
            //     .tap_err(|_| self.schemas.rollback_txn())?;
            // keys.push(schema);
        }
        self.schemas.accept_txn();
        Ok(keys
            .into_iter()
            .map(|key| self.schemas.get(key, &self.sources).unwrap().into_owned())
            .collect())
    }
    #[allow(clippy::unused_async, dead_code)]
    async fn compile_schema(
        &mut self,
        base_uri: AbsoluteUri,
        source_uri: AbsoluteUri,
        value: &Value,
        path: &Pointer,
    ) -> Result<Key, CompileError> {
        // determining the dialect
        let dialect = self.dialects.pertinent_to_or_default(value);

        // identifying the schema
        let (id, uris) = dialect.identify(base_uri.clone(), path, value)?;

        // if identify did not find a primary id, use the uri + pointer fragment
        // as the lookup which will be at index 0 of uris
        let lookup_id = id.as_ref().unwrap_or(&uris[0]);

        // checking to see if the schema has already been compiled under the id
        if let Some(schema) = self.schemas.get_by_uri(lookup_id, &self.sources) {
            // if so, return it
            return Ok(schema.key);
        }

        // // compiling this schema
        // let compiled = CompiledSchema {
        //     id,
        //     uris,
        //     metaschema: dialect.primary_metaschema_id().clone(),
        //     handlers: dialect.handlers.clone().into_boxed_slice(),
        //     source_uri,
        //     source_path: path.clone(),
        // };

        // let key = self.schemas.insert(compiled).map_err(|uri| {
        //     SourceError::from(SourceConflictError {
        //         uri,
        //         value: value.clone().into(),
        //     })
        // })?;

        // // gathering nested schemas
        // let mut located_schemas = dialect.locate_schemas(&Pointer::default(), value);

        // compiling nested schemas

        todo!()
    }

    #[must_use]
    pub fn dialects(&self) -> &[Dialect] {
        &self.dialects
    }

    /// Attempts to resolve deserialize, and store the schema at the given URI
    /// using either local in-mem storage or resolved with any of the attached
    /// implementations of [`Resolve`]s.
    async fn resolve(
        &mut self,
        uri: impl Borrow<AbsoluteUri>,
    ) -> Result<(Pointer, Value), SourceError> {
        let uri = uri.borrow();
        // if the value has already been indexed, return a clone of the local copy
        if let Some(schema) = self.sources.get(uri) {
            return Ok((Pointer::default(), schema.clone()));
        }

        // checking to see if the root resource has already been stored
        let mut base_uri = uri.clone();
        base_uri.set_fragment(None).unwrap();

        // resolving the base uri
        let resolved = self.resolvers.resolve(&base_uri).await?;

        // add the base value to the local store of sources
        let root = self
            .sources
            .source_string(base_uri, resolved, &self.deserializers)?
            .clone(); // need to clone to avoid borrow checker constraints.

        let uri_fragment = uri.fragment().unwrap_or_default();

        // if the uri does not have a fragment, we are done and can return the
        // root-level schema
        if uri_fragment.is_empty() {
            return Ok((Pointer::default(), root));
        }

        // if the uri does have a fragment then there is more work to do.
        // first, perform lookup again to see if add_source indexed the schema
        if let Some(schema) = self.sources.get(uri) {
            return Ok((Pointer::default(), schema.clone()));
        }

        // if not, the fragment must be a json pointer as all anchors and
        // schemas with fragmented ids should have been located and indexed
        // TODO: better error handling here.
        let ptr =
            Pointer::from_str(uri_fragment).map_err(|err| ResolveError::new(err, uri.clone()))?;

        let value = root.resolve(&ptr).cloned().map_err(|err| {
            SourceError::ResolutionFailed(ResolveError::new(err, uri.clone()).into())
        })?;
        Ok((ptr, value))
    }

    /// Returns the default [`Dialect`] for the `Interrogator`.
    #[must_use]
    pub fn default_dialect(&self) -> &Dialect {
        self.dialects.default_dialect()
    }

    /// Returns the [`Dialect`] for the given schema, if any.
    pub fn determine_dialect(
        &self,
        schema: &Value,
    ) -> Result<Option<&Dialect>, DialectUnknownError> {
        if let Some(schema) = self.dialects.pertinent_to(schema) {
            return Ok(Some(schema));
        }
        // TODO: this is the only place outside of a Handler that a specific
        // json schema keyword is used. This should be refactored.
        match schema
            .get(Keyword::SCHEMA.as_str())
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
        let source = Source::String(
            uri.try_into_absolute_uri()?,
            String::from_utf8(source.to_vec())?,
        );

        self.source(source)
    }
    pub fn source(&mut self, source: Source) -> Result<&Value, SourceError> {
        self.sources.source(source, &self.deserializers)
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
        self.source(Source::String(
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
        self.source(Source::Value(
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
            self.source(Source::String(k.try_into_absolute_uri()?, v.to_string()))?;
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
            self.source(Source::String(
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
            self.source(Source::Value(
                k.try_into_absolute_uri()?,
                v.borrow().clone(),
            ))?;
        }
        Ok(())
    }
}

impl Interrogator {
    /// Returns a new, empty [`Builder`].
    #[must_use]
    #[allow(unused_must_use)]
    pub fn builder() -> Builder<SchemaKey> {
        Builder::new()
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 2020-12 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_2020_12() -> Builder<SchemaKey> {
        Builder::<SchemaKey>::default()
            .json_schema_2020_12()
            .default_dialect(json_schema::draft_2020_12::JSON_SCHEMA_2020_12_ABSOLUTE_URI.clone())
            .unwrap()
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 2019-09 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_2019_09() -> Builder<SchemaKey> {
        Builder::<SchemaKey>::default()
            .json_schema_2019_09()
            .default_dialect(json_schema::draft_2019_09::JSON_SCHEMA_2019_09_ABSOLUTE_URI.clone())
            .unwrap()
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 07 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_07() -> Builder<SchemaKey> {
        Builder::<SchemaKey>::default()
            .json_schema_07()
            .default_dialect(json_schema::draft_07::JSON_SCHEMA_07_ABSOLUTE_URI.clone())
            .unwrap()
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 04 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_04() -> Builder<SchemaKey> {
        // safety: &AbsoluteUri::try_into_absolute_uri never returns an error
        Builder::<SchemaKey>::default()
            .json_schema_04()
            .default_dialect(json_schema::draft_04::JSON_SCHEMA_04_ABSOLUTE_URI.clone())
            .unwrap()
    }
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
