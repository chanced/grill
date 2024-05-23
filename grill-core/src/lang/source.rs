//! Source repository for JSON Schema documents.

use std::{borrow::Cow, collections::HashMap, sync::Arc};

use grill_uri::AbsoluteUri;
use jsonptr::Pointer;
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};
use snafu::{Backtrace, ResultExt, Snafu};

new_key_type! {
/// Key to root documents within [`Sources`]
pub struct SrcKey; }

#[derive(Debug, Clone, PartialEq, Eq)]
/// A reference to a location within a source
pub struct Source<'i> {
    key: SrcKey,
    /// The URI of the source
    uri: Cow<'i, AbsoluteUri>,
    /// The path within the source
    path: Cow<'i, Pointer>,
    /// The value of the source
    document: Arc<Value>,
}
impl Source<'_> {
    /// The [`SrcKey`] of the root document
    pub fn key(&self) -> SrcKey {
        self.key
    }
    /// The `AbsoluteUri` of the root document.
    pub fn uri(&self) -> &AbsoluteUri {
        self.uri.as_ref()
    }
    /// The path of the source, as a JSON [`Pointer`], within the root
    /// document.
    pub fn path(&self) -> &Pointer {
        self.path.as_ref()
    }
    /// The root document of the source as an `Arc<Value>`. Use
    /// [`document_ref`](Self::document_ref) for a reference.
    pub fn document(&self) -> Arc<Value> {
        self.document.clone()
    }
    /// The root document of the source.
    pub fn document_ref(&self) -> &Value {
        self.document.as_ref()
    }
    /// Resolves source the path within the document, returning the
    /// [`Value`] at the location.
    pub fn resolve(&self) -> &Value {
        self.path.resolve(&self.document).unwrap()
    }
    /// Consumes the source, returning an owned version.
    pub fn into_owned(self) -> Source<'static> {
        Source {
            key: self.key,
            uri: Cow::Owned(self.uri.into_owned()),
            path: Cow::Owned(self.path.into_owned()),
            document: self.document,
        }
    }
}

/// A reference to a [`&Value`](`Value`) within [`Sources`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// The key of the root document within the [`Sources`] store
    pub key: SrcKey,
    /// The path within the document
    pub path: Pointer,
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
/// sources.link(uri, Link { key, path }).unwrap();
///
/// let uri = AbsoluteUri::parse("https://example.com/#/foo/bar").unwrap();
/// let source = sources.get(&uri).unwrap();
/// assert_eq!(source.resolve(), &json!("baz"));
/// ```
#[derive(Debug, Default, Clone)]
pub struct Sources {
    docs: SlotMap<SrcKey, Arc<Value>>,
    links: HashMap<AbsoluteUri, Link>,
}

impl Sources {
    /// Instantiates a new `Sources`
    pub fn new() -> Self {
        Self {
            docs: SlotMap::with_key(),
            links: HashMap::new(),
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
    /// let document = json!({"foo": { "bar": "baz" }});
    /// let base_uri = AbsoluteUri::parse("https://example.com").unwrap();
    /// let path = Pointer::new(["foo", "bar"]);
    /// let uri = base_uri.with_fragment(Some(&path)).unwrap();
    ///
    /// let mut sources = Sources::new();
    /// // Insert the root document at the base uri
    /// let key = sources.insert(base_uri, Arc::new(document)).unwrap();
    ///
    /// // creates a Link from the uri `https://example.com#/foo/bar` to the
    /// // value at the path (as a JSON Pointer) `/foo/bar` within the document.
    /// sources.link(uri, Link { key, path }).unwrap();
    ///
    /// let uri = AbsoluteUri::parse("https://example.com/#/foo/bar").unwrap();
    /// let source = sources.get(&uri).unwrap();
    /// assert_eq!(source.resolve(), &json!("baz"));
    /// ```
    ///
    /// # Errors
    /// Returns [`LinkError`] if:
    /// - The URI is already linked to a different source.
    /// - The JSON pointer of the link cannot be resolved within the source.
    pub fn link(&mut self, uri: AbsoluteUri, link: Link) -> Result<(), LinkError> {
        match self.links.get(&uri) {
            None => self.insert_link(uri, link),
            Some(existing) => self.handle_duplicate_link(existing, uri, link),
        }
    }

    /// Retrieves a [`Source`] from the store by [`AbsoluteUri`], if a [`Link`]
    /// exists.
    pub fn get<'s>(&'s self, uri: &AbsoluteUri) -> Option<Source<'s>> {
        self.links.get_key_value(uri).map(|(uri, link)| Source {
            key: link.key,
            uri: Cow::Borrowed(uri),
            path: Cow::Borrowed(&link.path),
            document: self.docs[link.key].clone(),
        })
    }

