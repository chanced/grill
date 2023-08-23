use super::{Deserializers, Link, Resolvers};
use crate::error::{DeserializationError, LinkConflictError, LinkError, SourceConflictError};
use crate::schema::CompiledSchema;
use crate::Key;
use crate::{
    error::{DeserializeError, SourceError},
    schema::Metaschema,
    uri::AbsoluteUri,
};
use jsonptr::{Pointer, Resolve};
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};
use std::borrow::Cow;
use std::collections::hash_map::{Entry, HashMap};
use std::ops::Deref;

new_key_type! {
    pub struct SourceKey;
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
        let value = sources.store.get(src.key).unwrap();
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

pub(crate) enum SrcValue {
    String(AbsoluteUri, String),
    Value(AbsoluteUri, Value),
}

impl SrcValue {
    pub(crate) fn deserialize_or_take_value(
        self,
        deserializers: &Deserializers,
    ) -> Result<Value, DeserializeError> {
        match self {
            Self::Value(_uri, val) => Ok(val),
            Self::String(_uri, str) => {
                let src = deserializers.deserialize(&str)?;
                Ok(src)
            }
        }
    }
    #[must_use]
    pub fn uri(&self) -> &AbsoluteUri {
        match self {
            Self::Value(uri, _) | Self::String(uri, _) => uri,
        }
    }

    #[must_use]
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(_, s) => Some(s),
            Self::Value(_, _) => None,
        }
    }
}

