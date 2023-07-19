mod const_handler;
mod enum_handler;
mod multiple_of_handler;
mod type_handler;

pub use const_handler::{ConstHandler, ConstInvalid};
pub use enum_handler::{EnumHandler, EnumInvalid};
use jsonptr::Pointer;
pub use multiple_of_handler::{MultipleOfHandler, MultipleOfInvalid};
pub use type_handler::{TypeHandler, TypeInvalid};

use crate::{
    dialect::{Dialect, Dialects, LocatedSchema},
    error::{IdentifyError, LocateSchemasError},
    uri::AbsoluteUri,
    Metaschema, Uri,
};
use once_cell::sync::Lazy;
use serde_json::Value;
use url::Url;

pub const JSON_SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/schema#";
pub static JSON_SCHEMA_07_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_SCHEMA_07_URI_STR).unwrap());
pub static JSON_SCHEMA_07_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_SCHEMA_07_URL).clone()));
pub static JSON_SCHEMA_07_ABSOLUTE_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::Url(Lazy::force(&JSON_SCHEMA_07_URL).clone()));

pub const JSON_HYPER_SCHEMA_07_URI_STR: &str = "http://json-schema.org/draft-07/hyper-schema#";
pub static JSON_HYPER_SCHEMA_07_URL: Lazy<Url> =
    Lazy::new(|| Url::parse(JSON_HYPER_SCHEMA_07_URI_STR).unwrap());
pub static JSON_HYPER_SCHEMA_07_URI: Lazy<Uri> =
    Lazy::new(|| Uri::Url(Lazy::force(&JSON_HYPER_SCHEMA_07_URL).clone()));
pub static JSON_HYPER_SCHEMA_07_ABSOLUTE_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::Url(Lazy::force(&JSON_HYPER_SCHEMA_07_URL).clone()));

pub const JSON_SCHEMA_07_BYTES: &[u8] = include_bytes!("../../../json_schema/07/schema.json");
pub const JSON_HYPER_SCHEMA_07_BYTES: &[u8] =
    include_bytes!("../../../json_schema/07/hyper-schema.json");
pub const JSON_HYPER_SCHEMA_07_LINKS_BYTES: &[u8] =
    include_bytes!("../../../json_schema/07/links.json");
pub const JSON_HYPER_SCHEMA_07_OUTPUT_BYTES: &[u8] =
    include_bytes!("../../../json_schema/07/hyper-schema-output.json");

pub static JSON_SCHEMA_07_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        json_schema_07_absolute_uri().clone(),
        serde_json::from_slice(JSON_SCHEMA_07_BYTES).unwrap(),
    )
});

pub static JSON_SCHEMA_07_DIALECT: Lazy<Dialect> = Lazy::new(|| {
    Dialect::new(
        json_schema_07_absolute_uri().clone(),
        [Lazy::force(&JSON_SCHEMA_07_METASCHEMA)],
        [ConstHandler::new()],
    )
});

/// An implementation of [`LocateSchemas`](`crate::dialect::LocateSchemas`)
/// which recursively traverses a [`Value`] and returns a [`Vec`] of
/// [`LocatedSchema`]s for each identified (via `$id`) subschema and for each
/// schema with an`"$anchor"`.
///
pub fn locate_schemas<'v>(
    _ptr: Pointer,
    _value: &'v Value,
    _dialects: Dialects,
    _base_uri: &AbsoluteUri,
) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
    // match value {
    //     Value::Array(arr) => ident_locations_from_arr(ptr, arr, dialects, base_uri),
    //     Value::Object(_) => ident_locations_from_obj(ptr, value, dialects, base_uri),
    //     _ => Vec::new(),
    // }
    todo!()
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

