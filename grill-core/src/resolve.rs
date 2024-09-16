use grill_uri::AbsoluteUri;
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    sync::Arc,
};

use crate::lang::source::InvalidLinkPathError;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Resolve                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait for resolving and deserializing a [`Value`] at a given [`AbsoluteUri`].
#[trait_variant::make(Send)]
pub trait Resolve: fmt::Debug + Send + Sync {
    /// The error type that can be returned when resolving a [`Value`].
    type Error: 'static + std::error::Error + Send + Sync;

    /// Resolves and deserializes a [`Value`] at the supplied [`AbsoluteUri`].
    ///
    /// # Errors
    /// Returns [`Self::Error`] if an error occurs during resolution.
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error>;
}

/// A [`Resolve`] implementation that always returns [`NotFoundError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoResolve;
impl Resolve for NoResolve {
    type Error = NotFoundError;
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error> {
        Err(NotFoundError::new(uri.clone()))
    }
}

macro_rules! resolve_maps {
    ($($map:ident),*) => {
        $(
            impl Resolve for $map<AbsoluteUri, Arc<Value>> {
                type Error = NotFoundError;
                async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error> {
                    self.get(uri)
                        .cloned()
                        .ok_or_else(|| NotFoundError::new(uri.clone()))
                }
            }
            impl Resolve for $map<AbsoluteUri, Value> {
                type Error = NotFoundError;
                async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error> {
                    self.get(uri)
                        .cloned()
                        .map(Arc::new)
                        .ok_or_else(|| NotFoundError::new(uri.clone()))
                }
            }
        )*
    };
}
resolve_maps!(HashMap, BTreeMap);

impl Resolve for () {
    type Error = NotFoundError;
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error> {
        Err(NotFoundError::new(uri.clone()))
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 ResolveError                                 ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug)]
pub enum ResolveError<R: 'static + Resolve> {
    FailedToResolve {
        uri: AbsoluteUri,
        source: R::Error,
    },
    UnknownAnchor {
        uri: AbsoluteUri,
    },
    /// The JSON pointer fragment of the uri is malformed.
    InvalidPointer {
        uri: AbsoluteUri,
        source: jsonptr::ParseError,
    },
    /// Failed to link source
    PathNotFound {
        uri: AbsoluteUri,
        source: Box<InvalidLinkPathError>,
    },
}

impl<R: Resolve> ResolveError<R> {
    pub fn uri(&self) -> &AbsoluteUri {
        match self {
            Self::FailedToResolve { uri, .. } => uri,
            Self::InvalidPointer { uri, .. } => uri,
            Self::PathNotFound { uri, .. } => uri,
            Self::UnknownAnchor { uri } => uri,
        }
    }
}

impl<R: Resolve> fmt::Display for ResolveError<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToResolve { uri, .. } => {
                write!(f, "failed to resolve source at uri \"{uri}\"")
            }
            Self::InvalidPointer { uri, .. } => {
                write!(
                    f,
                    "json pointer (\"{}\") of uri \"{uri}\" is invalid",
                    uri.fragment().unwrap_or_default()
                )
            }
            ResolveError::PathNotFound { uri, .. } => {
                write!(
                    f,
                    "path \"{}\" is not present in value at \"{uri}\"",
                    uri.fragment().unwrap_or_default()
                )
            }
            ResolveError::UnknownAnchor { uri } => {
                write!(
                    f,
                    "anchor \"{}\" is not linked to a location in the value at \"{uri}\"",
                    uri.fragment().unwrap_or_default()
                )
            }
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                NotFoundError                                 ║
║                               ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A source was not found at the given URI.
#[derive(Debug, PartialEq)]
pub struct NotFoundError {
    /// The URI that was not found.
    pub uri: AbsoluteUri,
}
impl NotFoundError {
    /// Returns a new `Result<T, Self>::Err(Self)` with the given URI.
    #[must_use]
    pub fn new(uri: AbsoluteUri) -> Self {
        NotFoundError { uri }
    }
    pub fn err_with<F, T, E>(f: F) -> Result<T, E>
    where
        E: From<Self>,
        F: Fn() -> Self,
    {
        Err(f().into())
    }
}
impl std::error::Error for NotFoundError {}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "source not found: {}", self.uri)
    }
}

impl<R: Resolve> std::error::Error for ResolveError<R> {}
