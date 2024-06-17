use crate::{
    schema::CompiledSchema,
    spec::{self, Specification},
};
use grill_core::{
    lang::{Numbers, Schemas, Sources, Values},
    Key,
};
use grill_uri::AbsoluteUri;
use serde_json::Value;
use snafu::Snafu;
use std::{fmt, marker::PhantomData, ops::Deref};

mod consts;
pub use consts::*;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Keyword                                ║
║                               ¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A JSON Schema keyword.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {}

impl<S, K> spec::Keyword<S, K> for Keyword
where
    S: spec::Specification<K>,
    K: 'static + Send + Key,
{
    fn compile<'i>(
        &self,
        compile: spec::alias::Compile<S, K>,
    ) -> Option<Result<(), spec::alias::CompileError<S, K>>> {
        todo!()
    }

    fn evaluate<'v>(
        &self,
        eval: spec::alias::Evaluate<S, K>,
    ) -> Result<(), spec::alias::EvaluateError<S, K>> {
        todo!()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Keywords                               ║
║                               ¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A slice of [`Keyword`]s belonging to a schema.
pub struct Keywords<'i>(pub &'i [Keyword]);

impl<'i> From<&'i [Keyword]> for Keywords<'i> {
    fn from(keywords: &'i [Keyword]) -> Self {
        Self(keywords)
    }
}
impl<'i> IntoIterator for Keywords<'i> {
    type Item = &'i Keyword;
    type IntoIter = std::slice::Iter<'i, Keyword>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Keywords<'_> {
    /// Returns the slice of keywords.
    pub fn as_slice(&self) -> &[Keyword] {
        self.0
    }
}
impl AsRef<[Keyword]> for Keywords<'_> {
    fn as_ref(&self) -> &[Keyword] {
        self.0
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Compile                                ║
║                               ¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Context for [`Keyword::compile`].
pub struct Compile<'i, S: Specification<K>, K>
where
    K: 'static + Key + Send,
{
    pub(crate) schemas: &'i mut Schemas<CompiledSchema<S, K>, K>,
    pub(crate) sources: &'i mut Sources,
    pub(crate) numbers: &'i mut Numbers,
    pub(crate) values: &'i mut Values,
}

impl<'i, S, K> spec::Compile<'i, K> for Compile<'i, S, K>
where
    K: 'static + Send + Key,
    S: Specification<K>,
{
    fn numbers(&mut self) -> &'i mut Numbers {
        todo!()
    }

    fn values(&mut self) -> &'i mut Values {
        todo!()
    }

    fn sources(&self) -> &'i Sources {
        todo!()
    }

    fn schema(&self, uri: &AbsoluteUri) -> Option<K> {
        todo!()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               Evaluate                                ║
║                              ¯¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Context for [`Keyword::evaluate`].
pub struct Evaluate<'i> {
    marker: PhantomData<&'i ()>,
    // pub(crate) schemas: Schemas,
}
impl<'i, K: 'static + Key> spec::Evaluate<'i, K> for Evaluate<'i> {
    fn schema(&self, uri: &AbsoluteUri) -> Option<K> {
        todo!()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                              invaid_type                              ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub mod invalid_type {
    use super::{fmt, Snafu, Value};

    #[derive(Clone, Debug, Copy, strum::Display)]
    pub enum Actual {
        Bool,
        Number,
        String,
        Array,
        Object,
        Null,
    }
    impl Actual {
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
    #[derive(Clone, Debug, Copy)]
    pub enum Expectated {
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
        AnyOf(&'static [Expectated]),
    }

    impl fmt::Display for Expectated {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expectated::Bool => write!(f, "Bool"),
                Expectated::Number => write!(f, "Number"),
                Expectated::String => write!(f, "String"),
                Expectated::Array => write!(f, "Array"),
                Expectated::Object => write!(f, "Object"),
                Expectated::Null => write!(f, "Null"),
                Expectated::AnyOf(anyof) => write_anyof(f, anyof),
            }
        }
    }

    fn write_anyof(f: &mut fmt::Formatter<'_>, anyof: &[Expectated]) -> fmt::Result {
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
    #[derive(Debug, Snafu)]
    #[snafu(
        display("expected value of type {expected}, found {actual}"),
        visibility(pub)
    )]
    pub struct InvalidTypeError {
        /// The expected type of value.
        pub expected: Expectated,
        /// The actual value.
        pub value: Box<Value>,
        pub actual: Actual,
        pub backtrace: snafu::Backtrace,
    }
    impl InvalidTypeError {
        pub fn new(value: Value, expected: Expectated) -> Self {
            InvalidTypeSnafu {
                actual: Actual::from_value(&value),
                value: Box::new(value),
                expected,
            }
            .build()
        }
    }
}