// #[must_use]
// pub fn locate_schemas<'v>(ptr: Pointer, value: &'v Value) -> Vec<SchemaLocation<'v>> {
//     let recurse = |(tok, value): (Token, &'v Value)| {
//         let mut ptr = ptr.clone();
//         ptr.push_back(tok);
//         locate_schemas(ptr, value)
//     };
//     match value {
//         Value::Array(arr) => arr
//             .iter()
//             .enumerate()
//             .map(|(k, v)| (k.into(), v))
//             .flat_map(recurse)
//             .collect(),
//         Value::Object(obj) => {
//             let mut results = Vec::new();
//             if let Some(Value::String(id)) = obj.get(Keyword::ID.as_str()) {
//                 let Ok(uri) = Uri::parse(id) else { return Vec::new() };
//                 if uri.path_or_nss().is_empty() && uri.authority_or_namespace().is_none() {
//                     if let Some(fragment) = uri.fragment() {
//                         if !fragment.is_empty() && !fragment.starts_with("/") {
//                             results.push(AnchorLocation::new(
//                                 Keyword::ID,
//                                 ptr.clone(),
//                                 anchor,
//                                 value,
//                             ));
//                         }
//                     }
//                 }
//             }
//             results.extend(
//                 obj.iter()
//                     .filter(|(_, v)| matches!(v, Value::Object(_) | Value::Array(_)))
//                     .map(|(k, v)| (k.into(), v))
//                     .flat_map(recurse),
//             );
//             results
//         }
//         _ => Vec::new(),
//     }
// }

#[must_use]
pub fn is_json_schema_07(v: &Value) -> bool {
    // bools are handled the same way across json schema dialects
    // so there's no need to cycle through the remaining schemas
    // just to ultimately end up with a default dialect
    if v.is_boolean() {
        return true;
    }

    let Value::Object(obj) = v else { return false };
    let Some(s) = obj.get("$schema").and_then(Value::as_str) else { return false };
    if s == JSON_SCHEMA_07_URI_STR {
        return true;
    }

    let Ok(uri) = Uri::parse(s) else { return false };
    is_json_schema_07_uri(&uri)
}

#[must_use]
pub fn json_schema_07_url() -> &'static Url {
    Lazy::force(&JSON_SCHEMA_07_URL)
}
#[must_use]
pub fn json_schema_07_uri() -> &'static Uri {
    Lazy::force(&JSON_SCHEMA_07_URI)
}
#[must_use]
pub fn json_schema_07_absolute_uri() -> &'static AbsoluteUri {
    Lazy::force(&JSON_SCHEMA_07_ABSOLUTE_URI)
}

#[must_use]
pub fn json_hyper_schema_07_url() -> &'static Url {
    Lazy::force(&JSON_HYPER_SCHEMA_07_URL)
}
#[must_use]
pub fn json_hyper_schema_07_uri() -> &'static Uri {
    Lazy::force(&JSON_HYPER_SCHEMA_07_URI)
}
#[must_use]
pub fn json_hyper_schema_07_absolute_uri() -> &'static AbsoluteUri {
    Lazy::force(&JSON_HYPER_SCHEMA_07_ABSOLUTE_URI)
}

#[must_use]
pub fn dialect() -> &'static Dialect {
    Lazy::force(&JSON_SCHEMA_07_DIALECT)
}

pub fn is_json_hyper_schema_07_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_07_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}
pub fn is_json_hyper_schema_07_absolute_uri(uri: &AbsoluteUri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_HYPER_SCHEMA_07_URL.domain()
                && u.path() == JSON_HYPER_SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_07_uri(uri: &Uri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_07_URL.domain()
                && u.path() == JSON_SCHEMA_07_URL.path()
                && u.fragment().unwrap_or_default() == ""
        }
    } else {
        false
    }
}

pub fn is_json_schema_07_absolute_uri(uri: &AbsoluteUri) -> bool {
    if let Some(u) = uri.as_url() {
        if u.scheme() != "http" && u.scheme() != "https" {
            false
        } else {
            u.domain() == JSON_SCHEMA_07_URL.domain()
                && u.path() == JSON_SCHEMA_07_URL.path()
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
