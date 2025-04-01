use grill_core::{
    resolve::{Error, Resolve},
    source::{DocumentKey, New, SourceKey, Sources},
};
use grill_uri::AbsoluteUri;
use jsonptr::PointerBuf;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug)]
pub(super) struct UnknownAnchor {
    /// the document key
    pub document_key: DocumentKey,
    /// root source key
    pub root_source_key: SourceKey,
    /// the anchor
    pub anchor: String,
}

#[derive(Debug)]
pub(super) enum Resolved {
    /// Returned when the source path can be located within the document.
    Source(SourceKey),
    /// The root document is returned if an anchor exists in the uri.
    UnknownAnchor(UnknownAnchor),
}
impl From<SourceKey> for Resolved {
    fn from(source_key: SourceKey) -> Self {
        Self::Source(source_key)
    }
}

pub(super) async fn resolve<'res, 'int, R: 'static + Resolve>(
    sources: &'int mut Sources,
    resolver: &'res R,
    uri: &AbsoluteUri,
) -> Result<Resolved, Error<R>> {
    if let Some(source_key) = sources.source_key_by_uri(uri) {
        return Ok(source_key.into());
    }
    let fragment = uri.fragment_decoded_lossy().unwrap_or_default();
    let base = uri.without_fragment();
    let root = resolve_root(sources, resolver, &base).await?;

    if !fragment.is_empty() && !fragment.starts_with('/') {
        // resolve_source(sources, resolver, uri, base, fragment).await
    } else {
        // resolve_unknown_anchor(sources, resolver, base, fragment).await
    }
    todo!()
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
/// ## Errors
/// Returns `ResolveError::FailedToResolve` if the uri could not be resolved by
/// `resolver`.
async fn resolve_root<'int, R: 'static + Resolve>(
    sources: &'int mut Sources,
    resolver: &R,
    uri: &AbsoluteUri,
) -> Result<SourceKey, Error<R>> {
    let uri = uri.without_fragment();
    if let Some(source_key) = sources.source_key_by_uri(&uri) {
        return Ok(source_key);
    }
    let value = fetch(resolver, &uri).await?;
    // insert is safe due to the check above
    Ok(sources.insert(uri, value).unwrap())
}

async fn resolve_source<'int, R: 'static + Resolve>(
    sources: &'int mut Sources,
    resolver: &R,
    uri: &AbsoluteUri,
    base: AbsoluteUri,
    fragment: String,
) -> Result<Resolved, Error<R>> {
    todo!()
    // let Source {
    //     source_key,
    //     located,
    //     document_key,
    // } = resolve_root(sources, resolver, &base).await?;
    // let relative_path = PointerBuf::parse(&fragment).map_err(|source| Error::InvalidPointer {
    //     uri: uri.clone(),
    //     source,
    // })?;

    // let absolute_path = sources
    //     .get(source_key)
    //     .absolute_path()
    //     .concat(&relative_path);

    // let link = sources.link(New {
    //     uri: uri.clone(),
    //     fragment: Some(relative_path.into()),
    //     document_key,
    //     absolute_path: absolute_path.clone(),
    // });

    // Ok(Resolved::Unlinked(Unlinked {
    //     located,
    //     absolute_path,
    //     root_source_key: resolved_root.source_key,
    // }))
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
