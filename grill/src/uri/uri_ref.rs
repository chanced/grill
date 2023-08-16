use super::{AbsoluteUri, QueryParameters, RelativeUri, Uri, Url, Urn};
use inherent::inherent;
use std::{borrow::Cow, ops::Deref};

/// A borrowed [`Uri`], [`AbsoluteUri`], or [`RelativeUri`].
#[derive(Clone, Copy, Debug)]
pub enum UriRef<'a> {
    /// A borrowed [`Uri`].
    Uri(&'a Uri),
    /// A borrowed [`AbsoluteUri`].
    AbsoluteUri(&'a AbsoluteUri),
    /// A borrowed [`RelativeUri`].
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
    pub fn as_relative_uri(&self) -> Option<&'a RelativeUri> {
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

    /// Returns `true` if the URI has an authority or namespace.
    #[must_use]
    pub fn has_authority(&self) -> bool {
        match self {
            UriRef::RelativeUri(rel) => rel.has_authority(),
            _ => true,
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

    /// Returns the path (if this `UriRef` is a [`Url`]) or namespace specific
    /// string (if this `Uri` is a [`Urn`])
    #[must_use]
    pub fn path_or_nss(&self) -> &str {
        match self {
            UriRef::Uri(uri) => uri.path_or_nss(),
            UriRef::AbsoluteUri(uri) => uri.path_or_nss(),
            UriRef::RelativeUri(uri) => uri.path(),
        }
    }
    /// Returns the base path of the `UriRef`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap().as_uri_ref();
    ///
    /// assert_eq!(uri.base_path(), "/path/to");
    /// ```
    #[must_use]
    pub fn base_path(&self) -> &str {
        self.path_or_nss().rsplit_once('/').map_or("", |(a, _)| a)
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
    /// Returns the query component if it exists.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        match self {
            UriRef::Uri(uri) => uri.query(),
            UriRef::AbsoluteUri(uri) => uri.query(),
            UriRef::RelativeUri(uri) => uri.query(),
        }
    }
    /// Returns an [`Iterator`] of [`QueryParameter`] of this `UriRef`.
    #[must_use]
    pub fn query_parameters(&self) -> QueryParameters<'_> {
        QueryParameters::new(self.query()).unwrap()
    }

    /// Returns the path [`normalized`](super::path::normalize) by removing dot segments, i.e. `'.'`,
    /// `'..'`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("https://example.com/./foo/../bar").unwrap();
    /// let uri_ref = uri.as_uri_ref();
    /// let normalized = uri_ref.path_normalized();
    /// assert_eq!(normalized, "/bar");
    /// ```
    /// [`normalize`]:super::path::normalize
    #[must_use]
    pub fn path_normalized(&self) -> Cow<'_, str> {
        super::path::normalize(self.path_or_nss())
    }
}

impl Deref for UriRef<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

/// A trait which enables borrowing a [`Uri`], [`AbsoluteUri`], or
/// [`RelativeUri`] as a singular type.
pub trait AsUriRef {
    /// Borrows `self` as a [`UriRef`].
    fn as_uri_ref(&self) -> UriRef<'_>;
}

#[inherent]
impl AsUriRef for AbsoluteUri {
    #[must_use]
    pub fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::AbsoluteUri(self)
    }
}

impl AsUriRef for &AbsoluteUri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::AbsoluteUri(self)
    }
}

#[inherent]
impl AsUriRef for Uri {
    #[must_use]
    pub fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::Uri(self)
    }
}

impl AsUriRef for &Uri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::Uri(*self)
    }
}
impl AsUriRef for &RelativeUri {
    fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::RelativeUri(*self)
    }
}

#[inherent]
impl AsUriRef for RelativeUri {
    #[must_use]
    pub fn as_uri_ref(&self) -> UriRef<'_> {
        UriRef::RelativeUri(self)
    }
}