    /// Retrieves the root document [`Value`] by [`SrcKey`].
    pub fn get_document(&self, key: SrcKey) -> Option<Arc<Value>> {
        self.docs.get(key).cloned()
    }

    /// Retrieves the associated [`Link`] by [`AbsoluteUri`], if it eists.
    pub fn get_link(&self, uri: &AbsoluteUri) -> Option<&Link> {
        self.links.get(uri)
    }

    /// Removes and returns the [`Link`] associated to the `uri` if it exists.
    pub fn remove_link(&mut self, uri: &AbsoluteUri) -> Option<Link> {
        self.links.remove(uri)
    }

    /// Inserts a new source, unique [`Arc<Value>`](`Value`) into the store
    /// and creates applicable [`Link`]s.
    ///
    /// Values placed into the repository are considered immutable. Once a value
    /// is associated to an [`AbsoluteUri`], it cannot be changed. Attempting to
    /// insert a value which is different from one already indexed to different
    /// URI will result in an error (see [Errors](##Errors)).
    ///
    /// Additionally, the [`AbsoluteUri`] must not contain a fragment with a
    /// JSON pointer. Attempting to insert with a fragment that starts with `/`
    /// will be treated as a reference to a path within the document and an
    /// error will be returned (see [Errors](##Errors)).
    ///
    /// Upon insertion, a [`Link`] is created for the base URI (`uri` without a
    /// fragment). In the event the URI contains a non-pointer fragment, a
    /// [`Link`] is created for the anchored URI (`uri` with the fragment) as
    /// well.
    ///
    /// ## Example
    /// ```rust
    /// # use std::sync::Arc;
    /// # use grill_uri::AbsoluteUri;
    /// # use serde_json::json;
    ///
    /// let mut sources = Sources::new();
    /// let uri = AbsoluteUri::parse("https://example.com").unwrap();
    /// let document = json!({"foo": "bar"});
    /// let key = sources.insert(uri.clone(), Arc::new(document)).unwrap();
    /// assert_eq!(sources.get_document(key).unwrap().as_ref(), &document);
    /// assert_eq!(sources.get(&uri).unwrap().resolve(), &document);
    /// ```
    ///
    /// ## Errors
    /// Returns [`InsertError`] if:
    /// - If the URI contains a JSON pointer fragment (e.g.
    ///   `https://example.com#/foo/bar`)
    /// - If the URI is already indexed to a different value
    pub fn insert(&mut self, uri: AbsoluteUri, value: Arc<Value>) -> Result<SrcKey, InsertError> {
        if uri.has_fragment() {
            self.insert_fragmented(uri, value)
        } else {
            self.insert_base(uri, value)
        }
    }

    fn insert_fragmented(
        &mut self,
        uri: AbsoluteUri,
        document: Arc<Value>,
    ) -> Result<SrcKey, InsertError> {
        let fragment = uri.fragment().unwrap_or_default().trim();
        if fragment.starts_with('/') {
            return insert_error::PointerFragmentSnafu
                .fail()
                .with_context(|_| InsertSnafu { uri, document });
        }
        let key = self.insert(uri.without_fragment(), document)?;
        let path = Pointer::default();
        self.link(uri, Link { key, path }).unwrap();
        Ok(key)
    }

    fn insert_base(
        &mut self,
        uri: AbsoluteUri,
        document: Arc<Value>,
    ) -> Result<SrcKey, InsertError> {
        if self.links.contains_key(&uri) {
            return self.check_existing(uri, document);
        }
        let key = self.docs.insert(document);
        let path = Pointer::default();
        self.links.insert(uri, Link { key, path });
        Ok(key)
    }

    fn check_existing(&self, uri: AbsoluteUri, value: Arc<Value>) -> Result<SrcKey, InsertError> {
        let existing = self.links.get(&uri).unwrap();
        let existing_value = &self.docs[existing.key];
        if value.as_ref() != existing_value.as_ref() {
            InsertError::source_conflict(uri, value, existing.clone(), existing_value.clone())
        } else {
            Ok(existing.key)
        }
    }

    fn insert_link(&mut self, uri: AbsoluteUri, link: Link) -> Result<(), LinkError> {
        let src = self.docs.get(link.key).unwrap();
        let _ = link
            .path
            .resolve(src)
            .context(link_error::ResolutionFailedSnafu)
            .with_context(|_| LinkSnafu {
                link: link.clone(),
                uri: uri.clone(),
            })?;

        self.links.insert(uri, link);
        Ok(())
    }

