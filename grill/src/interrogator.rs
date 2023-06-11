use std::{
    any::{self},
    borrow::Borrow,
    collections::{HashMap, HashSet},
    ops::Deref,
};

use dyn_clone::{clone_trait_object, DynClone};
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};
use snafu::ResultExt;

use crate::{
    deserialize::json,
    dialect::Dialect,
    error::{
        build_error, AbsoluteUriParseError, BuildError, CompileError, DeserializeError,
        DuplicateSourceError, EvaluateError, FragmentedDialectIdError, FragmentedSourceUriError,
        SourceError, SourceSliceError,
    },
    graph::DependencyGraph,
    json_schema,
    uri::{AbsoluteUri, TryIntoAbsoluteUri},
    Deserializer, Handler, Output, Resolve, Structure,
};

#[derive(Clone)]
pub struct Schema {
    /// The URI of the schema.
    pub id: Option<AbsoluteUri>,
    /// The URI of the schema's `Metaschema`.
    pub meta_schema: AbsoluteUri,
    /// The Handlers associated with the schema.
    pub handlers: Box<[Handler]>,
}

new_key_type! {
    /// Reference to a [`CompiledSchema`]
    pub struct SchemaKey;
}

/// Compiles and evaluates JSON Schemas.
#[derive(Clone)]
pub struct Interrogator<Key: slotmap::Key = SchemaKey> {
    dialects: Vec<Dialect>,
    dialect_lookup: HashMap<AbsoluteUri, usize>,
    default_dialect: AbsoluteUri,
    sources: HashMap<AbsoluteUri, Value>,
    resolvers: Vec<Box<dyn Resolve>>,
    schemas: SlotMap<Key, Schema>,
    deserializers: Vec<(&'static str, Box<dyn Deserializer>)>,
    dep_graph: DependencyGraph,
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
    pub fn compile(&mut self, uri: impl TryIntoAbsoluteUri) -> Result<SchemaKey, CompileError> {
        todo!()
    }

    pub fn evaluate(
        &self,
        key: Key,
        value: &Value,
        structure: Structure,
    ) -> Result<Output, EvaluateError> {
        todo!()
    }

    /// Adds a source schema from a slice of bytes that will be deserialized
    /// with avaialble [`Deserializer`] at the time of
    /// [`build`](`Builder::build`).
    ///
    /// # Example
    /// ```rust
    /// let mut interrogator = grill::Interrogator::json_schema().build().unwrap();
    /// interrogator.source_slice("https://example.com/schema.json", br#"{"type": "string"}"#).unwrap();
    /// ```
    /// # Errors
    /// Returns [`SourceSliceError`] if:
    /// - the `uri` fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    pub fn source_slice(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &[u8],
    ) -> Result<(), SourceError> {
        let source = Source::String(
            uri.try_into_absolute_uri()?,
            String::from_utf8(source.to_vec())?,
        );

        self.add_source(source)
    }

    fn add_source(&mut self, source: Source) -> Result<(), SourceError> {
        let uri = source.uri();
        let source = source.value(&self.deserializers)?;

        if let Some(src) = self.sources.get(&uri) {
            if src == &source {
                return Ok(());
            }
            return Err(SourceError::DuplicateSource {
                source: DuplicateSourceError { uri, source },
            });
        }
        self.sources.insert(uri, source);
        Ok(())
    }

