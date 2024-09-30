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
//!
//! ```rust
//! # use grill_core::uri::{ Uri, AbsoluteUri };
//!
//! let input =
//!      "https://john.doe@example.com:123/forum/questions/?tag=networking&order=newest#top";
//! let uri = Uri::parse(input);
//! eprintln!("uri: {uri:#?}");
//! let uri = uri.unwrap();
//! assert_eq!(&uri, input);
//! assert_eq!(uri.scheme(), Some("https"));
//! assert_eq!(uri.username(), Some("john.doe"));
//! assert_eq!(uri.host().as_deref(), Some("example.com"));
//! assert_eq!(uri.port(), Some(123));
//! assert_eq!(uri.path_or_nss(), "/forum/questions/");
//! assert_eq!(uri.query(), Some("tag=networking&order=newest"));
//! assert_eq!(uri.fragment(), Some("top"));
//! assert_eq!(
//!     uri.authority_or_namespace().unwrap(),
//!     "john.doe@example.com:123"
//! );
//! assert!(uri.is_url());
//!
//! let abs_uri = AbsoluteUri::parse(input).unwrap();
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
//!
//! "urn:example:catalog:articles:record?+r-component:ex=idk?=category=science#fragment"
//!  └┬┘ └───┬┘ └──────────────┬───────┘ └─────────┬───────┘ └───────┬────────┘ └───┬──┘
//! scheme  nid               nss             r-component        q-component    fragment
//!     namespace   namespace specific string                       query                                   
//! ```
//! ```rust
//! # use grill_core::uri::{ Uri, AbsoluteUri };
//!
//! let input = "urn:example:articles:record";
//! let uri = Uri::parse(input).unwrap();
//! assert_eq!(&uri, input);
//! assert_eq!(uri.scheme(), Some("urn"));
//! assert_eq!(uri.username(), None);
//! assert_eq!(uri.authority_or_namespace().as_deref(), Some("example"));
//! assert_eq!(uri.port(), None);
//! assert_eq!(uri.path_or_nss(), "articles:record");
//! assert!(uri.is_urn());
//! let abs_uri = AbsoluteUri::parse(input).unwrap();
//! assert_eq!(uri, abs_uri);
//!
//! let abs_uri = AbsoluteUri::parse(input).unwrap();
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
//! # use grill_core::uri::{ Uri };
//! let input =
//!     "//john.doe:password@example.com:123/forum/questions/?tag=networking&order=newest#top";
//! let uri = Uri::parse(input).unwrap();
//! assert_eq!(&uri, input);
//! assert_eq!(uri.scheme(), None);
//! assert_eq!(uri.username(), Some("john.doe"));
//! assert_eq!(uri.password(), Some("password"));
//! assert_eq!(uri.path_or_nss(), "/forum/questions/");
//! assert_eq!(uri.host().as_deref(), Some("example.com"));
//! assert_eq!(
//!     uri.authority_or_namespace().as_deref(),
//!     Some("john.doe:password@example.com:123")
//! );
//! assert_eq!(uri.port(), Some(123));
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
//! # use grill_core::uri::{ Uri };
//! let input = "/forum/questions/?tag=networking&order=newest#top";
//! let uri = Uri::parse(input).unwrap();
//! assert_eq!(&uri, input);
//! assert_eq!(uri.path_or_nss(), "/forum/questions/");
//! assert_eq!(uri.scheme(), None);
//! assert_eq!(uri.username(), None);
//! assert_eq!(uri.authority_or_namespace(), None);
//! assert_eq!(uri.port(), None);
//! assert_eq!(uri.query(), Some("tag=networking&order=newest"));
//! assert_eq!(uri.fragment(), Some("top"));
//! ```

#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::similar_names,
    clippy::module_name_repetitions,
    // a lot of methods unwrap but never panic
    clippy::missing_panics_doc
)]

mod encode;
mod get;
mod parse;
mod set;
mod write;

use crate::error::UrnError;
pub use error::Error;
use error::{InvalidSchemeError, NotAbsoluteError};
pub use url;
pub use urn;
pub mod error;
use crate::error::{OverflowError, RelativeUriError};
use percent_encoding::percent_decode;
use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    iter::Peekable,
    ops::{Deref, Index},
    path::PathBuf,
    str::{FromStr, Split},
    string::{String, ToString},
};
#[doc(no_inline)]
pub use url::Url;
#[doc(no_inline)]
pub use urn::Urn;

#[macro_export]
macro_rules! uri {
    ($($t:tt)*) => {
        {
            let uri = format!($($t)*);
            $crate::Uri::parse(&uri).expect(&format!("failed to parse uri \"{}\"", uri))
        }
    };
}

