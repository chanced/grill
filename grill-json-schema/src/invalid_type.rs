use serde_json::Value;
use std::{error::Error, fmt};

#[derive(Clone, Debug, Copy, strum::Display, PartialEq, Eq)]
pub enum Actual {
    Bool,
    Number,
    String,
    Array,
    Object,
    Null,
}
impl Actual {
    #[must_use]
    pub fn from_value(value: &Value) -> Self {
        Self::from(value)
    }
}
impl From<&Value> for Actual {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(_) => Self::Bool,
            Value::Number(_) => Self::Number,
            Value::String(_) => Self::String,
            Value::Array(_) => Self::Array,
            Value::Object(_) => Self::Object,
        }
    }
}

/// The expected type of a [`Value`].
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Expected {
    /// Expected a null value
    Null,
    /// Expected a boolean
    Bool,
    /// Expected a number
    Number,
    /// Expected a string
    String,
    /// Execpted an array
    Array,
    /// Expected an object
    Object,
    /// Expected any of the types in the slice
    AnyOf(&'static [Expected]),
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expected::Bool => write!(f, "Bool"),
            Expected::Number => write!(f, "Number"),
            Expected::String => write!(f, "String"),
            Expected::Array => write!(f, "Array"),
            Expected::Object => write!(f, "Object"),
            Expected::Null => write!(f, "Null"),
            Expected::AnyOf(anyof) => write_anyof(f, anyof),
        }
    }
}

fn write_anyof(f: &mut fmt::Formatter<'_>, anyof: &[Expected]) -> fmt::Result {
    // this isn't super effecient but simple enough for error output
    write!(f, "[")?;
    for (i, expected) in anyof.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "{expected}")?;
    }
    write!(f, "]")
}

/// A [`Value`] was not of the expected type.
#[derive(Debug, PartialEq, Eq)]
pub struct InvalidTypeError {
    /// The expected type of value.
    pub expected: Expected,
    /// The actual value.
    pub value: Box<Value>,
}
impl fmt::Display for InvalidTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "expected value of type {}, found {}",
            self.expected,
            self.actual()
        )
    }
}
impl Error for InvalidTypeError {}

impl InvalidTypeError {
    pub fn new(value: Value, expected: Expected) -> Self {
        Self {
            value: Box::new(value),
            expected,
        }
    }
    pub fn actual(&self) -> Actual {
        Actual::from_value(&self.value)
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn expected(&self) -> Expected {
        self.expected
    }

    pub fn take_value(self) -> Value {
        *self.value
    }
}
