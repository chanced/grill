use std::{fmt::Display, ops::Deref, str::FromStr};

use fancy_regex::Regex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use url::Url;
use urn::Urn;

static URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z][[a-zA-Z]\d+\.-]*:").unwrap());

use crate::error::{AbsoluteUriParseError, UriParseError};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum AbsoluteUri {
    Url(Url),
    Urn(Urn),
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
impl From<&AbsoluteUri> for &str {
    fn from(value: &AbsoluteUri) -> Self {
        value.as_str()
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

impl AbsoluteUri {
    /// Attempts to parse an `AbsoluteUri`.
    ///
    /// # Errors
    /// Returns [`AbsoluteUriParseError`] if `value` can not be parsed as a [`Uri`](`uri::Uri`) or [`Urn`](`urn::Urn`)
    pub fn parse(value: &str) -> Result<Self, AbsoluteUriParseError> {
        if value.starts_with("urn:") {
            Ok(AbsoluteUri::Urn(Urn::from_str(value)?))
        } else {
            Ok(AbsoluteUri::Url(Url::parse(value)?))
        }
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            AbsoluteUri::Url(url) => url.as_str(),
            AbsoluteUri::Urn(urn) => urn.as_str(),
        }
    }
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        match self {
            AbsoluteUri::Url(uri) => uri.fragment(),
            AbsoluteUri::Urn(urn) => urn.f_component(),
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
            AbsoluteUri::Url(url) => url.as_str(),
            AbsoluteUri::Urn(urn) => urn.as_str(),
        }
    }
}
impl Display for AbsoluteUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbsoluteUri::Url(url) => url.fmt(f),
            AbsoluteUri::Urn(urn) => urn.fmt(f),
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
#[serde(try_from = "String", into = "String")]
pub enum Uri {
    Url(Url),
    Urn(Urn),
    Partial {
        path: String,
        hash_idx: Option<usize>,
    },
}

impl PartialEq<Uri> for AbsoluteUri {
    fn eq(&self, other: &Uri) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Uri {
    pub fn fragment(&self) -> Option<&str> {
        match self {
            Uri::Url(url) => url.fragment(),
            Uri::Urn(urn) => urn.f_component(),
            Uri::Partial { path, hash_idx } => hash_idx.and_then(|i| {
                path.chars()
                    .nth(i)
                    .filter(|c| *c == '#')
                    .and_then(|_| {
                        if i == path.len() - 1 {
                            return Some("");
                        } else if i < path.len() {
                            return path.get(i + 1..);
                        }
                        None
                    })
                    .or_else(|| path.split_once('#').map(|(_, frag)| frag))
            }),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Uri::Url(url) => url.as_str(),
            Uri::Urn(urn) => urn.as_str(),
            Uri::Partial { path, .. } => path.as_str(),
        }
    }
    pub fn parse(value: &str) -> Result<Self, UriParseError> {
        if value.starts_with("urn:") {
            Ok(Uri::Urn(Urn::from_str(value)?))
        } else if url_regex().is_match(value)? {
            Ok(Uri::Url(Url::parse(value)?))
        } else {
            let hash_idx = value.find('#');

            Ok(Uri::Partial {
                path: value.to_string(),
                hash_idx,
            })
        }
    }

    /// Returns `true` if the uri is [`Url`].
    ///
    /// [`Url`]: Uri::Url
    #[must_use]
    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url(..))
    }

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

    pub fn as_urn(&self) -> Option<&Urn> {
        if let Self::Urn(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_url(self) -> Result<Url, Self> {
        if let Self::Url(v) = self {
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
        Self::parse(&value)
    }
}

impl Deref for Uri {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Uri::Url(url) => url.as_str(),
            Uri::Urn(urn) => urn.as_str(),
            Uri::Partial { path, .. } => path.as_str(),
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
impl From<&Uri> for &str {
    fn from(value: &Uri) -> Self {
        value.as_str()
    }
}

fn url_regex() -> &'static Regex {
    Lazy::force(&URL_REGEX)
}
