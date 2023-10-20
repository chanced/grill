use ahash::AHashMap;
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
    /// An error occurred locating subschemas due to an invalid character in an
    /// anchor.
    MalformedAnchor(#[from] AnchorError),
    /// An error occurred locating subschemas due to an error identifying a schema.
    FailedToIdentifySchema(#[from] IdentifyError),
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           AnchorEmptyError                            ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An anchor keyword which does not allow for empty values (e.g. `$anchor`,
/// `$dynamicAnchor`) was found with an empty string.
#[derive(Debug, Clone, Error)]
#[error("{keyword} at {location} must not be an empty string")]
pub struct AnchorEmptyError {
    /// The location of the anchor in the form of an [`AbsoluteUri`].
    ///
    /// The keyword's path can be found as a JSON pointer in the fragment.
    pub location: AbsoluteUri,
    /// The [`Keyword`] of the anchor.
    pub keyword: &'static str,
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
#[error("{keyword} at {location} must be an empty string; found {value}")]
pub struct AnchorNotEmptyError {
    /// The location of the anchor in the form of an [`AbsoluteUri`].
    ///
    /// The keyword's path can be found as a JSON pointer in the fragment.
    pub location: AbsoluteUri,

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
#[error("{keyword} must start with either a letter (([A-Za-z])) or an underscore (_); found {value} for {character} at {location}")]
pub struct AnchorInvalidLeadCharError {
    /// The location of the anchor in the form of an [`AbsoluteUri`].
    ///
    /// The keyword's path can be found as a JSON pointer in the fragment.
    pub location: AbsoluteUri,
    /// The value of the anchor.
    pub value: String,
    /// The [`Keyword`] of the anchor.
    pub keyword: &'static str,
    /// The character which caused the error.
    pub character: char,
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
#[error("{keyword} may only contain letters (([A-Za-z])), digits ([0-9]), hyphens ('-'), underscores ('_'), and periods ('.'); found {value} for {character} at {location}")]
pub struct AnchorInvalidCharError {
    /// The location of the anchor in the form of an [`AbsoluteUri`].
    ///
    /// The keyword's path can be found as a JSON pointer in the fragment.
    pub location: AbsoluteUri,
    /// The value of the anchor.
    pub value: String,
    /// The [`Keyword`] of the anchor.
    pub keyword: &'static str,
    /// The character which caused the error.
    pub character: char,
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
#[derive(Debug, Error, Clone)]
pub enum AnchorError {
    /// An anchor keyword which does not allow for empty values (e.g. `$anchor`,
    /// `$dynamicAnchor`) was found with an empty string.
    #[error(transparent)]
    EmptyNotAllowed(#[from] AnchorEmptyError),

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
    InvalidCharacter(#[from] AnchorInvalidCharError),
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
    #[error("no dialects were provided")]
    Empty,
    #[error(transparent)]
    Dialect(#[from] DialectError),
    /// Multiple [`Dialect`]s with the same
    /// [`AbsoluteUri`] id were provided.
    #[error("duplicate dialect id provided: {0}")]
    Duplicate(DialectExistsError),
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

#[derive(Debug, Error)]
pub enum LinkError {
    #[error(transparent)]
    Conflict(#[from] LinkConflictError),

    #[error("failed to resolve link path: {0}")]
    PathNotFound(#[from] jsonptr::Error),

    #[error("source not found: {0}")]
    NotFound(AbsoluteUri),
}

#[derive(Debug, Error)]
#[error("source address {:?} @ {:?} already assigned to {:?} @ {:?}", new.0, new.1, existing.0, existing.1)]
pub struct LinkConflictError {
    pub existing: (AbsoluteUri, Pointer),
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
    Compile(#[from] CompileError),

    #[error(transparent)]
    /// An issue with [`Dialect`]s occurred.
    Dialect(#[from] DialectsError),

    #[error(transparent)]
    /// An error occurred while adding, resolving, or deserializing a
    /// [`Source`](crate::source::Source).
    Source(#[from] SourceError),
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
    #[error("failed to parse exponent of number \"{value}\":\n\t{source}")]
    FailedToParseExponent {
        value: String,
        #[source]
        source: ParseIntError,
    },
    #[error("failed to parse number \"{value}\":\n\tunexpected character: '{character}' at index {index}")]
    UnexpectedChar {
        value: String,
        character: char,
        index: usize,
    },
    #[error("failed to parse number \"{value}\":\n\tnot an integer")]
    NotAnInteger { value: String },
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
    Regex(EvaluateRegexError),

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
║                              RegexError                               ║
║                              ¯¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred with a regular expression, evaluated with either the `regex`
/// or `fancy_regex` crate.
#[derive(Debug, Error)]
pub enum RegexError {
    #[error(transparent)]
    /// An error from the [`regex`] crate.
    Regex(#[from] regex::Error),

    #[error(transparent)]
    /// An error from the [`fancy_regex`] crate.
    FancyRegex(#[from] fancy_regex::Error),
}

/// A regular expression failed to evaluate against a [`Value`].
#[derive(Debug, Error)]
#[error("failed to evaluate regex \"{regex}\" against value \"{value:?}\":\n\t{source}")]
pub struct EvaluateRegexError {
    /// the regular expression
    pub regex: String,
    /// The value which the regex failed to evaluate against.
    pub value: Option<Box<Value>>,
    /// The underlying regex error.
    pub source: fancy_regex::Error,
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
    pub formats: AHashMap<&'static str, erased_serde::Error>,
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
    pub fn new() -> Self {
        Self {
            errors: Vec::default(),
        }
    }
    /// Appends a new [`ResolveError`] to the list of errors.
    pub fn push(&mut self, err: ResolveError) {
        self.errors.push(err);
    }

    pub fn push_not_found(&mut self, uri: AbsoluteUri) {
        self.errors.push(ResolveError::not_found(uri));
    }

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
    pub fn new(err: impl Into<ResolveErrorSource>, uri: AbsoluteUri) -> Self {
        Self {
            source: err.into(),
            uri,
            referring_location: None,
        }
    }

    #[must_use]
    pub fn not_found(uri: AbsoluteUri) -> Self {
        Self {
            source: NotFoundError(uri.clone()).into(),
            uri,
            referring_location: None,
        }
    }

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

    /// Failed to locate subschemas within a schema.
    #[error(transparent)]
    FailedToLocateSubschemas(#[from] LocateSchemasError),

    /// If a [`Schema`] does not have an identifier, then the first [`AbsoluteUri`]
    /// returned from [`Dialect::locate`](`crate::schema::Dialect`) must have the
    /// schema's path as a JSON [`Pointer`].
    #[error(transparent)]
    LocatedUriMalformed(#[from] LocatedSchemaUriPointerError),

    #[error(transparent)]
    /// A [`Schema`] contains a cyclic dependency.
    CyclicGraph(#[from] CyclicDependencyError),

    /// Failed to link sources
    #[error("failed to create source link: {0}")]
    FailedToLinkSource(#[from] LinkError),

    /// Could not locate an anchor referenced in a schema
    #[error(transparent)]
    AnchorNotFound(#[from] UnknownAnchorError),

    /// Failed to parse an anchor field
    #[error(transparent)]
    FailedToParseAnchor(#[from] AnchorError),

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

    /// An error occurred while parsing a ref field (e.g. `"$ref"`,
    /// `"$recursiveRef"`, `"$recursiveAnchor"`)
    #[error(transparent)]
    RefError(#[from] RefError),

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

#[derive(Clone, Copy, Debug, strum::EnumVariantNames, strum::Display)]
pub enum Expected {
    Bool,
    Number,
    String,
    Array,
    Object,
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

#[derive(Debug, Error)]
#[error("expected value with type {expected}, found {actual:?}")]
pub struct InvalidTypeError {
    pub expected: Expected,
    pub actual: Value,
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

#[derive(Debug, Error)]
#[error("unknown anchor: \"{}\" in URI \"{}\"", .anchor, .uri)]
pub struct UnknownAnchorError {
    pub anchor: String,
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
#[derive(Debug, Error)]
#[error("schema \"{}\" contains a cyclic dependency to \"{}\"", .from, .to)]
pub struct CyclicDependencyError {
    pub from: AbsoluteUri,
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
    ContainsPath(String),
    ContainsQuery(String),
    ContainsFragment(String),
    InvalidPort(#[from] InvalidPortError),
    Urn(UrnError),
    /// The username cannot be set due to the scheme of the Uri (e.g. `file`)
    UsernameNotAllowed(String),
    /// The password cannot be set due to the scheme of the Uri (e.g. `file`)
    PasswordNotAllowed(String),
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
        keyword: &'static str,
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
║                     LocatedSchemaUriPointerError                      ║
║                     ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                      ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug, Error)]
#[error("expected schema URI to contain path; found {uri}")]
pub struct LocatedSchemaUriPointerError {
    #[source]
    pub source: MalformedPointerError,
    pub uri: AbsoluteUri,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                          DialectExistsError                           ║
║                          ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                           ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug, Clone, Error)]
#[error("dialect already exists with id \"{id}\"")]
pub struct DialectExistsError {
    pub id: AbsoluteUri,
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
#[derive(Debug, Error)]
pub enum RefError {
    #[error(transparent)]
    UnexpectedType(#[from] InvalidTypeError),
    #[error(transparent)]
    UriError(#[from] UriError),
}
