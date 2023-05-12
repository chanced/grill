use std::{borrow::Cow, error::Error};

use crate::{value_type_name, Types, Uri};
use jsonptr::Pointer;
use serde_json::Value;

// intentionally not worrying about the fact that this is missing

#[derive(Debug, thiserror::Error)]
pub enum SetupError {
    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    #[error(r#"error parsing "$schema": {0}"#)]
    MetaSchema(Box<MetaSchemaError>),
    #[error(transparent)]
    Uri(Box<UriError>),
    #[error(r#"schema not found: "{0}""#)]
    SchemaNotFound(Box<SchemaNotFoundError>),
    #[error(r#"failed to parse property "{}" at path "{}""#, .0.property, .0.pointer)]
    UnexpectedValue(Box<UnexpectedValueError>),
    #[error(transparent)]
    Internal(#[from] Box<dyn Error + Send + Sync>),
}
impl From<MetaSchemaError> for SetupError {
    fn from(error: MetaSchemaError) -> Self {
        Self::MetaSchema(Box::new(error))
    }
}
impl From<UriError> for SetupError {
    fn from(error: UriError) -> Self {
        Self::Uri(Box::new(error))
    }
}
impl From<SchemaNotFoundError> for SetupError {
    fn from(error: SchemaNotFoundError) -> Self {
        Self::SchemaNotFound(Box::new(error))
    }
}
impl From<UnexpectedValueError> for SetupError {
    fn from(error: UnexpectedValueError) -> Self {
        Self::UnexpectedValue(Box::new(error))
    }
}

impl SetupError {
    pub fn internal(e: impl 'static + Error + Send + Sync) -> Self {
        Self::Internal(Box::new(e))
    }
}

/// Custom errors are only returned by custom [`Execute`](crate::keyword::Execute) implementations.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct InternalError(#[from] pub Box<dyn Error + Send + Sync>);

impl InternalError {
    pub fn new<E>(error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self(Box::new(error))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error(r#"schema "{}" at path "{}" not found"#, schema_id, path)]
pub struct SchemaNotFoundError {
    pub schema_id: Uri,
    pub path: Pointer,
}

#[derive(Debug, Clone, thiserror::Error)]
#[error(r#"error parsing {pointer}: {msg}"#)]
pub struct UnexpectedValueError {
    pub schema_id: Option<Uri>,
    pub pointer: Pointer,
    pub property: Cow<'static, str>,
    pub expected_types: Types,
    pub found: Box<Value>,
    pub msg: String,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum MetaSchemaError {
    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    #[error("unknown meta schema: {0}")]
    Unknown(Uri),
    #[error(r#"error parsing "$schema": {0}"#)]
    Uri(#[from] UriError),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum UriError {
    #[error(
        r#"expected a string in URI format for "{property}", found {}"#,
        value_type_name(found)
    )]
    Invalid {
        property: Cow<'static, str>,
        found: Box<Value>,
    },
    #[error(r#"malformed URI for "{property}": {source}"#)]
    Malformed {
        property: Cow<'static, str>,
        source: crate::uri::Error,
        value: Box<Value>,
    },
}
impl UriError {
    pub fn property(&self) -> Cow<'static, str> {
        match self {
            UriError::Invalid { property, .. } => property.clone(),
            UriError::Malformed { property, .. } => property.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_error() {
        let err = UriError::Invalid {
            property: "$schema".into(),
            found: Value::Bool(true).into(),
        };
        assert_eq!(
            err.to_string(),
            r#"expected a string in URI format for "$schema", found boolean"#
        );

        let source = crate::uri::Uri::parse("invalid uri").unwrap_err();
        let uri_err = UriError::Malformed {
            property: "$id".into(),
            source,
            value: Value::String("invalid uri".into()).into(),
        };

        assert!(uri_err
            .to_string()
            .starts_with(r#"malformed URI for "$id": "#));

        let err = SetupError::from(uri_err);
        assert!(err.to_string().starts_with(r#"malformed URI for "$id": "#));
    }
}
