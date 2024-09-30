//! Source repository for JSON Schema documents.

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{self, Debug},
    iter::FusedIterator,
    sync::Arc,
};

use grill_uri::AbsoluteUri;
use jsonptr::{Pointer, PointerBuf, Resolve as _};
use serde_json::Value;
use slotmap::{new_key_type, SecondaryMap, SlotMap};
use walk::WalkValue;

pub mod walk;

new_key_type! {
    /// Key to root documents within [`Sources`]
    pub struct DocumentKey;
    /// Link to a position within a source document, associated to a specific
    /// URI.
    pub struct SourceKey;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Document                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A document within the source repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document<'int> {
    key: DocumentKey,
    uri: Cow<'int, AbsoluteUri>,
    value: Arc<Value>,
    links: Cow<'int, [InternalLink]>,
    indexed: bool,
}

impl Document<'_> {
    /// The key of the document within the source repository.
    pub fn key(&self) -> DocumentKey {
        self.key
    }

    /// The URI of the document.
    pub fn uri(&self) -> &AbsoluteUri {
        &self.uri
    }

    /// The [`Value`] of the document as a reference.
    pub fn value_ref(&self) -> &Value {
        &self.value
    }

    /// The [`Value`] of the document as an `Arc<Value>`.
    pub fn value_arc(&self) -> Arc<Value> {
        self.value.clone()
    }

    /// The links within the document.
    pub fn links(&self) -> Links {
        Links {
            iter: self.links.iter(),
        }
    }

    /// Consumes `self` and returns an owned, 'static variant.
    pub fn into_owned(self) -> Document<'static> {
        Document {
            key: self.key,
            uri: Cow::Owned(self.uri.into_owned()),
            value: self.value.clone(),
            links: Cow::Owned(self.links.into_owned()),
            indexed: self.indexed,
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Source                                    ║
║                                   ¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A document within the source repository.
#[derive(Debug, Clone, PartialEq, Eq)]
/// A reference to a location within a source
pub struct Source<'int> {
    key: SourceKey,
    link: Cow<'int, InternalLink>,
    /// The value of the source
    document: Document<'int>,
}

