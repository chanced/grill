use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use serde_json::Value;
use snafu::ResultExt;
use url::Url;

use crate::{
    error::{resolve_error, DeserializeError, ResolveError},
    uri::AbsoluteUri,
};

#[async_trait]
pub trait Resolve: DynClone + Send + Sync + 'static {
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Option<String>, ResolveError>;
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
/// A [`Resolve`] implementation that uses HTTP(S) to resolve schema sources.
impl HttpResolver {
    #[must_use]
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[cfg(feature = "http")]
#[async_trait]
impl Resolve for HttpResolver {
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Option<String>, ResolveError> {
        let Some(url) = uri.as_url() else { return Ok(None) };
        let scheme = url.scheme();
        if scheme != "http" && scheme != "https" {
            return Ok(None);
        }

        match self.client.get(url.clone()).send().await {
            Ok(resp) => {
                let text = resp.text().await.context(resolve_error::Reqwest {
                    schema_id: url.to_string(),
                })?;
                Ok(Some(text))
            }
            Err(err) => {
                if matches!(err.status(), Some(reqwest::StatusCode::NOT_FOUND)) {
                    return Ok(None);
                }
                Err(ResolveError::Reqwest {
                    schema_id: url.to_string(),
                    source: err,
                })
            }
        }
    }
}
