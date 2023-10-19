use std::{any, borrow::Cow, fmt::Debug, ops::Deref};

use jsonptr::Pointer;
use serde_json::Value;

use crate::{
    error::{
        BuildError, CompileError, DeserializeError, EvaluateError, SourceError, UnknownKeyError,
        UriError,
    },
    keyword::cache::{Numbers, Values},
    output::{Output, Structure},
    schema::{
        compiler::Compiler,
        iter::{Iter, IterUnchecked},
        traverse::{
            AllDependents, Ancestors, Descendants, DirectDependencies, DirectDependents,
            TransitiveDependencies,
        },
        Dialect, Dialects, Key, Schema, Schemas,
    },
    source::{
        deserialize_json, Deserializer, Deserializers, Resolve, Resolvers, SourceKey, Sources, Src,
    },
    uri::{AbsoluteUri, TryIntoAbsoluteUri},
};

use crate::anymap::AnyMap;

/// Constructs an [`Interrogator`].
pub struct Build {
    dialects: Vec<Dialect>,
    sources: Vec<Src>,
    default_dialect_idx: Option<usize>,
    resolvers: Vec<Box<dyn Resolve>>,
    deserializers: Vec<(&'static str, Box<dyn Deserializer>)>,
    state: AnyMap,
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}
impl Build {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dialects: Vec::new(),
            sources: Vec::new(),
            resolvers: Vec::new(),
            deserializers: Vec::new(),
            state: AnyMap::new(),
            default_dialect_idx: None,
        }
    }
}
impl Build {
    #[must_use]
    pub fn dialect(mut self, dialect: Dialect) -> Self {
        let idx = self.dialects.len();
        self.dialects.push(dialect);
        if self.default_dialect_idx.is_none() {
            self.default_dialect_idx = Some(idx);
        }
        self
    }
    #[must_use]
    pub fn default_dialect(mut self, dialect: Dialect) -> Self {
        let idx = self.dialects.len();
        self.dialects.push(dialect);
        self.default_dialect_idx = Some(idx);
        self
    }

