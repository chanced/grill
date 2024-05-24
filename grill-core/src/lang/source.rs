//! Source repository for JSON Schema documents.

use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use grill_uri::AbsoluteUri;
use jsonptr::Pointer;
use serde_json::Value;
use slotmap::{new_key_type, SecondaryMap, SlotMap};
use snafu::{Backtrace, ResultExt, Snafu};

new_key_type! {
    /// Key to root documents within [`Sources`]
    pub struct DocumentKey;
}
new_key_type! {
    /// Link to a position within a source document, associated to a
    /// specific URI.
    pub struct SourceKey;
}

/// A document within the source repository.
pub struct Document<'i> {
    key: DocumentKey,
    uri: Cow<'i, AbsoluteUri>,
    value: Arc<Value>,
    links: Cow<'i, [Link]>,
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

    /// The value of the document.
    pub fn value(&self) -> Arc<Value> {
        self.value.clone()
    }

    /// The links within the document.
    pub fn links(&self) -> &[Link] {
        &self.links
    }

    /// Consumes `self` and returns an owned, 'static variant.
    pub fn into_owned(self) -> Document<'static> {
        Document {
            key: self.key,
            uri: Cow::Owned(self.uri.into_owned()),
            value: self.value,
            links: Cow::Owned(self.links.into_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A reference to a location within a source
pub struct Source<'i> {
    key: SourceKey,
    link: Cow<'i, Link>,
    /// The value of the source
    document: Arc<Value>,
}

impl<'i> Source<'i> {
    fn new(key: SourceKey, link: &'i Link, document: Arc<Value>) -> Self {
        Self {
            key,
            link: Cow::Borrowed(link),
            document,
        }
    }

    /// The [`SourceKey`] of the source
    pub fn key(&self) -> SourceKey {
        self.key
    }

    /// The [`DocumentKey`] of the root document
    pub fn document_key(&self) -> DocumentKey {
        self.link.key
    }

    /// The `AbsoluteUri` of the root document.
    pub fn uri(&self) -> &AbsoluteUri {
        &self.link.uri
    }

    /// The path of the source, as a JSON [`Pointer`], within the root
    /// document.
    pub fn path(&self) -> &Pointer {
        &self.link.path
    }

    /// Returns the `LinkKey` of the source.
    pub fn link_key(&self) -> SourceKey {
        self.key
    }

    /// Returns a reference to the `Link` of this source.
    pub fn link(&self) -> &Link {
        &self.link
    }

    /// The root document of the source as an `Arc<Value>`. Use
    /// [`document_ref`](Self::document_ref) for a reference.
    pub fn document(&self) -> Arc<Value> {
        self.document.clone()
    }

    /// The root document of the source.
    pub fn document_ref(&self) -> &Value {
        &self.document
    }

    /// Resolves source the path within the document, returning the
    /// [`Value`] at the location.
    pub fn resolve(&self) -> &Value {
        self.link.path.resolve(&self.document).unwrap()
    }

    pub fn into_owned(self) -> Source<'static> {
        Source {
            key: self.key,
            link: Cow::Owned(self.link.into_owned()),
            document: self.document,
        }
    }
}

/// A reference to a [`&Value`](`Value`) within [`Sources`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// The URI of the source
    pub uri: AbsoluteUri,
    /// The key of the root document within the [`Sources`] store
    pub key: DocumentKey,
    /// The path within the document
    pub path: Pointer,
}
impl Link {
    /// Instantiates a new `Link`
    fn new(uri: AbsoluteUri, doc_key: DocumentKey, path: Pointer) -> Self {
        Self {
            uri,
            key: doc_key,
            path,
        }
    }
}

/// A repository of [`Value`]s indexed by [`AbsoluteUri`]s with interior
/// indexing of paths by JSON [`Pointer`]s.
///
/// # Example
/// ```rust
/// # use std::sync::Arc;
/// # use grill_uri::AbsoluteUri;
/// # use jsonptr::Pointer;
/// # use serde_json::json;
///
/// let base_uri = AbsoluteUri::parse("https://example.com").unwrap();
///
/// let path = Pointer::new(["foo", "bar"]);
/// let uri = base_uri.with_fragment(Some(&path)).unwrap());
/// let document = json!({"foo": { "bar": "baz" }});
///
/// let mut sources = Sources::new();
/// let key = sources.insert(base_uri, Arc::new(document)).unwrap();
///
/// let uri = AbsoluteUri::parse("https://example.com/#/foo/bar").unwrap();
/// let source = sources.get(&uri).unwrap();
/// assert_eq!(source.resolve(), &json!("baz"));
/// ```
#[derive(Debug, Default, Clone)]
pub struct Sources {
    docs: SlotMap<DocumentKey, Arc<Value>>,
    links: SlotMap<SourceKey, Link>,
    index: HashMap<AbsoluteUri, SourceKey>,
    doc_links: SecondaryMap<DocumentKey, Vec<SourceKey>>,
}

