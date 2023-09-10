use jsonptr::Pointer;

use crate::AbsoluteUri;

use super::SourceKey;

/// A file reference
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Link {
    pub(crate) key: SourceKey,
    pub(crate) uri: AbsoluteUri,
    pub(crate) path: Pointer,
}

impl Link {
    pub(crate) fn new(key: SourceKey, uri: AbsoluteUri, path: Pointer) -> Self {
        Self { key, uri, path }
    }
    pub fn uri(&self) -> &AbsoluteUri {
        &self.uri
    }
    pub fn path(&self) -> &Pointer {
        &self.path
    }
}
impl From<&Link> for (AbsoluteUri, Pointer) {
    fn from(value: &Link) -> Self {
        (value.uri.clone(), value.path.clone())
    }
}
