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

mod absolute_uri;
pub use absolute_uri::{AbsoluteUri, TryIntoAbsoluteUri};

mod authority;
pub use authority::Authority;

mod component;
pub use component::{Component, Components};

mod path_segment;
pub use path_segment::{PathSegment, PathSegments};

mod query_parameter;
pub use query_parameter::{QueryParameter, QueryParameters};

mod relative_uri;
pub use relative_uri::RelativeUri;

mod uri;
pub use uri::{ToUri, Uri};

pub use uri_ref::{AsUriRef, UriRef};
mod uri_ref;

// ┌───────────────────────────────────────────┐
// │                 RE-EXPORTS                │
// └───────────────────────────────────────────┘

pub use url::Url;
pub use urn::Urn;

// ┌───────────────────────────────────────────┐
// │                  INTERNAL                 │
// └───────────────────────────────────────────┘

mod get;
mod parse;
mod set;
mod write;

// TODO: encode and path could be made public. Is it worth the surface?

mod encode;
mod path;

#[cfg(test)]
mod test;
