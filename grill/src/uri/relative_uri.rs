use std::{
    borrow::Cow,
    fmt::Display,
    ops::{Deref, Index},
};

use serde::Serialize;

use crate::{
    error::{RelativeUriError, UriError},
    Uri,
};

use super::{
    encode, parse, path, write, AsUriRef, Authority, Components, PathSegments, QueryParameters,
};

/// A relative URI, with or without an authority.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RelativeUri {
    pub(super) value: String,
    pub(super) username_idx: Option<u32>,
    pub(super) password_idx: Option<u32>,
    pub(super) host_idx: Option<u32>,
    pub(super) port_idx: Option<u32>,
    pub(super) port: Option<u16>,
    pub(super) path_idx: u32,
    pub(super) query_idx: Option<u32>,
    pub(super) fragment_idx: Option<u32>,
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
        Components::from_relative_uri(self)
    }

    /// Returns a new [`PathSegments`] iterator over all segments of this
    /// `RelativeUri`'s path.
    /// # Example
    /// ```rust
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file");
    /// let uri = uri.as_relative_uri().unwrap();
    /// assert_eq!(uri.path_segments().collect::<Vec<_>>(), vec!["path", "to", "file"]);
    /// ```
    #[must_use]
    pub fn path_segments(&self) -> PathSegments {
        PathSegments::from(self.path())
    }

    /// returns a new `Uri` that is the result of resolving the given
    /// reference against this `RelativeUri`.
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
            let mut uri: Uri = self.clone().into();
            uri.set_authority_or_namespace(&authority)?;
            uri.set_query(reference.query())?;
            uri.set_fragment(reference.fragment())?;
            uri.set_path_or_nss(&reference.path_normalized())?;
            return Ok(uri);
        }

        if reference.path().is_empty() {
            let mut uri: Uri = self.clone().into();
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

        let mut uri: Uri = self.clone().into();
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

    /// Returns a [`PathSegments`] iterator over the base path segments,
    /// essentially every segment except the last.
    ///
    /// # Example
    /// ```rust
    /// use grill::uri::Uri;
    /// let uri = Uri::parse("/path/to/file").unwrap();
    /// let relative_uri = uri.as_relative_uri().unwrap();
    /// let segments = relative_uri.base_path_segments().collect::<Vec<_>>();
    /// assert_eq!(&segments, &["", "path", "to"]);
    #[must_use]
    pub fn base_path_segments(&self) -> PathSegments<'_> {
        let mut segments = self.path_segments();
        segments.base_only = true;
        segments
    }

    /// Returns the base path of the `RelativeUri`.
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
        self.path().rsplit_once('/').map_or("", |(a, _)| a)
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

    /// Returns an [`Iterator`] of [`QueryParameter`] of this `RelativeUri`.
    #[must_use]
    pub fn query_parameters(&self) -> QueryParameters<'_> {
        QueryParameters::new(self.query()).unwrap()
    }

    /// Returns `true` if this `Uri` has a query component.
    #[must_use]
    pub fn has_query(&self) -> bool {
        self.query().is_some()
    }

    /// Returns `true` if this `Uri` has a fragment component.
    #[must_use]
    pub fn has_fragment(&self) -> bool {
        self.query().is_some()
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
    /// Returns the username if it exists.
    #[must_use]
    pub fn host(&self) -> Option<&str> {
        let mut start = self.host_idx()?;
        if self.has_username() || self.has_password() {
            start += 1;
        }
        let end = self.port_idx().unwrap_or_else(|| self.path_idx());
        Some(&self.value[start..end])
    }

    /// Returns the port  if it exists.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Sets the authority and returns the previous value as an [`Authority`], if it existed.
    pub fn set_authority<'a>(
        &'a mut self,
        authority: Option<&str>,
    ) -> Result<Option<Authority<'a>>, UriError> {
        let existing_authority = self.authority().map(|a| a.into_owned());
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

    /// Returns `true` if the `RelativeUri` has a host.
    ///
    /// # Example
    /// ```
    /// use grill::Uri;
    ///
    /// let uri = Uri::parse("//example.com").unwrap()
    ///     .as_relative_uri()
    ///     .unwrap();
    /// assert!(uri.has_host());
    /// ```
    #[must_use]
    pub fn has_host(&self) -> bool {
        self.host_idx.is_some()
    }

    /// Returns `true` if the `RelativeUri` has a port.
    #[must_use]
    pub fn has_port(&self) -> bool {
        self.port_idx.is_some()
    }

    pub(crate) fn authority_str(&self) -> Option<&str> {
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

    /// Returns the path normalized by removing dot segments, i.e. `'.'`, `'..'`.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let mut uri = Uri::parse("https://example.com/./foo/../bar").unwrap()
    ///     .as_relative_uri()
    ///     .unwrap();
    /// let normalized = uri.path_normalized();
    /// assert_eq!(normalized, "/bar");
    /// ```
    #[must_use]
    pub fn path_normalized(&self) -> Cow<'_, str> {
        path::normalize(self.path())
    }

    /// Normalizes the path by removing dot segments, i.e. `'.'`, `'..'`.
    ///
    /// This method mutates the `RelativeUri` in place. If that is not desired, use
    /// [`path_normalized`](RelativeUri::path_normalized) instead.
    ///
    /// # Example
    /// ```
    /// use grill::uri::Uri;
    /// let mut uri = Uri::parse("https://example.com/./foo/../bar").unwrap();
    /// uri.normalize_path();
    /// assert_eq!(uri.path(), "/bar");
    /// ```
    pub fn normalize_path(&mut self) {
        let normalized = path::normalize(self.path()).to_string();
        self.set_path(&normalized).unwrap();
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
