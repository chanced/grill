use grill_core::{
    lang::{
        source::{Document, DocumentKey, LinkError, Source},
        Sources,
    },
    resolve::Error,
    Resolve,
};
use grill_uri::AbsoluteUri;
use jsonptr::Pointer;
use polonius_the_crab::{polonius, polonius_return};
use serde_json::Value;
use std::sync::Arc;

// cound't get around cloning & returning an owned `Resolved` here.
pub(super) async fn resolve<'res, 'int, R: 'static + Resolve>(
    sources: &'int mut Sources,
    resolver: &'res R,
    uri: &AbsoluteUri,
) -> Result<Resolved, Error<R>> {
    if let Some(source) = sources.source_by_uri(uri).map(|s| s.into_owned()) {
        return Ok(Resolved::Source(resolved::Src {
            link_created: false,
            source,
            located: Located::Local,
        }));
    }
    let fragment = grill_uri::decode_lossy(uri.fragment().unwrap_or_default());
    let base = uri.without_fragment();
    if fragment.is_empty() || fragment.starts_with('/') {
        resolve_path(sources, resolver, uri, base, &fragment).await
    } else {
        resolve_unknown_anchor(sources, resolver, base, fragment).await
    }
}

async fn fetch<'int, R: 'static + Resolve>(
    resolver: &R,
    uri: &AbsoluteUri,
) -> Result<Arc<Value>, Error<R>> {
    resolver
        .resolve(uri)
        .await
        .map_err(|source| Error::FailedToResolve {
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
async fn resolve_root<'int, R: 'static + Resolve>(
    mut sources: &'int mut Sources,
    resolver: &R,
    uri: &AbsoluteUri,
) -> Result<resolved::Src, Error<R>> {
    assert!(uri.fragment_is_empty_or_none());
    polonius!(|sources| -> Result<resolved::Src, Error<R>> {
        if let Some(source) = sources.source_by_uri(uri).map(|s| s.into_owned()) {
            polonius_return!(Ok(resolved::Src {
                link_created: false,
                source,
                located: Located::Local
            }));
        }
    });
    let value = fetch(resolver, uri).await?;
    let source_key = sources.insert(uri.clone(), value).unwrap();
    let source = sources.source(source_key).unwrap().into_owned();
    Ok(resolved::Src {
        link_created: true,
        source,
        located: Located::Remote,
    })
}

async fn resolve_unknown_anchor<'int, R: 'static + Resolve>(
    sources: &'int mut Sources,
    resolver: &R,
    base: AbsoluteUri,
    anchor: String,
) -> Result<Resolved, Error<R>> {
    let root = resolve_root(sources, resolver, &base).await?;
    let located = root.located;
    let document = root.take_document();
    Ok(Resolved::Document(resolved::Doc {
        document,
        located,
        anchor,
    }))
}

async fn resolve_path<'int, R: 'static + Resolve>(
    sources: &'int mut Sources,
    resolver: &R,
    uri: &AbsoluteUri,
    base: AbsoluteUri,
    fragment: &str,
) -> Result<Resolved, Error<R>> {
    let root = resolve_root(sources, resolver, &base).await?;
    let located = root.located;
    let document_key = root.take_document().key(); // drops root
    let path = Pointer::parse(fragment).map_err(|source| Error::InvalidPointer {
        uri: uri.clone(),
        source,
    })?;
    let source_key =
        sources
            .link(uri.clone(), document_key, path.to_buf())
            .map_err(|source| match source {
                LinkError::InvalidPath(err) => Error::PathNotFound {
                    uri: uri.clone(),
                    source: err,
                },
                LinkError::SourceConflict(err) => unreachable!(
                "source conflict: uri: {uri}, document_key: {document_key:?}, path: {path}, err: {err}",
            ),
            })?;
    let source = sources.source(source_key).unwrap().into_owned();

    Ok(Resolved::Source(resolved::Src {
        link_created: true,
        source,
        located,
    }))
}
pub(super) mod resolved {
    use super::Located;

    #[derive(Debug)]
    pub(crate) struct Src {
        /// whether or not a link was created
        pub(crate) link_created: bool,
        /// the source
        pub(super) source: super::Source<'static>,
        /// whether the root doc already existed or if it had to be fetched with
        /// `Resolve`
        pub(super) located: Located,
    }

    impl Src {
        pub(super) fn take_document(self) -> super::Document<'static> {
            self.source.take_document()
        }
    }

    #[derive(Debug)]
    pub(crate) struct Doc {
        /// the document
        pub(super) document: super::Document<'static>,
        /// whether the root doc already existed or if it had to be fetched with
        /// `Resolve`
        pub(super) located: Located,
        /// the anchor
        pub(super) anchor: String,
    }
    impl Doc {
        pub(super) fn take_document(self) -> super::Document<'static> {
            self.document
        }
    }
}

#[derive(Debug)]
pub(super) enum Resolved {
    /// Returned when the source path can be located within the document.
    Source(resolved::Src),
    /// The root document is returned if an anchor exists in the uri.
    Document(resolved::Doc),
}
impl Resolved {
    fn located(&self) -> Located {
        match self {
            Self::Document(doc) => doc.located,
            Self::Source(source) => source.located,
        }
    }
    fn document_key(&self) -> DocumentKey {
        self.document().key()
    }
    fn document(&self) -> &Document {
        match self {
            Self::Document(doc) => &doc.document,
            Self::Source(src) => src.source.document(),
        }
    }
    fn take_document(self) -> Document<'static> {
        match self {
            Self::Document(doc) => doc.take_document(),
            Self::Source(src) => src.take_document(),
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
