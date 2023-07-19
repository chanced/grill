use crate::{
    deserialize::Deserializers,
    error::{
        new_sources_error, DeserializeError, DuplicateSourceError, FragmentedUriError,
        NewSourcesError, SourceError,
    },
    AbsoluteUri, Deserializer, Metaschema,
};
use serde_json::Value;
use snafu::ResultExt;
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
    pub fn uri(&self) -> &AbsoluteUri {
        match self {
            Self::Value(uri, _) | Self::String(uri, _) => &uri,
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
    pub fn new(
        sources: Vec<Source>,
        deserializers: &[(&'static str, Box<dyn Deserializer>)],
    ) -> Result<Self, NewSourcesError> {
        let capacity = sources.len();
        let iter = sources.into_iter();
        let mut sources: HashMap<AbsoluteUri, Value> = HashMap::with_capacity(capacity);
        for src in iter {
            let uri = src.uri().clone(); // need the uri below for the error context
            let src = src
                .try_take_value(deserializers)
                .context(new_sources_error::Deserialize { uri: uri.clone() })?;
            if let Some(fragment) = uri.fragment() {
                if !fragment.is_empty() {
                    return Err(FragmentedUriError { uri }.into());
                }
            }
            sources.insert(uri, src);
        }
        Ok(Self { sources })
    }
    pub fn get(&self, uri: &AbsoluteUri) -> Option<&Value> {
        self.sources.get(uri)
    }

    pub fn insert(
        &mut self,
        source: Source,
        deserializers: &Deserializers,
    ) -> Result<&Value, SourceError> {
        let uri = source.uri().clone();
        let frag = uri.fragment();
        if frag.is_some() && frag != Some("") {
            return Err(SourceError::FragmentedSourceUri {
                source: FragmentedUriError { uri },
            });
        }
        let source = source.value(&deserializers)?.into_owned();
        // if the source has already been indexed, no-op
        if self.sources.contains_key(&uri) {
            // safe, see check above.
            let src = self.sources.get(&uri).unwrap();
            if src == &source {
                return Ok(self.sources.get(&uri).unwrap());
            }
            // error out if a source with the same uri has been indexed and the
            // values do not match
            return Err(DuplicateSourceError { uri, source }.into());
        }
        Ok(self.insert_value(uri, source)?)
    }

    fn insert_value(
        &mut self,
        uri: AbsoluteUri,
        source: Value,
    ) -> Result<&Value, DuplicateSourceError> {
        if self.sources.contains_key(&uri) {
            // safe, checked above
            let src = self.sources.get(&uri).unwrap();
            if src == &source {
                return Ok(src);
            }
            // error out if a source with the same uri has been indexed and the
            // values do not match
            return Err(DuplicateSourceError { uri, source });
        }
        Ok(self.sources.entry(uri.clone()).or_insert(source))
    }
    pub fn entry<'e>(&'e mut self, key: AbsoluteUri) -> Entry<'e, AbsoluteUri, Value> {
        self.sources.entry(key)
    }

    pub(crate) fn contains(&self, uri: &AbsoluteUri) -> bool {
        self.sources.contains_key(uri)
    }
}
