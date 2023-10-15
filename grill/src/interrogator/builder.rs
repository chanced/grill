use std::{any, borrow::Cow};

use serde_json::Value;

use crate::{
    error::{BuildError, SourceError, UriError},
    json_schema,
    keyword::cache::{Numbers, Values},
    schema::{dialect::Dialects, Dialect, Schemas},
    source::{deserialize_json, Deserializer, Deserializers, Resolve, Resolvers, Sources, Src},
    uri::TryIntoAbsoluteUri,
    AbsoluteUri, Interrogator,
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
    /// let interrogator = grill::Builder::default()
    ///     .json_schema_2020_12()
    ///     .source_str("https://example.com/schema.json", r#"{"type": "string"}"#).unwrap()
    ///     .build()
    ///     .unwrap();
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
        I: IntoIterator<Item = (K, Cow<'static, Value>)>,
    {
        for (k, v) in sources {
            self.sources.push(Src::Value(k.try_into_absolute_uri()?, v));
        }
        Ok(self)
    }

    /// Adds JSON Schema 04 [`Dialect`]
    #[must_use]
    pub fn json_schema_04(self) -> Self {
        self.dialect(json_schema::draft_04::dialect().clone())
    }

    /// Adds JSON Schema 07 [`Dialect`]
    #[must_use]
    pub fn json_schema_07(self) -> Self {
        self.dialect(json_schema::draft_07::dialect().clone())
    }

    /// Adds JSON Schema 2019-09 [`Dialect`]
    #[must_use]
    pub fn json_schema_2019_09(self) -> Self {
        self.dialect(json_schema::draft_2019_09::dialect().clone())
    }

    /// Adds JSON Schema 2020-12 [`Dialect`]
    #[must_use]
    pub fn json_schema_2020_12(self) -> Self {
        self.dialect(json_schema::draft_2020_12::dialect().clone())
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
