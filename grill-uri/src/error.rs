use core::fmt;

#[doc(no_inline)]
pub use url::ParseError as UrlError;
#[doc(no_inline)]
pub use urn::Error as UrnError;

use crate::Uri;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Error                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Errors which can occur when parsing or interacting with
/// [`Uri`](`crate::uri::Uri`), [`AbsoluteUri`](`crate::uri::AbsoluteUri`), or
/// [`RelativeUri`](`crate::uri::RelativeUri`).
#[derive(Debug)]
pub enum Error {
    /// an issue occurred parsing a [`Url`](`url::Url`)
    FailedToParseUrl(UrlError),

    /// an issue occurred parsing a [`Urn`](`urn::Urn`)
    FailedToParseUrn(UrnError),

    /// an issue occurred parsing a [`RelativeUri`](`crate::uri::RelativeUri`)
    FailedToParseRelativeUri(RelativeUriError),

    /// The [`Uri`] is not absolute and cannot be made into an [`AbsoluteUri`].
    NotAbsolute(NotAbsoluteError),

    /// An issue occurred while setting the Authority of a
    /// [`Uri`] or [`RelativeUri`](crate::uri::RelativeUri).
    MalformedAuthority(AuthorityError),

    /// The scheme of a [`Uri`] or [`AbsoluteUri`] is malformed.
    InvalidScheme(InvalidSchemeError),

    /// The Uri exceeds the maximum size of 4GB
    Overflow(OverflowError),
}

impl Error {
    /// Returns a new `Result<T, Self>::Err(Self)` with the given URI and document.
    #[allow(clippy::missing_errors_doc)]
    pub fn err_with<F, T, X, E>(f: F) -> Result<T, E>
    where
        F: FnOnce() -> X,
        X: Into<Self>,
        E: From<Self>,
    {
        Err(f().into().into())
    }
}
impl From<InvalidPortError> for Error {
    fn from(source: InvalidPortError) -> Self {
        AuthorityError::InvalidPort { source }.into()
    }
}
impl From<UrlError> for Error {
    fn from(err: UrlError) -> Self {
        Self::FailedToParseUrl(err)
    }
}

impl From<OverflowError> for Error {
    fn from(err: OverflowError) -> Self {
        Self::Overflow(err)
    }
}

impl From<UrnError> for Error {
    fn from(err: UrnError) -> Self {
        Self::FailedToParseUrn(err)
    }
}
impl From<RelativeUriError> for Error {
    fn from(err: RelativeUriError) -> Self {
        Self::FailedToParseRelativeUri(err)
    }
}
impl From<NotAbsoluteError> for Error {
    fn from(err: NotAbsoluteError) -> Self {
        Self::NotAbsolute(err)
    }
}
impl From<AuthorityError> for Error {
    fn from(err: AuthorityError) -> Self {
        Self::MalformedAuthority(err)
    }
}
impl From<InvalidSchemeError> for Error {
    fn from(err: InvalidSchemeError) -> Self {
        Self::InvalidScheme(err)
    }
}
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FailedToParseUrl(e) => write!(f, "failed to parse url: {e}"),
            Error::FailedToParseUrn(e) => write!(f, "failed to parse urn: {e}"),
            Error::FailedToParseRelativeUri(e) => write!(f, "failed to parse relative uri: {e}"),
            Error::NotAbsolute(e) => fmt::Display::fmt(e, f),
            Error::MalformedAuthority(e) => fmt::Display::fmt(e, f),
            Error::InvalidScheme(scheme) => fmt::Display::fmt(scheme, f),
            Error::Overflow(_) => write!(f, "uri length exceeds maximum size of 4GB"),
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                              InvalidSchemeError                              ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug, PartialEq)]
pub struct InvalidSchemeError {
    scheme: String,
}
impl InvalidSchemeError {
    pub fn new(scheme: String) -> Self {
        Self { scheme }
    }
    pub fn scheme(&self) -> &str {
        &self.scheme
    }
}
impl fmt::Display for InvalidSchemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid scheme: {}", self.scheme)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                               NotAbsoluteerror                               ║
║                              ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug, PartialEq)]
pub struct NotAbsoluteError {
    uri: Uri,
}
impl NotAbsoluteError {
    pub fn new(uri: Uri) -> Self {
        Self { uri }
    }
    pub fn uri(&self) -> &Uri {
        &self.uri
    }
}

