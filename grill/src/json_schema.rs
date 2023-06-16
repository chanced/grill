//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.

pub mod draft_04;
pub mod draft_07;
pub mod draft_2019_09;
pub mod draft_2020_12;

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
use jsonptr::{Pointer, Token};

use crate::{
    error::{HasFragmentError, IdentifyError, UriParseError},
    Uri,
};
use crate::{Anchor, Keyword};
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

/// An implementation of [`Anchors`](`crate::dialect::Anchors`) which
/// recursively traverses a [`Value`] and returns a [`Vec`] of [`Anchor`]s for
/// all `"$anchor"`, `"$recursiveAnchor"`, `"$dynamicAnchor"` anchors.
///
#[must_use]
pub fn anchors<'v>(ptr: Pointer, value: &'v Value) -> Vec<Anchor<'v>> {
    let recurse = |(tok, value): (Token, &'v Value)| {
        let mut ptr = ptr.clone();
        ptr.push_back(tok);
        anchors(ptr, value)
    };
    match value {
        Value::Array(arr) => arr
            .iter()
            .enumerate()
            .map(|(k, v)| (k.into(), v))
            .flat_map(recurse)
            .collect(),
        Value::Object(obj) => {
            let mut results = Vec::new();
            if let Some(Value::String(anchor)) = obj.get(Keyword::ANCHOR.as_str()) {
                // TODO: did this get renamed from "anchor" in 04 or 07?
                results.push(Anchor::Static {
                    container: value,
                    name: anchor,
                    pointer: ptr.clone(),
                });
            }
            if let Some(Value::String(_)) = obj.get(Keyword::RECURSIVE_ANCHOR.as_str()) {
                results.push(Anchor::Recursive {
                    container: value,
                    pointer: ptr.clone(),
                });
            }
            if let Some(Value::String(anchor)) = obj.get(Keyword::DYNAMIC_ANCHOR.as_str()) {
                results.push(Anchor::Dynamic {
                    name: anchor.as_str(),
                    container: value,
                    pointer: ptr.clone(),
                });
            }
            results.extend(
                obj.iter()
                    .filter(|(_, v)| matches!(v, Value::Object(_) | Value::Array(_)))
                    .map(|(k, v)| (k.into(), v))
                    .flat_map(recurse),
            );
            results
        }
        _ => Vec::new(),
    }
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
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn assert_anchors_contains(anchors: &[Anchor], expected: Anchor) {
        for anc in anchors {
            if anc == &expected {
                return;
            }
        }
        panic!("anchor {expected:?} not found in {anchors:?}");
    }
    fn assert_anchors_not_contains_pointer(anchors: &[Anchor], ptr: &str) {
        for anc in anchors {
            assert!(
                !(anc.pointer() == ptr),
                "Expected anchors not to contain {ptr}"
            );
        }
    }
    #[test]
    fn test_anchors() {
        let fixture = json!({
            "obj": {
                "$anchor": "obj-anchor",
                "$dynamicAnchor": "obj-dynamic-anchor",
                "$recursiveAnchor": ""
            },
            "arr": [
                {
                    "$anchor": "arr-anchor-0"
                },
                {
                    "$anchor": "arr-anchor-1"
                },
                {
                    "$anchor": "arr-anchor-2"
                },
                {
                    "nested": {
                        "$anchor": "nested-anchor",
                    },
                },
            ],
            "malformed": {
                "$anchor": {},
                "$dynamicAnchor": [{}],
                "$recursiveAnchor": 12
            }
        });
        let results = anchors(Pointer::default(), &fixture);
        assert_anchors_contains(
            &results,
            Anchor::Static {
                name: "obj-anchor",
                pointer: "/obj".try_into().unwrap(),
                container: fixture.get("obj").unwrap(),
            },
        );
        assert_anchors_contains(
            &results,
            Anchor::Dynamic {
                name: "obj-dynamic-anchor",
                pointer: "/obj".try_into().unwrap(),
                container: fixture.get("obj").unwrap(),
            },
        );
        assert_anchors_contains(
            &results,
            Anchor::Recursive {
                pointer: "/obj".try_into().unwrap(),
                container: fixture.get("obj").unwrap(),
            },
        );
        assert_anchors_contains(
            &results,
            Anchor::Static {
                name: "arr-anchor-0",
                pointer: "/arr/0".try_into().unwrap(),
                container: fixture.get("arr").unwrap().get(0).unwrap(),
            },
        );
        assert_anchors_contains(
            &results,
            Anchor::Static {
                name: "arr-anchor-1",
                pointer: "/arr/1".try_into().unwrap(),
                container: fixture.get("arr").unwrap().get(1).unwrap(),
            },
        );
        assert_anchors_contains(
            &results,
            Anchor::Static {
                name: "arr-anchor-2",
                pointer: "/arr/2".try_into().unwrap(),
                container: fixture.get("arr").unwrap().get(2).unwrap(),
            },
        );

        assert_anchors_not_contains_pointer(&results, "/malformed");
    }
}
