use crate::{
    dialect::{Dialect, Vocabulary},
    json_schema::identify,
    uri::AbsoluteUri,
    Metaschema, Uri,
};
use once_cell::sync::Lazy;
use serde_json::Value;
use url::Url;

pub const JSON_SCHEMA_2020_12_URI_STR: &str = "https://json-schema.org/draft/2020-12/schema";
pub static JSON_SCHEMA_2020_12_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_SCHEMA_2020_12_URI_STR).unwrap());
pub static JSON_SCHEMA_2020_12_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_SCHEMA_2020_12_URL).clone()));
pub static JSON_SCHEMA_2020_12_ABSOLUTE_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::Url(Lazy::force(&JSON_SCHEMA_2020_12_URL).clone()));

pub const JSON_HYPER_SCHEMA_2020_12_URI_STR: &str =
    "https://json-schema.org/draft/2020-12/hyper-schema";
pub static JSON_HYPER_SCHEMA_2020_12_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_HYPER_SCHEMA_2020_12_URI_STR).unwrap());
pub static JSON_HYPER_SCHEMA_2020_12_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_HYPER_SCHEMA_2020_12_URL).clone()));
pub static JSON_HYPER_SCHEMA_2020_12_ABSOLUTE_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_HYPER_SCHEMA_2020_12_URL).clone()));

pub const JSON_SCHEMA_2020_12_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/schema.json");
pub const JSON_HYPER_SCHEMA_2020_12_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/hyper-schema.json");
pub const JSON_HYPER_SCHEMA_2020_12_LINKS_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/links.json");
pub const JSON_HYPER_SCHEMA_2020_12_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/output/hyper-schema.json");

pub const JSON_SCHEMA_2020_12_APPLICATOR_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/applicator.json");
pub const JSON_SCHEMA_2020_12_CONTENT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/content.json");
pub const JSON_SCHEMA_2020_12_CORE_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/core.json");
pub const JSON_SCHEMA_2020_12_FORMAT_ANNOTATION_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/format-annotation.json");
pub const JSON_SCHEMA_2020_12_FORMAT_ASSERTION_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/format-annotation.json");
pub const JSON_HYPER_SCHEMA_2020_12_META_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/hyper-schema.json");
pub const JSON_SCHEMA_2020_12_META_DATA_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/meta-data.json");
pub const JSON_SCHEMA_2020_12_UNEVALUATED_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/unevaluated.json");
pub const JSON_SCHEMA_2020_12_VALIDATION_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/validation.json");

pub static JSON_SCHEMA_2020_12: Lazy<Value> =
    Lazy::new(|| serde_json::from_slice(JSON_SCHEMA_2020_12_BYTES).unwrap());

pub static JSON_SCHEMA_2020_12_DIALECT: Lazy<Dialect> = Lazy::new(|| {
    Dialect::new(
        json_schema_2020_12_absolute_uri().clone(),
        &[Metaschema {
            id: JSON_SCHEMA_2020_12_ABSOLUTE_URI.clone(),
            schema: JSON_SCHEMA_2020_12.as_object().unwrap().clone(),
        }],
        &[Vocabulary::new(
            json_schema_2020_12_absolute_uri(),
            [super::draft_07::ConstHandler::new()], // TOOD: FIX
        )],
        is_json_schema_2020_12,
        identify,
    )
});

#[must_use]
pub fn is_json_schema_2020_12(v: &Value) -> bool {
    let Value::Object(obj) = v else { return false };
    let Some(s) = obj.get("$schema").and_then(Value::as_str) else { return false };
    if s == JSON_SCHEMA_2020_12_URI_STR {
        return true;
    }

    let Ok(uri) = Uri::parse(s) else { return false };
    is_json_schema_2020_12_uri(&uri)
}

#[must_use]
pub fn json_schema_2020_12_url() -> &'static Url {
    Lazy::force(&JSON_SCHEMA_2020_12_URL)
}
#[must_use]
pub fn json_schema_2020_12_uri() -> &'static Uri {
    Lazy::force(&JSON_SCHEMA_2020_12_URI)
}
#[must_use]
pub fn json_schema_2020_12_absolute_uri() -> &'static AbsoluteUri {
    Lazy::force(&JSON_SCHEMA_2020_12_ABSOLUTE_URI)
}

#[must_use]
pub fn json_hyper_schema_2020_12_url() -> &'static Url {
    Lazy::force(&JSON_HYPER_SCHEMA_2020_12_URL)
}
#[must_use]
pub fn json_hyper_schema_2020_12_uri() -> &'static Uri {
    Lazy::force(&JSON_HYPER_SCHEMA_2020_12_URI)
}
#[must_use]
pub fn json_hyper_schema_2020_12_absolute_uri() -> &'static Uri {
    Lazy::force(&JSON_HYPER_SCHEMA_2020_12_ABSOLUTE_URI)
}

#[must_use]
pub fn dialect() -> &'static Dialect {
    Lazy::force(&JSON_SCHEMA_2020_12_DIALECT)
}

pub fn is_json_hyper_schema_2020_12_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_2020_12_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_2020_12_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}
pub fn is_json_hyper_schema_2020_12_absolute_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_2020_12_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_2020_12_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_2020_12_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_2020_12_URL.domain()
                && u.path() == JSON_SCHEMA_2020_12_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_2020_12_absolute_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_2020_12_URL.domain()
                && u.path() == JSON_SCHEMA_2020_12_URL.path()
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
    fn test_json_schema_2020_12_filter() {
        let schema = json!({
            "$schema": "https://json-schema.org/draft-2020-12/schema#",
            "$id": "https://example.com"
        });

        assert!(is_json_schema_2020_12(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft-2020-12/schema",
            "$id": "https://example.com"
        });
        assert!(is_json_schema_2020_12(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft-2020-12/schema",
            "$id": "https://example.com"
        });
        assert!(!is_json_schema_2020_12(&schema));
    }
}
