use super::{Deserializers, Link, Resolvers};
use crate::error::{
    DeserializationError, LinkConflictError, LinkError, PointerError, SourceConflictError,
};
use crate::{
    error::{DeserializeError, SourceError},
    uri::AbsoluteUri,
};

use ahash::AHashMap;
use jsonptr::{Pointer, Resolve};
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};
use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::convert::AsRef;
use std::ops::Deref;

const SANDBOX_ERR: &str = "transaction failed: source sandbox not found.\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new";

new_key_type! {
    pub struct SourceKey;
}

pub(crate) enum Src {
    String(AbsoluteUri, String),
    Value(AbsoluteUri, Cow<'static, Value>),
}

impl Src {
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
    pub uri: Cow<'i, AbsoluteUri>,
    pub path: Cow<'i, Pointer>,
    pub value: Cow<'i, Value>,
}

impl<'i> Source<'i> {
    pub(crate) fn new(src: &'i Link, sources: &'i Sources) -> Source<'i> {
        let mut value = sources.get(src.key);
        if !src.path.is_empty() {
            value = value.resolve(&src.path).unwrap();
        }
        Self {
            key: src.key,
            uri: Cow::Borrowed(&src.uri),
            path: Cow::Borrowed(&src.path),
            value: Cow::Borrowed(value),
        }
    }
    #[must_use]
    pub fn into_owned(&self) -> Source<'static> {
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
struct Store {
    table: SlotMap<SourceKey, Cow<'static, Value>>,
    index: AHashMap<AbsoluteUri, Link>,
}
impl Store {
    fn check_and_get_occupied(
        &self,
        uri: AbsoluteUri,
        src: Cow<'static, Value>,
    ) -> Result<(SourceKey, Link, Cow<'static, Value>), SourceError> {
        let key = self.index.get(&uri).unwrap().key;
        let existing_src = self.table.get(key).unwrap().clone();
        if src != existing_src {
            println!("src: {}", serde_json::to_string_pretty(&src).unwrap());
            println!(
                "existing_src: {}",
                serde_json::to_string_pretty(&existing_src).unwrap()
            );

            return Err(SourceConflictError { uri: uri.clone() }.into());
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
        let key = self.table.insert(src);
        let src = self.table.get(key).unwrap().clone();
        self.index.insert(
            base_uri.clone(),
            Link::new(key, base_uri.clone(), Pointer::default()),
        );
        let link = self.index.get(&base_uri).unwrap().clone();
        if fragment.trim().is_empty() {
            return Ok((key, link, src));
        }
        if fragment.starts_with('/') {
            let ptr = Pointer::parse(&fragment).map_err(PointerError::from)?;
            self.index
                .insert(uri.clone(), Link::new(key, uri.clone(), ptr.clone()));
            let key = self.index.get(&uri).unwrap().key;
            let src = src.resolve(&ptr).map_err(PointerError::from)?.clone();
            return Ok((key, link, Cow::Owned(src)));
        }
        Ok((key, link, src))
    }

    fn insert(
        &mut self,
        uri: AbsoluteUri,
        src: Cow<'static, Value>,
    ) -> Result<(SourceKey, Link, Cow<'static, Value>), SourceError> {
        let fragment = uri.fragment().unwrap_or_default();
        if !fragment.trim().is_empty() {
            return Err(SourceError::UnexpectedUriFragment(uri.clone()));
        }
        match self.index.entry(uri.clone()) {
            Entry::Occupied(_) => self.check_and_get_occupied(uri, src),
            Entry::Vacant(_) => self.insert_vacant(uri, src),
        }
    }

    fn insert_link(&mut self, key: SourceKey, uri: AbsoluteUri, path: Pointer) -> &mut Link {
        self.index
            .entry(uri.clone())
            .or_insert(Link::new(key, uri, path))
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
pub(crate) struct Sources {
    store: Store,
    sandbox: Option<Store>,
}

impl Sources {
    /// Returns a new [`Sources`] instance.
    ///
    /// # Errors
    ///
    /// Returns a [`SourceError`] if:
    /// - a [`Source`]'s [`AbsoluteUri`] has a fragment.
    /// - duplicate [`Source`]s are provided with the same [`AbsoluteUri`].
    /// - all [`Deserializer`]s in `deserializers` fail to deserialize a [`Source`].
    pub(crate) fn new(
        sources: Vec<Src>,
        deserializers: &Deserializers,
    ) -> Result<Self, SourceError> {
        let mut store = Store::default();
        let iter = sources.into_iter();
        for src in iter {
            let uri = src.uri().clone(); // need the uri below for the error context
            let src = src
                .deserialize_or_take_value(deserializers)
                .map_err(|error| DeserializationError {
                    uri: uri.clone(),
                    error,
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

    pub(crate) fn link(
        &mut self,
        from: AbsoluteUri,
        to: AbsoluteUri,
        path: Pointer,
    ) -> Result<&Link, LinkError> {
        let key = self
            .store_mut()
            .get_link(&to)
            .ok_or_else(|| LinkError::NotFound(to.clone()))?
            .key;
        let link = Link::new(key, to, path.clone());

        match self.store_mut().link_entry(from.clone()) {
            Entry::Occupied(_) => self.check_existing_link(link),
            Entry::Vacant(_) => self.create_link(from, link),
        }
    }

    pub(crate) fn link_all(
        &mut self,
        primary: Option<&AbsoluteUri>,
        from: &[AbsoluteUri],
        to: &Link,
    ) -> Result<(), LinkError> {
        let to_uri = to.uri.clone();
        let to_path = to.path.clone();
        if let Some(primary) = primary {
            self.link(primary.clone(), to_uri.clone(), to_path.clone())?;
        }
        for uri in from {
            self.link(uri.clone(), to_uri.clone(), to_path.clone())?;
        }
        Ok(())
    }

    // pub(crate) fn get_link(&self, uri: &AbsoluteUri) -> Option<&Link> {
    //     self.store().get_link(uri)
    // }

    pub(crate) async fn resolve_link(
        &mut self,
        uri: AbsoluteUri,
        resolvers: &Resolvers,
        deserializers: &Deserializers,
    ) -> Result<Link, SourceError> {
        let (link, _) = self.resolve(&uri, resolvers, deserializers).await?;
        Ok(link.clone())
    }

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
        self.store.get(key)
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
        let src = deserializers
            .deserialize(&source)
            .map_err(|e| DeserializationError::new(uri.clone(), e))?;
        self.insert_value(uri, Cow::Owned(src))
    }

    fn check_existing_link(&mut self, link: Link) -> Result<&Link, LinkError> {
        let entry = self.store().get_link(&link.uri).unwrap();
        if &link == entry {
            return Ok(entry);
        }
        Err(LinkConflictError {
            existing: entry.into(),
            new: (&link).into(),
        }
        .into())
    }

    fn create_link(&mut self, from: AbsoluteUri, link: Link) -> Result<&Link, LinkError> {
        match self.store_mut().link_entry(link.root_uri.clone()) {
            Entry::Occupied(_) => {
                let root = self.store().get_link(&link.root_uri).unwrap();
                let root_src = self.store().get(root.key);
                root_src.resolve(&link.path)?;
                let link = self.store_mut().link_entry(from).or_insert(link);
                Ok(link)
            }
            Entry::Vacant(_) => Err(LinkError::NotFound(link.root_uri.clone())),
        }
    }

    fn sandbox(&mut self) -> &mut Store {
        self.sandbox.as_mut().expect(SANDBOX_ERR)
    }
    fn store_mut(&mut self) -> &mut Store {
        self.sandbox()
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
        let link = if let Some(link) = self.store().get_link(base_uri) {
            link
        } else {
            let (link, _) = self
                .resolve_external(uri, base_uri, fragment, resolvers, deserializers)
                .await?;
            link
        }
        .clone();

        let src = self.store().get(link.key).clone();
        let ptr = Pointer::parse(fragment)?;
        ptr.resolve(&src).map_err(PointerError::from)?;
        self.create_link(uri.clone(), Link::new(link.key, uri.clone(), ptr.clone()))?;
        let src = self
            .store()
            .get(link.key)
            .resolve(&ptr)
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

        let src = deserializers
            .deserialize(&resolved)
            .map_err(|e| DeserializationError::new(base_uri.clone(), e))?;

        self.store_mut()
            .insert_vacant(base_uri.clone(), Cow::Owned(src))?;

        if fragment.is_empty() || !fragment.starts_with('/') {
            let link = self.store().get_link(base_uri).unwrap();
            let src = self.store().get(link.key);
            return Ok((link, src));
        }

        let ptr = Pointer::parse(fragment).map_err(PointerError::from)?;

        let link = self.store().get_link(base_uri).unwrap().clone();

        self.store_mut()
            .insert_link(link.key, uri.clone(), ptr.clone());

        let src = self
            .store()
            .get(link.key)
            .resolve(&ptr)
            .map_err(PointerError::from)?;
        let link = self.store().get_link(uri).unwrap();

        Ok((link, src))
    }
    fn resolve_internal(&self, uri: &AbsoluteUri) -> Result<(&Link, &Value), SourceError> {
        let link = self.store().get_link(uri).unwrap();
        let mut src = self.store().get(link.key);
        if !link.path.is_empty() {
            src = src.resolve(&link.path).map_err(PointerError::from)?;
        }
        Ok((link, src))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::source::resolve::MockResolver;

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
        assert_eq!(link.path, Pointer::default());

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
        assert_eq!(link.path, Pointer::parse("/foo").unwrap());
        assert_eq!(sources.store_mut().index.len(), 2);
        assert_eq!(sources.store_mut().table.len(), 1);
        let (link, src) = sources
            .resolve(&base_uri, &resolvers, &deserializers)
            .await
            .unwrap();
        assert_eq!(src, &value);
        assert_eq!(link.path, Pointer::default());
        assert_eq!(link.uri, base_uri);
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
        assert_eq!(link.path, Pointer::default());
        assert_eq!(link.uri, base_uri);
        assert_eq!(src, &value);
        assert_eq!(sources.store_mut().index.len(), 1);
        assert_eq!(sources.store_mut().table.len(), 1);
    }
}
