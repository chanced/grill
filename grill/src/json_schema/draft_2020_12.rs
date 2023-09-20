mod metaschemas;

pub use metaschemas::*;

pub use super::draft_2019_09::identify_schema;
use crate::{
    schema::Dialect,
    uri::{AbsoluteUri, AsUriRef, Uri},
};
use serde_json::Value;
use url::Url;

/// URI for JSON Schema Draft 2020-12 in the form of a `&str`.
///
/// <https://json-schema.org/draft/2020-12/schema>
pub const JSON_SCHEMA_2020_12_URI_STR: &str = "https://json-schema.org/draft/2020-12/schema";

/// URI for JSON Hyper-Schema Draft 2020-12 in the form of a `&str`.
///
/// <https://json-schema.org/draft/2020-12/hyper-schema>
pub const JSON_HYPER_SCHEMA_2020_12_URI_STR: &str =
    "https://json-schema.org/draft/2020-12/hyper-schema";

/// Bytes for JSON Schema Draft Metaschema 2020-12.
pub const JSON_SCHEMA_2020_12_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/schema.json");

/// Bytes for JSON Hyper-Schema Metaschema Draft 2020-12.
pub const JSON_HYPER_SCHEMA_2020_12_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/hyper-schema.json");

/// Bytes for JSON Hyper Schema Links Metaschema Draft 2020-12.
///
/// <https://json-schema.org/draft/2020-12/links>
pub const JSON_HYPER_SCHEMA_2020_12_LINKS_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/links.json");

/// Bytes for JSON Hyper Schema Output Metaschema Draft 2020-12.
///
/// <https://json-schema.org/draft/2019-09/output/hyper-schema>
pub const JSON_HYPER_SCHEMA_2020_12_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/output/hyper-schema.json");

/// Bytes for JSON Schema Applicator Metaschema Draft 2020-12.
///
/// <https://json-schema.org/draft/2020-12/meta/applicator>
pub const JSON_SCHEMA_2020_12_APPLICATOR_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/applicator.json");

/// Bytes for JSON Schema Content Metaschema Draft 2020-12
///
/// <https://json-schema.org/draft/2020-12/meta/content>
pub const JSON_SCHEMA_2020_12_CONTENT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/content.json");

/// Bytes for JSON Schema Core Metaschema Draft 2020-12
///
/// <https://json-schema.org/draft/2020-12/meta/core>
pub const JSON_SCHEMA_2020_12_CORE_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/core.json");

/// Bytes for JSON Schema Format Annotation Metaschema Draft 2020-12
///
/// <https://json-schema.org/draft/2020-12/meta/format-annotation>
pub const JSON_SCHEMA_2020_12_FORMAT_ANNOTATION_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/format-annotation.json");

/// Bytes for JSON Schema Format Assertion Metaschema Draft 2020-12
///
/// <https://json-schema.org/draft/2020-12/meta/format-assertion>
pub const JSON_SCHEMA_2020_12_FORMAT_ASSERTION_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/format-annotation.json");

/// Bytes for JSON Hyper Schema Metaschema Draft 2020-12
///
/// <https://json-schema.org/draft/2020-12/meta/hyper-schema>
pub const JSON_HYPER_SCHEMA_2020_12_META_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/hyper-schema.json");

/// Bytes for JSON Schema Meta Data Metaschema Draft 2020-12
///
/// <https://json-schema.org/draft/2020-12/meta/meta-data>
pub const JSON_SCHEMA_2020_12_META_DATA_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/meta-data.json");

/// Bytes for JSON Schema Unevaluated Metaschema Draft 2020-12
///
/// <https://json-schema.org/draft/2020-12/meta/unevaluated>
pub const JSON_SCHEMA_2020_12_UNEVALUATED_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/unevaluated.json");

/// Bytes for JSON Schema Validation Metaschema Draft 2020-12
///
/// <https://json-schema.org/draft/2020-12/meta/validation>
pub const JSON_SCHEMA_2020_12_VALIDATION_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2020-12/meta/validation.json");

