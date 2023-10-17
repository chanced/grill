//! # Data structures to represent Uniform Resource Identifiers (URI) [RFC 3986](https://tools.ietf.org/html/rfc3986).
//!
//! A Uniform Resource Identifier (URI) provides a simple and extensible means
//! for identifying a resource.
//!
//! ## Formats
//!
//! URIs can come in four different formats:
//!
//! ### Uniform Resource Locator (URL)
//!
//! A URL is fully qualified reference to a web resource. For
//! example`"https://example.com"` or `"mailto:me@example.com"`.
//!
//! URLs are represented using the [`url`](`url`) crate and can be  in the form
//! of a [`Url`], [`Uri`], [`AbsoluteUri`], or [`UriRef`].
//!
//! ```plaintext
//!               userinfo         host    port
//!          ┌───────┴───────┐ ┌────┴────┐ ┌┴┐
//! "https://john.doe:password@example.com:123/forum/questions/?tag=networking&order=newest#top"
//!  └─┬─┘   └───────────────┬───────────────┘└───────┬───────┘ └────────────┬────────────┘ └┬┘
//! scheme               authority                   path                  query        fragment
//! ```
//! ```rust
//! use grill::uri::{ Uri, AbsoluteUri };
//!
//! let input = "https://john.doe@example.com:123/forum/questions/?tag=networking&order=newest#top";
//! let uri = Uri::parse(input).unwrap();
//! assert_eq!(&uri, input);
//! assert_eq!(uri.scheme(), "https");
//! assert_eq!(uri.user(), Some("john.doe"));
//! assert_eq!(uri.host_or_namespace(), "example.com");
//! assert_eq!(uri.port(), Some(123));
//! assert_eq!(uri.path_or_nss(), "/forum/questions/");
//! assert_eq!(uri.query(), Some("tag=networking&order=newest"));
//! assert_eq!(uri.fragment(), Some("top"));
//! assert_eq!(uri.authority().unwrap(), "john.doe@www.example.com:123");
//! assert!(uri.is_url())
//!
//! let abs_uri = AbsoluteUri::parse(s).unwrap();
//! assert_eq!(uri, abs_uri);
//! ```
//! Note that parsing a URL by means of a [`Uri`] or [`AbsoluteUri`] will take
//! an additional `O(n + 1)`, where `n` is the length of the scheme, over
//! parsing with [`url::Url`] directly.
//!
//! ### Uniform Resource Location (URN)
//!
//! A URN is fully qualified, globally unique, persistent identifier. e.g.
//! `"urn:oasis:names:specification:docbook:dtd:xml:4.1.2"`.
//!
//! URNs are represented using the [`urn`](`urn`) crate and can be in the form
//! of a [`Urn`], [`Uri`], [`AbsoluteUri`] or [`UriRef`].
//!
//! ```plaintext
//! "urn:example:articles:record?category=science#fragment"
//!  └┬┘ └─┬───┘ └──────┬──────┘ └────────────┬─┘ └──┬───┘
//! scheme │  namespace specific string (NSS) │   fragment
//!   namespace (NID)                       query
//! ```
//! ```rust
//! use grill::uri::{ Uri, AbsoluteUri };
//!
//! let s = "urn:example:articles:record?category=science#fragment";
//! let uri = Uri::parse(s).unwrap();
//! assert_eq!(&uri, s);
//! assert_eq!(uri.scheme(), "urn");
//! assert_eq!(uri.user(), None);
//! assert_eq!(uri.host_or_namespace(), "example");
//! assert_eq!(uri.port(), None);
//! assert_eq!(uri.path_or_nss(), "articles:record");
//! assert_eq!(uri.query(), Some("category=science"));
//! assert_eq!(uri.fragment(), Some("fragment"));
//! assert_eq!(uri.authority(), None);
//! assert!(uri.is_urn())
//!
//! let abs_uri = AbsoluteUri::parse(s).unwrap();
//! assert_eq!(uri, abs_uri);
//! ```
//!
//! Note that parsing a URN by means of a [`Uri`] or [`AbsoluteUri`] will take
//! an additional `O(4)` over parsing with [`urn::Urn`] directly.
//!
//! ### Relative URI with authority
//! A relative URI with an authority is indicated by the prefixed double slashes
//! (`"//"`) and may contain user credentials, host, port, path, query, and
//! fragment. For example: `"//user:password@example.com/path/to/resource`.
//!
//! Relative URIs with authority are represented using the [`RelativeUri`] type
//! and can be in the form of a [`RelativeUri`], [`Uri`] or [`UriRef`].
//!
//! ```plaintext
//!         userinfo        host    port
//!    ┌───────┴───────┐ ┌────┴────┐ ┌┴┐
//! "//john.doe:password@example.com:123/forum/questions/?tag=networking&order=newest#top
//!    └───────────────┬───────────────┘└───────┬───────┘ └────────────┬────────────┘ └┬┘
//!                authority                   path                  query         fragment
//! ```
//! ```rust
//!  use grill::uri::{ Uri };
//! let s = "//john.doe@example.com:123/forum/questions/?tag=networking&order=newest#top";
//! let uri = Uri::parse(s).unwrap();
//! assert_eq!(&uri, s);
//! assert_eq!(uri.scheme(), None);
//! assert_eq!(uri.user(), Some("jon.doe"));
//! assert_eq!(uri.password(), "password");
//! assert_eq!(uri.path_or_nss(), "/forum/questions/")
//! assert_eq!(uri.host_or_namespace(), "example.com");
//! assert_eq!(uri.port(), None);
//! assert_eq!(uri.query(), Some("tag=networking&order=newest"));
//! assert_eq!(uri.fragment(), Some("top"));
//! ```
//!
//! ### Relative URI without authority
//!
//! A relative URI without authority is a partial URI that does not contain user
//! crednetials, host, or port. A relative URI without authority may contain a
//! path, query, and fragment. For example:
//! `"/path/to/resource?query=string#fragment"`.
//!
//! Relative URIs without authority are represented using the [`RelativeUri`]
//! type and can be in the form of a [`RelativeUri`], [`Uri`], or a [`UriRef`].
//!
//! ```plaintext
//! "/forum/questions/?tag=networking&order=newest#top"
//!  └───────┬───────┘ └─────────────┬───────────┘ └┬┘
//!         path                   query         fragment
//! ```
//! ```rust
//!  use grill::uri::{ Uri };
//! let s = "/forum/questions/?tag=networking&order=newest#top";
//! let uri = Uri::parse(s).unwrap();
//! assert_eq!(&uri, s);
//! assert_eq!(uri.path_or_nss(), "/forum/questions/")
//! assert_eq!(uri.scheme(), None);
//! assert_eq!(uri.user(), None);
//! assert_eq!(uri.host_or_namespace(), None);
//! assert_eq!(uri.port(), None);
//! assert_eq!(uri.query(), Some("tag=networking&order=newest"));
//! assert_eq!(uri.fragment(), Some("top"));
//! ```

use percent_encoding::percent_decode;

mod encode;
mod get;
mod parse;
mod set;
mod write;

#[cfg(test)]
mod test;

use crate::error::UrnError;
#[doc(no_inline)]
pub use url::Url;
#[doc(no_inline)]
pub use urn::Urn;

use crate::{
    big::usize_to_u32,
    error::{OverflowError, RelativeUriError, UriError},
};
use inherent::inherent;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    iter::Peekable,
    ops::{Deref, Index},
    path::PathBuf,
    str::{FromStr, Split},
    string::{String, ToString},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
/// The Authority component of a Relative URI.
pub struct Authority<'a> {
    pub(super) value: Cow<'a, str>,
    pub(super) username_index: Option<u32>,
    pub(super) password_index: Option<u32>,
    pub(super) host_index: Option<u32>,
    pub(super) port_index: Option<u32>,
    pub(super) port: Option<u16>,
}
impl Deref for Authority<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Display for Authority<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<Authority<'_>> for String {
    fn from(value: Authority) -> Self {
        value.to_string()
    }
}

impl From<Authority<'_>> for Cow<'_, str> {
    fn from(value: Authority) -> Self {
        Cow::Owned(value.to_string())
    }
}

impl<'a> Authority<'a> {
    /// Returns the username component if it exists.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        let start = self.username_index()?;
        let end = self
            .password_index
            .or(self.host_index)
            .or(self.port_index)
            .map_or(self.value.len(), |idx| idx as usize);

        Some(&self.value[start..end])
    }

    /// Returns the password component if it exists.
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        let start = self.password_index()?;
        let end = self
            .host_index()
            .or(self.port_index())
            .unwrap_or(self.value.len());
        Some(&self.value[start..end])
    }

    /// Returns the host component if it exists.
    #[must_use]
    pub fn host(&self) -> Option<&str> {
        let offset = usize::from(self.username_index.is_some() || self.password_index.is_some());
        let start = self.host_index()? + offset;
        let end = self.port_index().unwrap_or(self.value.len());
        Some(&self.value[start..end])
    }

    /// Returns the port component if it exists.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Returns the port as an `&str` if it exists.
    #[must_use]
    pub fn port_str(&self) -> Option<&str> {
        self.port_index().map(|idx| &self.value[idx + 1..])
    }

    /// Returns the
    #[must_use]
    pub fn into_owned(&self) -> Authority<'static> {
        Authority {
            value: Cow::Owned(self.value.to_string()),
            username_index: self.username_index,
            password_index: self.password_index,
            host_index: self.host_index,
            port_index: self.port_index,
            port: self.port,
        }
    }

    /// Returns the `&str` representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    fn port_index(&self) -> Option<usize> {
        self.port_index.map(|idx| idx as usize)
    }
    fn host_index(&self) -> Option<usize> {
        self.host_index.map(|idx| idx as usize)
    }
    fn username_index(&self) -> Option<usize> {
        self.username_index.map(|idx| idx as usize)
    }
    fn password_index(&self) -> Option<usize> {
        self.password_index.map(|idx| idx as usize)
    }
}

/// A single segment of a URI's path.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum PathSegment<'a> {
    /// The root of the path
    Root,

    /// A reference to the current path segment, i.e., `.`.
    Current,

    /// A reference to the parent path segment, i.e., `..`.
    Parent,

    /// A normal path segment, e.g., `a` and `b` in `a/b`.
    Normal(Cow<'a, str>),
}

impl<'a> PathSegment<'a> {
    /// Returns `true` if the path segment is [`Root`].
    ///
    /// [`Root`]: PathSegment::Root
    #[must_use]
    pub fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    /// Returns `true` if the path segment is [`Current`].
    ///
    /// [`Current`]: PathSegment::Current
    #[must_use]
    pub fn is_current(&self) -> bool {
        matches!(self, Self::Current)
    }

