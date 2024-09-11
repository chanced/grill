pub mod dialect;
pub mod keyword;

use std::borrow::Cow;

use grill_core::{
    lang::{
        schema,
        source::{self, Source, SourceKey},
    },
    DefaultKey, Key,
};
use grill_uri::{AbsoluteUri, Uri};

use crate::spec::Specification;

/// A JSON Schema.
pub struct Schema<'int, S, K = DefaultKey>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    key: K,
    uris: Cow<'int, [AbsoluteUri]>,
    keywords: Cow<'int, [S::Keyword]>,
    references: Cow<'int, [Reference<K>]>,
    referenced_by: Cow<'int, [Reference<K>]>,
    source: Source<'int>,
    embedded_in: Option<K>,
    embedded: Cow<'int, [K]>,
}

impl<'int, S, K> Schema<'int, S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    pub fn key(&self) -> K {
        self.key
    }
    pub fn primary_uri(&self) -> &AbsoluteUri {
        &self.uris[0]
    }
    pub fn embedded_in(&self) -> Option<K> {
        self.embedded_in
    }
    pub fn embedded(&self) -> &[K] {
        self.embedded.as_ref()
    }
    pub fn source(&self) -> Source<'_> {
        self.source.as_borrowed()
    }
    pub fn references(&self) -> &[Reference<K>] {
        self.references.as_ref()
    }
    pub fn referenced_by(&self) -> &[Reference<K>] {
        self.referenced_by.as_ref()
    }
    pub fn uris(&self) -> &[AbsoluteUri] {
        self.uris.as_ref()
    }
    pub fn keywords(&self) -> &[S::Keyword] {
        self.keywords.as_ref()
    }
}
impl<'int, S, K> schema::Schema<'int, K> for Schema<'int, S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    fn key(&self) -> K {
        self.key
    }
    fn source(&self) -> Source<'_> {
        self.source.as_borrowed()
    }
}

impl<'int, S, K> schema::References<K> for Schema<'int, S, K>
where
    S: Specification<K> + Send + Sync,
    K: Key + Send + Sync,
{
    type Ref = Reference<K>;

    fn references(&self) -> &[Self::Ref] {
        self.references.as_ref()
    }
}

impl<'int, S, K> schema::ReferencedBy<K> for Schema<'int, S, K>
where
    S: Specification<K> + Send + Sync,
    K: Key + Send + Sync,
{
    type Ref = Reference<K>;

    fn referenced_by(&self) -> &[Self::Ref] {
        self.referenced_by.as_ref()
    }
}

impl<S, K> AsRef<K> for Schema<'_, S, K>
where
    S: Specification<K> + Send + Sync,
    K: Key + Send + Sync,
{
    fn as_ref(&self) -> &K {
        &self.key
    }
}

#[derive(Debug)]
pub struct CompiledSchema<S, K>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    key: K,
    uris: Vec<AbsoluteUri>,
    keywords: Box<[S::Keyword]>,
    references: Vec<Reference<K>>,
    referenced_by: Vec<Reference<K>>,
    source_key: SourceKey,
    embedded_in: Option<K>,
    embedded: Vec<K>,
}

impl<S, K> CompiledSchema<S, K>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    pub fn key(&self) -> K {
        self.key
    }

    pub fn uris(&self) -> &[AbsoluteUri] {
        &self.uris
    }
}

impl<S, K> schema::References<K> for CompiledSchema<S, K>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    type Ref = Reference<K>;

    fn references(&self) -> &[Self::Ref] {
        &self.references
    }
}
impl<S, K> schema::ReferencedBy<K> for CompiledSchema<S, K>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    type Ref = Reference<K>;

    fn referenced_by(&self) -> &[Self::Ref] {
        &self.referenced_by
    }
}
impl<S, K> AsRef<K> for CompiledSchema<S, K>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    fn as_ref(&self) -> &K {
        &self.key
    }
}

impl<S, K> PartialEq for CompiledSchema<S, K>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.keywords == other.keywords
    }
}

impl<S, K> Clone for CompiledSchema<S, K>
where
    S: Specification<K>,
    K: Key + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            uris: self.uris.clone(),
            key: self.key,
            keywords: self.keywords.clone(),
            references: self.references.clone(),
            referenced_by: self.referenced_by.clone(),
            source_key: self.source_key,
            embedded_in: self.embedded_in,
            embedded: self.embedded.clone(),
        }
    }
}

impl<S, K> schema::CompiledSchema<K> for CompiledSchema<S, K>
where
    K: 'static + Key + Send + Sync,
    S: Specification<K>,
{
    type Schema<'int> = Schema<'int, S, K> where S: 'int;

    fn set_key(&mut self, key: K) {
        self.key = key;
    }

    fn to_schema<'int>(&'int self, sources: &'int source::Sources) -> Self::Schema<'int> {
        Schema {
            key: self.key,
            uris: Cow::Borrowed(&self.uris),
            keywords: Cow::Borrowed(&self.keywords),
            references: Cow::Borrowed(&self.references),
            source: sources.source(self.source_key).unwrap(),
            referenced_by: Cow::Borrowed(&self.references),
            embedded_in: self.embedded_in,
            embedded: Cow::Borrowed(&self.embedded),
        }
    }

    fn key(&self) -> K {
        self.key
    }

    fn source_key(&self) -> SourceKey {
        self.source_key
    }
}

/// A reference to another schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference<K> {
    /// Key to the referenced [`Schema`]
    pub key: K,

    /// Key to the referring [`Schema`]
    pub referrer_key: K,

    /// The URI of the reference, as per the JSON Schema specification
    pub uri: Uri,

    /// The resolved Absolute URI
    pub absolute_uri: AbsoluteUri,

    /// The keyword of the reference (e.g. $ref, $dynamicRef, $recursiveRef,
    /// etc)
    pub keyword: &'static str,
}

impl<K> Reference<K> {}

impl<K> schema::Reference<K> for Reference<K>
where
    K: Key,
{
    fn reference(&self) -> K {
        self.key
    }

    fn referrer(&self) -> K {
        self.referrer_key
    }
}
