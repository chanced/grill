use std::{borrow::Cow, ops::Deref, sync::Arc};

use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Display)]
#[serde(untagged)]
pub enum Annotation<'v> {
    Cow(Cow<'v, Value>),
    Arc(Arc<Value>),
}

impl<'de> Deserialize<'de> for Annotation<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(Self::Arc(Arc::from(value)))
    }
}
impl<'v> From<&'v Value> for Annotation<'v> {
    fn from(value: &'v Value) -> Self {
        Self::Cow(Cow::Borrowed(value))
    }
}
impl From<Value> for Annotation<'static> {
    fn from(value: Value) -> Self {
        Self::Cow(Cow::Owned(value))
    }
}
impl From<Arc<Value>> for Annotation<'_> {
    fn from(value: Arc<Value>) -> Self {
        Self::Arc(value)
    }
}
impl From<&Arc<Value>> for Annotation<'_> {
    fn from(value: &Arc<Value>) -> Self {
        Self::Arc(value.clone())
    }
}

impl Deref for Annotation<'_> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Cow(value) => value,
            Self::Arc(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_name() {
        println!("{}", Annotation::Arc(Arc::new(json!({"key": "value"}))));
    }
}
