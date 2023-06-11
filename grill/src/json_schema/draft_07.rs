mod const_handler;
mod enum_handler;
mod multiple_of_handler;
mod type_handler;

pub use const_handler::{ConstHandler, ConstInvalid};
pub use enum_handler::{EnumHandler, EnumInvalid};
pub use multiple_of_handler::{MultipleOfHandler, MultipleOfInvalid};
pub use type_handler::{TypeHandler, TypeInvalid};

use crate::{
    dialect::{Dialect, Vocabulary},
    json_schema::identify,
    uri::AbsoluteUri,
    Metaschema, Uri,
};
use once_cell::sync::Lazy;
use serde_json::Value;
use url::Url;

pub const JSON_SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/schema#";
pub static JSON_SCHEMA_07_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_SCHEMA_07_URI_STR).unwrap());
pub static JSON_SCHEMA_07_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_SCHEMA_07_URL).clone()));
pub static JSON_SCHEMA_07_ABSOLUTE_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::Url(Lazy::force(&JSON_SCHEMA_07_URL).clone()));

pub const JSON_HYPER_SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/hyper-schema#";
pub static JSON_HYPER_SCHEMA_07_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_HYPER_SCHEMA_07_URI_STR).unwrap());
pub static JSON_HYPER_SCHEMA_07_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_HYPER_SCHEMA_07_URL).clone()));
pub static JSON_HYPER_SCHEMA_07_ABSOLUTE_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_HYPER_SCHEMA_07_URL).clone()));

pub const JSON_SCHEMA_07_BYTES: &[u8] = include_bytes!("../../../json_schema/07/schema.json");
pub const JSON_HYPER_SCHEMA_07_BYTES: &[u8] =
    include_bytes!("../../../json_schema/07/hyper-schema.json");
pub const JSON_HYPER_SCHEMA_07_LINKS_BYTES: &[u8] =
    include_bytes!("../../../json_schema/07/links.json");
pub const JSON_HYPER_SCHEMA_07_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/07/hyper-schema-output.json");

pub static JSON_SCHEMA_07: Lazy<Value> =
    Lazy::new(|| serde_json::from_slice(JSON_SCHEMA_07_BYTES).unwrap());

pub static JSON_SCHEMA_07_DIALECT: Lazy<Dialect> = Lazy::new(|| {
    Dialect::new(
        json_schema_07_absolute_uri().clone(),
        &[Metaschema {
            id: JSON_SCHEMA_07_ABSOLUTE_URI.clone(),
            schema: JSON_SCHEMA_07.as_object().unwrap().clone(),
        }],
        &[Vocabulary::new(
            json_schema_07_absolute_uri(),
            [ConstHandler::new()],
        )],
        is_json_schema_07,
        identify,
    )
});

#[must_use]
pub fn is_json_schema_07(v: &Value) -> bool {
    let Value::Object(obj) = v else { return false };
    let Some(s) = obj.get("$schema").and_then(Value::as_str) else { return false };
    if s == JSON_SCHEMA_07_URI_STR {
        return true;
    }

    let Ok(uri) = Uri::parse(s) else { return false };
    is_json_schema_07_uri(&uri)
}

#[must_use]
pub fn json_schema_07_url() -> &'static Url {
    Lazy::force(&JSON_SCHEMA_07_URL)
}
#[must_use]
pub fn json_schema_07_uri() -> &'static Uri {
    Lazy::force(&JSON_SCHEMA_07_URI)
}
#[must_use]
pub fn json_schema_07_absolute_uri() -> &'static AbsoluteUri {
    Lazy::force(&JSON_SCHEMA_07_ABSOLUTE_URI)
}

#[must_use]
pub fn json_hyper_schema_07_url() -> &'static Url {
    Lazy::force(&JSON_HYPER_SCHEMA_07_URL)
}
#[must_use]
pub fn json_hyper_schema_07_uri() -> &'static Uri {
    Lazy::force(&JSON_HYPER_SCHEMA_07_URI)
}
#[must_use]
pub fn json_hyper_schema_07_absolute_uri() -> &'static Uri {
    Lazy::force(&JSON_HYPER_SCHEMA_07_ABSOLUTE_URI)
}

#[must_use]
pub fn dialect() -> &'static Dialect {
    Lazy::force(&JSON_SCHEMA_07_DIALECT)
}

pub fn is_json_hyper_schema_07_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_07_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}
pub fn is_json_hyper_schema_07_absolute_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_07_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_07_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_07_URL.domain()
                && u.path() == JSON_SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_07_absolute_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_07_URL.domain()
                && u.path() == JSON_SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_json_schema_07_filter() {
        let schema = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": "https://example.com"
        });

        assert!(is_json_schema_07(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft-07/schema",
            "$id": "https://example.com"
        });
        assert!(is_json_schema_07(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft-2020-12/schema",
            "$id": "https://example.com"
        });
        assert!(!is_json_schema_07(&schema));
    }
}
