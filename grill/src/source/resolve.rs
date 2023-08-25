//! Traits and implementations for loading JSON Schema source definitions.

use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};

use crate::{
    error::{NotFoundError, ResolveError, ResolveErrors},
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
                let text = resp
                    .text()
                    .await
                    .map_err(|err| ResolveError::new(err, uri.clone()))?;
                Ok(Some(text))
            }
            Err(err) if matches!(err.status(), Some(reqwest::StatusCode::NOT_FOUND)) => Ok(None),
            Err(err) => Err(ResolveError::new(err, uri.clone())),
        }
    }
}

#[derive(Clone)]
pub struct Resolvers {
    resolvers: Vec<Box<dyn Resolve>>,
}

impl Resolvers {
    #[must_use]
    pub fn new(resolvers: Vec<Box<dyn Resolve>>) -> Self {
        Self { resolvers }
    }

    pub async fn resolve(&self, uri: &AbsoluteUri) -> Result<String, ResolveErrors> {
        let mut errors = ResolveErrors::default();
        for resolver in &self.resolvers {
            match resolver.resolve(uri).await {
                Ok(Some(data)) => {
                    return Ok(data);
                }
                Err(err) => errors.push(err),
                _ => continue,
            }
        }
        if errors.is_empty() {
            errors.push_not_found(uri.clone());
        }
        Err(errors)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Resolve>> {
        self.resolvers.iter()
    }
}
impl<'a> IntoIterator for &'a Resolvers {
    type Item = &'a Box<dyn Resolve>;
    type IntoIter = std::slice::Iter<'a, Box<dyn Resolve>>;

    fn into_iter(self) -> Self::IntoIter {
        self.resolvers.iter()
    }
}

#[cfg(test)]
mockall::mock! {
    pub Resolver{}

    #[async_trait]
    impl Resolve for Resolver {
        async fn resolve(&self, uri: &AbsoluteUri) -> Result<Option<String>, ResolveError>;
    }
    impl Clone for Resolver {
        fn clone(&self) -> Self;
    }
}