impl From<&Metaschema> for SrcValue {
    fn from(value: &Metaschema) -> Self {
        Self::Value(value.id.clone(), value.schema.clone().into())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Sources {
    pub(crate) store: SlotMap<SourceKey, Value>,
    index: HashMap<AbsoluteUri, Link>,
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
        sources: Vec<SrcValue>,
        deserializers: &Deserializers,
    ) -> Result<Self, SourceError> {
        let mut store = SlotMap::with_capacity_and_key(sources.len());
        let iter = sources.into_iter();
        let mut index = HashMap::new();

        for src in iter {
            let uri = src.uri().clone(); // need the uri below for the error context
            let src = src
                .deserialize_or_take_value(deserializers)
                .map_err(|error| DeserializationError {
                    uri: uri.clone(),
                    error,
                })?;

            let fragment = uri.fragment();
            if fragment.is_some() && fragment != Some("") {
                return Err(SourceError::UnexpectedUriFragment(uri));
            }
            let key = store.insert(src);
            index.insert(uri.clone(), Link::new(key, uri, Pointer::default()));
        }

        Ok(Self { store, index })
    }

    pub(crate) fn link(
        &mut self,
        from: AbsoluteUri,
        to: AbsoluteUri,
        path: Pointer,
    ) -> Result<&Link, LinkError> {
        let key = self
            .index
            .get(&to)
            .ok_or_else(|| LinkError::NotFound(to.clone()))?
            .key;
        let link = Link::new(key, to, path.clone());
        match self.index.entry(from.clone()) {
            Entry::Occupied(_) => self.check_existing_link(link),
            Entry::Vacant(_) => self.try_create_link(from, link),
        }
    }
    fn check_existing_link(&mut self, link: Link) -> Result<&Link, LinkError> {
        let entry = self.index.get(&link.uri).unwrap();
        if &link == entry {
            return Ok(entry);
        }
        Err(LinkConflictError {
            existing: entry.into(),
            new: (&link).into(),
        }
        .into())
    }
    fn try_create_link(&mut self, from: AbsoluteUri, link: Link) -> Result<&Link, LinkError> {
        match self.index.entry(link.uri.clone()) {
            Entry::Occupied(root) => {
                let root = root.get();
                let root_src = self.store.get(root.key).unwrap();
                let _ = root_src.resolve(&link.path)?;
                let link = self.index.entry(from).or_insert(link);
                Ok(link)
            }
            Entry::Vacant(_) => Err(LinkError::NotFound(link.uri.clone())),
        }
    }

    pub(crate) fn resolve_local(&self, uri: &AbsoluteUri) -> Result<(&Link, &Value), SourceError> {
        let link = self.index.get(uri).unwrap();
        let mut src = self.store.get(link.key).unwrap();
        if !link.path.is_empty() {
            src = src.resolve(&link.path)?;
        }
        Ok((link, src))
    }

    pub(crate) async fn resolve_remote(
        &mut self,
        uri: &AbsoluteUri,
        resolvers: &Resolvers,
        deserializers: &Deserializers,
    ) -> Result<(&Link, &Value), SourceError> {
        let mut base_uri = uri.clone();
        let fragment = base_uri.set_fragment(None)?.unwrap_or_default();
        let resolved = resolvers.resolve(uri).await?;
        let src = deserializers
            .deserialize(&resolved)
            .map_err(|e| DeserializationError::new(base_uri.clone(), e))?;
        let key = self.store.insert(src);
        let src = self.store.get(key).unwrap();
        if !fragment.is_empty() {
            if !fragment.starts_with('/') {
                return Err(SourceError::UnexpectedUriFragment(uri.clone()));
            }
            let ptr = Pointer::parse(&fragment)
                .map_err(|_| SourceError::UnexpectedUriFragment(uri.clone()))?;
            let _ = src.resolve(&ptr)?;
            self.index
                .insert(uri.clone(), Link::new(key, base_uri.clone(), ptr));
        }
        let src = self.store.get(key).unwrap();
        let link = self.index.entry(base_uri.clone()).or_insert(Link::new(
            key,
            base_uri.clone(),
            Pointer::default(),
        ));
        Ok((link, src))
    }

    pub(crate) async fn resolve(
        &mut self,
        uri: &AbsoluteUri,
        resolvers: &Resolvers,
        deserializers: &Deserializers,
    ) -> Result<(&Link, &Value), SourceError> {
        // if the value has already been indexed, return it
        match self.index.entry(uri.clone()) {
            Entry::Occupied(_) => self.resolve_local(uri),
            Entry::Vacant(_) => self.resolve_remote(uri, resolvers, deserializers).await,
        }
    }

    pub fn get(&mut self, key: SourceKey) -> &Value {
        self.store.get(key).unwrap()
    }

    #[must_use]
    pub fn get_by_uri(&self, uri: &AbsoluteUri) -> Option<&Value> {
        self.index
            .get(uri)
            .map(|link| self.store.get(link.key).unwrap())
    }
    pub(crate) fn insert_value(
        &mut self,
        uri: AbsoluteUri,
        source: Value,
    ) -> Result<(SourceKey, &Value), SourceError> {
        if uri.fragment().is_some() && uri.fragment() != Some("") {
            return Err(SourceError::UnexpectedUriFragment(uri));
        }
        match self.index.entry(uri.clone()) {
            Entry::Occupied(entry) => {
                let key = entry.get().key;
                let src = self.store.get(key).unwrap();
                if src != &source {
                    return Err(SourceConflictError {
                        uri: uri.clone(),
                        value: source.clone().into(),
                    }
                    .into());
                }
                Ok((key, src))
            }
            Entry::Vacant(entry) => {
                let key = self.store.insert(source);
                entry.insert(Link::new(key, uri.clone(), Pointer::default()));
                let value = self.store.get(key).unwrap();
                Ok((key, value))
            }
        }
    }

    pub(crate) fn insert_string(
        &mut self,
        uri: AbsoluteUri,
        source: String,
        deserializers: &Deserializers,
    ) -> Result<(SourceKey,&Value), SourceError> {
        let src = deserializers
            .deserialize(&source)
            .map_err(|e| DeserializationError::new(uri.clone(), e))?;
        self.insert_value(uri, src)
    }

    #[must_use]
    pub fn contains(&self, uri: &AbsoluteUri) -> bool {
        self.index.contains_key(uri)
    }
}
