//! Logical errors which can occur while interacting this library.
//!
//!
//!
use grill_uri::error::UriError;
use jsonptr::Pointer;
#[doc(no_inline)]
pub use jsonptr::{Error as ResolvePointerError, MalformedPointerError};
use snafu::Backtrace;
use snafu::Snafu;
use std::collections::HashMap;

use crate::Key;
use crate::{schema::Anchor, uri::AbsoluteUri, uri::Uri};
use serde_json::Value;
use std::{
    error::Error as StdError,
    fmt::{self, Debug, Display},
    num::ParseIntError,
    ops::Deref,
    string::FromUtf8Error,
};

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             InvalidAnchor                             ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An issue with an anchor keyword (e.g. `$anchor`, `$dynamicAnchor`,
/// `$recursiveAnchor`) occurred
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(Ctx)), module)]
pub enum AnchorError {
    /// An anchor keyword which does not allow for empty values (e.g. `$anchor`,
    /// `$dynamicAnchor`) was found with an empty string.
    #[snafu(display("{keyword} must be a non-empty string"))]
    Empty {
        keyword: &'static str,
        backtrace: Backtrace,
    },

    /// An anchor keyword which does not allow for non-empty values (e.g.
    /// `$recursiveAnchor`) was found with a value.
    #[snafu(display("{keyword} must be an empty string; found {value}"))]
    ValueNotAllowed {
        /// The [`Keyword`] of the anchor.
        keyword: &'static str,
        /// The value of the anchor.
        value: Box<Value>,
        backtrace: Backtrace,
    },

    /// `$anchor` and `$dynamicAnchor` must start with either a letter
    /// (`([A-Za-z])`) or an underscore (`_`).
    #[snafu(display("{keyword} must start with either a letter (([A-Za-z])) or an underscore (_); found {value} for {char}"))]
    InvalidLeadingCharacter {
        /// The value of the anchor.
        value: String,
        /// The [`Keyword`] of the anchor.
        keyword: &'static str,
        /// The character which caused the error.
        char: char,
        backtrace: Backtrace,
    },

    /// An anchor keyword contained an invalid character.
    ///
    /// `$anchor` and `$dynamicAnchor` may only contain letters (`([A-Za-z])`),
    /// digits (`[0-9]`), hyphens (`'-'`), underscores (`'_'`), and periods
    /// (`'.'`).
    #[snafu(display("{keyword} may only contain letters (([A-Za-z])), digits ([0-9]), hyphens ('-'), underscores ('_'), and periods ('.'); found {value} for {char}"))]
    InvalidChar {
        /// The value of the anchor.
        value: String,
        /// The [`Keyword`] of the anchor.
        keyword: &'static str,
        /// The character which caused the error.
        char: char,
        backtrace: Backtrace,
    },

    /// The anchor value was not of the expected type.
    #[snafu(display("invalid anchor: {}", source))]
    InvalidType {
        source: InvalidTypeError,
        backtrace: Backtrace,
    },

    #[snafu(display("duplicate anchor found: \"{}\"", existing.name))]
    Duplicate {
        existing: Anchor,
        duplicate: Anchor,
        backtrace: Backtrace,
    },
}

// /// An error occurred parsing or resolving a JSON [`Pointer`].
// #[derive(Debug, snafu)]
// pub enum PointerError {
//     #[snafu(transparent)]
//     /// The JSON [`Pointer`] was malformed.
//     ParsingFailed{ #[snafu(backtrace)] source: MalformedPointerError },

//     #[snafu(transparent)]
//     /// The JSON [`Pointer`] could not be resolved.
//     ResolutionFailed{ #[snafu(backtrace)] source: ResolvePointerError },
// }

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             FailedToSource                            ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while attempting to add a new a schema source.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(Ctx)), module)]
pub enum SourceError {
    /// An error occurred while attempting to deserialize a source.
    #[snafu(display("failed to deserialize source \"{uri}\":\n\t{source}"))]
    DeserializationFailed {
        /// The [`AbsoluteUri`] of the source.
        uri: AbsoluteUri,
        /// The underlying [`DeserializeError`].
        source: DeserializeError,
        backtrace: Backtrace,
    },