    /// Adds a schema source from a `str`
    /// # Example
    /// ```rust
    /// let mut interrogator = grill::Interrogator::json_schema().build().unwrap();
    /// interrogator.source_str("https://example.com/schema.json", r#"{"type": "string"}"#).unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    pub fn source_str(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &str,
    ) -> Result<(), SourceError> {
        self.add_source(Source::String(
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
    /// let mut interrogator = Interrogator::json_schema().build();
    /// interrogator.source_value("https://example.com/schema.json", json!({"type": "string"})).unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    ///
    pub fn source_value(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
        source: impl Borrow<Value>,
    ) -> Result<(), SourceError> {
        self.add_source(Source::Value(
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
    /// let mut interrogator = Interrogator::json_schema().build();
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
            self.add_source(Source::String(k.try_into_absolute_uri()?, v.to_string()))?;
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
    /// let interrogator = Interrogator::json_schema().build().unwrap();
    ///     .json_schema()
    ///     .source_slices(sources)
    ///     .unwrap();
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
            self.add_source(Source::String(
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
    /// sources.insert("https://example.com/schema.json", json!({"type": "string"});
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
        K: Borrow<AbsoluteUri>,
        V: Borrow<Value>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.add_source(Source::Value(k.borrow().clone(), v.borrow().clone()))?;
        }
        Ok(())
    }

    /// Returns a new, empty [`Builder`].
    #[must_use]
    #[allow(unused_must_use)]
    pub fn builder() -> Builder<Key> {
        Builder::new()
    }
}

impl Interrogator {
    /// Returns a new [`Builder`] loaded [`Dialect`]s for JSON Schema Drafts
    /// 2020-12, 2019-09, 7, and 4
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema() -> Builder {
        Builder::<SchemaKey>::default().json_schema()
    }
}

/// Constructs an [`Interrogator`].
pub struct Builder<Key: slotmap::Key = SchemaKey> {
    dialects: Vec<Dialect>,
    sources: Vec<Source>,
    resolvers: Vec<Box<dyn Resolve>>,
    resolver_lookup: HashMap<any::TypeId, usize>,
    deserializers: Vec<(&'static str, Box<dyn Deserializer>)>,
    precompile: HashSet<AbsoluteUri>,
    _marker: std::marker::PhantomData<Key>,
}

impl Default for Builder<SchemaKey> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Key> Builder<Key>
where
    Key: slotmap::Key,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            dialects: Vec::new(),
            sources: Vec::new(),
            resolvers: Vec::new(),
            resolver_lookup: HashMap::new(),
            deserializers: Vec::new(),
            precompile: HashSet::new(),
            _marker: std::marker::PhantomData,
        }
    }
    #[must_use]
    pub fn dialect(mut self, dialect: impl Borrow<Dialect>) -> Self {
        self.dialects.push(dialect.borrow().clone());
        self
    }
    /// Precompiles schemas at the given URIs.
    ///
    /// # Example
    /// ```rust
    /// let interrogator = grill::Builder::default()
    ///    .json_schema()
    ///    .source_str("https://example.com/schema.json", r#"{"type": "string"}"#).unwrap()
    ///    .precompile(["https://example.com/schema.json"]).unwrap()
    ///    .build()
    ///    .unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if the URI fails to convert
    /// into an [`AbsoluteUri`](`crate::AbsoluteUri`).
    pub fn precompile<I, V>(mut self, schemas: I) -> Result<Self, AbsoluteUriParseError>
    where
        I: IntoIterator<Item = V>,
        V: TryIntoAbsoluteUri,
    {
        for schema in schemas {
            self.precompile.insert(schema.try_into_absolute_uri()?);
        }
        Ok(self)
    }

    /// Adds a source schema from a slice of bytes that will be deserialized
    /// with avaialble [`Deserializer`] at the time of
    /// [`build`](`Builder::build`).
    ///
    /// # Example
    /// ```rust
    /// let interrogator = grill::Builder::default()
    ///     .json_schema()
    ///     .source_slice("https://example.com/schema.json", br#"{"type": "string"}"#).unwrap()
    ///     .build()
    ///     .unwrap();
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
    ///     .json_schema()
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
    ) -> Result<Self, AbsoluteUriParseError> {
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
    ///     .json_schema()
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
    ) -> Result<Self, AbsoluteUriParseError> {
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
    ///     .json_schema()
    ///     .source_strs(sources).unwrap()
    ///     .build()
    ///     .unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if a URI fails to convert to an
    /// [`AbsoluteUri`]
    pub fn source_strs<I, K, V>(mut self, sources: I) -> Result<Self, AbsoluteUriParseError>
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
    ///     .json_schema()
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
    /// sources.insert("https://example.com/schema.json", json!({"type": "string"});
    /// let interrogator = grill::Builder::default()
    ///     .json_schema()
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

    /// Adds [`Dialect`]s for JSON Schema Drafts 2020-12, 2019-09, 7, and 4
    #[must_use]
    pub fn json_schema(self) -> Builder {
        Builder::default()
            .json_schema_2020_12()
            .json_schema_2019_09()
            .json_schema_07()
            .json_schema_04()
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
        if let Some(idx) = self.resolver_lookup.get(&id) {
            self.resolvers.remove(*idx);
        }
        self.resolvers.push(Box::new(resolver));
        self.resolver_lookup.insert(id, self.resolvers.len() - 1);
        self
    }

    /// Adds JSON source [`Deserializer`] [`deserialize::json`](`crate::deserialize::json`)
    #[must_use]
    pub fn json(self) -> Self {
        self.deserializer("json", json)
    }

    /// Adds TOML source [`Deserializer`] [`deserialize::toml`](`crate::deserialize::toml`)
    #[cfg(feature = "toml")]
    #[must_use]
    pub fn toml(self) -> Self {
        self.deserializer("toml", crate::deserialize::toml)
    }

    /// Adds YAML source [`Deserializer`] [`deserialize::yaml`](`crate::deserialize::yaml`)
    #[cfg(feature = "yaml")]
    #[must_use]
    pub fn yaml(self) -> Self {
        self.deserializer("yaml", crate::deserialize::yaml)
    }

    /// Inserts a source [`Deserializer`]. If a [`Deserializer`] for the given
    /// format eists, it will be replaced.
    ///
    /// If a `Deserializer` is not provided prior to building, the default
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

    pub fn build(self) -> Result<Interrogator<Key>, BuildError> {
        let Self {
            dialects,
            sources,
            resolver_lookup: _resolver_lookup,
            resolvers,
            deserializers,
            precompile,
            _marker,
        } = self;
        let dep_graph = DependencyGraph::new();

        let (dialects, dialect_lookup) = Self::get_dialects(dialects)?;
        let deserializers = Self::get_deserializers(deserializers);
        let sources = Self::get_sources(sources, &deserializers)?;

        let default_dialect = dialects[0].id.clone();
        let schemas: SlotMap<Key, Schema> = SlotMap::with_key();
        let mut interrogator = Interrogator {
            dialects,
            dialect_lookup,
            default_dialect,
            sources,
            resolvers,
            schemas,
            deserializers,
            dep_graph,
        };
        for id in precompile {
            interrogator.compile(id)?;
        }
        Ok(interrogator)
    }

    fn get_sources(
        sources: Vec<Source>,
        deserializers: &[(&'static str, Box<dyn Deserializer>)],
    ) -> Result<HashMap<AbsoluteUri, Value>, BuildError> {
        let mut res: HashMap<AbsoluteUri, Value> = HashMap::with_capacity(sources.len());
        for src in sources {
            let uri = src.uri();
            if let Some(fragment) = uri.fragment() {
                if !fragment.is_empty() {
                    return Err(FragmentedSourceUriError { uri }.into());
                }
            }
            let src = src
                .value(deserializers)
                .context(build_error::DeserializeSource { uri: uri.clone() })?;

            res.insert(uri, src);
        }
        // if res.capacity() > res.len() {
        //     res.shrink_to_fit();
        // }
        Ok(res)
    }
    fn get_deserializers(
        deserializers: Vec<(&'static str, Box<dyn Deserializer>)>,
    ) -> Vec<(&'static str, Box<dyn Deserializer>)> {
        if deserializers.is_empty() {
            vec![("json", Box::new(json))]
        } else {
            deserializers
        }
    }
    fn get_dialects(
        dialects: Vec<Dialect>,
    ) -> Result<(Vec<Dialect>, HashMap<AbsoluteUri, usize>), FragmentedDialectIdError> {
        let mut queue = Vec::with_capacity(dialects.len());
        let mut indexed = HashSet::with_capacity(dialects.len());
        for dialect in dialects.into_iter().rev() {
            if let Some(fragment) = dialect.id.fragment() {
                if !fragment.is_empty() {
                    return Err(FragmentedDialectIdError {
                        id: dialect.id.clone(),
                    });
                }
            }
            if indexed.contains(&dialect.id) {
                continue;
            }
            let id = dialect.id.clone();
            queue.push(dialect);
            indexed.insert(id);
        }
        let mut dialects = Vec::with_capacity(queue.len());
        let mut lookup = HashMap::with_capacity(queue.len());
        for dialect in queue.into_iter().rev() {
            lookup.insert(dialect.id.clone(), dialects.len());
            dialects.push(dialect);
        }
        Ok((dialects, lookup))
    }
}
enum Source {
    String(AbsoluteUri, String),
    Value(AbsoluteUri, Value),
}

impl Source {
    fn uri(&self) -> AbsoluteUri {
        match self {
            Self::Value(uri, _) | Self::String(uri, _) => uri.clone(),
        }
    }
    fn value(
        &self,
        deserializers: &[(&'static str, Box<dyn Deserializer>)],
    ) -> Result<Value, DeserializeError> {
        match self {
            Self::String(_, s) => {
                let mut source = None;
                let mut errs: HashMap<&'static str, erased_serde::Error> = HashMap::new();
                for (fmt, de) in deserializers {
                    match de.deserialize(s) {
                        Ok(v) => {
                            source = Some(v);
                            break;
                        }
                        Err(e) => {
                            errs.insert(fmt, e);
                            continue;
                        }
                    }
                }
                let Some(source) = source  else {
                    return Err(DeserializeError { formats: errs });
                };
                Ok(source)
            }
            Self::Value(_, source) => Ok(source.clone()),
        }
    }
}
