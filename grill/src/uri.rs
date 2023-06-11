use std::{
    borrow::Cow, convert::Infallible, fmt::Display, ops::Deref, str::FromStr, string::ToString,
};

use serde::{Deserialize, Serialize};
use url::Url;
use urn::Urn;

use crate::error::{AbsoluteUriParseError, UriParseError, UrnError};

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
    /// Returns [`AbsoluteUriParseError`] if `value` can not be parsed as a [`Uri`](`uri::Uri`) or [`Urn`](`urn::Urn`)
    pub fn parse(value: &str) -> Result<Self, AbsoluteUriParseError> {
        if value.starts_with("urn:") {
            Ok(Self::Urn(Urn::from_str(value)?))
        } else {
            Ok(Self::Url(Url::parse(value)?))
        }
    }
    /// Returns the fragment if it exists.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        match self {
            Self::Url(uri) => uri.fragment(),
            Self::Urn(urn) => urn.f_component(),
        }
    }
    /// Sets the fragment component of the [`Url`] or [`Urn`] and returns the
    /// previous value, if it exists.
    ///
    /// # Errors
    /// Returns [`urn::Error`](`urn::Error`) if the `AbsoluteUri` is a
    /// [`Urn`](`urn::Urn`) and the fragment and the fragment fails validation.
    pub fn set_fragment(&mut self, fragment: Option<&str>) -> Result<Option<String>, UrnError> {
        match self {
            Self::Url(uri) => {
                let existing = uri.fragment().map(ToString::to_string);
                uri.set_fragment(fragment);
                Ok(existing)
            }
            Self::Urn(urn) => {
                let existing = urn.f_component().map(ToString::to_string);
                urn.set_f_component(fragment)?;
                Ok(existing)
            }
        }
    }

    /// Returns the namespace if the absolute uri is [`Urn`], otherwise returns
    /// the host for a [`Url`].
    #[must_use]
    pub fn authority_or_namespace(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::Url(url) => {
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
            Self::Urn(urn) => Some(Cow::Borrowed(urn.nid())),
        }
    }

    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            Self::Url(url) => url.path(),
            Self::Urn(urn) => urn.nss(),
        }
    }

    pub fn set_path_or_nss(&mut self, path_or_nss: &str) -> Result<String, UrnError> {
        match self {
            Self::Url(url) => {
                let path = url.path().to_string();
                url.set_path(path_or_nss);
                Ok(path)
            }
            Self::Urn(urn) => {
                let nss = urn.nss().to_string();
                urn.set_nss(path_or_nss)?;
                Ok(nss)
            }
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Url(url) => url.as_str(),
            Self::Urn(urn) => urn.as_str(),
        }
    }

    /// Returns `true` if the absolute uri is [`Url`].
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

    /// Returns `true` if the absolute uri is [`Urn`].
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
    type Error = AbsoluteUriParseError;

    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        match value {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url)),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn)),
            Uri::Partial(p) => Self::parse(p.as_str()),
        }
    }
}

impl TryFrom<&Uri> for AbsoluteUri {
    type Error = AbsoluteUriParseError;