#[macro_export]
macro_rules! absolute_uri {
    ($($t:tt)*) => {
        {
            let uri = format!($($t)*);
            $crate::AbsoluteUri::parse(&uri).expect(format!("failed to parse uri {}", uri))
        }
    };
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  Authority                                   ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
/// The Authority component of a Relative URI.
pub struct Authority<'a> {
    value: Cow<'a, str>,
    username_index: Option<u32>,
    password_index: Option<u32>,
    host_index: Option<u32>,
    port_index: Option<u32>,
    port: Option<u16>,
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

    /// Returns the `Authority` as owned.
    #[must_use]
    pub fn into_owned(self) -> Authority<'static> {
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 PathSegment                                  ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
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
    /// Returns a new [`PathSegment::Normal`] from a `&str`.
    #[must_use]
    pub fn normal(val: &'a str) -> Self {
        Self::Normal(Cow::Borrowed(val))
    }
    /// Attempts to decode the path segment as a `&str`.
    ///
    /// # Errors
    /// Returns a [`std::str::Utf8Error`] if the path segment is not valid UTF-8.
    pub fn decode(&'a self) -> Result<Cow<'a, str>, std::str::Utf8Error> {
        match self {
            PathSegment::Root => Ok(Cow::Borrowed("")),
            PathSegment::Current => Ok(Cow::Borrowed(".")),
            PathSegment::Parent => Ok(Cow::Borrowed("..")),
            PathSegment::Normal(val) => percent_decode(val.as_bytes()).decode_utf8(),
        }
    }

    /// Decodes the path segment as a `&str` and replaces invalid UTF-8 with
    /// the Unicode replacement character � (U+FFFD).
    #[must_use]
    pub fn decode_lossy(&'a self) -> Cow<'a, str> {
        match self {
            PathSegment::Root => Cow::Borrowed(""),
            PathSegment::Current => Cow::Borrowed("."),
            PathSegment::Parent => Cow::Borrowed(".."),
            PathSegment::Normal(val) => percent_decode(val.as_bytes()).decode_utf8_lossy(),
        }
    }
    fn parse_root(val: &'a str) -> Self {
        match val {
            "" => Self::Root,
            "." | ".." => Self::remove_dots(val),
            _ => Self::Normal(val.into()),
        }
    }
    fn parse_path_segment(val: &'a str) -> Self {
        match val {
            "." | ".." => Self::remove_dots(val),
            _ => Self::Normal(val.into()),
        }
    }
    fn remove_dots(val: &'a str) -> Self {
        match val {
            "." => Self::Current,
            ".." => Self::Parent,
            _ => Self::Normal(val.into()),
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 PathSegments                                 ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// An [`Iterator`] of [`PathSegment`]s.
#[derive(Debug, Default)]
pub struct PathSegments<'a> {
    path: Option<Peekable<Split<'a, char>>>,
    pub(crate) base_only: bool,
    root_sent: bool,
}

impl<'a> PathSegments<'a> {
    /// Returns a new [`PathSegments`] iterator over the given path.
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
            if base_only && next.is_none() {
                return None;
            }
            return Some(PathSegment::parse_path_segment(value));
        }
        self.root_sent = true;
        Some(PathSegment::parse_root(value))
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 AbsoluteUri                                  ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A URI in the form of a fully qualified [`Url`] or [`Urn`].
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub enum AbsoluteUri {
    /// A fully qualified [`Url`].
    Url(Url),
    /// A fully qualified [`Urn`].
    Urn(Urn),
}
impl std::fmt::Debug for AbsoluteUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl<'a> PartialEq<&'a AbsoluteUri> for AbsoluteUri {
    fn eq(&self, other: &&AbsoluteUri) -> bool {
        (*other).eq(self)
    }
}

impl AbsoluteUri {
    /// Attempts to parse an `AbsoluteUri`.
    ///
    /// # Errors
    /// Returns [`Error`] if `value` can not be parsed as a
    /// [`Url`](`url::Url`) or [`Urn`](`urn::Urn`)
    pub fn parse(value: &str) -> Result<Self, Error> {
        if value.starts_with("urn:") {
            Ok(Urn::from_str(value)?.into())
        } else {
            Ok(Url::parse(value)?.into())
        }
    }

    /// Attempts to parse an `AbsoluteUri`.
    ///
    /// # Panics
    /// Panics if `value` can not be parsed as a [`Url`](`url::Url`) or
    /// [`Urn`](`urn::Urn`)
    pub fn must_parse(value: &str) -> Self {
        match Self::parse(value) {
            Ok(uri) => uri,
            Err(err) => panic!("failed to parse AbsoluteUri \"{value}\"\n\ncaused by:\n\t:{err}"),
        }
    }

    /// Returns a cloned [`Uri`](`crate::uri::Uri`) representation of the this
    /// `AbsoluteUri`.
    #[must_use]
    pub fn to_uri(&self) -> Uri {
        ToUri::to_uri(self)
    }

    /// Returns a new [`Components`] iterator over all components of the `AbsoluteUri`.
    /// # Example
    /// ```rust
    /// # use grill_core::uri::{ AbsoluteUri, Component, PathSegment, QueryParameter };
    /// let input = "https://user:password@example.com/path/to/file/?query=str#fragment";
    /// let uri = AbsoluteUri::parse(input).unwrap();
    /// let expected = vec![
    ///     Component::Scheme("https".into()),                                    // https
    ///     Component::Username("user".into()),                                   // user
    ///     Component::Password("password".into()),                               // password
    ///     Component::Host("example.com".into()),                                // example.com
    ///     Component::PathSegment(PathSegment::Root),                            // /
    ///     Component::PathSegment(PathSegment::Normal("path".into())),           // path/
    ///     Component::PathSegment(PathSegment::Normal("to".into())),             // to/
    ///     Component::PathSegment(PathSegment::Normal("file".into())),           // file/
    ///     Component::PathSegment(PathSegment::Normal("".into())),               // /
    ///                                                                           // ?
    ///     Component::QueryParameter(QueryParameter::new("query=str").unwrap()), // query=str
    ///                                                                           // #
    ///     Component::Fragment("fragment".into()),                               // fragment
    /// ];
    /// let res = uri.components().collect::<Vec<_>>();
    /// assert_eq!(res, expected, "\nexpected: {expected:#?}\nreceived: {res:#?}");
    #[must_use]
    pub fn components(&self) -> Components {
        Components::from_absolute_uri(self)
    }

    /// Returns a new [`PathSegments`] iterator over all segments of this
    /// `AbsoluteUri`'s path.
    ///
    /// # Example
    /// ```rust
    /// # use grill_core::uri::{ AbsoluteUri, PathSegment };
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
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    /// let uri_ref = uri.as_relative_uri().unwrap();
    ///
    /// assert_eq!(uri.base_path(), "/path/to/");
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
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    /// let relative_uri = uri.as_relative_uri().unwrap();
    /// assert_eq!(relative_uri.base_path_segments().collect::<Vec<_>>(), vec!["", "path", "to"]);
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

    #[must_use]
    pub fn fragment_decoded_lossy(&self) -> Option<String> {
        self.fragment().map(decode_lossy)
    }

    /// Percent encodes and sets the fragment component of the [`Url`] or
    /// [`Urn`] and returns the previous fragment in percent-encoded format if
    /// it exists.
    ///
    /// # Errors
    /// Returns [`UriError::FailedToParseUrn`] if the [`AbsoluteUri`] is a [`Urn`] and
    /// the fragment is not a valid [`Urn`] fragment.
    pub fn set_fragment(&mut self, fragment: Option<&str>) -> Result<Option<String>, Error> {
        match self {
            Self::Url(url) => Ok(set::url::fragment(url, fragment)),
            Self::Urn(urn) => set::urn::fragment(urn, fragment),
        }
    }

    /// Returns a clone of this `AbsoluteUri` with the fragment component set to
    /// `fragment`.
    ///
    /// ## Errors
    /// returns [`Error`] if setting the fragment would exceed the maximum
    /// length (4gb).
    pub fn with_fragment(&self, fragment: &str) -> Result<AbsoluteUri, Error> {
        let mut new = self.clone();
        let _ = new.set_fragment(Some(fragment))?;
        Ok(new)
    }

    /// Returns a clone of this `AbsoluteUri` without a fragment
    #[must_use]
    // this method is not fallible
    #[allow(clippy::missing_panics_doc)]
    pub fn without_fragment(&self) -> AbsoluteUri {
        let mut new = self.clone();
        let _ = new.set_fragment(None).unwrap();
        new
    }

    /// Returns the authority (`Url`) or namespace (`Urn`)
    #[must_use]
    pub fn authority_or_namespace(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::Url(url) => get::url::authority(url).map(Cow::Owned),
            Self::Urn(urn) => Some(Cow::Borrowed(urn.nid())),
        }
    }

    /// Returns the path ([`Url`]) or Name Specific String
    /// ([`Urn`]
    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            Self::Url(url) => url.path(),
            Self::Urn(urn) => urn.nss(),
        }
    }

    /// Sets the path (`Url`) or Name Specific String (`Urn`)
    ///
    /// ## Errors
    /// Returns [`Error`] if setting the path would exceed the maximum length
    /// (4gb)
    pub fn set_path_or_nss(&mut self, path_or_nss: &str) -> Result<String, Error> {
        match self {
            Self::Url(url) => Ok(set::url::path(url, path_or_nss)),
            Self::Urn(urn) => set::urn::nss(urn, path_or_nss),
        }
    }

    /// Sets the authority (`Url`) or namespace (`Urn`)
    ///
    /// ## Errors
    /// Returns [`Error`] if setting the authority or namespace to
    /// `authority_or_namespace` would exceed the maximum length (4gb)
    pub fn set_authority_or_namespace(
        &mut self,
        authority_or_namespace: &str,
    ) -> Result<Option<String>, Error> {
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

    /// Returns the port if the `AbsoluteUri` is [`Url`] or a [`RelativeUri`]
    /// with a port. Returns `None` otherwise.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        match self {
            AbsoluteUri::Url(url) => url.port(),
            AbsoluteUri::Urn(..) => None,
        }
    }

    /// Returns the username if the `AbsoluteUri` is [`Url`] or a [`RelativeUri`]
    /// with a username. Returns `None` otherwise.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            AbsoluteUri::Url(url) => match url.username() {
                "" => None,
                s => Some(s),
            },
            AbsoluteUri::Urn(..) => None,
        }
    }
    /// Returns the password if the `AbsoluteUri` is [`Url`] or a [`RelativeUri`]
    /// with a username. Returns `None` otherwise.
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        match self {
            AbsoluteUri::Url(url) => url.password(),
            AbsoluteUri::Urn(..) => None,
        }
    }

    /// Returns the host if the `AbsoluteUri` is [`Url`] or a [`RelativeUri`]
    /// with a username. Returns `None` otherwise.
    #[must_use]
    pub fn host(&self) -> Option<Cow<str>> {
        match self {
            AbsoluteUri::Url(url) => url.host().map(|h| match h {
                url::Host::Domain(s) => Cow::Borrowed(s),
                url::Host::Ipv4(ip) => Cow::Owned(ip.to_string()),
                url::Host::Ipv6(ip) => Cow::Owned(ip.to_string()),
            }),
            AbsoluteUri::Urn(..) => None,
        }
    }

    /// Returns `true` if the `AbsoluteUri` is a [`Url`](`url::Url`).
    ///
    /// [`Url`]: AbsoluteUri::Url
    #[must_use]
    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url(..))
    }

    /// Returns the [`Url`] if this `AbsoluteUri` is a [`Url`] or
    /// `None` otherwise.
    #[must_use]
    pub fn as_url(&self) -> Option<&Url> {
        if let Self::Url(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the `AbsoluteUri` is a [`Urn`].
    #[must_use]
    pub fn is_urn(&self) -> bool {
        matches!(self, Self::Urn(..))
    }

    /// Returns the [`Urn`] if this `AbsoluteUri` is a [`Urn`] or
    /// `None` otherwise.
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
    // self.query is always < u32::MAX
    #[allow(clippy::missing_panics_doc)]
    pub fn query_parameters(&self) -> QueryParameters<'_> {
        QueryParameters::new(self.query()).unwrap()
    }

    /// Sets the query component of the [`Url`] or [`Urn`] and returns the
    /// previous query, if it existed.
    ///
    /// # Errors
    /// Returns an [`UrnError`] if the query is not a valid URN query.
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

    /// Returns `true` if the fragment is either `None` or an empty string
    #[must_use]
    pub fn fragment_is_empty_or_none(&self) -> bool {
        self.fragment().map_or(true, |f| f.trim().is_empty())
    }

    /// returns a new `AbsoluteUri` that is the result of resolving the given
    /// reference against this `AbsoluteUri`.
    ///
    /// See [RFC3986, Section
    /// 5.2.2](https://tools.ietf.org/html/rfc3986#section-5.2.2).
    ///
    /// ## Errors
    /// Returns an [`Error`] if resolving the `reference` results in a malformed
    /// URI or if the URI exceeds 4GB.
    #[allow(clippy::missing_panics_doc)]
    pub fn resolve(&self, reference: &impl AsUriRef) -> Result<AbsoluteUri, Error> {
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

    /// Returns the scheme of the `AbsoluteUri`
    /// # Example
    /// ```rust
    /// # use grill::AbsoluteUri;
    /// let uri = AbsoluteUri::parse("https://example.com").unwrap();
    /// assert_eq!(uri.scheme(), "https");
    #[must_use]
    pub fn scheme(&self) -> &str {
        match self {
            AbsoluteUri::Url(url) => url.scheme(),
            AbsoluteUri::Urn(_) => "urn",
        }
    }
    /// Sets the scheme of the `AbsoluteUri` and returns the previous scheme. If
    /// this `AbsoluteUri` is an [`Urn`], then a `scheme` value of
    /// anything other than `"urn"` will result in the URN being parsed as a
    /// URL. If the `AbsoluteUri` is a [`Url`], then a scheme value
    /// of anything other than `"urn"` will result in the URL being parsed as a
    /// [`Urn`].
    ///
    /// # Errors
    /// Returns a [`UriError`] if:
    /// - the scheme is invalid
    /// - the `AbsoluteUri` is currently an [`Urn`], the value of
    ///  `scheme` is something other than `"urn"`, and the URN cannot be parsed
    ///  as a URL.
    /// - the `AbsoluteUri` is currently an [`Url`], the value of
    ///   `scheme` is `"urn"`, and the URL cannot be parsed as a URN.
    pub fn set_scheme(&mut self, scheme: &str) -> Result<String, Error> {
        let scheme = scheme.trim_end_matches('/').trim_end_matches(':');

        let prev = self.scheme().to_string();
        match self {
            AbsoluteUri::Url(url) => {
                if scheme == "urn" {
                    let mut s = url.to_string();
                    let i = url.scheme().len() + 3;
                    s.replace_range(..i, "urn:");
                    let urn = Urn::from_str(&s)?;
                    *self = AbsoluteUri::Urn(urn);
                } else {
                    url.set_scheme(scheme)
                        .map_err(|()| InvalidSchemeError::new(scheme.to_string()))?;
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
    /// # use grill_core::uri::AbsoluteUri;
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
    /// # use grill_core::uri::AbsoluteUri;
    /// let mut uri = AbsoluteUri::parse("https://example.com/./foo/../bar").unwrap();
    /// uri.normalize_path();
    /// assert_eq!(uri.path_or_nss(), "/bar");
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn normalize_path(&mut self) {
        let normalized = normalize(self.path_or_nss()).to_string();
        self.set_path_or_nss(&normalized).unwrap();
    }

    /// Returns `true` if this URI has a query component.
    #[must_use]
    pub fn has_query(&self) -> bool {
        self.query().is_some()
    }
    /// Returns `true` if this URI has a fragment component.
    #[must_use]
    pub fn has_fragment(&self) -> bool {
        self.fragment().is_some()
    }

    /// Returns `true` if this `AbsoluteUri` has a fragment component which
    /// contains one or more non-whitespace characters.
    #[must_use]
    pub fn has_non_empty_fragment(&self) -> bool {
        self.fragment().map(str::trim).map_or(false, str::is_empty)
    }
}

impl Borrow<str> for AbsoluteUri {
    fn borrow(&self) -> &str {
        self.as_str()
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
    type Error = Error;

    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        match value {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url)),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn)),
            Uri::Relative(p) => Self::parse(p.as_str()),
        }
    }
}
impl TryFrom<&Uri> for AbsoluteUri {
    type Error = Error;

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
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<&String> for AbsoluteUri {
    type Error = Error;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for AbsoluteUri {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl FromStr for AbsoluteUri {
    type Err = Error;
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

impl<'a> TryIntoAbsoluteUri for UriRef<'a> {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        match self {
            UriRef::Uri(uri) => uri.try_into_absolute_uri(),
            UriRef::AbsoluteUri(uri) => Ok(uri.clone()),
            UriRef::RelativeUri(rel) => {
                Error::err_with(|| NotAbsoluteError::new(rel.clone().into()))
            }
        }
    }
}

impl TryIntoAbsoluteUri for String {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        AbsoluteUri::parse(&self)
    }
}

impl TryIntoAbsoluteUri for &str {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        AbsoluteUri::parse(self)
    }
}

impl TryIntoAbsoluteUri for AbsoluteUri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        Ok(self)
    }
}

