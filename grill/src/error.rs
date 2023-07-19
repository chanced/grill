//! Logical errors that can occur during usage of this crate.
//!
//! Validation errors are defined within their respective keyword's module.

use crate::{dialect::Dialect, keyword::Keyword, uri::AbsoluteUri, Location, Output, Uri};
use jsonptr::{MalformedPointerError, Pointer};
use serde_json::{Number, Value};
use snafu::Snafu;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Debug, Display},
    string::FromUtf8Error,
};

pub use crate::output::ValidationError;

pub type UrnError = urn::Error;
pub type UrlError = url::ParseError;

#[derive(Debug)]
pub struct DuplicateSourceError {
    pub uri: AbsoluteUri,
    pub source: Value,
}

#[derive(Debug)]
pub struct DuplicateDialectError {
    pub dialect: Dialect,
}

impl DuplicateDialectError {
    #[must_use]
    pub fn new(dialect: Dialect) -> Self {
        Self { dialect }
    }
}

impl Display for DuplicateDialectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "duplicate dialect provided: {}", self.dialect.id)
    }
}

impl std::error::Error for DuplicateDialectError {}

#[derive(Debug, Snafu)]
pub enum LocateSchemasError {
    #[snafu(display("{}", source), context(false))]
    Anchor { source: AnchorError },
    #[snafu(display("{}", source), context(false))]
    Urn { source: UrnError },
}

/// The inner error of a [`MalformedAnchorError`].
#[derive(Debug, Clone)]
pub struct AnchorError {
    pub location: AbsoluteUri,
    pub anchor: String,
    pub keyword: Keyword<'static>,
}
impl Display for AnchorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "anchor \"{}\" at \"{}\" is malformed",
            self.anchor, self.location
        )
    }
}
impl std::error::Error for AnchorError {}

#[derive(Debug, Snafu, Clone)]
/// An `$anchor` or `$dynamicAnchor` is malformed
pub enum MalformedAnchorError {
    /// an `$anchor` or `$dynamicAnchor` must not be empty
    #[snafu(display("{} must not be empty", source.keyword), context(false))]
    Empty { source: AnchorError },
    /// `$anchor` and `$dynamicAnchor` must start with either a letter
    /// (`([A-Za-z])`) or an underscore (`_`).
    #[snafu(display("{} must start with either a letter (([A-Za-z])) or an underscore (_); found {} for {} at {}", source.keyword, character, source.anchor, source.location))]
    InvalidLeadingCharacter {
        source: AnchorError,
        character: char,
    },
    /// `$anchor` and `$dynamicAnchor` may only contain letters (`([A-Za-z])`),
    /// digits (`[0-9]`), hyphens (`'-'`), underscores (`'_'`), and periods
    /// (`'.'`).
    #[snafu(display("{} may only contain letters([A-Za-z]) digits ([0-9]), hyphens ('-'), underscores ('_') and periods ('.'); found {} for {} at {}", source.keyword, character, source.anchor, source.location))]
    InvalidCharacter {
        source: AnchorError,
        character: char,
    },
}

#[derive(Debug)]
pub struct FragmentedUriError {
    pub uri: AbsoluteUri,
}
impl FragmentedUriError {
    #[must_use]
    pub fn new(uri: AbsoluteUri) -> Self {
        Self { uri }
    }
}

impl Display for FragmentedUriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "source uris may not contain fragments; found: \"{}\"",
            self.uri
        )
    }
}
impl Error for FragmentedUriError {}

impl Display for DuplicateSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "duplicate source: {}", self.uri)
    }
}

impl Error for DuplicateSourceError {}

#[derive(Debug)]
pub struct FragmentedDialectIdError {
    pub id: AbsoluteUri,
}
impl Display for FragmentedDialectIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dialect ids may not contain fragments; found: \"{}\"",
            self.id
        )
    }
}
impl FragmentedDialectIdError {
    #[must_use]
    pub fn new(id: AbsoluteUri) -> Self {
        Self { id }
    }
}

impl Error for FragmentedDialectIdError {}

