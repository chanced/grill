use std::{any, borrow::Borrow, fmt::Debug, ops::Deref, str::FromStr};

use jsonptr::{Pointer, Resolve as _};
use serde_json::Value;

use crate::{
    error::{
        CompileError, DeserializeError, DialectUnknownError, EvaluateError, SourceConflictError,
        SourceError, UnknownKeyError,
    },
    json_schema,
    output::{Output, Structure},
    schema::{
        traverse::{
            Ancestors, Descendants, DirectDependencies, DirectDependents, TransitiveDependencies,
        },
        CompiledSchema, Dialect, Dialects, Keyword, Schema, SchemaKey, Schemas,
    },
    source::{Deserializers, Resolvers, Source, Sources},
    uri::{AbsoluteUri, TryIntoAbsoluteUri},
    Builder,
};

/// Compiles and evaluates JSON Schemas.
#[derive(Clone)]
pub struct Interrogator<Key: slotmap::Key = SchemaKey> {
    pub(super) dialects: Dialects<'static>,
    pub(super) sources: Sources,
    pub(super) resolvers: Resolvers,
    pub(super) schemas: Schemas<Key>,
    pub(super) deserializers: Deserializers,
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
    /// Returns the [`Schema`] with the given `key` if it exists.
    ///
    /// # Errors
    /// Returns [`UnknownKeyError`] if the `key` does not belong to this `Interrgator`.
    pub fn schema(&self, key: Key) -> Result<Schema<'_, Key>, UnknownKeyError> {
        self.schemas.get(key, &self.sources)
    }

    #[must_use]
    /// Returns the [`Schema`] with the given `key` if it exists.
    ///
    /// # Panics
    /// Panics if the `key` does not belong to this `Interrgator`.
    pub fn schema_unchecked(&self, key: Key) -> Schema<'_, Key> {
        self.schemas.get_unchecked(key, &self.sources)
    }

    /// Returns the [`Schema`] with the given `id` if it exists.
    #[must_use]
    pub fn schema_by_id(&self, id: &AbsoluteUri) -> Option<Schema<'_, Key>> {
        self.schemas.get_by_uri(id, &self.sources)
    }

    /// Returns `true` if `key` belongs to this `Interrogator`
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
    pub fn ancestors(&self, key: Key) -> Result<Ancestors<'_, Key>, UnknownKeyError> {
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
    pub fn ancestors_unchecked(&self, key: Key) -> Ancestors<'_, Key> {
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
    pub fn descendants(&self, key: Key) -> Result<Descendants<'_, Key>, UnknownKeyError> {
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
    pub fn descendants_unchecked(&self, key: Key) -> Result<Descendants<'_, Key>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.descendants(key, &self.sources))
    }

    /// Returns [`DirectDependencies`] which is an [`Iterator`] over the direct
    /// dependencies of a [`Schema`]
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn direct_dependencies(
        &self,
        key: Key,
    ) -> Result<DirectDependencies<'_, Key>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.direct_dependencies(key, &self.sources))
    }

    /// Returns [`DirectDependencies`] which is an [`Iterator`] over the direct
    /// dependencies of a [`Schema`]
    ///
    /// # Panics
    /// Panics if `key` does not belong to this `Interrogator`
    pub fn direct_dependencies_unchecked(&self, key: Key) -> DirectDependencies<'_, Key> {
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
    ) -> Result<TransitiveDependencies<'_, Key>, UnknownKeyError> {
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
    pub fn transitive_dependencies_unchecked(&self, key: Key) -> TransitiveDependencies<'_, Key> {
        self.schemas.transitive_dependencies(key, &self.sources)
    }

    /// Return [`Schema`](crate::schema::Schema)s which is an [`Iterator`] over
    /// [`Schema`]s which directly depend on a specified
    /// [`Schema`](crate::schema::Schema)
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn direct_dependents(
        &self,
        key: Key,
    ) -> Result<DirectDependents<'_, Key>, UnknownKeyError> {
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
    ) -> Result<DirectDependents<'_, Key>, UnknownKeyError> {
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
        if self.schemas.contains_key(key) {
            Ok(f())
        } else {
            Err(UnknownKeyError)
        }
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
        self.schemas.start_txn();
        // convert the uri to an absolute uri
        let uri = uri.try_into_absolute_uri()?;
        // resolving the uri provided.
        // let (ptr, value) = self.resolve(&uri).await?;
        // self.compile_schema(uri, &value)
        //     .await
        //     .tap_ok(|_| self.schemas.accept_txn())
        //     .tap_err(|_| self.schemas.rollback_txn());
        todo!()
    }
    #[allow(clippy::unused_async, dead_code)]
    async fn compile_schema(
        &mut self,
        base_uri: AbsoluteUri,
        path: &Pointer,
        value: &Value,
        src_uri: AbsoluteUri,
        src_path: &Pointer,
        mut parent: Option<Key>,
    ) -> Result<Key, CompileError> {
        // determining the dialect
        let dialect = self.dialects.pertinent_to_or_default(value);

        // identifying the schema
        let (id, uris) = dialect.identify(base_uri, path, value)?;

        // if identify did not find a primary id, use the uri + pointer fragment
        // as the lookup which will be at the first position in the uris list
        let lookup_id = id.as_ref().unwrap_or(&uris[0]);

        // checking to see if the schema has already been compiled under the id
        if let Some(schema) = self.schemas.get_by_uri(lookup_id, &self.sources) {
            // if so, return it
            return Ok(schema.key);
        }

        // if parent is None and this schema is not a document root (that is,
        // has an $id) then attempt to locate the parent using the pointer
        // fragment.
        if id.is_none()
            && parent.is_none()
            && lookup_id.has_fragment()
            && lookup_id.fragment().unwrap().starts_with('/')
        {
            parent = self.schemas.locate_parent(lookup_id.clone())?;
        }

        // create a new CompiledSchema and insert it. if compiling fails, the
        // schema store will rollback to its previous state.
        let key = self
            .schemas
            .insert(CompiledSchema {
                id,
                uris,
                metaschema: dialect.primary_metaschema_id().clone(),
                handlers: dialect.handlers.clone().into_boxed_slice(),
                src_uri,
                src_path: src_path.clone(),
                parent,
                subschemas: Vec::default(),
                dependents: Vec::default(),
                dependencies: Vec::default(),
                anchors: Vec::default(),
            })
            .map_err(|uri| {
                SourceError::from(SourceConflictError {
                    uri,
                    value: value.clone().into(),
                })
            })?;

        // // gathering nested schemas
        let located_schemas = dialect.locate_schemas(&Pointer::default(), value);
        for subschema_path in located_schemas {}

        // compiling nested schemas

        todo!()
    }

    #[must_use]
    pub fn dialects(&self) -> &[Dialect] {
        &self.dialects
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
    fn source(&mut self, source: Source) -> Result<&Value, SourceError> {
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