impl fmt::Display for NotAbsoluteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "uri is not absolute: {}", self.uri)
    }
}

impl std::error::Error for NotAbsoluteError {}

impl Error {}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                AuthorityError                                ║
║                               ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                               ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Returned from `set_authority` on [`Uri`], [`AbsoluteUri`], and [`RelativeUri`]
#[derive(Debug, PartialEq, Eq)]
pub enum AuthorityError {
    /// The authority contains a path
    ContainsPath { value: String },

    /// The authority contains a query
    ContainsQuery { value: String },

    /// The authority contains a fragment
    ContainsFragment { value: String },

    /// The authority contains a malformed port
    InvalidPort { source: InvalidPortError },

    /// An error occurred while setting the `authority` of a [`Urn`](urn::Urn)
    Urn { source: UrnError },

    /// The username cannot be set due to the scheme of the Uri (e.g. `file`)
    UsernameNotAllowed { scheme: String, value: String },

    /// The password cannot be set due to the scheme of the Uri (e.g. `file`)
    PasswordNotAllowed { value: String, scheme: String },

    /// The host cannot be set due to the scheme of the Uri (e.g. `file`)
    PortNotAllowed { port: u16, scheme: String },
}

impl From<InvalidPortError> for AuthorityError {
    fn from(source: InvalidPortError) -> Self {
        AuthorityError::InvalidPort { source }
    }
}

impl std::error::Error for AuthorityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AuthorityError::InvalidPort { source } => Some(source),
            AuthorityError::Urn { source } => Some(source),
            _ => None,
        }
    }
}

impl fmt::Display for AuthorityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthorityError::ContainsPath { value } => {
                write!(
                    f,
                    "authority is malformed due to containing a path segment: \"{value}\""
                )
            }
            AuthorityError::ContainsQuery { value } => {
                write!(f, "authority contains query: {value}")
            }
            AuthorityError::ContainsFragment { value } => {
                write!(f, "authority contains fragment: {value}")
            }
            AuthorityError::InvalidPort { source } => {
                write!(f, "authority contains a malformed port: {source}")
            }
            AuthorityError::Urn { source } => write!(f, "urn error: {source}"),
            AuthorityError::UsernameNotAllowed { scheme, .. } => {
                write!(f, "username cannot be set due to scheme: \"{scheme}\"")
            }
            AuthorityError::PasswordNotAllowed { scheme, .. } => {
                write!(f, "password cannot be set due to scheme: \"{scheme}\"")
            }
            AuthorityError::PortNotAllowed { scheme, .. } => {
                write!(f, "port cannot be set due to scheme: \"{scheme}\"")
            }
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                               RelativeUriError                               ║
║                              ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Errors which can occur when parsing or modifying a
/// [`RelativeUri`](crate::uri::RelativeUri).
#[derive(Debug, PartialEq, Eq)]
pub enum RelativeUriError {
    /// The length of the input exceeds `u32::MAX`
    Overflow(OverflowError),
    /// The decoded string is not valid UTF-8
    Utf8Encoding(std::str::Utf8Error),
    /// The port of a [`RelativeUri`] exceeded the maximum value of 65535.
    InvalidPort(InvalidPortError),
}
impl fmt::Display for RelativeUriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow(_) => {
                write!(f, "uri length exceeds maximum size of 4GB")
            }
            Self::Utf8Encoding(e) => write!(f, "uri is not valid utf-8: {e}"),

            Self::InvalidPort(e) => fmt::Display::fmt(e, f),
        }
    }
}
impl From<OverflowError> for RelativeUriError {
    fn from(err: OverflowError) -> Self {
        Self::Overflow(err)
    }
}
/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                OverflowError                                 ║
║                               ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug, PartialEq, Eq)]
pub struct OverflowError {
    pub len: u64,
}

impl fmt::Display for OverflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "len exceeds maximum size of 4GB: {}", self.len)
    }
}

impl std::error::Error for OverflowError {}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                               InvalidPortError                               ║
║                              ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A port of a [`RelativeUri`] exceeded the maximum value of `u16`.
#[derive(Debug, PartialEq, Eq)]
pub struct InvalidPortError {
    pub value: String,
}

impl fmt::Display for InvalidPortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "port is malformed or exceeds maximum value of 65535: {}",
            self.value
        )
    }
}

impl std::error::Error for InvalidPortError {}
