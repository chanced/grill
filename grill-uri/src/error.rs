use snafu::{Backtrace, Snafu};
#[doc(no_inline)]
pub use url::ParseError as UrlError;
#[doc(no_inline)]
pub use urn::Error as UrnError;

use crate::Uri;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               UriError                                ║
║                               ¯¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Errors which can occur when parsing or interacting with
/// [`Uri`](`crate::uri::Uri`), [`AbsoluteUri`](`crate::uri::AbsoluteUri`), or
/// [`RelativeUri`](`crate::uri::RelativeUri`).
#[derive(Debug, Snafu)]
pub enum UriError {
    /// an issue occurred parsing a [`Url`](`url::Url`)
    #[snafu(display("failed to parse url: {source}"))]
    FailedToParseUrl {
        source: UrlError,
        backtrace: Backtrace,
    },

    /// an issue occurred parsing a [`Urn`](`urn::Urn`)
    #[snafu(display("failed to parse urn: {source}"))]
    FailedToParseUrn {
        source: urn::Error,
        backtrace: Backtrace,
    },

    /// an issue occurred parsing a [`RelativeUri`](`crate::uri::RelativeUri`)
    #[snafu(transparent)]
    FailedToParseRelativeUri {
        #[snafu(backtrace)]
        source: RelativeUriError,
    },

    /// The [`Uri`] is not absolute and cannot be made into an [`AbsoluteUri`].
    #[snafu(display("uri is not absolute: {uri}"))]
    NotAbsolute { uri: Uri, backtrace: Backtrace },

    /// An issue occurred while setting the Authority of a
    /// [`Uri`] or [`RelativeUri`](crate::uri::RelativeUri).
    #[snafu(transparent)]
    MalformedAuthority {
        #[snafu(backtrace)]
        source: AuthorityError,
    },

    /// The scheme of a [`Uri`] or [`AbsoluteUri`] is malformed.
    #[snafu(display("invalid uri scheme: {scheme}"))]
    InvalidScheme {
        /// The scheme which was found to be invalid.
        scheme: String,
        backtrace: Backtrace,
    },
}

impl From<InvalidPortError> for UriError {
    fn from(err: InvalidPortError) -> Self {
        Self::FailedToParseRelativeUri { source: err.into() }
    }
}
impl From<OverflowError> for UriError {
    fn from(err: OverflowError) -> Self {
        Self::FailedToParseRelativeUri { source: err.into() }
    }
}

impl UriError {
    /// Returns `true` if the uri parse error is [`Url`].
    ///
    /// [`Url`]: UriParseError::Url
    #[must_use]
    pub fn is_url(&self) -> bool {
        matches!(self, Self::FailedToParseUrl { .. })
    }

    /// Returns `true` if the uri parse error is [`Urn`].
    ///
    /// [`Urn`]: UriParseError::Urn
    #[must_use]
    pub fn is_urn(&self) -> bool {
        matches!(self, Self::FailedToParseUrn { .. })
    }

    /// Returns `true` if the uri error is [`Relative`].
    ///
    /// [`Relative`]: UriError::Relative
    #[must_use]
    pub fn is_relative(&self) -> bool {
        matches!(self, Self::FailedToParseRelativeUri { .. })
    }

    /// Returns `true` if the uri error is [`NotAbsolute`].
    ///
    /// [`NotAbsolute`]: UriError::NotAbsolute
    #[must_use]
    pub fn is_not_absolute(&self) -> bool {
        matches!(self, Self::NotAbsolute { .. })
    }

    /// If the error is [`UriError::Url`], returns a reference to the underlying
    /// [`UrlError`].
    #[must_use]
    pub fn as_url(&self) -> Option<&UrlError> {
        if let Self::FailedToParseUrl { source, backtrace } = self {
            Some(source)
        } else {
            None
        }
    }

    /// If the error is [`UriError::Urn`], returns a reference to the underlying
    /// [`UrnError`].
    #[must_use]
    pub fn as_urn(&self) -> Option<&urn::Error> {
        if let Self::FailedToParseUrn {
            source,
            backtrace: _,
        } = self
        {
            Some(source)
        } else {
            None
        }
    }

