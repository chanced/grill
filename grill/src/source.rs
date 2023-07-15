use crate::{error::DeserializeError, AbsoluteUri, Deserializer, Metaschema};
use serde_json::Value;
use std::{borrow::Cow, collections::HashMap};

pub enum Source {
    String(AbsoluteUri, String),
    Value(AbsoluteUri, Value),
}

impl Source {
    pub(crate) fn try_take(
        self,
        deserializers: &[(&'static str, Box<dyn Deserializer>)],
    ) -> Result<(AbsoluteUri, Value), DeserializeError> {
        match self {
            Self::Value(uri, val) => Ok((uri, val)),
            Self::String(uri, str) => {
                let val = self.deserialize(&str, deserializers)?;
                Ok((uri, val))
            }
        }
    }
    pub fn uri(&self) -> &AbsoluteUri {
        match self {
            Self::Value(uri, _) | Self::String(uri, _) => &uri,
        }
    }
    fn deserialize(
        &self,
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
    pub fn value(
        &self,
        deserializers: &[(&'static str, Box<dyn Deserializer>)],
    ) -> Result<Cow<Value>, DeserializeError> {
        match self {
            Self::String(_, s) => Ok(Cow::Owned(self.deserialize(s, deserializers)?)),
            Self::Value(_, source) => Ok(Cow::Borrowed(source)),
        }
    }
}

impl From<&Metaschema> for Source {
    fn from(value: &Metaschema) -> Self {
        Self::Value(value.id.clone(), value.schema.clone().into())
    }
}
