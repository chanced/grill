use url::Url;

use crate::{AbsoluteUri, Uri};



use once_cell::sync::Lazy;
mod const_handler;
mod enum_handler;
mod multiple_of_handler;
mod type_handler;

pub const SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/schema#";
pub static SCHEMA_07_URL: Lazy<Url> = Lazy::new(|| Url::parse(SCHEMA_07_URI_STR).unwrap());
pub static SCHEMA_07_URI: Lazy<Uri> = Lazy::new(|| Uri::Url(Lazy::force(&SCHEMA_07_URL).clone()));
pub static SCHEMA_07_ABSOLUTE_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::Url(Lazy::force(&SCHEMA_07_URL).clone()));

pub const HYPER_SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/hyper-schema#";
pub static HYPER_SCHEMA_07_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(HYPER_SCHEMA_07_URI_STR).unwrap());

pub const SCHEMA_07_BYTES: &[u8] = include_bytes!("../../../dialect/07/schema.json");
pub const HYPER_SCHEMA_07_BYTES: &[u8] = include_bytes!("../../../dialect/07/hyper-schema.json");
pub const HYPER_SCHEMA_07_LINKS_BYTES: &[u8] = include_bytes!("../../../dialect/07/links.json");
pub const HYPER_SCHEMA_07_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../dialect/07/hyper-schema-output.json");

#[must_use]
pub fn schema_07_url() -> &'static Url {
    Lazy::force(&SCHEMA_07_URL)
}

pub fn is_hyper_schema_07_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == HYPER_SCHEMA_07_URL.domain()
                && u.path() == HYPER_SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}
pub fn is_hyper_schema_07_absolute_uri(uri: &AbsoluteUri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == HYPER_SCHEMA_07_URL.domain()
                && u.path() == HYPER_SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_schema_07_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == SCHEMA_07_URL.domain()
                && u.path() == SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_schema_07_absolute_uri(uri: &AbsoluteUri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == SCHEMA_07_URL.domain()
                && u.path() == SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub mod handler {
    pub use super::{
        const_handler::{ConstHandler, ConstInvalid},
        enum_handler::{EnumHandler, EnumInvalid},
        multiple_of_handler::{MultipleOfHandler, MultipleOfInvalid},
        type_handler::{TypeHandler, TypeInvalid},
    };
}