    /// Resolution of a source failed
    #[snafu(transparent)]
    ResolutionFailed {
        #[snafu(backtrace)]
        source: ResolveErrors,
    },

    /// The source was not valid UTF-8.
    #[snafu(display("source is not valid UTF-8: {source}"))]
    InvalidUtf8 {
        source: FromUtf8Error,
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// The source's URI was not able to be parsed
    #[snafu(display("failed to parse source URI: {source}"))]
    UriFailedToParse {
        #[snafu(backtrace)]
        source: UriError,
    },

    /// The source URI contains afragment which is not allowed.
    #[snafu(display("source URIs may not contain fragments, found \"{uri}\""))]
    UnexpectedUriFragment {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// A JSON Pointer failed to parse or resolve.
    #[snafu(display("failed to parse json pointer: {source}"))]
    PointerFailedToParse {
        source: MalformedPointerError,
        backtrace: Backtrace,
    },

    /// A JSON Pointer failed to resolve.
    #[snafu(display("failed to resolve json pointer: {source}"))]
    PointerFailedToResolve {
        source: ResolvePointerError,
        backtrace: Backtrace,
    },

    /// A conflict occurred (i.e. a source was linked from multiple locations).
    #[snafu(display(
        "source address {:?} @ {:?} already assigned to {:?}",
        uri,
        new_path,
        existing_path
    ))]
    SourceConflict {
        uri: AbsoluteUri,
        /// The existing schema location.
        existing_path: Pointer,
        /// The new schema location.
        new_path: Pointer,
    },
    /// Failed to resolve a path
    #[snafu(display("failed to resolve link path: {source}"))]
    PathNotFound {
        source: jsonptr::Error,
        backtrace: Backtrace,
    },

    /// Failed to resolve a URI
    #[snafu(display("source not found: \"{uri}\""))]
    SourceNotFound {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// An unknown anchor (non-pointer fragment of a URI) was encountered
    #[snafu(display("unknown anchor: \"{anchor}\" in URI \"{uri}\""))]
    UnknownAnchor {
        /// The anchor which was not found.
        anchor: String,
        /// The URI of the keyword which referenced the anchor.
        uri: AbsoluteUri,
    },
}
impl From<jsonptr::MalformedPointerError> for SourceError {
    fn from(err: jsonptr::MalformedPointerError) -> Self {
        Self::PointerFailedToParseOrResolve(err.into())
    }
}

