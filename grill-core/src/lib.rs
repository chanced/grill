#![doc = include_str!("../../README.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
// #![warn(missing_docs)]
#![allow(
    clippy::module_name_repetitions,
    clippy::result_large_err,
    clippy::enum_glob_use,
    clippy::implicit_hasher,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::missing_panics_doc, // TODO: remove after todo!()s are removed
    clippy::missing_errors_doc, // TODO: remove when I get around to documenting
    clippy::wildcard_imports,
    clippy::module_inception
)]
#![cfg_attr(test, allow(clippy::too_many_lines))]

pub mod cache;
pub mod error;
/// Keywords
pub mod keyword;
pub mod visitor;
pub use grill_uri as uri;
pub use keyword::Keyword;

pub mod schema;
pub use schema::{Key, Schema};

pub mod source;
pub(crate) use source::Src;
use uri::{error::Error, AbsoluteUri};

pub mod big;

#[cfg(test)]
pub mod test;

use std::{
    any,
    borrow::Cow,
    collections::HashSet,
    fmt::Debug,
    future::{Future, IntoFuture},
    ops::Deref,
};

use jsonptr::Pointer;
use serde_json::{Number, Value};

use crate::{
    cache::{Numbers, Values},
    error::{DeserializeError, EvaluateError, SourceError, UnknownKeyError},
    schema::{
        compiler::Compiler,
        iter::{Iter, IterUnchecked},
        traverse::{
            AllDependents, Ancestors, Descendants, DirectDependencies, DirectDependents,
            TransitiveDependencies,
        },
        Dialect, Dialects, Schemas,
    },
    source::{deserialize_json, Deserializer, Deserializers, Resolve, Resolvers, Sources},
    uri::TryIntoAbsoluteUri,
};

pub trait Structure: Copy + Clone + Debug + serde::Serialize + serde::de::DeserializeOwned {
    fn verbose() -> Self;
}

pub trait Output: serde::Serialize + serde::de::DeserializeOwned {
    type Error: serde::Serialize + serde::de::DeserializeOwned;
    type Structure: crate::Structure;

    fn append(&mut self, nodes: impl Iterator<Item = Self>);
    fn push(&mut self, output: Self);
}

pub trait Lang {
    type Keyword: crate::Keyword;
    type BuildError;
    type Translator;

    /// Creates a new context for the given `params`.
    fn new_context(&mut self, params: NewContext<Self::Keyword>) -> keyword::Context<Self>;

    /// Creates a new `Self::Compile`
    fn new_compile(&mut self, params: NewCompile<Self::Keyword>) -> keyword::Compile<Self>;
}

#[derive(Debug)]
pub struct NewContext<'i, Keyword>
where
    Keyword: crate::Keyword,
{
    pub structure: keyword::Structure<Keyword>,
    pub eval_numbers: &'i mut Numbers,
    pub global_numbers: &'i Numbers,
    pub schemas: &'i mut Schemas<Keyword>,
    pub sources: &'i Sources,
    pub absolute_keyword_location: &'i AbsoluteUri,
    pub keyword_location: Pointer,
    pub instance_location: Pointer,
}

#[derive(Debug)]
pub struct NewCompile<'i, Keyword>
where
    Keyword: crate::Keyword,
{
    pub absolute_uri: &'i AbsoluteUri,
    pub global_numbers: &'i mut Numbers,
    pub schemas: &'i mut Schemas<Keyword>,
    pub sources: &'i mut Sources,
    pub dialects: &'i Dialects,
    pub resolvers: &'i Resolvers,
    pub deserializers: &'i Deserializers,
    pub values: &'i mut Values,
}

