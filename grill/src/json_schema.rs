//! JSON Schema integration

pub mod draft_04;
pub mod draft_07;
pub mod draft_2019_09;
pub mod draft_2020_12;
mod keyword;

pub use draft_04::dialect as draft_04_dialect;
pub use draft_07::dialect as draft_07_dialect;
pub use draft_2019_09::dialect as draft_2019_09_dialect;
pub use draft_2020_12::dialect as draft_2020_12_dialect;

pub use draft_04::{
    is_json_hyper_schema_04_absolute_uri, is_json_hyper_schema_04_uri, is_json_schema_04,
    is_json_schema_04_absolute_uri, is_json_schema_04_uri, json_hyper_schema_04_absolute_uri,
    json_hyper_schema_04_uri, json_hyper_schema_04_url, json_schema_04_absolute_uri,
    json_schema_04_uri, json_schema_04_url,
};

pub use draft_07::{
    is_json_hyper_schema_07_absolute_uri, is_json_hyper_schema_07_uri, is_json_schema_07,
    is_json_schema_07_absolute_uri, is_json_schema_07_uri, json_hyper_schema_07_absolute_uri,
    json_hyper_schema_07_uri, json_hyper_schema_07_url, json_schema_07_absolute_uri,
    json_schema_07_uri, json_schema_07_url,
};
pub use draft_2019_09::{
    is_json_hyper_schema_2019_09_absolute_uri, is_json_hyper_schema_2019_09_uri,
    is_json_schema_2019_09, is_json_schema_2019_09_absolute_uri, is_json_schema_2019_09_uri,
    json_hyper_schema_2019_09_absolute_uri, json_hyper_schema_2019_09_uri,
    json_hyper_schema_2019_09_url, json_schema_2019_09_absolute_uri, json_schema_2019_09_uri,
    json_schema_2019_09_url,
};

pub use draft_2020_12::{
    is_json_hyper_schema_2020_12_absolute_uri, is_json_hyper_schema_2020_12_uri,
    is_json_schema_2020_12, is_json_schema_2020_12_absolute_uri, is_json_schema_2020_12_uri,
    json_hyper_schema_2020_12_absolute_uri, json_hyper_schema_2020_12_uri,
    json_hyper_schema_2020_12_url, json_schema_2020_12_absolute_uri, json_schema_2020_12_uri,
    json_schema_2020_12_url,
};

use crate::{
    error::{HasFragmentError, IdentifyError, UriParseError},
    Uri,
};
pub use keyword::Keyword;
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

// #[derive(Default)]
// pub struct JsonSchema<'instance, 'schema, 'state> {
//     _instance_marker: PhantomData<&'instance ()>,
//     _schema_marker: PhantomData<&'schema ()>,
//     _state_marker: PhantomData<&'state ()>,
// }

// impl<'instance, 'schema, 'state> Integration for JsonSchema<'instance, 'schema, 'state> {
//     type Output = Value;
//     type Annotation = Annotation<'instance>;
//     type PartialId = Uri;
//     type Id = AbsoluteUri;
//     type Scope = Scope<'state>;
//     type Compile = Compile<'schema>;
// }
