//! Logical errors that can occur during usage of this crate.
//!
//! Validation errors are defined within their respective keyword's module.

#[doc(no_inline)]
pub use crate::output::ValidationError;
#[doc(no_inline)]
pub use big_rational_str::ParseError as BigRationalParseStrError;
#[doc(no_inline)]
pub use jsonptr::{Error as ResolvePointerError, MalformedPointerError};
#[doc(no_inline)]
pub use url::ParseError as UrlError;

pub use urn::Error as UrnError;

use crate::{dialect::Dialect, schema::Keyword, uri::AbsoluteUri, Output, Uri};
use serde_json::{Number, Value};
use std::{
    collections::HashMap,
    error::Error as StdError,
    fmt::{self, Debug, Display},
    ops::Deref,
    string::FromUtf8Error,
};
use thiserror::Error;

/// An error occurred while attempting to locate subschemas.
#[derive(Debug, Error)]
#[error("error locating subschemas")]
pub enum LocateSchemasError {
    /// An error occurred locating subschemas due to an invalid character in an
    /// anchor.
    Anchor(#[from] AnchorError),
    /// An error occurred locating subschemas due to an error identifying a schema.
    Identify(#[from] IdentifyError),
}

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
    pub keyword: Keyword<'static>,
}

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
    pub keyword: Keyword<'static>,

    /// The value of the anchor.
    pub value: Box<Value>,
}

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
    pub keyword: Keyword<'static>,
    /// The character which caused the error.
    pub character: char,
}

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
    pub keyword: Keyword<'static>,
    /// The character which caused the error.
    pub character: char,
}

