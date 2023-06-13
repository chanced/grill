//! Logical errors that can occur during usage of this crate.
//!
//! Validation errors are defined within their respective keyword's module.

use crate::{uri::AbsoluteUri, Location, Output, Uri};
use jsonptr::Pointer;
use serde_json::{Number, Value};
use snafu::Snafu;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    string::FromUtf8Error,
};

pub use crate::output::ValidationError;
pub use urn::Error as UrnError;

#[derive(Debug)]
pub struct DuplicateSourceError {
    pub uri: AbsoluteUri,
    pub source: Value,
}

#[derive(Debug)]
pub struct FragmentedSourceUriError {
    pub uri: AbsoluteUri,
}
impl FragmentedSourceUriError {
    #[must_use]
    pub fn new(uri: AbsoluteUri) -> Self {
        Self { uri }
    }
}

impl Display for FragmentedSourceUriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "source uris may not contain fragments; found: \"{}\"",
            self.uri
        )
    }
}
impl Error for FragmentedSourceUriError {}

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
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum BuildError {
    #[snafu(display("failed to compile schema: {}", source), context(false))]
    Compile { source: CompileError },
    #[snafu(display("duplicate dialect id: {}", source), context(false))]
    DuplicateSource { source: DuplicateSourceError },
    #[snafu(display("{}", source), context(false))]
    FragmentedSourceUri { source: FragmentedSourceUriError },
    #[snafu(display("{}", source), context(false))]
    FragmentedDialectId { source: FragmentedDialectIdError },
    #[snafu(display("failed to parse uri: {}", source), context(false))]
    MalformedAbsoluteUri { source: AbsoluteUriParseError },
    #[snafu(display("failed to deserialize source: {}", uri))]
    DeserializeSource {
        source: DeserializeError,
        uri: AbsoluteUri,
    },
}

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
        referering_location: Location,
    },
    /// An [`std::io::Error`] occurred while reading the schema
    #[snafu(display(r#"error reading schema "{schema_id}": {source}"#))]
    Io {
        /// The URI of the schema which was not able to be resolved
        schema_id: String,
        /// The [`std::io::Error`] which occurred
        source: std::io::Error,
    },
    /// A [`jsonptr::Error`] occurred while attempting to resolve a schema
    #[snafu(display(r#"error resolving pointer "{pointer}" in schema "{schema_id}": {source}"#))]
    Pointer {
        /// The URI of the schema which was not able to be resolved
        schema_id: String,
        /// The JSON Pointer which was not able to be resolved
        pointer: jsonptr::Pointer,
        /// The [`jsonptr::Error`] which occurred
        source: jsonptr::Error,
    },

    /// The schema was not able to be parsed with the enabled formats
    #[snafu(display(r#"error deserialzing schema "{schema_id}": {source}"#))]
    Deserialize {
        /// The URI of the schema which was not able to be resolved
        schema_id: String,
        /// The [`DeserializeError`] which occurred
        source: DeserializeError,
    },

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

impl ResolveError {
    #[must_use]
    pub fn not_found(schema_id: String, referering_location: Location) -> Self {
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
    EvaluationFailed {
        /// The [`EvaluateError`] which occurred
        source: Output<'static>,
    },
    /// An error occurred during evaluation of the schema
    FailedToEvaluate { source: Output<'static> },
    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    UnknownMetaschema {
        /// The [`Uri`] of meta schema which encountered an error
        schema_id: String,
        /// The error which occurred when parsing the meta schema
        source: UnknownMetaschemaError,
    },
    /// A schema was not able to be resolved.
    SchemaNotFound {
        /// The source [`ResolveError`]
        source: ResolveError,
    },
    Internal {
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

#[derive(Debug, Clone, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum UnknownMetaschemaError {
    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    #[snafu(display("{}", source), context(false))]
    Unknown { source: UnkownMetaSchemaError },
}

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
pub enum SourceError {
    #[snafu(display("{}", source), context(false))]
    InvalidUtf8 { source: FromUtf8Error },
    #[snafu(display("{}", source), context(false))]
    ParseAbsoluteUri { source: AbsoluteUriParseError },
    #[snafu(display("{}", source), context(false))]
    Deserialize { source: DeserializeError },
    #[snafu(display("{}", source), context(false))]
    DuplicateSource { source: DuplicateSourceError },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum SourceSliceError {
    #[snafu(display("{}", source), context(false))]
    InvalidUtf8 { source: FromUtf8Error },
    #[snafu(display("{}", source), context(false))]
    ParseAbsoluteUri { source: AbsoluteUriParseError },
}

#[derive(Debug, Clone, Snafu, PartialEq, Eq)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum AbsoluteUriParseError {
    #[snafu(display("{}", source), context(false))]
    Url { source: url::ParseError },
    #[snafu(display("{}", source), context(false))]
    Urn { source: urn::Error },
    #[snafu(display("{}", source))]
    NotAbsolute { source: UriNotAbsoluteError },
}

#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum UriParseError {
    #[snafu(display("{}", source), context(false))]
    Url { source: url::ParseError },
    #[snafu(display("{}", source), context(false))]
    Urn { source: urn::Error },
}

impl AbsoluteUriParseError {
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
    pub fn as_url(&self) -> Option<&url::ParseError> {
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
    Parse { source: UriParseError },
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

#[cfg(test)]
mod tests {
    use super::*;
}
