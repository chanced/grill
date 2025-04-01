use std::borrow::Cow;

use grill_core::{
    Key, schema,
    source::{Source, SourceKey, Sources},
};
use grill_uri::{AbsoluteUri, Uri};

use crate::spec::Specification;

/// A JSON Schema.
pub struct Schema<'int, S>
where
    S: Specification + Send + Sync,
{
    key: Key,
    uris: Cow<'int, [AbsoluteUri]>,
    keywords: Cow<'int, [S::Keyword]>,
    references: Cow<'int, [Reference]>,
    referenced_by: Cow<'int, [Reference]>,
    source: Source<'int>,
    embedded_in: Option<Key>,
    embedded: Cow<'int, [Key]>,
}

impl<'int, S> Schema<'int, S>
where
    S: Specification + Send + Sync,
{
    pub fn key(&self) -> Key {
        self.key
    }
    pub fn primary_uri(&self) -> &AbsoluteUri {
        &self.uris[0]
    }
    pub fn embedded_in(&self) -> Option<Key> {
        self.embedded_in
    }
    pub fn embedded(&self) -> &[Key] {
        self.embedded.as_ref()
    }
    pub fn source(&self) -> Source<'_> {
        self.source.as_borrowed()
    }
    pub fn references(&self) -> &[Reference] {
        self.references.as_ref()
    }
    pub fn referenced_by(&self) -> &[Reference] {
        self.referenced_by.as_ref()
    }
    pub fn uris(&self) -> &[AbsoluteUri] {
        self.uris.as_ref()
    }
    pub fn keywords(&self) -> &[S::Keyword] {
        self.keywords.as_ref()
    }
}
impl<'int, S> schema::Schema<'int> for Schema<'int, S>
where
    S: Specification + Send + Sync,
{
    fn key(&self) -> Key {
        self.key
    }
    fn source(&self) -> Source<'_> {
        self.source.as_borrowed()
    }
}

impl<'int, S> schema::References for Schema<'int, S>
where
    S: Specification + Send + Sync,
{
    type Ref = Reference;

    fn references(&self) -> &[Self::Ref] {
        self.references.as_ref()
    }
}

impl<'int, S> schema::ReferencedBy for Schema<'int, S>
where
    S: Specification + Send + Sync,
{
    type Ref = Reference;

    fn referenced_by(&self) -> &[Self::Ref] {
        self.referenced_by.as_ref()
    }
}

#[derive(Debug)]
pub struct CompiledSchema<S>
where
    S: Specification,
{
    key: Key,
    uris: Vec<AbsoluteUri>,
    keywords: Box<[S::Keyword]>,
    references: Vec<Reference>,
    referenced_by: Vec<Reference>,
    source_key: SourceKey,
    embedded_in: Option<Key>,
    embedded: Vec<Key>,
}

impl<S> CompiledSchema<S>
where
    S: Specification,
{
    pub fn key(&self) -> Key {
        self.key
    }

    pub fn uris(&self) -> &[AbsoluteUri] {
        &self.uris
    }
}

impl<S> schema::References for CompiledSchema<S>
where
    S: Specification,
{
    type Ref = Reference;

    fn references(&self) -> &[Self::Ref] {
        &self.references
    }
}
impl<S> schema::ReferencedBy for CompiledSchema<S>
where
    S: Specification,
{
    type Ref = Reference;

    fn referenced_by(&self) -> &[Self::Ref] {
        &self.referenced_by
    }
}

impl<S> PartialEq for CompiledSchema<S>
where
    S: Specification,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.keywords == other.keywords
    }
}

impl<S> Clone for CompiledSchema<S>
where
    S: Specification,
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

impl<S> schema::CompiledSchema for CompiledSchema<S>
where
    S: Specification,
{
    type Schema<'int>
        = Schema<'int, S>
    where
        S: 'int;

    fn set_key(&mut self, key: Key) {
        self.key = key;
    }

    fn to_schema<'int>(&'int self, sources: &'int Sources) -> Self::Schema<'int> {
        Schema {
            key: self.key,
            uris: Cow::Borrowed(&self.uris),
            keywords: Cow::Borrowed(&self.keywords),
            references: Cow::Borrowed(&self.references),
            source: sources.get(self.source_key),
            referenced_by: Cow::Borrowed(&self.references),
            embedded_in: self.embedded_in,
            embedded: Cow::Borrowed(&self.embedded),
        }
    }

    fn key(&self) -> Key {
        self.key
    }

    fn source_key(&self) -> SourceKey {
        self.source_key
    }
}

/// A reference to another schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// Key to the referenced [`Schema`]
    pub referenced_key: Key,

    /// Key to the referring [`Schema`]
    pub referrer_key: Key,

    /// The URI of the reference, as per the JSON Schema specification
    pub uri: Uri,

    /// The resolved Absolute URI of the reference
    pub absolute_uri: AbsoluteUri,

    /// The keyword of the reference (e.g. $ref, $dynamicRef, $recursiveRef,
    /// etc)
    pub keyword: &'static str,
}

impl schema::Reference for Reference {
    fn reference(&self) -> Key {
        self.referenced_key
    }

    fn referrer(&self) -> Key {
        self.referrer_key
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anchor {
    pub keyword: &'static str,
    pub name: String,
}
