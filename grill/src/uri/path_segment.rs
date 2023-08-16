use std::{borrow::Cow, iter::Peekable, str::Split};

use percent_encoding::percent_decode;

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
            "." | ".." => Self::remove_dots(val, next),
            _ => Self::Normal(val.into()),
        }
    }
    fn parse_path_segment(val: &'a str, next: Option<char>) -> Self {
        match val {
            "." | ".." => Self::remove_dots(val, next),
            _ => Self::Normal(val.into()),
        }
    }
    fn remove_dots(val: &'a str, next: Option<char>) -> Self {
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

/// An [`Iterator`] of [`PathSegment`]s.
#[derive(Debug, Default)]
pub struct PathSegments<'a> {
    path: Option<Peekable<Split<'a, char>>>,
    pub(crate) base_only: bool,
    root_sent: bool,
}

impl<'a> PathSegments<'a> {
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
            let next_char = next.and_then(|s| s.chars().next());
            if base_only && next.is_none() {
                return None;
            }
            return Some(PathSegment::parse_path_segment(value, next_char));
        }
        self.root_sent = true;
        let next = self.peek();
        let next_char = next.and_then(|s| s.chars().next());
        Some(PathSegment::parse_root(value, next_char))
    }
}
