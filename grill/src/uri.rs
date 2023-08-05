//! Data structures to represent Uniform Resource Identifiers (URI) [RFC 3986](https://tools.ietf.org/html/rfc3986).

#[cfg(test)]
mod tests;

mod encode;
mod parse;
mod write;

#[doc(no_inline)]
pub use url::Url;
#[doc(no_inline)]
pub use urn::Urn;

use crate::error::{AuthorityError, OverflowError, RelativeUriError, UriError, UrnError};
use inherent::inherent;
use percent_encoding::percent_decode;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Write},
    iter::Peekable,
    ops::{Deref, Index},
    path::PathBuf,
    str::{FromStr, Split},
    string::{String, ToString},
};
use urn::percent::{encode_f_component, encode_nss};

pub trait TryIntoAbsoluteUri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError>;
}

pub trait ToUri {
    fn to_uri(&self) -> Uri;
}

pub trait AsUriRef {
    fn as_uri_ref(&self) -> UriRef<'_>;
}
#[derive(Clone, Copy, Debug)]
pub enum UriRef<'a> {
    Uri(&'a Uri),
    AbsoluteUri(&'a AbsoluteUri),
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
    pub fn as_relative(&self) -> Option<&'a RelativeUri> {
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

    /// Returns the path (if this `Uri` is a [`Url`]) or namespace specific
    /// string (if this `Uri` is a [`Urn`])
    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            UriRef::Uri(uri) => uri.path_or_nss(),
            UriRef::AbsoluteUri(uri) => uri.path_or_nss(),
            UriRef::RelativeUri(uri) => uri.path(),
        }
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

    #[must_use]
    pub fn query(&self) -> Option<&str> {
        match self {
            UriRef::Uri(uri) => uri.query(),
            UriRef::AbsoluteUri(uri) => uri.query(),
            UriRef::RelativeUri(uri) => uri.query(),
        }
    }
}

impl Deref for UriRef<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsUriRef for Uri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::Uri(self)
    }
}
impl AsUriRef for AbsoluteUri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::AbsoluteUri(self)
    }
}

impl AsUriRef for RelativeUri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::RelativeUri(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
/// The Authority component of a Relative URI.
pub struct Authority<'a> {
    value: Cow<'a, str>,
    username_idx: Option<u32>,
    password_idx: Option<u32>,
    host_idx: Option<u32>,
    port_idx: Option<u32>,
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
        let start = self.username_idx()?;
        let end = self
            .password_idx
            .or(self.host_idx)
            .or(self.port_idx)
            .map_or(self.value.len(), |idx| idx as usize);

        Some(&self.value[start..end])
    }

    /// Returns the password component if it exists.
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        let start = self.password_idx()?;
        let end = self
            .host_idx()
            .or(self.port_idx())
            .unwrap_or(self.value.len());
        Some(&self.value[start..end])
    }

    /// Returns the host component if it exists.
    #[must_use]
    pub fn host(&self) -> Option<&str> {
        let offset = usize::from(self.username_idx.is_some() || self.password_idx.is_some());
        let start = self.host_idx()? + offset;
        let end = self.port_idx().unwrap_or(self.value.len());
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
        self.port_idx().map(|idx| &self.value[idx + 1..])
    }

    #[must_use]
    pub fn to_owned(&self) -> Authority<'static> {
        Authority {
            value: Cow::Owned(self.value.to_string()),
            username_idx: self.username_idx,
            password_idx: self.password_idx,
            host_idx: self.host_idx,
            port_idx: self.port_idx,
            port: self.port,
        }
    }

    /// Returns the `&str` representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    fn port_idx(&self) -> Option<usize> {
        self.port_idx.map(|idx| idx as usize)
    }
    fn host_idx(&self) -> Option<usize> {
        self.host_idx.map(|idx| idx as usize)
    }
    fn username_idx(&self) -> Option<usize> {
        self.username_idx.map(|idx| idx as usize)
    }
    fn password_idx(&self) -> Option<usize> {
        self.password_idx.map(|idx| idx as usize)
    }
}

/// A URI in the form of a fully qualified [`Url`] or [`Urn`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum AbsoluteUri {
    Url(Url),
    Urn(Urn),
}

