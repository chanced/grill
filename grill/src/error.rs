use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::result::Result as StdResult;
pub type Result<T> = StdResult<T, Error>;
use jsonptr::MalformedPointerError;
use serde_json::Error as JsonError;

pub type BoxedError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug)]
pub enum Error {
    /// For use with applicators and validators to use when an internal
    /// error occurs (e.g. unable to connect to a database, request timeout).
    Internal(Box<dyn StdError + Send + Sync + 'static>),
    /// An error occurred serializing or deserializing data.
    Serde(SerdeError),
    Field(FieldError),
}

impl Error {
    pub fn new_internal(err: impl StdError + Send + Sync + 'static) -> Self {
        Error::Internal(Box::new(err))
    }
}
impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Serde(SerdeError::from(err))
    }
}
// impl From<YamlError> for Error {
//     fn from(err: YamlError) -> Self {
//         Error::Serde(SerdeError::from(err))
//     }
// }
impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::Serde(err)
    }
}

impl From<FieldError> for Error {
    fn from(err: FieldError) -> Self {
        Self::Field(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(err) => Display::fmt(err, f),
            Error::Serde(err) => Display::fmt(err, f),
            Error::Field(err) => Display::fmt(err, f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Internal(err) => Some(err.as_ref()),
            Error::Serde(err) => Some(err),
            Error::Field(err) => err.source,
        }
    }
}

/// A wrapper for serialization or deserialization errors in either JSON or
/// YAML.
#[derive(Debug)]
pub enum SerdeError {
    Json(JsonError),
    // Yaml(YamlError),
}

impl From<JsonError> for SerdeError {
    fn from(err: JsonError) -> Self {
        SerdeError::Json(err)
    }
}
// impl From<YamlError> for SerdeError {
//     fn from(err: YamlError) -> Self {
//         SerdeError::Yaml(err)
//     }
// }

impl StdError for SerdeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            SerdeError::Json(err) => err.source(),
            // SerdeError::Yaml(err) => err.source(),
        }
    }
}

impl Display for SerdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerdeError::Json(err) => Display::fmt(&err, f),
            // SerdeError::Yaml(err) => std::fmt::Display::fmt(&err, f),
        }
    }
}

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
pub enum FieldError {
    MalformedPointer {
        error: MalformedPointerError,
        field: String,
    },
    ExpectedString {
        field: String,
    },
}
impl Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<MalformedPointerError> for Error {
    fn from(err: MalformedPointerError) -> Self {
        Error::MalformedPointer(err)
    }
}

impl StdError for IndexError {}

#[derive(Debug)]
pub struct MissingLayerError();
