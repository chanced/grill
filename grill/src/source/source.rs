use super::{Deserializers, Resolvers};
use crate::{
    error::{
        DeserializationError, DeserializeError, ResolveError, SourceConflictError, SourceError,
    },
    schema::Metaschema,
    source::Deserializer,
    uri::AbsoluteUri,
};
use jsonptr::{Pointer, Resolve};
use serde_json::Value;
use std::{
    borrow::{Borrow, Cow},
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

pub(crate) enum Source {
    String(AbsoluteUri, String),
    Value(AbsoluteUri, Value),
}

impl Source {
    pub(crate) fn try_take_value(
        self,
        deserializers: &[(&'static str, Box<dyn Deserializer>)],
    ) -> Result<Value, DeserializeError> {
        match self {
            Self::Value(_uri, val) => Ok(val),
            Self::String(_uri, str) => {
                let val = deserialize(&str, deserializers)?;
                Ok(val)
            }
        }
    }
    #[must_use]
    pub fn uri(&self) -> &AbsoluteUri {
        match self {
            Self::Value(uri, _) | Self::String(uri, _) => uri,
        }
    }

    pub fn value(
        &self,
        deserializers: &[(&'static str, Box<dyn Deserializer>)],
    ) -> Result<Cow<Value>, DeserializeError> {
        match self {
            Self::String(_, s) => Ok(Cow::Owned(deserialize(s, deserializers)?)),
            Self::Value(_, source) => Ok(Cow::Borrowed(source)),
        }
    }

    /// If
    #[must_use]
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(_, s) => Some(s),
            Self::Value(_, _) => None,
        }
    }
}

impl From<&Metaschema> for Source {
    fn from(value: &Metaschema) -> Self {
        Self::Value(value.id.clone(), value.schema.clone().into())
    }
}
fn deserialize(
    str: &str,
    deserializers: &[(&'static str, Box<dyn Deserializer>)],
) -> Result<Value, DeserializeError> {
    let mut source = None;
    let mut errs: HashMap<&'static str, erased_serde::Error> = HashMap::new();
    for (fmt, de) in deserializers {
        match de.deserialize(str) {
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
#[derive(Clone, Debug)]
pub(crate) struct Sources {
    store: HashMap<AbsoluteUri, Value>,
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
        sources: Vec<Source>,
        deserializers: &[(&'static str, Box<dyn Deserializer>)],
    ) -> Result<Self, SourceError> {
        let mut store = HashMap::with_capacity(sources.len());
        let iter = sources.into_iter();
        for src in iter {
            let uri = src.uri().clone(); // need the uri below for the error context
            let src = src
                .try_take_value(deserializers)
                .map_err(|e| DeserializationError {
                    uri: uri.clone(),
                    error: e,
                })?;

            let fragment = uri.fragment();
            if fragment.is_some() && fragment != Some("") {
                return Err(SourceError::FragmentedUri(uri));
            }
            store.insert(uri, src);
        }
        Ok(Self { store })
    }

    pub(crate) async fn resolve(
        &mut self,
        uri: &AbsoluteUri,
        resolvers: &Resolvers,
        deserializers: &Deserializers,
    ) -> Result<(Pointer, &Value), SourceError> {
        // if the value has already been indexed, return a clone of the local copy
        if self.store.contains_key(uri) {
            return Ok((Pointer::default(), self.store.get(uri).unwrap()));
        }

        // checking to see if the root resource has already been stored
        let mut base_uri = uri.clone();
        base_uri.set_fragment(None).unwrap();

        // resolving the base uri
        let resolved = resolvers.resolve(&base_uri).await?;

        // add the base value to the local store of sources
        let root = self.source_string(base_uri, resolved, deserializers)?;

        let uri_fragment = uri.fragment().unwrap_or_default();

        // if the uri does not have a fragment, we are done and can return the
        // root-level schema
        if uri_fragment.is_empty() {
            return Ok((Pointer::default(), &root));
        }

        // there is more work to do if the uri has a fragmen.
        // first, perform lookup again to see if add_source indexed the schema
        if self.store.contains_key(uri) {
            return Ok((Pointer::default(), self.store.get(uri).unwrap()));
        }

        // if not, the fragment must be a json pointer as all anchors and
        // schemas with fragmented ids should have been located and indexed
        // TODO: better error handling here.
        let ptr =
            Pointer::from_str(uri_fragment).map_err(|err| ResolveError::new(err, uri.clone()))?;

        let value = root.resolve(&ptr).map_err(|err| {
            SourceError::ResolutionFailed(ResolveError::new(err, uri.clone()).into())
        })?;
        todo!()
        // Ok((ptr, value))
    }

    #[must_use]
    pub fn get(&self, uri: &AbsoluteUri) -> Option<&Value> {
        self.store.get(uri)
    }
    pub(crate) fn source_value(
        &mut self,
        uri: AbsoluteUri,
        source: Value,
        deserializers: &Deserializers,
    ) -> Result<&Value, SourceError> {
        self.insert(Source::Value(uri, source), deserializers)
    }

    pub(crate) fn source_string(
        &mut self,
        uri: AbsoluteUri,
        source: String,
        deserializers: &Deserializers,
    ) -> Result<&Value, SourceError> {
        self.insert(Source::String(uri, source), deserializers)
    }

    pub(crate) fn insert(
        &mut self,
        source: Source,
        deserializers: &Deserializers,
    ) -> Result<&Value, SourceError> {
        let uri = source.uri().clone();
        let frag = uri.fragment();
        if frag.is_some() && frag != Some("") {
            return Err(SourceError::FragmentedUri(uri));
        }
        let source = source
            .value(deserializers)
            .map_err(|e| DeserializationError {
                uri: uri.clone(),
                error: e,
            })?;

        if self.store.contains_key(&uri) {
            return self.get_unchecked(&uri, &source);
        }
        Ok(self.insert_value(uri, source.into_owned())?)
    }
    /// Resolves a source [`&Value`](`Value`) from the internal hashmap.
    ///
    /// # Errors
    /// Returns a [`SourceConflictError`] if the `uri` is present in the
    /// internal hashmap and `source` does not match.
    ///
    /// # Panics
    /// This function panics if the `uri` is not present in the internal
    /// hashmap.
    fn get_unchecked(&self, uri: &AbsoluteUri, source: &Value) -> Result<&Value, SourceError> {
        if self.store.get(uri).unwrap() != source {
            // error out if a source with the same uri has been indexed and the
            // values do not match
            return Err(SourceConflictError {
                uri: uri.clone(),
                value: source.clone().into(),
            }
            .into());
        }

        Ok(self.store.get(uri).unwrap())
    }

    fn insert_value(
        &mut self,
        uri: AbsoluteUri,
        source: Value,
    ) -> Result<&Value, SourceConflictError> {
        if self.store.contains_key(&uri) {
            // safe, checked above
            let src = self.store.get(&uri).unwrap();
            if src == &source {
                return Ok(src);
            }
            // error out if a source with the same uri has been indexed and the
            // values do not match
            return Err(SourceConflictError {
                uri,
                value: source.into(),
            });
        }
        Ok(self.store.entry(uri).or_insert(source))
    }
    pub fn entry(&mut self, key: AbsoluteUri) -> Entry<'_, AbsoluteUri, Value> {
        self.store.entry(key)
    }

    #[must_use]
    pub fn contains(&self, uri: &AbsoluteUri) -> bool {
        self.store.contains_key(uri)
    }
}
