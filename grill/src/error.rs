use crate::evaluation::Field;
use crate::Schema;
use jsonptr::{Error as PointerError, MalformedPointerError};
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use uniresid::Error as UriError;

/// Represents all possible errors that can occur while initializing, setting up
/// or using an [`Interrogator`] and [`Schema`].
#[derive(Debug, Clone)]
pub enum Error {
    /// For use with applicators and validators to use when an internal
    /// error occurs (e.g. unable to connect to a database, request timeout).
    Internal(Arc<dyn StdError + Send + Sync + 'static>),
    /// An error occurred serializing or deserializing data.
    Serde(Arc<SerdeError>),
    /// A Schema did not have any matching `Applicator`s.
    InvalidSchema(InvalidSchemaError),
    /// A top-level `Schema` was not identifiable.
    UnindentifiedSchema(UnidentifiedSchemaError),
    /// An error occurred parsing a JSON Pointer.
    Pointer(PointerError),
    /// An `Evaluation` field required a String value but was provided something
    /// else.
    ExpectedString(ExpectedStringError),
    /// An error occurred parsing a URI.
    Uri(UriError),
}
impl Error {
    /// Wraps a `std::error::Error` in an [`Error::Internal`](Error::Internal).
    /// This should be used in an `Applicator` when an internal error arises which
    /// does not fit in one of the other error classifications.
    pub fn new_internal(err: impl StdError + Send + Sync + 'static) -> Self {
        Error::Internal(Arc::new(err))
    }
    /// Returns `true` if the error is an `Internal` error.
    pub fn is_internal(&self) -> bool {
        matches!(self, Error::Internal(_))
    }

    /// Returns `true` if the error is a `Serde` error.
    pub fn is_serde(&self) -> bool {
        matches!(self, Error::Serde(_))
    }
    /// Returns `true` if the error is an `InvalidSchema` error.
    pub fn is_invalid_schema(&self) -> bool {
        matches!(self, Error::InvalidSchema(_))
    }
    /// Returns `true` if the error is an `UnindentifiedSchema` error.
    pub fn is_unindentified_schema(&self) -> bool {
        matches!(self, Error::UnindentifiedSchema(_))
    }

    /// Returns `true` if the error is a `MalformedPointer` error.
    pub fn is_malformed_pointer(&self) -> bool {
        matches!(self, Error::Pointer(_))
    }

    /// Returns `true` if the error is an `ExpectedString` error.
    pub fn is_expected_string(&self) -> bool {
        matches!(self, Error::ExpectedString(_))
    }

    /// Returns `true` if the error is a `MalformedUri` error.
    pub fn is_malformed_uri(&self) -> bool {
        matches!(self, Error::Uri(_))
    }
}

impl From<UnidentifiedSchemaError> for Error {
    fn from(err: UnidentifiedSchemaError) -> Self {
        Error::UnindentifiedSchema(err)
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::Serde(Arc::new(err))
    }
}
impl From<UriError> for Error {
    fn from(err: UriError) -> Self {
        Error::Uri(err)
    }
}

impl From<InvalidSchemaError> for Error {
    fn from(err: InvalidSchemaError) -> Self {
        Error::InvalidSchema(err)
    }
}
// impl From<SerdeError> for Error {
//     fn from(err: SerdeError) -> Self {
//         Error::Serde(err)
//     }
// }
impl From<PointerError> for Error {
    fn from(err: PointerError) -> Self {
        Error::Pointer(err)
    }
}

impl From<MalformedPointerError> for Error {
    fn from(err: MalformedPointerError) -> Self {
        Error::Pointer(err.into())
    }
}

impl From<ExpectedStringError> for Error {
    fn from(err: ExpectedStringError) -> Self {
        Self::ExpectedString(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(err) => Display::fmt(err, f),
            Error::Serde(err) => Display::fmt(err, f),
            Error::InvalidSchema(err) => Display::fmt(err, f),
            Error::UnindentifiedSchema(err) => Display::fmt(err, f),
            Error::Pointer(err) => Display::fmt(err, f),
            Error::ExpectedString(err) => Display::fmt(err, f),
            Error::Uri(err) => Display::fmt(err, f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Internal(err) => Some(err.as_ref()),
            Error::Serde(err) => Some(err),
            Error::InvalidSchema(err) => Some(err),
            Error::UnindentifiedSchema(err) => Some(err),
            Error::Pointer(err) => Some(err),
            Error::ExpectedString(err) => Some(err),
            Error::Uri(err) => Some(err),
        }
    }
}

/// Indicates that the [`Schema`] was not identified. Top-level [`Schema`]s must
/// have an `id` set.
#[derive(Debug, Clone)]
pub struct UnidentifiedSchemaError {
    /// The [`Schema`] which lacks an `id`.
    pub schema: Schema,
}

impl std::fmt::Display for UnidentifiedSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "schema $id not set")
    }
}

impl StdError for UnidentifiedSchemaError {}

#[derive(Clone, Debug)]
pub struct InvalidSchemaError;

impl std::fmt::Display for InvalidSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: no applicators found for this schema")
    }
}

impl StdError for InvalidSchemaError {}

#[derive(Debug, Clone)]
pub struct ExpectedStringError {
    pub field: Field,
}

impl Display for ExpectedStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: expected string for field \"{}\".", self.field)
    }
}

impl StdError for ExpectedStringError {}
