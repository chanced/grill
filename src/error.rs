use crate::{value_type_name, Location, schema::Types, Uri};
use jsonptr::Pointer;
use serde_json::Value;
use snafu::Snafu;
use std::error::Error as StdError;
use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display},
};

pub use crate::output::ValidationError;

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
        schema_id: Uri,
        /// The Location of the referring keyword (e.g. `"$ref"`, `"$recursiveRef"`, `"$dynamicRef"` etc.)
        referering_location: Location,
    },
    /// An [`std::io::Error`] occurred while reading the schema
    #[snafu(display(r#"error reading schema "{schema_id}": {source}"#))]
    Io {
        /// The URI of the schema which was not able to be resolved
        schema_id: Uri,
        /// The [`std::io::Error`] which occurred
        source: std::io::Error,
    },
    /// The schema was not able to be parsed with the enabled formats
    #[snafu(display(r#"error deserialzing schema "{schema_id}": {source}"#))]
    Deserialize {
        /// The URI of the schema which was not able to be resolved
        schema_id: Uri,
        /// The [`DeserializeError`] which occurred
        source: DeserializeError,
    },

    /// A [`jsonptr::Error`] occurred while attempting to resolve a schema
    #[snafu(display(r#"error resolving pointer "{pointer}" in schema "{schema_id}": {source}"#))]
    Pointer {
        /// The URI of the schema which was not able to be resolved
        schema_id: Uri,
        /// The JSON Pointer which was not able to be resolved
        pointer: jsonptr::Pointer,
        /// The [`jsonptr::Error`] which occurred
        source: jsonptr::Error,
    },

    /// A [`Resolve`] implementation returned a custom error
    #[snafu(display(r#"error resolving "{schema_id}": {source}"#))]
    Custom {
        /// The URI of the schema which was not able to be resolved
        schema_id: Uri,
        /// The custom error which occurred
        source: Box<dyn StdError>,
    },
    #[cfg(feature = "reqwest")]
    #[snafu(display(r#"error fetching "{schema_id}": {source}"#))]
    Reqwest {
        schema_id: Uri,
        source: reqwest::Error,
    },
}

impl ResolveError {
    #[must_use]
    pub fn not_found(schema_id: Uri, referering_location: Location) -> Self {
        Self::NotFound {
            schema_id,
            referering_location,
        }
    }
    pub fn custom(schema_id: Uri, source: impl 'static + Error + Send + Sync) -> Self {
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
pub enum SetupError {
    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    MetaSchema {
        /// The [`Uri`] of meta schema which encountered an error
        schema_id: crate::uri::Uri,
        /// The error which occurred when parsing the meta schema
        source: MetaSchemaError,
    },
    /// An error occurred parsing a [`Uri`]
    Uri {
        /// The [`Uri`] of the schema which encountered an error
        schema_id: crate::uri::Uri,
        /// The error which occurred when parsing the [`Uri`]
        source: UriError,
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
    pub schema_id: Uri,
    pub path: Pointer,
}

impl std::fmt::Display for SchemaNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "schema \"{}\" not found", self.schema_id)
    }
}
impl StdError for SchemaNotFoundError {}

#[derive(Debug, Clone)]
pub struct UnexpectedValueError {
    pub schema_id: Option<Uri>,
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
            value_type_name(&self.found)
        )
    }
}
impl StdError for UnexpectedValueError {}

#[derive(Debug, Clone, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum MetaSchemaError {
    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    #[snafu(display("{}", source), context(false))]
    Unknown { source: UnkownMetaSchemaError },
    #[snafu(display(r#"error parsing "$schema": {source}"#))]
    MetaSchemaUri { source: UriError },
}

#[derive(Debug, Clone)]
pub struct UnkownMetaSchemaError {
    pub schema_id: Uri,
}
impl Display for UnkownMetaSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown meta schema: {}", self.schema_id)
    }
}
impl StdError for UnkownMetaSchemaError {}

#[derive(Debug, Clone, Snafu)]
#[snafu(visibility(pub), context(suffix(false)), module)]
pub enum UriError {
    #[snafu(display(
        r#"expected a string in URI format for "{property}", found {}"#,
        value_type_name(found)
    ))]
    Invalid {
        property: Cow<'static, str>,
        found: Box<Value>,
    },
    #[snafu(display(r#"malformed URI for "{property}": {source}"#))]
    Malformed {
        property: Cow<'static, str>,
        source: crate::uri::Error,
        value: Box<Value>,
    },
}

impl UriError {
    #[must_use]
    pub fn property(&self) -> Cow<'static, str> {
        match self {
            UriError::Invalid { property, .. } | UriError::Malformed { property, .. } => {
                property.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use snafu::ResultExt;

    use super::*;

    fn setup_error_uri() -> Result<(), SetupError> {
        Err(UriError::Invalid {
            property: "$schema".into(),
            found: Value::Bool(true).into(),
        })
        .context(setup_error::Uri {
            schema_id: crate::uri::Uri::default(),
        })
    }

    #[test]
    fn test_uri_error() {
        let err = setup_error_uri().unwrap_err();

        println!("{err}");
    }
}