impl Sources {
    /// Instantiates a new `Sources`
    pub fn new() -> Self {
        Self {
            docs: SlotMap::with_key(),
            links: SlotMap::with_key(),
            index: HashMap::new(),
            doc_links: SecondaryMap::new(),
        }
    }
    /// Inserts new [`Link`] into the store.
    ///
    /// # Example
    /// ```rust
    /// # use std::sync::Arc;
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
    /// let key = sources.insert(base_uri, Arc::new(document)).unwrap();
    ///
    /// // creates a Link from the uri `https://another.example` to the
    /// // value at `/foo/bar` within the document indexed at `"https://example.com"`.
    /// sources.link(uri, Link { key, path }).unwrap();
    ///
    /// let uri = AbsoluteUri::must_parse("https://another.example");
    /// let source = sources.get(&uri).unwrap();
    /// assert_eq!(source.resolve(), &json!("baz"));
    /// ```
    ///
    /// # Errors
    /// Returns [`LinkError`] if:
    /// - The URI is already linked to a different source.
    /// - The JSON pointer of the link cannot be resolved within the source.
    pub fn link(&mut self, link: Link) -> Result<SourceKey, LinkError> {
        match self.index.get(&link.uri) {
            None => self.insert_link(link),
            Some(&existing) => self.handle_duplicate_link(existing, link),
        }
    }

    /// Retrieves a [`Source`] from the store by [`AbsoluteUri`], if a [`Link`]
    /// exists.
    pub fn get<'s>(&'s self, uri: &AbsoluteUri) -> Option<Source<'s>> {
        self.index.get(uri).copied().map(|key| {
            let link = self.links.get(key).unwrap();
            Source::new(key, link, self.docs[link.key].clone())
        })
    }

    /// Retrieves the root document [`Value`] by [`SrcKey`].
    pub fn get_document(&self, key: DocumentKey) -> Option<Arc<Value>> {
        self.docs.get(key).cloned()
    }

    /// Retrieves the associated [`Link`] by [`AbsoluteUri`], if it eists.
    pub fn get_link(&self, uri: &AbsoluteUri) -> Option<&Link> {
        self.index
            .get(uri)
            .copied()
            .map(|k| self.links.get(k).unwrap())
    }

    /// Inserts a new source document for the given **absolute** (meaning it
    /// must not contain a fragment) [`AbsoluteUri`] into the repository and
    /// creates applicable [`Link`]s.
    ///
    /// In the event a source is already indexed at the URI, the document must
    /// be the same as the existing document otherwise an error is returned.
    ///
    /// Upon insertion, a [`Link`] is created for the URI as well as
    /// recursively for the entire document.
    ///
    /// ## Example
    /// ```rust
    /// # use std::sync::Arc;
    /// # use grill_uri::AbsoluteUri;
    /// # use serde_json::json;
    ///
    /// let document = Arc::new(json!({"foo": {"bar": "baz"}}));
    /// let uri = AbsoluteUri::must_parse("https://example.com");
    ///
    /// let mut sources = Sources::new();
    /// let key = sources.insert(uri.clone(), document.clone()).unwrap();
    ///
    /// assert_eq!(&sources.get_document(key), &document);
    /// assert_eq!(sources.get(&uri).unwrap().resolve(), &document);
    ///
    /// let uri = AbsoluteUri::must_parse("https://example.com#/foo");
    /// assert_eq!(sources.get(&uri).unwrap().resolve(), "baz");
    ///
    /// ```
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
    ) -> Result<DocumentKey, InsertError> {
        if absolute_uri.has_non_empty_fragment() {
            return InsertError::fail_not_absolute(absolute_uri, document);
        }
        if self.index.contains_key(&absolute_uri) {
            return self.check_existing(absolute_uri, document);
        }
        let key = self.docs.insert(document.clone());
        for link in build_links(key, &absolute_uri, &document) {
            self.insert_link_skip_check(link);
        }
        Ok(key)
    }

    fn check_existing(
        &self,
        uri: AbsoluteUri,
        value: Arc<Value>,
    ) -> Result<DocumentKey, InsertError> {
        let existing_key = self.index.get(&uri).copied().unwrap();
        let existing = self.links.get(existing_key).unwrap();
        let existing_value = &self.docs[existing.key];
        if value.as_ref() != existing_value.as_ref() {
            InsertError::source_conflict(uri, value, existing.clone(), existing_value.clone())
        } else {
            Ok(existing.key)
        }
    }

    fn insert_link(&mut self, link: Link) -> Result<SourceKey, LinkError> {
        let src = self.docs.get(link.key).unwrap();
        let _ = link
            .path
            .resolve(src)
            .context(ResolutionFailedSnafu)
            .with_context(|_| LinkSnafu { link: link.clone() })?;

        Ok(self.insert_link_skip_check(link))
    }

    fn insert_link_skip_check(&mut self, link: Link) -> SourceKey {
        let uri = link.uri.clone();
        let doc_key = link.key;
        let src_key = self.links.insert(link);
        self.doc_links
            .entry(doc_key)
            .unwrap()
            .or_default()
            .push(src_key);

        self.index.insert(uri, src_key);
        src_key
    }

    fn handle_duplicate_link(
        &self,
        existing_key: SourceKey,
        link: Link,
    ) -> Result<SourceKey, LinkError> {
        let existing = self.links.get(existing_key).unwrap();
        if &link != existing {
            LinkError::fail_confict(link, existing.clone())
        } else {
            Ok(existing_key)
        }
    }
}

