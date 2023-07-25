//! Data structures to represent Uniform Resource Identifiers (URI) [RFC 3986](https://tools.ietf.org/html/rfc3986).

use crate::error::{RelativeUriError, UriError, UrnError};
use inherent::inherent;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    ops::Deref,
    str::FromStr,
    string::ToString,
};
use url::Url;
use urn::{
    percent::{encode_f_component, encode_nss},
    Urn,
};

const URL_FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const URL_PATH: &AsciiSet = &URL_FRAGMENT.add(b'#').add(b'?').add(b'{').add(b'}');

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
    /// Returns [`AbsoluteUriParseError`] if `value` can not be parsed as a [`Url`](`url::Url`) or [`Urn`](`urn::Urn`)
    pub fn parse(value: &str) -> Result<Self, UriError> {
        if value.starts_with("urn:") {
            Ok(Urn::from_str(value)?.into())
        } else {
            Ok(Url::parse(value)?.into())
        }
    }
    /// Returns the percent encoded fragment, if it exists.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        match self {
            Self::Url(url) => url.fragment(),
            Self::Urn(urn) => urn.f_component(),
        }
    }

    /// Percent encodes and sets the fragment component of the [`Url`] or [`Urn`] and returns the
    /// previous fragment in percent-encoded format if it exists.
    ///
    /// # Errors
    /// Returns [`urn::Error`](`urn::Error`) if the `AbsoluteUri` is a
    /// [`Urn`](`urn::Urn`) and the fragment and the fragment fails validation.
    pub fn set_fragment(&mut self, fragment: Option<&str>) -> Option<String> {
        match self {
            Self::Url(url) => set_url_fragment(url, fragment),
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

    /// Returns the path (url) or Name Specific String (urn)
    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            Self::Url(url) => url.path(),
            Self::Urn(urn) => urn.nss(),
        }
    }
    /// Sets the path (`Url`) or Name Specific String (`Urn`)
    pub fn set_path_or_nss(&mut self, path_or_nss: &str) -> Result<String, UrnError> {
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

pub trait TryIntoAbsoluteUri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError>;
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

// TODO: consider whether or not the idxs in RelativeUri should be u32
// Likely but this would muddy up the API a bit.
// is it worth the savings in memory?

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct RelativeUri {
    path: String,
    fragment_idx: Option<usize>,
    query_idx: Option<usize>,
}

impl RelativeUri {
    pub fn parse(value: &str) -> Result<Self, RelativeUriError> {
        let hash_idx = value.find('#');
        let query_idx = value.find('?');
        if hash_idx < query_idx {
            return Err(RelativeUriError::Malformed(value.to_string()));
        }
        let path = value.to_string();
        Ok(Self {
            path,
            fragment_idx: hash_idx,
            query_idx,
        })
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.path
    }
    #[must_use]
    pub fn path(&self) -> &str {
        if let Some(query_idx) = self.query_idx {
            return &self.path[..query_idx];
        }
        if let Some(hash_idx) = self.fragment_idx {
            return &self.path[..hash_idx];
        }
        &self.path
    }
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        let fragment_idx = self.fragment_idx?;
        if fragment_idx + 1 == self.path.len() {
            return Some("");
        }

        Some(&self.path[fragment_idx + 1..])
    }

    /// Returns the query string segment of the `PartialUri`, if it exists.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        let query_idx = self.query_idx?;
        if query_idx + 1 == self.path.len() {
            return Some("");
        }
        let last = self.fragment_idx.unwrap_or(self.path.len());
        Some(&self.path[query_idx + 1..last])
    }
    pub fn set_query(&mut self, mut query: Option<&str>) -> Option<String> {
        query = query.map(|q| q.strip_prefix('?').unwrap_or(q));

        // TODO: refactor this; cloning, by means of owned_parts, is not ideal.
        let (path, prev_query, fragment) = self.owned_parts();
        self.path = path;
        self.fragment_idx = None;
        self.query_idx = None;
        if let Some(query) = query {
            self.query_idx = Some(self.path.len());
            self.path += "?";
            self.path += query;
        }
        if fragment.is_some() {
            self.set_fragment(fragment.as_deref());
        }
        prev_query
    }
    /// Sets the path of the `PartialUri` and returns the previous path.
    ///
    /// Note, fragments are left intact. Use `set_fragment` to change the fragment.
    pub fn set_path(&mut self, path: &str) -> String {
        let (prev_path, query, fragment) = self.owned_parts();
        self.path = utf8_percent_encode(path, URL_PATH).to_string();
        if let Some(query) = query {
            self.query_idx = Some(self.path.len());
            self.path += "?";
            self.path += &query;
        }
        if let Some(fragment) = fragment {
            self.fragment_idx = Some(self.path.len());
            self.path += "#";
            self.path += &fragment;
        }

        prev_path
    }
    fn owned_parts(&self) -> (String, Option<String>, Option<String>) {
        let query = self.query().map(ToString::to_string);
        let fragment = self.fragment().map(ToString::to_string);
        let path = self.path().to_string();
        (path, query, fragment)
    }
    /// Sets the fragment of the `PartialUri` and returns the previous fragment, if
    /// present.
    pub fn set_fragment(&mut self, fragment: Option<&str>) -> Option<String> {
        let fragment = fragment
            .map(|f| f.strip_prefix('#').unwrap_or(f))
            .map(|f| utf8_percent_encode(f, URL_FRAGMENT).to_string());

        // TODO: refactor this; cloning, by means of owned_parts, is not ideal.
        let (path, query, prev_fragment) = self.owned_parts();
        self.fragment_idx = None;
        self.query_idx = None;
        self.path = path;

        if let Some(query) = query {
            self.query_idx = Some(self.path.len());
            self.path = format!("{}?{}", self.path, query);
        }

        if let Some(fragment) = fragment {
            self.fragment_idx = Some(self.path.len());
            self.path = format!("{}#{}", self.path, fragment);
        }

        prev_fragment
    }
}

