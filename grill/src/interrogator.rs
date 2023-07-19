use std::{any, borrow::Borrow, collections::HashMap, fmt::Debug, ops::Deref, str::FromStr};

use either::Either;
use jsonptr::{Pointer, Resolve as _};
use serde_json::Value;
use slotmap::SlotMap;
use snafu::ResultExt;

use crate::{
    deserialize::{deserialize_json, Deserializers},
    dialect::{Dialect, Dialects},
    error::{
        build_error,
        resolve_error::{self, NestedSchemaNotFound},
        BuildError, CompileError, DeserializeError, DuplicateSourceError, EvaluateError,
        FragmentedUriError, PointerError, ResolveError, ResolveErrors, SourceError,
        SourceSliceError, UnknownDialectError, UriError,
    },
    json_schema,
    keyword::Keyword,
    resolve::Resolvers,
    schema::DependencyGraph,
    schema::Schemas,
    source::Sources,
    uri::{AbsoluteUri, TryIntoAbsoluteUri},
    Deserializer, Output, Resolve, Schema, SchemaKey, Source, Structure,
};

/// Compiles and evaluates JSON Schemas.
#[derive(Clone)]
pub struct Interrogator<Key: slotmap::Key = SchemaKey> {
    dialects: Dialects,
    sources: Sources,
    resolvers: Resolvers,
    schemas: Schemas<Key>,
    deserializers: Deserializers,
}

impl<K: slotmap::Key> Debug for Interrogator<K> {
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
    /// returning the [`SchemaRef`] of either the freshly compiled [`Schema`] or
    /// the existing [`SchemaRef`] of previously compiled, immutable [`Schema`].
    ///
    /// # Errors
    /// Returns [`CompileError`] if:
    ///   - the schema fails to compile.
    ///   - the uri fails to convert to an [`AbsoluteUri`].
    pub async fn compile(&mut self, uri: impl TryIntoAbsoluteUri) -> Result<Key, CompileError> {
        let uri = uri.try_into_absolute_uri()?;
        if let Some((key, _)) = self.schema_by_id(&uri) {
            return Ok(key);
        }
        let value = self.resolve(&uri).await?;
        let dialect = self
            .determine_dialect(&value)?
            .unwrap_or(self.default_dialect());

        todo!()
    }
    #[must_use]
    pub fn dialects(&self) -> &Dialects {
        &self.dialects
    }

    /// Attempts to resolve, deserialize, and store the schema at the given URI
    /// using either local in-mem storage or resolved with any of the attached
    /// implementations of [`Resolve`]s.
    async fn resolve(&mut self, uri: impl Borrow<AbsoluteUri>) -> Result<Value, ResolveErrors> {
        let uri = uri.borrow();

        // if the value has already been indexed, return a clone of the local copy
        if let Some(schema) = self.resolve_local(uri) {
            return Ok(schema.clone());
        }

        // checking to see if the root resource has already been stored
        let mut base_uri = uri.clone();
        base_uri.set_fragment(None);

        // resolving the base uri
        let resolved = self.try_resolvers(&base_uri).await?;

        // add the base value to the local store
        let root = self
            .source(Source::Value(base_uri, resolved.clone()))?
            .clone(); // need to clone to avoid borrow checker constraints.

        let uri_fragment = uri.fragment().unwrap_or_default();

        // if the uri does not have a fragment, we are done and can return the root-level schema
        if uri_fragment.is_empty() {
            return Ok(resolved);
        }
        // if the uri does have a fragment then there is more work to do

        // first, lookup again to see if add_source was able to index the schema
        if let Some(schema) = self.resolve_local(uri) {
            return Ok(schema.clone());
        }

        // if not, the fragment must be a json pointer as all anchors and
        // schemas with fragmented ids should have been located and indexed
        let ptr = Pointer::from_str(uri_fragment)
            .map_err(PointerError::from)
            .context(NestedSchemaNotFound { uri_fragment, uri })?;
        root.resolve(&ptr)
            .cloned()
            .map_err(PointerError::from)
            .context(NestedSchemaNotFound { uri_fragment, uri })
            .map_err(Into::into)
    }

    fn default_dialect(&self) -> &Dialect {
        self.dialects.default_dialect()
    }

    fn determine_dialect(&self, schema: &Value) -> Result<Option<&Dialect>, UnknownDialectError> {
        for dialect in &self.dialects {
            if dialect.matches(schema) {
                return Ok(Some(dialect));
            }
        }
        match schema
            .get(Keyword::SCHEMA.as_str())
            .and_then(Value::as_str)
            .map(ToString::to_string)
        {
            Some(metaschema_id) => Err(UnknownDialectError { metaschema_id }),
            None => Ok(None),
        }
    }