    /// Returns `true` if the path segment is [`Parent`].
    ///
    /// [`Parent`]: PathSegment::Parent
    #[must_use]
    pub fn is_parent(&self) -> bool {
        matches!(self, Self::Parent)
    }

    /// Returns `true` if the path segment is [`Normal`].
    ///
    /// [`Normal`]: PathSegment::Normal
    #[must_use]
    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal(..))
    }

    /// Returns the path segment as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Root => "",
            Self::Current => ".",
            Self::Parent => "..",
            Self::Normal(val) => val.as_ref(),
        }
    }
}

impl<'a> PathSegment<'a> {
    pub fn normal(val: impl Into<Cow<'a, str>>) -> Self {
        Self::Normal(val.into())
    }

    pub fn decode(&'a self) -> Result<Cow<'a, str>, std::str::Utf8Error> {
        match self {
            PathSegment::Root => Ok(Cow::Borrowed("")),
            PathSegment::Current => Ok(Cow::Borrowed(".")),
            PathSegment::Parent => Ok(Cow::Borrowed("..")),
            PathSegment::Normal(val) => percent_decode(val.as_bytes()).decode_utf8(),
        }
    }

    #[must_use]
    pub fn decode_lossy(&'a self) -> Cow<'a, str> {
        match self {
            PathSegment::Root => Cow::Borrowed(""),
            PathSegment::Current => Cow::Borrowed("."),
            PathSegment::Parent => Cow::Borrowed(".."),
            PathSegment::Normal(val) => percent_decode(val.as_bytes()).decode_utf8_lossy(),
        }
    }
    fn parse_root(val: &'a str, next: Option<char>) -> Self {
        match val {
            "" => Self::Root,
            "." | ".." => Self::remove_dots(val, next),
            _ => Self::Normal(val.into()),
        }
    }
    fn parse_path_segment(val: &'a str, next: Option<char>) -> Self {
        match val {
            "." | ".." => Self::remove_dots(val, next),
            _ => Self::Normal(val.into()),
        }
    }
    fn remove_dots(val: &'a str, next: Option<char>) -> Self {
        if next == Some('/') || next.is_none() {
            if val == "." {
                Self::Current
            } else {
                Self::Parent
            }
        } else {
            Self::Normal(val.into())
        }
    }
}

impl PartialEq<String> for PathSegment<'_> {
    fn eq(&self, other: &String) -> bool {
        self.eq(other.as_str())
    }
}
impl PartialEq<&String> for PathSegment<'_> {
    fn eq(&self, other: &&String) -> bool {
        self.eq(other.as_str())
    }
}

impl PartialEq<str> for PathSegment<'_> {
    fn eq(&self, other: &str) -> bool {
        match self {
            PathSegment::Root => other.is_empty(),
            PathSegment::Current => other == ".",
            PathSegment::Parent => other == "..",
            PathSegment::Normal(val) => val == other,
        }
    }
}
impl PartialEq<&str> for PathSegment<'_> {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<str> for &PathSegment<'_> {
    fn eq(&self, other: &str) -> bool {
        *self == other
    }
}
impl PartialEq<String> for &PathSegment<'_> {
    fn eq(&self, other: &String) -> bool {
        *self == other
    }
}

impl<'a> PartialEq<&PathSegment<'a>> for str {
    fn eq(&self, other: &&PathSegment) -> bool {
        other.eq(self)
    }
}
impl<'a> PartialEq<&PathSegment<'a>> for String {
    fn eq(&self, other: &&PathSegment) -> bool {
        other.eq(self)
    }
}

/// An [`Iterator`] of [`PathSegment`]s.
#[derive(Debug, Default)]
pub struct PathSegments<'a> {
    path: Option<Peekable<Split<'a, char>>>,
    pub(crate) base_only: bool,
    root_sent: bool,
}

impl<'a> PathSegments<'a> {
    #[must_use]
    pub fn new(path: &'a str) -> Self {
        Self {
            path: Some(path.split('/').peekable()),
            root_sent: false,
            base_only: false,
        }
    }

    fn peek(&mut self) -> Option<&str> {
        self.path.as_mut().and_then(|p| p.peek().copied())
    }
}

impl<'a> From<&'a str> for PathSegments<'a> {
    fn from(path: &'a str) -> Self {
        Self {
            path: Some(path.split('/').peekable()),
            root_sent: false,
            base_only: false,
        }
    }
}

impl<'a> Iterator for PathSegments<'a> {
    type Item = PathSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.path.as_mut()?.next()?;
        if self.root_sent {
            let base_only = self.base_only;
            let next = self.peek();
            let next_char = next.and_then(|s| s.chars().next());
            if base_only && next.is_none() {
                return None;
            }
            return Some(PathSegment::parse_path_segment(value, next_char));
        }
        self.root_sent = true;
        let next = self.peek();
        let next_char = next.and_then(|s| s.chars().next());
        Some(PathSegment::parse_root(value, next_char))
    }
}

/// A URI in the form of a fully qualified [`Url`] or [`Urn`].
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum AbsoluteUri {
    Url(Url),
    Urn(Urn),
}
impl std::fmt::Debug for AbsoluteUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl AbsoluteUri {
    /// Attempts to parse an `AbsoluteUri`.
    ///
    /// # Errors
    /// Returns [`UriError`] if `value` can not be parsed as a
    /// [`Url`](`url::Url`) or [`Urn`](`urn::Urn`)
    pub fn parse(value: &str) -> Result<Self, UriError> {
        if value.starts_with("urn:") {
            Ok(Urn::from_str(value)?.into())
        } else {
            Ok(Url::parse(value)?.into())
        }
    }

    /// Returns a new [`Components`] iterator over all components of the `AbsoluteUri`.
    /// # Example
    /// ```rust
    /// use grill::uri::{ AbsoluteUri, Component, PathSegment, QueryParameter };
    /// let s = "https://user:password@example.com/path/to/file/?query=str#fragment";
    /// let uri = AbsoluteUri::parse(s).unwrap();
    /// let components = vec![
    ///     Component::Scheme("https".into()),
    ///     Component::Username("user".into()),
    ///     Component::Password("password".into()),
    ///     Component::Host("example.com".into()),
    ///     Component::PathSegment(PathSegment::Root),
    ///     Component::PathSegment(PathSegment::Normal("path".into())),
    ///     Component::PathSegment(PathSegment::Normal("to".into())),
    ///     Component::PathSegment(PathSegment::Normal("file".into())),
    ///     Component::QueryParameter(QueryParameter::new("query=str").unwrap()),
    ///     Component::Fragment("fragment".into()),
    /// ];
    /// assert_eq!(uri.components().collect::<Vec<_>>(), components);
    #[must_use]
    pub fn components(&self) -> Components {
        Components::from_absolute_uri(self)
    }

    /// Returns a new [`PathSegments`] iterator over all segments of this
    /// `AbsoluteUri`'s path.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::{ AbsoluteUri, PathSegment };
    /// let uri = AbsoluteUri::parse("https://example.com/path/to/file").unwrap();
    /// let segments = uri.path_segments().collect::<Vec<_>>();
    /// assert_eq!(&segments, &["", "path", "to", "file"]);
    /// assert_eq!(&segments, &[
    ///     PathSegment::Root,
    ///     PathSegment::Normal("path".into()),
    ///     PathSegment::Normal("to".into()),
    ///     PathSegment::Normal("file".into()),
    /// ])
    /// ```
    #[must_use]
    pub fn path_segments(&self) -> PathSegments {
        PathSegments::from(self.path_or_nss())
    }

    /// Returns the base path of the `AbsoluteUri`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap()
    ///     .as_relative_uri().unwrap();
    ///
    /// assert_eq!(uri.base_path(), "/path/to");
    /// ```
    #[must_use]
    pub fn base_path(&self) -> &str {
        self.path_or_nss()
            .rsplit_once('/')
            .map_or(self.path_or_nss(), |(a, _)| a)
    }

