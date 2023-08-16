use inherent::inherent;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;
use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
};
use url::Url;
use urn::Urn;

use crate::RelativeUri;
use crate::{
    error::{UriError, UrnError},
    Uri,
};

use super::{get, path, set, AsUriRef, Components, PathSegments, QueryParameters, UriRef};

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
        let base = path::merge(base, reference.path());
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
        path::normalize(self.path_or_nss())
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
        let normalized = path::normalize(self.path_or_nss()).to_string();
        self.set_path_or_nss(&normalized).unwrap();
    }

    /// Returns `true` if this `AbsoluteUri` has a query component.
    #[must_use]
    pub fn has_query(&self) -> bool {
        self.query().is_some()
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
        println!("{} == {other} = {}", self.as_str(), self.as_str() == *other);
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