/// An error occurred while inserting a source document.
///
/// See [`InsertErrorCause`] for potential causes.
#[derive(Debug, Snafu)]
#[snafu(display("failed to insert source \"{uri}\""))]
pub struct InsertError {
    /// The [`AbsoluteUri`] attempting to be inserted
    pub uri: AbsoluteUri,

    /// The [`Arc<Value>`](`Value`) attempting to be inserted
    pub document: Arc<Value>,

    /// The cause ([`InsertErrorCause`]) of the error
    #[snafu(source)]
    pub cause: InsertErrorCause,

    /// Backtrace of the error
    pub backtrace: Backtrace,
}

impl InsertError {
    /// Returns a `Result<T, Self>::Err(Self)` with an [`InsertErrorCause`]
    /// of [`NotAbsolute`](InsertErrorCause::NotAbsolute).
    pub fn fail_not_absolute<T>(uri: AbsoluteUri, document: Arc<Value>) -> Result<T, Self> {
        NotAbsoluteSnafu
            .fail()
            .with_context(|_| InsertSnafu { uri, document })
    }

    /// Returns an `Result<T, Self>::Err(Self)` with an [`InertErrorCause`]
    /// of [`SourceConflict`](InsertErrorCause::SourceConflict).
    pub fn source_conflict<T>(
        uri: AbsoluteUri,
        document: Arc<Value>,
        existing_link: Link,
        existing_value: Arc<Value>,
    ) -> Result<T, Self> {
        SourceConflictSnafu {
            existing_link,
            existing_value,
        }
        .fail()
        .with_context(|_| InsertSnafu { uri, document })
    }
}

/// Cause of an [`InsertError`].
#[derive(Debug, Snafu)]
pub enum InsertErrorCause {
    /// The [`AbsoluteUri`] provided contained a fragment. Only root documents can be inserted.
    #[snafu(display("source URI must be absolute but contains a fragment",))]
    NotAbsolute,

    /// A source document was attempted to be inserted at an [`AbsoluteUri`]
    /// that is already indexed to a different source document.
    #[snafu(display("URI is indexed to a different source"))]
    SourceConflict {
        /// The existing [`Link`] associated with the URI
        existing_link: Link,
        /// The existing [`Value`] associated with the URI
        existing_value: Arc<Value>,
    },
}
/// An error occurred while inserting a [`Link`].
///
/// See [`LinkErrorCause`] for potential causes of this error.
#[derive(Debug, Snafu)]
#[snafu(display("failed to link source \"{}\"", link.uri))]
pub struct LinkError {
    /// The [`Link`] attempting to be inserted
    pub link: Link,

    /// The [`cause`](LinkErrorCause) of the error
    #[snafu(source)]
    pub cause: LinkErrorCause,

    /// Backtrace of the error
    pub backtrace: Backtrace,
}

impl LinkError {
    /// Returns an `Result<T, Self>::Err(Self)` with a [`LinkErrorCause`]
    /// of [`Conflict`](LinkErrorCause::Conflict).
    pub fn fail_confict<T>(link: Link, existing: Link) -> Result<T, Self> {
        ConflictSnafu { existing }
            .fail()
            .with_context(|_| LinkSnafu { link })
    }
}

/// Underlying cause of a [`LinkError`].
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum LinkErrorCause {
    /// The JSON pointer of the [`Link`] could not be resolved within the source.
    #[snafu(display("failed to resolve JSON pointer of sourced document: {source}"))]
    ResolutionFailed {
        /// Error encountered by while attempting to resolve the json pointer
        source: jsonptr::Error,
    },

    /// The URI is already linked to a different source.
    #[snafu(display("URI is already linked to a different source."))]
    Conflict {
        /// The existing [`Link`] associated with the URI
        existing: Link,
    },
}

