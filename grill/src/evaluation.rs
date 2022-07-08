mod iter;
pub use iter::Iter;
use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
};
use uniresid::AbsoluteUri;

use crate::{error::ExpectedStringError, Error, OutputFmt};
use jsonptr::Pointer;
use serde::Serialize;
use serde_json::{to_value, Map, Value};

/// An `Evaluation` is the constructed by [`Applicators`](crate::Applicator) and returned from
/// annotating pertinent information, based upon the specified
/// [`OutputFmt`](crate::OutputFmt), regarding the validation state of the keyword
/// (and thus the [`Schema`](crate::Schema))
/// [`Schema::evaluate`](crate::Schema::evaluate)
#[derive(Debug, Clone)]
pub struct Evaluation {
    instance_location: Pointer,
    keyword_location: Pointer,
    absolute_keyword_location: Option<AbsoluteUri>,
    nested: Vec<Evaluation>,
    error: Option<String>,
    output: OutputFmt,
    data: Map<String, Value>,
}
///
impl Evaluation {
    /// Creates and returns a new `Evaluation`.
    pub fn new(instance_location: Pointer, keyword_location: Pointer, output: OutputFmt) -> Self {
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
        self.error.is_none() && self.nested.iter().all(Evaluation::is_valid)
    }
    /// The associated `error`, if one exists.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Sets the error message
    pub fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }
    /// Returns the specified `OutputFmt`
    pub fn output(&self) -> OutputFmt {
        self.output.clone()
    }

    /// Appends an `Evaluation` to the back of the nested `Evaluations`
    pub fn push(&mut self, value: Evaluation) {
        self.nested.push(value)
    }
    /// Appends elements to the collection of nested `Evaluation`s.
    pub fn append(&mut self, evals: impl IntoIterator<Item = Evaluation>) {
        self.extend(evals.into_iter())
    }
    /// Returns the field at the given `key` if it exists.
    pub fn get<K>(&self, key: &K) -> Option<Cow<Value>>
    where
        K: ?Sized + Borrow<str>,
    {
        match Field::from(key.borrow()) {
            Field::InstanceLocation => Some(Cow::Owned((&self.instance_location).into())),
            Field::KeywordLocation => Some(Cow::Owned((&self.keyword_location).into())),
            Field::AbsoluteKeywordLocation => self
                .absolute_keyword_location
                .as_ref()
                .map(|u| Cow::Owned(Value::String(u.to_string()))),
            Field::Error => self.error.clone().map(|e| Cow::Owned(Value::String(e))),
            _ => self.data.get(key.borrow()).map(Cow::Borrowed),
        }
    }

    /// The location of the JSON value within the instance being validated. The
    /// value MUST be expressed as a JSON Pointer.
    ///
    /// See [JSON Schema Core Specification 12.3.3 for more
    /// information](https://json-schema.org/draft/2020-12/json-schema-core.html#name-instance-location).
    pub fn instance_location(&self) -> &Pointer {
        &self.instance_location
    }

    /// Sets the `instance_location` field, returning the previous value
    pub fn set_instance_location(&mut self, loc: Pointer) -> Pointer {
        let old = self.instance_location.clone();
        self.instance_location = loc;
        old
    }
    /// The relative location of the validating keyword that follows the
    /// validation path.
    ///
    /// See [JSON Schema Core Specification 12.3.1 for more
    /// information](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-relative-location).
    pub fn keyword_location(&self) -> &Pointer {
        &self.keyword_location
    }

    /// Sets the `"keywordLocation"` field, returning the previous value
    pub fn set_keyword_location(&mut self, loc: Pointer) -> Pointer {
        let old = self.keyword_location.clone();
        self.keyword_location = loc;
        old
    }
    /// The absolute, dereferenced location of the validating keyword. The value
    /// MUST be expressed as a full URI using the canonical URI of the relevant
    /// schema resource with a JSON Pointer fragment, and it MUST NOT include
    /// by-reference applicators such as "$ref" or "$dynamicRef" as non-terminal
    /// path components. It MAY end in such keywords if the error or annotation
    /// is for that keyword, such as an unresolvable reference.
    ///
    /// See [JSON Schema Core Specification 12.3.2 for more information.](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-absolute-location)
    pub fn absolute_keyword_location(&self) -> Option<&AbsoluteUri> {
        self.absolute_keyword_location.as_ref()
    }
    /// Sets the `"absoluteKeywordLocation"` field, returning the previous value
    pub fn set_absolute_keyword_location(&mut self, uri: AbsoluteUri) -> Option<AbsoluteUri> {
        let old = self.absolute_keyword_location.take();
        self.absolute_keyword_location = Some(uri);
        old
    }

    /// Serializes data into `serde_json::Value` and inserts it into data.
    ///
    /// - If the data map did not have this key present, `None` is returned.
    /// - If the map did have this key present, the value is updated, and the old value is returned.
    ///
    pub fn insert(&mut self, k: String, v: impl Serialize) -> Result<Option<Value>, Error> {
        let v = to_value(v)?;

        match Field::from(k.as_str()) {
            Field::InstanceLocation => {
                if let Some(s) = v.as_str() {
                    Ok(Some(
                        self.set_instance_location(Pointer::try_from(s)?).into(),
                    ))
                } else {
                    Err(ExpectedStringError {
                        field: Field::from(k),
                    }
                    .into())
                }
            }
            Field::KeywordLocation => {
                if let Some(s) = v.as_str() {
                    Ok(Some(
                        self.set_keyword_location(Pointer::try_from(s)?).into(),
                    ))
                } else {
                    Err(ExpectedStringError {
                        field: Field::from(k),
                    }
                    .into())
                }
            }
            Field::AbsoluteKeywordLocation => {
                if let Some(s) = v.as_str() {
                    Ok(self
                        .set_absolute_keyword_location(AbsoluteUri::parse(s)?)
                        .map(|u| Value::String(u.to_string())))
                } else {
                    Err(ExpectedStringError {
                        field: Field::from(k),
                    }
                    .into())
                }
            }

            Field::Error => {
                if let Some(s) = v.as_str() {
                    let old = self.error.take();
                    self.error = Some(s.to_string());
                    Ok(old.map(Value::from))
                } else {
                    Err(ExpectedStringError {
                        field: Field::Error,
                    }
                    .into())
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

impl<E> Extend<E> for Evaluation
where
    E: Borrow<Evaluation>,
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

/// Represents a field of an `Evaluation`
#[derive(Debug, Clone)]
pub enum Field {
    /// The `"instanceLocation"` field.
    InstanceLocation,
    /// The `"keywordLocation"` field.
    KeywordLocation,
    /// The `"absoluteKeywordLocation"` field.
    AbsoluteKeywordLocation,
    /// The `"error"` field.
    Error,
    /// A custom field.
    Data(String),
}

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        match s {
            "instanceLocation" => Field::InstanceLocation,
            "keywordLocation" => Field::KeywordLocation,
            "absoluteKeywordLocation" => Field::AbsoluteKeywordLocation,
            "error" => Field::Error,
            _ => Field::Data(s.to_string()),
        }
    }
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::InstanceLocation => write!(f, "instanceLocation"),
            Field::KeywordLocation => write!(f, "keywordLocation"),
            Field::AbsoluteKeywordLocation => write!(f, "absoluteKeywordLocation"),
            Field::Error => write!(f, "error"),
            Field::Data(s) => write!(f, "{}", s),
        }
    }
}
impl From<String> for Field {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