lazy_static::lazy_static! {
    /// [`Url`](`url::Url`) for JSON Schema Draft 2020-12.
    pub static ref JSON_SCHEMA_2020_12_URL: Url = Url::parse(JSON_SCHEMA_2020_12_URI_STR).unwrap();
    /// [`Uri`] for JSON Schema Draft 2020-12.
    ///
    /// <https://json-schema.org/draft/2020-12/schema>

    pub static ref JSON_SCHEMA_2020_12_URI: Uri = Uri::Url(JSON_SCHEMA_2020_12_URL.clone());
    /// [`AbsoluteUri`] for JSON Schema Draft 2020-12.
    ///
    /// <https://json-schema.org/draft/2020-12/schema>

    /// [`AbsoluteUri`] for JSON Schema Draft 2020-12.
    ///
    /// <https://json-schema.org/draft/2020-12/schema>

    pub static ref JSON_SCHEMA_2020_12_ABSOLUTE_URI: AbsoluteUri = AbsoluteUri::Url(JSON_SCHEMA_2020_12_URL.clone());
    /// [`Url`] for JSON Hyper Schema Draft 2020-12.
    ///
    /// <https://json-schema.org/draft/2020-12/hyper-schema>
    pub static ref JSON_HYPER_SCHEMA_2020_12_URL: Url = Url::parse(JSON_HYPER_SCHEMA_2020_12_URI_STR).unwrap();

    /// [`Uri`] for JSON Hyper Schema Draft 2020-12.
    ///
    /// <https://json-schema.org/draft/2020-12/hyper-schema>
    pub static ref JSON_HYPER_SCHEMA_2020_12_URI: Uri = Uri::Url(JSON_HYPER_SCHEMA_2020_12_URL.clone());

    /// [`AbsoluteUri`] for JSON Hyper Schema Draft 2020-12.
    ///
    /// <https://json-schema.org/draft/2020-12/hyper-schema>
    pub static ref JSON_HYPER_SCHEMA_2020_12_ABSOLUTE_URI: Uri =  Uri::Url(JSON_HYPER_SCHEMA_2020_12_URL.clone());

    /// JSON [`Value`] for JSON Schema Draft 2020-12
    pub static ref JSON_SCHEMA_2020_12_VALUE: Value = serde_json::from_slice(
        JSON_SCHEMA_2020_12_BYTES
    ).unwrap();

}

/// Returns `true` if the `value` is definitively JSON Schema Draft 2020-12.
#[must_use]
pub fn is_json_schema_2020_12(value: &Value) -> bool {
    let Value::Object(obj) = value else { return false };
    let Some(s) = obj.get("$schema").and_then(Value::as_str) else { return false };
    if s == JSON_SCHEMA_2020_12_URI_STR {
        return true;
    }

    let Ok(uri) = AbsoluteUri::parse(s) else { return false };
    is_json_schema_2020_12_uri(uri)
}

/// Returns `true` if `uri` is equal to JSON Hyper Schema Draft 2020-12 URI.
///
/// Any of the following return `true`:
/// - `"https://json-schema.org/draft/2020-12/hyper-schema"`
/// - `"https://json-schema.org/draft/2020-12/hyper-schema#"`
/// - `"http://json-schema.org/draft/2020-12/hyper-schema"`
/// - `"http://json-schema.org/draft/2020-12/hyper-schema#"`
pub fn is_json_hyper_schema_2020_12_uri(uri: impl AsUriRef) -> bool {
    super::is_uri_for(&JSON_HYPER_SCHEMA_2020_12_URL, uri)
}

/// Returns `true` if `uri` is equal to JSON Schema Draft 2020-12 URI.
///
/// Any of the following return `true`:
/// - `"https://json-schema.org/draft/2020-12/schema"`
/// - `"https://json-schema.org/draft/2020-12/schema#"`
/// - `"http://json-schema.org/draft/2020-12/schema"`
/// - `"http://json-schema.org/draft/2020-12/schema#"`
pub fn is_json_schema_2020_12_uri(uri: impl AsUriRef) -> bool {
    super::is_uri_for(&JSON_HYPER_SCHEMA_2020_12_URL, uri)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_json_schema_2020_12_filter() {
        let schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema#",
            "$id": "https://example.com"
        });

        assert!(is_json_schema_2020_12(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
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