#[derive(Default)]
/// Constructs an [`Interrogator`].
pub struct Build<Lang> {
    dialects: Vec<Dialect>,
    precompile: Vec<Result<AbsoluteUri, Error>>,
    pending_srcs: Vec<PendingSrc>,
    default_dialect_idx: Option<usize>,
    resolvers: Vec<Box<dyn Resolve>>,
    deserializers: Vec<(&'static str, Box<dyn Deserializer>)>,
    numbers: Vec<Number>,
    lang: Lang,
}

// impl IntoFuture for Build {
//     type Output = Result<Interrogator, BuildError>;
//     type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>>;
//     fn into_future(self) -> Self::IntoFuture {
//         Box::pin(async move { self.finish().await })
//     }
// }

impl<Lang> Build<Lang>
where
    Lang: crate::Lang,
{
    /// Constructs a new `Build`
    #[must_use]
    pub fn new(lang: Lang) -> Self {
        Self::default()
    }
}

impl<Lang> Build<Lang>
where
    Lang: crate::Lang,
{
    /// Adds a new [`Dialect`] to the [`Interrogator`] constructed by [`Build`].
    #[must_use]
    pub fn dialect(mut self, dialect: Dialect) -> Self {
        let idx = self.dialects.len();
        self.dialects.push(dialect);
        if self.default_dialect_idx.is_none() {
            self.default_dialect_idx = Some(idx);
        }
        self
    }

    /// Sets the default [`Dialect`] for the [`Interrogator`] constructed by
    /// [`Build`].
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
    /// # #[tokio::main]
    /// # async fn main() {
    /// let source = br#"{"type": "string"}"#;
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .source_bytes("https://example.com/schema.json", source)
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_bytes(mut self, uri: impl TryIntoAbsoluteUri, source: &[u8]) -> Self {
        self.pending_srcs.push(PendingSrc::Bytes(
            uri.try_into_absolute_uri(),
            source.to_vec(),
        ));
        self
    }

    /// Adds a schema source from a `str`
    /// # Example
    /// ```rust
    /// use grill::{ Interrogator, json_schema::Build as _ };
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    ///let interrogator = Interrogator::build()
    ///    .json_schema_2020_12()
    ///    .source_str("https://example.com/schema.json", r#"{"type": "string"}"#)
    ///    .finish()
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_str(mut self, uri: impl TryIntoAbsoluteUri, source: &str) -> Self {
        self.pending_srcs.push(PendingSrc::String(
            uri.try_into_absolute_uri(),
            source.to_string(),
        ));
        self
    }
    /// Adds a source schema from an owned [`Value`]
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// # use std::{ collections::HashMap };
    /// # use serde_json::json;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let schema = json!({"type": "string"});
    /// let interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .source_owned_value("https://example.com/schema.json", schema)
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_owned_value(mut self, uri: impl TryIntoAbsoluteUri, source: Value) -> Self {
        self.pending_srcs.push(PendingSrc::Value(
            uri.try_into_absolute_uri(),
            Cow::Owned(source),
        ));
        self
    }

    /// Adds a source schema from a static reference to a [`Value`]
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// # use std::{ collections::HashMap };
    /// # use serde_json::json;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let schema = json!({"type": "string"});
    /// let interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .source_owned_value("https://example.com/schema.json", schema)
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_static_ref_value(
        mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &'static Value,
    ) -> Self {
        self.pending_srcs.push(PendingSrc::Value(
            uri.try_into_absolute_uri(),
            Cow::Borrowed(source),
        ));
        self
    }

    /// Adds a source schema from a [`Value`]
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// # use std::{ borrow::Cow, collections::HashMap };
    /// # use serde_json::json;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let schema = Cow::Owned(json!({"type": "string"}));
    /// let interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .source_value("https://example.com/schema.json", schema)
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_value(
        mut self,
        uri: impl TryIntoAbsoluteUri,
        source: Cow<'static, Value>,
    ) -> Self {
        self.pending_srcs
            .push(PendingSrc::Value(uri.try_into_absolute_uri(), source));
        self
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, Deref<Target=str>)`
    ///
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// use std::collections::HashMap;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", r#"{"type": "string"}"#);
    /// let interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .source_strs(sources)
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_strs<I, K, V>(mut self, sources: I) -> Self
    where
        K: TryIntoAbsoluteUri,
        V: std::ops::Deref<Target = str>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.pending_srcs
                .push(PendingSrc::String(k.try_into_absolute_uri(), v.to_string()));
        }
        self
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, AsRef<[u8]>)`
    ///
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// use std::collections::HashMap;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", br#"{"type": "string"}"#);
    ///
    /// let interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .source_slices(sources).unwrap()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_slices<I, K, V>(mut self, sources: I) -> Self
    where
        K: TryIntoAbsoluteUri,
        V: AsRef<[u8]>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.pending_srcs.push(PendingSrc::Bytes(
                k.try_into_absolute_uri(),
                v.as_ref().to_vec(),
            ));
        }
        self
    }

    /// Adds [`Iterator`] of sources in the form of a tuple consisting of
    /// [`TryIntoAbsoluteUri`], [`Cow`]`<'static,` [`Value`]`>`.
    ///
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// # use std::{ borrow::Cow, collections::HashMap };
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut sources = HashMap::new();
    /// sources.insert(
    ///     "https://example.com/schema.json",
    ///     Cow::Owned(json!({"type": "string"})),
    /// );
    /// let interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .source_values(sources)
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_values<I, K>(mut self, sources: I) -> Self
    where
        K: TryIntoAbsoluteUri,
        I: IntoIterator<Item = (K, Cow<'static, Value>)>,
    {
        for (k, v) in sources {
            self.pending_srcs
                .push(PendingSrc::Value(k.try_into_absolute_uri(), v));
        }
        self
    }

    /// Adds [`Iterator`] of sources in the form of a tuple consisting of
    /// [`TryIntoAbsoluteUri`], [`Value`].
    ///
    ///
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// # use std::{ borrow::Cow, collections::HashMap };
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut sources = HashMap::new();
    /// sources.insert(
    ///     "https://example.com/schema.json",
    ///     json!({"type": "string"}),
    /// );
    /// let interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .source_owned_values(sources)
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_owned_values<I, K>(self, sources: I) -> Self
    where
        K: TryIntoAbsoluteUri,
        I: IntoIterator<Item = (K, Value)>,
    {
        self.source_values(sources.into_iter().map(|(k, v)| (k, Cow::Owned(v))))
    }

    /// Adds [`Iterator`] of sources in the form of a tuple consisting of
    /// [`TryIntoAbsoluteUri`], [`&'static serde_json::Value`](`Value`).
    ///
    /// # Example
    /// ```
    /// use grill::{ AbsoluteUri, Interrogator, json_schema::Build as _ };
    /// use once_cell::sync::Lazy;
    /// # use std::{ collections::HashMap };
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() {
    /// static mut SOURCES: Lazy<HashMap<AbsoluteUri,Value>> = Lazy::new(|| {
    ///    let mut sources = HashMap::new();
    ///     sources.insert(
    ///         "https://example.com/schema.json",
    ///         json!({"type": "string"}),
    /// });
    /// let interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .source_owned_values(SOURCES.get().unwrap())
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    #[must_use]
    pub fn source_static_values<I, K>(self, sources: I) -> Self
    where
        K: TryIntoAbsoluteUri,
        I: IntoIterator<Item = (K, &'static Value)>,
    {
        self.source_values(sources.into_iter().map(|(k, v)| (k, Cow::Borrowed(v))))
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

    /// Finishes building the [`Interrogator`]. Alternatively, you can simply
    /// `await` any method as `Build` implements [`IntoFuture`].
    ///
    pub async fn finish(self) -> Result<Interrogator<Lang>, Lang::BuildError> {
        let Self {
            lang,
            dialects,
            pending_srcs,
            resolvers,
            deserializers,
            default_dialect_idx,
            precompile,
            numbers,
        } = self;
        let default_dialect_id = default_dialect_idx
            .as_ref()
            .map(|idx| dialects[*idx].id().clone());
        let dialects = Dialects::new(dialects, default_dialect_id)?;

        let deserializers = Deserializers::new(deserializers);
        let sources = Sources::new(srcs(dialects.sources(), pending_srcs)?, &deserializers)?;
        let resolvers = Resolvers::new(resolvers);
        let schemas = Schemas::new();

        let precompile: Result<Vec<AbsoluteUri>, Error> = precompile.into_iter().collect();
        let precompile = precompile.map_err(SourceError::UriFailedToParse)?;

        let dialect_ids: Vec<AbsoluteUri> = dialects.iter().map(Dialect::id).cloned().collect();

        let mut interrogator = Interrogator {
            dialects,
            sources,
            resolvers,
            schemas,
            deserializers,
            numbers: Numbers::new(numbers.iter())?,
            values: Values::default(),
        };
        interrogator.compile_dialects_schemas(dialect_ids).await?;
        interrogator.compile_all(precompile).await?;
        Ok(interrogator)
    }

    /// Precompiles schemas at the given URIs.
    ///
    /// # Example
    /// ```rust
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let interrogator = Interrogator::build()
    ///    .json_schema_2020_12()
    ///    .source_str("https://example.com/schema.json", r#"{"type": "string"}"#)
    ///    .precompile(["https://example.com/schema.json"])
    ///    .finish()
    ///    .await
    ///    .unwrap();
    /// let uri = AbsoluteUri::parse("https://example.com/schema.json").unwrap();
    /// let schema = interrogator.schema_by_uri(&uri).unwrap();
    /// assert_eq!(&schema, &json!({"type": "string"}));
    /// ```
    /// # Errors
    /// Returns [`UriError`] if the URI fails to convert
    /// into an [`AbsoluteUri`](`crate::AbsoluteUri`).
    #[must_use]
    pub fn precompile<I, V>(mut self, schemas: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: TryIntoAbsoluteUri,
    {
        for schema in schemas {
            self.precompile.push(schema.try_into_absolute_uri());
        }
        self
    }
}

/// Compiles and evaluates JSON Schemas.
#[derive(Clone)]
pub struct Interrogator<Lang: crate::Lang> {
    pub(crate) dialects: Dialects,
    pub(crate) sources: Sources,
    pub(crate) resolvers: Resolvers,
    pub(crate) schemas: Schemas<Lang::Keyword>,
    pub(crate) deserializers: Deserializers,
    pub(crate) numbers: Numbers,
    pub(crate) values: Values,
}

impl<Lang> Debug for Interrogator<Lang>
where
    Lang: crate::Lang,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interrogator")
            .field("dialects", &self.dialects)
            .field("sources", &self.sources)
            .field("schemas", &self.schemas)
            .field("deserializers", &self.deserializers)
            .finish_non_exhaustive()
    }
}

impl<Lang> Interrogator<Lang>
where
    Lang: crate::Lang,
{
    pub fn print_source_index(&self) {
        self.sources.print_index();
    }

    /// Returns the [`Schema`] with the given `key` if it exists.
    ///
    /// # Errors
    /// Returns [`UnknownKeyError`] if the `key` does not belong to this `Interrgator`.
    pub fn schema(&self, key: Key) -> Result<Schema<'_, Lang::Keyword>, UnknownKeyError> {
        self.schemas.get(key, &self.sources)
    }

    #[must_use]
    /// Returns the [`Schema`] with the given `key` if it exists.
    ///
    /// # Panics
    /// Panics if the `key` does not belong to this `Interrgator`.
    pub fn schema_unchecked(&self, key: Key) -> Schema<'_, Lang::Keyword> {
        self.schemas.get_unchecked(key, &self.sources)
    }

    /// Returns the [`Schema`] with the given `id` if it exists.
    #[must_use]
    pub fn schema_by_uri(&self, id: &AbsoluteUri) -> Option<Schema<'_, Lang::Keyword>> {
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
    pub fn direct_dependencies(
        &self,
        key: Key,
    ) -> Result<DirectDependencies<'_, Lang::Keyword>, UnknownKeyError> {
        self.schemas
            .ensure_key_exists(key, || self.schemas.direct_dependencies(key, &self.sources))
    }

    /// Returns [`DirectDependencies`] which is an [`Iterator`] over the direct
    /// dependencies of a [`Schema`]
    ///
    /// # Panics
    /// Panics if `key` does not belong to this `Interrogator`
    #[must_use]
    pub fn direct_dependencies_unchecked(&self, key: Key) -> DirectDependencies<'_, Lang::Keyword> {
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
    ) -> Result<TransitiveDependencies<'_, Lang::Keyword>, UnknownKeyError> {
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
    pub fn transitive_dependencies_unchecked(
        &self,
        key: Key,
    ) -> TransitiveDependencies<'_, Lang::Keyword> {
        self.schemas.transitive_dependencies(key, &self.sources)
    }

    /// Returns [`DirectDependents`] which is an [`Iterator`] over
    /// [`Schema`]s which directly depend on a specified
    /// [`Schema`](crate::schema::Schema)
    ///
    /// # Errors
    /// Returns `UnknownKeyError` if `key` does not belong to this `Interrogator`
    pub fn direct_dependents(
        &self,
        key: Key,
    ) -> Result<DirectDependents<'_, Lang::Keyword>, UnknownKeyError> {
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
    ) -> Result<DirectDependents<'_, Lang::Keyword>, UnknownKeyError> {
        self.ensure_key_exists(key, || self.schemas.direct_dependents(key, &self.sources))
    }

    /// Returns [`AllDependents`] which is an [`Iterator`] over [`Schema`]s which
    /// depend on a specified [`Schema`](crate::schema::Schema)
    pub fn all_dependents(
        &self,
        key: Key,
    ) -> Result<AllDependents<'_, Lang::Keyword>, UnknownKeyError> {
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
    /// ## Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// interrogator.source_str("https://example.com/string.json", r#"{"type": "string"}"#).unwrap();
    /// interrogator.source_str("https://example.com/number.json", r#"{"type": "number"}"#).unwrap();
    /// let schemas = interrogator.compile_all(vec![
    ///    "https://example.com/string.json",
    ///    "https://example.com/number.json",
    /// ]).await.unwrap();
    /// assert_eq!(schemas.len(), 2);
    /// # }
    /// ```
    ///
    /// ## Errors
    /// Returns [`CompileError`] if any of the schemas fail to compile.
    //
    #[allow(clippy::unused_async)]
    pub async fn compile_all<I>(
        &mut self,
        uris: I,
    ) -> Result<Vec<(AbsoluteUri, Key)>, <Lang::Keyword as crate::Keyword>::CompileError>
    where
        I: IntoIterator,
        I::Item: TryIntoAbsoluteUri,
    {
        let uris = uris.into_iter();
        self.start_txn();
        match Compiler::new(self, true).compile_all(uris).await {
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
    pub async fn compile(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
    ) -> Result<Key, keyword::CompileError<Lang::Keyword>> {
        // TODO: use the txn method once async closures are available: https://github.com/rust-lang/rust/issues/62290
        let uri = uri.try_into_absolute_uri()?;
        self.start_txn();
        match self.compile_schema(uri, true).await {
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

    async fn compile_dialects_schemas(
        &mut self,
        uris: Vec<AbsoluteUri>,
    ) -> Result<(), keyword::CompileError<Lang::Keyword>> {
        let uris = uris.into_iter();
        self.start_txn();
        match Compiler::new(self, false).compile_all(uris).await {
            Ok(results) => {
                self.commit_txn();
                self.dialects.set_keys(results);
                Ok(())
            }
            Err(err) => {
                self.rollback_txn();
                Err(err)
            }
        }
    }

    /// Returns an [`Iter`] of `Result<Schema, UnknownKeyError>`s for the given [`Key`]s.
    ///
    /// Each item in the iterator is a [`Result`] because it is possible a [`Key`] may not
    /// belong to this `Interrogator`.
    ///
    /// If you know that all of the keys belong to this `Interrogator`, you can use
    /// [`iter_unchecked`](`Interrogator::iter_unchecked`) instead.
    #[must_use]
    pub fn iter<'i, Keyword>(&'i self, keys: &'i [Key]) -> Iter<'i, Keyword> {
        Iter::new(keys, &self.schemas, &self.sources)
    }

    /// Returns an [`IterUnchecked`](`IterUnchecked`) of [`Schema`]s for the
    /// given [`Key`]s.
    ///
    /// # Panics
    /// Panics if a [`Key`] does not belong to this [`Interrogator`]. If you
    /// have multiple `Interrogator` instances where mixing up keys could occur,
    /// use [`iter`](`Interrogator::iter`) instead.
    #[must_use]
    pub fn iter_unchecked<'i, Keyword>(&'i self, keys: &'i [Key]) -> IterUnchecked<'i, Keyword> {
        self.iter(keys).unchecked()
    }

    async fn compile_schema(
        &mut self,
        uri: AbsoluteUri,
        validate: bool,
    ) -> Result<Key, keyword::CompileError<Lang::Keyword>> {
        Compiler::new(self, validate).compile(uri).await
    }

    /// Returns the [`Dialects`] for this `Interrogator`
    #[must_use]
    pub fn dialects(&self) -> &Dialects {
        &self.dialects
    }

    /// Returns the default [`Dialect`] for the `Interrogator`.
    #[must_use]
    pub fn default_dialect(&self) -> &Dialect {
        self.dialects.primary()
    }

    /// Evaluates a `Schema` with the given `key` against the given `value`,
    /// returning the result of the evaluation as an [`Output`] with the specified
    /// [`Structure`].
    pub fn evaluate<'v>(
        &self,
        structure: keyword::Structure<Lang::Keyword>,
        key: Key,
        value: &'v Value,
    ) -> Result<keyword::Output<Lang::Keyword>, EvaluateError> {
        let mut evaluated = HashSet::default();
        let mut eval_numbers = Numbers::with_capacity(7);
        self.schemas.evaluate(
            structure,
            key,
            value,
            Pointer::default(),
            Pointer::default(),
            &self.sources,
            &mut evaluated,
            &self.state,
            &self.numbers,
            &mut eval_numbers,
        )
    }

    /// Returns the schema's `Key` if it exists
    #[must_use]
    pub fn schema_key_by_uri(&self, id: &AbsoluteUri) -> Option<Key> {
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
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// let source = br#"{"type": "string"}"#;
    /// interrogator.source_slice("https://example.com/schema.json", source).unwrap();
    /// # }
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
    /// # #[tokio::main]
    /// # async fn main(){
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// let schema = r#"{"type": "string"}"#;
    /// interrogator.source_str("https://example.com/schema.json", schema).unwrap();
    /// # }
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
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// let source = Cow::Owned(json!({"type": "string"}));
    /// interrogator.source_value("https://example.com/schema.json", source).unwrap();
    /// # }
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

    pub fn source_owned_value(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
        source: Value,
    ) -> Result<&Value, SourceError> {
        self.source_value(uri, Cow::Owned(source))
    }

    pub fn source_static_value(
        &mut self,
        uri: impl TryIntoAbsoluteUri,
        source: &'static Value,
    ) -> Result<&Value, SourceError> {
        self.source_value(uri, Cow::Borrowed(source))
    }

    /// Adds a set of source schemas from an [`Iterator`] of
    /// `(TryIntoAbsoluteUri, Deref<Target=str>)`
    ///
    /// # Example
    /// ```
    /// use grill::{Interrogator, json_schema::Build as _};
    /// use std::collections::HashMap;
    ///
    /// # #[tokio::main]
    /// # async fn main(){
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", r#"{"type": "string"}"#);
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// interrogator.source_strs(sources).unwrap();
    /// # }
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
    /// # use std::collections::HashMap;
    /// use grill::{ Interrogator, json_schema::Build as _ };
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut sources = HashMap::new();
    /// sources.insert("https://example.com/schema.json", br#"{"type": "string"}"#);
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// interrogator.source_slices(sources).unwrap();
    /// # }
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
    /// # use std::{collections::HashMap, borrow::Cow};
    /// # use serde_json::json;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut sources = HashMap::new();
    /// let source = json!({"type": "string"});
    /// sources.insert("https://example.com/schema.json", Cow::Owned(source));
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// interrogator.source_values(sources).unwrap();
    /// # }
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

    // Adds [`Iterator`] of sources in the form of a tuple consisting of
    /// [`TryIntoAbsoluteUri`], [`Value`].
    ///
    ///
    /// # Example
    /// ```
    /// use grill::{ Interrogator, json_schema::Build as _ };
    /// # use std::{ borrow::Cow, collections::HashMap };
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut sources = HashMap::new();
    /// sources.insert(
    ///     "https://example.com/schema.json",
    ///     json!({"type": "string"}),
    /// );
    /// let mut interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    ///
    ///  interrogator.source_owned_values(sources).unwrap();
    /// # }
    /// ```
    pub fn source_owned_values<I, K>(&mut self, sources: I) -> Result<(), SourceError>
    where
        K: TryIntoAbsoluteUri,
        I: IntoIterator<Item = (K, Value)>,
    {
        self.source_values(sources.into_iter().map(|(k, v)| (k, Cow::Owned(v))))
    }

    /// Adds [`Iterator`] of sources in the form of a tuple consisting of
    /// [`TryIntoAbsoluteUri`], [`&'static serde_json::Value`](`Value`).
    ///
    /// # Example
    /// ```
    /// use grill::{ AbsoluteUri, Interrogator, json_schema::Build as _ };
    /// use once_cell::sync::Lazy;
    /// # use std::{ collections::HashMap };
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() {
    /// static mut SOURCES: Lazy<HashMap<AbsoluteUri,Value>> = Lazy::new(|| {
    ///    let mut sources = HashMap::new();
    ///     sources.insert(
    ///         "https://example.com/schema.json",
    ///         json!({"type": "string"}),
    /// });
    /// let interrogator = Interrogator::build()
    ///     .json_schema_2020_12()
    ///     .finish()
    ///     .await
    ///     .unwrap();
    /// interrogator.source_owned_values(SOURCES.get().unwrap())
    /// # }
    /// ```
    pub fn source_static_values<I, K>(&mut self, sources: I) -> Result<(), SourceError>
    where
        K: TryIntoAbsoluteUri,
        I: IntoIterator<Item = (K, Value)>,
    {
        self.source_values(sources.into_iter().map(|(k, v)| (k, Cow::Owned(v))))
    }

    /// Returns a new, empty [`Build`].
    #[must_use]
    #[allow(unused_must_use)]
    pub fn build(lang: Lang) -> Build<Lang> {
        Build::new(lang)
    }

    /// Returns a new [`Build`] with the JSON Schema Draft 2019-09 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    pub fn json_schema_2019_09() -> Build<Lang> {
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
    pub fn json_schema_07() -> Build<Lang> {
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
    pub fn json_schema_04() -> Build<Lang> {
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
}

enum PendingSrc {
    Bytes(Result<AbsoluteUri, Error>, Vec<u8>),
    String(Result<AbsoluteUri, Error>, String),
    Value(Result<AbsoluteUri, Error>, Cow<'static, Value>),
}
impl TryFrom<PendingSrc> for Src {
    type Error = SourceError;

    fn try_from(src: PendingSrc) -> Result<Self, Self::Error> {
        Ok(match src {
            PendingSrc::Bytes(uri, bytes) => Src::String(uri?, String::from_utf8(bytes)?),
            PendingSrc::String(uri, string) => Src::String(uri?, string),
            PendingSrc::Value(uri, value) => Src::Value(uri?, value),
        })
    }
}

fn srcs(mut srcs: Vec<Src>, pending_srcs: Vec<PendingSrc>) -> Result<Vec<Src>, SourceError> {
    srcs.reserve(pending_srcs.len());
    for pending in pending_srcs {
        srcs.push(pending.try_into()?);
    }
    Ok(srcs)
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
