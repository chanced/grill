//! Schema source store, resolvers, and deserializers.
//!
use crate::{
    error::{
        link_error::SourceConflictSnafu, DeserializeError, LinkError, PointerError, ResolveError,
        ResolveErrors, SourceError,
    },
    uri::{decode_lossy, AbsoluteUri},
};
use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::{Pointer, Resolve as _};
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};
use snafu::{ensure, Backtrace};
use std::{
    borrow::Cow,
    collections::hash_map::{Entry, HashMap},
    convert::AsRef,
    ops::Deref,
};

const SANDBOX_ERR: &str = "transaction failed: source sandbox not found.\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new";

new_key_type! {
    /// A key for a [`Source`].
    pub struct SourceKey;
}

pub(crate) enum Input {
    String(AbsoluteUri, String),
    Value(AbsoluteUri, Cow<'static, Value>),
}

impl Input {
    pub(crate) fn deserialize_or_take_value(
        self,
        deserializers: &Deserializers,
    ) -> Result<Cow<'static, Value>, DeserializeError> {
        match self {
            Self::Value(_uri, val) => Ok(val),
            Self::String(_uri, str) => {
                let src = deserializers.deserialize(&str)?;
                Ok(Cow::Owned(src))
            }
        }
    }
    #[must_use]
    pub fn uri(&self) -> &AbsoluteUri {
        match self {
            Self::Value(uri, _) | Self::String(uri, _) => uri,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A reference to a location within a source
pub struct Source<'i> {
    pub(crate) key: SourceKey,
    /// The URI of the source
    pub uri: Cow<'i, AbsoluteUri>,
    /// The path within the source
    pub path: Cow<'i, Pointer>,
    /// The value of the source
    pub value: Cow<'i, Value>,
}

impl<'i> Source<'i> {
    pub(crate) fn new(uri: &'i AbsoluteUri, src: &'i Link, sources: &'i Sources) -> Source<'i> {
        let mut value = sources.get(src.src_key);
        if !src.src_path.is_empty() {
            value = value.resolve(&src.src_path).unwrap();
        }
        Self {
            key: src.src_key,
            uri: Cow::Borrowed(uri),
            path: Cow::Borrowed(&src.src_path),
            value: Cow::Borrowed(value),
        }
    }
    /// Returns an owned (`'static`) copy of this `Source`
    #[must_use]
    pub fn to_owned(&self) -> Source<'static> {
        Source {
            key: self.key,
            uri: Cow::Owned(self.uri.clone().into_owned()),
            path: Cow::Owned(self.path.clone().into_owned()),
            value: Cow::Owned(self.value.clone().into_owned()),
        }
    }
}

impl Deref for Source<'_> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref()
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Store {
    table: SlotMap<SourceKey, Cow<'static, Value>>,
    pub(crate) index: HashMap<AbsoluteUri, Link>,
}
impl Store {
    fn check_and_get_occupied(
        &self,
        uri: AbsoluteUri,
        src: Cow<'static, Value>,
    ) -> Result<(SourceKey, Link, Cow<'static, Value>), SourceError> {
        let key = self.index.get(&uri).unwrap().src_key;
        let existing_src = self.table.get(key).unwrap().clone();
        if src != existing_src {
            return Err(SourceError::SourceConflict {
                uri: uri.clone(),
                backtrace: snafu::Backtrace::capture(),
            }
            .into());
        }
        let link = self.index.get(&uri).unwrap().clone();
        Ok((key, link, existing_src))
    }

    fn insert_vacant(
        &mut self,
        uri: AbsoluteUri,
        src: Cow<'static, Value>,
    ) -> Result<(SourceKey, Link, Cow<'static, Value>), SourceError> {
        let mut base_uri = uri.clone();
        let fragment = base_uri.set_fragment(None).unwrap().unwrap_or_default();
        let fragment = decode_lossy(&fragment);
        let src_key = self.table.insert(src);
        let src = self.table.get(src_key).unwrap().clone();

        let link = Link::new(src_key, Pointer::default());
        self.index.insert(base_uri.clone(), link.clone());

        if fragment.is_empty() {
            return Ok((src_key, link, src));
        }
        if fragment.starts_with('/') {
            let ptr = Pointer::parse(&fragment)?;
            let link = Link::new(src_key, ptr.clone());
            self.index.insert(uri.clone(), link.clone());
            let key = self.index.get(&uri).unwrap().src_key;
            let src = src.resolve(&ptr)?.clone();
            return Ok((key, link, Cow::Owned(src)));
        }
        Ok((src_key, link, src))
    }

    fn insert(
        &mut self,
        uri: AbsoluteUri,
        src: Cow<'static, Value>,
    ) -> Result<(SourceKey, Link, Cow<'static, Value>), SourceError> {
        let fragment = uri.fragment_decoded_lossy().unwrap_or_default();

        if !fragment.is_empty() {
            return Err(SourceError::UnexpectedUriFragment {
                uri: uri.clone(),
                backtrace: Backtrace::capture(),
            });
        }
        match self.index.entry(uri.clone()) {
            Entry::Occupied(_) => self.check_and_get_occupied(uri, src),
            Entry::Vacant(_) => self.insert_vacant(uri, src),
        }
    }

    fn insert_link(&mut self, uri: AbsoluteUri, link: Link) -> &mut Link {
        self.index.entry(uri).or_insert(link)
    }

    fn get(&self, key: SourceKey) -> &Value {
        self.table.get(key).unwrap()
    }

    // fn get_by_uri(&self, uri: &AbsoluteUri) -> Option<&Value> {
    //     self.index
    //         .get(uri)
    //         .map(|link| self.table.get(link.key).unwrap())
    //         .map(AsRef::as_ref)
    // }

    fn get_link(&self, uri: &AbsoluteUri) -> Option<&Link> {
        self.index.get(uri)
    }

    fn link_entry(&mut self, uri: AbsoluteUri) -> Entry<'_, AbsoluteUri, Link> {
        self.index.entry(uri)
    }

    // fn get_mut(&mut self, key: SourceKey) -> &Value {
    //     self.table.get_mut(key).unwrap()
    // }
}

#[derive(Clone, Debug, Default)]
pub struct Sources {
    store: Store,
    pub(crate) sandbox: Option<Store>,
}

impl Sources {
    pub fn print_index(&self) {
        for (uri, link) in &self.store.index {
            println!("  {uri}: {link:?}");
        }
    }

