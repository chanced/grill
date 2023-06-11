use crate::{
    dialect::{Dialect, Vocabulary},
    json_schema::identify,
    uri::AbsoluteUri,
    Metaschema, Uri,
};
use once_cell::sync::Lazy;
use serde_json::Value;
use url::Url;

pub const JSON_SCHEMA_2019_09_URI_STR: &str = "https://json-schema.org/draft/2019-09/schema";
pub static JSON_SCHEMA_2019_09_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_SCHEMA_2019_09_URI_STR).unwrap());
pub static JSON_SCHEMA_2019_09_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_SCHEMA_2019_09_URL).clone()));
pub static JSON_SCHEMA_2019_09_ABSOLUTE_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::Url(Lazy::force(&JSON_SCHEMA_2019_09_URL).clone()));

pub const JSON_HYPER_SCHEMA_2019_09_URI_STR: &str =
    "https://json-schema.org/draft/2019-09/hyper-schema";
pub static JSON_HYPER_SCHEMA_2019_09_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_HYPER_SCHEMA_2019_09_URI_STR).unwrap());
pub static JSON_HYPER_SCHEMA_2019_09_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_HYPER_SCHEMA_2019_09_URL).clone()));
pub static JSON_HYPER_SCHEMA_2019_09_ABSOLUTE_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_HYPER_SCHEMA_2019_09_URL).clone()));

pub const JSON_SCHEMA_2019_09_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/schema.json");
pub const JSON_HYPER_SCHEMA_2019_09_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/hyper-schema.json");
pub const JSON_HYPER_SCHEMA_2019_09_LINKS_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/links.json");
pub const JSON_HYPER_SCHEMA_2019_09_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/output/hyper-schema.json");
pub const JSON_SCHEMA_2019_09_APPLICATOR_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/applicator.json");
pub const JSON_SCHEMA_2019_09_CONTENT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/content.json");
pub const JSON_SCHEMA_2019_09_CORE_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/core.json");
pub const JSON_SCHEMA_2019_09_FORMAT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/format.json");
pub const JSON_SCHEMA_2019_09_META_DATA_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/meta-data.json");
pub const JSON_SCHEMA_2019_09_VALIDATION_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/validation.json");

pub static JSON_SCHEMA_2019_09: Lazy<Value> =
    Lazy::new(|| serde_json::from_slice(JSON_SCHEMA_2019_09_BYTES).unwrap());

pub static JSON_SCHEMA_2019_09_DIALECT: Lazy<Dialect> = Lazy::new(|| {
    Dialect::new(
        json_schema_2019_09_absolute_uri().clone(),
        &[Metaschema {
            id: JSON_SCHEMA_2019_09_ABSOLUTE_URI.clone(),
            schema: JSON_SCHEMA_2019_09.as_object().unwrap().clone(),
        }],
        &[Vocabulary::new(
            json_schema_2019_09_absolute_uri(),
            [super::draft_07::ConstHandler::new()], // TOOD: FIX
        )],
        is_json_schema_2019_09,
        identify,
    )
});

#[must_use]
pub fn is_json_schema_2019_09(v: &Value) -> bool {
    let Value::Object(obj) = v else { return false };
    let Some(s) = obj.get("$schema").and_then(Value::as_str) else { return false };
    if s == JSON_SCHEMA_2019_09_URI_STR {
        return true;
    }

    let Ok(uri) = Uri::parse(s) else { return false };
    is_json_schema_2019_09_uri(&uri)
}

#[must_use]
pub fn json_schema_2019_09_url() -> &'static Url {
    Lazy::force(&JSON_SCHEMA_2019_09_URL)
}
#[must_use]
pub fn json_schema_2019_09_uri() -> &'static Uri {
    Lazy::force(&JSON_SCHEMA_2019_09_URI)
}
#[must_use]
pub fn json_schema_2019_09_absolute_uri() -> &'static AbsoluteUri {
    Lazy::force(&JSON_SCHEMA_2019_09_ABSOLUTE_URI)
}

#[must_use]
pub fn json_hyper_schema_2019_09_url() -> &'static Url {
    Lazy::force(&JSON_HYPER_SCHEMA_2019_09_URL)
}
#[must_use]
pub fn json_hyper_schema_2019_09_uri() -> &'static Uri {
    Lazy::force(&JSON_HYPER_SCHEMA_2019_09_URI)
}
#[must_use]
pub fn json_hyper_schema_2019_09_absolute_uri() -> &'static Uri {
    Lazy::force(&JSON_HYPER_SCHEMA_2019_09_ABSOLUTE_URI)
}

#[must_use]
pub fn dialect() -> &'static Dialect {
    Lazy::force(&JSON_SCHEMA_2019_09_DIALECT)
}

pub fn is_json_hyper_schema_2019_09_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_2019_09_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_2019_09_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}
pub fn is_json_hyper_schema_2019_09_absolute_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_2019_09_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_2019_09_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_2019_09_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_2019_09_URL.domain()
                && u.path() == JSON_SCHEMA_2019_09_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_2019_09_absolute_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_2019_09_URL.domain()
                && u.path() == JSON_SCHEMA_2019_09_URL.path()
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
    fn test_json_schema_2019_09_filter() {
        let schema = json!({
            "$schema": "https://json-schema.org/draft-2019-09/schema#",
            "$id": "https://example.com"
        });

        assert!(is_json_schema_2019_09(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft-2019-09/schema",
            "$id": "https://example.com"
        });
        assert!(is_json_schema_2019_09(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft-2020-12/schema",
            "$id": "https://example.com"
        });
        assert!(!is_json_schema_2019_09(&schema));
    }
}