#[derive(Debug, Snafu)]
pub enum PointerError {
    #[snafu(display("{}", source), context(false))]
    Malformed { source: MalformedPointerError },
    #[snafu(display("{}", source), context(false))]
    Resolve { source: jsonptr::Error },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum NewSourcesError {
    #[snafu(display("failed to deserialize source: {}", uri))]
    Deserialize {
        source: DeserializeError,
        uri: AbsoluteUri,
    },
    #[snafu(display("source URIs may not contain fragments; found: \"{}\"", source.uri), context(false))]
    FragmentedUri { source: FragmentedUriError },
}

impl From<NewSourcesError> for BuildError {
    fn from(err: NewSourcesError) -> Self {
        match err {
            NewSourcesError::Deserialize { source, uri } => {
                BuildError::DeserializeSource { source, uri }
            }
            NewSourcesError::FragmentedUri { source } => BuildError::FragmentedSourceUri { source },
        }
    }
}

#[derive(Clone, Debug)]
pub struct DefaultDialectNotFoundError {
    pub uri: AbsoluteUri,
}

impl Display for DefaultDialectNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "default dialect not found: {}", self.uri)
    }
}
impl Error for DefaultDialectNotFoundError {}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum NewDialectsError {
    #[snafu(display("{}", source), context(false))]
    DefaultNotFound { source: DefaultDialectNotFoundError },
    #[snafu(display("{}", source), context(false))]
    DuplicateDialect { source: DuplicateDialectError },
    #[snafu(display("{}", source), context(false))]
    FragmentedDialectId { source: FragmentedDialectIdError },
    #[snafu(display("{}", source), context(false))]
    Empty { source: EmptyDialectsError },
}

impl From<NewDialectsError> for BuildError {
    fn from(value: NewDialectsError) -> Self {
        match value {
            NewDialectsError::DefaultNotFound { source } => {
                BuildError::DefaultDialectNotFound { source }
            }
            NewDialectsError::DuplicateDialect { source } => {
                BuildError::DuplicateDialect { source }
            }
            NewDialectsError::FragmentedDialectId { source } => {
                BuildError::FragmentedDialectId { source }
            }
            NewDialectsError::Empty { source } => BuildError::EmptyDialects { source },
        }
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum BuildError {
    #[snafu(display("failed to compile schema: {}", source), context(false))]
    Compile { source: CompileError },
    #[snafu(display("{}", source), context(false))]
    DuplicateSource { source: DuplicateSourceError },
    #[snafu(display("{}", source), context(false))]
    DuplicateDialect { source: DuplicateDialectError },
    #[snafu(display("source URIs may not contain fragments; found: \"{}\"", source.uri))]
    FragmentedSourceUri { source: FragmentedUriError },
    #[snafu(display("{}", source), context(false))]
    FragmentedDialectId { source: FragmentedDialectIdError },
    #[snafu(display("failed to parse uri: {}", source), context(false))]
    MalformedAbsoluteUri { source: UriError },
    #[snafu(display("failed to deserialize source: {}", uri))]
    DeserializeSource {
        source: DeserializeError,
        uri: AbsoluteUri,
    },
    #[snafu(display("{}", source), context(false))]
    EmptyDialects { source: EmptyDialectsError },
    #[snafu(display("{}", source), context(false))]
    DefaultDialectNotFound { source: DefaultDialectNotFoundError },
}

#[derive(Debug, Clone, Copy)]
pub struct EmptyDialectsError;

impl Display for EmptyDialectsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "at least one dialect is required to build an Interrogator; none were provided"
        )
    }
}
impl std::error::Error for EmptyDialectsError {}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum StoreError {
    #[snafu(display("failed to resolve schema: {uri}\ncaused by:\n\t{source}"))]
    Resolve { uri: String, source: ResolveError },
    #[snafu(display("failed to compile schema: {uri}\ncaused by:\n\t{source}"))]
    Compile { uri: String, source: CompileError },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum EvaluateError<'v> {
    #[snafu(display("error parsing number: {}", source))]
    ParseNumber {
        source: big_rational_str::ParseError,
        number: &'v Number,
    },

    #[snafu(display("custom error: {}", source))]
    Custom {
        source: Box<dyn Error>,
        value: &'v Value,
    },

    #[snafu(display("error evaluating regular expression: {}", source))]
    Regex {
        regex: String,
        value: &'v Value,
        source: fancy_regex::Error,
    },
}

/// Contains one or more errors that occurred during deserialization.
#[derive(Debug)]
pub struct DeserializeError {
    pub formats: HashMap<&'static str, erased_serde::Error>,
}

impl DeserializeError {
    #[must_use]
    pub fn new() -> Self {
        Self {
            formats: HashMap::new(),
        }
    }
    pub fn add(&mut self, format: &'static str, err: erased_serde::Error) {
        self.formats.insert(format, err);
    }
}