    /// Returns a new [`Sources`] instance.
    ///
    /// # Errors
    ///
    /// Returns a [`SourceError`] if:
    /// - a [`Source`]'s [`AbsoluteUri`] has a fragment.
    /// - duplicate [`Source`]s are provided with the same [`AbsoluteUri`].
    /// - all [`Deserializer`]s in `deserializers` fail to deserialize a [`Source`].
    pub(crate) fn new(
        sources: Vec<Input>,
        deserializers: &Deserializers,
    ) -> Result<Self, SourceError> {
        let mut store = Store::default();

        let iter = sources.into_iter();
        for src in iter {
            let uri = src.uri().clone(); // need the uri below for the error context
            let src = src
                .deserialize_or_take_value(deserializers)
                .map_err(|source| SourceError::DeserializationFailed {
                    uri: uri.clone(),
                    source,
                    backtrace: Backtrace::capture(),
                })?;

            store.insert(uri, src)?;
        }
        Ok(Self {
            store,
            sandbox: None,
        })
    }
    pub(crate) fn start_txn(&mut self) {
        assert!(self.sandbox.is_none(), "source sandbox already exists\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new");
        self.sandbox = Some(self.store.clone());
    }
    /// Accepts the current transaction, committing all changes.
    pub(crate) fn commit_txn(&mut self) {
        let sandbox = self.sandbox.take().expect(SANDBOX_ERR);
        self.store = sandbox;
    }
    /// Rejects the current transaction, discarding all changes.
    pub(crate) fn rollback_txn(&mut self) {
        self.sandbox = None;
    }

    pub(crate) fn insert_link(&mut self, uri: AbsoluteUri, link: Link) -> Result<&Link, LinkError> {
        match self.store_mut().link_entry(uri.clone()) {
            Entry::Occupied(_) => self.check_existing_link(&uri, link),
            Entry::Vacant(_) => self.create_link(uri, link),
        }
    }

    pub(crate) fn link_existing(
        &mut self,
        from: AbsoluteUri,
        to: &Link,
    ) -> Result<&Link, LinkError> {
        match self.store_mut().link_entry(from.clone()) {
            Entry::Occupied(_) => self.check_existing_link(&from, to.clone()),
            Entry::Vacant(_) => self.create_link(from.clone(), to.clone()),
        }
    }

