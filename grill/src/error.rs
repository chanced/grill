use jsonptr::MalformedPointerError;
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use url::ParseError as UrlParseError;

use crate::evaluation::Field;
use crate::Schema;
pub type BoxedError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug)]
pub enum Error {
    /// For use with applicators and validators to use when an internal
    /// error occurs (e.g. unable to connect to a database, request timeout).
    Internal(Box<dyn StdError + Send + Sync + 'static>),
    /// An error occurred serializing or deserializing data.
    Serde(SerdeError),
    Annotation(AnnotationError),
    InvalidSchema(InvalidSchemaError),
    UnindentifiedSchema(UnidentifiedSchemaError),
}

impl Error {
    pub fn new_internal(err: impl StdError + Send + Sync + 'static) -> Self {
        Error::Internal(Box::new(err))
    }
}

impl From<UnidentifiedSchemaError> for Error {
    fn from(err: UnidentifiedSchemaError) -> Self {
        Error::UnindentifiedSchema(err)
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::Serde(err)
    }
}
impl From<UrlParseError> for Error {
    fn from(err: UrlParseError) -> Self {
        Error::Annotation(AnnotationError::from(err))
    }
}

impl From<InvalidSchemaError> for Error {
    fn from(err: InvalidSchemaError) -> Self {
        Error::InvalidSchema(err)
    }
}

// impl From<YamlError> for Error {
//     fn from(err: YamlError) -> Self {
//         Error::Serde(SerdeError::from(err))
//     }
// }

impl From<AnnotationError> for Error {
    fn from(err: AnnotationError) -> Self {
        Self::Annotation(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(err) => Display::fmt(err, f),
            Error::Serde(err) => Display::fmt(err, f),
            Error::Annotation(err) => Display::fmt(err, f),
            Error::InvalidSchema(err) => Display::fmt(err, f),
            Error::UnindentifiedSchema(err) => Display::fmt(err, f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Internal(err) => Some(err.as_ref()),
            Error::Serde(err) => Some(err),
            Error::Annotation(err) => err.source(),
            Error::InvalidSchema(err) => Some(err),
            Error::UnindentifiedSchema(err) => Some(err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnidentifiedSchemaError {
    pub schema: Schema,
}

impl std::fmt::Display for UnidentifiedSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "schema $id not set")
    }
}

impl StdError for UnidentifiedSchemaError {}

#[derive(Clone)]
pub enum IndexError {
    /// the schema does not contain an identifier (id) and thus can not be
    /// indexed
    NotIdentified,
}
impl Debug for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexError::NotIdentified => write!(f, "schema is not identifiable"),
        }
    }
}
impl Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexError::NotIdentified => write!(f, "schema is not identifiable"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnnotationError {
    MalformedPointer(MalformedPointerError),
    ExpectedString(Field),
    ParseUrl(url::ParseError),
}

impl Display for AnnotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotationError::MalformedPointer(err) => std::fmt::Display::fmt(&err, f),
            AnnotationError::ExpectedString(field) => {
                write!(f, "error: expected string for \"{}\"", field)
            }
            AnnotationError::ParseUrl(err) => std::fmt::Display::fmt(&err, f),
        }
    }
}

impl From<UrlParseError> for AnnotationError {
    fn from(err: UrlParseError) -> Self {
        Self::ParseUrl(err)
    }
}

impl StdError for AnnotationError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AnnotationError::MalformedPointer(err) => Some(err),
            AnnotationError::ExpectedString(_) => None,
            AnnotationError::ParseUrl(err) => Some(err),
        }
    }
}

impl From<MalformedPointerError> for Error {
    fn from(err: MalformedPointerError) -> Self {
        Error::Annotation(AnnotationError::MalformedPointer(err))
    }
}

impl StdError for IndexError {}

#[derive(Debug)]
pub struct MissingLayerError();

#[derive(Clone, Debug)]
pub struct InvalidSchemaError;

impl std::fmt::Display for InvalidSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no applicators found for this schema")
    }
}

impl StdError for InvalidSchemaError {}
