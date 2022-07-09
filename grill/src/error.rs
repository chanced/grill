use crate::evaluation::Field;
use crate::{Evaluation, Schema};

use jsonptr::{Error as PointerError, MalformedPointerError};
use serde_json::{Error as SerdeError, Value};
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use uniresid::{Error as UriError, Uri};

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
    InvalidPointer(PointerError),
    /// An `Evaluation` field required a String value but was provided something
    /// else.
    ExpectedString(ExpectedStringError),
    /// An error occurred parsing a URI.
    InvalidUri(UriError),
    /// An unkown error occurred parsing the `"$schema" field.
    MetaSchema(MetaSchemaError),
    /// A [`MetaSchema`] required a `"$vocabulary"` which is not present in the [`Interrogator`].
    MissingRequiredVocabulary(MissingRequiredVocabularyError),

    /// Schema is not setup. Add it to the [`Interrogator`] before using it.
    SchemaNotSetup(SchemaNotSetupError),
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
        matches!(self, Error::InvalidPointer(_))
    }

    /// Returns `true` if the error is an `ExpectedString` error.
    pub fn is_expected_string(&self) -> bool {
        matches!(self, Error::ExpectedString(_))
    }

    /// Returns `true` if the error is a `Pointer` error.
    pub fn is_invalid_pointer(&self) -> bool {
        matches!(self, Error::InvalidPointer(_))
    }

    /// Returns `true` if the error is an `Uri` error.
    pub fn is_invalid_uri(&self) -> bool {
        matches!(self, Error::InvalidUri(_))
    }

    /// Returns `true` if the error is a `MetaSchema` error.
    pub fn is_meta_schema(&self) -> bool {
        matches!(self, Error::MetaSchema(_))
    }

    /// Returns `true` if the error is a `MissingRequiredVocabulary` error.
    pub fn is_missing_required_vocabulary(&self) -> bool {
        matches!(self, Error::MissingRequiredVocabulary(_))
    }

    /// Returns `true` if the error is a `SchemaNotSetup` error.
    pub fn is_schema_not_setup(&self) -> bool {
        matches!(self, Error::SchemaNotSetup(_))
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
        Error::InvalidUri(err)
    }
}

impl From<UnknownMetaSchema> for Error {
    fn from(err: UnknownMetaSchema) -> Self {
        Error::MetaSchema(MetaSchemaError::UnknownMetaSchema(err))
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
        Error::InvalidPointer(err)
    }
}

impl From<MalformedPointerError> for Error {
    fn from(err: MalformedPointerError) -> Self {
        Error::InvalidPointer(err.into())
    }
}

impl From<ExpectedStringError> for Error {
    fn from(err: ExpectedStringError) -> Self {
        Self::ExpectedString(err)
    }
}

impl From<MetaSchemaError> for Error {
    fn from(err: MetaSchemaError) -> Self {
        Error::MetaSchema(err)
    }
}

impl From<MissingRequiredVocabularyError> for Error {
    fn from(err: MissingRequiredVocabularyError) -> Self {
        Error::MissingRequiredVocabulary(err)
    }
}

impl From<SchemaNotSetupError> for Error {
    fn from(err: SchemaNotSetupError) -> Self {
        Error::SchemaNotSetup(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(err) => Display::fmt(err, f),
            Error::Serde(err) => Display::fmt(err, f),
            Error::InvalidSchema(err) => Display::fmt(err, f),
            Error::UnindentifiedSchema(err) => Display::fmt(err, f),
            Error::InvalidPointer(err) => Display::fmt(err, f),
            Error::ExpectedString(err) => Display::fmt(err, f),
            Error::InvalidUri(err) => Display::fmt(err, f),
            Error::MetaSchema(err) => Display::fmt(err, f),
            Error::MissingRequiredVocabulary(err) => Display::fmt(err, f),
            Error::SchemaNotSetup(err) => Display::fmt(err, f),
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
            Error::InvalidPointer(err) => Some(err),
            Error::ExpectedString(err) => Some(err),
            Error::InvalidUri(err) => Some(err),
            Error::MetaSchema(err) => Some(err),
            Error::MissingRequiredVocabulary(err) => Some(err),
            Error::SchemaNotSetup(err) => Some(err),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MetaSchemaError {
    /// The `$schema` is not known to the [`Interrogator`].
    UnknownMetaSchema(UnknownMetaSchema),
    /// Expected a String value for the `"$schema"` field.
    InvalidValueForSchema(Value),
    /// Expected a String in the format of a Uri for the `"$schema"` field.
    InvalidUri(UriError),
}
impl StdError for MetaSchemaError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            MetaSchemaError::UnknownMetaSchema(err) => Some(err),
            MetaSchemaError::InvalidValueForSchema(_) => None,
            MetaSchemaError::InvalidUri(err) => Some(err),
        }
    }
}
impl Display for MetaSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaSchemaError::UnknownMetaSchema(err) => Display::fmt(err, f),
            MetaSchemaError::InvalidValueForSchema(err) => Display::fmt(err, f),
            MetaSchemaError::InvalidUri(err) => Display::fmt(err, f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnknownMetaSchema {
    pub uri: Uri,
}
impl StdError for UnknownMetaSchema {}
impl Display for UnknownMetaSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "encountered unknown $schema: {}", self.uri)
    }
}

/// Indicates that the [`Schema`] was not identified. Top-level [`Schema`]s must
/// have an `id` set.
#[derive(Debug, Clone)]
pub struct UnidentifiedSchemaError {
    /// The [`Schema`] which lacks an `id`.
    pub schema: Schema,
}

impl Display for UnidentifiedSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "schema $id not set")
    }
}

impl StdError for UnidentifiedSchemaError {}

#[derive(Clone, Debug)]
/// Indicates that no [`Applicator`](crate::Applicator) was applicable for the
/// given [`Schema`](crate::Schema).
pub struct InvalidSchemaError {
    /// The [`Schema`] for which no [`Applicator`](crate::Applicator) were found.
    pub schema: Schema,
    pub evaluation: Evaluation,
}

impl Display for InvalidSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the schema{} has validation errors",
            self.schema
                .id()
                .map_or("".to_string(), |v| format!(" [{}]", &v))
        )
    }
}

impl StdError for InvalidSchemaError {}

#[derive(Debug, Clone)]
/// Indicates that the [`Field`] expected a
/// [`Value::String`](serde_json::Value::String) but received something else.
///
/// This is applicable to reserved fields:
/// ```ignore
/// ["instanceLocation"`, "keywordLocation", `"absoluteKeywordLocation", "error"]
/// ```
pub struct ExpectedStringError {
    /// The [`Field`] which was expected to be a `String`(serde_json::Value::String).
    pub field: Field,
}

impl Display for ExpectedStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: expected string for field \"{}\".", self.field)
    }
}

impl StdError for ExpectedStringError {}

#[derive(Debug, Clone)]
pub struct SchemaNotSetupError {
    pub schema: Schema,
}
impl Display for SchemaNotSetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "schema{} is not setup; add it to an Interrogator before use",
            format_args!(
                "{}",
                self.schema
                    .id()
                    .map_or("".to_string(), |v| format!(" [{}]", &v))
            )
        )
    }
}
impl StdError for SchemaNotSetupError {}

#[derive(Debug, Clone)]
pub struct MissingRequiredVocabularyError {
    pub schema_id: Uri,
    pub vocabulary: String,
}
impl Display for MissingRequiredVocabularyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "missing required vocabulary [{}] for MetaSchema [{}]",
            self.vocabulary, self.schema_id
        )
    }
}
impl StdError for MissingRequiredVocabularyError {}
