use crate::{
    error::IdentifyError,
    keyword,
    schema::{Dialect, Metaschema},
    uri::{AbsoluteUri, AsUriRef, Uri},
};
use lazy_static::lazy_static;
use serde_json::Value;
use url::Url;
/// URI for JSON Schema Draft 2019-09 in the form of a `&str`.
///
/// <https://json-schema.org/draft/2019-09/schema>
pub const JSON_SCHEMA_2019_09_URI_STR: &str = "https://json-schema.org/draft/2019-09/schema";

/// URI for JSON Hyper-Schema Draft 2019-09 in the form of a `&str`.
///
/// <https://json-schema.org/draft/2019-09/hyper-schema>
pub const JSON_HYPER_SCHEMA_2019_09_URI_STR: &str =
    "https://json-schema.org/draft/2019-09/hyper-schema";

/// Bytes for JSON Schema Draft 2019-09.
pub const JSON_SCHEMA_2019_09_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/schema.json");

/// Bytes for JSON Hyper-Schema Draft 2019-09.
pub const JSON_HYPER_SCHEMA_2019_09_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/hyper-schema.json");

/// Bytes for JSON Hyper Schema Links Draft 2019-09.
pub const JSON_HYPER_SCHEMA_2019_09_LINKS_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/links.json");

/// Bytes for JSON Schema Output Draft 2019-09.
pub const JSON_SCHEMA_2019_09_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/output/schema.json");

/// Bytes for JSON Hyper Schema Output Draft 2019-09.
pub const JSON_HYPER_SCHEMA_2019_09_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/output/hyper-schema.json");

/// Bytes for JSON Schema Applicator Draft 2019-09.
pub const JSON_SCHEMA_2019_09_APPLICATOR_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/applicator.json");

/// Bytes for JSON Schema Content Draft 2019-09.
pub const JSON_SCHEMA_2019_09_CONTENT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/content.json");

/// Bytes for JSON Schema Core Draft 2019-09.
pub const JSON_SCHEMA_2019_09_CORE_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/core.json");

/// Bytes for JSON Schema Format Draft 2019-09.
pub const JSON_SCHEMA_2019_09_FORMAT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/format.json");

/// Bytes for JSON Schema Meta-Data Draft 2019-09.
pub const JSON_SCHEMA_2019_09_META_DATA_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/meta-data.json");

/// Bytes for JSON Schema Validation Draft 2019-09.
pub const JSON_SCHEMA_2019_09_VALIDATION_BYTES: &[u8] =
    include_bytes!("../../../json_schema/2019-09/meta/validation.json");

