use std::str::FromStr;

use crate::{error::UriError, AbsoluteUri, Uri};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
    Primary(Uri),
    Secondary(Uri),
}

impl Identifier {
    /// Returns `true` if the identifier is [`Primary`].
    ///
    /// [`Primary`]: Identifier::Primary
    #[must_use]
    pub fn is_primary(&self) -> bool {
        matches!(self, Self::Primary(..))
    }

    /// Returns `true` if the identifier is [`Secondary`].
    ///
    /// [`Secondary`]: Identifier::Secondary
    #[must_use]
    pub fn is_secondary(&self) -> bool {
        matches!(self, Self::Secondary(..))
    }

    #[must_use]
    pub fn uri(&self) -> &Uri {
        match self {
            Self::Secondary(uri) | Self::Primary(uri) => uri,
        }
    }

    #[must_use]
    pub fn take_uri(self) -> Uri {
        match self {
            Self::Secondary(uri) | Self::Primary(uri) => uri,
        }
    }
}

impl PartialEq<Uri> for Identifier {
    fn eq(&self, other: &Uri) -> bool {
        match self {
            Self::Primary(uri) | Self::Secondary(uri) => uri == other,
        }
    }
}
impl PartialEq<AbsoluteUri> for Identifier {
    fn eq(&self, other: &AbsoluteUri) -> bool {
        match self {
            Self::Primary(uri) | Self::Secondary(uri) => uri == other,
        }
    }
}
impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::Primary(uri) | Self::Secondary(uri) => uri == other,
        }
    }
}