    #[must_use]
    /// If the error is [`UriError::Relative`], returns a reference to the underlying
    /// [`RelativeUriError`].
    pub fn as_relative(&self) -> Option<&RelativeUriError> {
        if let Self::FailedToParseRelativeUri { source } = self {
            Some(source)
        } else {
            None
        }
    }

    #[must_use]
    /// If the error is [`UriError::NotAbsolute`], returns a reference to the underlying
    /// [`UriNotAbsoluteError`].
    pub fn as_not_absolute(&self) -> Option<&Uri> {
        if let Self::NotAbsolute { uri, backtrace: _ } = self {
            Some(uri)
        } else {
            None
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                            AuthorityError                             ║
║                            ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Returned from `set_authority` on [`Uri`], [`AbsoluteUri`], and [`RelativeUri`]
#[derive(Debug, Snafu)]
#[snafu(context(suffix(Ctx)), module)]
pub enum AuthorityError {
    /// The authority contains a path
    #[snafu(display("authority contains path: {value}"))]
    ContainsPath { value: String, backtrace: Backtrace },

    /// The authority contains a query
    #[snafu(display("authority contains query: {value}"))]
    ContainsQuery { value: String, backtrace: Backtrace },

    /// The authority contains a fragment
    #[snafu(display("authority contains path: {value}"))]
    ContainsFragment { value: String, backtrace: Backtrace },

    /// The authority contains a malformed port
    #[snafu(transparent)]
    InvalidPort {
        #[snafu(backtrace)]
        source: InvalidPortError,
    },

    #[snafu(transparent)]
    /// An error occurred while setting the `authority` of a [`Urn`](urn::Urn)
    Urn {
        source: UrnError,
        backtrace: Backtrace,
    },
    /// The username cannot be set due to the scheme of the Uri (e.g. `file`)
    #[snafu(display("username cannot be set due to scheme: {scheme}"))]
    UsernameNotAllowed {
        scheme: String,
        value: String,
        backtrace: Backtrace,
    },
    #[snafu(display("password cannot be set due to scheme: {scheme}"))]
    /// The password cannot be set due to the scheme of the Uri (e.g. `file`)
    PasswordNotAllowed {
        value: String,
        scheme: String,
        backtrace: Backtrace,
    },
    /// The host cannot be set due to the scheme of the Uri (e.g. `file`)
    #[snafu(display("host cannot be set due to scheme: {scheme}"))]
    PortNotAllowed { port: u16, scheme: String },
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           InvalidPortError                            ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A port of a [`RelativeUri`] exceeded the maximum value of `u16`.
#[derive(Debug, Snafu)]
#[snafu(
    display("port \"{value}\" is malformed or exceeds maximum value of 65535"),
    context(suffix(Ctx)),
    module
)]
pub struct InvalidPortError {
    pub value: String,
    pub backtrace: Backtrace,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           RelativeUriError                            ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Errors which can occur when parsing or modifying a
/// [`RelativeUri`](crate::uri::RelativeUri).
#[derive(Debug, Snafu)]
#[snafu(context(suffix(Ctx)), module)]
pub enum RelativeUriError {
    /// The length of the input exceeds `u32::MAX`
    Overflow {
        #[snafu(backtrace)]
        source: OverflowError,
    },
    /// The decoded string is not valid UTF-8
    #[snafu(display("uri is not valid utf-8: {source}"))]
    Utf8Encoding {
        source: std::str::Utf8Error,
        backtrace: Backtrace,
    },
    /// The port of a [`RelativeUri`] exceeded the maximum value of 65535.
    #[snafu(transparent)]
    InvalidPort {
        #[snafu(backtrace)]
        source: InvalidPortError,
    },
}
impl From<OverflowError> for RelativeUriError {
    fn from(err: OverflowError) -> Self {
        Self::Overflow { source: err }
    }
}

#[derive(Debug, snafu::Snafu)]
#[snafu(
    display("length of uri exceeds maximum size of 4GB"),
    visibility(pub),
    context(suffix(Ctx)),
    module
)]
/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             OverflowError                             ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[snafu()]
pub struct OverflowError {
    pub value: u64,
    pub backtrace: Backtrace,
}
impl OverflowError {
    pub const MAX: u64 = u32::MAX as u64;
}
