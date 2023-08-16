use std::{
    borrow::Cow,
    fmt::{Display, Write},
    ops::Deref,
};

use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
/// The Authority component of a Relative URI.
pub struct Authority<'a> {
    value: Cow<'a, str>,
    username_idx: Option<u32>,
    password_idx: Option<u32>,
    host_idx: Option<u32>,
    port_idx: Option<u32>,
    port: Option<u16>,
}
impl Deref for Authority<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Display for Authority<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<Authority<'_>> for String {
    fn from(value: Authority) -> Self {
        value.to_string()
    }
}

impl From<Authority<'_>> for Cow<'_, str> {
    fn from(value: Authority) -> Self {
        Cow::Owned(value.to_string())
    }
}

impl<'a> Authority<'a> {
    /// Returns the username component if it exists.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        let start = self.username_idx()?;
        let end = self
            .password_idx
            .or(self.host_idx)
            .or(self.port_idx)
            .map_or(self.value.len(), |idx| idx as usize);

        Some(&self.value[start..end])
    }

    /// Returns the password component if it exists.
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        let start = self.password_idx()?;
        let end = self
            .host_idx()
            .or(self.port_idx())
            .unwrap_or(self.value.len());
        Some(&self.value[start..end])
    }

    /// Returns the host component if it exists.
    #[must_use]
    pub fn host(&self) -> Option<&str> {
        let offset = usize::from(self.username_idx.is_some() || self.password_idx.is_some());
        let start = self.host_idx()? + offset;
        let end = self.port_idx().unwrap_or(self.value.len());
        Some(&self.value[start..end])
    }

    /// Returns the port component if it exists.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Returns the port as an `&str` if it exists.
    #[must_use]
    pub fn port_str(&self) -> Option<&str> {
        self.port_idx().map(|idx| &self.value[idx + 1..])
    }

    /// Returns the
    #[must_use]
    pub fn into_owned(&self) -> Authority<'static> {
        Authority {
            value: Cow::Owned(self.value.to_string()),
            username_idx: self.username_idx,
            password_idx: self.password_idx,
            host_idx: self.host_idx,
            port_idx: self.port_idx,
            port: self.port,
        }
    }

    /// Returns the `&str` representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    fn port_idx(&self) -> Option<usize> {
        self.port_idx.map(|idx| idx as usize)
    }
    fn host_idx(&self) -> Option<usize> {
        self.host_idx.map(|idx| idx as usize)
    }
    fn username_idx(&self) -> Option<usize> {
        self.username_idx.map(|idx| idx as usize)
    }
    fn password_idx(&self) -> Option<usize> {
        self.password_idx.map(|idx| idx as usize)
    }
}

fn url_authority(url: &Url) -> Option<Cow<'_, str>> {
    let mut result = String::default();
    let host = url.host()?;
    if !url.username().is_empty() {
        result.write_str(url.username()).unwrap();
        if let Some(password) = url.password() {
            result.write_char(':').unwrap();
            result.write_str(password).unwrap();
        }
        result.write_char('@').unwrap();
    }
    result.write_str(&host.to_string()).unwrap();
    if let Some(port) = url.port() {
        result.write_char(':').unwrap();
        result.write_str(&port.to_string()).unwrap();
    }
    Some(result.to_string().into())
}
