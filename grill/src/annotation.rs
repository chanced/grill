pub mod iter;
pub use iter::Iter;
use url::Url;

use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
};

use crate::{AnnotationError, Error, Output};
use jsonptr::Pointer;
use serde::Serialize;
use serde_json::{to_value, Map, Value};
#[derive(Debug, Clone)]
pub struct Annotation {
    instance_location: Pointer,
    keyword_location: Pointer,
    absolute_keyword_location: Option<Url>,
    nested: Vec<Annotation>,
    error: Option<String>,
    output: Output,
    data: Map<String, Value>,
}
///
impl Annotation {
    pub fn new(instance_location: Pointer, keyword_location: Pointer, output: Output) -> Self {
        Self {
            output,
            nested: Vec::new(),
            data: Map::new(),
            error: None,
            instance_location,
            keyword_location,
            absolute_keyword_location: None,
        }
    }
    /// Returns `true` if this or any nested `Annotation` has an error set
    pub fn is_valid(&self) -> bool {
        self.error.is_none() && self.nested.iter().all(Annotation::is_valid)
    }
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
    /// Sets the error message
    pub fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }
    /// Returns the specified `Output`
    pub fn output(&self) -> Output {
        self.output.clone()
    }

    /// Appends an `Evaluation` to the back of the nested `Evaluations`
    pub fn push(&mut self, value: Annotation) {
        self.nested.push(value)
    }
    /// Appends elements to the collection of nested `Evaluation`s.
    pub fn append(&mut self, evals: impl IntoIterator<Item = Annotation>) {
        self.extend(evals.into_iter())
    }

    pub fn get<K>(&self, key: &K) -> Option<Cow<Value>>
    where
        K: ?Sized + Borrow<str>,
    {
        match AnnotationField::from(key.borrow()) {
            AnnotationField::InstanceLocation => Some(Cow::Owned((&self.instance_location).into())),
            AnnotationField::KeywordLocation => Some(Cow::Owned((&self.keyword_location).into())),
            AnnotationField::AbsoluteKeywordLocation => self
                .absolute_keyword_location
                .as_ref()
                .map(|u| Cow::Owned(Value::String(u.to_string()))),
            AnnotationField::Error => self.error.clone().map(|e| Cow::Owned(Value::String(e))),
            _ => self.data.get(key.borrow()).map(Cow::Borrowed),
        }
    }

    pub fn instance_location(&self) -> &Pointer {
        &self.instance_location
    }

    /// Sets the `instance_location` field, returning the previous value
    pub fn set_instance_location(&mut self, loc: Pointer) -> Pointer {
        let old = self.instance_location.clone();
        self.instance_location = loc;
        old
    }

    pub fn keyword_location(&self) -> &Pointer {
        &self.keyword_location
    }

    /// Sets the `keyword_location` field, returning the previous value
    pub fn set_keyword_location(&mut self, loc: Pointer) -> Pointer {
        let old = self.keyword_location.clone();
        self.keyword_location = loc;
        old
    }

    pub fn absolute_keyword_location(&self) -> Option<&Url> {
        self.absolute_keyword_location.as_ref()
    }

    pub fn set_absolute_keyword_location(&mut self, url: Url) -> Option<Url> {
        let old = self.absolute_keyword_location.clone();
        self.absolute_keyword_location = Some(url);
        old
    }

    /// Serializes data into `serde_json::Value` and inserts it into data.
    ///
    /// - If the data map did not have this key present, `None` is returned.
    /// - If the map did have this key present, the value is updated, and the old value is returned.
    ///
    pub fn insert(&mut self, k: String, v: impl Serialize) -> Result<Option<Value>, Error> {
        let v = to_value(v)?;

        match AnnotationField::from(k.as_str()) {
            AnnotationField::InstanceLocation => {
                if let Some(s) = v.as_str() {
                    Ok(Some(
                        self.set_instance_location(Pointer::try_from(s)?).into(),
                    ))
                } else {
                    Err(AnnotationError::ExpectedString(AnnotationField::from(k)).into())
                }
            }
            AnnotationField::KeywordLocation => {
                if let Some(s) = v.as_str() {
                    Ok(Some(
                        self.set_keyword_location(Pointer::try_from(s)?).into(),
                    ))
                } else {
                    Err(AnnotationError::ExpectedString(AnnotationField::from(k)).into())
                }
            }
            AnnotationField::AbsoluteKeywordLocation => {
                if let Some(s) = v.as_str() {
                    Ok(self
                        .set_absolute_keyword_location(Url::parse(s)?)
                        .map(|u| Value::String(u.to_string())))
                } else {
                    Err(AnnotationError::ExpectedString(AnnotationField::from(k)).into())
                }
            }

            AnnotationField::Error => {
                if let Some(s) = v.as_str() {
                    let old = self.error.take();
                    self.error = Some(s.to_string());
                    Ok(old.map(Value::from))
                } else {
                    Err(AnnotationError::ExpectedString(AnnotationField::Error).into())
                }
            }
            _ => Ok(self.data.insert(k, v)),
        }
    }

    /// Sets the internal error to `None`
    /// - If the error was previously set, it is returned
    pub fn reset_error(&mut self) -> Option<String> {
        self.error.take()
    }
}

impl<E> Extend<E> for Annotation
where
    E: Borrow<Annotation>,
{
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        let (_, hint) = iter.size_hint();
        if let Some(size) = hint {
            if size > 0 {
                self.nested.reserve(size)
            }
        }
        for eval in iter {
            self.push(eval.borrow().clone())
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnnotationField {
    InstanceLocation,
    KeywordLocation,
    AbsoluteKeywordLocation,
    Error,
    Data(String),
}

impl From<&str> for AnnotationField {
    fn from(s: &str) -> Self {
        match s {
            "instanceLocation" => AnnotationField::InstanceLocation,
            "keywordLocation" => AnnotationField::KeywordLocation,
            "absoluteKeywordLocation" => AnnotationField::AbsoluteKeywordLocation,
            "error" => AnnotationField::Error,
            _ => AnnotationField::Data(s.to_string()),
        }
    }
}
impl Display for AnnotationField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationField::InstanceLocation => write!(f, "instanceLocation"),
            AnnotationField::KeywordLocation => write!(f, "keywordLocation"),
            AnnotationField::AbsoluteKeywordLocation => write!(f, "absoluteKeywordLocation"),
            AnnotationField::Error => write!(f, "error"),
            AnnotationField::Data(s) => write!(f, "{}", s),
        }
    }
}
impl From<String> for AnnotationField {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