impl From<ResolveError> for SourceError {
    fn from(value: ResolveError) -> Self {
        Self::ResolutionFailed(ResolveErrors {
            sources: vec![value],
            backtrace: Backtrace::capture(),
        })
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
#[derive(Debug, Snafu)]
pub enum DialectsError {
    /// No dialects were provided.
    #[snafu(display("no dialects were provided"))]
    Empty { backtrace: Backtrace },
    /// An error occurred creating a [`Dialect`].
    #[snafu(transparent)]
    Dialect {
        #[snafu(backtrace)]
        source: DialectError,
    },
    /// Multiple [`Dialect`]s with the same [`AbsoluteUri`] id were provided.
    #[snafu(display("duplicate dialect id provided: {uri}"))]
    Duplicate {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },
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
#[derive(Debug, Snafu)]
pub enum DialectError {
    /// The default [`Dialect`] was not found.
    #[snafu(display("default dialect not found: {uri}"))]
    DefaultNotFound {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// A [`Dialect`] ID contained a non-empty fragment.
    #[snafu(display("dialect ids may not contain fragments; found: \"{uri}\""))]
    FragmentedId {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// `Dialect` was constructed but a metaschema with the `Dialect`'s `id` was
    /// not present.
    #[snafu(display(
        "primary metaschema with id \"{uri}\" not found within the supplied metaschemas"
    ))]
    PrimaryMetaschemaNotFound {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// Exactly one [`Keyword`](crate::keyword::Keyword) must implement
    /// implement [`is_pertinent_to`](`crate::keyword::Keyword::is_pertinent_to`) but none were provided.
    #[snafu(display(
        "exactly one `Keyword` must implemenet the `is_pertinent_to` method; none were found"
    ))]
    IsPertinentToNotImplemented {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// Exactly one [`Keyword`](crate::keyword::Keyword) must implement
    /// implement [`dialect`](`crate::keyword::Keyword::dialect`) but none were provided.
    #[snafu(display(
        "at least one `Keyword` must implement the `dialect` method; none were found"
    ))]
    DialectNotImplemented {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// At least one [`Keyword`](crate::keyword::Keyword) must implement
    /// implement [`identify`](`crate::keyword::Keyword::identify`) but none were provided.
    #[snafu(display(
        "at least one `Keyword` must implement the `identify` method; none were found"
    ))]
    IdentifyNotImplemented {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// An [`AbsoluteUri`] failed to parse.
    #[snafu(transparent)]
    UriPFailedToParse {
        #[snafu(backtrace)]
        source: UriError,
    },
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
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(Ctx)), module)]
pub enum LinkError {
    /// A conflict occurred (i.e. a source was linked from multiple locations).
    #[snafu(display(
        "source address {:?} @ {:?} already assigned to {:?}",
        uri,
        new_path,
        existing_path
    ))]
    SourceConflict {
        uri: AbsoluteUri,
        /// The existing schema location.
        existing_path: Pointer,
        /// The new schema location.
        new_path: Pointer,
    },
    /// Failed to resolve a path
    #[snafu(display("failed to resolve link path: {source}"))]
    PathNotFound {
        source: jsonptr::Error,
        backtrace: Backtrace,
    },

    /// Failed to resolve a URI
    #[snafu(display("source not found: \"{uri}\""))]
    SourceNotFound {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },
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
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(Ctx)), module)]
pub enum BuildError<CompileError> {
    #[snafu(transparent)]
    /// A [`Schema`](crate::schema::Schema) failed to compile.
    FailedToCompile {
        #[snafu(backtrace)]
        source: CompileError,
    },

    #[snafu(transparent)]
    /// An issue with [`Dialect`]s occurred.
    FailedToCreateDialects {
        #[snafu(backtrace)]
        source: DialectsError,
    },

    #[snafu(transparent)]
    /// An error occurred while adding, resolving, or deserializing a
    /// [`Source`](crate::source::Source).
    FailedToSource {
        #[snafu(backtrace)]
        source: SourceError,
    },

    /// Failed to parse a number
    #[snafu(transparent)]
    FailedToParseNumber {
        #[snafu(backtrace)]
        source: NumberError,
    },
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
#[derive(Debug, PartialEq, Eq, Snafu)]
#[snafu(visibility(pub), context(suffix(Ctx)), module)]
pub enum NumberError {
    /// Failed to parse exponent of a number.
    #[snafu(display("failed to parse exponent of number \"{value}\":\n\t{source}"))]
    FailedToParseExponent {
        /// the value of the string being parsed
        value: String,
        /// the underlying error
        source: ParseIntError,
        backtrace: Backtrace,
    },
    /// Unexpected character found in a number.
    #[snafu(display("failed to parse number \"{value}\":\n\tunexpected character: '{character}' at index {index}"))]
    UnexpectedChar {
        /// the value of the string being parsed
        value: String,
        /// the character which caused the error
        character: char,
        /// the index of the character which caused the error
        index: usize,
        backtrace: Backtrace,
    },
    /// The number is not an integer.
    #[snafu(display("failed to parse number \"{value}\":\n\tnot an integer"))]
    NotAnInteger {
        /// value of string being parsed
        value: String,
        backtrace: Backtrace,
    },
    #[cfg(not(target_pointer_width = "64"))]
    #[snafu(display("exponent ({value}) exceeds maximum value for non-64-bit architecture"))]
    ExponentTooLarge {
        #[snafu(backtrace)]
        source: OverflowError<u64, { usize::MAX as u64 }>,
    },
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
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(Ctx)), module)]
pub enum EvaluateError {
    /// Failed to parse a [`Number`] in a [`].
    #[snafu(transparent)]
    FailedToParseNumber {
        #[snafu(backtrace)]
        source: NumberError,
    },