impl Display for RelativeUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path.fmt(f)
    }
}

impl FromStr for RelativeUri {
    type Err = RelativeUriError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<String> for RelativeUri {
    type Error = RelativeUriError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl TryFrom<&str> for RelativeUri {
    type Error = RelativeUriError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
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
        self.path.as_str()
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
        if value.starts_with("urn:") {
            Ok(Urn::from_str(value)?.into())
        } else if matches_url(value) {
            Ok(Url::parse(value)?.into())
        } else {
            Ok(RelativeUri::parse(value)?.into())
        }
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
    pub fn set_fragment(&mut self, mut fragment: Option<&str>) -> Option<String> {
        if let Some(frag) = &fragment {
            if frag.is_empty() {
                fragment = None;
            }
        }
        match self {
            Uri::Url(url) => set_url_fragment(url, fragment),
            Uri::Urn(urn) => set_urn_fragment(urn, fragment),
            Uri::Relative(rel) => rel.set_fragment(fragment),
        }
    }

    /// Sets the query component of the [`Url`] or [`Urn`] and returns the
    /// previous query, if it existed.
    pub fn set_query(&mut self, query: Option<&str>) -> Result<Option<String>, UrnError> {
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
            Uri::Relative(rel) => Ok(rel.set_query(query)),
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
    /// the host for a [`Url`].
    #[must_use]
    pub fn authority_or_namespace(&self) -> Option<Cow<'_, str>> {
        match self {
            Uri::Url(url) => {
                let host = url.host()?;
                let mut result: Cow<'_, str> = Cow::Owned(host.to_string());
                if let Some(port) = url.port() {
                    result = Cow::Owned(format!("{result}:{port}"));
                }
                let mut authority = url.username().to_string();
                if !authority.is_empty() {
                    if let Some(pass) = url.password() {
                        authority.push_str(&format!(":{pass}"));
                    }
                    result = Cow::Owned(format!("{authority}@{result}"));
                }
                Some(result)
            }
            Uri::Urn(urn) => Some(Cow::Borrowed(urn.nid())),
            Uri::Relative(_) => None,
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
    /// # Errors
    pub fn set_path_or_nss(&mut self, path_or_nss: &str) -> Result<String, UrnError> {
        match self {
            Self::Url(url) => Ok(set_url_path(url, path_or_nss)),
            Self::Urn(urn) => set_urn_nss(urn, path_or_nss),
            Self::Relative(rel) => Ok(rel.set_path(path_or_nss)),
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
    pub fn as_relative(&self) -> Option<&RelativeUri> {
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

fn matches_url(value: &str) -> bool {
    for (i, c) in value.chars().enumerate() {
        if i == 0 && !c.is_ascii_alphabetic() {
            return false;
        }
        if c == ':' {
            return true;
        }
        if !c.is_ascii_alphanumeric() {
            return false;
        }
    }
    false
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

    let (userinfo, host_and_port) = authority
        .split_once('@')
        .map_or((None, authority), |(a, b)| (Some(a), b));

    let (user, pass) = userinfo.map_or((None, None), |userinfo| {
        userinfo
            .split_once(':')
            .map_or((Some(userinfo), None), |(user, pwd)| {
                (Some(user), Some(pwd))
            })
    });

    let (host, port) = host_and_port
        .split_once(':')
        .map_or((host_and_port, None), |(a, b)| (a, Some(b)));
    let port = port
        .map(str::parse)
        .transpose()
        .map_err(|_| url::ParseError::InvalidPort)?;
    u.set_port(port)
        .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?;

    u.set_host(Some(host))?;

    if let Some(user) = user {
        u.set_username(user)
            .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?;
    } else {
        // ignoring the error; internally the url crate does not check for non-empty strings
        // and errors if the username cannot be set (e.g., in the case of file://).
        // https://github.com/servo/rust-url/issues/844
        _ = u.set_username("");
    }
    // same as above, ignoring error
    _ = u.set_password(pass);

    Ok(prev_authority)
}

fn set_urn_fragment(urn: &mut Urn, fragment: Option<&str>) -> Option<String> {
    let existing = urn.f_component().map(ToString::to_string);
    // safety: encode_f_component does not currently return an error.
    let fragment = fragment.map(encode_f_component).map(Result::unwrap);
    urn.set_f_component(fragment.as_deref())
        .expect("fragment should be valid after percent encoding");
    existing
}

fn set_url_fragment(url: &mut Url, fragment: Option<&str>) -> Option<String> {
    let existing = url.fragment().map(ToString::to_string);
    url.set_fragment(fragment);
    existing
}

fn set_urn_nss(urn: &mut Urn, nss: &str) -> Result<String, UrnError> {
    let existing = urn.nss().to_string();
    urn.set_nss(&encode_nss(nss)?)?;
    Ok(existing)
}

fn set_url_path(url: &mut Url, path: &str) -> String {
    let existing = url.path().to_string();
    url.set_path(path);
    existing
}

pub trait AsUri {
    fn as_uri(&self) -> Uri;
}
impl AsUri for &Uri {
    fn as_uri(&self) -> Uri {
        (*self).clone()
    }
}
impl AsUri for Uri {
    fn as_uri(&self) -> Uri {
        self.clone()
    }
}

#[inherent]
impl AsUri for AbsoluteUri {
    /// Returns a cloned [`Uri`](`crate::uri::Uri`) representation of the this
    /// `AbsoluteUri`.
    #[must_use]
    pub fn as_uri(&self) -> Uri {
        match self {
            AbsoluteUri::Url(url) => Uri::Url(url.clone()),
            AbsoluteUri::Urn(urn) => Uri::Urn(urn.clone()),
        }
    }
}

impl AsUri for &AbsoluteUri {
    /// Returns a cloned [`Uri`](`crate::uri::Uri`) representation of the this
    /// `AbsoluteUri`.
    #[must_use]
    fn as_uri(&self) -> Uri {
        match self {
            AbsoluteUri::Url(url) => Uri::Url(url.clone()),
            AbsoluteUri::Urn(urn) => Uri::Urn(urn.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_query() {
        let mut uri = RelativeUri::parse("http://example.com").unwrap();
        assert_eq!(uri.query(), None);
        assert_eq!(uri.fragment(), None);

        uri.set_query(Some("q=str"));
        assert_eq!(uri.as_str(), "http://example.com?q=str");
        assert_eq!(uri.query(), Some("q=str"));

        uri.set_fragment(Some("fragment"));
        assert_eq!(uri.as_str(), "http://example.com?q=str#fragment");
        assert_eq!(uri.fragment(), Some("fragment"));

        uri.set_query(None);
        assert_eq!(uri.query(), None);
        assert_eq!(uri.as_str(), "http://example.com#fragment");

        uri.set_query(Some("?q=str"));
        assert_eq!(uri.as_str(), "http://example.com?q=str#fragment");

        uri.set_query(Some("q=str"));
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
            absolute_uri.set_fragment(fragment);
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
        ];
        for (input, fragment, expected_fragment, expected_uri) in tests {
            let mut uri = Uri::parse(input).unwrap();
            uri.set_fragment(fragment);
            assert_eq!(expected_uri, uri.to_string());
            assert_eq!(expected_fragment, uri.fragment());
        }
    }
    #[test]
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
}