    fn handle_duplicate_link(
        &self,
        existing: &Link,
        uri: AbsoluteUri,
        link: Link,
    ) -> Result<(), LinkError> {
        if &link != existing {
            LinkError::confict(uri, link, existing.clone())
        } else {
            Ok(())
        }
    }
}

/// An error occurred while inserting a source document.
///
/// See [`InsertErrorCause`] for potential causes.
#[derive(Debug, Snafu)]
#[snafu(display("Failed to insert source {uri}\n\tcaused by:{cause}"))]
pub struct InsertError {
    /// The [`AbsoluteUri`] attempting to be inserted
    pub uri: AbsoluteUri,

    /// The [`Arc<Value>`](`Value`) attempting to be inserted
    pub document: Arc<Value>,

    /// The [`Cause`](`insert_error::Cause`) of the error
    #[snafu(source)]
    pub cause: insert_error::Cause,

    /// Backtrace of the error
    pub backtrace: Backtrace,
}

impl InsertError {
    /// Returns an `Result<T, Self>::Err(Self)` with an [`InsertErrorCause`]
    /// of [`PointerFragment`](InsertErrorCause::PointerFragment).
    pub fn pointer_fragment<T>(uri: AbsoluteUri, document: Value) -> Result<T, Self> {
        insert_error::PointerFragmentSnafu
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
        insert_error::SourceConflictSnafu {
            existing_link,
            existing_value,
        }
        .fail()
        .with_context(|_| InsertSnafu { uri, document })
    }
}

pub use insert_error::Cause as InsertErrorCause;

mod insert_error {
    use super::{Arc, Link, Snafu, Value};

    /// Cause of an [`InsertError`].
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(super)))]
    pub enum Cause {
        /// A source document was attempted to be inserted at an [`AbsoluteUri`]
        /// with a fragment starting with '/'` (e.g. `https://example.com#/foo/bar`).
        #[snafu(display(
            "URI contains a JSON pointer fragment; only root documents can be added as sources.",
        ))]
        PointerFragment,

        /// A source document was attempted to be inserted at an [`AbsoluteUri`]
        /// that is already indexed to a different source document.
        #[snafu(display("URI is indexed to a different source."))]
        SourceConflict {
            /// The existing [`Link`] associated with the URI
            existing_link: Link,
            /// The existing [`Value`] associated with the URI
            existing_value: Arc<Value>,
        },
    }
}

/// An error occurred while inserting a [`Link`].
///
/// See [`LinkErrorCause`] for potential causes of this error.
#[derive(Debug, Snafu)]
#[snafu(display("Failed to link {uri}\ncaused by:\n\t{cause}"))]
pub struct LinkError {
    /// The [`AbsoluteUri`] attempting to be linked
    pub uri: AbsoluteUri,

    /// The [`Link`] attempting to be inserted
    pub link: Link,

    /// The [`cause`](LinkErrorCause) of the error
    #[snafu(source)]
    pub cause: link_error::Cause,
}
impl LinkError {
    /// Returns an `Result<T, Self>::Err(Self)` with a [`LinkErrorCause`]
    /// of [`Conflict`](LinkErrorCause::Conflict).
    pub fn confict<T>(uri: AbsoluteUri, link: Link, existing: Link) -> Result<T, Self> {
        link_error::ConflictSnafu { existing }
            .fail()
            .with_context(|_| LinkSnafu { uri, link })
    }
}

pub use link_error::Cause as LinkErrorCause;

mod link_error {
    use super::{Link, Snafu};

    /// Underlying cause of a [`LinkError`].
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(super)))]
    pub enum Cause {
        /// The JSON pointer of the [`Link`] could not be resolved within the source.
        #[snafu(display("Failed to resolve JSON pointer of sourced document: {source}"))]
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
}
#[cfg(test)]
pub mod test {
    use super::*;
    use serde_json::json;
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
        sources.link(uri, Link { key, path }).unwrap();

        let uri = AbsoluteUri::parse("https://example.com/#/foo/bar").unwrap();
        let source = sources.get(&uri).unwrap();
        assert_eq!(source.resolve(), &json!("baz"));
        let invalid_path_uri = base_uri
            .with_fragment(Some(&Pointer::new(["bad", "path"])))
            .unwrap();
        assert_eq!(sources.get(&invalid_path_uri), None)
    }
}
