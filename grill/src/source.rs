use crate::{
    deserialize::Deserializers,
    dialect::Metaschema,
    error::{DeserializeError, SourceDeserializationError, SourceDuplicateError, SourceError},
    AbsoluteUri, Deserializer,
};
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
};

pub enum Source {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sources {
    sources: HashMap<AbsoluteUri, Value>,
}

impl Sources {
    /// Returns a new [`Sources`] instance.
    ///
    /// # Errors
    ///
    /// Returns a [`SourceError`] if any of the following conditions are met:
    /// - a [`Source`]'s [`AbsoluteUri`] has a fragment.
    /// - duplicate [`Source`]s are provided with the same [`AbsoluteUri`].
    /// - all [`Deserializer`]s in `deserializers` fail to deserialize a [`Source`].
    pub fn new(
        sources: Vec<Source>,
        deserializers: &[(&'static str, Box<dyn Deserializer>)],
    ) -> Result<Self, SourceError> {
        let capacity = sources.len();
        let iter = sources.into_iter();
        let mut sources: HashMap<AbsoluteUri, Value> = HashMap::with_capacity(capacity);
        for src in iter {
            let uri = src.uri().clone(); // need the uri below for the error context
            let src =
                src.try_take_value(deserializers)
                    .map_err(|e| SourceDeserializationError {
                        uri: uri.clone(),
                        error: e,
                    })?;
            if let Some(fragment) = uri.fragment() {
                if !fragment.is_empty() {
                    return Err(SourceError::FragmentedUri(uri));
                }
            }
            sources.insert(uri, src);
        }
        Ok(Self { sources })
    }
    #[must_use]
    pub fn get(&self, uri: &AbsoluteUri) -> Option<&Value> {
        self.sources.get(uri)
    }
    pub fn source_value(
        &mut self,
        uri: AbsoluteUri,
        source: Value,
        deserializers: &Deserializers,
    ) -> Result<&Value, SourceError> {
        self.source(Source::Value(uri, source), deserializers)
    }

    pub fn source_string(
        &mut self,
        uri: AbsoluteUri,
        source: String,
        deserializers: &Deserializers,
    ) -> Result<&Value, SourceError> {
        self.source(Source::String(uri, source), deserializers)
    }

    pub fn source(
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
            .map_err(|e| SourceDeserializationError {
                uri: uri.clone(),
                error: e,
            })?;

        if self.sources.contains_key(&uri) {
            // safe, see check above.
            let src = self.sources.get(&uri).unwrap();
            if src == source.as_ref() {
                return Ok(self.sources.get(&uri).unwrap());
            }

            // error out if a source with the same uri has been indexed and the
            // values do not match

            return Err(SourceDuplicateError {
                uri,
                value: source.into_owned().into(),
            }
            .into());
        }
        Ok(self.insert_value(uri, source.into_owned())?)
    }

    fn insert_value(
        &mut self,
        uri: AbsoluteUri,
        source: Value,
    ) -> Result<&Value, SourceDuplicateError> {
        if self.sources.contains_key(&uri) {
            // safe, checked above
            let src = self.sources.get(&uri).unwrap();
            if src == &source {
                return Ok(src);
            }
            // error out if a source with the same uri has been indexed and the
            // values do not match
            return Err(SourceDuplicateError {
                uri,
                value: source.into(),
            });
        }
        Ok(self.sources.entry(uri).or_insert(source))
    }
    pub fn entry(&mut self, key: AbsoluteUri) -> Entry<'_, AbsoluteUri, Value> {
        self.sources.entry(key)
    }

    #[must_use]
    pub fn contains(&self, uri: &AbsoluteUri) -> bool {
        self.sources.contains_key(uri)
    }
}