    /// Returns a [`PathSegments`] iterator over the base path segments,
    /// essentially every segment except the last.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file");
    /// let relative_uri = uri.as_relative_uri().unwrap();
    /// assert_eq!(relative_uri.base_path_segments().collect::<Vec<_>>(), vec!["path", "to"]);
    #[must_use]
    pub fn base_path_segments(&self) -> PathSegments<'_> {
        let mut segments = self.path_segments();
        segments.base_only = true;
        segments
    }

    /// Returns the percent encoded fragment, if it exists.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        match self {
            Self::Url(url) => url.fragment(),
            Self::Urn(urn) => urn.f_component(),
        }
    }

    /// Percent encodes and sets the fragment component of the [`Url`] or
    /// [`Urn`] and returns the previous fragment in percent-encoded format if
    /// it exists.
    ///
    /// # Errors
    /// Returns [`UriError::Urn`] if the [`AbsoluteUri`] is a [`Urn`](`urn::Urn`) and
    /// the fragment is not a valid [`Urn`] fragment.
    pub fn set_fragment(&mut self, fragment: Option<&str>) -> Result<Option<String>, UriError> {
        match self {
            Self::Url(url) => Ok(set::url::fragment(url, fragment)),
            Self::Urn(urn) => set::urn::fragment(urn, fragment),
        }
    }

    /// Returns the authority (`Url`) or namespace (`Urn`)
    #[must_use]
    pub fn authority_or_namespace(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::Url(url) => get::url::authority(url).map(Cow::Owned),
            Self::Urn(urn) => Some(Cow::Borrowed(urn.nid())),
        }
    }

    /// Returns the path ([`Url`](crate::uri::Url)) or Name Specific String
    /// ([`Urn`](crate::uri::Urn)
    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            Self::Url(url) => url.path(),
            Self::Urn(urn) => urn.nss(),
        }
    }

    /// Sets the path (`Url`) or Name Specific String (`Urn`)
    pub fn set_path_or_nss(&mut self, path_or_nss: &str) -> Result<String, UriError> {
        match self {
            Self::Url(url) => Ok(set::url::path(url, path_or_nss)),
            Self::Urn(urn) => set::urn::nss(urn, path_or_nss),
        }
    }

    /// Sets the authority (`Url`) or namespace (`Urn`)
    pub fn set_authority_or_namespace(
        &mut self,
        authority_or_namespace: &str,
    ) -> Result<Option<String>, UriError> {
        match self {
            Self::Url(u) => set::url::authority(u, authority_or_namespace),
            Self::Urn(u) => set::urn::namespace(u, authority_or_namespace),
        }
    }
    /// Returns the `&str` representation of the `AbsoluteUri`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Url(url) => url.as_str(),
            Self::Urn(urn) => urn.as_str(),
        }
    }

    /// Returns `true` if the `AbsoluteUri` is a [`Url`](`url::Url`).
    ///
    /// [`Url`]: AbsoluteUri::Url
    #[must_use]
    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url(..))
    }

    #[must_use]
    pub fn as_url(&self) -> Option<&Url> {
        if let Self::Url(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the `AbsoluteUri` is a [`Urn`](`urn::Urn`).
    ///
    /// [`Urn`]: AbsoluteUri::Urn
    #[must_use]
    pub fn is_urn(&self) -> bool {
        matches!(self, Self::Urn(..))
    }
    #[must_use]
    pub fn as_urn(&self) -> Option<&Urn> {
        if let Self::Urn(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the query component if it exists.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        match self {
            Self::Url(url) => url.query(),
            Self::Urn(urn) => urn.q_component(),
        }
    }
    /// Returns an [`Iterator`] of [`QueryParameter`] of this `AbsoluteUri`.
    #[must_use]
    pub fn query_parameters(&self) -> QueryParameters<'_> {
        QueryParameters::new(self.query()).unwrap()
    }
    /// Sets the query component of the [`Url`] or [`Urn`] and returns the
    /// previous query, if it existed.
    pub fn set_query(&mut self, query: Option<&str>) -> Result<Option<String>, UrnError> {
        let prev = self.query().map(ToString::to_string);
        match self {
            Self::Url(url) => {
                url.set_query(query);
            }
            Self::Urn(urn) => {
                urn.set_q_component(query)?;
            }
        }
        Ok(prev)
    }

    #[must_use]
    pub fn is_fragment_empty_or_none(&self) -> bool {
        self.fragment().map_or(true, |f| f.trim().is_empty())
    }

    /// returns a new `AbsoluteUri` that is the result of resolving the given
    /// reference against this `AbsoluteUri`.
    ///
    /// See [RFC3986, Section
    /// 5.2.2](https://tools.ietf.org/html/rfc3986#section-5.2.2).
    pub fn resolve(&self, reference: &impl AsUriRef) -> Result<AbsoluteUri, UriError> {
        let reference = reference.as_uri_ref();

        // if the reference has a scheme, normalize the path and return
        if let Ok(mut uri) = reference.try_into_absolute_uri() {
            uri.normalize_path();
            return Ok(uri);
        }

        // safety: urls and urns will get processed in the match above
        let reference = reference.as_relative_uri().unwrap();

        if let Some(authority) = reference.authority() {
            let mut uri = self.clone();
            uri.set_authority_or_namespace(&authority)?;
            uri.set_query(reference.query())?;
            uri.set_fragment(reference.fragment())?;
            uri.set_path_or_nss(&reference.path_normalized())?;
            return Ok(uri);
        }

        if reference.path().is_empty() {
            let mut uri = self.clone();
            if let Some(fragment) = reference.fragment() {
                if !fragment.is_empty() {
                    uri.set_fragment(Some(fragment))?;
                }
            }
            if let Some(query) = reference.query() {
                if !query.is_empty() {
                    uri.set_query(Some(query))?;
                }
            }
            return Ok(uri);
        }

        let mut uri = self.clone();
        uri.set_query(reference.query())?;
        uri.set_fragment(reference.fragment())?;

        if reference.path().starts_with('/') {
            uri.set_path_or_nss(&reference.path_normalized())?;
            return Ok(uri);
        }
        let base = self.base_path();
        let base = merge(base, reference.path());
        uri.set_path_or_nss(&base)?;

        Ok(uri)
    }

    #[must_use]
    pub fn scheme(&self) -> &str {
        match self {
            AbsoluteUri::Url(url) => url.scheme(),
            AbsoluteUri::Urn(_) => "urn",
        }
    }
    /// Sets the scheme of the `AbsoluteUri` and returns the previous scheme. If
    /// this `AbsoluteUri` is an [`Urn`](`urn::Urn`), then a `scheme` value of
    /// anything other than `"urn"` will result in the URN being parsed as a
    /// URL. If the `AbsoluteUri` is a [`Url`](`url::Url`), then a scheme value
    /// of anything other than `"urn"` will result in the URL being parsed as a
    /// [`Urn`](crate::uri::Urn).
    ///
    /// # Errors
    /// Returns a [`UriError`] if:
    /// - the scheme is invalid
    /// - the `AbsoluteUri` is currently an [`Urn`](`urn::Urn`), the value of
    ///  `scheme` is something other than `"urn"`, and the URN cannot be parsed
    ///  as a URL.
    /// - the `AbsoluteUri` is currently an [`Url`](`url::Url`), the value of
    ///   `scheme` is `"urn"`, and the URL cannot be parsed as a URN.
    pub fn set_scheme(&mut self, scheme: &str) -> Result<String, UriError> {
        let scheme = scheme.trim_end_matches('/').trim_end_matches(':');

        let prev = self.scheme().to_string();
        let to_uri_err = |()| UriError::InvalidScheme(scheme.to_string());
        match self {
            AbsoluteUri::Url(url) => {
                if scheme == "urn" {
                    let mut s = url.to_string();
                    let i = url.scheme().len() + 3;
                    s.replace_range(..i, "urn:");
                    let urn = Urn::from_str(&s)?;
                    *self = AbsoluteUri::Urn(urn);
                } else {
                    url.set_scheme(scheme).map_err(to_uri_err)?;
                }
            }
            AbsoluteUri::Urn(urn) => {
                if scheme != "urn" {
                    let mut s = urn.to_string();
                    s.replace_range(..3, scheme);
                    match Url::from_str(&s) {
                        Ok(url) => *self = AbsoluteUri::Url(url),
                        Err(err) => {
                            s.insert_str(4, "//");
                            *self = AbsoluteUri::Url(Url::parse(&s).map_err(|_| err)?);
                        }
                    }
                }
            }
        }
        Ok(prev)
    }

    /// Returns the normalized path which removes dot segments, i.e. `'.'`,
    /// `'..'`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::AbsoluteUri;
    /// let mut uri = AbsoluteUri::parse("https://example.com/./foo/../bar").unwrap();
    /// let normalized = uri.path_normalized();
    /// assert_eq!(normalized, "/bar");
    /// ```
    #[must_use]
    pub fn path_normalized(&self) -> Cow<'_, str> {
        normalize(self.path_or_nss())
    }

    /// Normalizes the path by removing dot segments, i.e. `'.'`, `'..'`.
    ///
    /// This method mutates the `AbsoluteUri` in place. If that is not desired, use
    /// [`path_normalized`](AbsoluteUri::path_normalized) instead.
    ///
    /// # Example
    /// ```
    /// use grill::uri::AbsoluteUri;
    /// let mut uri = AbsoluteUri::parse("https://example.com/./foo/../bar").unwrap();
    /// uri.normalize_path();
    /// assert_eq!(uri.path_or_nss(), "/bar");
    /// ```
    pub fn normalize_path(&mut self) {
        let normalized = normalize(self.path_or_nss()).to_string();
        self.set_path_or_nss(&normalized).unwrap();
    }

    /// Returns `true` if this `AbsoluteUri` has a query component.
    #[must_use]
    pub fn has_query(&self) -> bool {
        self.query().is_some()
    }
    /// Returns `true` if this `AbsoluteUri` has a fragment component.
    #[must_use]
    pub fn has_fragment(&self) -> bool {
        self.fragment().is_some()
    }
}

impl Borrow<str> for AbsoluteUri {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl Borrow<[u8]> for AbsoluteUri {
    fn borrow(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl From<&AbsoluteUri> for AbsoluteUri {
    fn from(value: &AbsoluteUri) -> Self {
        value.clone()
    }
}

impl From<AbsoluteUri> for String {
    fn from(value: AbsoluteUri) -> Self {
        value.to_string()
    }
}
impl From<&AbsoluteUri> for String {
    fn from(value: &AbsoluteUri) -> Self {
        value.to_string()
    }
}
impl TryFrom<Uri> for AbsoluteUri {
    type Error = UriError;

    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        match value {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url)),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn)),
            Uri::Relative(p) => Self::parse(p.as_str()),
        }
    }
}
impl TryFrom<&Uri> for AbsoluteUri {
    type Error = UriError;

    fn try_from(value: &Uri) -> Result<Self, Self::Error> {
        match value {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url.clone())),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn.clone())),
            Uri::Relative(p) => Self::parse(p.as_str()),
        }
    }
}
impl PartialOrd for AbsoluteUri {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AbsoluteUri {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(self.as_str(), other.as_str())
    }
}
impl PartialEq<&str> for AbsoluteUri {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<AbsoluteUri> for &str {
    fn eq(&self, other: &AbsoluteUri) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<AbsoluteUri> for str {
    fn eq(&self, other: &AbsoluteUri) -> bool {
        match other {
            AbsoluteUri::Url(url) => self == url.as_str(),
            AbsoluteUri::Urn(urn) => self == urn.as_str(),
        }
    }
}
impl PartialEq<str> for AbsoluteUri {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<String> for AbsoluteUri {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}
impl PartialEq<&String> for AbsoluteUri {
    fn eq(&self, other: &&String) -> bool {
        self == *other
    }
}
impl PartialEq<AbsoluteUri> for String {
    fn eq(&self, other: &AbsoluteUri) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<AbsoluteUri> for &String {
    fn eq(&self, other: &AbsoluteUri) -> bool {
        *self == other
    }
}

impl Deref for AbsoluteUri {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Url(url) => url.as_str(),
            Self::Urn(urn) => urn.as_str(),
        }
    }
}
impl Display for AbsoluteUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Url(url) => url.fmt(f),
            Self::Urn(urn) => urn.fmt(f),
        }
    }
}

impl From<Url> for AbsoluteUri {
    fn from(value: Url) -> Self {
        Self::Url(value)
    }
}

impl From<Urn> for AbsoluteUri {
    fn from(value: Urn) -> Self {
        Self::Urn(value)
    }
}
impl From<&Url> for AbsoluteUri {
    fn from(value: &Url) -> Self {
        AbsoluteUri::Url(value.clone())
    }
}
impl From<&Urn> for AbsoluteUri {
    fn from(value: &Urn) -> Self {
        AbsoluteUri::Urn(value.clone())
    }
}

