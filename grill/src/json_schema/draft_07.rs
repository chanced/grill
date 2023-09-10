mod const_keyword;
mod enum_keyword;
mod multiple_of_keyword;
mod type_keyword;

pub use const_keyword::{ConstInvalid, ConstKeyword};
pub use enum_keyword::{EnumInvalid, EnumKeyword};
// pub use multiple_of_keyword::{MultipleOfKeyword, MultipleOfInvalid};
pub use type_keyword::{TypeInvalid, TypeKeyword};

use crate::{
    error::IdentifyError,
    schema::{Dialect, Metaschema},
    uri::{AbsoluteUri, AsUriRef, Uri},
};
use serde_json::Value;
use url::Url;

pub const JSON_SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/schema#";
pub const JSON_HYPER_SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/hyper-schema#";
pub const JSON_SCHEMA_07_BYTES: &[u8] = include_bytes!("../../../json_schema/07/schema.json");
pub const JSON_HYPER_SCHEMA_07_BYTES: &[u8] =
    include_bytes!("../../../json_schema/07/hyper-schema.json");

pub const JSON_HYPER_SCHEMA_07_LINKS_BYTES: &[u8] =
    include_bytes!("../../../json_schema/07/links.json");

pub const JSON_HYPER_SCHEMA_07_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/07/hyper-schema-output.json");

lazy_static::lazy_static! {
    pub static ref JSON_SCHEMA_07_URL: Url = Url::parse(JSON_SCHEMA_07_URI_STR).unwrap();
    pub static ref JSON_SCHEMA_07_URI: Uri = Uri::Url(JSON_SCHEMA_07_URL.clone());
    pub static ref JSON_SCHEMA_07_ABSOLUTE_URI: AbsoluteUri = AbsoluteUri::Url(JSON_SCHEMA_07_URL.clone());
    pub static ref JSON_HYPER_SCHEMA_07_URL: Url = Url::parse(JSON_HYPER_SCHEMA_07_URI_STR).unwrap();
    pub static ref JSON_HYPER_SCHEMA_07_URI: Uri = Uri::Url(JSON_HYPER_SCHEMA_07_URL.clone());
    pub static ref JSON_HYPER_SCHEMA_07_ABSOLUTE_URI: AbsoluteUri = AbsoluteUri::Url(JSON_HYPER_SCHEMA_07_URL.clone());
    pub static ref JSON_SCHEMA_07_VALUE: Value = serde_json::from_slice(JSON_SCHEMA_07_BYTES).unwrap();
    pub static ref JSON_SCHEMA_07_METASCHEMA: Metaschema = Metaschema::new(
        JSON_SCHEMA_07_ABSOLUTE_URI.clone(),
        JSON_SCHEMA_07_VALUE.as_object().unwrap().clone(),
    );

    pub static ref JSON_SCHEMA_07: Dialect = Dialect::new(
        JSON_SCHEMA_07_ABSOLUTE_URI.clone(),
        [JSON_SCHEMA_07_METASCHEMA.clone()],
        [ConstKeyword::new()],
    )
    .unwrap();
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

#[must_use]
pub fn is_json_schema_07(v: &Value) -> bool {
    let Value::Object(obj) = v else { return false };
    let Some(s) = obj.get("$schema").and_then(Value::as_str) else { return false };
    if s == JSON_SCHEMA_07_URI_STR {
        return true;
    }
    let Ok(uri) = Uri::parse(s) else { return false };
    is_json_schema_07_uri(uri)
}

pub fn is_json_hyper_schema_07_uri(uri: impl AsUriRef) -> bool {
    super::is_uri_for(&JSON_HYPER_SCHEMA_07_URL, uri)
}

pub fn is_json_schema_07_uri(uri: impl AsUriRef) -> bool {
    super::is_uri_for(&JSON_SCHEMA_07_URL, uri)
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