/// An issue with an anchor keyword (e.g. `$anchor`, `$dynamicAnchor`,
/// `$recursiveAnchor`) occurred
#[derive(Debug, Error, Clone)]
pub enum AnchorError {
    /// An anchor keyword which does not allow for empty values (e.g. `$anchor`,
    /// `$dynamicAnchor`) was found with an empty string.
    #[error(transparent)]
    Empty(#[from] AnchorEmptyError),

    /// An anchor keyword which does not allow for non-empty values (e.g.
    /// `$recursiveAnchor`) was found with a value.
    #[error(transparent)]
    NotEmpty(#[from] AnchorNotEmptyError),

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

/// An error occurred parsing or resolving a JSON [`Pointer`](jsonptr::Pointer).
#[derive(Debug, Error)]
pub enum PointerError {
    #[error(transparent)]
    /// The JSON [`Pointer`] was malformed.
    Malformed(#[from] MalformedPointerError),

    #[error(transparent)]
    /// The JSON [`Pointer`] could not be resolved.
    Resolution(#[from] ResolvePointerError),
}

/// An error occurred while attempting to add a new a schema source.
#[derive(Debug, Error)]
pub enum SourceError {
    /// An error occurred while attempting to deserialize a source.
    #[error(transparent)]
    DeserializationFailed(#[from] SourceDeserializationError),

    /// A source URI contained a non-empty fragment.
    #[error("source URIs may not contain fragments; found: \"{0}\"")]
    FragmentedUri(AbsoluteUri),

    /// Multiple sources with the same URI were provided.
    #[error(transparent)]
    Duplicate(#[from] SourceDuplicateError),

    /// Resolution of a source failed
    #[error(transparent)]
    ResolutionFailed(#[from] ResolveErrors),

    /// The source was not valid UTF-8.
    #[error(transparent)]
    InvalidUtf8(#[from] FromUtf8Error),

    /// The source's URI was not able to be parsed or contained a fragment
    #[error(transparent)]
    InvalidUri(#[from] UriError),
}

/// Multiple sources with the same URI were provided.
#[derive(Debug, Error)]
#[error("duplicate source provided: {uri}")]
pub struct SourceDuplicateError {
    /// The URI of the duplicate source.
    pub uri: AbsoluteUri,
    /// The value of the duplicate source.
    pub value: Box<Value>,
}

impl From<ResolveError> for SourceError {
    fn from(value: ResolveError) -> Self {
        Self::ResolutionFailed(ResolveErrors {
            errors: vec![value],
        })
    }
}

/// An error occurred while attempting to deserialize a source.
#[derive(Debug, Error)]
#[error("failed to deserialize source \"{uri}\":\n\t{error}")]
pub struct SourceDeserializationError {
    /// The [`AbsoluteUri`] of the source.
    pub uri: AbsoluteUri,

    /// The underlying [`DeserializeError`].
    #[source]
    pub error: DeserializeError,
}

/// Possible errors that may occur while creating a
/// [`Dialects`](crate::dialect::Dialects)
#[derive(Debug, Error)]
pub enum DialectError {
    /// The default [`Dialect`] was not found.
    #[error("default dialect not found: {0}")]
    DefaultNotFound(AbsoluteUri),

    /// Multiple [`Dialect`]s with the same
    /// [`AbsoluteUri`] id were provided.
    #[error("duplicate dialect id provided: {0}")]
    Duplicate(Dialect),

    /// A [`Dialect`] ID contained a non-empty fragment.
    #[error("dialect ids may not contain fragments; found: \"{0}\"")]
    FragmentedId(AbsoluteUri),

    /// The [`Dialect`] did not have the minimum required number of
    /// [`Handler`](`crate::handler::Handler`)s (2).
    #[error("at least one dialect is required to build an Interrogator; none were provided")]
    Empty,
}

/// Various errors that can occur while building an [`Interrogator`](crate::Interrogator).
#[derive(Debug, Error)]
pub enum BuildError {
    #[error(transparent)]
    /// A [`Schema`](crate::schema::Schema) failed to compile.
    Compile(#[from] CompileError),

    #[error(transparent)]
    /// An issue with [`Dialect`]s occurred.
    Dialect(#[from] DialectError),

    #[error(transparent)]
    /// An error occurred while adding, resolving, or deserializing a
    /// [`Source`](crate::source::Source).
    Source(#[from] SourceError),
}

/// An error occurred while parsing a [`Number`] as a [`num::BigRational`].
#[derive(Debug, Error)]
#[error("failed to parse number \"{number}\":\n\t{source}")]
pub struct BigRationalParseError {
    #[source]
    /// The underlying [`ParseRatioError`].
    pub source: BigRationalParseStrError,
    /// The [`Number`] which failed to parse.
    pub number: Number,
}

/// An error occurred while evaluating a [`Value`].
#[derive(Debug, Error)]
pub enum EvaluateError {
    /// Failed to parse a [`Number`] in a [`].
    #[error(transparent)]
    ParseNumber(#[from] BigRationalParseError),

    #[error(transparent)]
    Regex(EvaluateRegexError),

    /// A custom error occurred in a [`Handler`](crate::handler::Handler).
    #[error("{source}")]
    Custom {
        #[source]
        source: Box<dyn StdError>,
        value: Option<Box<Value>>,
    },
}

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

/// Contains one or more errors that occurred during deserialization.
#[derive(Debug, Default)]
pub struct DeserializeError {
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

/// The source of a [`ResolveError`]
#[derive(Debug, Error)]
pub enum ResolveErrorSource {
    /// The [`std::io::Error`] which occurred while resolving a source.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// The [`reqwest::Error`] which occurred while resolving a source.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// The path, as a JSON [`Pointer`], failed to resolve.
    #[error(transparent)]
    Pointer(#[from] PointerError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    /// Any other error which occurred while resolving a source.
    #[error(transparent)]
    Other(#[from] Box<dyn StdError>),
}

impl From<MalformedPointerError> for ResolveErrorSource {
    fn from(err: MalformedPointerError) -> Self {
        Self::Pointer(err.into())
    }
}

impl From<jsonptr::Error> for ResolveErrorSource {
    fn from(err: jsonptr::Error) -> Self {
        Self::Pointer(err.into())
    }
}

/// An error occurred while compiling a schema.
#[derive(Debug, Error)]
pub enum CompileError {
    /// The schema failed evaluation, represented by the failed [`Output`].
    #[error("schema failed evaluation:\n{0}")]
    SchemaInvalid(Output<'static>),

    /// Failed to identify a schema
    #[error(transparent)]
    SchemaIdentification(#[from] IdentifyError),

    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    #[error(transparent)]
    DialectNotKnown(#[from] DialectUnknownError),

    /// Failed to parse a [`Uri`](crate::uri::Uri) or
    /// [`AbsoluteUri`](`crate::uri::AbsoluteUri`)
    #[error(transparent)]
    Uri(#[from] UriError),

    /// All attached implementations of [`Resolve`](crate::resolve::Resolve)
    /// failed to resolve a source.
    #[error(transparent)]
    Resolve(#[from] ResolveErrors),

    /// Failed to deserialize and  a resolved
    #[error(transparent)]
    Source(#[from] SourceError),

    /// Failed to locate subschemas within a schema.
    #[error(transparent)]
    LocateSchemas(#[from] LocateSchemasError),

    /// Custom errors returned by a [`Handler`]
    #[error(transparent)]
    Custom(#[from] Box<dyn StdError + Send + Sync>),
}

/// A source or schema could not be found.
#[derive(Debug, Clone, Error)]
#[error("unable to resolve \"{0}\" due to not being found")]
pub struct NotFoundError(pub AbsoluteUri);

/// The schema's [`Dialect`] is not registered with the
/// [`Interrogator`](crate::Interrogator).
#[derive(Debug, Clone, Error)]
#[error("metaschema dialect not found: {}", .metaschema_id)]
pub struct DialectUnknownError {
    pub metaschema_id: String,
}

/// Errors which can occur when parsing or interacting with
/// [`Uri`](`crate::uri::Uri`), [`AbsoluteUri`](`crate::uri::AbsoluteUri`), or
/// [`RelativeUri`](`crate::uri::RelativeUri`).
#[derive(Debug, Clone, Error)]
pub enum UriError {
    /// an issue occurred parsing a [`Url`](`url::Url`)
    #[error(transparent)]
    Url(#[from] UrlError),

    /// an issue occurred parsing a [`Urn`](`urn::Urn`)
    #[error(transparent)]
    Urn(#[from] UrnError),

    /// an issue occurred parsing a [`RelativeUri`](`crate::uri::RelativeUri`)
    #[error(transparent)]
    Relative(#[from] RelativeUriError),

    /// Indicates that the [`AbsoluteUri`](crate::uri::AbsoluteUri) is not
    /// absolute. This is not applicable to [`Uri`](crate::uri::Uri) as they can
    /// be relative.
    #[error("uri is not absolute: {0}")]
    NotAbsolute(Uri),

    /// An issue occurred while setting the Authority of a
    /// [`Uri`](crate::uri::Uri) or [`RelativeUri`](crate::uri::RelativeUri).
    #[error(transparent)]
    Authority(#[from] AuthorityError),

    /// The scheme of a [`Uri`](crate::uri::Uri) or
    /// [`AbsoluteUri`](crate::uri::AbsoluteUri) is malformed.
    #[error("invalid scheme: {0}")]
    InvalidScheme(String),
}

impl From<InvalidPortError> for UriError {
    fn from(err: InvalidPortError) -> Self {
        Self::Relative(err.into())
    }
}
impl From<OverflowError> for UriError {
    fn from(err: OverflowError) -> Self {
        Self::Relative(err.into())
    }
}

impl UriError {
    /// Returns `true` if the uri parse error is [`Url`].
    ///
    /// [`Url`]: UriParseError::Url
    #[must_use]
    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url { .. })
    }

    /// Returns `true` if the uri parse error is [`Urn`].
    ///
    /// [`Urn`]: UriParseError::Urn
    #[must_use]
    pub fn is_urn(&self) -> bool {
        matches!(self, Self::Urn { .. })
    }

    /// Returns `true` if the uri error is [`Relative`].
    ///
    /// [`Relative`]: UriError::Relative
    #[must_use]
    pub fn is_relative(&self) -> bool {
        matches!(self, Self::Relative(..))
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
        if let Self::Url(err) = self {
            Some(err)
        } else {
            None
        }
    }

    /// If the error is [`UriError::Urn`], returns a reference to the underlying
    /// [`UrnError`].
    #[must_use]
    pub fn as_urn(&self) -> Option<&urn::Error> {
        if let Self::Urn(err) = self {
            Some(err)
        } else {
            None
        }
    }

    #[must_use]
    /// If the error is [`UriError::Relative`], returns a reference to the underlying
    /// [`RelativeUriError`].
    pub fn as_relative(&self) -> Option<&RelativeUriError> {
        if let Self::Relative(v) = self {
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

/// Returned from `set_authority` on [`Uri`](crate::uri::Uri), [`AbsoluteUri`](crate::uri::AbsoluteUri), and [`RelativeUri`](crate::uri::RelativeUri)
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
    Overflow(#[from] OverflowError),

    /// The decoded string is not valid UTF-8
    #[error(transparent)]
    Utf8Encoding(#[from] std::str::Utf8Error),

    /// The port of a [`RelativeUri`] exceeded the maximum value of 65535.
    #[error(transparent)]
    InvalidPort(#[from] InvalidPortError),
}

/// An error occurred while attempting to identify a schema
#[derive(Debug, Error)]
pub enum IdentifyError {
    /// The URI could not be parsed.
    #[error(transparent)]
    FailedToParseUri(#[from] UriError),

    /// The URI is not absolute (i.e. contains a non-empty fragment).
    #[error("the $id of a schema is not absolute: {0}")]
    FragmentedId(Uri),

    /// Any custom error which a [`Handler`](crate::handler::Handler) may need
    /// to return.
    #[error(transparent)]
    Custom(#[from] Box<dyn StdError>),
}

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

/// A schema [`Key`](crate::Interrogator::Key) was not found.
///
/// If this is encountered, odds are it is because you have two
/// [`Interrogator`](crate::Interrogator)s and mismatched keys. Consider using a
/// unique key type per [`Interrogator`](crate::Interrogator). See the macro
/// [`new_key_type`](crate::new_key_type), re-exported from slotmap.
///
/// If this is not the case, there may be a bug. Please create an issue at:
/// <https://github.com/chanced/grill/issues/new>
#[derive(Debug, Clone, Copy, Error)]
#[error("the provided key could not be found; if using multiple Interrogators, consider using a unique key type per")]
pub struct UnknownKeyError;

/// A slice or string overflowed an allowed length maximum of `M`.
#[derive(Debug, Clone, Copy, Error)]
#[error("the length of a string or slice overflows the maximum of {M}, received {0}")]
pub struct OverflowError<const M: usize = { u32::MAX as usize }, V = usize>(pub V);
impl<const M: usize, V> OverflowError<M, V> {
    /// The maximum allowed size.
    pub const MAX: usize = M;
}
