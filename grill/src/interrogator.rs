use std::{
    any::{self},
    borrow::Borrow,
    collections::{HashMap, HashSet},
    ops::Deref,
};

use dyn_clone::{clone_trait_object, DynClone};
use fancy_regex::CompileError;
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};

use crate::{
    deserialize::json,
    dialect::Dialect,
    error::{AbsoluteUriParseError, BuildError, FragmentedDialectIdError, SourceSliceError},
    graph::DependencyGraph,
    json_schema,
    uri::AbsoluteUri,
    Deserializer, Handler, Resolve,
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

/// Compiles and stores schemas.
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
    pub fn compile(
        &mut self,
        uri: impl TryInto<AbsoluteUri, Error = AbsoluteUriParseError>,
    ) -> Result<SchemaKey, CompileError> {
        todo!()
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
        V: TryInto<AbsoluteUri, Error = AbsoluteUriParseError>,
    {
        for schema in schemas {
            self.precompile.insert(schema.try_into()?);
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
    ///     .build().unwrap();
    /// ```
    /// # Errors
    /// Returns [`SourceSliceError`] if:
    /// - the `uri` fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    pub fn source_slice(
        mut self,
        uri: impl TryInto<AbsoluteUri, Error = AbsoluteUriParseError>,
        source: &[u8],
    ) -> Result<Self, SourceSliceError> {
        self.sources.push(Source::String(
            uri.try_into()?,
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
    ///     .build().unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    pub fn source_str(
        mut self,
        uri: impl TryInto<AbsoluteUri, Error = AbsoluteUriParseError>,
        source: &str,
    ) -> Result<Self, AbsoluteUriParseError> {
        self.sources
            .push(Source::String(uri.try_into()?, source.to_string()));
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
    ///     .build().unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if the `uri` fails to convert to an
    /// [`AbsoluteUri`](`crate::AbsoluteUri`).
    ///
    pub fn source_value(
        mut self,
        uri: impl TryInto<AbsoluteUri, Error = AbsoluteUriParseError>,
        source: impl Borrow<Value>,
    ) -> Result<Self, AbsoluteUriParseError> {
        self.sources
            .push(Source::Value(uri.try_into()?, source.borrow().clone()));
        Ok(self)
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryInto<AbsoluteUri, Error = AbsoluteUriParseError>, Deref<Target=str>)`
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", r#"{"type": "string"}"#);
    /// let interrogator = grill::Builder::default()
    ///     .json_schema()
    ///     .source_strs(sources).unwrap()
    ///     .build().unwrap();
    /// ```
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if a URI fails to convert to an
    /// [`AbsoluteUri`]
    pub fn source_strs<I, K, V>(mut self, sources: I) -> Result<Self, AbsoluteUriParseError>
    where
        K: TryInto<AbsoluteUri, Error = AbsoluteUriParseError>,
        V: Deref<Target = str>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.sources
                .push(Source::String(k.try_into()?, v.to_string()));
        }
        Ok(self)
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryInto<AbsoluteUri, Error = AbsoluteUriParseError>, AsRef<[u8]>)`
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", br#"{"type": "string"}"#);
    /// let interrogator = grill::Builder::default()
    ///     .json_schema()
    ///     .source_slices(sources)
    ///     .unwrap();
    /// ```
    /// # Errors
    /// Returns [`SourceSliceError`] if:
    /// - an Absolute URI fails to convert to an [`AbsoluteUri`]
    /// - a source is not valid UTF-8
    ///
    pub fn source_slices<I, K, V>(mut self, sources: I) -> Result<Self, SourceSliceError>
    where
        K: TryInto<AbsoluteUri, Error = AbsoluteUriParseError>,
        V: AsRef<[u8]>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.sources.push(Source::String(
                k.try_into()?,
                String::from_utf8(v.as_ref().to_vec())?,
            ));
        }
        Ok(self)
    }
    #[must_use]
    pub fn source_values<I, K, V>(mut self, sources: I) -> Self
    where
        K: Borrow<AbsoluteUri>,
        V: Borrow<Value>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.sources
                .push(Source::Value(k.borrow().clone(), v.borrow().clone()));
        }
        self
    }

    /// Adds [`Dialect`]s for JSON Schema Drafts 2020-12, 2019-09, 7, and 4
    #[must_use]
    pub fn json_schema(mut self) -> Builder {
        Builder::default()
            .json_schema_2020_12()
            .json_schema_2019_09()
            .json_schema_07()
            .json_schema_04()
    }

    /// Adds JSON Schema 04 [`Dialect`]
    #[must_use]
    pub fn json_schema_04(mut self) -> Self {
        // TODO: add 04 dialect
        // self.dialect(crate::Dialect::json_schema_04::json_schema_04_dialect())
        self
    }

    /// Adds JSON Schema 07 [`Dialect`]
    #[must_use]
    pub fn json_schema_07(mut self) -> Self {
        self.dialect(json_schema::draft_07::dialect())
    }

    /// Adds JSON Schema 2019-09 [`Dialect`]
    #[must_use]
    pub fn json_schema_2019_09(mut self) -> Self {
        // TODO: add 2019-09 dialect
        // self.dialect(crate::json_schema::draft_2019_09::dialect())
        self
    }

    /// Adds JSON Schema 2020-12 [`Dialect`]
    #[must_use]
    pub fn json_schema_2020_12(mut self) -> Self {
        // TODO: add 2020-12 dialect
        // self.dialect(crate::json_schema::draft_2020_12::dialect())
        self
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
        for (idx, (fmt, de)) in self.deserializers.iter().enumerate() {
            if fmt.to_lowercase() == f {
                self.deserializers[idx] = (format, Box::new(deserializer));
                return self;
            }
        }
        self.deserializers.push((format, Box::new(deserializer)));
        self
    }

    pub fn build(self) -> Result<Interrogator, BuildError> {
        let Self {
            dialects,
            sources,
            resolver_lookup,
            resolvers,
            deserializers,
            precompile,
            _marker,
        } = self;
        Ok(Interrogator {
            default_dialect: dialects[0].id.clone(),
            dep_graph: DependencyGraph::new(),
            deserializers: Vec::new(),
            dialect_lookup: HashMap::new(),
            dialects: Vec::new(),
            resolvers: Vec::new(),
            schemas: SlotMap::with_key(),
            sources: HashMap::new(),
        })
        // let graph = DependencyGraph::new();
        // let (dialects, dialect_lookup) = Self::get_dialects(dialects)?;
        // let sources = Self::get_sources(sources)?;
        // let deserializers = Self::get_deserializers(deserializers);
        // let dep_graph = DependencyGraph::new();
        // let default_dialect = dialects[0].id.clone(); // TODO: FIX
        // let i = Interrogator::<Key> {
        //     dialects,
        //     dialect_lookup,
        //     sources,
        //     dep_graph,
        //     resolvers,
        //     deserializers,
        //     default_dialect,
        //     schemas: SlotMap::default(),
        // };
        // todo!()
    }

    fn get_sources(sources: Vec<Source>) -> Result<HashMap<AbsoluteUri, Value>, BuildError> {
        // let mut sources = HashMap::with_capacity(sources.len());
        // for (uri, source) in sources {
        //     if let Some(fragment) = uri.fragment() {
        //         if !fragment.is_empty() {
        //             return Err(FragmentedSourceUriError::new(uri).into());
        //         }
        //     }
        //     sources.insert(uri, source);
        // }
        todo!()
        // Ok(sources)
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
