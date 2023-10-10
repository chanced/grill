use crate::{source::SourceKey, AbsoluteUri, Key, Uri};

#[derive(Debug)]
pub struct Ref {
    pub uri: Uri,
    pub keyword: &'static str,
}

/// A reference to a schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub(crate) src_key: SourceKey,
    /// Key to the referenced [`Schema`]
    pub key: Key,
    /// The referenced URI
    pub uri: Uri,
    /// The resolved Absolute URI
    pub absolute_uri: AbsoluteUri,
    /// The keyword of the reference (e.g. $ref, $dynamicRef, $recursiveRef, etc)
    pub keyword: &'static str,
}

impl Reference {}
