use std::{borrow::Cow, string, marker::PhantomData};

pub mod iter;

pub use iter::Iter;

use crate::{Error, FieldError, Implementation, Output};
use jsonptr::Pointer;
use serde::Serialize;
use serde_json::{to_value, Map, Value};

#[derive(Debug, Clone)]
pub struct Evaluation<I: ?Sized> {
    instance_location: Pointer,
    keyword_location: Pointer,
    pub nested: Vec<Evaluation<I>>,
    pub error: Option<String>,
    pub output: Output,
    data: Map<String, Value>,
    implementation: Box<I>,
}
///
impl<I> Evaluation<I>
where
    I: Implementation + 'static,
{
    pub fn new(
        implementation: I,
        instance_location: Pointer,
        keyword_location: Pointer,
        output: Output,
    ) -> Self {
        Self {
            implementation: Box::new(implementation),
            output,
            nested: Vec::new(),
            data: Map::new(),
            error: None,
            instance_location,
            keyword_location,
        }
    }
    /// Returns `true` if this `Error`
    pub fn is_valid(&self) -> bool {
        if let Some(err) = self.error {
            false
        } else if self.nested.iter().any(|n| !n.is_valid()) {
            false
        } else {
            true
        }
    }

    /// Appends an `Evaluation` to the back of the nested `Evaluations`
    pub fn push(&self, value: Evaluation<I>) {
        self.nested.push(value)
    }
    /// Appends elements to the collection of nested `Evaluation`s.
    pub fn append(&mut self, evals: Vec<Evaluation<I>>) {
        self.extend(evals.iter())
    }

    pub fn get(&self, key: &str) -> Option<Cow<Value>> {
        match self.field(key) {
            Field::Instance => Some(Cow::Owned(self.instance_location.into())),
            Field::Keyword => Some(Cow::Owned(self.keyword_location.into())),
            Field::Error => self.error.clone().map(|e| Cow::Owned(Value::String(e))),
            _ => self.data.get(key).map(Cow::Borrowed),
        }
    }
    /// Sets the `instance_location` field, returning the previous value
    pub fn set_instance_location(&mut self, loc: Pointer) -> Pointer {
        let v: Value;
        let old = self.instance_location;
        self.instance_location = loc;
        old
    }
    /// Sets the `keyword_location` field, returning the previous value
    pub fn set_keyword_location(&mut self, loc: Pointer) -> Pointer {
        let old = self.keyword_location;
        self.keyword_location = loc;
        old
    }

    /// Serializes data into `serde_json::Value` and inserts it into data.
    ///
    /// - If the data map did not have this key present, `None` is returned.
    /// - If the map did have this key present, the value is updated, and the old value is returned.
    ///
    pub fn insert(&mut self, k: String, v: impl Serialize) -> Result<Option<Value>, Error> {
        let v = to_value(v)?;
        match self.field(&k) {
            Field::Instance => {
                if let Some(s) = v.as_str() {
                    Ok(Some(
                        self.set_instance_location(Pointer::try_from(s)?).into(),
                    ))
                } else {
                    Err(FieldError::ExpectedString { field: k }.into())
                }
            }
            Field::Keyword => {
                if let Some(s) = v.as_str() {
                    Ok(Some(
                        self.set_keyword_location(Pointer::try_from(s)?).into(),
                    ))
                } else {
                    Err(FieldError::ExpectedString { field: k }.into())
                }
            }
            Field::Error => {
                if let Some(s) = v.as_str() {
                    let old = self.error.take();
                    self.error = Some(s.to_string());
                    Ok(old.map(Value::from))
                } else {
                    Err(FieldError::ExpectedString { field: k }.into())
                }
            }
            _ => Ok(self.data.insert(k, v)),
        }
    }
    pub fn field(&self, key: &str) -> Field<I> {
        if key == I::keyword_location_field() {
            Field::Keyword
        } else if key == I::instance_location_field() {
            Field::Instance
        } else if key == I::error_field() {
            Field::Error
        } else {
            Field::Data
        }
    }
    /// Sets the error message
    pub fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }
    /// Sets the internal error to `None`
    /// - If the error was previously set, it is returned
    pub fn reset_error(&mut self) -> Option<String> {
        self.error.take()
    }
}

impl<I, A> Extend<A> for Evaluation<I> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        let (_, hint) = iter.size_hint();
        if let Some(size) = hint {
            if size > 0 {
                self.nested.reserve(size)
            }
        }
        for eval in iter {
            self.push(eval)
        }
    }
}

#[allow(non_camel_case_types)]
enum Field<I> {
    Instance,
    Keyword,
    Error,
    Data,
    _phantom(PhantomData<I>)
}