impl TryIntoAbsoluteUri for &Url {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        Ok(AbsoluteUri::Url(self.clone()))
    }
}
impl TryIntoAbsoluteUri for &Urn {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        Ok(AbsoluteUri::Urn(self.clone()))
    }
}
impl TryIntoAbsoluteUri for Url {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        Ok(AbsoluteUri::Url(self))
    }
}
impl TryIntoAbsoluteUri for Urn {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        Ok(AbsoluteUri::Urn(self))
    }
}
impl TryIntoAbsoluteUri for &String {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        AbsoluteUri::parse(self)
    }
}
impl TryIntoAbsoluteUri for &Uri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        match self {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url.clone())),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn.clone())),
            Uri::Relative(rel) => AbsoluteUri::parse(rel.as_str()),
        }
    }
}

impl TryIntoAbsoluteUri for Uri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        match self {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url)),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn)),
            Uri::Relative(rel) => AbsoluteUri::parse(rel.as_str()),
        }
    }
}

impl TryIntoAbsoluteUri for &AbsoluteUri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        Ok(self.clone())
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                              TryIntoAbsoluteUri                              ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A trait for possibly converting a type to an [`AbsoluteUri`].
///
pub trait TryIntoAbsoluteUri {
    /// Attempts to convert `self` into an [`AbsoluteUri`].
    ///
    /// # Errors
    /// Returns an error if the conversion fails due to the value not being an
    /// absolute in the sense of having a scheme and an authority (URL) or a namespace
    /// (URN).
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error>;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                QueryParameter                                ║
║                               ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                               ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A single query parameter key value pair.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct QueryParameter<'a> {
    full: Cow<'a, str>,
    eq_index: Option<u32>,
}
impl<'a> QueryParameter<'a> {
    /// Creates a new `QueryParameter` from the given `full` query parameter
    /// string.
    ///
    /// # Errors
    /// Returns `OverflowError` if the length of `full` is greater than `u32::MAX`
    #[allow(clippy::missing_panics_doc)]
    pub fn new(full: &'a str) -> Result<Self, OverflowError> {
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
    type Error = OverflowError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                               QueryParameters                                ║
║                              ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                               ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// An iterator over [`QueryParameter`]s.
#[derive(Debug, Default)]
pub struct QueryParameters<'a> {
    query: Option<Split<'a, char>>,
}
impl<'a> QueryParameters<'a> {
    /// Creates a new `QueryParameters` iterator from the given query string.
    ///
    /// # Errors
    /// Returns `OverflowError` if the length of `query` is greater than `u32::MAX`
    pub fn new(query: Option<&'a str>) -> Result<Self, OverflowError> {
        let Some(query) = query else {
            return Ok(Self { query: None });
        };
        if query.len() > u32::MAX as usize {
            return Err(OverflowError {
                len: query.len() as u64,
            });
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 RelativeUri                                  ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A relative URI, with or without an authority.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RelativeUri {
    value: String,
    username_index: Option<u32>,
    password_index: Option<u32>,
    host_index: Option<u32>,
    port_index: Option<u32>,
    port: Option<u16>,
    path_index: u32,
    query_index: Option<u32>,
    fragment_index: Option<u32>,
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
    /// # use grill_core::uri::{ RelativeUri, Uri, Component, PathSegment, QueryParameter };
    /// let input = "/path/to/file/?query=str#fragment";
    /// let uri = Uri::parse(input).unwrap();
    /// let relative_uri = uri.as_relative_uri().unwrap();
    /// let res = relative_uri.components().collect::<Vec<_>>();
    ///
    /// let expected = vec![
    ///     Component::PathSegment(PathSegment::Root),                            // /
    ///     Component::PathSegment(PathSegment::Normal("path".into())),           // path
    ///                                                                           // /
    ///     Component::PathSegment(PathSegment::Normal("to".into())),             // to
    ///                                                                           // /
    ///     Component::PathSegment(PathSegment::Normal("file".into())),           // file
    ///     Component::PathSegment(PathSegment::Normal("".into())),               // /
    ///                                                                           // ?
    ///     Component::QueryParameter(QueryParameter::new("query=str").unwrap()), // query=str
    ///                                                                           // #
    ///     Component::Fragment("fragment".into()),                               // fragment
    /// ];
    /// assert_eq!(res, expected);
    #[must_use]
    pub fn components(&self) -> Components {
        Components::from_relative_uri(self)
    }

    /// Returns a new [`PathSegments`] iterator over all segments of this
    /// `RelativeUri`'s path.
    /// # Example
    /// ```rust
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    /// let uri = uri.as_relative_uri().unwrap();
    /// assert_eq!(uri.path_segments().collect::<Vec<_>>(), vec!["", "path", "to", "file"]);
    /// ```
    #[must_use]
    pub fn path_segments(&self) -> PathSegments {
        PathSegments::from(self.path())
    }

    /// returns a new `Uri` that is the result of resolving the given reference
    /// against this `RelativeUri`.
    ///
    /// See [RFC3986, Section
    /// 5.2.2](https://tools.ietf.org/html/rfc3986#section-5.2.2).
    ///
    /// ## Errors
    /// Returns [`Error`] if the length of the `Uri` exceeds 4GB after
    /// resolving.
    #[allow(clippy::missing_panics_doc)]
    pub fn resolve(&self, reference: &impl AsUriRef) -> Result<Uri, Error> {
        let reference = reference.as_uri_ref();

        // if the reference has a scheme, normalize the path and return
        if let Ok(mut uri) = reference.try_into_absolute_uri() {
            uri.normalize_path();
            return Ok(uri.into());
        }

        // urls and urns will get processed in the match above
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
            uri.set_fragment(reference.fragment())?;
            uri.set_query(reference.query())?;
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
    /// # use grill_core::uri::Uri;
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
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    /// let rel_uri = uri.as_relative_uri().unwrap();
    ///
    /// assert_eq!(rel_uri.base_path(), "/path/to");
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
    /// # use grill_core::uri::{ Uri, RelativeUri };
    /// let uri = Uri::parse("//user:pass@host/path?query#fragment").unwrap();
    /// let relative_uri = uri.as_relative_uri().unwrap();
    /// assert_eq!(relative_uri.username(), Some("user"));
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        let start = self.username_index()?;
        let end = self.username_end_index()?;
        Some(&self.value[start..end])
    }
    /// Returns the password portion of the `RelativeUri` if it exists.
    /// # Example
    /// ```rust
    /// # use grill_core::uri::{ Uri, RelativeUri };
    /// let uri = Uri::parse("//user:pass@host/path?query#fragment").unwrap();
    /// let relative_uri = uri.as_relative_uri().unwrap();
    /// assert_eq!(relative_uri.password(), Some("pass"));
    /// ```
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
        if fragment_index + 1 == self.as_str().len() {
            return Some("");
        }

        Some(&self.value[fragment_index + 1..])
    }

    /// Returns the query string segment of the `RelativeUri`, if it exists.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        let query_index = self.query_index()?;
        if query_index + 1 == self.as_str().len() {
            return Some("");
        }
        let last = self.fragment_index().unwrap_or(self.as_str().len());
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
        self.fragment().is_some()
    }

    /// Returns `true` if this

    fn has_path(&self) -> bool {
        !self.path().is_empty()
    }

    /// Sets the query string portion of the `RelativeUri` and returns the
    /// previous query, if it existed.
    ///
    /// ## Errors
    /// Returns a [`RelativeUriError`] if the length of the new query exceeds
    /// `u32::MAX` (4GB)
    pub fn set_query(&mut self, query: Option<&str>) -> Result<Option<String>, RelativeUriError> {
        let existing_query = self.query().map(ToString::to_string);
        let cap = self.as_str().len()
            - existing_query.as_ref().map(String::len).unwrap_or_default()
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
    ///
    /// ## Errors
    /// Returns a [`RelativeUriError`] if the length of the uri exceeds
    /// `u32::MAX` (4GB) after setting the path
    pub fn set_path(&mut self, path: &str) -> Result<String, RelativeUriError> {
        let existing_path = self.path().to_string();
        let mut buf = String::with_capacity(self.as_str().len() - existing_path.len() + path.len());
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

    /// Sets the fragment of the `RelativeUri` and returns the previous
    /// fragment, if present.
    ///
    /// ## Errors
    /// Returns a [`RelativeUriError`] if the length of the uri exceeds
    /// `u32::MAX` (4GB) after setting the fragment
    pub fn set_fragment(
        &mut self,
        fragment: Option<&str>,
    ) -> Result<Option<String>, RelativeUriError> {
        let existing_fragment = self.fragment().map(ToString::to_string);
        let mut buf = String::with_capacity(
            self.as_str().len()
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

    /// Sets the authority and returns the previous value as an [`Authority`],
    /// if it existed.
    ///
    /// ## Errors
    /// Returns a [`RelativeUriError`] if the length of the uri exceeds
    /// `u32::MAX` (4GB) after setting the authority.
    pub fn set_authority<'a>(
        &'a mut self,
        authority: Option<&str>,
    ) -> Result<Option<Authority<'a>>, Error> {
        let existing_authority = self.authority().map(Authority::into_owned);
        let new = authority
            .map(parse::authority)
            .transpose()?
            .unwrap_or_default();
        let mut buf = String::with_capacity(
            self.as_str().len()
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

    /// Returns `true` if the `RelativeUri` has an authority.
    /// # Example
    /// ```
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("//example.com").unwrap();
    /// let rel_uri = uri.as_relative_uri().unwrap();
    /// assert!(rel_uri.has_authority());
    #[must_use]
    pub fn has_authority(&self) -> bool {
        self.path_index() > 2
    }

    /// Returns `true` if the `RelativeUri` has a username.
    /// # Example
    /// ```
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("//user@host").unwrap();
    /// let rel_uri = uri.as_relative_uri().unwrap();
    /// assert!(rel_uri.has_username());
    /// let uri = Uri::parse("//host").unwrap();
    /// let rel_uri = uri.as_relative_uri().unwrap();
    /// assert!(!rel_uri.has_username());
    /// ```
    #[must_use]
    pub fn has_username(&self) -> bool {
        self.username_index.is_some()
    }

    /// Returns `true` if the `RelativeUri` has a password.
    /// # Example
    /// ```
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("//user:pass@host").unwrap();
    /// let rel_uri = uri.as_relative_uri().unwrap();
    /// assert!(rel_uri.has_password());
    /// let uri = Uri::parse("//user@host").unwrap();
    /// let rel_uri = uri.as_relative_uri().unwrap();
    /// assert!(!rel_uri.has_password());
    /// ```
    #[must_use]
    pub fn has_password(&self) -> bool {
        self.password_index.is_some()
    }

    /// Returns `true` if the `RelativeUri` has a host.
    ///
    /// # Example
    /// ```
    /// # use grill_core::Uri;
    ///
    /// let uri = Uri::parse("//example.com").unwrap();
    /// let rel_uri = uri.as_relative_uri().unwrap();
    /// assert!(uri.host().is_some());
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
    /// # use grill_core::uri::Uri;
    /// let mut uri = Uri::parse("/./foo/../bar").unwrap();
    /// let uri_ref = uri.as_relative_uri().unwrap();
    /// let normalized = uri_ref.path_normalized();
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
    /// # use grill_core::uri::Uri;
    /// let mut uri = Uri::parse("https://example.com/./foo/../bar").unwrap();
    /// uri.normalize_path();
    /// assert_eq!(uri.path_or_nss(), "/bar");
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    UriRef                                    ║
║                                   ¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
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
    /// Attempts to convert this `UriRef` into an [`AbsoluteUri`].
    ///
    /// # Errors
    /// Returns a [`UriError`] if the conversion fails due to the value not
    /// being an absolute in the sense of having a scheme and an authority (URL)
    /// or a namespace (URN).
    pub fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        TryIntoAbsoluteUri::try_into_absolute_uri(self)
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
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    /// let uri_ref = uri.as_uri_ref();
    ///
    /// assert_eq!(uri_ref.base_path(), "/path/to");
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

    /// Returns the path [`normalize`]d by removing dot segments, i.e. `'.'`,
    /// `'..'`.
    ///
    /// # Example
    /// ```
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("https://example.com/./foo/../bar").unwrap();
    /// let uri_ref = uri.as_uri_ref();
    /// let normalized = uri_ref.path_normalized();
    /// assert_eq!(normalized, "/bar");
    /// ```
    /// [`normalize`]
    #[must_use]
    pub fn path_normalized(&self) -> Cow<'_, str> {
        normalize(self.path_or_nss())
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   AsUriRef                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A trait which enables borrowing a [`Uri`], [`AbsoluteUri`], or
/// [`RelativeUri`] as a singular type.
pub trait AsUriRef {
    /// Borrows `self` as a [`UriRef`].
    fn as_uri_ref(&self) -> UriRef<'_>;
}

impl AsUriRef for AbsoluteUri {
    #[must_use]
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::AbsoluteUri(self)
    }
}

impl AsUriRef for &AbsoluteUri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::AbsoluteUri(self)
    }
}

impl AsUriRef for Uri {
    #[must_use]
    fn as_uri_ref(&self) -> UriRef<'_> {
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

impl AsUriRef for RelativeUri {
    #[must_use]
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::RelativeUri(self)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  Component                                   ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
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
        self.as_str() == other
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  Components                                  ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                     Uri                                      ║
║                                    ¯¯¯¯¯                                     ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A relative or absolute URI in the form of a [`Url`], [`Urn`], or
/// [`RelativeUri`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub enum Uri {
    /// Uniform Resource Locator (URL)
    Url(Url),
    /// Uniform Resource Name (URN)
    Urn(Urn),
    /// Relative URI
    Relative(RelativeUri),
}

impl Uri {
    /// Attempts to convert this `Uri` into an [`AbsoluteUri`]
    ///
    /// ## Errors
    /// Returns a [`Error`] if the conversion fails due to the `Uri` not
    /// being absolute (has a scheme and an authority (URL) or a namespace (URN))
    pub fn try_into_absolute_uri(self) -> Result<AbsoluteUri, Error> {
        AbsoluteUri::try_from(self)
    }
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
    pub fn parse(value: &str) -> Result<Self, Error> {
        parse::uri(value)
    }

    /// returns a new `Uri` that is the result of resolving the given reference
    /// against this `Uri`.
    ///
    /// See [RFC3986, Section
    /// 5.2.2](https://tools.ietf.org/html/rfc3986#section-5.2.2).
    ///
    /// ## Errors
    /// Returns a [`Error`] if resolving the `reference` would cause the
    /// resulting `Uri` to exceed `u32::MAX`.
    pub fn resolve(&self, reference: &impl AsUriRef) -> Result<Uri, Error> {
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
    /// # use grill_core::uri::{  Uri, Component, PathSegment, QueryParameter };
    /// let input = "https://user:password@example.com/path/to/file/?query=str#fragment";
    /// let uri = Uri::parse(input).unwrap();
    /// let expected = vec![
    ///     Component::Scheme("https".into()),                                    // https
    ///     Component::Username("user".into()),                                   // user
    ///     Component::Password("password".into()),                               // password
    ///     Component::Host("example.com".into()),                                // example.com
    ///     Component::PathSegment(PathSegment::Root),                            // /
    ///     Component::PathSegment(PathSegment::Normal("path".into())),           // path/
    ///     Component::PathSegment(PathSegment::Normal("to".into())),             // to/
    ///     Component::PathSegment(PathSegment::Normal("file".into())),           // file/
    ///     Component::PathSegment(PathSegment::Normal("".into())),               // /
    ///                                                                           // ?
    ///     Component::QueryParameter(QueryParameter::new("query=str").unwrap()), // query=str
    ///                                                                           // #
    ///     Component::Fragment("fragment".into()),                               // fragment
    /// ];
    /// let res = uri.components().collect::<Vec<_>>();
    /// assert_eq!(res, expected, "\nexpected: {expected:#?}\nreceived: {res:#?}");
    #[must_use]
    pub fn components(&self) -> Components {
        Components::from_uri(self)
    }
    /// Returns a new [`PathSegments`] iterator over all segments of this
    /// `Uri`'s path.
    /// # Example
    /// ```rust
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("https://example.com/path/to/file").unwrap();
    /// assert_eq!(uri.path_segments().collect::<Vec<_>>(), vec!["", "path", "to", "file"]);
    /// ```
    #[must_use]
    pub fn path_segments(&self) -> PathSegments {
        PathSegments::from(self.path_or_nss())
    }

    /// Returns the base path, that is all path segments except for the last.
    ///
    /// # Example
    /// ```
    /// # use grill_core::uri::Uri;
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
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    /// assert_eq!(uri.base_path_segments().collect::<Vec<_>>(), vec!["", "path", "to"]);
    #[must_use]
    pub fn base_path_segments(&self) -> PathSegments<'_> {
        let mut segments = self.path_segments();
        segments.base_only = true;
        segments
    }

    /// Returns the fragment component of the [`Url`] or [`Urn`] if it exists.
    ///
    /// # Example
    /// ```
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("https://example.com/path/to/file#fragment").unwrap();
    /// assert_eq!(uri.fragment(), Some("fragment"));
    /// ```
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
    pub fn set_fragment(&mut self, mut fragment: Option<&str>) -> Result<Option<String>, Error> {
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
    /// Returns `true` if the fragment component of the `Uri` is empty or
    /// `None`.
    #[must_use]
    pub fn is_fragment_empty_or_none(&self) -> bool {
        self.fragment().map_or(true, |f| f.trim().is_empty())
    }

    /// Sets the query component of the [`Url`] or [`Urn`] and returns the
    /// previous query, if it existed.
    ///
    /// ## Errors
    /// Returns [`Error`] if this `Uri` would exceed `u32::MAX` after setting
    /// the query.
    pub fn set_query(&mut self, query: Option<&str>) -> Result<Option<String>, Error> {
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
    /// Returns the namespace if the `Uri` is [`Urn`], otherwise returns
    /// the authority string for a [`Url`] or [`RelativeUri`].
    #[must_use]
    pub fn authority_or_namespace(&self) -> Option<Cow<'_, str>> {
        match self {
            Uri::Url(url) => get::url::authority(url).map(Into::into),
            Uri::Urn(urn) => Some(Cow::Borrowed(urn.nid())),
            Uri::Relative(rel) => rel.authority_str().map(Cow::Borrowed),
        }
    }

    /// Returns the port if the `Uri` is [`Url`] or a [`RelativeUri`]
    /// with a port. Returns `None` otherwise.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        match self {
            Uri::Url(url) => url.port(),
            Uri::Relative(rel) => rel.port(),
            Uri::Urn(..) => None,
        }
    }

    /// Returns the username if the `Uri` is [`Url`] or a [`RelativeUri`]
    /// with a username. Returns `None` otherwise.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Uri::Url(url) => match url.username() {
                "" => None,
                s => Some(s),
            },
            Uri::Urn(..) => None,
            Uri::Relative(rel) => rel.username(),
        }
    }

