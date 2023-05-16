use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use serde_json::Value;
use snafu::ResultExt;

use crate::{
    error::{DeserializeError, DeserializeSnafu, ReqwestSnafu, ResolveError},
    Schema,
};

#[async_trait]
pub trait Resolve: DynClone + Send + Sync + 'static {
    async fn resolve(&self, uri: &crate::Uri) -> Result<Option<Value>, ResolveError>;
    async fn resolve_schema(&self, uri: &crate::Uri) -> Result<Option<Schema>, ResolveError> {
        match self.resolve(uri).await? {
            Some(value) => match serde_json::from_value(value) {
                Ok(schema) => Ok(Some(schema)),
                Err(err) => Err(DeserializeError::from(err)).context(DeserializeSnafu {
                    schema_id: uri.clone(),
                }),
            },
            None => Ok(None),
        }
    }
}

clone_trait_object!(Resolve);

///
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
#[derive(Clone, Debug)]
pub struct HttpResolver {
    client: reqwest::Client,
}

#[cfg(feature = "http")]
// #[cfg_attr(docsrs, doc(cfg(feature = "rt")))]
/// A [`Resolve`] implementation that uses HTTP(S) to resolve schemas.
impl HttpResolver {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[cfg(feature = "http")]
#[async_trait]
impl Resolve for HttpResolver {
    async fn resolve(&self, uri: &crate::Uri) -> Result<Option<Value>, ResolveError> {
        match uri.scheme() {
            Some("http" | "https") => {}
            _ => return Ok(None),
        }
        let req_uri = uri.to_string();
        let resp = self.client.get(&req_uri).send().await;
        if let Err(err) = resp {
            if matches!(err.status(), Some(reqwest::StatusCode::NOT_FOUND)) {
                return Ok(None);
            }
            return Err(ResolveError::Reqwest {
                schema_id: uri.clone(),
                source: err,
            });
        }
        let resp = resp.unwrap();

        let text = resp.text().await.context(ReqwestSnafu {
            schema_id: uri.clone(),
        })?;
        Ok(Some(deserialize_str_value(&text).context(
            DeserializeSnafu {
                schema_id: uri.clone(),
            },
        )?))
    }
}
/// Attempts to deserialize a [`&str`] into a [`Value`] first with JSON. If JSON
/// deserialization fails and either `"yaml"` or `"toml"` features are enabled,
/// it will attempt to deserialize with the enabled formats.
///
/// # Errors
/// Returns a [`DeserializeError`] containing errors for each format attempted in
/// the event all formats fail.
pub fn deserialize_str_value(text: &str) -> Result<Value, DeserializeError> {
    let mut de_err: DeserializeError;
    match serde_json::from_str::<Value>(text) {
        Ok(schema) => return Ok(schema),
        Err(err) => {
            #[cfg(all(not(feature = "yaml"), not(feature = "toml")))]
            {
                return Err(DeserializeError { json: err });
            }
            #[cfg(any(feature = "yaml", feature = "toml"))]
            {
                de_err = err.into();
            }
        }
    }
    #[cfg(feature = "yaml")]
    {
        match yaml::from_str::<Value>(text) {
            Ok(schema) => return Ok(schema),
            Err(err) => de_err.yaml = err.into(),
        }
    }
    #[cfg(feature = "toml")]
    {
        match toml::from_str::<Value>(text) {
            Ok(schema) => return Ok(schema),
            Err(err) => de_err.toml = err.into(),
        }
    }
    Err(de_err)
}

/// Attempts to deserialize a `&[u8]` into a [`Value`] first with JSON. If JSON
/// deserialization fails and either `"yaml"` or `"toml"` features are enabled,
/// it will attempt to deserialize with the enabled formats.
///
/// # Errors
/// Returns a [`DeserializeError`] containing errors for each format attempted in
/// the event all formats fail.
pub fn deserialize_slice_value(slice: &[u8]) -> Result<Value, DeserializeError> {
    let mut de_err: DeserializeError;
    match serde_json::from_slice::<Value>(slice) {
        Ok(schema) => return Ok(schema),
        Err(err) => {
            #[cfg(all(not(feature = "yaml"), not(feature = "toml")))]
            {
                return Err(DeserializeError { json: err });
            }
            #[cfg(any(feature = "yaml", feature = "toml"))]
            {
                de_err = err.into();
            }
        }
    }
    #[cfg(feature = "yaml")]
    {
        match yaml::from_slice::<Value>(slice) {
            Ok(schema) => return Ok(schema),
            Err(err) => de_err.yaml = err.into(),
        }
    }
    #[cfg(feature = "toml")]
    {
        let str = String::from_utf8_lossy(slice);
        match toml::from_str::<Value>(&str) {
            Ok(schema) => return Ok(schema),
            Err(err) => de_err.toml = err.into(),
        }
    }
    Err(de_err)
}
