use crate::{dialect::Dialect, json_schema::identify, uri::AbsoluteUri, Metaschema, Uri};
use once_cell::sync::Lazy;
use serde_json::Value;
use url::Url;

use super::anchors;

pub const JSON_SCHEMA_04_URI_STR: &str = "http://json-schema.org/draft-04/schema#";
pub static JSON_SCHEMA_04_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_SCHEMA_04_URI_STR).unwrap());
pub static JSON_SCHEMA_04_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_SCHEMA_04_URL).clone()));
pub static JSON_SCHEMA_04_ABSOLUTE_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::Url(Lazy::force(&JSON_SCHEMA_04_URL).clone()));

pub const JSON_HYPER_SCHEMA_04_URI_STR: &str = "http://json-schema.org/draft-04/hyper-schema#";
pub static JSON_HYPER_SCHEMA_04_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_HYPER_SCHEMA_04_URI_STR).unwrap());
pub static JSON_HYPER_SCHEMA_04_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_HYPER_SCHEMA_04_URL).clone()));
pub static JSON_HYPER_SCHEMA_04_ABSOLUTE_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_HYPER_SCHEMA_04_URL).clone()));

pub const JSON_SCHEMA_04_BYTES: &[u8] = include_bytes!("../../../json_schema/04/schema.json");
pub const JSON_HYPER_SCHEMA_04_BYTES: &[u8] =
    include_bytes!("../../../json_schema/04/hyper-schema.json");
pub const JSON_HYPER_SCHEMA_04_LINKS_BYTES: &[u8] =
    include_bytes!("../../../json_schema/04/links.json");

pub static JSON_SCHEMA_04: Lazy<Value> =
    Lazy::new(|| serde_json::from_slice(JSON_SCHEMA_04_BYTES).unwrap());

pub static JSON_SCHEMA_04_DIALECT: Lazy<Dialect> = Lazy::new(|| {
    Dialect::new(
        json_schema_04_absolute_uri().clone(),
        [Metaschema {
            id: JSON_SCHEMA_04_ABSOLUTE_URI.clone(),
            schema: JSON_SCHEMA_04.as_object().unwrap().clone(),
        }],
        [super::draft_07::ConstHandler::new()], // TODO: FIX,
        is_json_schema_04,
        identify,
        anchors,
    )
});

#[must_use]
pub fn is_json_schema_04(v: &Value) -> bool {
    let Value::Object(obj) = v else { return false };
    let Some(s) = obj.get("$schema").and_then(Value::as_str) else { return false };
    if s == JSON_SCHEMA_04_URI_STR {
        return true;
    }

    let Ok(uri) = Uri::parse(s) else { return false };
    is_json_schema_04_uri(&uri)
}

#[must_use]
pub fn json_schema_04_url() -> &'static Url {
    Lazy::force(&JSON_SCHEMA_04_URL)
}
#[must_use]
pub fn json_schema_04_uri() -> &'static Uri {
    Lazy::force(&JSON_SCHEMA_04_URI)
}
#[must_use]
pub fn json_schema_04_absolute_uri() -> &'static AbsoluteUri {
    Lazy::force(&JSON_SCHEMA_04_ABSOLUTE_URI)
}

#[must_use]
pub fn json_hyper_schema_04_url() -> &'static Url {
    Lazy::force(&JSON_HYPER_SCHEMA_04_URL)
}
#[must_use]
pub fn json_hyper_schema_04_uri() -> &'static Uri {
    Lazy::force(&JSON_HYPER_SCHEMA_04_URI)
}
#[must_use]
pub fn json_hyper_schema_04_absolute_uri() -> &'static Uri {
    Lazy::force(&JSON_HYPER_SCHEMA_04_ABSOLUTE_URI)
}

#[must_use]
pub fn dialect() -> &'static Dialect {
    Lazy::force(&JSON_SCHEMA_04_DIALECT)
}

pub fn is_json_hyper_schema_04_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_04_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_04_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}
pub fn is_json_hyper_schema_04_absolute_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_04_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_04_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_04_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_04_URL.domain()
                && u.path() == JSON_SCHEMA_04_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_04_absolute_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_04_URL.domain()
                && u.path() == JSON_SCHEMA_04_URL.path()
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
    fn test_json_schema_04_filter() {
        let schema = json!({
            "$schema": "http://json-schema.org/draft-04/schema#",
            "$id": "https://example.com"
        });

        assert!(is_json_schema_04(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft-04/schema",
            "$id": "https://example.com"
        });
        assert!(is_json_schema_04(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft-2020-12/schema",
            "$id": "https://example.com"
        });
        assert!(!is_json_schema_04(&schema));
    }
}
