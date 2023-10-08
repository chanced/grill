#[doc(no_inline)]
pub use url::Url;
#[doc(no_inline)]
pub use urn::Urn;

use super::{path, set, *};

use crate::error::UriError;
use inherent::inherent;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::Display,
    ops::Deref,
    str::FromStr,
    string::{String, ToString},
};

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
        let base = path::merge(base, reference.path());
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
        path::normalize(self.path_or_nss())
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
        let normalized = path::normalize(self.path_or_nss()).to_string();
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
