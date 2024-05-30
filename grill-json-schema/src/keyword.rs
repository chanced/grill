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
use std::fmt;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {}

impl<S, K> spec::Keyword<S, K> for Keyword
where
    S: spec::Specification<K>,
    K: Send + Key,
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
║                                Compile                                ║
║                               ¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Context for [`Keyword::compile`].
pub struct Compile<S: Specification<K>, K: Key> {
    pub(crate) schemas: Schemas<CompiledSchema<S, K>, K>,
    pub(crate) sources: Sources,
    pub(crate) numbers: Numbers,
    pub(crate) values: Values,
}
impl<S, K> spec::Compile<K> for Compile<S, K>
where
    K: Send + Key,
    S: Specification<K>,
{
    fn schema(&self, uri: &AbsoluteUri) -> Option<K> {
        todo!()
    }

    fn numbers(&mut self) -> &mut Numbers {
        todo!()
    }

    fn number(
        &mut self,
        number: &serde_json::Number,
    ) -> Result<std::sync::Arc<grill_core::big::BigRational>, grill_core::big::ParseError> {
        todo!()
    }

    fn values(&mut self) -> &mut Values {
        todo!()
    }

    fn value(&mut self, value: &Value) -> std::sync::Arc<Value> {
        todo!()
    }

    fn sources(&self) -> &Sources {
        todo!()
    }

    fn source(&self, uri: &AbsoluteUri) -> Option<grill_core::lang::source::Source> {
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
pub struct Evaluate {
    // pub(crate) schemas: Schemas,
}
impl<K: Key> spec::Evaluate<K> for Evaluate {
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
}