lazy_static! {
    /// [`Url`] for JSON Schema Draft 2019-09.
    ///
    /// <https://json-schema.org/draft/2019-09/schema>
    pub static ref JSON_SCHEMA_2019_09_URL: Url = Url::parse(JSON_SCHEMA_2019_09_URI_STR).unwrap();

    /// [`Uri`] for JSON Schema Draft 2019-09.
    ///
    /// <https://json-schema.org/draft/2019-09/schema>
    pub static ref JSON_SCHEMA_2019_09_URI: Uri = Uri::Url(JSON_SCHEMA_2019_09_URL.clone());

    /// [`Url`] for JSON Schema Hyper Draft 2019-09.
    ///
    pub static ref JSON_HYPER_SCHEMA_2019_09_URL: Url = Url::parse(JSON_HYPER_SCHEMA_2019_09_URI_STR).unwrap();

    /// [`Uri`] for JSON Hyper Schema Draft 2019-09.
    ///
    /// <https://json-schema.org/draft/2019-09/hyper-schema>
    pub static ref JSON_HYPER_SCHEMA_2019_09_URI: Uri = Uri::Url(JSON_HYPER_SCHEMA_2019_09_URL.clone());

    /// [`AbsoluteUri`] for JSON Schema Draft 2019-09.
    ///
    /// <https://json-schema.org/draft/2019-09/schema>
    pub static ref JSON_SCHEMA_2019_09_ABSOLUTE_URI: AbsoluteUri = AbsoluteUri::Url(JSON_SCHEMA_2019_09_URL.clone());

    /// [`AbsoluteUri`] for JSON Hyper Schema Draft 2019-09.
    ///
    /// <https://json-schema.org/draft/2019-09/hyper-schema>
    pub static ref JSON_HYPER_SCHEMA_2019_09_ABSOLUTE_URI: AbsoluteUri = AbsoluteUri::Url(JSON_HYPER_SCHEMA_2019_09_URL.clone());

    /// [`Value`] for JSON Schema Output Draft 2019-09.
    pub static ref JSON_SCHEMA_2019_09_OUTPUT_VALUE: Value = serde_json::from_slice(
        JSON_SCHEMA_2019_09_OUTPUT_BYTES
    ).unwrap();

    /// [`Value`] for JSON Hyper Schema Output Draft 2019-09.
    pub static ref JSON_HYPER_SCHEMA_2019_09_OUTPUT_VALUE: Value = serde_json::from_slice(
        JSON_HYPER_SCHEMA_2019_09_OUTPUT_BYTES
    ).unwrap();

    // /// [`Metaschema`] of JSON Schema Draft 2019-09.
    // pub static ref JSON_SCHEMA_2019_09_METASCHEMA: Metaschema =  Metaschema::new(
    //     JSON_SCHEMA_2019_09_ABSOLUTE_URI.clone(),
    //     serde_json::from_slice(JSON_SCHEMA_2019_09_BYTES).unwrap(),
    // );

    // /// [`Metaschema`] of JSON Hyper Schema Draft 2019-09.
    // pub static ref JSON_HYPER_SCHEMA_2019_09_METASCHEMA: Metaschema = Metaschema::new(
    //     JSON_HYPER_SCHEMA_2019_09_ABSOLUTE_URI.clone(),
    //     serde_json::from_slice(JSON_HYPER_SCHEMA_2019_09_BYTES).unwrap(),
    // );

    // /// [`Metaschema`] of JSON Hyper Schema Links Draft 2019-09.
    // pub static ref JSON_HYPER_SCHEMA_2019_09_LINKS_METASCHEMA: Metaschema = Metaschema::new(
    //     AbsoluteUri::parse("https://json-schema.org/draft/2019-09/links").unwrap(),
    //     serde_json::from_slice(JSON_HYPER_SCHEMA_2019_09_LINKS_BYTES).unwrap(),
    // );

    // /// [`Metaschema`] of JSON Schema Applicator Draft 2019-09.
    // pub static ref JSON_SCHEMA_2019_09_APPLICATOR_METASCHEMA: Metaschema = Metaschema::new(
    //     AbsoluteUri::parse("https://json-schema.org/draft/2019-09/meta/applicator").unwrap(),
    //     serde_json::from_slice(JSON_SCHEMA_2019_09_APPLICATOR_BYTES).unwrap(),
    // );

    // /// [`Metaschema`] of JSON Schema Content Draft 2019-09.
    // pub static ref JSON_SCHEMA_2019_09_CONTENT_METASCHEMA: Metaschema = Metaschema::new(
    //     AbsoluteUri::parse("https://json-schema.org/draft/2019-09/meta/content").unwrap(),
    //     serde_json::from_slice(JSON_SCHEMA_2019_09_CONTENT_BYTES).unwrap(),
    // );

    // /// [`Metaschema`] of JSON Schema Core Draft 2019-09.
    // pub static ref JSON_SCHEMA_2019_09_CORE_METASCHEMA: Metaschema = Metaschema::new(
    //     AbsoluteUri::parse("https://json-schema.org/draft/2019-09/meta/core").unwrap(),
    //     serde_json::from_slice(JSON_SCHEMA_2019_09_CORE_BYTES).unwrap(),
    // );

    // /// [`Metaschema`] of JSON Schema Format Draft 2019-09.
    // pub static ref JSON_SCHEMA_2019_09_FORMAT_METASCHEMA: Metaschema = Metaschema::new(
    //     AbsoluteUri::parse("https://json-schema.org/draft/2019-09/meta/format").unwrap(),
    //     serde_json::from_slice(JSON_SCHEMA_2019_09_FORMAT_BYTES).unwrap(),
    // );

    // /// [`Metaschema`] of JSON Schema Meta-Data Draft 2019-09.
    // pub static ref JSON_SCHEMA_2019_09_META_DATA_METASCHEMA: Metaschema =  Metaschema::new(
    //     AbsoluteUri::parse("https://json-schema.org/draft/2019-09/meta/meta-data").unwrap(),
    //     serde_json::from_slice(JSON_SCHEMA_2019_09_META_DATA_BYTES).unwrap(),
    // );

    // /// [`Metaschema`] of JSON Schema Validation Draft 2019-09.
    // pub static ref JSON_SCHEMA_2019_09_VALIDATION_METASCHEMA: Metaschema = Metaschema::new(
    //     AbsoluteUri::parse("https://json-schema.org/draft/2019-09/meta/validation").unwrap(),
    //     serde_json::from_slice(JSON_SCHEMA_2019_09_VALIDATION_BYTES).unwrap(),
    // );

    /// [`Dialect`] for JSON Schema Draft 2019-09.
    pub static ref JSON_SCHEMA_2019_09: Dialect = Dialect::builder(JSON_SCHEMA_2019_09_ABSOLUTE_URI.clone()).build().unwrap();
}

