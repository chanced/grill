use crate::{
    error::IdentifyError,
    schema::{Dialect, Metaschema},
    uri::{AbsoluteUri, AsUriRef},
    Uri,
};
use lazy_static::lazy_static;
use serde_json::Value;
use url::Url;

pub const JSON_SCHEMA_04_URI_STR: &str = "http://json-schema.org/draft-04/schema#";
pub const JSON_HYPER_SCHEMA_04_URI_STR: &str = "http://json-schema.org/draft-04/hyper-schema#";
pub const JSON_SCHEMA_04_BYTES: &[u8] = include_bytes!("../../../../json_schema/04/schema.json");
pub const JSON_HYPER_SCHEMA_04_BYTES: &[u8] =
    include_bytes!("../../../../json_schema/04/hyper-schema.json");
pub const JSON_HYPER_SCHEMA_04_LINKS_BYTES: &[u8] =
    include_bytes!("../../../../json_schema/04/links.json");

lazy_static! {
    pub static ref JSON_SCHEMA_04_URL: Url = Url::parse(JSON_SCHEMA_04_URI_STR).unwrap();
    pub static ref JSON_SCHEMA_04_URI: Uri = Uri::Url(JSON_SCHEMA_04_URL.clone());
    pub static ref JSON_SCHEMA_04_ABSOLUTE_URI: AbsoluteUri =
        AbsoluteUri::Url(JSON_SCHEMA_04_URL.clone());
    pub static ref JSON_HYPER_SCHEMA_04_URL: Url =
        Url::parse(JSON_HYPER_SCHEMA_04_URI_STR).unwrap();
    pub static ref JSON_HYPER_SCHEMA_04_URI: Uri = Uri::Url(JSON_HYPER_SCHEMA_04_URL.clone());
    pub static ref JSON_HYPER_SCHEMA_04_ABSOLUTE_URI: Uri =
        Uri::Url(JSON_HYPER_SCHEMA_04_URL.clone());
    pub static ref JSON_SCHEMA_04_VALUE: Value =
        serde_json::from_slice(JSON_SCHEMA_04_BYTES).unwrap();
    pub static ref JSON_SCHEMA_04: Dialect = Dialect::builder(JSON_SCHEMA_04_ABSOLUTE_URI.clone())
        .with_metaschema(
            JSON_SCHEMA_04_ABSOLUTE_URI.clone(),
            JSON_SCHEMA_04_VALUE.clone()
        )
        .build()
        .unwrap();
}

#[must_use]
pub fn is_json_schema_04(v: &Value) -> bool {
    let Value::Object(obj) = v else { return false };
    let Some(s) = obj.get("$schema").and_then(Value::as_str) else { return false };
    if s == JSON_SCHEMA_04_URI_STR {
        return true;
    }
    let Ok(uri) = Uri::parse(s) else { return false };
    is_json_schema_04_uri(uri)
}

pub fn is_json_schema_04_uri(uri: impl AsUriRef) -> bool {
    super::is_uri_for(&JSON_SCHEMA_04_URL, uri)
}

pub fn is_json_hyper_schema_04_uri(uri: impl AsUriRef) -> bool {
    super::is_uri_for(&JSON_HYPER_SCHEMA_04_URL, uri)
}

/// Identifies JSON Schema Draft 2019-09, 2020-12 schemas.
///
///
/// # Example
/// ```
/// use grill::{Uri, json_schema::identify};
/// use serde_json::json;
/// let schema = json!({
///     "$schema": "https://json-schema.org/draft/2020-12/schema",
///     "$id": "https://example.com/example"
/// });
/// let expected_id = Uri::parse("https://example.com/example").unwrap();
/// assert_eq!(identify(&schema), Ok(Some(expected_id)))
/// ```
/// # Errors
/// Returns [`IdentifyError`] if `schema`:
///   * has an `"$id"` field which can not be parsed as a [`Uri`]
///   * The [`Uri`] parsed from`"$id"` contains a non-empty fragment (i.e. `"https://example.com/example#fragment"`)
pub fn identify_schema(_schema: &Value) -> Result<Option<Uri>, IdentifyError> {
    todo!()
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