    pub fn evaluate(
        &self,
        key: Key,
        value: &Value,
        structure: Structure,
    ) -> Result<Output, EvaluateError> {
        todo!()
    }
    #[must_use]
    pub fn schema_by_id(&self, id: &AbsoluteUri) -> Option<(Key, &Schema)> {
        self.schemas.get(id)
    }

    async fn try_resolvers(&self, uri: &AbsoluteUri) -> Result<Value, ResolveErrors> {
        let mut errors = ResolveErrors::new();
        for resolver in &self.resolvers {
            match resolver.resolve(uri).await {
                Ok(Some(data)) => {
                    match self.deserialize(&data).context(resolve_error::Deserialize {
                        schema_id: uri.clone(),
                    }) {
                        Ok(value) => return Ok(value),
                        Err(err) => errors.push(err),
                    }
                }
                Err(err) => errors.push(err),
                _ => continue,
            }
        }
        errors.push(ResolveError::not_found(uri.to_string(), None));
        Err(errors)
    }

    #[must_use]
    pub fn deserializers(&self) -> &Deserializers {
        &self.deserializers
    }
    pub fn deserialize(&self, data: &str) -> Result<Value, DeserializeError> {
        self.deserializers.deserialize(data)
    }

    fn resolve_local(&self, uri: &AbsoluteUri) -> Option<&Value> {
        self.sources.get(uri)
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
        self.sources.insert(source, &self.deserializers)
    }

    /// Adds a schema source from a `&str`
    /// # Example
    /// ```rust
    /// let mut interrogator = grill::Interrogator::json_schema_2020_12().build().unwrap();
    /// interrogator.source_str("https://example.com/schema.json", r#"{"type": "string"}"#).unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if the `uri` fails to convert to an
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
    /// Returns [`AbsoluteUriParseError`] if the `uri` fails to convert to an
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
    /// Returns [`AbsoluteUriParseError`] if a URI fails to convert to an
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
            .default_dialect(json_schema::json_schema_2020_12_absolute_uri())
            .unwrap()
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 2019-09 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_2019_09() -> Builder<SchemaKey> {
        Builder::<SchemaKey>::default()
            .json_schema_2019_09()
            .default_dialect(json_schema::json_schema_2019_09_absolute_uri())
            .unwrap()
    }

    /// Returns a new [`Builder`] with the JSON Schema Draft 07 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_07() -> Builder<SchemaKey> {
        Builder::<SchemaKey>::default()
            .json_schema_07()
            .default_dialect(json_schema::json_schema_07_absolute_uri())
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
            .default_dialect(json_schema::json_schema_04_absolute_uri())
            .unwrap()
    }
}

