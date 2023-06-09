mod keyword;

pub use keyword::Keyword;

use crate::{
    error::{HasFragmentError, IdentifyError, UriParseError},
    Uri,
};
use serde_json::Value;

/// Identifies JSON Schema Draft 07 through current
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
pub fn identify(schema: &Value) -> Result<Option<Uri>, IdentifyError> {
    let Some(id) = schema.get(Keyword::ID.as_ref()).and_then(Value::as_str) else { return Ok(None) };
    let uri = Uri::parse(id)?;
    let Some(fragment) = uri.fragment() else { return Ok(Some(uri))};
    if fragment.is_empty() {
        Ok(Some(uri))
    } else {
        Err(IdentifyError::HasFragment {
            source: HasFragmentError { uri },
        })
    }
}

/// Identifies JSON Schema Draft 04 and earlier.
///
/// # Example
/// ```
/// use grill::{Uri, json_schema::identify_legacy};
/// use serde_json::json;
/// let schema = json!({
///     "$schema": "https://json-schema.org/draft/2020-12/schema",
///     "id": "https://example.com/example"
/// });
/// let expected_id = Uri::parse("https://example.com/example").unwrap();
/// assert_eq!(identify_legacy(&schema), Ok(Some(expected_id)))
/// ```
/// # Errors
/// Returns [`UriParseError`] if `schema` has an `"id"` field which can not be parsed as a [`Uri`]
pub fn identify_legacy(schema: &Value) -> Result<Option<Uri>, UriParseError> {
    let Some(id) = schema.get(Keyword::ID_LEGACY.as_ref()).and_then(Value::as_str) else { return Ok(None)};
    Ok(Some(Uri::parse(id)?))
}
