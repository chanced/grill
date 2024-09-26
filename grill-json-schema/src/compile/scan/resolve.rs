use grill_core::{
    resolve::{Error, Resolve},
    source::{Fragment, LinkError, New, Sources},
};
use grill_uri::{decode_lossy, AbsoluteUri};
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
    if let Some(source_key) = sources.source_key_by_uri(uri) {
        return Ok(Resolved::Source(resolved::Src {
            link_created: false,
            source_key,
            located: Located::Local,
            document_key: sources.document_key_for(source_key),
        }));
    }

    let fragment = uri.fragment_decoded_lossy().unwrap_or_default();
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
    let uri = uri.without_fragment();
    polonius!(|sources| -> Result<resolved::Src, Error<R>> {
        if let Some(source_key) = sources.source_key_by_uri(&uri) {
            polonius_return!(Ok(resolved::Src {
                link_created: false,
                source_key,
                located: Located::Local,
                document_key: sources.document_key_for(source_key),
            }));
        }
    });
    let value = fetch(resolver, &uri).await?;
    let source_key = sources.insert(uri, value).unwrap();
    let document_key = sources.document_key_for(source_key);
    Ok(resolved::Src {
        link_created: true,
        source_key,
        document_key,
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
    Ok(Resolved::UnknownAnchor(resolved::UnknownAnchor {
        document_key: root.document_key,
        located: root.located,
        source_key: root.source_key,
        anchor,
    }))
}

async fn resolve_path<'int, R: 'static + Resolve>(
    sources: &'int mut Sources,
    resolver: &R,
    uri: &AbsoluteUri,
    base: AbsoluteUri,
    fragment: String,
) -> Result<Resolved, Error<R>> {
    let resolved_root = resolve_root(sources, resolver, &base).await?;
    let located = resolved_root.located;
    let document_key = resolved_root.document_key;

    let relative_path = Pointer::parse(&fragment).map_err(|source| Error::InvalidPointer {
        uri: uri.clone(),
        source,
    })?;
    let mut absolute_path = sources.source(resolved_root.source_key).path().to_buf();
    absolute_path.append(&relative_path);
    let source_key = sources
        .link(New {
            uri: uri.clone(),
            document_key,
            absolute_path,
            fragment: Some(Fragment::Pointer(relative_path.to_buf())),
        })
        .map_err(|source| match source {
            LinkError::InvalidPath(err) => Error::PathNotFound {
                uri: uri.clone(),
                source: err,
            },
            LinkError::SourceConflict(err) => unreachable!(
                "\n\nsource conflict:\n\nuri: {uri}\ndocument_key: {document_key:?}\nerr: {err}",
            ),
        })?;

    Ok(Resolved::Source(resolved::Src {
        link_created: true,
        source_key,
        document_key,
        located,
    }))
}

pub(super) mod resolved {
    use super::Located;
    use grill_core::source::{DocumentKey, SourceKey};
    #[derive(Debug)]
    pub struct Src {
        /// whether or not a link was created
        pub link_created: bool,
        /// the source
        pub source_key: SourceKey,
        /// whether the root doc already existed or if it had to be fetched with
        /// `Resolve`
        pub located: Located,
        pub document_key: DocumentKey,
    }

    #[derive(Debug)]
    pub struct UnknownAnchor {
        /// the document key
        pub document_key: DocumentKey,
        /// root source key
        pub source_key: SourceKey,
        /// whether the root doc already existed or if it had to be fetched with
        /// `Resolve`
        pub located: Located,
        /// the anchor
        pub anchor: String,
    }
    #[derive(Debug)]
    pub enum Resolved {
        /// Returned when the source path can be located within the document.
        Source(Src),
        /// The root document is returned if an anchor exists in the uri.
        UnknownAnchor(UnknownAnchor),
    }
    impl Resolved {
        fn located(&self) -> Located {
            match self {
                Self::UnknownAnchor(doc) => doc.located,
                Self::Source(source) => source.located,
            }
        }
    }
}
pub(super) use resolved::Resolved;

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