/// Returns `true` if the `value` is definitively JSON Schema Draft 2019-09.
#[must_use]
pub fn is_json_schema_2019_09(v: &Value) -> bool {
    // booleans are handled the same way across json schema dialects
    // so there's no need to cycle through the remaining schemas
    // just to ultimately end up with a default dialect
    if v.is_boolean() {
        return true;
    }
    let Value::Object(obj) = v else { return false };
    let Some(s) = obj.get(keyword::SCHEMA).and_then(Value::as_str) else { return false };
    if s == JSON_SCHEMA_2019_09_URI_STR {
        return true;
    }
    let Ok(uri) = AbsoluteUri::parse(s) else { return false };
    is_json_schema_2019_09_uri(uri)
}

/// Returns `true` if `uri` is equal to JSON Schema Draft 2019-09 URI.
///
/// Any of the following return `true`:
///
/// - `"https://json-schema.org/draft/2019-09/schema"`
/// - `"https://json-schema.org/draft/2019-09/schema#"`
/// - `"http://json-schema.org/draft/2019-09/schema"`
/// - `"http://json-schema.org/draft/2019-09/schema#"`
#[must_use]
pub fn is_json_schema_2019_09_uri(uri: impl AsUriRef) -> bool {
    super::is_uri_for(&JSON_SCHEMA_2019_09_URL, uri)
}

