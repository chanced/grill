use crate::schema::Object;
use crate::{schema::Types, Location};
use jsonptr::Pointer;
use serde_json::Value;
use snafu::Snafu;
use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display},
};

pub use crate::output::ValidationError;

#[derive(Debug, Snafu)]
pub enum StoreError {
    #[snafu(display("failed to resolve schema: {uri}\ncaused by:\n\t{source}"))]
    Resolve { uri: String, source: ResolveError },
    #[snafu(display("failed to compile schema: {uri}\ncaused by:\n\t{source}"))]
    Compile { uri: String, source: CompileError },
}

// TODO: Finish EvaluateError

#[derive(Debug)]
pub enum EvaluateError {
    Any,
}
impl Display for EvaluateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluateError::Any => write!(f, "Any"),
        }
    }
}
impl Error for EvaluateError {}

#[derive(Debug)]
pub enum DialectError {
    MissingSchemaId {
        schema: Object,
    },
    MissingRequiredVocabulary {
        vocabulary_id: String,
        meta_schema_id: String,
    },
    SchemaIdNotAbsolute {
        id: String,
    },
}

impl Display for DialectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DialectError::*;
        match self {
            MissingSchemaId { .. } => {
                write!(f, "Schema is not identified")
            }
            MissingRequiredVocabulary { vocabulary_id, .. } => {
                write!(f, "Vocabulary \"{vocabulary_id}\" is required")
            }
            SchemaIdNotAbsolute { id } => {
                write!(f, "Schema ID is not absolute: \"{id}\"")
            }
        }
    }
}
impl std::error::Error for DialectError {}

/// Contains one or more errors that occurred during deserialization.
#[derive(Debug)]
pub struct DeserializeError {
    #[cfg(all(not(feature = "yaml"), not(feature = "toml")))]
    /// JSON deserialization error
    pub json: serde_json::Error,

    /// JSON deserialization error
    #[cfg(any(feature = "yaml", feature = "toml"))]
    pub json: Option<serde_json::Error>,

    /// YAML deserialization error
    #[cfg(feature = "yaml")]
    #[cfg_attr(docsrs, doc(cfg(feature = "yaml")))]
    pub yaml: Option<yaml::Error>,

    /// TOML deserialization error
    #[cfg_attr(docsrs, doc(cfg(feature = "yaml")))]
    #[cfg(feature = "toml")]
    pub toml: Option<toml::de::Error>,
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(all(not(feature = "yaml"), not(feature = "toml")))]
        {
            write!(f, "{}", self.json)
        }
        #[cfg(any(feature = "yaml", feature = "toml"))]
        {
            if let Some(err) = &self.json {
                write!(f, "json: {err}")?;
            }
            #[cfg(feature = "yaml")]
            if let Some(err) = &self.yaml {
                write!(f, "yaml: {err}")?;
            }
            #[cfg(feature = "toml")]
            if let Some(err) = &self.toml {
                write!(f, "toml: {err}")?;
            }
            Ok(())
        }
    }
}

impl Error for DeserializeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        #[cfg(all(not(feature = "yaml"), not(feature = "toml")))]
        {
            Some(&self.json)
        }
        #[cfg(any(feature = "yaml", feature = "toml"))]
        {
            if let Some(err) = &self.json {
                return Some(err);
            }
            #[cfg(feature = "yaml")]
            if let Some(err) = &self.yaml {
                return Some(err);
            }
            #[cfg(feature = "toml")]
            if let Some(err) = &self.toml {
                return Some(err);
            }
            None
        }
    }
}

impl From<serde_json::Error> for DeserializeError {
    fn from(err: serde_json::Error) -> Self {
        #[cfg(all(not(feature = "yaml"), not(feature = "toml")))]
        {
            Self { json: err }
        }
        #[cfg(any(feature = "yaml", feature = "toml"))]
        {
            Self {
                json: Some(err),
                #[cfg(feature = "yaml")]
                yaml: None,
                #[cfg(feature = "toml")]
                toml: None,
            }
        }
    }
}

#[cfg(feature = "yaml")]
impl From<yaml::Error> for DeserializeError {
    fn from(value: yaml::Error) -> Self {
        Self {
            yaml: Some(value),
            json: None,
            #[cfg(feature = "toml")]
            toml: None,
        }
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for DeserializeError {
    fn from(value: toml::de::Error) -> Self {
        Self {
            toml: Some(value),
            json: None,
            #[cfg(feature = "yaml")]
            yaml: None,
        }
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
    /// The schema was not able to be parsed with the enabled formats
    #[snafu(display(r#"error deserialzing schema "{schema_id}": {source}"#))]
    Deserialize {
        /// The URI of the schema which was not able to be resolved
        schema_id: String,
        /// The [`DeserializeError`] which occurred
        source: DeserializeError,
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
    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    MetaSchema {
        /// The [`Uri`] of meta schema which encountered an error
        schema_id: String,
        /// The error which occurred when parsing the meta schema
        source: MetaSchemaError,
    },
    /// A schema was not able to be resolved.
    SchemaNotFound {
        /// The source [`ResolveError`]
        source: ResolveError,
    },
    UnexpectedValue {
        source: UnexpectedValueError,
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

#[derive(Debug, Clone)]
pub struct UnexpectedValueError {
    pub schema_id: Option<String>,
    pub pointer: Pointer,
    pub property: Cow<'static, str>,
    pub expected_types: Types,
    pub found: Box<Value>,
    pub msg: String,
}

impl fmt::Display for UnexpectedValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"expected {} for "{}", found {}"#,
            self.expected_types,
            self.property,
            Types::of_value(&self.found)
        )
    }
}
impl Error for UnexpectedValueError {}

#[derive(Debug, Clone, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum MetaSchemaError {
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

#[derive(Debug, Clone)]
pub struct UriNotAbsoluteError {
    pub uri: String,
}
impl Display for UriNotAbsoluteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "uri is not absolute: \"{}\"", self.uri)
    }
}

impl Error for UriNotAbsoluteError {}

#[derive(Debug, Clone, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum AbsoluteUriParseError {
    #[snafu(display("{}", source), context(false))]
    Url { source: url::ParseError },
    #[snafu(display("{}", source), context(false))]
    Urn { source: urn::Error },
    #[snafu(display("{}", source))]
    NotAbsolute { source: UriNotAbsoluteError },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum UriParseError {
    #[snafu(display("{}", source), context(false))]
    Url { source: url::ParseError },
    #[snafu(display("{}", source), context(false))]
    Urn { source: urn::Error },
    #[snafu(
        display(
            "failed to parse uri due to a regular expression error: \n\t{}",
            source
        ),
        context(false)
    )]
    Regex { source: fancy_regex::Error },
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

    pub fn as_url(&self) -> Option<&url::ParseError> {
        if let Self::Url { source } = self {
            Some(source)
        } else {
            None
        }
    }
    pub fn as_urn(&self) -> Option<&urn::Error> {
        if let Self::Urn { source } = self {
            Some(source)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