/// Constructs an [`Interrogator`].
pub struct Builder<Key: slotmap::Key = SchemaKey> {
    dialects: Vec<Dialect>,
    sources: Vec<Source>,
    default_dialect: Option<AbsoluteUri>,
    resolvers: Vec<Box<dyn Resolve>>,
    deserializers: Vec<(&'static str, Box<dyn Deserializer>)>,
    _marker: std::marker::PhantomData<Key>,
}

impl Default for Builder<SchemaKey> {
    fn default() -> Self {
        Self::new()
    }
}
impl Builder<SchemaKey> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dialects: Vec::new(),
            sources: Vec::new(),
            resolvers: Vec::new(),
            deserializers: Vec::new(),
            default_dialect: None,
            _marker: std::marker::PhantomData,
        }
    }
}
impl<Key> Builder<Key>
where
    Key: slotmap::Key,
{
    /// Sets a custom key type for schemas within the [`Interrogator`]. The default
    /// key type is [`grill::SchemaKey`](`SchemaKey`).
    ///
    /// This is useful if you have multiple `Interrogator`s and want to ensure that
    /// keys from one `Interrogator` are not accidentally used in another.
    ///
    /// # Example
    /// ```
    /// use grill::{Interrogator, new_key_type};
    ///
    /// new_key_type! {
    ///     pub struct MySchemaKey;
    /// }
    /// let mut interrogator = Interrogator::json_schema_2020_12()
    ///     .key::<MySchemaKey>()
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn key<K>(self) -> Builder<K>
    where
        K: slotmap::Key,
    {
        Builder {
            dialects: self.dialects,
            sources: self.sources,
            resolvers: self.resolvers,
            deserializers: self.deserializers,
            default_dialect: self.default_dialect,
            _marker: std::marker::PhantomData,
        }
    }

    #[must_use]
    pub fn dialect(mut self, dialect: impl Borrow<Dialect>) -> Self {
        self.dialects.push(dialect.borrow().clone());
        self
    }

    /// Sets the default dialect to use when a `$schema` is not provided.
    ///
    /// If not set, the first `Dialect` added to the `Builder` is used.
    ///
    /// # Example
    /// ```
    /// use grill::{Builder, json_schema_2020_12_absolute_uri};
    ///
    /// let interrogator = Builder::default()
    ///     .json_schema_2020_12()
    ///     .default_dialect(json_schema_2020_12_absolute_uri())
    ///     .build()
    ///     .unwrap()
    /// ```
    pub fn default_dialect(mut self, dialect: impl TryIntoAbsoluteUri) -> Result<Self, UriError> {
        let dialect = dialect.try_into_absolute_uri()?;
        self.default_dialect = Some(dialect);
        Ok(self)
    }

    /// Adds a source schema from a slice of bytes that will be deserialized
    /// with avaialble [`Deserializer`] at the time of
    /// [`build`](`Builder::build`).
    ///
    /// # Example
    /// ```rust
    /// use grill::Builder;
    /// let source = br#"{"type": "string"}"#;
    /// let interrogator = Builder::default().json_schema_2020_12().build().unwrap()
    /// interrogator.source_slice("https://example.com/schema.json", ).unwrap();
    /// ```
    /// # Errors
    /// Returns [`SourceError`] if:
    /// - the `uri` fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    pub fn source_slice(
        mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &[u8],
    ) -> Result<Self, SourceSliceError> {
        self.sources.push(Source::String(
            uri.try_into_absolute_uri()?,
            String::from_utf8(source.to_vec())?,
        ));
        Ok(self)
    }

    /// Adds a schema source from a `str`
    /// # Example
    /// ```rust
    /// let interrogator = grill::Builder::default()
    ///     .json_schema_2020_12()
    ///     .source_str("https://example.com/schema.json", r#"{"type": "string"}"#).unwrap()
    ///     .build()
    ///     .unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    pub fn source_str(
        mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &str,
    ) -> Result<Self, UriError> {
        self.sources.push(Source::String(
            uri.try_into_absolute_uri()?,
            source.to_string(),
        ));
        Ok(self)
    }

    /// Adds a source schema from a [`Value`]
    /// # Example
    /// ```rust
    /// use serde_json::json;
    ///
    /// let interrogator = grill::Builder::default()
    ///     .json_schema_2020_12()
    ///     .source_value("https://example.com/schema.json", json!({"type": "string"})).unwrap()
    ///     .build()
    ///     .unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    ///
    pub fn source_value(
        mut self,
        uri: impl TryIntoAbsoluteUri,
        source: impl Borrow<Value>,
    ) -> Result<Self, UriError> {
        self.sources.push(Source::Value(
            uri.try_into_absolute_uri()?,
            source.borrow().clone(),
        ));
        Ok(self)
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, Deref<Target=str>)`
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", r#"{"type": "string"}"#);
    /// let interrogator = grill::Builder::default()
    ///     .json_schema_2020_12()
    ///     .source_strs(sources).unwrap()
    ///     .build()
    ///     .unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if a URI fails to convert to an
    /// [`AbsoluteUri`]
    pub fn source_strs<I, K, V>(mut self, sources: I) -> Result<Self, UriError>
    where
        K: TryIntoAbsoluteUri,
        V: Deref<Target = str>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.sources
                .push(Source::String(k.try_into_absolute_uri()?, v.to_string()));
        }
        Ok(self)
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, AsRef<[u8]>)`
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", br#"{"type": "string"}"#);
    /// let interrogator = grill::Builder::default()
    ///     .json_schema_2020_12()
    ///     .source_slices(sources).unwrap()
    ///     .build()
    ///     .unwrap();
    /// ```
    /// # Errors
    /// Returns [`SourceSliceError`] if:
    /// - an Absolute URI fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    ///
    pub fn source_slices<I, K, V>(mut self, sources: I) -> Result<Self, SourceSliceError>
    where
        K: TryIntoAbsoluteUri,
        V: AsRef<[u8]>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.sources.push(Source::String(
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
    /// use std::collections::HashMap;
    /// use serde_json::json;
    ///
    /// let mut sources = HashMap::new();
    /// let source = json!({"type": "string"});
    /// sources.insert("https://example.com/schema.json", source);
    /// let interrogator = grill::Builder::default()
    ///     .json_schema_2020_12()
    ///     .source_values(sources).unwrap()
    ///     .build()
    ///     .unwrap();
    /// ```
    /// # Errors
    /// Returns [`SourceError`] if:
    /// - [`TryIntoAbsoluteUri`] fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    pub fn source_values<I, K, V>(mut self, sources: I) -> Result<Self, SourceError>
    where
        K: TryIntoAbsoluteUri,
        V: Borrow<Value>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.sources.push(Source::Value(
                k.try_into_absolute_uri()?,
                v.borrow().clone(),
            ));
        }
        Ok(self)
    }

    /// Adds JSON Schema 04 [`Dialect`]
    #[must_use]
    pub fn json_schema_04(self) -> Self {
        self.dialect(json_schema::draft_04::dialect())
    }

    /// Adds JSON Schema 07 [`Dialect`]
    #[must_use]
    pub fn json_schema_07(self) -> Self {
        self.dialect(json_schema::draft_07::dialect())
    }

    /// Adds JSON Schema 2019-09 [`Dialect`]
    #[must_use]
    pub fn json_schema_2019_09(self) -> Self {
        self.dialect(json_schema::draft_2019_09::dialect())
    }

    /// Adds JSON Schema 2020-12 [`Dialect`]
    #[must_use]
    pub fn json_schema_2020_12(self) -> Self {
        self.dialect(json_schema::draft_2020_12::dialect())
    }

    /// Adds a [`Resolve`] for resolving schema references.
    #[must_use]
    pub fn resolver<R>(mut self, resolver: R) -> Self
    where
        R: 'static + Resolve,
    {
        let id = any::TypeId::of::<R>();
        self.resolvers.push(Box::new(resolver));
        self
    }

    /// Adds JSON source [`Deserializer`] [`deserialize::json`](`crate::deserialize::json`)
    #[must_use]
    pub fn json(self) -> Self {
        self.deserializer("json", deserialize_json)
    }

    /// Adds TOML source [`Deserializer`] [`deserialize::toml`](`crate::deserialize::toml`)
    #[cfg(feature = "toml")]
    #[must_use]
    pub fn toml(self) -> Self {
        self.deserializer("toml", crate::deserialize::deserialize_toml)
    }

    /// Adds YAML source [`Deserializer`] [`deserialize::yaml`](`crate::deserialize::yaml`)
    #[cfg(feature = "yaml")]
    #[must_use]
    pub fn yaml(self) -> Self {
        self.deserializer("yaml", crate::deserialize::deserialize_yaml)
    }

    /// Inserts a source [`Deserializer`]. If a [`Deserializer`] for the given
    /// format eists, it will be replaced.
    ///
    /// If a `Deserializer` is not provided prior to invoking [`build`](`Builder::build`), the default
    /// [`json`] [`Deserializer`] will be added.
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

    pub async fn build(self) -> Result<Interrogator<Key>, BuildError> {
        let Self {
            dialects,
            mut sources,
            resolvers,
            deserializers,
            default_dialect,
            _marker,
        } = self;

        let dialects = Dialects::new(dialects, default_dialect)?;
        let deserializers = Deserializers::new(deserializers);
        sources.append(&mut dialects.sources());
        let sources = Sources::new(sources, &deserializers)?;
        let precompile = dialects
            .iter()
            .map(|d| d.id.clone())
            .collect::<Vec<AbsoluteUri>>();
        let resolvers = Resolvers::new(resolvers);
        let schemas = Schemas::new();
        let mut interrogator = Interrogator {
            dialects,
            sources,
            resolvers,
            schemas,
            deserializers,
        };

        for dialect_id in precompile {
            interrogator.compile(dialect_id).await?;
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
    // /// Returns [`AbsoluteUriParseError`] if the URI fails to convert
    // /// into an [`AbsoluteUri`](`crate::AbsoluteUri`).
    // pub fn precompile<I, V>(mut self, schemas: I) -> Result<Self, AbsoluteUriParseError>
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

fn ensure_no_fragment(uri: &AbsoluteUri) -> Result<(), FragmentedUriError> {
    if let Some(fragment) = uri.fragment() {
        if !fragment.is_empty() {
            return Err(FragmentedUriError { uri: uri.clone() });
        }
    }
    Ok(())
}

fn parse_pointer_or_anchor(
    fragment: &str,
) -> Result<Either<Pointer, &str>, jsonptr::MalformedPointerError> {
    if fragment.starts_with('/') {
        let ptr = Pointer::from_str(fragment)?;
        Ok(Either::Left(ptr))
    } else {
        Ok(Either::Right(fragment))
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
