use jsonptr::Pointer;

use crate::AbsoluteUri;

use super::SourceKey;

/// A file reference
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Link {
    pub(crate) key: SourceKey,
    pub(crate) uri: AbsoluteUri,
    pub(crate) root_uri: AbsoluteUri,
    pub(crate) path: Pointer,
}

impl Link {
    pub(crate) fn new(key: SourceKey, uri: AbsoluteUri, path: Pointer) -> Self {
        let mut root_uri = uri.clone();
        root_uri.set_fragment(None).unwrap();
        Self {
            key,
            uri,
            root_uri,
            path,
        }
    }
}
impl From<&Link> for (AbsoluteUri, Pointer) {
    fn from(value: &Link) -> Self {
        (value.uri.clone(), value.path.clone())
    }
}
