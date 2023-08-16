use super::{
    AbsoluteUri, PathSegment, PathSegments, QueryParameter, QueryParameters, RelativeUri, Uri,
};
use percent_encoding::percent_decode;
use std::{borrow::Cow, ops::Deref};
use url::Url;
use urn::Urn;

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