impl TryFrom<&str> for AbsoluteUri {
    type Error = UriError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<&String> for AbsoluteUri {
    type Error = UriError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for AbsoluteUri {
    type Error = UriError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl FromStr for AbsoluteUri {
    type Err = UriError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl PartialEq<Uri> for AbsoluteUri {
    fn eq(&self, other: &Uri) -> bool {
        self.as_str() == other.as_str()
    }
}
impl PartialEq<&Uri> for AbsoluteUri {
    fn eq(&self, other: &&Uri) -> bool {
        self.as_str() == other.as_str()
    }
}
impl PartialEq<RelativeUri> for AbsoluteUri {
    fn eq(&self, other: &RelativeUri) -> bool {
        self.as_str() == other.as_str()
    }
}
impl PartialEq<&RelativeUri> for AbsoluteUri {
    fn eq(&self, other: &&RelativeUri) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<UriRef<'_>> for AbsoluteUri {
    fn eq(&self, other: &UriRef) -> bool {
        self.as_str() == other.as_str()
    }
}
impl PartialEq<&UriRef<'_>> for AbsoluteUri {
    fn eq(&self, other: &&UriRef<'_>) -> bool {
        self.as_str() == other.as_str()
    }
}

#[inherent]
impl<'a> TryIntoAbsoluteUri for UriRef<'a> {
    pub fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        match self {
            UriRef::Uri(uri) => uri.try_into_absolute_uri(),
            UriRef::AbsoluteUri(uri) => Ok(uri.clone()),
            UriRef::RelativeUri(rel) => Err(UriError::NotAbsolute(Uri::Relative(rel.clone()))),
        }
    }
}

impl TryIntoAbsoluteUri for String {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        AbsoluteUri::parse(&self)
    }
}

impl TryIntoAbsoluteUri for &str {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        AbsoluteUri::parse(self)
    }
}
#[inherent]
impl TryIntoAbsoluteUri for AbsoluteUri {
    pub fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        Ok(self)
    }
}

impl TryIntoAbsoluteUri for &Url {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        Ok(AbsoluteUri::Url(self.clone()))
    }
}
impl TryIntoAbsoluteUri for &Urn {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        Ok(AbsoluteUri::Urn(self.clone()))
    }
}
impl TryIntoAbsoluteUri for Url {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        Ok(AbsoluteUri::Url(self))
    }
}
impl TryIntoAbsoluteUri for Urn {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        Ok(AbsoluteUri::Urn(self))
    }
}
impl TryIntoAbsoluteUri for &String {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        AbsoluteUri::parse(self)
    }
}
impl TryIntoAbsoluteUri for &Uri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        match self {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url.clone())),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn.clone())),
            Uri::Relative(rel) => AbsoluteUri::parse(rel.as_str()),
        }
    }
}

#[inherent]
impl TryIntoAbsoluteUri for Uri {
    pub fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        match self {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url)),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn)),
            Uri::Relative(rel) => AbsoluteUri::parse(rel.as_str()),
        }
    }
}

impl TryIntoAbsoluteUri for &AbsoluteUri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        Ok(self.clone())
    }
}

/// A trait for possibly converting a type to an [`AbsoluteUri`].
///
pub trait TryIntoAbsoluteUri {
    /// Attempts to convert `self` into an [`AbsoluteUri`].
    ///
    /// # Errors
    /// Returns an error if the conversion fails due to the value not being an
    /// absolute in the sense of having a scheme and an authority (URL) or a namespace
    /// (URN).
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError>;
}

/// A single query parameter key value pair.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct QueryParameter<'a> {
    full: Cow<'a, str>,
    eq_index: Option<u32>,
}
impl<'a> QueryParameter<'a> {
    pub fn new(full: &'a str) -> Result<Self, OverflowError<usize, { u32::MAX as u64 }>> {
        usize_to_u32(full.len())?;
        let eq_index = full.find('=').map(|i| i.try_into().unwrap());
        let full = full.into();
        Ok(Self { full, eq_index })
    }

    /// Converts this `QueryParameter` into an owned version.
    #[must_use]
    pub fn into_owned(self) -> QueryParameter<'static> {
        QueryParameter {
            full: self.full.into_owned().into(),
            eq_index: self.eq_index,
        }
    }

    /// Returns the full query parameter string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.full.as_ref()
    }
    /// Returns the key, i.e. anything to the left of `'='`, of the query
    /// parameter.
    #[must_use]
    pub fn key(&self) -> &str {
        self.full[..self.eq_index().unwrap_or(self.full.len())].as_ref()
    }
    /// Returns the value, i.e. anything to the right of `'='`, of the query
    /// parameter, if it exists.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.eq_index().map(|i| &self.full[i + 1..])
    }

    fn eq_index(&self) -> Option<usize> {
        self.eq_index.map(|i| i as usize)
    }
}

impl<'a> TryFrom<&'a str> for QueryParameter<'a> {
    type Error = OverflowError<usize, { u32::MAX as u64 }>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[derive(Debug, Default)]
pub struct QueryParameters<'a> {
    query: Option<Split<'a, char>>,
}
impl<'a> QueryParameters<'a> {
    pub fn new(
        query: Option<&'a str>,
    ) -> Result<Self, OverflowError<usize, { usize::MAX as u64 }>> {
        let Some(query) = query else {
            return Ok(Self { query: None });
        };
        if query.len() > u32::MAX as usize {
            return Err(OverflowError(query.len()));
        }
        Ok(Self {
            query: Some(query.split('&')),
        })
    }
}

impl<'a> Iterator for QueryParameters<'a> {
    type Item = QueryParameter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.query
            .as_mut()
            .and_then(Iterator::next)
            .filter(|qp| !qp.is_empty())
            .map(QueryParameter::new)
            .map(Result::unwrap)
    }
}

/// A relative URI, with or without an authority.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RelativeUri {
    pub(super) value: String,
    pub(super) username_index: Option<u32>,
    pub(super) password_index: Option<u32>,
    pub(super) host_index: Option<u32>,
    pub(super) port_index: Option<u32>,
    pub(super) port: Option<u16>,
    pub(super) path_index: u32,
    pub(super) query_index: Option<u32>,
    pub(super) fragment_index: Option<u32>,
}

impl RelativeUri {
    /// Returns the `RelativeUri` as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns a new [`Components`] iterator over all components (scheme,
    /// username (URL), password (URL), host (URL) or namespace (URN), port
    /// (URL), path (URL) or namespace specific string (URN), query (URL or
    /// URN), and fragment (URL or URN)) of this `RelativeUri`.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::{ Uri, Component, PathSegment, QueryParameter };
    /// let s = "//user:password@example.com/path/to/file/?query=str#fragment";
    /// let uri = Uri::parse(s).unwrap();
    /// let uri = AbsoluteUri::parse(s).unwrap();
    /// let components = vec![
    ///     Component::Username("user".into()),
    ///     Component::Password("password".into()),
    ///     Component::Host("example.com".into()),
    ///     Component::PathSegment(PathSegment::Root),
    ///     Component::PathSegment(PathSegment::Normal("path".into())),
    ///     Component::PathSegment(PathSegment::Normal("to".into())),
    ///     Component::PathSegment(PathSegment::Normal("file".into())),
    ///     Component::QueryParameter(QueryParameter::new("query=str").unwrap()),
    ///     Component::Fragment("fragment".into()),
    /// ];
    /// assert_eq!(uri.components().collect::<Vec<_>>(), components);
    #[must_use]
    pub fn components(&self) -> Components {
        Components::from_relative_uri(self)
    }

    /// Returns a new [`PathSegments`] iterator over all segments of this
    /// `RelativeUri`'s path.
    /// # Example
    /// ```rust
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file");
    /// let uri = uri.as_relative_uri().unwrap();
    /// assert_eq!(uri.path_segments().collect::<Vec<_>>(), vec!["path", "to", "file"]);
    /// ```
    #[must_use]
    pub fn path_segments(&self) -> PathSegments {
        PathSegments::from(self.path())
    }

    /// returns a new `Uri` that is the result of resolving the given
    /// reference against this `RelativeUri`.
    ///
    /// See [RFC3986, Section
    /// 5.2.2](https://tools.ietf.org/html/rfc3986#section-5.2.2).
    pub fn resolve(&self, reference: &impl AsUriRef) -> Result<Uri, UriError> {
        let reference = reference.as_uri_ref();

        // if the reference has a scheme, normalize the path and return
        if let Ok(mut uri) = reference.try_into_absolute_uri() {
            uri.normalize_path();
            return Ok(uri.into());
        }

        // safety: urls and urns will get processed in the match above
        let reference = reference.as_relative_uri().unwrap();

        if let Some(authority) = reference.authority() {
            let mut uri: Uri = self.clone().into();
            uri.set_authority_or_namespace(&authority)?;
            uri.set_query(reference.query())?;
            uri.set_fragment(reference.fragment())?;
            uri.set_path_or_nss(&reference.path_normalized())?;
            return Ok(uri);
        }

        if reference.path().is_empty() {
            let mut uri: Uri = self.clone().into();
            if let Some(fragment) = reference.fragment() {
                if !fragment.is_empty() {
                    uri.set_fragment(Some(fragment))?;
                }
            }
            if let Some(query) = reference.query() {
                if !query.is_empty() {
                    uri.set_query(Some(query))?;
                }
            }
            return Ok(uri);
        }

        let mut uri: Uri = self.clone().into();
        uri.set_query(reference.query())?;
        uri.set_fragment(reference.fragment())?;

        if reference.path().starts_with('/') {
            uri.set_path_or_nss(&reference.path_normalized())?;
            return Ok(uri);
        }
        let base = self.base_path();
        let base = merge(base, reference.path());
        uri.set_path_or_nss(&base)?;

        Ok(uri)
    }

