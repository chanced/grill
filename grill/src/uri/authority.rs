use std::{borrow::Cow, fmt::Display, ops::Deref};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
/// The Authority component of a Relative URI.
pub struct Authority<'a> {
    pub(super) value: Cow<'a, str>,
    pub(super) username_index: Option<u32>,
    pub(super) password_index: Option<u32>,
    pub(super) host_index: Option<u32>,
    pub(super) port_index: Option<u32>,
    pub(super) port: Option<u16>,
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
        let start = self.username_index()?;
        let end = self
            .password_index
            .or(self.host_index)
            .or(self.port_index)
            .map_or(self.value.len(), |idx| idx as usize);

        Some(&self.value[start..end])
    }

    /// Returns the password component if it exists.
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        let start = self.password_index()?;
        let end = self
            .host_index()
            .or(self.port_index())
            .unwrap_or(self.value.len());
        Some(&self.value[start..end])
    }

    /// Returns the host component if it exists.
    #[must_use]
    pub fn host(&self) -> Option<&str> {
        let offset = usize::from(self.username_index.is_some() || self.password_index.is_some());
        let start = self.host_index()? + offset;
        let end = self.port_index().unwrap_or(self.value.len());
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
        self.port_index().map(|idx| &self.value[idx + 1..])
    }

    /// Returns the
    #[must_use]
    pub fn into_owned(&self) -> Authority<'static> {
        Authority {
            value: Cow::Owned(self.value.to_string()),
            username_index: self.username_index,
            password_index: self.password_index,
            host_index: self.host_index,
            port_index: self.port_index,
            port: self.port,
        }
    }

    /// Returns the `&str` representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    fn port_index(&self) -> Option<usize> {
        self.port_index.map(|idx| idx as usize)
    }
    fn host_index(&self) -> Option<usize> {
        self.host_index.map(|idx| idx as usize)
    }
    fn username_index(&self) -> Option<usize> {
        self.username_index.map(|idx| idx as usize)
    }
    fn password_index(&self) -> Option<usize> {
        self.password_index.map(|idx| idx as usize)
    }
}

// fn url_authority(url: &Url) -> Option<Cow<'_, str>> {
//     let mut result = String::default();
//     let host = url.host()?;
//     if !url.username().is_empty() {
//         result.write_str(url.username()).unwrap();
//         if let Some(password) = url.password() {
//             result.write_char(':').unwrap();
//             result.write_str(password).unwrap();
//         }
//         result.write_char('@').unwrap();
//     }
//     result.write_str(&host.to_string()).unwrap();
//     if let Some(port) = url.port() {
//         result.write_char(':').unwrap();
//         result.write_str(&port.to_string()).unwrap();
//     }
//     Some(result.to_string().into())
// }