impl Default for DeserializeError {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to deserialize")?;
        for (format, err) in &self.formats {
            write!(f, "\t{format}: {err}")?;
        }
        Ok(())
    }
}

impl Error for DeserializeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.formats.iter().next().map(|(_, err)| err as _)
    }
}

#[derive(Debug)]
pub struct ResolveErrors {
    pub errors: Vec<ResolveError>,
}
impl ResolveErrors {
    #[must_use]
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
    pub fn push(&mut self, err: ResolveError) {
        self.errors.push(err);
    }
}

impl Default for ResolveErrors {
    fn default() -> Self {
        Self::new()
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
impl Error for ResolveErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.errors.first().map(|err| err as _)
    }
}

/// Errors which can occur during schema resolution.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum ResolveError {
    /// The schema was not found
    #[snafu(display(r#"schema "{schema_id}" not found"#))]
    NotFound {
        /// The URI of the schema which was not found
        schema_id: String,
        /// The Location of the referring keyword (e.g. `"$ref"`, `"$recursiveRef"`, `"$dynamicRef"` etc.)
        referering_location: Option<Location>,
    },
    /// An [`std::io::Error`] occurred while reading the schema
    #[snafu(display(r#"error reading schema "{schema_id}": {source}"#))]
    Io {
        /// The URI of the schema which was not able to be resolved
        schema_id: String,
        /// The [`std::io::Error`] which occurred
        source: std::io::Error,
    },

    /// Indicates that a source was able to be resolved but the schema referenced by
    /// a fragment was not able to be located.
    #[snafu(display("{}", source))]
    NestedSchemaNotFound {
        source: PointerError,
        /// The URI fragment of the schema which was not able to be resolved
        uri_fragment: String,
        /// The URI of the schema which was not able to be resolved
        uri: AbsoluteUri,
    },

    /// The schema was not able to be deserialized with the attached
    /// implementations of [`Deserializer`](`crate::Deserializer`)
    #[snafu(display(r#"error deserialzing schema "{schema_id}": {source}"#))]
    Deserialize {
        /// The URI of the schema which was not able to be resolved
        schema_id: AbsoluteUri,
        /// The [`DeserializeError`] which occurred
        source: DeserializeError,
    },
    /// An error occurred while adding the value as a source
    #[snafu(display("{}", source), context(false))]
    Source { source: SourceError },

    /// A [`Resolve`] implementation returned a custom error
    #[snafu(display(r#"error resolving "{schema_id}": {source}"#))]
    Custom {
        /// The URI of the schema which was not able to be resolved
        schema_id: String,
        /// The custom error which occurred
        source: Box<dyn Error>,
    },
    #[cfg(feature = "reqwest")]
    #[snafu(display(r#"error fetching "{schema_id}": {source}"#))]
    Reqwest {
        schema_id: String,
        source: reqwest::Error,
    },
}

impl From<ResolveError> for ResolveErrors {
    fn from(err: ResolveError) -> Self {
        Self { errors: vec![err] }
    }
}
impl From<SourceError> for ResolveErrors {
    fn from(err: SourceError) -> Self {
        Self {
            errors: vec![err.into()],
        }
    }
}

impl ResolveError {
    #[must_use]
    pub fn not_found(schema_id: String, referering_location: Option<Location>) -> Self {
        Self::NotFound {
            schema_id,
            referering_location,
        }
    }

    pub fn custom(schema_id: String, source: impl 'static + Error + Send + Sync) -> Self {
        Self::Custom {
            schema_id,
            source: Box::new(source),
        }
    }

    /// Returns `true` if the resolve error is [`NotFound`].
    ///
    /// [`NotFound`]: ResolveError::NotFound
    #[must_use]
    pub fn is_schema_not_found(&self) -> bool {
        matches!(self, Self::NotFound { .. })
    }

    /// Returns `true` if the resolve error is [`Deserialize`].
    ///
    /// [`Deserialize`]: ResolveError::Deserialize
    #[must_use]
    pub fn is_deserialize(&self) -> bool {
        matches!(self, Self::Deserialize { .. })
    }

    /// Returns `true` if the resolve error is [`Custom`].
    ///
    /// [`Custom`]: ResolveError::Custom
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom { .. })
    }
}