impl<'int> Source<'int> {
    pub fn as_borrowed(&self) -> Source<'_> {
        Source {
            key: self.key,
            link: Cow::Borrowed(&self.link),
            document: self.document.clone(),
        }
    }

    /// The [`SourceKey`] of the source
    pub fn key(&self) -> SourceKey {
        self.key
    }

    /// The [`DocumentKey`] of the root document
    pub fn document_key(&self) -> DocumentKey {
        self.link.document_key
    }

    /// The `AbsoluteUri` of the root document.
    pub fn uri(&self) -> &AbsoluteUri {
        &self.link.uri
    }

    /// The absolute path of the source, as a JSON [`Pointer`], within the root
    /// document.
    pub fn absolute_path(&self) -> &Pointer {
        &self.link.absolute_path
    }

    pub fn fragment(&self) -> Option<&Fragment> {
        self.link.fragment.as_ref()
    }

    /// Returns the `LinkKey` of the source.
    pub fn link_key(&self) -> SourceKey {
        self.key
    }

    /// Returns a reference to the `Link` of this source.
    pub fn link(&self) -> Link {
        self.link.as_link()
    }

    pub fn document(&self) -> &Document<'int> {
        &self.document
    }

    pub fn take_document(self) -> Document<'int> {
        self.document
    }

    /// Resolves source the path within the document, returning the
    /// [`Value`] at the location.
    pub fn resolve(&self) -> &Value {
        self.link
            .absolute_path
            .resolve(&*self.document.value)
            .unwrap()
    }

    /// Consumes this `Source`` and returns an owned, `'static` variant.
    pub fn into_owned(self) -> Source<'static> {
        Source {
            key: self.key,
            link: Cow::Owned(self.link.into_owned()),
            document: self.document.into_owned(),
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                     Link                                     ║
║                                    ¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A reference to a [`&Value`](`Value`) within [`Sources`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link<'s> {
    /// The key of the root document within the [`Sources`] store
    pub document_key: DocumentKey,
    /// The key of this link
    pub source_key: SourceKey,
    /// The URI of the source
    pub uri: Cow<'s, AbsoluteUri>,
    /// The path within the document
    pub path: Cow<'s, Pointer>,
}

impl<'s> Link<'s> {
    fn from_internal(link: &'s InternalLink) -> Self {
        Self {
            document_key: link.document_key,
            source_key: link.source_key,
            uri: Cow::Borrowed(&link.uri),
            path: Cow::Borrowed(&link.absolute_path),
        }
    }
    pub fn into_owned(self) -> Link<'static> {
        Link {
            document_key: self.document_key,
            source_key: self.source_key,
            uri: Cow::Owned(self.uri.into_owned()),
            path: Cow::Owned(self.path.into_owned()),
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Links                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

pub struct Links<'s> {
    iter: std::slice::Iter<'s, InternalLink>,
}
impl<'s> Iterator for Links<'s> {
    type Item = Link<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Link::from_internal)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'s> ExactSizeIterator for Links<'s> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl FusedIterator for Links<'_> {}

impl DoubleEndedIterator for Links<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Link::from_internal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fragment {
    Anchor(String),
    Pointer(PointerBuf),
}

impl From<String> for Fragment {
    fn from(v: String) -> Self {
        Self::Anchor(v)
    }
}

impl From<PointerBuf> for Fragment {
    fn from(v: PointerBuf) -> Self {
        Self::Pointer(v)
    }
}

impl Fragment {
    /// Returns `true` if the fragment is [`Anchor`].
    ///
    /// [`Anchor`]: Fragment::Anchor
    #[must_use]
    pub fn is_anchor(&self) -> bool {
        matches!(self, Self::Anchor(..))
    }

    pub fn as_anchor(&self) -> Option<&String> {
        if let Self::Anchor(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_anchor(self) -> Result<String, Self> {
        if let Self::Anchor(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the fragment is [`Pointer`].
    ///
    /// [`Pointer`]: Fragment::Pointer
    #[must_use]
    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::Pointer(..))
    }

    pub fn as_pointer(&self) -> Option<&Pointer> {
        if let Self::Pointer(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_pointer(self) -> Result<PointerBuf, Self> {
        if let Self::Pointer(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 InternalLink                                 ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug, Clone, PartialEq, Eq)]
struct InternalLink {
    uri: AbsoluteUri,
    document_key: DocumentKey,
    source_key: SourceKey,
    /// absolute path of the value from the document root
    absolute_path: PointerBuf,
    fragment: Option<Fragment>,
}

impl InternalLink {
    fn as_link(&self) -> Link {
        Link {
            uri: Cow::Borrowed(&self.uri),
            document_key: self.document_key,
            path: Cow::Borrowed(&self.absolute_path),
            source_key: self.source_key,
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Sources                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A repository of [`Value`]s indexed by [`AbsoluteUri`]s with interior
/// indexing of paths by JSON [`Pointer`]s.
///
/// # Example:
/// ```rust
/// # use grill_uri::AbsoluteUri;
/// # use jsonptr::Pointer;
/// # use serde_json::json;
/// # use grill_core::lang::source::{Sources, Link};
///
/// let uri = AbsoluteUri::parse("https://example.com").unwrap();
/// let document = json!({"foo": { "bar": "baz" }});
///
/// let mut sources = Sources::new();
/// let key = sources.insert(uri.clone(), document.clone()).unwrap();
/// let source = sources.get(&uri).unwrap();
/// assert_eq!(source.resolve(), &document);
///
/// let path = Pointer::from_static("/foo/bar");
/// let uri = uri.with_fragment(Some(path.as_str())).unwrap();
/// source.link(uri.clone(), key, path).unwrap();
/// let source = sources.get(&uri).unwrap();
/// assert_eq!(source.resolve(), &document);
///
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct Sources {
    values: SlotMap<DocumentKey, Arc<Value>>,
    links: SlotMap<SourceKey, InternalLink>,
    src_keys: HashMap<AbsoluteUri, SourceKey>,
    doc_links: SecondaryMap<DocumentKey, Vec<InternalLink>>,
    doc_uris: SecondaryMap<DocumentKey, AbsoluteUri>,
    indexed: SecondaryMap<DocumentKey, ()>,
}
pub struct New {
    pub uri: AbsoluteUri,
    pub fragment: Option<Fragment>,
    pub document_key: DocumentKey,
    pub absolute_path: PointerBuf,
}

impl Sources {
    /// Instantiates a new `Sources`
    pub fn new() -> Self {
        Self {
            values: SlotMap::with_key(),
            links: SlotMap::with_key(),
            src_keys: HashMap::new(),
            doc_links: SecondaryMap::new(),
            doc_uris: SecondaryMap::new(),
            indexed: SecondaryMap::new(),
        }
    }
    /// Inserts new [`Link`] into the store.
    ///
    /// # Example
    /// ```rust
    /// # use grill_uri::AbsoluteUri;
    /// # use jsonptr::Pointer;
    /// # use serde_json::json;
    /// use grill_core::lang::source::{Sources, Link};
    ///
    /// let document = json!({"foo": { "bar": "baz" }});
    /// let base_uri = AbsoluteUri::must_parse("https://example.com");
    /// let path = Pointer::new(["foo", "bar"]);
    /// let uri = base_uri.with_fragment(Some(&path)).unwrap();
    ///
    /// let mut sources = Sources::new();
    /// // Insert the root document at the base uri
    /// let key = sources.insert(base_uri, document).unwrap();
    ///
    ///
    /// let uri = AbsoluteUri::must_parse("https://another.example");
    /// // creates a Link from the uri `https://another.example` to the
    /// // value at `/foo/bar` within the document indexed at `"https://example.com"`.
    /// sources.link(uri.clone(), Link { key, path }).unwrap();
    ///
    /// let source = sources.get(&uri).unwrap();
    /// assert_eq!(source.resolve(), &json!("baz"));
    /// ```
    ///
    /// # Errors
    /// Returns [`LinkError`] if:
    /// - The URI is already linked to a different source.
    /// - The JSON pointer of the link cannot be resolved within the source.
    pub fn link(&mut self, new: New) -> Result<SourceKey, LinkError> {
        let link = InternalLink {
            document_key: new.document_key,
            uri: new.uri,
            absolute_path: new.absolute_path,
            fragment: new.fragment,
            source_key: SourceKey::default(), // will be updated
        };
        match self.src_keys.get(&link.uri) {
            None => self.insert_link(link),
            Some(&existing) => self.handle_duplicate_link(existing, link),
        }
    }

    pub fn document(&self, key: DocumentKey) -> Option<Document> {
        Some(Document {
            key,
            uri: Cow::Borrowed(self.doc_uris.get(key)?),
            value: self.values.get(key)?.clone(),
            links: Cow::Borrowed(self.doc_links.get(key)?.as_slice()),
            indexed: self.indexed.contains_key(key),
        })
    }

    pub fn value(&self, key: SourceKey) -> Option<&Value> {
        self.links
            .get(key)
            .map(|link| self.values[link.document_key].as_ref())
    }

    pub fn source(&self, key: SourceKey) -> Source<'_> {
        let link = Cow::Borrowed(self.links.get(key).unwrap());
        let document = self.document(link.document_key).unwrap();
        Source {
            key,
            link,
            document,
        }
    }
    /// Retrieves a [`Source`] from the store by [`AbsoluteUri`], if a [`Link`]
    /// exists.
    pub fn source_by_uri<'s>(&'s self, uri: &AbsoluteUri) -> Option<Source<'s>> {
        self.source_key_by_uri(uri).map(|key| self.source(key))
    }

    /// Retrieves a [`SourceKey`] from the store by [`AbsoluteUri`], if a [`Link`]
    /// exists.
    pub fn source_key_by_uri(&self, uri: &AbsoluteUri) -> Option<SourceKey> {
        self.src_keys.get(uri).copied()
    }

    pub fn uris_for_source(&self, key: SourceKey) -> impl Iterator<Item = &AbsoluteUri> {
        self.src_keys
            .iter()
            .filter_map(move |(uri, &k)| if k == key { Some(uri) } else { None })
    }

    /// Retrieves the root document [`Value`] by [`SrcKey`].
    pub fn document_value(&self, key: DocumentKey) -> &Value {
        self.values.get(key).unwrap().as_ref()
    }

    pub fn document_key_for(&self, src_key: SourceKey) -> DocumentKey {
        self.links.get(src_key).unwrap().document_key
    }
    /// Inserts a new source document for the given **absolute** (meaning it
    /// must not contain a fragment) [`AbsoluteUri`] into the repository,
    /// creating and returning a [`Link`] to the document.
    ///
    /// In the event a source is already indexed at the URI, the document must
    /// be the same as the existing document otherwise an error is returned.
    ///
    /// ## Errors
    /// Returns [`InsertError`] if:
    /// - If the URI contains a JSON pointer fragment (e.g.
    ///   `https://example.com#/foo/bar`)
    /// - If the URI is already indexed to a different value
    pub fn insert(
        &mut self,
        absolute_uri: AbsoluteUri,
        document: Arc<Value>,
    ) -> Result<SourceKey, InsertError> {
        if absolute_uri.has_non_empty_fragment() {
            InsertError::err_with(|| {
                NotAbsoluteError::new(absolute_uri.clone(), document.clone())
            })?;
        }
        if self.src_keys.contains_key(&absolute_uri) {
            Ok(self.check_existing(absolute_uri, &document)?)
        } else {
            Ok(self.insert_document(absolute_uri, document))
        }
    }

    fn insert_document(&mut self, mut uri: AbsoluteUri, document: Arc<Value>) -> SourceKey {
        let without_fragment = uri.without_fragment();
        uri.set_fragment(Some("")).unwrap();
        let with_fragment = uri;
        let doc_key = self.values.insert(document.clone());
        let source_key = self
            .insert_link(InternalLink {
                document_key: doc_key,
                uri: without_fragment,
                fragment: None,
                absolute_path: PointerBuf::new(),
                source_key: SourceKey::default(),
            })
            .unwrap();
        // creating a second link with an empty fragment
        // eg `https://example.com` -> `https://example.com#`
        self.insert_link(InternalLink {
            document_key: doc_key,
            uri: with_fragment,
            fragment: Some(Fragment::Anchor(String::from(""))),
            absolute_path: PointerBuf::new(),
            source_key: SourceKey::default(),
        })
        .unwrap();
        source_key
    }

    pub fn index_document<F, O, E>(&mut self, document_key: DocumentKey, f: F) -> Result<(), E>
    where
        F: for<'v> Fn(SourceKey, PointerBuf, &'v Value) -> Result<O, E>,
        E: From<LinkError>,
    {
        if self.indexed.contains_key(document_key) {
            return Ok(());
        }
        let doc_uri = self.doc_uris.get(document_key).cloned().unwrap();
        let value = self.values.get(document_key).unwrap().clone();
        for (path, value) in WalkValue::new(PointerBuf::new(), &value) {
            let uri = doc_uri.with_fragment(path.as_str()).unwrap();
            let source_key = self.link(New {
                uri,
                absolute_path: path.clone(),
                document_key,
                fragment: Some(Fragment::Pointer(path.clone())),
            })?;
            f(source_key, path, value)?;
        }
        Ok(())
    }

    fn check_existing(&self, uri: AbsoluteUri, value: &Value) -> Result<SourceKey, InsertError> {
        let existing_src_key = self.src_keys.get(&uri).copied().unwrap();
        let existing_doc_key = self.document_key_for(existing_src_key);
        let existing_value = &self.values[existing_doc_key];
        if value != existing_value.as_ref() {
            return SourceConflictError::err_with(|| SourceConflictError {
                uri: uri.clone(),
                value: Box::new(value.clone()),
                existing_link: self.links[existing_src_key].as_link().into_owned(),
                existing_value: existing_value.clone(),
            });
        }
        Ok(existing_src_key)
    }

    fn insert_link(&mut self, link: InternalLink) -> Result<SourceKey, LinkError> {
        let src = self.values.get(link.document_key).unwrap();
        src.resolve(&link.absolute_path)
            .map_err(|source| InvalidLinkPathError::new(source, link.as_link()))?;

        let uri = link.uri.clone();

        let doc_key = link.document_key;
        let src_key = self.links.insert(link.clone());
        self.links.get_mut(src_key).unwrap().source_key = src_key;
        self.doc_links
            .entry(doc_key)
            .unwrap()
            .or_default()
            .push(link);
        self.doc_uris.entry(doc_key).unwrap().or_insert(uri.clone());
        self.src_keys.insert(uri, src_key);
        Ok(src_key)
    }

    fn handle_duplicate_link(
        &self,
        existing_key: SourceKey,
        link: InternalLink,
    ) -> Result<SourceKey, LinkError> {
        let existing_link = self.links.get(existing_key).unwrap();
        if &link != existing_link {
            SourceConflictError::err_with(|| SourceConflictError {
                uri: link.uri.clone(),
                value: Box::new(link.absolute_path.to_string().into()),
                existing_link: existing_link.as_link().into_owned(),
                existing_value: self.values[existing_link.document_key].clone(),
            })
        } else {
            Ok(existing_link.source_key)
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 InsertError                                  ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// An error occurred while inserting a source document.
///
/// See [`InsertErrorCause`] for potential causes.
#[derive(Debug)]
pub enum InsertError {
    /// The URI provided contained a fragment. Only root documents can be inserted.
    NotAbsolute(NotAbsoluteError),
    /// The URI is already linked to a different source.
    SourceConflict(SourceConflictError),
}
impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to insert source \"{}\"", self.uri())
    }
}
impl From<NotAbsoluteError> for InsertError {
    fn from(e: NotAbsoluteError) -> Self {
        Self::NotAbsolute(e)
    }
}
impl From<SourceConflictError> for InsertError {
    fn from(e: SourceConflictError) -> Self {
        Self::SourceConflict(e)
    }
}

impl InsertError {
    /// The [`AbsoluteUri`] attempting to be inserted
    pub fn uri(&self) -> &AbsoluteUri {
        match self {
            Self::NotAbsolute(e) => &e.uri,
            Self::SourceConflict(e) => &e.uri,
        }
    }

    fn err_with<F, T, X>(f: F) -> Result<T, Self>
    where
        F: FnOnce() -> X,
        X: Into<Self>,
    {
        Err(f().into())
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                               NotAbsoluteError                               ║
║                              ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug, PartialEq)]
pub struct NotAbsoluteError {
    /// The [`AbsoluteUri`] attempting to be inserted
    pub uri: AbsoluteUri,

    /// The [`Value`] attempting to be inserted
    pub document: Arc<Value>,
}

impl NotAbsoluteError {
    /// Returns a new `Result<T, Self>::Err(Self)` with the given URI and document.
    #[must_use]
    pub fn new(uri: AbsoluteUri, document: Arc<Value>) -> Self {
        Self { uri, document }
    }

    /// Returns a new `Result<T, Self>::Err(Self)` with the given URI and document.
    pub fn err_with<F, T, E>(f: F) -> Result<T, E>
    where
        F: FnOnce() -> Self,
        E: From<Self>,
    {
        Err(f().into())
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  LinkError                                   ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// An error occurred while inserting a [`Link`].
///
/// See [`LinkErrorCause`] for potential causes of this error.
#[derive(Debug, PartialEq)]
pub enum LinkError {
    InvalidPath(Box<InvalidLinkPathError>),
    /// A source document was attempted to be inserted at an [`AbsoluteUri`]
    /// that is already indexed to a different source document.
    SourceConflict(Box<SourceConflictError>),
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath(e) => fmt::Display::fmt(e, f),
            Self::SourceConflict(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for LinkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidPath(e) => Some(e),
            Self::SourceConflict(e) => Some(e),
        }
    }
}

impl From<InvalidLinkPathError> for LinkError {
    fn from(e: InvalidLinkPathError) -> Self {
        Self::InvalidPath(Box::new(e))
    }
}

impl From<SourceConflictError> for LinkError {
    fn from(e: SourceConflictError) -> Self {
        Self::SourceConflict(Box::new(e))
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                             InvalidLinkPathError                             ║
║                            ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// The JSON pointer of the [`Link`] could not be resolved within the source.
#[derive(Debug, PartialEq)]
pub struct InvalidLinkPathError {
    /// The [`Link`] attempting to be inserted
    link: Link<'static>,
    /// Error encountered by while attempting to resolve the json pointer
    source: jsonptr::resolve::ResolveError,
}
impl std::error::Error for InvalidLinkPathError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl InvalidLinkPathError {
    pub fn new(source: jsonptr::resolve::ResolveError, link: Link<'_>) -> Self {
        Self {
            source,
            link: link.into_owned(),
        }
    }
    pub fn err_with<F, T, E>(f: F) -> Result<T, E>
    where
        F: Fn() -> Self,
        E: From<Self>,
    {
        Err(f().into())
    }
    pub fn source(&self) -> &jsonptr::resolve::ResolveError {
        &self.source
    }
    pub fn link(&self) -> &Link {
        &self.link
    }
}

impl fmt::Display for InvalidLinkPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to resolve JSON pointer of sourced document: {}",
            self.source
        )
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                            SourceConflictError                               ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug, PartialEq)]
pub struct SourceConflictError {
    /// The URI that was attempted to be inserted
    uri: AbsoluteUri,

    /// The value that was attempted to be inserted
    value: Box<Value>,

    /// The existing [`Link`] associated with the URI
    existing_link: Link<'static>,

    /// The existing [`Value`] associated with the URI
    existing_value: Arc<Value>,
}

impl std::error::Error for SourceConflictError {}

impl fmt::Display for SourceConflictError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "source conflict: URI \"{}\" is already linked to a different source.",
            self.uri
        )
    }
}

impl SourceConflictError {
    /// Returns a new `Result<T, Self>::Err(Self)` with the given URI, value, existing link, and existing value.
    pub fn err_with<F, R, T, E>(f: F) -> R
    where
        F: Fn() -> Self,
        E: From<Self>,
        R: From<Result<T, E>>,
    {
        Err(f().into()).into()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    tests                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // #[test]
    // fn test_build_links() {
    //     let document = json!({
    //         "foo": {
    //             "string": "string",
    //             "number": 1,
    //             "array": [
    //                 { "key": "0" },
    //                 { "key": "1" },
    //                 { "key": "2" }
    //             ],
    //             "object": {
    //                 "key": "value"
    //             }
    //         },
    //         "bar": {},
    //         "baz": {}

    //     });
    //     let document = Arc::new(document);
    //     let base_uri = AbsoluteUri::parse("https://example.com").unwrap();
    //     let key = DocumentKey::default();
    //     let links = build_links(key, &base_uri, &document).collect::<Vec<_>>();
    //     let mut paths: Vec<_> = links.iter().map(|l| l.path.to_string()).collect();
    //     paths.sort();
    //     assert_eq!(
    //         &paths,
    //         &[
    //             "",
    //             "/bar",
    //             "/baz",
    //             "/foo",
    //             "/foo/array",
    //             "/foo/array/0",
    //             "/foo/array/0/key",
    //             "/foo/array/1",
    //             "/foo/array/1/key",
    //             "/foo/array/2",
    //             "/foo/array/2/key",
    //             "/foo/number",
    //             "/foo/object",
    //             "/foo/object/key",
    //             "/foo/string"
    //         ]
    //     )
    // }

    #[test]
    fn test_link() {
        let document = Arc::new(json!({"foo": { "bar": "baz" }}));
        let base_uri = AbsoluteUri::parse("https://example.com").unwrap();
        let absolute_path = PointerBuf::from_tokens(["foo", "bar"]);
        let uri = base_uri.with_fragment(absolute_path.as_str()).unwrap();

        let mut sources = Sources::new();
        // Insert the root document at the base uri
        let source_key = sources.insert(base_uri.clone(), document).unwrap();
        let source = sources.source(source_key);
        // creates a Link from the uri `https://example.com#/foo/bar` to the
        // value at the path (as a JSON Pointer) `/foo/bar` within the document.
        sources
            .link(New {
                uri,
                document_key: source.document_key(),
                absolute_path,
                fragment: Some(Fragment::Pointer("/foo/bar".parse().unwrap())),
            })
            .unwrap();

        let uri = AbsoluteUri::parse("https://example.com/#/foo/bar").unwrap();
        let source = sources.source_by_uri(&uri).unwrap();
        assert_eq!(source.resolve(), &json!("baz"));
        let invalid_path_uri = base_uri.with_fragment("/bad/path").unwrap();
        assert_eq!(sources.source_by_uri(&invalid_path_uri), None)
    }
}