    pub(crate) fn link_all(&mut self, from: &[AbsoluteUri], to: &Link) -> Result<(), LinkError> {
        for from_uri in from {
            self.link_existing(from_uri.clone(), to)?;
        }
        Ok(())
    }

    // pub(crate) fn get_link(&self, uri: &AbsoluteUri) -> Option<&Link> {
    //     self.store().get_link(uri)
    // }

    pub(crate) async fn resolve(
        &mut self,
        uri: &AbsoluteUri,
        resolvers: &Resolvers,
        deserializers: &Deserializers,
    ) -> Result<(&Link, &Value), SourceError> {
        let entry = self.store_mut().link_entry(uri.clone());
        match entry {
            // if the value has already been indexed, return it
            Entry::Occupied(_) => self.resolve_internal(uri),
            Entry::Vacant(_) => {
                self.resolve_ptr_or_external(uri, resolvers, deserializers)
                    .await
            }
        }
    }

    pub(crate) fn get(&self, key: SourceKey) -> &Value {
        self.store().get(key)
    }

    // #[must_use]
    // pub(crate) fn get_by_uri(&self, uri: &AbsoluteUri) -> Option<&Value> {
    //     self.store().get_by_uri(uri)
    // }

    pub(crate) fn insert_value(
        &mut self,
        uri: AbsoluteUri,
        src: Cow<'static, Value>,
    ) -> Result<(SourceKey, Link, Cow<'static, Value>), SourceError> {
        self.store_mut().insert(uri, src)
    }

    pub(crate) fn insert_string(
        &mut self,
        uri: AbsoluteUri,
        source: String,
        deserializers: &Deserializers,
    ) -> Result<(SourceKey, Link, Cow<'static, Value>), SourceError> {
        let src = deserializers.deserialize(&source).map_err(|source| {
            SourceError::DeserializationFailed {
                uri: uri.clone(),
                source,
                backtrace: Backtrace::capture(),
            }
        })?;
        self.insert_value(uri, Cow::Owned(src))
    }

    fn check_existing_link(&mut self, uri: &AbsoluteUri, link: Link) -> Result<&Link, LinkError> {
        let entry = self.store().get_link(uri).unwrap();
        ensure!(
            entry == &link,
            SourceConflictSnafu {
                uri: uri.clone(),
                existing_path: entry.src_path.clone(),
                new_path: link.src_path,
            }
        );
        Ok(entry)
    }

    fn create_link(&mut self, from: AbsoluteUri, link: Link) -> Result<&Link, LinkError> {
        match self.store_mut().link_entry(from.clone()) {
            Entry::Occupied(_) => {
                let existing_link = self.store().get_link(&from).unwrap();
                ensure!(
                    &link == existing_link,
                    SourceConflictSnafu {
                        uri: from.clone(),
                        existing_path: existing_link.src_path.clone(),
                        new_path: link.src_path,
                    }
                );
                Ok(existing_link)
            }
            Entry::Vacant(_) => {
                self.store_mut().insert_link(from.clone(), link);
                Ok(self.store().get_link(&from).unwrap())
            }
        }
    }

    fn store_mut(&mut self) -> &mut Store {
        self.sandbox.as_mut().expect(SANDBOX_ERR)
    }
    fn store(&self) -> &Store {
        if let Some(sandbox) = self.sandbox.as_ref() {
            return sandbox;
        }
        &self.store
    }
    async fn resolve_ptr(
        &mut self,
        uri: &AbsoluteUri,
        base_uri: &AbsoluteUri,
        fragment: &str,
        resolvers: &Resolvers,
        deserializers: &Deserializers,
    ) -> Result<(&Link, &Value), SourceError> {
        let link = if let Some(link) = self.store().get_link(base_uri).cloned() {
            link
        } else {
            let (link, _) = self
                .resolve_external(uri, base_uri, fragment, resolvers, deserializers)
                .await?;
            link.clone()
        };
        let Link {
            src_key,
            mut src_path,
        } = link;

        let fragment = Pointer::parse(fragment)?;
        src_path.append(&fragment);
        self.create_link(uri.clone(), Link::new(src_key, src_path.clone()))?;
        let src = self
            .store()
            .get(src_key)
            .resolve(&src_path)
            .map_err(PointerError::from)?;
        let link = self.store().get_link(uri).unwrap();
        Ok((link, src))
    }

