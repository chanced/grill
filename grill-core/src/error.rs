//! Logical errors which can occur while interacting this library.
//!
//!
use std::collections::HashMap;

use jsonptr::Pointer;
#[doc(no_inline)]
pub use jsonptr::{Error as ResolvePointerError, MalformedPointerError};
#[doc(no_inline)]
pub use url::ParseError as UrlError;
pub use urn::Error as UrnError;

use crate::{uri::AbsoluteUri, Output, Uri};
use serde_json::Value;
use std::{
    error::Error as StdError,
    fmt::{self, Debug, Display},
    num::ParseIntError,
    ops::Deref,
    string::FromUtf8Error,
};
use thiserror::Error;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           LocateSchemaError                           ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                           ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while attempting to locate subschemas.
#[derive(Debug, Error)]
#[error("error locating subschemas")]
pub enum LocateSchemasError {
    /// An error occurred locating subschemas due to a malformed anchor.
    MalformedAnchor(#[from] AnchorError),
    /// An error occurred locating subschemas due to an error identifying a schema.
    FailedToIdentifySchema(#[from] IdentifyError),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                          AnchorNotEmptyError                          ║
║                          ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                          ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An anchor keyword which does not allow for non-empty values (e.g.
/// `$recursiveAnchor`) was found with a value.
#[derive(Debug, Clone, Error)]
#[error("{keyword} must be an empty string; found {value}")]
pub struct AnchorNotEmptyError {
    /// The [`Keyword`] of the anchor.
    pub keyword: &'static str,

    /// The value of the anchor.
    pub value: Box<Value>,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                       AnchorInvalidLeadCharError                      ║
║                       ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                      ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An anchor keyword which requires that the value must start with either a
/// letter (`([A-Za-z])`) or an underscore (`_`) (e.g. `$anchor` and
/// `$dynamicAnchor`) was found with an invalid leading character.
#[derive(Debug, Clone, Error)]
#[error("{keyword} must start with either a letter (([A-Za-z])) or an underscore (_); found {value} for {char}")]
pub struct AnchorInvalidLeadCharError {
    /// The value of the anchor.
    pub value: String,
    /// The [`Keyword`] of the anchor.
    pub keyword: &'static str,
    /// The character which caused the error.
    pub char: char,
}
/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                         AnchorInvalidCharError                        ║
║                         ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                        ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An anchor keyword contained an invalid character.
#[derive(Debug, Clone, Error)]
#[error("{keyword} may only contain letters (([A-Za-z])), digits ([0-9]), hyphens ('-'), underscores ('_'), and periods ('.'); found {value} for {char}")]
pub struct AnchorInvalidCharError {
    /// The value of the anchor.
    pub value: String,
    /// The [`Keyword`] of the anchor.
    pub keyword: &'static str,
    /// The character which caused the error.
    pub char: char,
}
/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                              AnchorError                              ║
║                              ¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An issue with an anchor keyword (e.g. `$anchor`, `$dynamicAnchor`,
/// `$recursiveAnchor`) occurred
#[derive(Debug, Error)]
pub enum AnchorError {
    /// An anchor keyword which does not allow for empty values (e.g. `$anchor`,
    /// `$dynamicAnchor`) was found with an empty string.
    #[error("{0} must be a non-empty string")]
    Empty(&'static str),

    /// An anchor keyword which does not allow for non-empty values (e.g.
    /// `$recursiveAnchor`) was found with a value.
    #[error(transparent)]
    ValueNotAllowed(#[from] AnchorNotEmptyError),

    /// `$anchor` and `$dynamicAnchor` must start with either a letter
    /// (`([A-Za-z])`) or an underscore (`_`).
    #[error(transparent)]
    InvalidLeadingCharacter(#[from] AnchorInvalidLeadCharError),

    /// `$anchor` and `$dynamicAnchor` may only contain letters (`([A-Za-z])`),
    /// digits (`[0-9]`), hyphens (`'-'`), underscores (`'_'`), and periods
    /// (`'.'`).
    #[error(transparent)]
    InvalidChar(#[from] AnchorInvalidCharError),

    /// The anchor value was not of the expected type.
    #[error(transparent)]
    InvalidType(#[from] InvalidTypeError),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             PointerError                              ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred parsing or resolving a JSON [`Pointer`].
#[derive(Debug, Error)]
pub enum PointerError {
    #[error(transparent)]
    /// The JSON [`Pointer`] was malformed.
    ParsingFailed(#[from] MalformedPointerError),

    #[error(transparent)]
    /// The JSON [`Pointer`] could not be resolved.
    ResolutionFailed(#[from] ResolvePointerError),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                              SourceError                              ║
║                              ¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while attempting to add a new a schema source.
#[derive(Debug, Error)]
pub enum SourceError {
    /// An error occurred while attempting to deserialize a source.
    #[error(transparent)]
    DeserializationFailed(#[from] DeserializationError),

    /// Multiple sources with the same URI but different values were provided.
    #[error(transparent)]
    SourceConflict(#[from] SourceConflictError),

    /// Resolution of a source failed
    #[error(transparent)]
    ResolutionFailed(#[from] ResolveErrors),

    /// The source was not valid UTF-8.
    #[error(transparent)]
    InvalidUtf8(#[from] FromUtf8Error),

    /// The source's URI was not able to be parsed
    #[error(transparent)]
    UriFailedToParse(#[from] UriError),

    /// The source URI contains afragment which is not allowed.
    #[error("source URIs may not contain fragments, found \"{0}\"")]
    UnexpectedUriFragment(AbsoluteUri),

    /// A JSON Pointer failed to parse or resolve.
    #[error("failed to locate json pointer path:\n{0}")]
    PointerFailedToParseOrResolve(#[from] PointerError),

    /// Failed to create a source link.
    #[error(transparent)]
    FailedToLinkSource(#[from] LinkError),
}
impl From<jsonptr::MalformedPointerError> for SourceError {
    fn from(err: jsonptr::MalformedPointerError) -> Self {
        Self::PointerFailedToParseOrResolve(err.into())
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                          SourceConflictError                          ║
║                          ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                          ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Multiple sources with the same URI were provided.
#[derive(Debug, Error)]
#[error("duplicate source provided: {uri}")]
pub struct SourceConflictError {
    /// The URI of the duplicate source.
    pub uri: AbsoluteUri,
}

impl From<ResolveError> for SourceError {
    fn from(value: ResolveError) -> Self {
        Self::ResolutionFailed(ResolveErrors {
            errors: vec![value],
        })
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                          DeserializationError                         ║
║                          ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                         ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while attempting to deserialize a source.
#[derive(Debug, Error)]
#[error("failed to deserialize source \"{uri}\":\n\t{error}")]
pub struct DeserializationError {
    /// The [`AbsoluteUri`] of the source.
    pub uri: AbsoluteUri,

    /// The underlying [`DeserializeError`].
    #[source]
    pub error: DeserializeError,
}

impl DeserializationError {
    /// Create a new [`DeserializationError`].
    #[must_use]
    pub fn new(uri: AbsoluteUri, error: DeserializeError) -> Self {
        Self { uri, error }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                              DialectsError                            ║
║                              ¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Possible errors that may occur while creating a
/// [`Dialects`](crate::dialect::Dialects)
#[derive(Debug, Error)]
pub enum DialectsError {
    /// No dialects were provided.
    #[error("no dialects were provided")]
    Empty,
    /// An error occurred creating a [`Dialect`].
    #[error(transparent)]
    Dialect(#[from] DialectError),
    /// Multiple [`Dialect`]s with the same [`AbsoluteUri`] id were provided.
    #[error("duplicate dialect id provided: {0}")]
    Duplicate(AbsoluteUri),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               DialectError                            ║
║                               ¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Possible errors that may occur while creating a
/// [`Dialect`](crate::dialect::Dialect)
#[derive(Debug, Error)]
pub enum DialectError {
    /// The default [`Dialect`] was not found.
    #[error("default dialect not found: {0}")]
    DefaultNotFound(AbsoluteUri),

    /// A [`Dialect`] ID contained a non-empty fragment.
    #[error("dialect ids may not contain fragments; found: \"{0}\"")]
    FragmentedId(AbsoluteUri),

    /// `Dialect` was constructed but a metaschema with the `Dialect`'s `id` was
    /// not present.
    #[error(
        "the primary metaschema with id \"{0}\" was not found within the supplied metaschemas"
    )]
    PrimaryMetaschemaNotFound(AbsoluteUri),

    /// Exactly one [`Keyword`](crate::keyword::Keyword) must implement
    /// implement [`is_pertinent_to`](`crate::keyword::Keyword::is_pertinent_to`) but none were provided.
    #[error("exactly one `Keyword` must implemenet the `is_pertinent_to` method; none were found")]
    IsPertinentToNotImplemented(AbsoluteUri),

    /// Exactly one [`Keyword`](crate::keyword::Keyword) must implement
    /// implement [`dialect`](`crate::keyword::Keyword::dialect`) but none were provided.
    #[error("at least one `Keyword` must implement the `dialect` method; none were found")]
    DialectNotImplemented(AbsoluteUri),

    /// At least one [`Keyword`](crate::keyword::Keyword) must implement
    /// implement [`identify`](`crate::keyword::Keyword::identify`) but none were provided.
    #[error("at least one `Keyword` must implement the `identify` method; none were found")]
    IdentifyNotImplemented(AbsoluteUri),

    /// An [`AbsoluteUri`] failed to parse.
    #[error(transparent)]
    UriPFailedToParse(#[from] UriError),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                LinkError                              ║
║                                ¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Failed to associate a schema to a location within a source.
#[derive(Debug, Error)]
pub enum LinkError {
    /// A conflict occurred (i.e. a source was linked from multiple locations).
    #[error(transparent)]
    Conflict(#[from] LinkConflictError),

    /// Failed to resolve a path
    #[error("failed to resolve link path: {0}")]
    PathNotFound(#[from] jsonptr::Error),

    /// Failed to resolve a URI
    #[error("source not found: {0}")]
    NotFound(AbsoluteUri),
}

/// Source was linked from multiple schemas.
#[derive(Debug, Error)]
#[error("source address {:?} @ {:?} already assigned to {:?} @ {:?}", new.0, new.1, existing.0, existing.1)]
pub struct LinkConflictError {
    /// The existing schema location.
    pub existing: (AbsoluteUri, Pointer),
    /// The new schema location.
    pub new: (AbsoluteUri, Pointer),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               BuildError                              ║
║                               ¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Various errors that can occur while building an [`Interrogator`](crate::Interrogator).
#[derive(Debug, Error)]
pub enum BuildError {
    #[error(transparent)]
    /// A [`Schema`](crate::schema::Schema) failed to compile.
    FailedToCompile(#[from] CompileError),

    #[error(transparent)]
    /// An issue with [`Dialect`]s occurred.
    FailedToCreateDialects(#[from] DialectsError),

    #[error(transparent)]
    /// An error occurred while adding, resolving, or deserializing a
    /// [`Source`](crate::source::Source).
    FailedToSource(#[from] SourceError),

    /// Failed to parse a number
    #[error(transparent)]
    FailedToParseNumber(#[from] NumberError),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               NumberError                             ║
║                               ¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while parsing a [`Number`] as a [`num::BigRational`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum NumberError {
    /// Failed to parse exponent of a number.
    #[error("failed to parse exponent of number \"{value}\":\n\t{source}")]
    FailedToParseExponent {
        /// the value of the string being parsed
        value: String,
        /// the underlying error
        #[source]
        source: ParseIntError,
    },
    /// Unexpected character found in a number.
    #[error("failed to parse number \"{value}\":\n\tunexpected character: '{character}' at index {index}")]
    UnexpectedChar {
        /// the value of the string being parsed
        value: String,
        /// the character which caused the error
        character: char,
        /// the index of the character which caused the error
        index: usize,
    },
    /// The number is not an integer.
    #[error("failed to parse number \"{value}\":\n\tnot an integer")]
    NotAnInteger {
        /// value of string being parsed
        value: String,
    },
    #[cfg(not(target_pointer_width = "64"))]
    #[error("exponent ({value}) exceeds maximum value for non-64-bit architecture")]
    ExponentTooLarge(OverflowError<u64, { usize::MAX as u64 }>),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             EvaluateError                             ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while evaluating a [`Value`].
#[derive(Debug, Error)]
pub enum EvaluateError {
    /// Failed to parse a [`Number`] in a [`].
    #[error(transparent)]
    ParseNumber(#[from] NumberError),

    /// Failed to evaluate a regular expression.
    #[error(transparent)]
    Regex(#[from] regex::Error),

    /// A [`Key`] was provided that is not known to the `Interrogator`
    #[error(transparent)]
    UnknownKey(#[from] UnknownKeyError),

    /// A custom error occurred in a [`Keyword`](crate::keyword::Keyword).
    #[error("{source}")]
    Custom {
        /// `Box<dyn std::error::Error>`
        #[source]
        source: Box<anyhow::Error>,
        /// The [`Value`] which failed to evaluate.
        value: Option<Box<Value>>,
    },
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           DeserializeError                            ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Contains one or more errors that occurred during deserialization.
#[derive(Debug, Default)]
pub struct DeserializeError {
    /// A table of errors keyed by the format which failed to deserialize.
    pub formats: HashMap<&'static str, erased_serde::Error>,
}

impl DeserializeError {
    /// Adds a [`erased_serde::Error`], key'ed by `format` to the table of
    /// deserialization errors.
    pub fn add(&mut self, format: &'static str, err: erased_serde::Error) {
        self.formats.insert(format, err);
    }
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to deserialize")?;
        for (format, err) in &self.formats {
            write!(f, "\n\t{format}: {err}")?;
        }
        Ok(())
    }
}

impl StdError for DeserializeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.formats.iter().next().map(|(_, err)| err as _)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             ResolveErrors                             ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A list of errors, one per implementation of
/// [`Resolve`](crate::resolve::Resolve) attached to the
/// [`Interrogator`](crate::Interrogator), indicating why a source failed to
/// resolve.
#[derive(Debug, Default)]
pub struct ResolveErrors {
    /// A list of errors, one per implementation of [`Resolve`].
    pub errors: Vec<ResolveError>,
}
impl IntoIterator for ResolveErrors {
    type Item = ResolveError;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'a> IntoIterator for &'a ResolveErrors {
    type Item = &'a ResolveError;
    type IntoIter = std::slice::Iter<'a, ResolveError>;
    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}
impl Deref for ResolveErrors {
    type Target = Vec<ResolveError>;
    fn deref(&self) -> &Self::Target {
        &self.errors
    }
}

impl From<ResolveError> for ResolveErrors {
    fn from(error: ResolveError) -> Self {
        Self {
            errors: vec![error],
        }
    }
}
impl ResolveErrors {
    #[must_use]
    /// Create a new [`ResolveErrors`].
    pub fn new() -> Self {
        Self {
            errors: Vec::default(),
        }
    }
    /// Appends a new [`ResolveError`] to the list of errors.
    pub fn push(&mut self, err: ResolveError) {
        self.errors.push(err);
    }
    /// Appends a new [`NotFoundError`] to the list of errors.
    pub fn push_not_found(&mut self, uri: AbsoluteUri) {
        self.errors.push(ResolveError::not_found(uri));
    }

    /// Appends a new [`ResolveError`] from a [`ResolveErrorSource`] to the list
    /// of errors.
    pub fn push_new(&mut self, err: impl Into<ResolveErrorSource>, uri: AbsoluteUri) {
        self.errors.push(ResolveError {
            source: err.into(),
            uri,
            referring_location: None,
        });
    }

    /// Sets the `referring_location` of each `ResolveError` to `referring_location`.
    pub fn set_referring_location(&mut self, referring_location: AbsoluteUri) {
        for err in &mut self.errors {
            err.referring_location = Some(referring_location.clone());
        }
    }
}

impl Display for ResolveErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to resolve schema")?;
        for err in &self.errors {
            write!(f, "\n\t{err}")?;
        }
        Ok(())
    }
}
impl StdError for ResolveErrors {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.errors.first().map(|err| err as _)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             ResolveError                              ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while attempting to resolve a source within the source.
#[derive(Debug, Error)]
#[error("failed to resolve source \"{uri}\"\n\ncaused by:\n\t{source}")]
pub struct ResolveError {
    /// The source of the error.
    #[source]
    pub source: ResolveErrorSource,

    /// The [`AbsoluteUri`] of the source which was not able to be resolved.
    pub uri: AbsoluteUri,

    /// The [`AbsoluteUri`] of the referring keyword which was not found, if
    /// any.
    ///
    /// The path of the keyword can be found as a fragment of the URI.
    pub referring_location: Option<AbsoluteUri>,
}

impl ResolveError {
    /// Create a new [`ResolveError`].
    pub fn new(err: impl Into<ResolveErrorSource>, uri: AbsoluteUri) -> Self {
        Self {
            source: err.into(),
            uri,
            referring_location: None,
        }
    }
    /// Creates a new [`ResolveError`] with a [`ResolveErrorSource::NotFound`]
    #[must_use]
    pub fn not_found(uri: AbsoluteUri) -> Self {
        Self {
            source: NotFoundError(uri.clone()).into(),
            uri,
            referring_location: None,
        }
    }
    /// Sets the `referring_location` of the `ResolveError` to `referring_location`.
    pub fn set_referring_location(&mut self, referring_location: AbsoluteUri) {
        self.referring_location = Some(referring_location);
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                          ResolveErrorSource                           ║
║                          ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                           ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// The source of a [`ResolveError`]
#[derive(Debug, Error)]
pub enum ResolveErrorSource {
    /// The [`std::io::Error`] which occurred while resolving a source.
    #[error(transparent)]
    IoFailed(#[from] std::io::Error),

    /// The [`reqwest::Error`] which occurred while resolving a source.
    #[error(transparent)]
    ReqwestFailed(#[from] reqwest::Error),

    /// The path, as a JSON [`Pointer`], failed to resolve.
    #[error(transparent)]
    PointerMalformed(#[from] PointerError),

    /// A source or schema could not be found.
    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    /// Any other error which occurred while resolving a source.
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}

impl From<MalformedPointerError> for ResolveErrorSource {
    fn from(err: MalformedPointerError) -> Self {
        Self::PointerMalformed(err.into())
    }
}

impl From<jsonptr::Error> for ResolveErrorSource {
    fn from(err: jsonptr::Error) -> Self {
        Self::PointerMalformed(err.into())
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             CompileError                              ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while compiling a schema.
#[derive(Debug, Error)]
pub enum CompileError {
    /// The schema failed evaluation, represented by the failed [`Output`].
    #[error("schema failed evaluation:\n{0}")]
    SchemaInvalid(Output<'static>),

    /// Failed to identify a schema
    #[error(transparent)]
    SchemaIdentificationFailed(#[from] IdentifyError),

    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    #[error(transparent)]
    DialectNotKnown(#[from] DialectUnknownError),

    /// Failed to parse a [`Uri`] or
    /// [`AbsoluteUri`](`crate::uri::AbsoluteUri`)
    #[error(transparent)]
    FailedToParseUri(#[from] UriError),

    /// All attached implementations of [`Resolve`](crate::resolve::Resolve)
    /// failed to resolve a source.
    #[error(transparent)]
    FailedToResolve(#[from] ResolveErrors),

    /// Failed to resolve or deserialize a source
    #[error(transparent)]
    FailedToSource(#[from] SourceError),

    #[error(transparent)]
    FailedToEvaluateSchema(#[from] EvaluateError),

    /// Failed to locate subschemas within a schema.
    #[error(transparent)]
    FailedToLocateSubschemas(#[from] LocateSchemasError),

    /// If a [`Schema`] does not have an identifier, then the first [`AbsoluteUri`]
    /// returned from [`Dialect::locate`](`crate::schema::Dialect`) must have the
    /// schema's path as a JSON [`Pointer`].
    #[error("expected schema URI to contain path; found {uri}")]
    LocatedUriMalformed {
        /// The [`MalformedPointerError`] which occurred.
        source: MalformedPointerError,
        /// The [`AbsoluteUri`] which was returned from
        uri: AbsoluteUri,
    },

    #[error(transparent)]
    /// A [`Schema`] contains a cyclic dependency.
    CyclicGraph(#[from] CyclicDependencyError),

    /// Failed to link sources
    #[error("failed to create source link: {0}")]
    FailedToLinkSource(#[from] LinkError),

    /// Could not locate an anchor referenced in a schema
    #[error(transparent)]
    UnknownAnchor(#[from] UnknownAnchorError),

    /// Failed to parse an anchor field
    #[error(transparent)]
    FailedToParseAnchor(#[from] AnchorError),

    /// Failed to find a schema with the given uri
    #[error("schema not found: \"{0}\"")]
    SchemaNotFound(AbsoluteUri),

    /// Failed to parse a number
    #[error(transparent)]
    FailedToParseNumber(#[from] NumberError),

    /// Failed to parse json pointer path
    #[error(transparent)]
    FailedToParsePointer(#[from] PointerError),

    /// A keyword encountered a value type which was not expected
    /// and was not caught by the schema
    #[error(transparent)]
    InvalidType(#[from] InvalidTypeError),

    /// A keyword encountered a value which was not expected
    #[error(transparent)]
    UnexpectedValue(#[from] UnexpectedValueError),

    /// An error occurred while parsing a ref field (e.g. `"$ref"`,
    /// `"$recursiveRef"`, `"$recursiveAnchor"`)
    #[error(transparent)]
    RefError(#[from] RefError),

    /// A regular expression failed to parse
    #[error(transparent)]
    FailedToEvaluateRegex(#[from] regex::Error),

    /// Custom errors returned by a [`Keyword`]
    #[error(transparent)]
    Custom(#[from] Box<anyhow::Error>),
}
impl From<SourceConflictError> for CompileError {
    fn from(value: SourceConflictError) -> Self {
        Self::FailedToSource(value.into())
    }
}
impl From<MalformedPointerError> for CompileError {
    fn from(err: MalformedPointerError) -> Self {
        Self::FailedToParsePointer(err.into())
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               Expected                                ║
║                               ¯¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// The expected type of a [`Value`].
#[derive(Clone, Debug, Copy)]
pub enum Expected {
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

impl Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expected::Bool => write!(f, "Bool"),
            Expected::Number => write!(f, "Number"),
            Expected::String => write!(f, "String"),
            Expected::Array => write!(f, "Array"),
            Expected::Object => write!(f, "Object"),
            Expected::AnyOf(anyof) => {
                write!(f, "[")?;
                for (i, expected) in anyof.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{expected}")?;
                }
                write!(f, "]")
            }
        }
    }
}
/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           InvalidTypeError                            ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A [`Value`] was not of the expected type.
#[derive(Debug, Error)]
#[error("expected value with type {expected}, found {actual:?}")]
pub struct InvalidTypeError {
    /// The expected type of value.
    pub expected: Expected,
    /// The actual value.
    pub actual: Box<Value>,
}

/// A [`Value`] was .
#[derive(Debug, Error)]
#[error("unexpected value; expected {expected} found {value:?}")]
pub struct UnexpectedValueError {
    /// A description of the expected value
    pub expected: &'static str,
    /// The actual value.
    pub value: Box<Value>,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                          UnknownAnchorError                           ║
║                          ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                           ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A schema referenced an anchor which was not found.
#[derive(Debug, Error)]
#[error("unknown anchor: \"{}\" in URI \"{}\"", .anchor, .uri)]
pub struct UnknownAnchorError {
    /// The anchor which was not found.
    pub anchor: String,
    /// The URI of the keyword which referenced the anchor.
    pub uri: AbsoluteUri,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                         CyclicDependencyError                         ║
║                         ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                         ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A [`Schema`] contains a cyclic dependency.
#[derive(Debug, Error)]
#[error("schema \"{}\" contains a cyclic dependency to \"{}\"", .from, .to)]
pub struct CyclicDependencyError {
    /// The [`AbsoluteUri`] of the schema which, through transitive
    /// dependencies, creates a cycle.
    pub from: AbsoluteUri,
    /// The [`AbsoluteUri`] of the schema which is the target of the cycle.
    pub to: AbsoluteUri,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             NotFoundError                             ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A source or schema could not be found.
#[derive(Debug, Clone, Error)]
#[error("unable to resolve \"{0}\" due to not being found")]
pub struct NotFoundError(pub AbsoluteUri);

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                          DialectUnknownError                          ║
║                          ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                          ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// The schema's [`Dialect`] is not registered with the
/// [`Interrogator`](crate::Interrogator).
#[derive(Debug, Clone, Error)]
#[error("metaschema dialect not found: {}", .metaschema_id)]
pub struct DialectUnknownError {
    /// The id of the [`Dialect`] which is not registered with the
    /// [`Interrogator`](crate::Interrogator).
    pub metaschema_id: String,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               UriError                                ║
║                               ¯¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Errors which can occur when parsing or interacting with
/// [`Uri`](`crate::uri::Uri`), [`AbsoluteUri`](`crate::uri::AbsoluteUri`), or
/// [`RelativeUri`](`crate::uri::RelativeUri`).
#[derive(Debug, Clone, Error)]
pub enum UriError {
    /// an issue occurred parsing a [`Url`](`url::Url`)
    #[error(transparent)]
    FailedToParseUrl(#[from] UrlError),

    /// an issue occurred parsing a [`Urn`](`urn::Urn`)
    #[error(transparent)]
    FailedToParseUrn(#[from] UrnError),

    /// an issue occurred parsing a [`RelativeUri`](`crate::uri::RelativeUri`)
    #[error(transparent)]
    FailedToParseRelativeUri(#[from] RelativeUriError),

    /// The [`Uri`] is not absolute and cannot be made into an [`AbsoluteUri`].
    #[error("uri is not absolute: {0}")]
    NotAbsolute(Uri),

    /// An issue occurred while setting the Authority of a
    /// [`Uri`] or [`RelativeUri`](crate::uri::RelativeUri).
    #[error(transparent)]
    MalformedAuthority(#[from] AuthorityError),

    /// The scheme of a [`Uri`] or [`AbsoluteUri`] is malformed.
    #[error("invalid scheme: {0}")]
    InvalidScheme(String),
}

impl From<InvalidPortError> for UriError {
    fn from(err: InvalidPortError) -> Self {
        Self::FailedToParseRelativeUri(err.into())
    }
}
impl From<OverflowError<usize, { u32::MAX as u64 }>> for UriError {
    fn from(err: OverflowError<usize, { u32::MAX as u64 }>) -> Self {
        Self::FailedToParseRelativeUri(err.into())
    }
}

impl UriError {
    /// Returns `true` if the uri parse error is [`Url`].
    ///
    /// [`Url`]: UriParseError::Url
    #[must_use]
    pub fn is_url(&self) -> bool {
        matches!(self, Self::FailedToParseUrl { .. })
    }

    /// Returns `true` if the uri parse error is [`Urn`].
    ///
    /// [`Urn`]: UriParseError::Urn
    #[must_use]
    pub fn is_urn(&self) -> bool {
        matches!(self, Self::FailedToParseUrn { .. })
    }

    /// Returns `true` if the uri error is [`Relative`].
    ///
    /// [`Relative`]: UriError::Relative
    #[must_use]
    pub fn is_relative(&self) -> bool {
        matches!(self, Self::FailedToParseRelativeUri(..))
    }

    /// Returns `true` if the uri error is [`NotAbsolute`].
    ///
    /// [`NotAbsolute`]: UriError::NotAbsolute
    #[must_use]
    pub fn is_not_absolute(&self) -> bool {
        matches!(self, Self::NotAbsolute(..))
    }

    /// If the error is [`UriError::Url`], returns a reference to the underlying
    /// [`UrlError`].
    #[must_use]
    pub fn as_url(&self) -> Option<&UrlError> {
        if let Self::FailedToParseUrl(err) = self {
            Some(err)
        } else {
            None
        }
    }

    /// If the error is [`UriError::Urn`], returns a reference to the underlying
    /// [`UrnError`].
    #[must_use]
    pub fn as_urn(&self) -> Option<&urn::Error> {
        if let Self::FailedToParseUrn(err) = self {
            Some(err)
        } else {
            None
        }
    }

    #[must_use]
    /// If the error is [`UriError::Relative`], returns a reference to the underlying
    /// [`RelativeUriError`].
    pub fn as_relative(&self) -> Option<&RelativeUriError> {
        if let Self::FailedToParseRelativeUri(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    /// If the error is [`UriError::NotAbsolute`], returns a reference to the underlying
    /// [`UriNotAbsoluteError`].
    pub fn as_not_absolute(&self) -> Option<&Uri> {
        if let Self::NotAbsolute(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                            AuthorityError                             ║
║                            ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Returned from `set_authority` on [`Uri`], [`AbsoluteUri`], and [`RelativeUri`]
#[derive(Debug, Clone, Error)]
#[error("invalid authority: {0}")]
pub enum AuthorityError {
    /// The authority contained a path
    ContainsPath(String),
    /// The authority contained a query
    ContainsQuery(String),
    /// The authority contained a fragment
    ContainsFragment(String),
    /// The authority contained a malformed port
    InvalidPort(#[from] InvalidPortError),
    /// An error occurred while setting the `authority` of a [`Urn`](urn::Urn)
    Urn(UrnError),
    /// The username cannot be set due to the scheme of the Uri (e.g. `file`)
    UsernameNotAllowed(String),
    /// The password cannot be set due to the scheme of the Uri (e.g. `file`)
    PasswordNotAllowed(String),
    /// The host cannot be set due to the scheme of the Uri (e.g. `file`)
    PortNotAllowed(u16),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           InvalidPortError                            ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A port of a [`RelativeUri`] exceeded the maximum value of `u16`.
#[derive(Debug, Clone, Error)]
#[error("port \"{0}\" is malformed or exceeds maximum value of 65535")]
pub struct InvalidPortError(pub String);

/// Errors which can occur when parsing or modifying a
/// [`RelativeUri`](crate::uri::RelativeUri).
#[derive(Debug, Clone, Error)]
pub enum RelativeUriError {
    /// The length of the input exceeds `u32::MAX`
    #[error(transparent)]
    Overflow(#[from] OverflowError<usize, { u32::MAX as u64 }>),

    /// The decoded string is not valid UTF-8
    #[error(transparent)]
    Utf8Encoding(#[from] std::str::Utf8Error),

    /// The port of a [`RelativeUri`] exceeded the maximum value of 65535.
    #[error(transparent)]
    InvalidPort(#[from] InvalidPortError),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             IdentifyError                             ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while attempting to identify a schema
#[derive(Debug, Error)]
pub enum IdentifyError {
    /// The URI could not be parsed.
    #[error(transparent)]
    InvalidUri(#[from] UriError),

    /// The URI is not absolute (i.e. contains a non-empty fragment).
    #[error("the $id of a schema is not absolute: {0}")]
    FragmentedId(Uri),

    /// Any custom error which a [`Keyword`](crate::keyword::Keyword) may need
    /// to return.
    #[error(transparent)]
    Custom(#[from] anyhow::Error),

    /// The value of `$id` was not a string
    #[error("the {keyword} of a schema must be a string in the form of a uri; found {value:?}")]
    NotAString {
        /// The keyword which was not a string
        keyword: &'static str,
        /// The value of the keyword
        value: Box<Value>,
    },
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                         DialectNotFoundError                          ║
║                         ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                          ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A [`Dialect`] with the [`AbsoluteUri`] was not able to be found.
#[derive(Clone, Debug, Error)]
#[error("dialect not found: {id}")]
pub struct DialectNotFoundError {
    /// The [`AbsoluteUri`] of the [`Dialect`] that was not able
    /// to be found.
    pub id: AbsoluteUri,
}

impl DialectNotFoundError {
    #[must_use]
    /// Create a new [`DialectNotFoundError`].
    pub fn new(id: AbsoluteUri) -> Self {
        Self { id }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                            UnknownKeyError                            ║
║                            ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A schema [`Key`](crate::schema::Key) was not found.
///
/// If this is encountered, odds are it is because you have two
/// [`Interrogator`](crate::Interrogator)s and mismatched keys.
#[derive(Debug, Clone, Copy, Error)]
#[error("the provided key could not be found")]
pub struct UnknownKeyError;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             OverflowError                             ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A slice or string overflowed an allowed length maximum of `M`.
#[derive(Debug, Clone, Copy, Error)]
#[error("the length of a string or slice overflows the maximum of {M}, received {0}")]
pub struct OverflowError<Value, const M: u64 = { u32::MAX as u64 }>(pub Value);
impl<V, const M: u64> OverflowError<V, M> {
    /// The maximum allowed size.
    pub const MAX: u64 = M;
}

impl From<u64> for OverflowError<u64, { usize::MAX as u64 }> {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               RefError                                ║
║                               ¯¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// An error occurred while parsing a ref
#[derive(Debug, Error)]
pub enum RefError {
    /// The ref value was not a string.
    #[error(transparent)]
    UnexpectedType(#[from] InvalidTypeError),
    /// The ref value failed to parse as a URI.
    #[error(transparent)]
    UriError(#[from] UriError),
}