    /// Returns the password if the `Uri` is [`Url`] or a [`RelativeUri`]
    /// with a username. Returns `None` otherwise.
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        match self {
            Uri::Url(url) => url.password(),
            Uri::Urn(..) => None,
            Uri::Relative(rel) => rel.password(),
        }
    }

    /// Returns the host if the `Uri` is [`Url`] or a [`RelativeUri`]
    /// with a username. Returns `None` otherwise.
    pub fn host(&self) -> Option<Cow<str>> {
        match self {
            Uri::Url(url) => url.host().map(|h| match h {
                url::Host::Domain(s) => Cow::Borrowed(s),
                url::Host::Ipv4(ip) => Cow::Owned(ip.to_string()),
                url::Host::Ipv6(ip) => Cow::Owned(ip.to_string()),
            }),
            Uri::Urn(..) => None,
            Uri::Relative(rel) => rel.host().map(Cow::Borrowed),
        }
    }
    /// Sets the authority (if a `Url` or `RelativeUri`) or namespace (if a
    /// `Urn`) to `authority_or_namespace`.
    ///
    /// ## Errors
    /// Returns an [`Error`] if setting `authority_or_namespace` would cause
    /// this `Uri` to exceed `u32::MAX` (4GB).
    pub fn set_authority_or_namespace(
        &mut self,
        authority_or_namespace: &str,
    ) -> Result<Option<String>, Error> {
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
    /// # use grill_core::uri::Uri;
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
    /// # use grill_core::uri::Uri;
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
    pub fn set_path_or_nss(&mut self, path_or_nss: &str) -> Result<String, Error> {
        match self {
            Self::Url(url) => Ok(set::url::path(url, path_or_nss)),
            Self::Urn(urn) => set::urn::nss(urn, path_or_nss),
            Self::Relative(rel) => Ok(rel.set_path(path_or_nss)?),
        }
    }
    /// Returns the `Uri` as a `&str`
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Uri::Url(url) => url.as_str(),
            Uri::Urn(urn) => urn.as_str(),
            Uri::Relative(rel) => rel.as_str(),
        }
    }

    /// Returns `true` if the uri is [`Url`].
    #[must_use]
    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url(..))
    }

    /// Returns a reference to a [`Url`] if the `Uri` is a `Url` or `None`
    /// otherwise.
    #[must_use]
    pub fn as_url(&self) -> Option<&Url> {
        if let Self::Url(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the uri is [`Urn`].
    #[must_use]
    pub fn is_urn(&self) -> bool {
        matches!(self, Self::Urn(..))
    }

    /// Returns a reference to a [`Urn`] if the `Uri` is a `Urn` or `None`
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
    /// Returns this `Uri` as a reference to a  [`RelativeUri`] if it is a
    /// relative URI or `None` otherwise.
    #[must_use]
    pub fn as_relative_uri(&self) -> Option<&RelativeUri> {
        if let Self::Relative(v) = self {
            Some(v)
        } else {
            None
        }
    }
    /// Attempts to convert this `Uri` into a [`RelativeUri`].
    ///
    /// ## Errors
    /// Returns `Err(self)` if the `Uri` is not a `RelativeUri`.
    pub fn try_into_relative(self) -> Result<RelativeUri, Self> {
        if let Self::Relative(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
    /// Consumes and returns this `Uri` as a [`Urn`] if it is a `Urn` or
    /// returns `Err(self)` otherwise.
    ///
    /// ## Errors
    /// Returns `Err(self)` if the `Uri` is not a `Urn`.
    pub fn try_into_urn(self) -> Result<Urn, Self> {
        if let Self::Urn(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Consumes and returns this `Uri` as a [`Url`] if it is a `Url` or
    /// returns `Err(self)` otherwise.
    ///
    /// ## Errors
    /// Returns `Err(self)` if the `Uri` is not a `Url`.
    pub fn try_into_url(self) -> Result<Url, Self> {
        if let Self::Url(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns the scheme of the `Uri` if it exists.
    /// # Example
    /// ```
    /// # use grill_core::uri::Uri;
    /// let uri = Uri::parse("https://example.com/path/to/file").unwrap();
    /// assert_eq!(uri.scheme(), Some("https"));
    /// ```
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<String> for Uri {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}
impl TryFrom<&String> for Uri {
    type Error = Error;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::parse(value)
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    ToUri                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

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

impl ToUri for AbsoluteUri {
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  normalize                                   ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

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
            PathSegment::Root => buf.push("/"),
        }
    }
    if normalized {
        // safety: path is already in utf8
        buf.to_str().unwrap().to_string().into()
    } else {
        path.into()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    merge                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Merges two paths. This is essentially the same as [`PathBuf::push`], but
/// operates UTF-8 strings.
///
/// Note: this does not normalize the paths. See [`resolve`] or [`normalize`] for dot removal.
///
/// # Example
/// ```
/// # use grill_core::uri::merge;
/// assert_eq!(merge("/path/to", "file"), "/path/to/file");
/// ```
#[must_use]
pub fn merge(base: &str, path: &str) -> String {
    let mut buf = PathBuf::from(base);
    buf.push(path);
    // safety: path is already in utf8
    buf.to_str().unwrap().to_string()
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   resolve                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Normalizes and merges `base` with `path`.
/// # Example
/// ```
/// # use grill_core::uri::resolve;
/// assert_eq!(resolve("/path/to/other", "../file"), "/path/to/file");
/// ```
#[must_use]
pub fn resolve(base: &str, path: &str) -> String {
    let buf = merge(base, path);
    normalize(&buf).into_owned()
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    decode                                    ║
║                                   ¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Percent decodes `value`
///
/// # Errors
/// Returns a `std::str::Utf8Error` if the decoded value is not valid utf-8
pub fn decode(value: &str) -> Result<String, std::str::Utf8Error> {
    Ok(percent_encoding::percent_decode_str(value)
        .decode_utf8()?
        .to_string())
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 decode_lossy                                 ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Percent decodes `value` lossily
#[must_use]
pub fn decode_lossy(fragment: &str) -> String {
    percent_encoding::percent_decode_str(fragment)
        .decode_utf8_lossy()
        .to_string()
}

/// Attempts to convert a `usize` to `u32`
///
/// # Errors
/// Returns `OverflowError` if `v` exceeds
/// `u32::MAX` (`4294967295`)
#[inline]
pub(crate) fn usize_to_u32(value: usize) -> Result<u32, OverflowError> {
    value
        .try_into()
        .map_err(|_| OverflowError { len: value as u64 })
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
    #[test]
    fn test_uri_resolve() {
        let abs_uri = AbsoluteUri::parse("http://a/b/c/d;p?q").unwrap();
        let uri = Uri::parse("http://a/b/c/d;p?q").unwrap();
        let tests = [
            ("g:h", "g:h"),
            ("g", "http://a/b/c/g"),
            ("./g", "http://a/b/c/g"),
            ("g/", "http://a/b/c/g/"),
            ("/g", "http://a/g"),
            ("//g", "http://g/"),
            ("?y", "http://a/b/c/d;p?y"),
            ("g?y", "http://a/b/c/g?y"),
            ("#s", "http://a/b/c/d;p?q#s"),
            ("g#s", "http://a/b/c/g#s"),
            ("g?y#s", "http://a/b/c/g?y#s"),
            (";x", "http://a/b/c/;x"),
            ("g;x", "http://a/b/c/g;x"),
            ("g;x?y#s", "http://a/b/c/g;x?y#s"),
            ("", "http://a/b/c/d;p?q"),
            (".", "http://a/b/c/"),
            ("./", "http://a/b/c/"),
            ("..", "http://a/b/"),
            ("../", "http://a/b/"),
            ("../g", "http://a/b/g"),
            ("../..", "http://a/"),
            ("../../", "http://a/"),
            ("../../g", "http://a/g"),
            ("../../../g", "http://a/g"),
            ("../../../../g", "http://a/g"),
            ("/./g", "http://a/g"),
            ("/../g", "http://a/g"),
            ("g.", "http://a/b/c/g."),
            (".g", "http://a/b/c/.g"),
            ("g..", "http://a/b/c/g.."),
            ("..g", "http://a/b/c/..g"),
            ("./../g", "http://a/b/g"),
            ("./g/.", "http://a/b/c/g/"),
            ("g/./h", "http://a/b/c/g/h"),
            ("g/../h", "http://a/b/c/h"),
            ("g;x=1/./y", "http://a/b/c/g;x=1/y"),
            ("g;x=1/../y", "http://a/b/c/y"),
            ("g?y/./x", "http://a/b/c/g?y/./x"),
            ("g?y/../x", "http://a/b/c/g?y/../x"),
            ("g#s/./x", "http://a/b/c/g#s/./x"),
            ("g#s/../x", "http://a/b/c/g#s/../x"),
        ];
        for (input, expected) in tests {
            let input = Uri::parse(input);
            if let Err(e) = &input {
                println!(
                    "\n\nfailed to parse input: {:?};\n\terror: {}\n\n",
                    &input, e
                );
            }
            let input = input.unwrap();

            let result = abs_uri.resolve(&input);

            if let Err(e) = &result {
                println!(
                    r#"
            
            failed to resolve:
                input: "{input:?}"
                error: {e:?}
                
                "#
                );
            }
            let result = result.unwrap();
            assert_eq!(
                &result, expected,
                r#"

    failed to resolve:
        input:      "{input}",
        result:     {result}
        expected:   {expected}
            "#
            );

            let result = uri.resolve(&input);

            if let Err(e) = &result {
                println!(
                    r#"
            
            failed to resolve:
                input: "{input:?}"
                error: {e:?}
                
                "#
                );
            }
            let result = result.unwrap();
            assert_eq!(
                &result, expected,
                r#"
            
            failed to resolve:
            input:      "{input}"
            result:     "{result}"
            expected:   "{expected}"


            "#
            );
        }
    }

    #[test]
    fn test_base_path_segments() {
        let uri = Uri::parse("/path/to/file").unwrap();
        let segments = uri.base_path_segments().collect::<Vec<_>>();
        assert_eq!(
            segments,
            vec![
                PathSegment::Root,
                PathSegment::normal("path"),
                PathSegment::normal("to")
            ]
        );
    }

    #[test]
    fn test_join() {
        let base = "/a/b/c";
        assert_eq!(super::merge(base, "x/y/z"), "/a/b/c/x/y/z");
        assert_eq!(super::merge(base, "/x/y/z"), "/x/y/z");
    }

    #[test]
    fn test_uri_components() {
        let uri = Uri::parse("http://example.com/path?query#fragment").unwrap();
        let mut components = uri.components();
        assert_eq!(components.next(), Some(Component::Scheme("http".into())));
        assert_eq!(
            components.next(),
            Some(Component::Host("example.com".into()))
        );
    }

    #[test]
    fn test_relative_uri_parse() {
        let tests = [
            (
                "/path?query#fragment",
                None,
                "/path",
                Some("query"),
                Some("fragment"),
            ),
            (
                "//example.com/path/path2?query=str#fragment",
                Some("example.com"),
                "/path/path2",
                Some("query=str"),
                Some("fragment"),
            ),
        ];

        for (input, authority, path, query, fragment) in tests {
            let uri = Uri::parse(input).unwrap();
            assert_eq!(authority, uri.authority_or_namespace().as_deref());
            assert_eq!(path, uri.path_or_nss());
            assert_eq!(query, uri.query());
            assert_eq!(fragment, uri.fragment());
        }
    }

    #[test]
    fn test_set_query() {
        let mut uri = Uri::parse("/path").unwrap();
        assert_eq!(uri.query(), None);
        assert_eq!(uri.fragment(), None);

        uri.set_query(Some("q=str")).unwrap();
        assert_eq!(uri.as_str(), "/path?q=str");
        assert_eq!(uri.query(), Some("q=str"));

        uri.set_fragment(Some("fragment")).unwrap();
        assert_eq!(uri.as_str(), "/path?q=str#fragment");
        assert_eq!(uri.fragment(), Some("fragment"));

        uri.set_query(None).unwrap();
        assert_eq!(uri.query(), None);
        assert_eq!(uri.as_str(), "/path#fragment");

        uri.set_query(Some("?q=str")).unwrap();
        assert_eq!(uri.as_str(), "/path?q=str#fragment");

        uri.set_query(Some("q=str")).unwrap();
        assert_eq!(uri.query(), Some("q=str"));
    }

    #[test]
    fn test_get_url_authority() {
        let url = Url::parse("https://user:example@example.com:8080").unwrap();
        let uri: AbsoluteUri = url.into();
        assert_eq!(
            uri.authority_or_namespace().as_deref(),
            Some("user:example@example.com:8080")
        );
    }

    #[test]
    fn test_uri_authority_or_namespace() {
        let tests = [
            ("https://www.example.com", Some("www.example.com")),
            ("urn:example:resource", Some("example")),
            (
                "https://username:password@example.com/path",
                Some("username:password@example.com"),
            ),
            ("http://127.0.0.0:3400", Some("127.0.0.0:3400")),
            (
                "https://username@example.com/somepath",
                Some("username@example.com"),
            ),
            ("mailto:example@example.com", None),
        ];

        for (input, expected) in tests {
            let absolute_uri = AbsoluteUri::parse(input).unwrap();
            assert_eq!(expected, absolute_uri.authority_or_namespace().as_deref());
        }

        let tests = [
            ("https://www.example.com", Some("www.example.com")),
            ("urn:example:com", Some("example")),
            (
                "https://username:password@example.com/path",
                Some("username:password@example.com"),
            ),
            ("http://127.0.0.0:3400", Some("127.0.0.0:3400")),
            (
                "https://username@example.com/somepath",
                Some("username@example.com"),
            ),
            ("mailto:example@example.com", None),
            ("/relative", None),
        ];

        for (input, expected) in tests {
            let uri = Uri::parse(input).unwrap();
            assert_eq!(expected, uri.authority_or_namespace().as_deref());
        }
    }

    #[test]
    fn test_fragment() {
        let tests = [
            ("https://www.example.com", None),
            ("urn:example:resource", None),
            (
                "https://username:password@example.com/path#fraggle-rock",
                Some("fraggle-rock"),
            ),
            ("https://example.com:3400/path#with-port", Some("with-port")),
            (
                "https://username:password@example.com/somepath#with-credentials",
                Some("with-credentials"),
            ),
            ("mailto:example@example.com", None),
        ];

        for (input, expected) in tests {
            let absolute_uri = AbsoluteUri::parse(input).unwrap();
            assert_eq!(expected, absolute_uri.fragment());
        }
        let tests = [
            ("https://www.example.com", None),
            ("urn:example:resource", None),
            (
                "https://username:password@example.com/path#fraggle-rock",
                Some("fraggle-rock"),
            ),
            ("https://example.com:3400/path#with-port", Some("with-port")),
            (
                "https://username:password@example.com/somepath#with-credentials",
                Some("with-credentials"),
            ),
            ("mailto:example@example.com", None),
            ("/relative#fragment", Some("fragment")),
            ("#fragment", Some("fragment")),
        ];

        for (input, expected) in tests {
            let uri = Uri::parse(input).unwrap();
            assert_eq!(expected, uri.fragment());
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_set_fragment() {
        let tests = [
            (
                "https://www.example.com/",
                None,
                None,
                "https://www.example.com/",
            ),
            (
                "https://username:password@example.com/path#fragment",
                Some("fragment/nested"),
                Some("fragment/nested"),
                "https://username:password@example.com/path#fragment/nested",
            ),
            (
                "https://example.com/path#with-fragment",
                None,
                None,
                "https://example.com/path",
            ),
            (
                "urn:example:resource",
                Some("fragment"),
                Some("fragment"),
                "urn:example:resource#fragment",
            ),
            (
                "urn:example:resource",
                Some("some fragment with spaces"),
                Some("some%20fragment%20with%20spaces"),
                "urn:example:resource#some%20fragment%20with%20spaces",
            ),
            (
                "https://example.com/path#with-fragment",
                Some("fragment with spaces"),
                Some("fragment%20with%20spaces"),
                "https://example.com/path#fragment%20with%20spaces",
            ),
        ];

        for (input, fragment, expected_fragment, expected_uri) in tests {
            let mut absolute_uri = AbsoluteUri::parse(input).unwrap();
            absolute_uri.set_fragment(fragment).unwrap();
            assert_eq!(expected_uri, absolute_uri.to_string());
            assert_eq!(expected_fragment, absolute_uri.fragment());
        }

        let tests = [
            (
                "https://www.example.com/",
                None,
                None,
                "https://www.example.com/",
            ),
            (
                "https://username:password@example.com/path#fragment",
                Some("fragment/nested"),
                Some("fragment/nested"),
                "https://username:password@example.com/path#fragment/nested",
            ),
            (
                "https://example.com/path#with-fragment",
                None,
                None,
                "https://example.com/path",
            ),
            (
                "urn:example:resource",
                Some("fragment"),
                Some("fragment"),
                "urn:example:resource#fragment",
            ),
            (
                "urn:example:resource",
                Some("some fragment with spaces"),
                Some("some%20fragment%20with%20spaces"),
                "urn:example:resource#some%20fragment%20with%20spaces",
            ),
            (
                "https://example.com/path#with-fragment",
                Some("fragment with spaces"),
                Some("fragment%20with%20spaces"),
                "https://example.com/path#fragment%20with%20spaces",
            ),
            (
                "/partial/path#existing-fragment",
                Some("new-fragment"),
                Some("new-fragment"),
                "/partial/path#new-fragment",
            ),
            (
                "#existing-fragment",
                Some("new-fragment"),
                Some("new-fragment"),
                "#new-fragment",
            ),
            ("#existing-fragment", None, None, ""),
            (
                "/partial/path#existing-fragment",
                None,
                None,
                "/partial/path",
            ),
            (
                "#existing-fragment",
                Some("new fragment with spaces"),
                Some("new%20fragment%20with%20spaces"),
                "#new%20fragment%20with%20spaces",
            ),
            (
                "/partial/path",
                Some("fragment%20with%20spaces"),
                Some("fragment%20with%20spaces"),
                "/partial/path#fragment%20with%20spaces",
            ),
        ];
        for (input, fragment, expected_fragment, expected_uri) in tests {
            let mut uri = Uri::parse(input).unwrap();
            uri.set_fragment(fragment).unwrap();
            assert_eq!(expected_uri, uri.to_string());
            assert_eq!(expected_fragment, uri.fragment());
        }
    }
    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_set_path() {
        let tests = [
            (
                "https://www.example.com",
                "/new-path",
                "/new-path",
                "https://www.example.com/new-path",
            ),
            (
                "https://username:password@example.com/path#fraggle-rock",
                "/new-path",
                "/new-path",
                "https://username:password@example.com/new-path#fraggle-rock",
            ),
            (
                "https://example.com/path#with-fragment",
                "",
                "/",
                "https://example.com/#with-fragment",
            ),
            (
                "urn:example:resource#fragment",
                "new-resource",
                "new-resource",
                "urn:example:new-resource#fragment",
            ),
            (
                "urn:example:resource",
                "new-resource",
                "new-resource",
                "urn:example:new-resource",
            ),
            (
                "https://example.com/",
                "new path",
                "/new%20path",
                "https://example.com/new%20path",
            ),
            (
                "urn:example:resource#fragment",
                "new resource",
                "new%20resource",
                "urn:example:new%20resource#fragment",
            ),
            (
                "urn:example:resource",
                "some path with spaces",
                "some%20path%20with%20spaces",
                "urn:example:some%20path%20with%20spaces",
            ),
        ];
        for (input, new_path, expected_path, expected) in tests {
            let mut absolute_uri = AbsoluteUri::parse(input).unwrap();
            absolute_uri.set_path_or_nss(new_path).unwrap();
            assert_eq!(expected, absolute_uri.to_string());
            assert_eq!(expected_path, absolute_uri.path_or_nss());
        }

        let tests = [
            (
                "https://www.example.com",
                "/new-path",
                "/new-path",
                "https://www.example.com/new-path",
            ),
            (
                "https://username:password@example.com/path#fraggle-rock",
                "/new-path",
                "/new-path",
                "https://username:password@example.com/new-path#fraggle-rock",
            ),
            (
                "https://example.com/path#with-fragment",
                "",
                "/",
                "https://example.com/#with-fragment",
            ),
            (
                "urn:example:resource#fragment",
                "new-resource",
                "new-resource",
                "urn:example:new-resource#fragment",
            ),
            (
                "urn:example:resource",
                "new-resource",
                "new-resource",
                "urn:example:new-resource",
            ),
            ("", "/new-path", "/new-path", "/new-path"),
            ("/", "/resource", "/resource", "/resource"),
            (
                "/path#fragment",
                "/new-path",
                "/new-path",
                "/new-path#fragment",
            ),
            (
                "https://example.com/",
                "new path",
                "/new%20path",
                "https://example.com/new%20path",
            ),
            (
                "urn:example:resource#fragment",
                "new resource",
                "new%20resource",
                "urn:example:new%20resource#fragment",
            ),
        ];
        for (input, new_path, expected_path, expected) in tests {
            let mut uri = Uri::parse(input).unwrap();
            uri.set_path_or_nss(new_path).unwrap();
            assert_eq!(expected, uri.to_string());
            assert_eq!(expected_path, uri.path_or_nss());
        }
    }
    #[test]
    fn first_doc_test() {
        let input =
            "https://john.doe@example.com:123/forum/questions/?tag=networking&order=newest#top";
        let uri = Uri::parse(input).unwrap();
        assert_eq!(&uri, input);
        assert_eq!(uri.scheme(), Some("https"));
        assert_eq!(uri.username(), Some("john.doe"));
        assert_eq!(uri.host().as_deref(), Some("example.com"));
        assert_eq!(uri.port(), Some(123));
        assert_eq!(uri.path_or_nss(), "/forum/questions/");
        assert_eq!(uri.query(), Some("tag=networking&order=newest"));
        assert_eq!(uri.fragment(), Some("top"));
        assert_eq!(
            uri.authority_or_namespace().unwrap(),
            "john.doe@example.com:123"
        );
        assert!(uri.is_url());

        let abs_uri = AbsoluteUri::parse(input).unwrap();
        assert_eq!(uri, abs_uri);
    }
    #[test]
    fn doc_tests() {
        let input =
            "https://john.doe@example.com:123/forum/questions/?tag=networking&order=newest#top";
        let uri = Uri::parse(input).unwrap();
        assert_eq!(&uri, input);
        assert_eq!(uri.scheme(), Some("https"));
        assert_eq!(uri.username(), Some("john.doe"));
        assert_eq!(uri.host().as_deref(), Some("example.com"));
        assert_eq!(uri.port(), Some(123));
        assert_eq!(uri.path_or_nss(), "/forum/questions/");
        assert_eq!(uri.query(), Some("tag=networking&order=newest"));
        assert_eq!(uri.fragment(), Some("top"));
        assert_eq!(
            uri.authority_or_namespace().unwrap(),
            "john.doe@example.com:123"
        );
        assert!(uri.is_url());

        let abs_uri = AbsoluteUri::parse(input).unwrap();
        assert_eq!(uri, abs_uri);

        let input = "urn:example:articles:record";
        let uri = Uri::parse(input).unwrap();
        assert_eq!(&uri, input);
        assert_eq!(uri.scheme(), Some("urn"));
        assert_eq!(uri.username(), None);
        assert_eq!(uri.authority_or_namespace().as_deref(), Some("example"));
        assert_eq!(uri.port(), None);
        assert_eq!(uri.path_or_nss(), "articles:record");
        assert!(uri.is_urn());
        let abs_uri = AbsoluteUri::parse(input).unwrap();
        assert_eq!(uri, abs_uri);

        let input =
            "//john.doe:password@example.com:123/forum/questions/?tag=networking&order=newest#top";
        let uri = Uri::parse(input).unwrap();
        assert_eq!(&uri, input);
        assert_eq!(uri.scheme(), None);
        assert_eq!(uri.username(), Some("john.doe"));
        assert_eq!(uri.password(), Some("password"));
        assert_eq!(uri.path_or_nss(), "/forum/questions/");
        assert_eq!(uri.host().as_deref(), Some("example.com"));
        assert_eq!(
            uri.authority_or_namespace().as_deref(),
            Some("john.doe:password@example.com:123")
        );
        assert_eq!(uri.port(), Some(123));
        assert_eq!(uri.query(), Some("tag=networking&order=newest"));
        assert_eq!(uri.fragment(), Some("top"));

        let s = "/forum/questions/?tag=networking&order=newest#top";
        let uri = Uri::parse(s).unwrap();
        assert_eq!(&uri, s);
        assert_eq!(uri.path_or_nss(), "/forum/questions/");
        assert_eq!(uri.scheme(), None);
        assert_eq!(uri.username(), None);
        assert_eq!(uri.authority_or_namespace(), None);
        assert_eq!(uri.port(), None);
        assert_eq!(uri.query(), Some("tag=networking&order=newest"));
        assert_eq!(uri.fragment(), Some("top"));
    }
    #[test]
    fn test_fragment_decoded_lossy() {
        let uri = AbsoluteUri::parse("https://example.com/#/patternProperties/^%C3%A1").unwrap();
        println!("{uri}");
        println!("{:?}", uri.fragment_decoded_lossy());
    }

    #[test]
    fn test_uri() {
        let uri = uri!("https://example.com");
        dbg!(uri);
    }
}