    async fn resolve_ptr_or_external(
        &mut self,
        uri: &AbsoluteUri,
        resolvers: &Resolvers,
        deserializers: &Deserializers,
    ) -> Result<(&Link, &Value), SourceError> {
        let mut base_uri = uri.clone();
        let fragment = base_uri.set_fragment(None).unwrap().unwrap_or_default();
        let fragment = decode_lossy(&fragment);
        if fragment.starts_with('/') {
            self.resolve_ptr(uri, &base_uri, &fragment, resolvers, deserializers)
                .await
        } else {
            self.resolve_external(uri, &base_uri, &fragment, resolvers, deserializers)
                .await
        }
    }

    async fn resolve_external(
        &mut self,
        uri: &AbsoluteUri,
        base_uri: &AbsoluteUri,
        fragment: &str,
        resolvers: &Resolvers,
        deserializers: &Deserializers,
    ) -> Result<(&Link, &Value), SourceError> {
        let resolved = resolvers.resolve(base_uri).await?;

        let src = deserializers.deserialize(&resolved).map_err(|source| {
            SourceError::DeserializationFailed {
                uri: uri.clone(),
                source,
                backtrace: Backtrace::capture(),
            }
        })?;

        self.store_mut()
            .insert_vacant(base_uri.clone(), Cow::Owned(src))?;

        if fragment.is_empty() || !fragment.starts_with('/') {
            let link = self.store().get_link(base_uri).unwrap();
            let src = self.store().get(link.src_key);
            return Ok((link, src));
        }

        let ptr = Pointer::parse(fragment).map_err(PointerError::from)?;

        let mut link = self.store().get_link(base_uri).unwrap().clone();
        link.src_path.append(&ptr);
        let src_key = link.src_key;
        let src_path = link.src_path.clone();
        self.store_mut().insert_link(uri.clone(), link);
        let src = self
            .store()
            .get(src_key)
            .resolve(&src_path)
            .map_err(PointerError::from)?;
        let link = self.store().get_link(uri).unwrap();

        Ok((link, src))
    }
    fn resolve_internal(&self, uri: &AbsoluteUri) -> Result<(&Link, &Value), SourceError> {
        let link = self.store().get_link(uri).unwrap();
        let mut src = self.store().get(link.src_key);
        if !link.src_path.is_empty() {
            src = src.resolve(&link.src_path).map_err(PointerError::from)?;
        }
        Ok((link, src))
    }
}

/// A trait implemented by functions or types which can deserialize data into a [`Value`].
///
///
/// Three implementations are provided:
/// - [`deserialize_json`](`deserialize_json`): Deserializes JSON data. Always enabled.
/// - [`deserialize_yaml`](`deserialize_yaml`): Deserializes YAML data. Enabled with the `"yaml"` feature.
/// - [`deserialize_toml`](`deserialize_toml`): Deserializes TOML data. Enabled with the `"toml"` feature.
/// # Example
/// Implementing a custom deserializer for a format that has serde integration
/// is straightforward. `Deserializer` is implemented for `Fn(&str) ->
/// Result<Value, erased_serde::Error>`:
/// ```rust
/// pub fn deserialize_yaml(data: &str) -> Result<serde_json::Value, erased_serde::Error> {
///     use erased_serde::Deserializer;
///     let yaml = serde_yaml::Deserializer::from_str(data);
///     erased_serde::deserialize(&mut <dyn Deserializer>::erase(yaml))
/// }
/// ```
pub trait Deserializer: DynClone + Send + Sync + 'static {
    /// Deserializes the given data into a [`Value`].
    fn deserialize(&self, data: &str) -> Result<Value, erased_serde::Error>;
}

clone_trait_object!(Deserializer);
impl<F> Deserializer for F
where
    F: Fn(&str) -> Result<Value, erased_serde::Error> + Clone + Send + Sync + 'static,
{
    fn deserialize(&self, data: &str) -> Result<Value, erased_serde::Error> {
        self(data)
    }
}