    /// Returns a [`PathSegments`] iterator over the base path segments,
    /// essentially every segment except the last.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    /// let relative_uri = uri.as_relative_uri().unwrap();
    /// let segments = relative_uri.base_path_segments().collect::<Vec<_>>();
    /// assert_eq!(&segments, &["", "path", "to"]);
    #[must_use]
    pub fn base_path_segments(&self) -> PathSegments<'_> {
        let mut segments = self.path_segments();
        segments.base_only = true;
        segments
    }

    /// Returns the base path of the `RelativeUri`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap()
    ///     .as_relative_uri().unwrap();
    ///
    /// assert_eq!(uri.base_path(), "/path/to");
    /// ```
    #[must_use]
    pub fn base_path(&self) -> &str {
        self.path().rsplit_once('/').map_or("", |(a, _)| a)
    }

    /// Returns the path segment of the `RelativeUri`.
    #[must_use]
    pub fn path(&self) -> &str {
        let end = self
            .query_index()
            .or(self.fragment_index())
            .unwrap_or(self.value.len());
        &self.value[self.path_index()..end]
    }
    /// returns the username portion of the `RelativeUri` if it exists.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::{ Uri, RelativeUri };
    /// let uri = Uri::parse("//user:pass@host/path?query#fragment");
    /// let relative_uri = uri.relative_uri().unwrap();
    /// assert_eq!(relative_uri.username(), Some("user"));
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        let start = self.username_index()?;
        let end = self.username_end_index()?;
        Some(&self.value[start..end])
    }
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        let start = self.password_index()? + 1;
        let end = self.host_index().unwrap_or(self.path_index());
        Some(&self.value[start..end])
    }

    fn username_end_index(&self) -> Option<usize> {
        self.username_index?;
        self.password_index()
            .or(self.host_index())
            .unwrap_or(self.path_index())
            .into()
    }

    /// Returns the path of the `RelativeUri` if it exists.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        let fragment_index = self.fragment_index()?;
        if fragment_index + 1 == self.len() {
            return Some("");
        }

        Some(&self.value[fragment_index + 1..])
    }

    /// Returns the query string segment of the `RelativeUri`, if it exists.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        let query_index = self.query_index()?;
        if query_index + 1 == self.len() {
            return Some("");
        }
        let last = self.fragment_index().unwrap_or(self.len());
        Some(&self.value[query_index + 1..last])
    }

    /// Returns an [`Iterator`] of [`QueryParameter`] of this `RelativeUri`.
    #[must_use]
    pub fn query_parameters(&self) -> QueryParameters<'_> {
        QueryParameters::new(self.query()).unwrap()
    }

    /// Returns `true` if this `Uri` has a query component.
    #[must_use]
    pub fn has_query(&self) -> bool {
        self.query().is_some()
    }

    /// Returns `true` if this `Uri` has a fragment component.
    #[must_use]
    pub fn has_fragment(&self) -> bool {
        self.query().is_some()
    }

    fn has_path(&self) -> bool {
        !self.path().is_empty()
    }

    /// Sets the query string portion of the `RelativeUri` and returns the
    /// previous query, if it existed.
    ///
    pub fn set_query(&mut self, query: Option<&str>) -> Result<Option<String>, RelativeUriError> {
        let existing_query = self.query().map(ToString::to_string);
        let cap = self.len() - existing_query.as_ref().map(String::len).unwrap_or_default()
            + query.map(str::len).unwrap_or_default();
        let mut buf = String::with_capacity(cap);
        let username_index = write::username(&mut buf, self.username())?;
        let password_index = write::password(&mut buf, self.password())?;
        let host_index = write::host(&mut buf, self.host())?;
        let port_index = write::port(&mut buf, self.port_str())?;
        let path_index = write::path(&mut buf, self.path())?;
        let query_index = write::query(
            &mut buf,
            encode::query(query),
            self.has_authority(),
            self.has_path(),
        )?;
        let fragment_index: Option<u32> = write::fragment(&mut buf, self.fragment())?;
        self.value = buf;
        self.username_index = username_index;
        self.password_index = password_index;
        self.host_index = host_index;
        self.port_index = port_index;
        self.path_index = path_index;
        self.query_index = query_index;
        self.fragment_index = fragment_index;
        Ok(existing_query)
    }

    /// Sets the path of the `RelativeUri` and returns the previous path.
    ///
    /// Note, fragments are left intact. Use `set_fragment` to change the fragment.
    pub fn set_path(&mut self, path: &str) -> Result<String, RelativeUriError> {
        let existing_path = self.path().to_string();
        let mut buf = String::with_capacity(self.len() - existing_path.len() + path.len());
        let username_index = write::username(&mut buf, self.username())?;
        let password_index = write::password(&mut buf, self.password())?;
        let host_index = write::host(&mut buf, self.host())?;
        let port_index = write::port(&mut buf, self.port_str())?;
        let path_index = write::path(&mut buf, encode::path(path))?;
        let query_index = write::query(
            &mut buf,
            self.query(),
            self.has_authority(),
            self.has_path(),
        )?;
        let fragment_index: Option<u32> = write::fragment(&mut buf, self.fragment())?;
        self.value = buf;
        self.username_index = username_index;
        self.password_index = password_index;
        self.host_index = host_index;
        self.port_index = port_index;
        self.path_index = path_index;
        self.query_index = query_index;
        self.fragment_index = fragment_index;
        Ok(existing_path)
    }

    /// Sets the fragment of the `RelativeUri` and returns the previous fragment, if
    /// present.
    pub fn set_fragment(
        &mut self,
        fragment: Option<&str>,
    ) -> Result<Option<String>, RelativeUriError> {
        let existing_fragment = self.fragment().map(ToString::to_string);
        let mut buf = String::with_capacity(
            self.len()
                - existing_fragment
                    .as_ref()
                    .map(String::len)
                    .unwrap_or_default()
                + fragment.unwrap_or_default().len(),
        );
        let username_index = write::username(&mut buf, self.username())?;
        let password_index = write::password(&mut buf, self.password())?;
        let host_index = write::host(&mut buf, self.host())?;
        let port_index = write::port(&mut buf, self.port_str())?;
        let path_index: u32 = write::path(&mut buf, self.path())?;
        let query_index: Option<u32> = write::query(
            &mut buf,
            self.query(),
            self.has_authority(),
            self.has_path(),
        )?;
        let fragment_index: Option<u32> = write::fragment(&mut buf, encode::fragment(fragment))?;
        self.value = buf;
        self.username_index = username_index;
        self.password_index = password_index;
        self.host_index = host_index;
        self.port_index = port_index;
        self.path_index = path_index;
        self.query_index = query_index;
        self.fragment_index = fragment_index;
        Ok(existing_fragment)
    }

    #[must_use]
    /// Returns the authority of the `RelativeUri` if it exists.
    ///
    /// A relative URI may have an authority if it starts starts with `"//"`.
    pub fn authority(&self) -> Option<Authority> {
        let host_index = self.host_index()?;
        Some(Authority {
            value: Cow::Borrowed(&self.value[host_index..self.path_index()]),
            username_index: self.username_index,
            password_index: self.password_index,
            host_index: self.host_index,
            port_index: self.port_index,
            port: self.port,
        })
    }
    /// Returns the username if it exists.
    #[must_use]
    pub fn host(&self) -> Option<&str> {
        let mut start = self.host_index()?;
        if self.has_username() || self.has_password() {
            start += 1;
        }
        let end = self.port_index().unwrap_or_else(|| self.path_index());
        Some(&self.value[start..end])
    }

    /// Returns the port  if it exists.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Sets the authority and returns the previous value as an [`Authority`], if it existed.
    pub fn set_authority<'a>(
        &'a mut self,
        authority: Option<&str>,
    ) -> Result<Option<Authority<'a>>, UriError> {
        let existing_authority = self.authority().map(|a| a.into_owned());
        let new = authority
            .map(parse::authority)
            .transpose()?
            .unwrap_or_default();
        let mut buf = String::with_capacity(
            self.len()
                - existing_authority
                    .as_deref()
                    .map(str::len)
                    .unwrap_or_default()
                + new.len(),
        );
        let username_index = write::username(&mut buf, new.username())?;
        let password_index = write::password(&mut buf, new.password())?;
        let host_index = write::host(&mut buf, new.host())?;
        let port_index = write::port(&mut buf, new.port_str())?;
        let path_index: u32 = write::path(&mut buf, self.path())?;
        let query_index: Option<u32> = write::query(
            &mut buf,
            self.query(),
            self.has_authority(),
            self.has_path(),
        )?;
        let fragment_index = write::fragment(&mut buf, self.fragment())?;
        self.value = buf;
        self.username_index = username_index;
        self.password_index = password_index;
        self.host_index = host_index;
        self.port_index = port_index;
        self.path_index = path_index;
        self.query_index = query_index;
        self.fragment_index = fragment_index;
        Ok(existing_authority)
    }

    #[must_use]
    pub fn has_authority(&self) -> bool {
        self.path_index() > 2
    }

    #[must_use]
    pub fn has_username(&self) -> bool {
        self.username_index.is_some()
    }

    #[must_use]
    pub fn has_password(&self) -> bool {
        self.password_index.is_some()
    }

    /// Returns `true` if the `RelativeUri` has a host.
    ///
    /// # Example
    /// ```
    /// use grill::Uri;
    ///
    /// let uri = Uri::parse("//example.com").unwrap()
    ///     .as_relative_uri()
    ///     .unwrap();
    /// assert!(uri.has_host());
    /// ```
    #[must_use]
    pub fn has_host(&self) -> bool {
        self.host_index.is_some()
    }

    /// Returns `true` if the `RelativeUri` has a port.
    #[must_use]
    pub fn has_port(&self) -> bool {
        self.port_index.is_some()
    }

    pub(crate) fn authority_str(&self) -> Option<&str> {
        let start = self.username_index().or(self.host_index())?;
        Some(&self.value[start..self.path_index()])
    }

    fn path_index(&self) -> usize {
        self.path_index as usize
    }

    fn fragment_index(&self) -> Option<usize> {
        self.fragment_index.map(|idx| idx as usize)
    }

    fn query_index(&self) -> Option<usize> {
        self.query_index.map(|idx| idx as usize)
    }

    fn username_index(&self) -> Option<usize> {
        self.username_index.map(|idx| idx as usize)
    }

    fn host_index(&self) -> Option<usize> {
        self.host_index.map(|idx| idx as usize)
    }

    fn port_index(&self) -> Option<usize> {
        self.port_index.map(|idx| idx as usize)
    }

    fn password_index(&self) -> Option<usize> {
        self.password_index.map(|idx| idx as usize)
    }

    fn port_str(&self) -> Option<&str> {
        self.port_index()
            .map(|idx| &self.value[idx + 1..self.path_index()])
    }

    /// Returns the path normalized by removing dot segments, i.e. `'.'`, `'..'`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let mut uri = Uri::parse("https://example.com/./foo/../bar").unwrap()
    ///     .as_relative_uri()
    ///     .unwrap();
    /// let normalized = uri.path_normalized();
    /// assert_eq!(normalized, "/bar");
    /// ```
    #[must_use]
    pub fn path_normalized(&self) -> Cow<'_, str> {
        normalize(self.path())
    }

    /// Normalizes the path by removing dot segments, i.e. `'.'`, `'..'`.
    ///
    /// This method mutates the `RelativeUri` in place. If that is not desired, use
    /// [`path_normalized`](RelativeUri::path_normalized) instead.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let mut uri = Uri::parse("https://example.com/./foo/../bar").unwrap();
    /// uri.normalize_path();
    /// assert_eq!(uri.path(), "/bar");
    /// ```
    pub fn normalize_path(&mut self) {
        let normalized = normalize(self.path()).to_string();
        self.set_path(&normalized).unwrap();
    }
}

impl Index<usize> for RelativeUri {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index..]
    }
}