// intentionally not worrying about the fact that this is missing

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum CompileError {
    /// The schema failed evaluation
    #[snafu(display("schema failed evaluation: {}", source))]
    InvalidSchema {
        /// The [`EvaluateError`] which occurred
        source: Output<'static>,
    },

    /// Failed to identify a schema
    #[snafu(display("{}", source), context(false))]
    FailedToIdentifySchema {
        /// The [`IdentifyError`] which occurred
        source: IdentifyError,
    },

    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    #[snafu(display("{}", source), context(false))]
    DialectNotKnown { source: UnknownDialectError },

    /// A schema was not able to be resolved.
    #[snafu(display("failed to resolve schema: {}", source,), context(false))]
    FailedToResolveSchema {
        /// The source [`ResolveError`]
        source: ResolveErrors,
    },

    /// Failed to parse an [`AbsoluteUri`](`crate::uri::AbsoluteUri`)
    #[snafu(display("failed to parse absolute URI: {}", source))]
    ParseAbsoluteUri { value: String, source: UriError },

    #[snafu(display("{}", source), context(false))]
    FailedToParseAbsoluteUri { source: UriError },

    /// Custom errors returned by a custom [`Handler`]
    Custom {
        source: Box<dyn Error + Send + Sync>,
    },
}

#[derive(Debug, Clone)]
pub struct SchemaNotFoundError {
    pub schema_id: String,
    pub path: Pointer,
}

impl std::fmt::Display for SchemaNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "schema \"{}\" not found", self.schema_id)
    }
}
impl Error for SchemaNotFoundError {}

#[derive(Debug, Clone)]
pub struct UnknownDialectError {
    pub metaschema_id: String,
}
impl Display for UnknownDialectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "metaschema dialect not found: {}", self.metaschema_id)
    }
}
impl std::error::Error for UnknownDialectError {}

#[derive(Debug, Clone)]
pub struct UnkownMetaSchemaError {
    pub schema_id: String,
}
impl Display for UnkownMetaSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown meta schema: {}", self.schema_id)
    }
}
impl Error for UnkownMetaSchemaError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UriNotAbsoluteError {
    pub uri: String,
}
impl Display for UriNotAbsoluteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "uri is not absolute: \"{}\"", self.uri)
    }
}

impl Error for UriNotAbsoluteError {}
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum SourceError {
    #[snafu(display("{}", source), context(false))]
    InvalidUtf8 { source: FromUtf8Error },
    #[snafu(display("{}", source), context(false))]
    ParseAbsoluteUri { source: UriError },
    #[snafu(display("{}", source), context(false))]
    Deserialize { source: DeserializeError },
    #[snafu(display("{}", source), context(false))]
    DuplicateSource { source: DuplicateSourceError },
    #[snafu(display("{}", source), context(false))]
    FragmentedSourceUri { source: FragmentedUriError },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum SourceSliceError {
    #[snafu(display("{}", source), context(false))]
    InvalidUtf8 { source: FromUtf8Error },
    #[snafu(display("{}", source), context(false))]
    ParseAbsoluteUri { source: UriError },
}

#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum UriError {
    #[snafu(display("{}", source), context(false))]
    Url { source: UrlError },
    #[snafu(display("{}", source), context(false))]
    Urn { source: urn::Error },
    #[snafu(display("{}", source))]
    NotAbsolute { source: UriNotAbsoluteError },
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
    #[must_use]
    pub fn as_url(&self) -> Option<&UrlError> {
        if let Self::Url { source } = self {
            Some(source)
        } else {
            None
        }
    }
    #[must_use]
    pub fn as_urn(&self) -> Option<&urn::Error> {
        if let Self::Urn { source } = self {
            Some(source)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum IdentifyError {
    #[snafu(display("{}", source), context(false))]
    Parse { source: UriError },
    #[snafu(display("{}", source), context(false))]
    HasFragment { source: HasFragmentError<Uri> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HasFragmentError<U>
where
    U: PartialEq + Eq,
{
    pub uri: U,
}
impl<U> Display for HasFragmentError<U>
where
    U: Display + PartialEq + Eq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"URI "{}" contains a fragment"#, self.uri)
    }
}
impl<U> Error for HasFragmentError<U> where U: std::fmt::Debug + Display + PartialEq + Eq {}

#[derive(Clone, Debug)]
pub struct DialectNotFoundError {
    pub dialect_id: AbsoluteUri,
}
impl Display for DialectNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dialect not found: {}", self.dialect_id)
    }
}
impl std::error::Error for DialectNotFoundError {}

impl DialectNotFoundError {
    #[must_use]
    pub fn new(dialect_id: AbsoluteUri) -> Self {
        Self { dialect_id }
    }
}