    /// Adds a source schema from a slice of bytes that will be deserialized
    /// with avaialble [`Deserializer`] at the time of
    /// [`build`](`Builder::build`).
    ///
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let source = br#"{"type": "string"}"#;
    ///     let mut interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    ///     interrogator.source_slice("https://example.com/schema.json", source).unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns [`SourceError`] if:
    /// - the `uri` fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    pub fn source_slice(
        mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &[u8],
    ) -> Result<Self, SourceError> {
        self.sources.push(Src::String(
            uri.try_into_absolute_uri()?,
            String::from_utf8(source.to_vec())?,
        ));
        Ok(self)
    }

    /// Adds a schema source from a `str`
    /// # Example
    /// ```rust
    /// use grill::{ Interrogator, json_schema::Build as _ };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .source_str("https://example.com/schema.json", r#"{"type": "string"}"#).unwrap()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns [`UriError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    pub fn source_str(
        mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &str,
    ) -> Result<Self, UriError> {
        self.sources.push(Src::String(
            uri.try_into_absolute_uri()?,
            source.to_string(),
        ));
        Ok(self)
    }

    /// Adds a source schema from a [`Value`]
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// use std::{ borrow::Cow, collections::HashMap };
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let schema = Cow::Owned(json!({"type": "string"}));
    ///     let interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .source_value("https://example.com/schema.json", schema).unwrap()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns [`UriError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    ///
    pub fn source_value(
        mut self,
        uri: impl TryIntoAbsoluteUri,
        source: Cow<'static, Value>,
    ) -> Result<Self, UriError> {
        self.sources
            .push(Src::Value(uri.try_into_absolute_uri()?, source));
        Ok(self)
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, Deref<Target=str>)`
    ///
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut sources = HashMap::new();
    ///     sources.insert("https://example.com/schema.json", r#"{"type": "string"}"#);
    ///     let interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .source_strs(sources).unwrap()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    ///     }
    /// ```
    /// # Errors
    /// Returns [`UriError`] if a URI fails to convert to an
    /// [`AbsoluteUri`]
    pub fn source_strs<I, K, V>(mut self, sources: I) -> Result<Self, UriError>
    where
        K: TryIntoAbsoluteUri,
        V: std::ops::Deref<Target = str>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.sources
                .push(Src::String(k.try_into_absolute_uri()?, v.to_string()));
        }
        Ok(self)
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, AsRef<[u8]>)`
    ///
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut sources = HashMap::new();
    ///     sources.insert("https://example.com/schema.json", br#"{"type": "string"}"#);
    ///     
    ///     let interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .source_slices(sources).unwrap()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns [`SourceSliceError`] if:
    /// - an Absolute URI fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    ///
    pub fn source_slices<I, K, V>(mut self, sources: I) -> Result<Self, SourceError>
    where
        K: TryIntoAbsoluteUri,
        V: AsRef<[u8]>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.sources.push(Src::String(
                k.try_into_absolute_uri()?,
                String::from_utf8(v.as_ref().to_vec())?,
            ));
        }
        Ok(self)
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, Borrow<serde_json::Value>>)`
    ///
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// use std::{ borrow::Cow, collections::HashMap };
    /// use serde_json::json;
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut sources = HashMap::new();
    ///     let source = Cow::Owned(json!({"type": "string"}));
    ///     sources.insert("https://example.com/schema.json", source);
    ///     let interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .source_values(sources).unwrap()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns [`SourceError`] if:
    /// - [`TryIntoAbsoluteUri`] fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    pub fn source_values<I, K>(mut self, sources: I) -> Result<Self, SourceError>
    where
        K: TryIntoAbsoluteUri,
        I: IntoIterator<Item = (K, Cow<'static, Value>)>,
    {
        for (k, v) in sources {
            self.sources.push(Src::Value(k.try_into_absolute_uri()?, v));
        }
        Ok(self)
    }

    /// Adds a [`Resolve`] for resolving schema references.
    #[must_use]
    pub fn resolver<R>(mut self, resolver: R) -> Self
    where
        R: 'static + Resolve,
    {
        let _id = any::TypeId::of::<R>();
        self.resolvers.push(Box::new(resolver));
        self
    }

    /// Enables support for deserializing JSON with
    /// [`deserialize_json`](`crate::deserialize::deserialize_json`)
    #[must_use]
    pub fn deserialize_json(self) -> Self {
        self.deserializer("json", deserialize_json)
    }

    /// Enables support for deserializing TOML with
    /// [`deserialize_toml`](`crate::deserialize::deserialize_toml`)
    #[cfg(feature = "toml")]
    #[must_use]
    pub fn toml_support(self) -> Self {
        self.deserializer("toml", crate::source::deserialize_toml)
    }

    /// Enables support for deserializing YAML with
    /// [`deserialize_yaml`](`crate::deserialize::deserialize_yaml`)
    #[cfg(feature = "yaml")]
    #[must_use]
    pub fn yaml_support(self) -> Self {
        self.deserializer("yaml", crate::source::deserialize_yaml)
    }

    /// Add an implementation [`Deserializer`]. If a `Deserializer` for the
    /// given format exists, it will be replaced.
    ///
    /// [`deserialize_json`] will be enabled by default.
    ///.
    #[must_use]
    pub fn deserializer<R>(mut self, format: &'static str, deserializer: R) -> Self
    where
        R: 'static + Deserializer,
    {
        let f = format.to_lowercase();
        for (idx, (fmt, _)) in self.deserializers.iter().enumerate() {
            if fmt.to_lowercase() == f {
                self.deserializers[idx] = (format, Box::new(deserializer));
                return self;
            }
        }
        self.deserializers.push((format, Box::new(deserializer)));
        self
    }

    pub async fn finish(self) -> Result<Interrogator, BuildError> {
        let Self {
            dialects,
            mut sources,
            resolvers,
            deserializers,
            default_dialect_idx,
            state,
        } = self;
        let default_dialect_id = default_dialect_idx
            .as_ref()
            .map(|idx| dialects[*idx].id().clone());
        let dialects = Dialects::new(dialects, default_dialect_id)?;
        sources.append(&mut dialects.sources());
        let deserializers = Deserializers::new(deserializers);
        let sources = Sources::new(sources, &deserializers)?;
        let resolvers = Resolvers::new(resolvers);
        let schemas = Schemas::new();
        let precompile = dialects.source_ids().cloned().collect::<Vec<AbsoluteUri>>();

        let mut interrogator = Interrogator {
            dialects,
            sources,
            resolvers,
            schemas,
            deserializers,
            state,
            numbers: Numbers::default(),
            values: Values::default(),
        };

        for id in precompile {
            interrogator.compile(id).await?;
        }

        Ok(interrogator)
    }

    // /// Precompiles schemas at the given URIs.
    // ///
    // /// # Example
    // /// ```rust

    // /// let interrogator = grill::Builder::default()
    // ///    .json_schema_2020_12()
    // ///    .source_str("https://example.com/schema.json", r#"{"type": "string"}"#).unwrap()
    // ///    .precompile(["https://example.com/schema.json"]).unwrap()
    // ///    .build()
    // ///    .unwrap();
    // /// ```
    // /// # Errors
    // /// Returns [`UriError`] if the URI fails to convert
    // /// into an [`AbsoluteUri`](`crate::AbsoluteUri`).
    // pub fn precompile<I, V>(mut self, schemas: I) -> Result<Self, UriError>
    // where
    //     I: IntoIterator<Item = V>,
    //     V: TryIntoAbsoluteUri,
    // {
    //     for schema in schemas {
    //         self.precompile.insert(schema.try_into_absolute_uri()?);
    //     }
    //     Ok(self)
    // }
}

/// Compiles and evaluates JSON Schemas.
#[derive(Clone)]
pub struct Interrogator {
    pub(crate) dialects: Dialects,
    pub(crate) sources: Sources,
    pub(crate) resolvers: Resolvers,
    pub(crate) schemas: Schemas,
    pub(crate) deserializers: Deserializers,
    pub(crate) numbers: Numbers,
    pub(crate) values: Values,
    pub(crate) state: AnyMap,
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

    /// Returns [`Ancestors`] which is an [`Iterator`] over the [`Schema`]s
    /// that contain the specified [`Schema`] by [`Key`].
    ///
    ///
    /// Note that the JSON Schema specification states that if a schema is
    /// identified (by having either an `$id` field for Draft 07 and beyond or
    /// an `id` field for Draft 04 and earlier), then it must be the document
    /// root. As such, embedded schemas with an id will not have a parent, even
    /// if the [`Schema`] is embedded.
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this
    /// `Interrogator`
    pub fn ancestors(&self, key: Key) -> Result<Ancestors<'_>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.ancestors(key, &self.sources))
    }
    /// Returns [`Ancestors`] which is an [`Iterator`] over the [`Schema`]s
    /// that contain the specified [`Schema`] by [`Key`].
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

    /// Returns [`DirectDependents`] which is an [`Iterator`] over
    /// [`Schema`]s which directly depend on a specified
    /// [`Schema`](crate::schema::Schema)
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn direct_dependents(&self, key: Key) -> Result<DirectDependents<'_>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.direct_dependents(key, &self.sources))
    }

    /// Return [`DirectDependents`] which is an [`Iterator`] over
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

    /// Returns [`AllDependents`] which is an [`Iterator`] over [`Schema`]s which
    /// depend on a specified [`Schema`](crate::schema::Schema)
    pub fn all_dependents(&self, key: Key) -> Result<AllDependents<'_>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.all_dependents(key, &self.sources))
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
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    ///     interrogator.source_str("https://example.com/string.json", r#"{"type": "string"}"#).unwrap();
    ///     interrogator.source_str("https://example.com/number.json", r#"{"type": "number"}"#).unwrap();
    ///     let schemas = interrogator.compile_all(vec![
    ///        "https://example.com/string.json",
    ///        "https://example.com/number.json",
    ///     ]).await.unwrap();
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
        self.start_txn();
        match Compiler::new(self).compile_all(uris).await {
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
        let uri = uri.try_into_absolute_uri()?;
        self.start_txn();
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
    pub fn dialects(&self) -> &Dialects {
        &self.dialects
    }

    /// Returns the default [`Dialect`] for the `Interrogator`.
    #[must_use]
    pub fn default_dialect(&self) -> &Dialect {
        self.dialects.primary()
    }

    // /// Returns the [`Dialect`] for the given schema, if any.
    // pub fn determine_dialect(
    //     &self,
    //     schema: &Value,
    // ) -> Result<Option<&Dialect>, DialectUnknownError> {
    //     if let Some(schema) = self.dialects.pertinent_to(schema) {
    //         return Ok(Some(schema));
    //     }
    //     // TODO: this is the only place outside of a Keyword that a specific
    //     // json schema keyword is used. This should be refactored.
    //     match schema
    //         .get(json_schema::SCHEMA)
    //         .and_then(Value::as_str)
    //         .map(ToString::to_string)
    //     {
    //         Some(metaschema_id) => Err(DialectUnknownError { metaschema_id }),
    //         None => Ok(None),
    //     }
    // }

    pub fn evaluate<'v>(
        &self,
        key: Key,
        structure: Structure,
        value: &'v Value,
    ) -> Result<Output<'v>, EvaluateError> {
        let mut eval_state = AnyMap::new();
        self.schemas.evaluate(
            structure,
            key,
            value,
            Pointer::default(),
            Pointer::default(),
            &self.sources,
            &self.state,
            &mut eval_state,
        )
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
    /// use grill::{ Interrogator, json_schema::Build as _ };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    ///     let source = br#"{"type": "string"}"#;
    ///     interrogator.source_slice("https://example.com/schema.json", source).unwrap();
    /// }
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
    fn source(&mut self, src: Src) -> Result<&Value, SourceError> {
        self.sources.start_txn();
        let result = match src {
            Src::String(uri, s) => self.sources.insert_string(uri, s, &self.deserializers),
            Src::Value(uri, v) => self.sources.insert_value(uri, v),
        };
        match result {
            Ok((key, _, _)) => {
                self.sources.commit_txn();
                Ok(self.sources.get(key))
            }
            Err(err) => {
                self.sources.rollback_txn();
                Err(err)
            }
        }
    }

    /// Adds a schema source from a `&str`
    /// # Example
    /// ```rust
    /// use grill::{ Interrogator, json_schema::Build as _ };
    ///
    /// #[tokio::main]
    /// async fn main(){
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// let schema = r#"{"type": "string"}"#;
    /// interrogator.source_str("https://example.com/schema.json", schema).unwrap();
    /// }
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
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// use serde_json::json;
    /// use std::borrow::Cow;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    ///     let source = Cow::Owned(json!({"type": "string"}));
    ///     interrogator.source_value("https://example.com/schema.json", source).unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns [`UriError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    ///
    pub fn source_value(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
        source: Cow<'static, Value>,
    ) -> Result<&Value, SourceError> {
        self.source(Src::Value(uri.try_into_absolute_uri()?, source))
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, Deref<Target=str>)`
    ///
    /// # Example
    /// ```
    /// use grill::{Interrogator, json_schema::Build as _};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main(){
    ///     let mut sources = HashMap::new();
    ///     sources.insert("https://example.com/schema.json", r#"{"type": "string"}"#);
    ///     let mut interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    ///     interrogator.source_strs(sources).unwrap();
    /// }
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
    /// use grill::{ Interrogator, json_schema::Build };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut sources = HashMap::new();
    ///     sources.insert("https://example.com/schema.json", br#"{"type": "string"}"#);
    ///     let mut interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    ///     interrogator.source_slices(sources).unwrap();
    /// }
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
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// use std::{collections::HashMap, borrow::Cow};
    /// use serde_json::json;
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut sources = HashMap::new();
    ///     let source = json!({"type": "string"});
    ///     sources.insert("https://example.com/schema.json", Cow::Owned(source));
    ///
    ///     let mut interrogator = Interrogator::build()
    ///         .json_schema_2020_12()
    ///         .finish()
    ///         .await
    ///         .unwrap();
    ///     interrogator.source_values(sources).unwrap();
    /// }
    /// ```
    /// # Errors
    /// Returns [`SourceSliceError`] if:
    /// - an Absolute URI fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    ///
    pub fn source_values<I, K>(&mut self, sources: I) -> Result<(), SourceError>
    where
        K: TryIntoAbsoluteUri,
        I: IntoIterator<Item = (K, Cow<'static, Value>)>,
    {
        for (k, v) in sources {
            self.source(Src::Value(k.try_into_absolute_uri()?, v))?;
        }
        Ok(())
    }

    /// Returns a new, empty [`Build`].
    #[must_use]
    #[allow(unused_must_use)]
    pub fn build() -> Build {
        Build::new()
    }

    /// Returns a new [`Build`] with the JSON Schema Draft 2019-09 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_2019_09() -> Build {
        // Builder::default()
        //     .with_json_schema_2019_09()
        //     .with_default_dialect(
        //         json_schema::draft_2019_09::JSON_SCHEMA_2019_09_ABSOLUTE_URI.clone(),
        //     )
        todo!()
    }

    /// Returns a new [`Build`] with the JSON Schema Draft 07 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_07() -> Build {
        // Builder::default()
        //     .with_json_schema_07()
        //     .with_default_dialect(json_schema::draft_07::JSON_SCHEMA_07_ABSOLUTE_URI.clone())
        //     .unwrap()
        todo!()
    }

    /// Returns a new [`Build`] with the JSON Schema Draft 04 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_04() -> Build {
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs::File;
//     use std::io::prelude::*;
//     #[tokio::test]
//     async fn test_build() {
//         let interrogator = Build::default()
//             .json_schema_2020_12()
//             .source_str("https://example.com/schema.json", r#"{"type": "string"}"#)
//             .unwrap()
//             .finish()
//             .await
//             .unwrap();

//         let mut file = File::create("foo.txt").unwrap();
//         file.write_all(format!("{interrogator:#?}").as_bytes())
//             .unwrap();
//     }
// }