impl AbsoluteUri {
    /// Attempts to parse an `AbsoluteUri`.
    ///
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if `value` can not be parsed as a
    /// [`Url`](`url::Url`) or [`Urn`](`urn::Urn`)
    pub fn parse(value: &str) -> Result<Self, UriError> {
        if value.starts_with("urn:") {
            Ok(Urn::from_str(value)?.into())
        } else {
            Ok(Url::parse(value)?.into())
        }
    }

    /// Returns a new [`Components`] iterator over all components (scheme,
    /// username (URL), password (URL), host (URL) or namespace (URN), port (URL), path (URL) or namespace specific
    /// string (URN), query (URL or URN), and fragment (URL or URN)) of this `AbsoluteUri`.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::{ AbsoluteUri, Component };
    /// let uri_str = "https://username:password@example.com/path/to/file/?query=string#fragment";
    /// let uri = AbsoluteUri::parse(uri_str).unwrap();
    /// let components = vec![
    ///     Component::Scheme("https".into()),
    ///     Component::Username("username".into()),
    ///     Component::Password("password".into()),
    ///     Component::Host("example.com".into()),
    ///     Component::Path("/path/to/file/".into()),
    ///     Component::Query("query=string".into()),
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

    /// Returns a [`PathSegments`] iterator over the base path segments,
    /// essentially every segment except the last.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::{ Uri };
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
            Self::Url(url) => Ok(set_url_fragment(url, fragment)),
            Self::Urn(urn) => set_urn_fragment(urn, fragment),
        }
    }

    /// Returns the authority (`Url`) or namespace (`Urn`)
    #[must_use]
    pub fn authority_or_namespace(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::Url(url) => get_url_authority(url).map(Cow::Owned),
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
            Self::Url(url) => Ok(set_url_path(url, path_or_nss)),
            Self::Urn(urn) => set_urn_nss(urn, path_or_nss),
        }
    }

    /// Sets the authority (`Url`) or namespace (`Urn`)
    pub fn set_authority_or_namespace(
        &mut self,
        authority_or_namespace: &str,
    ) -> Result<Option<String>, UriError> {
        match self {
            Self::Url(u) => set_url_authority(u, authority_or_namespace),
            Self::Urn(u) => set_urn_namespace(u, authority_or_namespace),
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

    /// returns a new `AbsoluteUri` that is the result of resolving the given
    /// reference against this `AbsoluteUri`.
    ///
    /// See [RFC3986, Section
    /// 5.2.2](https://tools.ietf.org/html/rfc3986#section-5.2.2).
    pub fn resolve(&self, reference: &impl AsUriRef) -> Result<AbsoluteUri, UriError> {
        let reference = reference.as_uri_ref();
        todo!()
        // if let Some(scheme) = reference.scheme() {
        //     result
        //         .set_scheme(scheme)
        //         .map_err(|_| UriError::InvalidScheme(scheme.to_string()))?;
        // }
        // if let Some(authority) = reference.authority_or_namespace() {
        //     result.set_authority_or_namespace(&authority)?;
        // }

        // result.set_path_or_nss(reference.path_or_nss())?;
        // result.set_query(reference.query())?;
        // result.set_fragment(reference.fragment())?;
        // todo!()
        /*
        if ((not strict) and (R.scheme == Base.scheme)) then
           undefine(R.scheme);
        endif;
        if defined(R.scheme) then
           T.scheme    = R.scheme;
           T.authority = R.authority;
           T.path      = remove_dot_segments(R.path);
           T.query     = R.query;
        else
           if defined(R.authority) then
              T.authority = R.authority;
              T.path      = remove_dot_segments(R.path);
              T.query     = R.query;
           else
              if (R.path == "") then
                 T.path = Base.path;
                 if defined(R.query) then
                    T.query = R.query;
                 else
                    T.query = Base.query;
                 endif;
              else
                 if (R.path starts-with "/") then
                    T.path = remove_dot_segments(R.path);
                 else
                    T.path = merge(Base.path, R.path);
                    T.path = remove_dot_segments(T.path);
                 endif;
                 T.query = R.query;
              endif;
              T.authority = Base.authority;
           endif;
           T.scheme = Base.scheme;
        endif;

        T.fragment = R.fragment;

          */
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
        let to_uri_err = |_| UriError::InvalidScheme(scheme.to_string());
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
}

/// Normalizes a URI path by removing dot segments, i.e. `'.'`, `'..'`.
#[must_use]
pub fn normalize_path(path: &str) -> String {
    let mut buf = PathBuf::new();
    for segment in PathSegments::new(path) {
        match segment {
            PathSegment::Parent => {
                buf.pop();
            }
            PathSegment::Normal(seg) => buf.push(seg.as_ref()),
            _ => {}
        }
    }
    // safety: path is already in utf8
    buf.to_str().unwrap().to_string()
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

impl AsRef<str> for AbsoluteUri {
    fn as_ref(&self) -> &str {
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
        PartialOrd::partial_cmp(self.as_str(), other.as_str())
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

impl<'a> TryIntoAbsoluteUri for UriRef<'a> {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
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
impl TryIntoAbsoluteUri for AbsoluteUri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        Ok(self)
    }
}
impl TryIntoAbsoluteUri for &AbsoluteUri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        Ok(self.clone())
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
impl TryIntoAbsoluteUri for Uri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        match self {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url)),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn)),
            Uri::Relative(rel) => AbsoluteUri::parse(rel.as_str()),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RelativeUri {
    value: String,
    username_idx: Option<u32>,
    password_idx: Option<u32>,
    host_idx: Option<u32>,
    port_idx: Option<u32>,
    port: Option<u16>,
    path_idx: u32,
    query_idx: Option<u32>,
    fragment_idx: Option<u32>,
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
    /// use grill::uri::{ Uri, Component };
    /// let uri = Uri::parse("//username:password@example.com/path/to/file/?query=string#fragment").unwrap();
    /// let uri = uri.as_relative_uri().unwrap();
    /// let components = vec![
    ///     Component::Username("username".into()),
    ///     Component::Password("password".into()),
    ///     Component::Host("example.com".into()),
    ///     Component::Path("/path/to/file/".into()),
    ///     Component::Query("query=string".into()),
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
    /// use grill::uri::{ Uri };
    /// let uri = Uri::parse("/path/to/file");
    /// let uri = uri.as_relative_uri().unwrap();
    /// assert_eq!(uri.path_segments().collect::<Vec<_>>(), vec!["path", "to", "file"]);
    /// ```
    #[must_use]
    pub fn path_segments(&self) -> PathSegments {
        PathSegments::from(self.path())
    }

    /// Returns a [`PathSegments`] iterator over the base path segments,
    /// essentially every segment except the last.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::{ Uri };
    /// let uri = Uri::parse("/path/to/file");
    /// let relative_uri = uri.as_relative_uri().unwrap();
    /// assert_eq!(relative_uri.base_path_segments().collect::<Vec<_>>(), vec!["path", "to"]);
    #[must_use]
    pub fn base_path_segments(&self) -> PathSegments<'_> {
        let mut segments = self.path_segments();
        segments.base_only = true;
        segments
    }

    /// Returns the path segment of the `RelativeUri`.
    #[must_use]
    pub fn path(&self) -> &str {
        let end = self
            .query_idx()
            .or(self.fragment_idx())
            .unwrap_or(self.value.len());
        &self.value[self.path_idx()..end]
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
        let start = self.username_idx()?;
        let end = self.username_end_idx()?;
        Some(&self.value[start..end])
    }
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        let start = self.password_idx()? + 1;
        let end = self.host_idx().unwrap_or(self.path_idx());
        Some(&self.value[start..end])
    }

    fn username_end_idx(&self) -> Option<usize> {
        self.username_idx?;
        self.password_idx()
            .or(self.host_idx())
            .unwrap_or(self.path_idx())
            .into()
    }

    /// Returns the path of the `RelativeUri` if it exists.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        let fragment_idx = self.fragment_idx()?;
        if fragment_idx + 1 == self.len() {
            return Some("");
        }

        Some(&self.value[fragment_idx + 1..])
    }

    /// Returns the query string segment of the `RelativeUri`, if it exists.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        let query_idx = self.query_idx()?;
        if query_idx + 1 == self.len() {
            return Some("");
        }
        let last = self.fragment_idx().unwrap_or(self.len());
        Some(&self.value[query_idx + 1..last])
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
        let username_idx = write::username(&mut buf, self.username())?;
        let password_idx = write::password(&mut buf, self.password())?;
        let host_idx = write::host(&mut buf, self.host())?;
        let port_idx = write::port(&mut buf, self.port_str())?;
        let path_idx = write::path(&mut buf, self.path())?;
        let query_idx = write::query(
            &mut buf,
            encode::query(query),
            self.has_authority(),
            self.has_path(),
        )?;
        let fragment_idx: Option<u32> = write::fragment(&mut buf, self.fragment())?;
        self.value = buf;
        self.username_idx = username_idx;
        self.password_idx = password_idx;
        self.host_idx = host_idx;
        self.port_idx = port_idx;
        self.path_idx = path_idx;
        self.query_idx = query_idx;
        self.fragment_idx = fragment_idx;
        Ok(existing_query)
    }

    /// Sets the path of the `RelativeUri` and returns the previous path.
    ///
    /// Note, fragments are left intact. Use `set_fragment` to change the fragment.
    pub fn set_path(&mut self, path: &str) -> Result<String, RelativeUriError> {
        let existing_path = self.path().to_string();
        let mut buf = String::with_capacity(self.len() - existing_path.len() + path.len());
        let username_idx = write::username(&mut buf, self.username())?;
        let password_idx = write::password(&mut buf, self.password())?;
        let host_idx = write::host(&mut buf, self.host())?;
        let port_idx = write::port(&mut buf, self.port_str())?;
        let path_idx = write::path(&mut buf, encode::path(path))?;
        let query_idx = write::query(
            &mut buf,
            self.query(),
            self.has_authority(),
            self.has_path(),
        )?;
        let fragment_idx: Option<u32> = write::fragment(&mut buf, self.fragment())?;
        self.value = buf;
        self.username_idx = username_idx;
        self.password_idx = password_idx;
        self.host_idx = host_idx;
        self.port_idx = port_idx;
        self.path_idx = path_idx;
        self.query_idx = query_idx;
        self.fragment_idx = fragment_idx;
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
        let username_idx = write::username(&mut buf, self.username())?;
        let password_idx = write::password(&mut buf, self.password())?;
        let host_idx = write::host(&mut buf, self.host())?;
        let port_idx = write::port(&mut buf, self.port_str())?;
        let path_idx: u32 = write::path(&mut buf, self.path())?;
        let query_idx: Option<u32> = write::query(
            &mut buf,
            self.query(),
            self.has_authority(),
            self.has_path(),
        )?;
        let fragment_idx: Option<u32> = write::fragment(&mut buf, encode::fragment(fragment))?;
        self.value = buf;
        self.username_idx = username_idx;
        self.password_idx = password_idx;
        self.host_idx = host_idx;
        self.port_idx = port_idx;
        self.path_idx = path_idx;
        self.query_idx = query_idx;
        self.fragment_idx = fragment_idx;
        Ok(existing_fragment)
    }

    #[must_use]
    /// Returns the authority of the `RelativeUri` if it exists.
    ///
    /// A relative URI may have an authority if it starts starts with `"//"`.
    pub fn authority(&self) -> Option<Authority> {
        let host_idx = self.host_idx()?;
        Some(Authority {
            value: Cow::Borrowed(&self.value[host_idx..self.path_idx()]),
            username_idx: self.username_idx,
            password_idx: self.password_idx,
            host_idx: self.host_idx,
            port_idx: self.port_idx,
            port: self.port,
        })
    }

    #[must_use]
    pub fn host(&self) -> Option<&str> {
        let mut start = self.host_idx()?;
        if self.has_username() || self.has_password() {
            start += 1;
        }
        let end = self.port_idx().unwrap_or_else(|| self.path_idx());
        Some(&self.value[start..end])
    }

    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Sets the authority of the `RelativeUri` and returns the previous
    /// authority, if it existed.
    pub fn set_authority<'a>(
        &'a mut self,
        authority: Option<&str>,
    ) -> Result<Option<Authority<'a>>, UriError> {
        let existing_authority = self.authority().map(|a| a.to_owned());
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
        let username_idx = write::username(&mut buf, new.username())?;
        let password_idx = write::password(&mut buf, new.password())?;
        let host_idx = write::host(&mut buf, new.host())?;
        let port_idx = write::port(&mut buf, new.port_str())?;
        let path_idx: u32 = write::path(&mut buf, self.path())?;
        let query_idx: Option<u32> = write::query(
            &mut buf,
            self.query(),
            self.has_authority(),
            self.has_path(),
        )?;
        let fragment_idx = write::fragment(&mut buf, self.fragment())?;
        self.value = buf;
        self.username_idx = username_idx;
        self.password_idx = password_idx;
        self.host_idx = host_idx;
        self.port_idx = port_idx;
        self.path_idx = path_idx;
        self.query_idx = query_idx;
        self.fragment_idx = fragment_idx;
        Ok(existing_authority)
    }

    #[must_use]
    pub fn has_authority(&self) -> bool {
        self.path_idx() > 2
    }

    #[must_use]
    pub fn has_username(&self) -> bool {
        self.username_idx.is_some()
    }

    #[must_use]
    pub fn has_password(&self) -> bool {
        self.password_idx.is_some()
    }

    #[must_use]
    pub fn has_host(&self) -> bool {
        self.host_idx.is_some()
    }

    #[must_use]
    pub fn has_port(&self) -> bool {
        self.port_idx.is_some()
    }

    #[must_use]
    pub fn has_query(&self) -> bool {
        self.query_idx.is_some()
    }

    #[must_use]
    pub fn has_fragment(&self) -> bool {
        self.fragment_idx.is_some()
    }

    fn authority_str(&self) -> Option<&str> {
        let start = self.username_idx().or(self.host_idx())?;
        Some(&self.value[start..self.path_idx()])
    }

    fn path_idx(&self) -> usize {
        self.path_idx as usize
    }

    fn fragment_idx(&self) -> Option<usize> {
        self.fragment_idx.map(|idx| idx as usize)
    }

    fn query_idx(&self) -> Option<usize> {
        self.query_idx.map(|idx| idx as usize)
    }

    fn username_idx(&self) -> Option<usize> {
        self.username_idx.map(|idx| idx as usize)
    }

    fn host_idx(&self) -> Option<usize> {
        self.host_idx.map(|idx| idx as usize)
    }

    fn port_idx(&self) -> Option<usize> {
        self.port_idx.map(|idx| idx as usize)
    }

    fn password_idx(&self) -> Option<usize> {
        self.password_idx.map(|idx| idx as usize)
    }

    fn port_str(&self) -> Option<&str> {
        self.port_idx()
            .map(|idx| &self.value[idx + 1..self.path_idx()])
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Uri {
    Url(Url),
    Urn(Urn),
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
impl PartialEq<Uri> for AbsoluteUri {
    fn eq(&self, other: &Uri) -> bool {
        self.as_str() == other.as_str()
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

    /// Returns a new [`Components`] iterator over all components (scheme,
    /// username (URL), password (URL), host (URL) or namespace (URN), port
    /// (URL), path (URL) or namespace specific string (URN), query (URL or
    /// URN), and fragment (URL or URN)) of this `RelativeUri`.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::{ Uri, Component };
    /// let uri_str = "https://username:password@example.com/path/to/file/?query=string#fragment";
    /// let uri = Uri::parse(uri_str).unwrap();
    /// let components = vec![
    ///     Component::Scheme("https".into()),
    ///     Component::Username("username".into()),
    ///     Component::Password("password".into()),
    ///     Component::Host("example.com".into()),
    ///     Component::Path("/path/to/file/".into()),
    ///     Component::Query("query=string".into()),
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
    /// use grill::uri::{ Uri };
    /// let uri = Uri::parse("https://example.com/path/to/file");
    /// assert_eq!(uri.path_segments().collect::<Vec<_>>(), vec!["path", "to", "file"]);
    /// ```
    #[must_use]
    pub fn path_segments(&self) -> PathSegments {
        PathSegments::from(self.path_or_nss())
    }

    /// Returns a [`PathSegments`] iterator over the base path segments,
    /// essentially every segment except the last.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::{ Uri };
    /// let uri = Uri::parse("/path/to/file");
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
            Uri::Url(url) => Ok(set_url_fragment(url, fragment)),
            Uri::Urn(urn) => set_urn_fragment(urn, fragment),
            Uri::Relative(rel) => Ok(rel.set_fragment(fragment)?),
        }
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

    #[must_use]
    pub fn query(&self) -> Option<&str> {
        match self {
            Uri::Url(url) => url.query(),
            Uri::Urn(urn) => urn.q_component(),
            Uri::Relative(rel) => rel.query(),
        }
    }

    /// Returns the namespace if the absolute uri is [`Urn`], otherwise returns
    /// the authority string for a [`Url`] or [`RelativeUri`].
    #[must_use]
    pub fn authority_or_namespace(&self) -> Option<Cow<'_, str>> {
        match self {
            Uri::Url(url) => url_authority(url),
            Uri::Urn(urn) => Some(Cow::Borrowed(urn.nid())),
            Uri::Relative(rel) => rel.authority_str().map(Cow::Borrowed),
        }
    }

    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            Self::Url(url) => url.path(),
            Self::Urn(urn) => urn.nss(),
            Self::Relative(rel) => rel.path(),
        }
    }
    /// Sets the path for a `Uri` in the shame of a [`Url`] or [`RelativeUri`])
    /// or the namespace specific string for a [`Urn`]
    /// # Errors
    pub fn set_path_or_nss(&mut self, path_or_nss: &str) -> Result<String, UriError> {
        match self {
            Self::Url(url) => Ok(set_url_path(url, path_or_nss)),
            Self::Urn(urn) => set_urn_nss(urn, path_or_nss),
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
        PartialOrd::partial_cmp(self.as_str(), other.as_str())
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

fn get_url_authority(u: &Url) -> Option<String> {
    if !u.has_authority() {
        return None;
    }
    let host = u.host();
    let port = u.port();
    let username = u.username();
    let password = u.password();
    let mut prev_authority = String::new();
    if !username.is_empty() {
        prev_authority.push_str(username);
        if let Some(password) = password {
            prev_authority.push(':');
            prev_authority.push_str(password);
        }
    }
    if let Some(host) = host {
        if !prev_authority.is_empty() {
            prev_authority.push('@');
        }
        prev_authority.push_str(host.to_string().as_str());
    }
    if let Some(port) = port {
        if !prev_authority.is_empty() {
            prev_authority.push(':');
        }
        prev_authority.push_str(&port.to_string());
    }
    Some(prev_authority)
}

fn set_urn_namespace(u: &mut Urn, namespace: &str) -> Result<Option<String>, UriError> {
    let prev_namespace = u.nid().to_string();
    u.set_nid(namespace)?;
    Ok(Some(prev_namespace))
}

fn set_url_authority(u: &mut Url, authority: &str) -> Result<Option<String>, UriError> {
    let prev_authority = get_url_authority(u);
    let authority = parse::authority(authority)?;
    if u.set_username(authority.username().unwrap_or_default())
        .is_err()
    {
        // the url crate doesn't check for empty values before returning `Err(())`
        // https://github.com/servo/rust-url/issues/844
        let username = authority.username().unwrap_or_default();
        if !username.is_empty() {
            return Err(AuthorityError::UsernameNotAllowed(username.to_string()).into());
        }
    }
    if u.set_password(authority.password()).is_err() {
        // the url crate doesn't check for empty values before returning `Err(())`
        // https://github.com/servo/rust-url/issues/844
        let password = authority.password().unwrap_or_default();
        if !password.is_empty() {
            return Err(AuthorityError::PasswordNotAllowed(password.to_string()).into());
        }
    }
    u.set_host(authority.host())?;
    if u.set_port(authority.port()).is_err() {
        // the url crate doesn't check for empty values before returning `Err(())`
        // https://github.com/servo/rust-url/issues/844
        if let Some(port) = authority.port() {
            return Err(AuthorityError::PortNotAllowed(port).into());
        }
    }
    Ok(prev_authority)
}

fn set_urn_fragment(urn: &mut Urn, fragment: Option<&str>) -> Result<Option<String>, UriError> {
    let existing = urn.f_component().map(ToString::to_string);
    // safety: encode_f_component does not currently return an error.
    let fragment = fragment.map(encode_f_component).map(Result::unwrap);
    urn.set_f_component(fragment.as_deref())?;
    Ok(existing)
}

fn set_url_fragment(url: &mut Url, fragment: Option<&str>) -> Option<String> {
    let existing = url.fragment().map(ToString::to_string);
    url.set_fragment(fragment);
    existing
}

fn set_urn_nss(urn: &mut Urn, nss: &str) -> Result<String, UriError> {
    let existing = urn.nss().to_string();
    urn.set_nss(&encode_nss(nss)?)?;
    Ok(existing)
}

fn set_url_path(url: &mut Url, path: &str) -> String {
    let existing = url.path().to_string();
    url.set_path(path);
    existing
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

/// A single component of a URI (i.e. scheme, authority, path, query, fragment):
/// - [`Scheme`](Component::Scheme) (e.g., `"https"` in
///   `"https://example.com"`),
/// - [`Username`](Component::Username) (e.g., `"username"` in
///   `"https://username:password@example.com"`),
/// - [`Password`](Component::Password) (e.g., `"password"` in
///   `"https://username:password@example.com"`),
/// - [`Host`](Component::Host) (e.g., `"example.com"` in
///   `"https://username:password@example.com"`),
/// - [`Path`](Component::Path) (e.g., `"/foo/bar"` in
///   `"https://example.com/foo/bar"`),
/// - [`Query`](Component::Query) (e.g., `"baz=qux"` in
///   `"https://example.com/foo/bar?baz=qux"`),
/// - [`Fragment`](Component::Fragment) (e.g., `"quux"` in
///   `"https://example.com/foo/bar?baz=qux#quux"`).
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

    /// The path of the URI, e.g., `"/path"` in `"https://example.com/path"`.
    Path(Cow<'a, str>),

    /// The query of the URI, e.g., `"query=str"` in
    /// `"https://example.com/?query=str"`.
    Query(Cow<'a, str>),

    /// The fragment of the URI, e.g., `"fragment"` in .
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
            | Component::Path(s)
            | Component::Query(s)
            | Component::Fragment(s) => s,
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
    path: Option<Cow<'a, str>>,
    query: Option<Cow<'a, str>>,
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
            path: Some(rel.path().into()),
            query: rel.query().map(Into::into),
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
            path: Some(urn.nss().into()),
            query: urn.q_component().map(Into::into),
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
            path: Some(url.path().into()),
            query: url.query().map(Into::into),
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
        if let Some(path) = self.path.take() {
            return Some(Component::Path(path));
        }
        if let Some(query) = self.query.take() {
            return Some(Component::Query(query));
        }
        if let Some(fragment) = self.fragment.take() {
            return Some(Component::Fragment(fragment));
        }
        None
    }
}

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
            "." | ".." => Self::resolve_dots(val, next),
            _ => Self::Normal(val.into()),
        }
    }
    fn parse_path_segment(val: &'a str, next: Option<char>) -> Self {
        match val {
            "." | ".." => Self::resolve_dots(val, next),
            _ => Self::Normal(val.into()),
        }
    }
    fn resolve_dots(val: &'a str, next: Option<char>) -> Self {
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

/// An [`Iterator`] of path [`PathSegment`]s.
pub struct PathSegments<'a> {
    path: Peekable<Split<'a, char>>,
    base_only: bool,
    root_sent: bool,
}

impl<'a> PathSegments<'a> {
    #[must_use]
    pub fn new(path: &'a str) -> Self {
        Self {
            path: path.split('/').peekable(),
            root_sent: false,
            base_only: false,
        }
    }

    fn peek_next(&mut self) -> Option<char> {
        self.path.peek().and_then(|s| s.chars().next())
    }
}

impl<'a> From<&'a str> for PathSegments<'a> {
    fn from(path: &'a str) -> Self {
        Self {
            path: path.split('/').peekable(),
            root_sent: false,
            base_only: false,
        }
    }
}

impl<'a> Iterator for PathSegments<'a> {
    type Item = PathSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.path.next()?;
        if self.root_sent {
            Some(PathSegment::parse_path_segment(val, self.peek_next()))
        } else {
            self.root_sent = true;
            let next = self.peek_next();
            if self.base_only && next.is_none() {
                None
            } else {
                Some(PathSegment::parse_root(val, next))
            }
        }
    }
}

#[inline]
fn to_u32(v: usize) -> Result<u32, OverflowError> {
    if v > u32::MAX as usize {
        Err(OverflowError(v))
    } else {
        #[allow(clippy::cast_possible_truncation)]
        Ok(v as u32)
    }
}

fn url_authority(url: &Url) -> Option<Cow<'_, str>> {
    let mut result = String::default();
    let host = url.host()?;
    if !url.username().is_empty() {
        result.write_str(url.username()).unwrap();
        if let Some(password) = url.password() {
            result.write_char(':').unwrap();
            result.write_str(password).unwrap();
        }
        result.write_char('@').unwrap();
    }
    result.write_str(&host.to_string()).unwrap();
    if let Some(port) = url.port() {
        result.write_char(':').unwrap();
        result.write_str(&port.to_string()).unwrap();
    }
    Some(result.to_string().into())
}