impl Display for RelativeUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl From<RelativeUri> for Uri {
    fn from(value: RelativeUri) -> Self {
        Uri::Relative(value)
    }
}

impl From<RelativeUri> for String {
    fn from(value: RelativeUri) -> Self {
        value.to_string()
    }
}

impl From<&RelativeUri> for String {
    fn from(value: &RelativeUri) -> Self {
        value.to_string()
    }
}

impl Deref for RelativeUri {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.value.as_str()
    }
}

/// A borrowed [`Uri`], [`AbsoluteUri`], or [`RelativeUri`].
#[derive(Clone, Copy, Debug)]
pub enum UriRef<'a> {
    /// A borrowed [`Uri`].
    Uri(&'a Uri),
    /// A borrowed [`AbsoluteUri`].
    AbsoluteUri(&'a AbsoluteUri),
    /// A borrowed [`RelativeUri`].
    RelativeUri(&'a RelativeUri),
}

impl<'a> UriRef<'a> {
    /// Returns the string representation of the URI reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            UriRef::Uri(uri) => uri.as_str(),
            UriRef::AbsoluteUri(uri) => uri.as_str(),
            UriRef::RelativeUri(rel) => rel.as_str(),
        }
    }

    /// Returns a reference to the underlying [`Url`] if `self` is either
    /// [`UriRef::Uri`] or [`UriRef::AbsoluteUri`] and the respective [`Uri`] or
    /// [`AbsoluteUri`] is a [`Url`](`url::Url`).
    #[must_use]
    pub fn as_url(&self) -> Option<&'a Url> {
        match self {
            UriRef::Uri(uri) => uri.as_url(),
            UriRef::AbsoluteUri(uri) => uri.as_url(),
            UriRef::RelativeUri(_) => None,
        }
    }

    /// Returns a reference to the underlying [`Urn`] if `self` is either
    /// [`UriRef::Uri`] or [`UriRef::AbsoluteUri`] and the respective [`Uri`] or
    /// [`AbsoluteUri`] is a [`Urn`].
    #[must_use]
    pub fn as_urn(&self) -> Option<&'a Urn> {
        match self {
            UriRef::Uri(uri) => uri.as_urn(),
            UriRef::AbsoluteUri(uri) => uri.as_urn(),
            UriRef::RelativeUri(_) => None,
        }
    }
    /// Returns a reference to the underlying [`RelativeUri`] if `self` is a
    /// [`UriRef::Uri`] with an underlying [`RelativeUri`] or
    /// [`UriRef::RelativeUri`].
    #[must_use]
    pub fn as_relative_uri(&self) -> Option<&'a RelativeUri> {
        match self {
            UriRef::Uri(uri) => uri.as_relative_uri(),
            UriRef::RelativeUri(rel) => Some(*rel),
            UriRef::AbsoluteUri(_) => None,
        }
    }
    /// Returns `true` if this underlying `Uri` is a [`Url`]
    #[must_use]
    pub fn is_url(&self) -> bool {
        match self {
            UriRef::Uri(uri) => uri.is_url(),
            UriRef::AbsoluteUri(uri) => uri.is_url(),
            UriRef::RelativeUri(_) => false,
        }
    }

    /// Returns `true` if this underlying `Uri` is a [`Urn`]
    #[must_use]
    pub fn is_urn(&self) -> bool {
        match self {
            UriRef::Uri(uri) => uri.is_urn(),
            UriRef::AbsoluteUri(uri) => uri.is_urn(),
            UriRef::RelativeUri(_) => false,
        }
    }

    /// Returns `true` if the URI has an authority or namespace.
    #[must_use]
    pub fn has_authority(&self) -> bool {
        match self {
            UriRef::RelativeUri(rel) => rel.has_authority(),
            _ => true,
        }
    }

    /// Returns `true` if the this `Uri` has a scheme.
    #[must_use]
    pub fn has_scheme(&self) -> bool {
        return self.scheme().is_some();
    }

    /// Returns the scheme of this `Uri` if it exists.
    #[must_use]
    pub fn scheme(&self) -> Option<&str> {
        match self {
            UriRef::Uri(uri) => uri.scheme(),
            UriRef::AbsoluteUri(uri) => Some(uri.scheme()),
            UriRef::RelativeUri(_) => None,
        }
    }

    /// Returns the path (if this `UriRef` is a [`Url`]) or namespace specific
    /// string (if this `Uri` is a [`Urn`])
    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            UriRef::Uri(uri) => uri.path_or_nss(),
            UriRef::AbsoluteUri(uri) => uri.path_or_nss(),
            UriRef::RelativeUri(uri) => uri.path(),
        }
    }
    /// Returns the base path of the `UriRef`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap().as_uri_ref();
    ///
    /// assert_eq!(uri.base_path(), "/path/to");
    /// ```
    #[must_use]
    pub fn base_path(&self) -> &str {
        self.path_or_nss().rsplit_once('/').map_or("", |(a, _)| a)
    }

    /// Returns the authority (if this `Uri` is a [`Url`]) or namespace (if this
    /// `Uri` is a [`Urn`])
    #[must_use]
    pub fn authority_or_namespace(&self) -> Option<Cow<'_, str>> {
        match self {
            UriRef::Uri(uri) => uri.authority_or_namespace(),
            UriRef::AbsoluteUri(uri) => uri.authority_or_namespace(),
            UriRef::RelativeUri(_) => None,
        }
    }
    /// Returns the query component if it exists.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        match self {
            UriRef::Uri(uri) => uri.fragment(),
            UriRef::AbsoluteUri(uri) => uri.fragment(),
            UriRef::RelativeUri(uri) => uri.fragment(),
        }
    }
    /// Returns the query component if it exists.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        match self {
            UriRef::Uri(uri) => uri.query(),
            UriRef::AbsoluteUri(uri) => uri.query(),
            UriRef::RelativeUri(uri) => uri.query(),
        }
    }
    /// Returns an [`Iterator`] of [`QueryParameter`] of this `UriRef`.
    #[must_use]
    pub fn query_parameters(&self) -> QueryParameters<'_> {
        QueryParameters::new(self.query()).unwrap()
    }

    /// Returns the path [`normalized`](super::normalize) by removing dot segments, i.e. `'.'`,
    /// `'..'`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("https://example.com/./foo/../bar").unwrap();
    /// let uri_ref = uri.as_uri_ref();
    /// let normalized = uri_ref.path_normalized();
    /// assert_eq!(normalized, "/bar");
    /// ```
    /// [`normalize`]:super::normalize
    #[must_use]
    pub fn path_normalized(&self) -> Cow<'_, str> {
        normalize(self.path_or_nss())
    }
}

impl Deref for UriRef<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

/// A trait which enables borrowing a [`Uri`], [`AbsoluteUri`], or
/// [`RelativeUri`] as a singular type.
pub trait AsUriRef {
    /// Borrows `self` as a [`UriRef`].
    fn as_uri_ref(&self) -> UriRef<'_>;
}

#[inherent]
impl AsUriRef for AbsoluteUri {
    #[must_use]
    pub fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::AbsoluteUri(self)
    }
}

impl AsUriRef for &AbsoluteUri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::AbsoluteUri(self)
    }
}

#[inherent]
impl AsUriRef for Uri {
    #[must_use]
    pub fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::Uri(self)
    }
}

impl AsUriRef for &Uri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::Uri(self)
    }
}
impl AsUriRef for &RelativeUri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::RelativeUri(self)
    }
}

#[inherent]
impl AsUriRef for RelativeUri {
    #[must_use]
    pub fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::RelativeUri(self)
    }
}