/// Identifies JSON Schema Draft 2019-09, 2020-12 schemas.
///
/// # Example
/// ```
/// use grill::{ Uri, json_schema::draft_2019_09::identify_schema };
/// use serde_json::json;
/// let schema = json!({
///     "$schema": "https://json-schema.org/draft/2020-12/schema",
///     "$id": "https://example.com/example"
/// });
/// let expected_id = Uri::parse("https://example.com/example").unwrap();
/// assert_eq!(identify_schema(&schema), Ok(Some(expected_id)))
/// ```
/// # Errors
/// Returns [`IdentifyError`] if `schema`:
///   * has an `"$id"` field which can not be parsed as a [`Uri`]
///   * The [`Uri`] parsed from`"$id"` contains a non-empty fragment (i.e. `"https://example.com/example#fragment"`)
pub fn identify_schema(schema: &Value) -> Result<Option<Uri>, IdentifyError> {
    let Some(id) = schema.get(keyword::ID).and_then(Value::as_str) else { return Ok(None) };
    let uri = Uri::parse(id)?;
    let Some(fragment) = uri.fragment() else { return Ok(Some(uri)) };
    if fragment.is_empty() {
        Ok(Some(uri))
    } else {
        Err(IdentifyError::FragmentedId(uri))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use serde_json::json;

    use crate::schema::Dialects;

    use super::*;

    #[test]
    fn test_json_schema_2019_09_filter() {
        let schema = json!({
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "$id": "https://example.com"
        });

        assert!(is_json_schema_2019_09(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "$id": "https://example.com"
        });
        assert!(is_json_schema_2019_09(&schema));

        let schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com"
        });
        assert!(!is_json_schema_2019_09(&schema));
    }

    #[test]
    fn test_locate_schemas() {
        let base_uri = "https://example.com/example-schema.json";
        let _tests = [
            (
                // 0
                json!({
                    "$schema": "https://json-schema.org/draft/2019-09/schema",
                    "$id": "https://example.com/root",
                    "properties": {
                        "foo": {
                            "$id": "https://example.com/foo",
                            "type": "string"
                        }
                    }
                }),
                HashSet::from([
                    base_uri,
                    "https://example.com/root",
                    "https://example.com/root#/properties/foo",
                    "https://example.com/foo",
                ]),
            ),
            // 1
            (json!({}), HashSet::from([base_uri])),
            // 2
            (
                json!({"$schema": "https://json-schema.org/draft/2019-09/schema"}),
                HashSet::from([base_uri]),
            ),
            // 3
            (
                json!({"$schema": "https://json-schema.org/draft/2019-09/schema", "$id": "https://example.com/root"}),
                HashSet::from([base_uri, "https://example.com/root"]),
            ),
            // 4
            (
                json!({
                    "$schema": "https://json-schema.org/draft/2019-09/schema",
                    "$id": "https://example.com/root",
                    "properties": {
                        "foo": {
                            "$anchor": "foo",
                            "type": "string"
                        }
                    }
                }),
                HashSet::from([
                    base_uri,
                    "https://example.com/root",
                    "https://example.com/root#/properties/foo",
                    "https://example.com/root#foo",
                ]),
            ),
            // 5
            (
                json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$id": "https://example.com/example.json",
                    "$defs": {
                        "foo": {
                            "$id": "https://example.com/foo.json",
                            "$defs": {
                                "bar": {
                                    "$id": "https://example.com/bar.json",
                                    "$defs": {
                                        "$anchor": "anchor",
                                        "$dynamicAnchor": "dynamicAnchor",
                                    }
                                }
                            }
                        }
                    }
                }),
                HashSet::from([
                    base_uri,
                    "https://example.com/example.json",
                    "https://example.com/example.json#/$defs/foo",
                    "https://example.com/foo.json",
                    "https://example.com/foo.json#/$defs/bar",
                    "https://example.com/bar.json",
                    "https://example.com/bar.json#anchor",
                    "https://example.com/bar.json#dynamicAnchor",
                ]),
            ),
        ];
        let dialect = JSON_SCHEMA_2019_09.clone();

        let _dialects: Dialects = Dialects::new(vec![dialect.clone()], Some(dialect.id())).unwrap();
        let _base_uri = "https://example.com/example-schema.json"
            .parse::<AbsoluteUri>()
            .unwrap();

        // for (idx, (schema, mut expected)) in tests.into_iter().enumerate() {
        //     let located = dialect
        //         .locate_schemas(Pointer::default(), &schema, dialects, &base_uri)
        //         .unwrap();
        //     for loc in located.iter().map(|ls| ls.uri.as_str()) {
        //         assert!(
        //             expected.contains(loc),
        //             "\n\nUnexpected location found in test #{idx}: \n\t\"{loc}\"\n\n"
        //         );
        //         expected.remove(loc);
        //     }
        //     assert!(
        //         expected.is_empty(),
        //         "\n\nExpected: {:#?} not found on test #{idx}.\n\n",
        //         expected.into_iter().collect::<Vec<_>>()
        //     );
        // }
    }

    // fn assert_anchors_contains(anchors: &[AnchorLocation], expected: AnchorLocation<'_>) {
    //     for anc in anchors {
    //         if anc == &expected {
    //             return;
    //         }
    //     }
    //     panic!("anchor {expected:?} not found in {anchors:?}");
    // }
    // fn assert_anchors_not_contains_pointer(anchors: &[AnchorLocation], ptr: &str) {
    //     for anc in anchors {
    //         assert!(
    //             !(anc.container_location == ptr),
    //             "Expected anchors not to contain {ptr}"
    //         );
    //     }
    // }
    // #[test]
    // fn test_anchors() {
    //     let fixture = json!({
    //         "obj": {
    //             "$anchor": "obj-anchor",
    //             "$dynamicAnchor": "obj-dynamic-anchor",
    //             "$recursiveAnchor": ""
    //         },
    //         "arr": [
    //             {
    //                 "$anchor": "arr-anchor-0"
    //             },
    //             {
    //                 "$anchor": "arr-anchor-1"
    //             },
    //             {
    //                 "$anchor": "arr-anchor-2"
    //             },
    //             {
    //                 "nested": {
    //                     "$anchor": "nested-anchor",
    //                 },
    //             },
    //         ],
    //         "malformed": {
    //             "$anchor": {},
    //             "$dynamicAnchor": [{}],
    //             "$recursiveAnchor": 12
    //         }
    //     });
    //     let results = locate_anchors(Pointer::default(), &fixture);
    //     assert_anchors_contains(
    //         &results,
    //         AnchorLocation::new(
    //             Keyword::ANCHOR,
    //             "/obj".try_into().unwrap(),
    //             "obj-anchor",
    //             fixture.get("obj").unwrap(),
    //         ),
    //     );
    //     assert_anchors_contains(
    //         &results,
    //         AnchorLocation::new(
    //             Keyword::DYNAMIC_ANCHOR,
    //             "/obj".try_into().unwrap(),
    //             "obj-dynamic-anchor",
    //             fixture.get("obj").unwrap(),
    //         ),
    //     );

    //     assert_anchors_contains(
    //         &results,
    //         AnchorLocation::new(
    //             Keyword::RECURSIVE_ANCHOR,
    //             "/obj".try_into().unwrap(),
    //             "",
    //             fixture.get("obj").unwrap(),
    //         ),
    //     );
    //     assert_anchors_contains(
    //         &results,
    //         AnchorLocation::new(
    //             Keyword::ANCHOR,
    //             "/arr/0".try_into().unwrap(),
    //             "arr-anchor-0",
    //             fixture.get("arr").unwrap().get(0).unwrap(),
    //         ),
    //     );
    //     assert_anchors_contains(
    //         &results,
    //         AnchorLocation::new(
    //             Keyword::ANCHOR,
    //             "/arr/1".try_into().unwrap(),
    //             "arr-anchor-1",
    //             fixture.get("arr").unwrap().get(1).unwrap(),
    //         ),
    //     );
    //     assert_anchors_contains(
    //         &results,
    //         AnchorLocation::new(
    //             Keyword::ANCHOR,
    //             "/arr/2".try_into().unwrap(),
    //             "arr-anchor-2",
    //             fixture.get("arr").unwrap().get(2).unwrap(),
    //         ),
    //     );

    //     assert_anchors_not_contains_pointer(&results, "/malformed");
    // }
}