    /// Failed to evaluate a regular expression.
    #[snafu(display("failed to evaluate regular expression: {source}"))]
    FailedToEvalRegex {
        source: regex::Error,
        backtrace: Backtrace,
    },

    /// A [`Key`] was provided that is not known to the `Interrogator`
    #[snafu(transparent)]
    UnknownKey {
        #[snafu(backtrace)]
        source: UnknownKeyError,
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
    /// A table of errors keyed by the name of the format which failed to
    /// deserialize.
    pub sources: HashMap<&'static str, erased_serde::Error>,
}

impl DeserializeError {
    /// Adds a [`erased_serde::Error`], key'ed by `format` to the table of
    /// deserialization errors.
    pub fn add(&mut self, format: &'static str, err: erased_serde::Error) {
        self.sources.insert(format, err);
    }
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to deserialize")?;
        for (format, err) in &self.sources {
            write!(f, "\n\t{format}: {err}")?;
        }
        Ok(())
    }
}

impl StdError for DeserializeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.sources.iter().next().map(|(_, err)| err as _)
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
    pub sources: Vec<ResolveError>,
    pub backtrace: Backtrace,
}
impl IntoIterator for ResolveErrors {
    type Item = ResolveError;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.sources.into_iter()
    }
}

impl<'a> IntoIterator for &'a ResolveErrors {
    type Item = &'a ResolveError;
    type IntoIter = std::slice::Iter<'a, ResolveError>;
    fn into_iter(self) -> Self::IntoIter {
        self.sources.iter()
    }
}
impl Deref for ResolveErrors {
    type Target = Vec<ResolveError>;
    fn deref(&self) -> &Self::Target {
        &self.sources
    }
}

impl From<ResolveError> for ResolveErrors {
    fn from(error: ResolveError) -> Self {
        Self {
            sources: vec![error],
            backtrace: Backtrace::capture(),
        }
    }
}
impl ResolveErrors {
    #[must_use]
    /// Create a new [`ResolveErrors`].
    pub fn new() -> Self {
        Self {
            sources: Vec::default(),
            backtrace: Backtrace::capture(),
        }
    }
    /// Appends a new [`ResolveError`] to the list of errors.
    pub fn push(&mut self, err: ResolveError) {
        self.sources.push(err);
    }
    /// Appends a new [`NotFoundError`] to the list of errors.
    pub fn push_not_found(&mut self, uri: AbsoluteUri) {
        self.sources.push(ResolveError::not_found(uri));
    }

    /// Appends a new [`ResolveError`] from a [`ResolveErrorSource`] to the list
    /// of errors.
    pub fn push_new(&mut self, err: impl Into<ResolveErrorSource>, uri: AbsoluteUri) {
        self.sources.push(ResolveError {
            source: err.into(),
            uri,
            referring_location: None,
        });
    }

    /// Sets the `referring_location` of each `ResolveError` to `referring_location`.
    pub fn set_referring_location(&mut self, referring_location: AbsoluteUri) {
        for err in &mut self.sources {
            err.referring_location = Some(referring_location.clone());
        }
    }
}

impl Display for ResolveErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to resolve schema")?;
        for err in &self.sources {
            write!(f, "\n\t{err}")?;
        }
        Ok(())
    }
}
impl StdError for ResolveErrors {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.sources.first().map(|err| err as _)
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
#[derive(Debug, Snafu)]
#[snafu(
    display("failed to resolve source \"{uri}\"\n\ncaused by:\n\t{source}"),
    visibility(pub),
    context(suffix(Ctx)),
    module
)]
pub struct ResolveError {
    /// The source of the error.
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