/// A single component of a URI (i.e. scheme, username, password, path, query,
/// fragment).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Component<'a> {
    /// The scheme of the URI, e.g., `"https"` in `"https://example.com"`.
    Scheme(Cow<'a, str>),

    /// The username of the URI, e.g., `"username"` in
    /// `"https://username:password@example.com"`.
    ///
    /// # Note:
    /// This is not applicable to URNs
    Username(Cow<'a, str>),

    /// The passsword of the URI, e.g., `"password"` in
    /// `"https://username:password@example.com"`.
    ///
    /// # Note:
    /// This is not applicable to URNs
    Password(Cow<'a, str>),

    /// The host or namespace of the URI, e.g., `"example.com"` in
    /// `"https://example.com"`.
    Host(Cow<'a, str>),

    /// The namespace of a URN. This is not applicable to URLs.
    Namespace(Cow<'a, str>),

    /// The namespace specific string (NSS) for a URN. Not applicable to URLs.
    Nss(Cow<'a, str>),

    /// The segment of the URI's path, e.g., `"/path"` in `"https://example.com/path/to/resource"`.
    PathSegment(PathSegment<'a>),

    /// The a query paramater of the URI, e.g., `"query=str"` in
    /// `"https://example.com/?query=str"`.
    QueryParameter(QueryParameter<'a>),

    /// The fragment of the URI, e.g., `"fragment"` in .
    /// `"https://example.com/#fragment"`
    Fragment(Cow<'a, str>),
}

impl<'a> Component<'a> {
    /// Percent decodes the component.
    ///
    /// # Errors
    /// Returns a [`std::str::Utf8Error`] if the percent decoded bytes are not
    /// valid UTF-8
    pub fn decode(&'a self) -> Result<Cow<'a, str>, std::str::Utf8Error> {
        percent_decode(self.as_bytes()).decode_utf8()
    }

    /// Percent decodes the component, lossily.
    ///
    /// Invalid UTF-8 percent-encoded byte sequences will be replaced `�` (`U+FFFD`),
    /// the replacement character.
    #[must_use]
    pub fn decode_lossy(&'a self) -> Cow<'a, str> {
        percent_decode(self.as_bytes()).decode_utf8_lossy()
    }

    /// Converts the component into a byte slice.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    #[must_use]
    /// Converts the component into a string.
    pub fn as_str(&self) -> &str {
        match self {
            Component::Scheme(s)
            | Component::Username(s)
            | Component::Password(s)
            | Component::Host(s)
            | Component::Namespace(s)
            | Component::Nss(s)
            | Component::Fragment(s) => s,
            Component::PathSegment(s) => s.as_str(),
            Component::QueryParameter(s) => s.as_str(),
        }
    }
}

impl<'a> Deref for Component<'a> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl PartialEq<str> for Component<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<str> for &Component<'_> {
    fn eq(&self, other: &str) -> bool {
        *self == other
    }
}
impl PartialEq<String> for &Component<'_> {
    fn eq(&self, other: &String) -> bool {
        self == other
    }
}
impl<'a> PartialEq<Component<'a>> for str {
    fn eq(&self, other: &Component) -> bool {
        other.eq(self)
    }
}
impl<'a> PartialEq<&Component<'a>> for str {
    fn eq(&self, other: &&Component) -> bool {
        other.eq(self)
    }
}
impl<'a> PartialEq<&Component<'a>> for String {
    fn eq(&self, other: &&Component) -> bool {
        other.eq(self)
    }
}

/// An [`Iterator`] of [`Component`]s of a URI.
pub struct Components<'a> {
    scheme: Option<Cow<'a, str>>,
    username: Option<Cow<'a, str>>,
    password: Option<Cow<'a, str>>,
    host: Option<Cow<'a, str>>,
    path: PathSegments<'a>,
    query: QueryParameters<'a>,
    fragment: Option<Cow<'a, str>>,
}

impl<'a> Components<'a> {
    /// Creates a new `Components` iterator from an [`AbsoluteUri`].
    #[must_use]
    pub fn from_absolute_uri(uri: &'a AbsoluteUri) -> Self {
        match uri {
            AbsoluteUri::Url(url) => Self::from_url(url),
            AbsoluteUri::Urn(urn) => Self::from_urn(urn),
        }
    }
    /// Creates a new `Components` iterator from a [`Uri`].
    #[must_use]
    pub fn from_uri(uri: &'a Uri) -> Self {
        match uri {
            Uri::Url(url) => Self::from_url(url),
            Uri::Urn(urn) => Self::from_urn(urn),
            Uri::Relative(rel) => Self::from_relative_uri(rel),
        }
    }

    /// Creates a new `Components` iterator from a [`RelativeUri`].
    #[must_use]
    pub fn from_relative_uri(rel: &'a RelativeUri) -> Self {
        Self {
            scheme: None,
            username: rel.username().map(Into::into),
            password: rel.password().map(Into::into),
            host: rel.host().map(Into::into),
            path: rel.path_segments(),
            query: rel.query_parameters(),
            fragment: rel.fragment().map(Into::into),
        }
    }

    /// Creates a new `Components` iterator from a [`Urn`].
    #[must_use]
    pub fn from_urn(urn: &'a Urn) -> Self {
        Self {
            scheme: Some("urn".into()),
            username: None,
            password: None,
            host: Some(urn.nid().into()),
            path: PathSegments::default(),
            query: QueryParameters::new(urn.q_component()).unwrap(),
            fragment: urn.f_component().map(Into::into),
        }
    }
    /// Creates a new `Components` iterator from a [`Url`].
    #[must_use]
    pub fn from_url(url: &'a Url) -> Self {
        Self {
            scheme: Some(url.scheme().into()),
            host: url.host().map(|h| h.to_string().into()),
            username: Some(url.username())
                .filter(|s| !s.is_empty())
                .map(Into::into),
            password: url.password().map(Into::into),
            path: PathSegments::new(url.path()),
            query: QueryParameters::new(url.query()).unwrap(),
            fragment: url.fragment().map(Into::into),
        }
    }
}

impl<'a> Iterator for Components<'a> {
    type Item = Component<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(scheme) = self.scheme.take() {
            return Some(Component::Scheme(scheme));
        }
        if let Some(username) = self.username.take() {
            return Some(Component::Username(username));
        }
        if let Some(password) = self.password.take() {
            return Some(Component::Password(password));
        }
        if let Some(host) = self.host.take() {
            return Some(Component::Host(host));
        }
        if let Some(path) = self.path.next() {
            return Some(Component::PathSegment(path));
        }
        if let Some(query) = self.query.next() {
            return Some(Component::QueryParameter(query));
        }
        if let Some(fragment) = self.fragment.take() {
            return Some(Component::Fragment(fragment));
        }
        None
    }
}

/// A relative or absolute URI in the form of a [`Url`], [`Urn`], or
/// [`RelativeUri`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Uri {
    /// Uniform Resource Locator (URL)
    Url(Url),
    /// Uniform Resource Name (URN)
    Urn(Urn),
    /// Relative URI
    Relative(RelativeUri),
}

impl Default for Uri {
    fn default() -> Self {
        Self::Relative(RelativeUri::default())
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Uri::Url(url) => Display::fmt(url, f),
            Uri::Urn(urn) => Display::fmt(urn, f),
            Uri::Relative(rel) => Display::fmt(rel, f),
        }
    }
}

impl Uri {
    /// Attempts to parse `value` as a `Uri`.
    ///
    /// # Errors
    /// Returns `UriParseError` if `value` fails to parse as a `Uri`
    pub fn parse(value: &str) -> Result<Self, UriError> {
        parse::uri(value)
    }

    /// returns a new `Uri` that is the result of resolving the given reference
    /// against this `Uri`.
    ///
    /// See [RFC3986, Section
    /// 5.2.2](https://tools.ietf.org/html/rfc3986#section-5.2.2).
    pub fn resolve(&self, reference: &impl AsUriRef) -> Result<Uri, UriError> {
        let reference = reference.as_uri_ref();

        // if the reference has a scheme, normalize the path and return
        if let Ok(mut uri) = reference.try_into_absolute_uri() {
            uri.normalize_path();
            return Ok(uri.into());
        }

        // safety: urls and urns will get processed in the match above
        let reference = reference.as_relative_uri().unwrap();

        if let Some(authority) = reference.authority() {
            let mut uri = self.clone();
            uri.set_authority_or_namespace(&authority)?;
            uri.set_query(reference.query())?;
            uri.set_fragment(reference.fragment())?;
            uri.set_path_or_nss(&reference.path_normalized())?;
            return Ok(uri);
        }

        if reference.path().is_empty() {
            let mut uri = self.clone();
            if let Some(fragment) = reference.fragment() {
                if !fragment.is_empty() {
                    uri.set_fragment(Some(fragment))?;
                }
            }
            if let Some(query) = reference.query() {
                if !query.is_empty() {
                    uri.set_query(Some(query))?;
                }
            }
            return Ok(uri);
        }

        let mut uri = self.clone();
        uri.set_query(reference.query())?;
        uri.set_fragment(reference.fragment())?;

        if reference.path().starts_with('/') {
            uri.set_path_or_nss(&reference.path_normalized())?;
            return Ok(uri);
        }
        let base = self.base_path();
        let base = merge(base, reference.path());
        uri.set_path_or_nss(&base)?;

        Ok(uri)
    }

    /// Returns a new [`Components`] iterator over all components (scheme,
    /// username (URL), password (URL), host (URL) or namespace (URN), port
    /// (URL), path (URL) or namespace specific string (URN), query (URL or
    /// URN), and fragment (URL or URN)) of this `RelativeUri`.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::{ Uri, Component, PathSegment, QueryParameter };
    /// let s = "//user:password@example.com/path/to/file/?query=str#fragment";
    /// let uri = Uri::parse(s).unwrap();
    /// let uri = AbsoluteUri::parse(s).unwrap();
    /// let components = vec![
    ///     Component::Username("user".into()),
    ///     Component::Password("password".into()),
    ///     Component::Host("example.com".into()),
    ///     Component::PathSegment(PathSegment::Root),
    ///     Component::PathSegment(PathSegment::Normal("path".into())),
    ///     Component::PathSegment(PathSegment::Normal("to".into())),
    ///     Component::PathSegment(PathSegment::Normal("file".into())),
    ///     Component::QueryParameter(QueryParameter::new("query=str").unwrap()),
    ///     Component::Fragment("fragment".into()),
    /// ];
    /// assert_eq!(uri.components().collect::<Vec<_>>(), components);
    #[must_use]
    pub fn components(&self) -> Components {
        Components::from_uri(self)
    }
    /// Returns a new [`PathSegments`] iterator over all segments of this
    /// `Uri`'s path.
    /// # Example
    /// ```rust
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("https://example.com/path/to/file");
    /// assert_eq!(uri.path_segments().collect::<Vec<_>>(), vec!["path", "to", "file"]);
    /// ```
    #[must_use]
    pub fn path_segments(&self) -> PathSegments {
        PathSegments::from(self.path_or_nss())
    }

    /// Returns the base path, that is all path segments except for the last.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    ///
    /// assert_eq!(uri.base_path(), "/path/to/");
    /// let uri = Uri::parse("/file").unwrap();
    /// assert_eq!(uri.base_path(), "/");
    /// ```
    #[must_use]
    pub fn base_path(&self) -> &str {
        self.path_or_nss()
            .rfind('/')
            .map_or("", |idx| &self.path_or_nss()[..=idx])
    }

    /// Returns a [`PathSegments`] iterator over the base path segments,
    /// essentially every segment except the last.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    /// assert_eq!(uri.base_path_segments().collect::<Vec<_>>(), vec!["path", "to"]);
    #[must_use]
    pub fn base_path_segments(&self) -> PathSegments<'_> {
        let mut segments = self.path_segments();
        segments.base_only = true;
        segments
    }

    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        match self {
            Uri::Url(url) => url.fragment(),
            Uri::Urn(urn) => urn.f_component(),
            Uri::Relative(rel) => rel.fragment(),
        }
    }

    /// Sets the fragment component of the [`Url`] or [`Urn`] and returns the
    /// previous value, if it exists.
    ///
    /// # Errors
    /// Returns [`urn::Error`](`urn::Error`) if the `AbsoluteUri` is a
    /// [`Urn`](`urn::Urn`) and the fragment and the fragment fails validation.
    pub fn set_fragment(&mut self, mut fragment: Option<&str>) -> Result<Option<String>, UriError> {
        if let Some(frag) = &fragment {
            if frag.is_empty() {
                fragment = None;
            }
        }
        match self {
            Uri::Url(url) => Ok(set::url::fragment(url, fragment)),
            Uri::Urn(urn) => set::urn::fragment(urn, fragment),
            Uri::Relative(rel) => Ok(rel.set_fragment(fragment)?),
        }
    }

    #[must_use]
    pub fn is_fragment_empty_or_none(&self) -> bool {
        self.fragment().map_or(true, |f| f.trim().is_empty())
    }

    /// Sets the query component of the [`Url`] or [`Urn`] and returns the
    /// previous query, if it existed.
    pub fn set_query(&mut self, query: Option<&str>) -> Result<Option<String>, UriError> {
        let prev = self.query().map(ToString::to_string);
        match self {
            Self::Url(url) => {
                url.set_query(query);
                Ok(prev)
            }
            Self::Urn(urn) => {
                urn.set_q_component(query)?;
                Ok(prev)
            }
            Uri::Relative(rel) => Ok(rel.set_query(query)?),
        }
    }
    /// Returns the query component of the [`Url`] or [`Urn`].
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        match self {
            Uri::Url(url) => url.query(),
            Uri::Urn(urn) => urn.q_component(),
            Uri::Relative(rel) => rel.query(),
        }
    }
    /// Returns an [`Iterator`] of [`QueryParameter`] of this `Uri`.
    #[must_use]
    pub fn query_parameters(&self) -> QueryParameters<'_> {
        QueryParameters::new(self.query()).unwrap()
    }
    /// Returns the namespace if the absolute uri is [`Urn`], otherwise returns
    /// the authority string for a [`Url`] or [`RelativeUri`].
    #[must_use]
    pub fn authority_or_namespace(&self) -> Option<Cow<'_, str>> {
        match self {
            Uri::Url(url) => get::url::authority(url).map(Into::into),
            Uri::Urn(urn) => Some(Cow::Borrowed(urn.nid())),
            Uri::Relative(rel) => rel.authority_str().map(Cow::Borrowed),
        }
    }

    pub fn set_authority_or_namespace(
        &mut self,
        authority_or_namespace: &str,
    ) -> Result<Option<String>, UriError> {
        match self {
            Uri::Url(url) => set::url::authority(url, authority_or_namespace),
            Uri::Urn(urn) => set::urn::namespace(urn, authority_or_namespace),
            Uri::Relative(rel) => Ok(rel
                .set_authority(Some(authority_or_namespace))?
                .map(|a| a.to_string())),
        }
    }

    /// Returns the path (if this `Uri` is a [`Url`]) or namespace specific
    /// string (if this `Uri` is a [`Urn`])
    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            Self::Url(url) => url.path(),
            Self::Urn(urn) => urn.nss(),
            Self::Relative(rel) => rel.path(),
        }
    }

    /// Returns the path normalized by removing dot segments, i.e. `'.'`, `'..'`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let mut uri = Uri::parse("https://example.com/./foo/../bar").unwrap();
    /// let normalized = uri.path_normalized();
    /// assert_eq!(normalized, "/bar");
    /// ```
    #[must_use]
    pub fn path_normalized(&self) -> Cow<'_, str> {
        normalize(self.path_or_nss())
    }

    /// Normalizes the path by removing dot segments, i.e. `'.'`, `'..'`.
    ///
    /// This method mutates the `Uri` in place. If that is not desired, use
    /// [`path_normalized`](Uri::path_normalized) instead.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let mut uri = Uri::parse("https://example.com/./foo/../bar").unwrap();
    /// uri.normalize_path();
    /// assert_eq!(uri.path_or_nss(), "/bar");
    /// ```
    pub fn normalize_path(&mut self) {
        let normalized = normalize(self.path_or_nss()).to_string();
        self.set_path_or_nss(&normalized).unwrap();
    }

    /// Sets the path for a `Uri` in the shame of a [`Url`] or [`RelativeUri`])
    /// or the namespace specific string for a [`Urn`]
    /// # Errors
    pub fn set_path_or_nss(&mut self, path_or_nss: &str) -> Result<String, UriError> {
        match self {
            Self::Url(url) => Ok(set::url::path(url, path_or_nss)),
            Self::Urn(urn) => set::urn::nss(urn, path_or_nss),
            Self::Relative(rel) => Ok(rel.set_path(path_or_nss)?),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Uri::Url(url) => url.as_str(),
            Uri::Urn(urn) => urn.as_str(),
            Uri::Relative(rel) => rel.as_str(),
        }
    }

    /// Returns `true` if the uri is [`Url`].
    ///
    /// [`Url`]: Uri::Url
    #[must_use]
    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url(..))
    }
    #[must_use]
    pub fn as_url(&self) -> Option<&Url> {
        if let Self::Url(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the uri is [`Urn`].
    ///
    /// [`Urn`]: Uri::Urn
    #[must_use]
    pub fn is_urn(&self) -> bool {
        matches!(self, Self::Urn(..))
    }
    #[must_use]
    pub fn as_urn(&self) -> Option<&Urn> {
        if let Self::Urn(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the uri is [`Relative`].
    ///
    /// [`Relative`]: Uri::Relative
    #[must_use]
    pub fn is_relative(&self) -> bool {
        matches!(self, Self::Relative(..))
    }
    #[must_use]
    pub fn as_relative_uri(&self) -> Option<&RelativeUri> {
        if let Self::Relative(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_relative(self) -> Result<RelativeUri, Self> {
        if let Self::Relative(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_urn(self) -> Result<Urn, Self> {
        if let Self::Urn(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_url(self) -> Result<Url, Self> {
        if let Self::Url(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
    #[must_use]
    pub fn scheme(&self) -> Option<&str> {
        match self {
            Uri::Url(url) => Some(url.scheme()),
            Uri::Urn(_) => Some("urn"),
            Uri::Relative(_) => None,
        }
    }
}
impl From<Url> for Uri {
    fn from(value: Url) -> Self {
        Self::Url(value)
    }
}

impl From<Urn> for Uri {
    fn from(value: Urn) -> Self {
        Self::Urn(value)
    }
}

impl From<&Url> for Uri {
    fn from(value: &Url) -> Self {
        Self::Url(value.clone())
    }
}
impl From<&Urn> for Uri {
    fn from(value: &Urn) -> Self {
        Self::Urn(value.clone())
    }
}

impl PartialEq<&str> for Uri {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl PartialEq<str> for Uri {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<AbsoluteUri> for Uri {
    fn eq(&self, other: &AbsoluteUri) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&AbsoluteUri> for Uri {
    fn eq(&self, other: &&AbsoluteUri) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<RelativeUri> for Uri {
    fn eq(&self, other: &RelativeUri) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&RelativeUri> for Uri {
    fn eq(&self, other: &&RelativeUri) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<UriRef<'_>> for Uri {
    fn eq(&self, other: &UriRef<'_>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&UriRef<'_>> for Uri {
    fn eq(&self, other: &&UriRef<'_>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<Uri> for &str {
    fn eq(&self, other: &Uri) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for Uri {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<str> for &Uri {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<Uri> for str {
    fn eq(&self, other: &Uri) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<Uri> for String {
    fn eq(&self, other: &Uri) -> bool {
        self.as_str() == other.as_str()
    }
}

impl FromStr for Uri {
    type Err = UriError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<String> for Uri {
    type Error = UriError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}
impl TryFrom<&String> for Uri {
    type Error = UriError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl Deref for Uri {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Uri::Url(url) => url.as_str(),
            Uri::Urn(urn) => urn.as_str(),
            Uri::Relative(rel) => rel.as_str(),
        }
    }
}

impl PartialOrd for Uri {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Uri {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(self.as_str(), other.as_str())
    }
}
impl From<AbsoluteUri> for Uri {
    fn from(value: AbsoluteUri) -> Self {
        match value {
            AbsoluteUri::Url(url) => Self::Url(url),
            AbsoluteUri::Urn(urn) => Self::Urn(urn),
        }
    }
}

impl From<&Uri> for Uri {
    fn from(value: &Uri) -> Self {
        value.clone()
    }
}

impl From<Uri> for String {
    fn from(value: Uri) -> Self {
        value.to_string()
    }
}

impl From<&Uri> for String {
    fn from(value: &Uri) -> Self {
        value.to_string()
    }
}

/// A trait which facilitates creating a [`Uri`] from an [`AbsoluteUri`],
/// [`&AbsoluteUri`](`AbsoluteUri`), [`Url`], [`&Url`](`Url`), [`Urn`], or
/// [`&Urn`](`Urn`).
pub trait ToUri {
    /// Clones and converts `&self` into an owned `Uri`.
    fn to_uri(&self) -> Uri;
}

impl ToUri for &Uri {
    fn to_uri(&self) -> Uri {
        (*self).clone()
    }
}
impl ToUri for Uri {
    fn to_uri(&self) -> Uri {
        self.clone()
    }
}

impl ToUri for Url {
    fn to_uri(&self) -> Uri {
        Uri::Url(self.clone())
    }
}

impl ToUri for &Url {
    fn to_uri(&self) -> Uri {
        Uri::Url((*self).clone())
    }
}

impl ToUri for Urn {
    fn to_uri(&self) -> Uri {
        Uri::Urn(self.clone())
    }
}

impl ToUri for &Urn {
    fn to_uri(&self) -> Uri {
        Uri::Urn((*self).clone())
    }
}

#[inherent]
impl ToUri for AbsoluteUri {
    /// Returns a cloned [`Uri`](`crate::uri::Uri`) representation of the this
    /// `AbsoluteUri`.
    #[must_use]
    pub fn to_uri(&self) -> Uri {
        match self {
            AbsoluteUri::Url(url) => Uri::Url(url.clone()),
            AbsoluteUri::Urn(urn) => Uri::Urn(urn.clone()),
        }
    }
}

impl ToUri for &AbsoluteUri {
    /// Returns a cloned [`Uri`](`crate::uri::Uri`) representation of the this
    /// `AbsoluteUri`.
    #[must_use]
    fn to_uri(&self) -> Uri {
        match self {
            AbsoluteUri::Url(url) => Uri::Url(url.clone()),
            AbsoluteUri::Urn(urn) => Uri::Urn(urn.clone()),
        }
    }
}

/// Normalizes a path by removing dot segments, i.e. `'.'`, `'..'`.
#[must_use]
pub fn normalize(path: &str) -> Cow<'_, str> {
    let mut normalized = false;
    let mut buf = PathBuf::new();
    for segment in PathSegments::new(path) {
        match segment {
            PathSegment::Parent => {
                normalized = true;
                buf.pop();
            }
            PathSegment::Current => normalized = true,
            PathSegment::Normal(seg) => buf.push(seg.as_ref()),
            PathSegment::Root => {}
        }
    }
    if normalized {
        // safety: path is already in utf8
        buf.to_str().unwrap().to_string().into()
    } else {
        path.into()
    }
}

/// Merges two paths. This is essentially the same as [`PathBuf::push`], but
/// operates UTF-8 strings.
///
/// Note: this does not normalize the paths. See [`resolve`] or [`normalize`] for dot removal.
///
/// # Example
/// ```
/// use grill::uri::merge;
/// assert_eq!(merge("/path/to", "file"), "/path/to/file");
/// ```
#[must_use]
pub fn merge(base: &str, path: &str) -> String {
    let mut buf = PathBuf::from(base);
    buf.push(path);
    // safety: path is already in utf8
    buf.to_str().unwrap().to_string()
}

/// Normalizes and merges `base` with `path`.
/// # Example
/// ```
/// use grill::uri::resolve;
/// assert_eq!(resolve("/path/to/other", "../file"), "/path/to/file");
/// ```
#[must_use]
pub fn resolve(base: &str, path: &str) -> String {
    let buf = merge(base, path);
    normalize(&buf).into_owned()
}