    fn try_from(value: &Uri) -> Result<Self, Self::Error> {
        match value {
            Uri::Url(url) => Ok(AbsoluteUri::Url(url.clone())),
            Uri::Urn(urn) => Ok(AbsoluteUri::Urn(urn.clone())),
            Uri::Partial(p) => Self::parse(p.as_str()),
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
    type Error = AbsoluteUriParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<&String> for AbsoluteUri {
    type Error = AbsoluteUriParseError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for AbsoluteUri {
    type Error = AbsoluteUriParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl FromStr for AbsoluteUri {
    type Err = AbsoluteUriParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub struct PartialUri {
    path: String,
    hash_idx: Option<usize>,
}

impl PartialUri {
    pub fn parse(value: &str) -> Self {
        let hash_idx = value.find('#');
        let path = value.to_string();
        Self { path, hash_idx }
    }
    pub fn as_str(&self) -> &str {
        &self.path
    }
    pub fn path(&self) -> &str {
        let Some(hash_idx) = self.hash_idx else { return &self.path };
        &self.path[..hash_idx]
    }
    pub fn fragment(&self) -> Option<&str> {
        let hash_idx = self.hash_idx?;
        if hash_idx + 1 == self.path.len() {
            Some("")
        } else {
            Some(&self.path[hash_idx + 1..])
        }
    }

    /// Sets the path of the `PartialUri` and returns the previous path.
    ///
    /// Note, fragments are left intact. Use `set_fragment` to change the fragment.
    pub fn set_path(&mut self, path: &str) -> String {
        let (prev_path, prev_frag) = self.hash_idx.map_or_else(
            || (self.path.clone(), None),
            |idx| {
                (
                    self.path[..idx].to_string(),
                    Some(self.path[idx + 1..].to_string()),
                )
            },
        );

        self.path = path.to_string();
        if let Some(fragment) = prev_frag {
            self.hash_idx = Some(path.len());
            self.path += "#";
            self.path += &fragment;
        }

        prev_path
    }

    /// Sets the fragment of the `PartialUri` and returns the previous fragment, if
    /// present.
    pub fn set_fragment(&mut self, fragment: Option<&str>) -> Option<String> {
        if let Some(hash_idx) = self.hash_idx {
            let previous = if hash_idx + 1 == self.path.len() {
                ""
            } else {
                &self.path[hash_idx + 1..]
            };
            let previous = previous.to_string();
            self.hash_idx = None;
            if hash_idx == 0 {
                self.path = String::new();
            } else {
                self.path.truncate(hash_idx);
            }
            if let Some(fragment) = fragment {
                self.hash_idx = Some(hash_idx);
                self.path += "#";
                self.path += fragment;
            } else {
                return Some(previous);
            };
            Some(previous)
        } else {
            let Some(fragment) = fragment else { return None };
            self.hash_idx = Some(self.path.len() - 1);
            self.path = format!("#{fragment}");
            None
        }
    }
}

impl Display for PartialUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path.fmt(f)
    }
}

impl From<String> for PartialUri {
    fn from(value: String) -> Self {
        Self {
            path: value,
            hash_idx: None,
        }
    }
}

impl FromStr for PartialUri {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl From<&String> for PartialUri {
    fn from(value: &String) -> Self {
        Self {
            path: value.clone(),
            hash_idx: None,
        }
    }
}
impl From<PartialUri> for Uri {
    fn from(value: PartialUri) -> Self {
        Uri::Partial(value)
    }
}

impl From<PartialUri> for String {
    fn from(value: PartialUri) -> Self {
        value.to_string()
    }
}

impl From<&PartialUri> for String {
    fn from(value: &PartialUri) -> Self {
        value.to_string()
    }
}

impl Deref for PartialUri {
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
    Partial(PartialUri),
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Uri::Url(url) => Display::fmt(url, f),
            Uri::Urn(urn) => Display::fmt(urn, f),
            Uri::Partial(par) => Display::fmt(par, f),
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
    pub fn parse(value: &str) -> Result<Self, UriParseError> {
        if value.starts_with("urn:") {
            Ok(Urn::from_str(value)?.into())
        } else if matches_url(value) {
            Ok(Url::parse(value)?.into())
        } else {
            Ok(PartialUri::parse(value).into())
        }
    }
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        match self {
            Uri::Url(url) => url.fragment(),
            Uri::Urn(urn) => urn.f_component(),
            Uri::Partial(par) => par.fragment(),
        }
    }

    /// Sets the fragment component of the [`Url`] or [`Urn`] and returns the
    /// previous value, if it exists.
    ///
    /// # Errors
    /// Returns [`urn::Error`](`urn::Error`) if the `AbsoluteUri` is a
    /// [`Urn`](`urn::Urn`) and the fragment and the fragment fails validation.
    pub fn set_fragment(&mut self, fragment: Option<&str>) -> Result<Option<String>, UrnError> {
        match self {
            Uri::Url(uri) => {
                let existing = uri.fragment().map(ToString::to_string);
                uri.set_fragment(fragment);
                Ok(existing)
            }
            Uri::Urn(urn) => {
                let existing = urn.f_component().map(ToString::to_string);
                urn.set_f_component(fragment)?;
                Ok(existing)
            }
            Uri::Partial(par) => Ok(par.set_fragment(fragment)),
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
            Uri::Partial(_) => None,
        }
    }

    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            Self::Url(url) => url.path(),
            Self::Urn(urn) => urn.nss(),
            Self::Partial(par) => par.path(),
        }
    }
    /// # Errors
    pub fn set_path_or_nss(&mut self, path_or_nss: &str) -> Result<String, UrnError> {
        match self {
            Self::Url(url) => {
                let path = url.path().to_string();
                url.set_path(path_or_nss);
                Ok(path)
            }
            Self::Urn(urn) => {
                let nss = urn.nss().to_string();
                let fragment = urn.f_component().map(ToString::to_string);
                urn.set_nss(path_or_nss)?;
                urn.set_f_component(fragment.as_deref())?;
                Ok(nss)
            }
            Self::Partial(par) => {
                let path = par.path().to_string();
                par.set_path(path_or_nss);
                Ok(path)
            }
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Uri::Url(url) => url.as_str(),
            Uri::Urn(urn) => urn.as_str(),
            Uri::Partial(par) => par.as_str(),
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
    type Err = UriParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<String> for Uri {
    type Error = UriParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}
impl TryFrom<&String> for Uri {
    type Error = UriParseError;
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
            Uri::Partial(par) => par.as_str(),
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
#[cfg(test)]
mod tests {
    use super::*;

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
            ("https://www.example.com/", None, "https://www.example.com/"),
            (
                "https://username:password@example.com/path#fraggle-rock",
                Some("fraggle-rock/back-to-rock"),
                "https://username:password@example.com/path#fraggle-rock/back-to-rock",
            ),
            (
                "https://example.com/path#with-fragment",
                None,
                "https://example.com/path",
            ),
            (
                "urn:example:resource",
                Some("fragment"),
                "urn:example:resource#fragment",
            ),
        ];
        for (input, fragment, expected) in tests {
            let mut absolute_uri = AbsoluteUri::parse(input).unwrap();
            absolute_uri.set_fragment(fragment).unwrap();
            assert_eq!(expected, absolute_uri.to_string());
            assert_eq!(fragment, absolute_uri.fragment());
        }

        let tests = [
            ("https://www.example.com/", None, "https://www.example.com/"),
            (
                "https://username:password@example.com/path#fraggle-rock",
                Some("fraggle-rock/back-to-rock"),
                "https://username:password@example.com/path#fraggle-rock/back-to-rock",
            ),
            (
                "https://example.com/path#with-fragment",
                None,
                "https://example.com/path",
            ),
            (
                "urn:example:resource",
                Some("fragment"),
                "urn:example:resource#fragment",
            ),
            (
                "/partial/path#existing-fragment",
                Some("new-fragment"),
                "/partial/path#new-fragment",
            ),
            ("#existing-fragment", Some("new-fragment"), "#new-fragment"),
            ("#existing-fragment", None, ""),
            ("/partial/path#existing-fragment", None, "/partial/path"),
        ];
        for (input, fragment, expected) in tests {
            let mut uri = Uri::parse(input).unwrap();
            uri.set_fragment(fragment).unwrap();
            assert_eq!(expected, uri.to_string());
            assert_eq!(fragment, uri.fragment());
        }
    }
    #[test]
    fn test_set_path() {
        let tests = [
            (
                "https://www.example.com",
                "/new-path",
                "https://www.example.com/new-path",
            ),
            (
                "https://username:password@example.com/path#fraggle-rock",
                "/new-path",
                "https://username:password@example.com/new-path#fraggle-rock",
            ),
            (
                "https://example.com/path#with-fragment",
                "",
                "https://example.com/#with-fragment",
            ),
            (
                "urn:example:resource#fragment",
                "new-resource",
                "urn:example:new-resource#fragment",
            ),
            (
                "urn:example:resource",
                "new-resource",
                "urn:example:new-resource",
            ),
        ];
        for (input, mut new_path, expected) in tests {
            let mut absolute_uri = AbsoluteUri::parse(input).unwrap();
            absolute_uri.set_path_or_nss(new_path).unwrap();
            assert_eq!(expected, absolute_uri.to_string());
            if new_path.is_empty() {
                new_path = "/";
            }
            assert_eq!(new_path, absolute_uri.path_or_nss());
        }

        let tests = [
            (
                "https://www.example.com",
                "/new-path",
                "https://www.example.com/new-path",
            ),
            (
                "https://username:password@example.com/path#fraggle-rock",
                "/new-path",
                "https://username:password@example.com/new-path#fraggle-rock",
            ),
            (
                "https://example.com/path#with-fragment",
                "",
                "https://example.com/#with-fragment",
            ),
            (
                "urn:example:resource#fragment",
                "new-resource",
                "urn:example:new-resource#fragment",
            ),
            (
                "urn:example:resource",
                "new-resource",
                "urn:example:new-resource",
            ),
            ("", "/new-path", "/new-path"),
            ("/", "/resource", "/resource"),
            ("/path#fragment", "/new-path", "/new-path#fragment"),
        ];
        for (input, mut new_path, expected) in tests {
            let mut uri = Uri::parse(input).unwrap();
            uri.set_path_or_nss(new_path).unwrap();
            assert_eq!(expected, uri.to_string());
            if new_path.is_empty() {
                new_path = "/";
            }
            assert_eq!(new_path, uri.path_or_nss());
        }
    }
}
