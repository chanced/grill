use jsonptr::Pointer;

use crate::{source::SourceKey, AbsoluteUri, Key};

// ///
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Ref {
//     /// Path to the $ref, $dynamicRef, $recursiveRef, etc
//     pub path: Pointer,
//     /// The referenced URI
//     pub uri: AbsoluteUri,
//     /// The keyword of the reference (e.g. $ref, $dynamicRef, $recursiveRef, etc)
//     pub keyword: &'static str,
// }

/// A reference to a schema. This type is almost identical to `Ref` except
/// contains keys for the referenced schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub(crate) src_key: SourceKey,
    /// Key to the referenced [`Schema`]
    pub key: Key,
    /// Path to the $ref, $dynamicRef, $recursiveRef, etc
    pub ref_path: Pointer,
    /// The referenced URI
    pub uri: AbsoluteUri,
    /// The keyword of the reference (e.g. $ref, $dynamicRef, $recursiveRef, etc)
    pub keyword: &'static str,
}

impl Reference {
    #[must_use]
    /// Creates a new reference
    pub fn new(key: Key, ref_path: Pointer, uri: AbsoluteUri, keyword: &'static str) -> Self {
        Self {
            src_key: SourceKey::default(),
            key,
            ref_path,
            uri,
            keyword,
        }
    }
}