    /// Sets the `referring_location` of the `ResolveError` to `referring_location`.
    pub fn set_referring_location(&mut self, referring_location: AbsoluteUri) {
        self.referring_location = Some(referring_location);
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           ResolveErrorSource                          ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                          ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// The source of a [`ResolveError`]
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(Ctx)), module)]
pub enum ResolveErrorSource {
    /// The [`std::io::Error`] which occurred while resolving a source.
    #[snafu(transparent)]
    Io {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    /// The [`reqwest::Error`] which occurred while resolving a source.
    #[snafu(transparent)]
    Reqwest {
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    /// The path, as a JSON [`Pointer`], failed to resolve.

    #[snafu(transparent)]
    PointerMalformed {
        source: MalformedPointerError,
        backtrace: Backtrace,
    },

    /// A source or schema could not be found.

    #[snafu(display("unable to resolve \"{uri}\" due to not being found"))]
    NotFound {
        /// The URI of the source which was not found.
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// Any other error which occurred while resolving a source.
    #[snafu(whatever, display("{message}"))]
    Custom {
        message: String,
        #[snafu(source(from(Box<dyn 'static + std::error::Error + Send + Sync>, Some)))]
        source: Box<dyn 'static + std::error::Error + Send + Sync>,
    },
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
║                              InvalidType                              ║
║                              ¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A [`Value`] was not of the expected type.
#[derive(Debug, Snafu)]
#[snafu(
    display("expected value with type {expected}, found {actual:?}"),
    context(suffix(Ctx)),
    module
)]
pub struct InvalidTypeError {
    /// The expected type of value.
    pub expected: Expected,
    /// The actual value.
    pub actual: Box<Value>,
    pub backtrace: snafu::Backtrace,
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
#[derive(Debug, Snafu)]
#[snafu(context(suffix(Ctx)), module)]
pub enum IdentifyError {
    /// The URI could not be parsed.
    #[snafu(transparent)]
    InvalidUri {
        #[snafu(backtrace)]
        source: UriError,
    },

    /// The URI is not absolute (i.e. contains a non-empty fragment).
    #[snafu(display("the $id of a schema is not absolute: {uri}"))]
    FragmentedId { uri: Uri, backtrace: Backtrace },

    /// The value of `$id` was not a string
    #[snafu(display(
        "the {keyword} of a schema must be a string in the form of a uri; found {value:?}"
    ))]
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
#[derive(Clone, Debug, Snafu)]
#[snafu(display("dialect not found: {id}"), context(suffix(Ctx)), module)]
pub struct DialectNotFoundError {
    /// The [`AbsoluteUri`] of the [`Dialect`] that was not able
    /// to be found.
    pub id: AbsoluteUri,
    pub backtrace: Backtrace,
}

impl DialectNotFoundError {
    #[must_use]
    /// Create a new [`DialectNotFoundError`].
    pub fn new(id: AbsoluteUri) -> Self {
        Self {
            id,
            backtrace: Backtrace::capture(),
        }
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
#[derive(Debug, Clone, Copy, Snafu)]
#[snafu(
    display("the provided key could not be found"),
    context(suffix(Ctx)),
    module
)]
pub struct UnknownKeyError {
    pub key: Key,
    pub backtrace: Backtrace,
}

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
#[derive(Debug, Clone, Copy, Snafu)]
#[snafu(
    display("The value {value} overflowed {}", Self::MAX),
    context(suffix(Ctx)),
    module
)]
pub struct OverflowError {
    pub value: u64,
    pub backtrace: Backtrace,
}
impl OverflowError {
    pub const MAX: u64 = usize::MAX as u64;
}

impl OverflowError {
    /// The maximum allowed size.
    pub const MAX: u64 = usize::MAX as u64;
}

impl From<u64> for OverflowError {
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
#[derive(Debug, Snafu)]
#[snafu(context(suffix(Ctx)), module)]
pub enum RefError {
    /// The ref value was not a string.
    #[snafu(transparent)]
    UnexpectedType {
        #[snafu(backtrace)]
        source: InvalidTypeError,
    },
    /// The ref value failed to parse as a URI.
    #[snafu(transparent)]
    UriError {
        #[snafu(backtrace)]
        source: UriError,
    },
}