/// An error occurred while sourcing a value.
#[derive(Debug, Snafu)]
pub enum SourceError {
    /// An error occurred while inserting a source.
    #[snafu(transparent)]
    Insert {
        /// source of the error
        #[snafu(backtrace)]
        source: InsertError,
    },
    /// An error occurred while linking a source.
    #[snafu(transparent)]
    Link {
        /// source of the error
        #[snafu(backtrace)]
        source: LinkError,
    },
}
/// A source was not found at the given URI.
#[derive(Debug, Snafu)]
#[snafu(display("source not found: {}", uri))]
pub struct NotFoundError {
    /// The URI that was not found.
    pub uri: AbsoluteUri,
    /// The backtrace.
    pub backtrace: Backtrace,
}
impl NotFoundError {
    /// Returns a new `Result<T, Self>::Err(Self)` with the given URI.
    pub fn new(uri: AbsoluteUri) -> Self {
        NotFoundSnafu { uri }.build()
    }
}

fn build_links<'i>(
    doc_key: DocumentKey,
    base_uri: &'i AbsoluteUri,
    document: &'i Value,
) -> BuildLinks<'i> {
    BuildLinks::new(doc_key, base_uri, document)
}
struct BuildLinks<'i> {
    doc_key: DocumentKey,
    base_uri: &'i AbsoluteUri,
    path_finder: FindPaths<'i>,
}
impl<'i> BuildLinks<'i> {
    fn new(doc_key: DocumentKey, base_uri: &'i AbsoluteUri, document: &'i Value) -> Self {
        let path_finder = FindPaths::new(document);
        Self {
            doc_key,
            base_uri,
            path_finder,
        }
    }
}
impl Iterator for BuildLinks<'_> {
    type Item = Link;
    fn next(&mut self) -> Option<Self::Item> {
        let path = self.path_finder.next()?;
        let uri = self.base_uri.with_fragment(Some(&path)).unwrap();
        Some(Link::new(uri, self.doc_key, path))
    }
}

struct FindPaths<'v> {
    queue: VecDeque<(Pointer, &'v Value)>,
}
impl<'v> FindPaths<'v> {
    fn new(value: &'v Value) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back((Pointer::default(), value));
        Self { queue }
    }
}
impl<'v> Iterator for FindPaths<'v> {
    type Item = Pointer;
    fn next(&mut self) -> Option<Self::Item> {
        let (path, value) = self.queue.pop_front()?;
        match value {
            Value::Object(map) => {
                for (key, value) in map.iter().rev() {
                    let mut ptr = path.clone();
                    ptr.push_back(key.into());
                    self.queue.push_back((ptr, value));
                }
            }
            Value::Array(array) => {
                for (i, value) in array.iter().enumerate().rev() {
                    let mut ptr = path.clone();
                    ptr.push_back(i.into());
                    self.queue.push_back((ptr, value));
                }
            }
            _ => {}
        }
        Some(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_build_links() {
        let document = json!({
            "foo": {
                "string": "string",
                "number": 1,
                "array": [
                    { "key": "0" },
                    { "key": "1" },
                    { "key": "2" }
                ],
                "object": {
                    "key": "value"
                }
            },
            "bar": {},
            "baz": {}

        });
        let document = Arc::new(document);
        let base_uri = AbsoluteUri::parse("https://example.com").unwrap();
        let key = DocumentKey::default();
        let links = build_links(key, &base_uri, &document).collect::<Vec<_>>();
        let paths: Vec<_> = links.iter().map(|l| l.path.to_string()).collect();

        assert_eq!(
            &paths,
            &[
                "",
                "/foo",
                "/bar",
                "/baz",
                "/foo/string",
                "/foo/number",
                "/foo/array",
                "/foo/object",
                "/foo/array/0",
                "/foo/array/1",
                "/foo/array/2",
                "/foo/object/key"
            ]
        )
    }

    #[test]
    fn test_link() {
        let document = json!({"foo": { "bar": "baz" }});
        let base_uri = AbsoluteUri::parse("https://example.com").unwrap();
        let path = Pointer::new(["foo", "bar"]);
        let uri = base_uri.with_fragment(Some(&path)).unwrap();

        let mut sources = Sources::new();
        // Insert the root document at the base uri
        let key = sources
            .insert(base_uri.clone(), Arc::new(document))
            .unwrap();

        // creates a Link from the uri `https://example.com#/foo/bar` to the
        // value at the path (as a JSON Pointer) `/foo/bar` within the document.
        sources.link(Link { uri, key, path }).unwrap();

        let uri = AbsoluteUri::parse("https://example.com/#/foo/bar").unwrap();
        let source = sources.get(&uri).unwrap();
        assert_eq!(source.resolve(), &json!("baz"));
        let invalid_path_uri = base_uri
            .with_fragment(Some(&Pointer::new(["bad", "path"])))
            .unwrap();
        assert_eq!(sources.get(&invalid_path_uri), None)
    }
}
