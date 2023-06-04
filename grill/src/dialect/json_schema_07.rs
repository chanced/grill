use std::collections::HashMap;

use once_cell::sync::Lazy;
mod const_handler;
mod enum_handler;
mod multiple_of_handler;
mod type_handler;

pub use const_handler::{ConstHandler, ConstInvalid};
pub use enum_handler::{EnumHandler, EnumInvalid};
pub use multiple_of_handler::{MultipleOfHandler, MultipleOfInvalid};
pub use type_handler::{TypeHandler, TypeInvalid};

use crate::{AbsoluteUri, Schema, Uri};
use url::Url;

use super::Dialect;

pub const SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/schema#";
pub static SCHEMA_07_URL: Lazy<Url> = Lazy::new(|| Url::parse(SCHEMA_07_URI_STR).unwrap());
pub static SCHEMA_07_URI: Lazy<Uri> = Lazy::new(|| Uri::Url(Lazy::force(&SCHEMA_07_URL).clone()));
pub static SCHEMA_07_ABSOLUTE_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::Url(Lazy::force(&SCHEMA_07_URL).clone()));

pub const HYPER_SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/hyper-schema#";
pub static HYPER_SCHEMA_07_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(HYPER_SCHEMA_07_URI_STR).unwrap());
pub static HYPER_SCHEMA_07_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&HYPER_SCHEMA_07_URL).clone()));
pub static HYPER_SCHEMA_07_ABSOLUTE_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::Url(Lazy::force(&HYPER_SCHEMA_07_URL).clone()));

pub const SCHEMA_07_BYTES: &[u8] = include_bytes!("../../../dialect/07/schema.json");
pub const HYPER_SCHEMA_07_BYTES: &[u8] = include_bytes!("../../../dialect/07/hyper-schema.json");
pub const HYPER_SCHEMA_07_LINKS_BYTES: &[u8] = include_bytes!("../../../dialect/07/links.json");
pub const HYPER_SCHEMA_07_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../dialect/07/hyper-schema-output.json");

pub static SCHEMA_07: Lazy<Schema> = Lazy::new(|| serde_json::from_slice(SCHEMA_07_BYTES).unwrap());

pub static SCHEMA_07_DIALECT: Lazy<Dialect> = Lazy::new(|| {
    let mut meta_schemas = HashMap::new();
    meta_schemas.insert(
        schema_07_absolute_uri().clone(),
        Lazy::force(&SCHEMA_07).as_object().unwrap().clone(),
    );

    let mut vocabularies = HashMap::new();

    Dialect {
        id: schema_07_absolute_uri().clone(),
        meta_schemas,
        vocabularies,
    }
});

#[must_use]
pub fn schema_07_url() -> &'static Url {
    Lazy::force(&SCHEMA_07_URL)
}
#[must_use]
pub fn schema_07_uri() -> &'static Uri {
    Lazy::force(&SCHEMA_07_URI)
}
#[must_use]
pub fn schema_07_absolute_uri() -> &'static AbsoluteUri {
    Lazy::force(&SCHEMA_07_ABSOLUTE_URI)
}

#[must_use]
pub fn hyper_schema_07_url() -> &'static Url {
    Lazy::force(&HYPER_SCHEMA_07_URL)
}
#[must_use]
pub fn hyper_schema_07_uri() -> &'static Uri {
    Lazy::force(&HYPER_SCHEMA_07_URI)
}
#[must_use]
pub fn hyper_schema_07_absolute_uri() -> &'static AbsoluteUri {
    Lazy::force(&HYPER_SCHEMA_07_ABSOLUTE_URI)
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