/// A collection of [`Deserializer`]s.
#[derive(Clone)]
pub struct Deserializers {
    deserializers: Vec<(&'static str, Box<dyn Deserializer>)>,
}

impl std::fmt::Debug for Deserializers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.deserializers.iter().map(|(name, _)| name))
            .finish()
    }
}

impl Deserializers {
    /// Returns a new [`Deserializers`] instance.
    #[must_use]
    pub fn new(mut deserializers: Vec<(&'static str, Box<dyn Deserializer>)>) -> Self {
        if !deserializers
            .iter()
            .any(|(name, _)| name.to_lowercase() == "json")
        {
            deserializers.push(("json", Box::new(deserialize_json)));
        }
        Self { deserializers }
    }
    /// Attempts to deserialize the given data into a [`Value`], trying each
    /// [`Deserializer`] until one is successful.
    pub fn deserialize(&self, data: &str) -> Result<Value, DeserializeError> {
        let mut errs = HashMap::new();
        for (name, deserializer) in &self.deserializers {
            match deserializer.deserialize(data) {
                Ok(value) => return Ok(value),
                Err(err) => {
                    errs.insert(*name, err);
                }
            }
        }
        Err(DeserializeError { sources: errs })
    }
}
impl Deref for Deserializers {
    type Target = [(&'static str, Box<dyn Deserializer>)];

    fn deref(&self) -> &Self::Target {
        &self.deserializers
    }
}
/// Deserializes JSON data.
pub fn deserialize_json(data: &str) -> Result<Value, erased_serde::Error> {
    use erased_serde::Deserializer;
    let mut json = serde_json::Deserializer::from_str(data);
    let data = &mut <dyn Deserializer>::erase(&mut json);
    erased_serde::deserialize(data)
}

/// Deserializes YAML data
#[cfg(feature = "yaml")]
pub fn deserialize_yaml(data: &str) -> Result<Value, erased_serde::Error> {
    use erased_serde::Deserializer;
    let yaml = serde_yaml::Deserializer::from_str(data);
    erased_serde::deserialize(&mut <dyn Deserializer>::erase(yaml))
}
/// Deserializes TOML data
#[cfg(feature = "toml")]
pub fn deserialize_toml(data: &str) -> Result<Value, erased_serde::Error> {
    use erased_serde::Deserializer;
    let toml = toml::Deserializer::new(data);
    erased_serde::deserialize(&mut <dyn Deserializer>::erase(toml))
}

/// A file reference
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Link {
    pub(crate) src_key: SourceKey,
    pub(crate) src_path: Pointer,
}

impl Link {
    pub(crate) fn new(src_key: SourceKey, src_path: Pointer) -> Self {
        Self { src_key, src_path }
    }
}
// impl From<&Link> for (AbsoluteUri, Pointer) {
//     fn from(value: &Link) -> Self {
//         (value.uri.clone(), value.src_path.clone())
//     }
// }

/// A trait which is capable of resolving a [`Source`] at the given [`AbsoluteUri`].
#[async_trait]
pub trait Resolve: DynClone + Send + Sync + 'static {
    /// Attempts to resolve a [`Source`] at the given `uri`
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Option<String>, ResolveError>;
}

clone_trait_object!(Resolve);

///
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
#[derive(Clone, Debug)]
pub struct HttpResolver {
    client: reqwest::Client,
}

#[cfg(feature = "http")]
/// A [`Resolve`] implementation that uses HTTP(S) to resolve schema sources.
impl HttpResolver {
    /// Returns a new [`HttpResolver`] instance.
    #[must_use]
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[cfg(feature = "http")]
#[async_trait]
impl Resolve for HttpResolver {
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Option<String>, ResolveError> {
        let Some(url) = uri.as_url() else {
            return Ok(None);
        };
        let scheme = url.scheme();
        if scheme != "http" && scheme != "https" {
            return Ok(None);
        }
        match self.client.get(url.clone()).send().await {
            Ok(resp) => {
                let text = resp
                    .text()
                    .await
                    .map_err(|err| ResolveError::new(err, uri.clone()))?;
                Ok(Some(text))
            }
            Err(err) if matches!(err.status(), Some(reqwest::StatusCode::NOT_FOUND)) => Ok(None),
            Err(err) => Err(ResolveError::new(err, uri.clone())),
        }
    }
}

/// A collection of [`Resolve`] implementations.
#[derive(Clone)]
pub struct Resolvers {
    resolvers: Vec<Box<dyn Resolve>>,
}

impl Resolvers {
    /// Returns a new [`Resolvers`] instance.
    #[must_use]
    pub fn new(resolvers: Vec<Box<dyn Resolve>>) -> Self {
        Self { resolvers }
    }
    /// Tries to resolve the given [`AbsoluteUri`] by attempting with each [`Resolve`] until
    /// one is successful.
    pub async fn resolve(&self, uri: &AbsoluteUri) -> Result<String, ResolveErrors> {
        let mut errors = ResolveErrors::default();
        for resolver in &self.resolvers {
            match resolver.resolve(uri).await {
                Ok(Some(data)) => {
                    return Ok(data);
                }
                Err(err) => errors.push(err),
                _ => continue,
            }
        }
        if errors.is_empty() {
            errors.push_not_found(uri.clone());
        }
        Err(errors)
    }
    /// Returns an [`Iterator`] over the [`Resolve`] implementations in this
    /// [`Resolvers`] instance.
    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Resolve>> {
        self.resolvers.iter()
    }
}
impl<'a> IntoIterator for &'a Resolvers {
    type Item = &'a Box<dyn Resolve>;
    type IntoIter = std::slice::Iter<'a, Box<dyn Resolve>>;

