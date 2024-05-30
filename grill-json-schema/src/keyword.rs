use crate::{alias, schema::CompiledSchema, Specification};
use enum_dispatch::enum_dispatch;
use grill_core::{
    big::BigRational,
    lang::{source::Source, Numbers, Schemas, Sources, Values},
    Key,
};
use grill_uri::AbsoluteUri;
use serde_json::{Number, Value};
use snafu::Snafu;
use std::{
    fmt::{self, Debug},
    sync::Arc,
};

mod consts;
pub use consts::*;

pub mod spec;

pub mod compile {
    use super::*;

    /// Context for [`Keyword::compile`].
    pub trait Compile<K>
    where
        K: Key,
    {
        /// Retrieves a schema from the store by [`AbsoluteUri`].
        fn schema(&self, uri: &AbsoluteUri) -> Option<K>;

        /// Returns a mutable reference to [`Numbers`] cache.
        fn numbers(&mut self) -> &mut Numbers;

        /// Parses a JSON number into an [`Arc<BigRational>`](`BigRational`) and
        /// stores it in the [`Numbers`] cache if it is not already present.
        /// Otherwise, the existing [`BigRational`] is returned.
        fn number(
            &mut self,
            number: &Number,
        ) -> Result<Arc<BigRational>, grill_core::big::ParseError>;

        /// Returns a mutable reference to [`Values`] cache.
        fn values(&mut self) -> &mut Values;

        /// If `value` is already in the [`Values`] cache, the existing
        /// `Arc<Value>` is cloned and returned. Otherwise, `value` is inserted
        /// as an `Arc<Value>`, cloned, and returned.
        fn value(&mut self, value: &Value) -> Arc<Value>;

        /// Returns a reference to [`Sources`].
        fn sources(&self) -> &Sources;

        /// Retrieves a [`Source`] from the store by [`AbsoluteUri`], if a
        /// [`Link`](grill_core::lang::source::Link) exists.
        fn source(&self, uri: &AbsoluteUri) -> Option<Source>;
    }

    /// Context for [`Keyword::compile`].
    pub struct Context<W, K> {
        pub(crate) schemas: Schemas<CompiledSchema<W, K>, K>,
        pub(crate) sources: Sources,
        pub(crate) numbers: Numbers,
        pub(crate) values: Values,
    }
}
pub use compile::Compile;

pub mod eval {
    use super::*;

    /// Context for [`Keyword::evaluate`].
    pub trait Evaluate<K: Key> {
        fn schema(&self, uri: &AbsoluteUri) -> Option<K>;
    }

    /// Context for [`Keyword::evaluate`].
    pub struct Context {
        // pub(crate) schemas: Schemas,
    }
}

pub use eval::Evaluate;

pub trait EvaluateKeyword {}

use spec::Spec;

pub trait Keyword<S, K>: Send + Debug + Clone
where
    S: Specification<K>,
    K: Key,
{
    fn compile<'i>(
        &self,
        compile: alias::Compile<S, K>,
    ) -> Option<Result<(), alias::CompileError<S, K>>>;

    fn evaluate<'v>(&self, eval: alias::Evaluate<S, K>) -> Result<(), alias::EvaluateError<S, K>>;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Actual                                ║
║                                ¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               Expected                                ║
║                              ¯¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

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
            Expectated::AnyOf(anyof) => {
                write!(f, "[")?;
                for (i, expected) in anyof.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{expected}")?;
                }
                write!(f, "]")
            }
            Expectated::Null => write!(f, "Null"),
        }
    }
}
/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                              InvalidType                              ║
║                              ¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A [`Value`] was not of the expected type.
#[derive(Debug, Snafu)]
#[snafu(
    display("expected value of type {expected}, found {actual}"),
    module,
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
