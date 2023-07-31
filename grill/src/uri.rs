//! Data structures to represent Uniform Resource Identifiers (URI) [RFC 3986](https://tools.ietf.org/html/rfc3986).

#[cfg(test)]
mod tests;

mod parse;
mod encode;
mod write;

use crate::error::{OverflowError, UriError, UrnError, RelativeUriError};
use inherent::inherent;
use percent_encoding::percent_decode;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    iter::Peekable,
    ops::{Deref, Index},
    path::PathBuf,
    str::{FromStr, Split},
    string::ToString,
};
use urn::percent::{encode_f_component, encode_nss};

pub use url::Url;
pub use urn::Urn;


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
            Self::Uri(uri) => uri.as_str(),
            Self::AbsoluteUri(uri) => uri.as_str(),
            Self::RelativeUri(uri) => uri.as_str(),
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
            UriRef::Uri(uri) => uri.as_relative(),
            UriRef::RelativeUri(rel) => Some(*rel),
            UriRef::AbsoluteUri(_) => None,
        }
    }
    /// Returns `true` if this underlying `Uri` is a [`Url`]
    #[must_use]
    pub fn is_url(&self) -> bool {
        match self {
            UriRef::Uri(_) => todo!(),
            UriRef::AbsoluteUri(_) => todo!(),
            UriRef::RelativeUri(_) => todo!(),
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

    /// returns a new `AbsoluteUri` that is the result of resolving the given
    /// reference against this `AbsoluteUri`.
    ///
    /// See [RFC3986, Section
    /// 5.2.2](https://tools.ietf.org/html/rfc3986#section-5.2.2).
    #[must_use]
    pub fn resolve(&self, reference: &impl AsUriRef) -> AbsoluteUri {
        let reference = reference.as_uri_ref();
        if reference.scheme().is_some() {
            match reference {
                UriRef::Uri(uri) => {
                    let mut result = uri.clone();
                    result
                        .set_path_or_nss(&normalize_path(uri.path_or_nss()))
                        .unwrap();
                    return result.try_into_absolute_uri().unwrap();
                }
                UriRef::AbsoluteUri(uri) => {
                    let mut result = uri.clone();
                    result
                        .set_path_or_nss(&normalize_path(uri.path_or_nss()))
                        .unwrap();
                    return result;
                }
                UriRef::RelativeUri(_) => unreachable!(),
            }
        }

        todo!()
        /*
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
}

/// Normalizes a URI path by removing dot segments, i.e. `'.'`, `'..'`.
#[must_use]
pub fn normalize_path(path: &str) -> String {
    let mut buf = PathBuf::new();

    for segment in Segments::from_path(path) {
        match segment {
            Segment::PathParent => {
                buf.pop();
            }
            Segment::PathNormal(c) => buf.push(c),
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

pub trait TryIntoAbsoluteUri {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError>;
}

impl<'a> TryIntoAbsoluteUri for UriRef<'a> {
    fn try_into_absolute_uri(self) -> Result<AbsoluteUri, UriError> {
        match self {
            UriRef::Uri(uri) => uri.try_into_absolute_uri(),
            UriRef::AbsoluteUri(uri) => Ok(uri.clone()),
            UriRef::RelativeUri(uri) => uri.try_into_absolute_uri(),
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
    has_authority: bool,
    path_idx: u32,
    query_idx: Option<u32>,
    fragment_idx: Option<u32>,
}

impl RelativeUri {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
    #[must_use]
    pub fn path(&self) -> &str {
        if let Some(query_idx) = self.query_idx() {
            return &self.value[self.path_idx()..query_idx];
        }
        if let Some(hash_idx) = self.fragment_idx() {
            return &self.value[self.path_idx()..hash_idx];
        }
        &self.value
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
        self.path().len() > 0
    }
    /// Sets the query string portion of the `RelativeUri` and returns the
    /// previous query, if it existed.
    /// 
    /// # Panics
    /// Panics if setting the query string would cause the `RelativeUri` to
    /// exceed [`u32::MAX`](core::u32::MAX) in length.
    pub fn set_query(&mut self, mut query: Option<&str>) -> Result<Option<String>, RelativeUriError> {
        let existing_query = self.query().map(ToString::to_string);
        let mut buf = String::with_capacity(self.len() - existing_query.unwrap_or_default().len() + query.unwrap_or_default().len());
        let has_authority = write::authority(self.authority(), &mut buf);
        let path_idx:u32 = write::path(self.path(), &mut buf).expect("RelativeUri::set_query: path length exceeds u32::MAX");
        let query_idx:Option<u32> = write::query(query.as_deref(), &mut buf, self.has_authority, self.has_path())?;
        let fragment_idx:Option<u32> = write::fragment(self.fragment(), &mut buf).expect("RelativeUri::set_query: fragment length exceeds u32::MAX");
        self.value = buf;
        self.has_authority = has_authority;
        self.path_idx = path_idx;
        self.query_idx = query_idx;
        self.fragment_idx = fragment_idx;
    }
    /// Sets the path of the `RelativeUri` and returns the previous path.
    ///
    /// Note, fragments are left intact. Use `set_fragment` to change the fragment.
    pub fn set_path(&mut self, path: &str) -> String {
        Url::set_fragment(&mut self, fragment)
        let (prev_path, query, fragment) = self.owned_parts();
        self.value = utf8_percent_encode(path, PATH).to_string();
        if let Some(query) = query {
            self.query_idx = Some(self.value.len() as u32);
            self.value += "?";
            self.value += &query;
        }
        if let Some(fragment) = fragment {
            self.fragment_idx = Some(self.value.len() as u32);
            self.value += "#";
            self.value += &fragment;
        }

        prev_path
    }
    fn owned_parts(
        &self,
    ) -> (
        Option<String>, /* authority */
        String,         /* path */
        Option<String>, /* query */
        Option<String>, /* fragment */
    ) {
        let authority = self.authority().map(ToString::to_string);
        let query = self.query().map(ToString::to_string);
        let fragment = self.fragment().map(ToString::to_string);
        let path = self.path().to_string();
        (authority, path, query, fragment)
    }
    /// Sets the fragment of the `RelativeUri` and returns the previous fragment, if
    /// present.
    pub fn set_fragment(&mut self, fragment: Option<&str>) -> Option<String> {
        let fragment = fragment
            .map(|f| f.strip_prefix('#').unwrap_or(f))
            .map(|f| utf8_percent_encode(f, FRAGMENT).to_string());

        // TODO: refactor this; cloning, by means of owned_parts, is not ideal.
        let (authority, path, query, prev_fragment) = self.owned_parts();
        self.fragment_idx = None;
        self.query_idx = None;
        self.value = path;

        if let Some(query) = query {
            self.query_idx = Some(self.value.len() as u32);
            self.value = format!("{}?{}", self.value, query);
        }

        if let Some(fragment) = fragment {
            self.fragment_idx = Some(self.value.len() as u32);
            self.value = format!("{}#{}", self.value, fragment);
        }

        prev_fragment
    }

    #[must_use]
    /// Returns the authority of the `RelativeUri` if it exists.
    ///
    /// A relative URI may have an authority if it starts starts with `"//"`.
    pub fn authority(&self) -> Option<&str> {
        if self.has_authority {
            Some(&self.value[2..self.path_idx()])
        } else {
            None
        }
    }

    // /// Sets the authority of the `RelativeUri` and returns the previous authority, if it existed.
    // pub fn set_authority(&self, authority: Option<&str>) -> Option<String> {

    // }
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
        todo!()
        // if value.starts_with("urn:") {
        //     Ok(Urn::from_str(value)?.into())
        // } else if matches_url(value) {
        //     Ok(Url::parse(value)?.into())
        // } else {
        //     Ok(RelativeUri::parse(value)?.into())
        // }
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

pub trait ToUri {
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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Segment<'a> {
    /// The scheme of the URI, i.e., `scheme:`.
    Scheme(&'a str),

    /// The authority of the URI, i.e., `//authority`.
    Authority(&'a str),

    /// The root of the path
    PathRoot,

    /// A reference to the current path segment, i.e., `.`.
    PathCurrent,

    /// A reference to the parent path segment, i.e., `..`.
    PathParent,

    /// A normal path segment, e.g., `a` and `b` in `a/b`.
    PathNormal(&'a str),

    /// The query of the URI, i.e., `?query`.
    Query(&'a str),

    /// The fragment of the URI, i.e., `#fragment`.
    Fragment(&'a str),
}

impl<'a> Segment<'a> {
    pub fn decode(&self) -> Result<Cow<'a, str>, std::str::Utf8Error> {
        match self {
            Segment::Scheme(s) | Segment::Authority(s) => Ok(Cow::Borrowed(*s)),
            Segment::PathRoot => Ok(Cow::Borrowed("")),
            Segment::PathCurrent => Ok(Cow::Borrowed(".")),
            Segment::PathParent => Ok(Cow::Borrowed("..")),
            Segment::PathNormal(val) | Segment::Query(val) | Segment::Fragment(val) => {
                percent_decode(val.as_bytes()).decode_utf8()
            }
        }
    }

    pub fn decode_lossy(&self) -> Cow<'a, str> {
        match self {
            Segment::Scheme(s) | Segment::Authority(s) => Cow::Borrowed(*s),
            Segment::PathRoot => Cow::Borrowed(""),
            Segment::PathCurrent => Cow::Borrowed("."),
            Segment::PathParent => Cow::Borrowed(".."),
            Segment::PathNormal(val) | Segment::Query(val) | Segment::Fragment(val) => {
                percent_decode(val.as_bytes()).decode_utf8_lossy()
            }
        }
    }
    fn parse_root(val: &'a str, next: Option<char>) -> Self {
        match val {
            "" => Self::PathRoot,
            "." | ".." => Self::resolve_dots(val, next),
            _ => Self::PathNormal(val),
        }
    }
    fn parse_path_segment(val: &'a str, next: Option<char>) -> Self {
        match val {
            "." | ".." => Self::resolve_dots(val, next),
            _ => Self::PathNormal(val),
        }
    }
    fn resolve_dots(val: &'a str, next: Option<char>) -> Self {
        if next == Some('/') || next.is_none() {
            if val == "." {
                Self::PathCurrent
            } else {
                Self::PathParent
            }
        } else {
            Self::PathNormal(val)
        }
    }
}

impl<'a> Deref for Segment<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Segment::Scheme(v)
            | Segment::Authority(v)
            | Segment::PathNormal(v)
            | Segment::Query(v)
            | Segment::Fragment(v) => v,
            Segment::PathRoot => "",
            Segment::PathCurrent => ".",
            Segment::PathParent => "..",
        }
    }
}

/// An [`Iterator`] of path [`Segment`]s.
pub struct Segments<'a> {
    scheme: Option<&'a str>,
    authority: Option<&'a str>,
    path: Peekable<Split<'a, char>>,
    query: Option<&'a str>,
    fragment: Option<&'a str>,
    root_sent: bool,
}

impl<'a> Segments<'a> {
    fn peek_next_char(&mut self) -> Option<char> {
        self.path.peek().and_then(|s| s.chars().next())
    }
}

impl<'a> Iterator for Segments<'a> {
    type Item = Segment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(scheme) = self.scheme.take() {
            return Some(Segment::Scheme(scheme));
        }
        if let Some(authority) = self.authority.take() {
            return Some(Segment::Authority(authority));
        }
        let Some(val) = self.path.next() else {
            // if let Some(query) = self.query.take() {
            //     return Some(Segment::Query(query));
            // }
            self.query.take().map(Segment::Query)?;
            if let Some(fragment) = self.fragment.take() {
                return Some(Segment::Fragment(fragment));
            }
            return None;
        };
        if self.root_sent {
            Some(Segment::parse_path_segment(val, self.peek_next_char()))
        } else {
            self.root_sent = true;
            Some(Segment::parse_root(val, self.peek_next_char()))
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum PathSegment<'a> {
    /// The root of the path
    Root,

    /// A reference to the current path segment, i.e., `.`.
    Current,

    /// A reference to the parent path segment, i.e., `..`.
    Parent,

    /// A normal path segment, e.g., `a` and `b` in `a/b`.
    Normal(&'a str),
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
    pub fn decode(&self) -> Result<Cow<'a, str>, std::str::Utf8Error> {
        match self {
            PathSegment::Root => Ok(Cow::Borrowed("")),
            PathSegment::Current => Ok(Cow::Borrowed(".")),
            PathSegment::Parent => Ok(Cow::Borrowed("..")),
            PathSegment::Normal(val) => {
                percent_decode(val.as_bytes()).decode_utf8()
            }
        }
    }

    pub fn decode_lossy(&self) -> Cow<'a, str> {
        match self {
            PathSegment::Root => Cow::Borrowed(""),
            PathSegment::Current => Cow::Borrowed("."),
            PathSegment::Parent => Cow::Borrowed(".."),
            PathSegment::Normal(val) => {
                percent_decode(val.as_bytes()).decode_utf8_lossy()
            }
        }
    }
    fn parse_root(val: &'a str, next: Option<char>) -> Self {
        match val {
            "" => Self::Root,
            "." | ".." => Self::resolve_dots(val, next),
            _ => Self::Normal(val),
        }
    }
    fn parse_path_segment(val: &'a str, next: Option<char>) -> Self {
        match val {
            "." | ".." => Self::resolve_dots(val, next),
            _ => Self::Normal(val),
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
            Self::Normal(val)
        }
    }
}


/// An [`Iterator`] of path [`PathSegment`]s.
pub struct PathSegments<'a> {
    path: Peekable<Split<'a, char>>,
    root_sent: bool,
}

impl<'a> PathSegments<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            path: path.split('/').peekable(),
            root_sent: false,
        }
    }
    fn peek_next_char(&mut self) -> Option<char> {
        self.path.peek().and_then(|s| s.chars().next())
    }
    
}
impl<'a> From<&'a str> for PathSegments<'a> {
    fn from(path: &'a str) -> Self {
        Self {
            path: path.split('/').peekable(),
            root_sent: false,
        }
    }
}
impl<'a> FromStr for PathSegments<'a> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}


impl<'a> Iterator for PathSegments<'a> {
    type Item = PathSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.path.next()?;
        if self.root_sent {
            Some(PathSegment::parse_path_segment(val, self.peek_next_char()))
        } else {
            self.root_sent = true;
            Some(PathSegment::parse_root(val, self.peek_next_char()))
        }
    }
}



#[inline]
fn to_u32(v: usize) -> Result<u32, OverflowError> {
    if v > u32::MAX as usize {
        Err(OverflowError)
    } else {
        Ok(v as u32)
    }
}