    fn into_iter(self) -> Self::IntoIter {
        self.resolvers.iter()
    }
}

#[cfg(test)]
mockall::mock! {
    pub Resolver{}

    #[async_trait]
    impl Resolve for Resolver {
        async fn resolve(&self, uri: &AbsoluteUri) -> Result<Option<String>, ResolveError>;
    }
    impl Clone for Resolver {
        fn clone(&self) -> Self;
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_resolve() {
        //=============================================================\\
        //                           absolute                          \\
        //=============================================================\\

        let get_value = || {
            json!({
                "foo": {
                    "bar": {
                        "baz": "qux"
                    }
                }
            })
        };
        let value = get_value();
        let mut sources = Sources::default();
        let mut resolver = MockResolver::new();

        resolver
            .expect_resolve()
            .returning(move |_| Ok(Some(get_value().to_string())));

        let resolvers = Resolvers::new(vec![Box::new(resolver)]);

        let uri: AbsoluteUri = "https://example.com/foo".parse().unwrap();
        let base_uri = uri.clone();
        let deserializers = Deserializers::new(vec![]);
        sources.start_txn();

        let (link, src) = sources
            .resolve(&uri, &resolvers, &deserializers)
            .await
            .unwrap();
        assert_eq!(src, &get_value());
        assert_eq!(link.src_path, Pointer::default());

        //=============================================================\\
        //                           pointer                           \\
        //=============================================================\\

        let mut sources = Sources::default();
        sources.start_txn();
        let mut uri: AbsoluteUri = base_uri.clone();
        uri.set_fragment(Some("/foo")).unwrap();
        let result = sources.resolve(&uri, &resolvers, &deserializers).await;
        assert!(result.is_ok(), "expected Ok, got {result:?}");
        let (link, src) = result.unwrap();

        assert_eq!(src, &value["foo"]);
        assert_eq!(link.src_path, Pointer::parse("/foo").unwrap());
        assert_eq!(sources.store_mut().index.len(), 2);
        assert_eq!(sources.store_mut().table.len(), 1);
        let (link, src) = sources
            .resolve(&base_uri, &resolvers, &deserializers)
            .await
            .unwrap();
        assert_eq!(src, &value);
        assert_eq!(link.src_path, Pointer::default());
        assert_eq!(sources.store_mut().index.len(), 2);
        assert_eq!(sources.store_mut().table.len(), 1);

        //=============================================================\\
        //                           anchor                            \\
        //=============================================================\\

        let mut sources = Sources::default();
        sources.start_txn();
        let mut uri = base_uri.clone();
        uri.set_fragment(Some("foo")).unwrap();

        let (link, src) = sources
            .resolve(&uri, &resolvers, &deserializers)
            .await
            .unwrap();
        assert_eq!(link.src_path, Pointer::default());
        assert_eq!(src, &value);
        assert_eq!(sources.store_mut().index.len(), 1);
        assert_eq!(sources.store_mut().table.len(), 1);
    }
}
