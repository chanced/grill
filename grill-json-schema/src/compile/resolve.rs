use grill_core::{
    lang::{
        source::{Document, DocumentKey, LinkError, Source},
        Sources,
    },
    resolve::ResolveError,
    Resolve,
};
use grill_uri::AbsoluteUri;
use jsonptr::Pointer;
use polonius_the_crab::{polonius, polonius_return};
use serde_json::Value;
use std::sync::Arc;

pub(super) async fn resolve<'int, 'u, R: 'static + Resolve>(
    mut sources: &'int mut Sources,
    resolver: &R,
    uri: &'u AbsoluteUri,
) -> Result<Resolved<'int, 'u>, ResolveError<R>> {
    polonius!(
        |sources| -> Result<Resolved<'polonius, 'u>, ResolveError<R>> {
            if let Some(source) = sources.source_by_uri(uri) {
                polonius_return!(Ok(Resolved::Source {
                    linked: false,
                    source,
                    located: Located::Local
                }));
            }
        }
    );
    let fragment = uri.fragment().unwrap_or_default();
    let base = uri.without_fragment();
    if fragment.is_empty() || fragment.starts_with('/') {
        resolve_path(sources, resolver, uri, base, fragment).await
    } else {
        resolve_unknown_anchor(sources, resolver, base, fragment).await
    }
}

async fn fetch<'int, R: 'static + Resolve>(
    resolver: &R,
    uri: &AbsoluteUri,
) -> Result<Arc<Value>, ResolveError<R>> {
    resolver
        .resolve(uri)
        .await
        .map_err(|source| ResolveError::FailedToResolve {
            uri: uri.clone(),
            source,
        })
}

/// Resolves the root document by calling `Resolve::resolve` on `resolver` and
/// inserting it into `sources`.
///
/// ## Panics
/// - panics if the uri must not already be in sources
///
/// ## Errors
/// Returns `ResolveError::FailedToResolve` if the uri could not be resolved by
/// `resolver`.
async fn resolve_root<'int, 'u, R: 'static + Resolve>(
    mut sources: &'int mut Sources,
    resolver: &R,
    uri: &'u AbsoluteUri,
) -> Result<Resolved<'int, 'u>, ResolveError<R>> {
    assert!(uri.fragment_is_empty_or_none());
    polonius!(
        |sources| -> Result<Resolved<'polonius, 'u>, ResolveError<R>> {
            if let Some(source) = sources.source_by_uri(uri) {
                polonius_return!(Ok(Resolved::Source {
                    linked: false,
                    source,
                    located: Located::Local
                }));
            }
        }
    );
    let value = fetch(resolver, uri).await?;
    let source_key = sources.insert(uri.clone(), value).unwrap();
    let source = sources.source(source_key).unwrap();
    Ok(Resolved::Source {
        linked: true,
        source,
        located: Located::Remote,
    })
}

async fn resolve_unknown_anchor<'int, 'u, R: 'static + Resolve>(
    sources: &'int mut Sources,
    resolver: &R,
    base: AbsoluteUri,
    anchor: &'u str,
) -> Result<Resolved<'int, 'u>, ResolveError<R>> {
    let root = resolve_root(sources, resolver, &base).await?;
    let located = root.located(); // copy
    let document = root.take_document(); // drops root
    Ok(Resolved::Document {
        document,
        located,
        anchor,
    })
}

async fn resolve_path<'int, 'u, R: 'static + Resolve>(
    sources: &'int mut Sources,
    resolver: &R,
    uri: &AbsoluteUri,
    base: AbsoluteUri,
    fragment: &str,
) -> Result<Resolved<'int, 'u>, ResolveError<R>> {
    let root = resolve_root(sources, resolver, &base).await?;
    let located = root.located(); // copy
    let document_key = root.take_document().key(); // drops root
    let path = Pointer::parse(fragment).map_err(|source| ResolveError::InvalidPointer {
        uri: uri.clone(),
        source,
    })?;
    let source_key =
        sources
            .link(uri.clone(), document_key, path.to_buf())
            .map_err(|source| match source {
                LinkError::InvalidPath(err) => ResolveError::PathNotFound {
                    uri: uri.clone(),
                    source: err,
                },
                LinkError::SourceConflict(err) => unreachable!(
                "source conflict: uri: {uri}, document_key: {document_key:?}, path: {path}, err: {err}",
            ),
            })?;
    let source = sources.source(source_key).unwrap();

    Ok(Resolved::Source {
        linked: true,
        source,
        located,
    })
}

#[derive(Debug)]
pub(super) enum Resolved<'int, 'u> {
    /// Returned when the source path can be located within the document.
    Source {
        /// whether or not a link was created
        linked: bool,
        /// the source
        source: Source<'int>,
        /// whether the root doc already existed or if it had to be fetched with
        /// `Resolve`
        located: Located,
    },
    /// The root document is returned if an anchor exists in the uri.
    Document {
        document: Document<'int>,
        located: Located,
        anchor: &'u str,
    },
}
impl<'int, 'u> Resolved<'int, 'u> {
    fn located(&self) -> Located {
        match self {
            Self::Document { located, .. } | Self::Source { located, .. } => *located,
        }
    }
    fn document_key(&self) -> DocumentKey {
        self.document().key()
    }
    fn document(&self) -> &Document<'int> {
        match self {
            Self::Document { document, .. } => document,
            Self::Source { source, .. } => source.document(),
        }
    }
    fn take_document(self) -> Document<'int> {
        match self {
            Self::Document { document, .. } => document,
            Self::Source { source, .. } => source.take_document(),
        }
    }
}

/// Indicates whether or not a `Source` was located within `Sources` or if it
/// had to be fetched with `Resolve`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Located {
    /// document was already in `Sources`
    Local,
    /// document was fetched with `Resolve`
    Remote,
}

impl Located {
    pub(super) fn is_local(&self) -> bool {
        self == &Self::Local
    }
    pub(super) fn is_remote(&self) -> bool {
        self == &Self::Remote
    }
}
