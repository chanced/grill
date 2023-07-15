pub use super::draft_2019_09::identify_schema;

use super::{
    ident_schema_location_by_anchor, ident_schema_location_by_dynamic_anchor,
    identify_schema_location_by_id, identify_schema_location_by_path, locate_schemas_in_array,
};
use crate::{
    dialect::{Dialect, Dialects, LocatedSchema},
    error::LocateSchemasError,
    keyword::Keyword,
    uri::AbsoluteUri,
    Metaschema, Uri,
};
use jsonptr::Pointer;
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

pub static JSON_SCHEMA_2020_12_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        JSON_SCHEMA_2020_12_ABSOLUTE_URI.clone(),
        serde_json::from_slice(JSON_SCHEMA_2020_12_BYTES).unwrap(),
    )
});
pub static JSON_SCHEMA_2020_12_CONTENT_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        AbsoluteUri::parse("https://json-schema.org/draft/2020-12/meta/content").unwrap(),
        serde_json::from_slice(JSON_SCHEMA_2020_12_CONTENT_BYTES).unwrap(),
    )
});

pub static JSON_SCHEMA_2020_12_CORE_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        AbsoluteUri::parse("https://json-schema.org/draft/2020-12/meta/core").unwrap(),
        serde_json::from_slice(JSON_SCHEMA_2020_12_CORE_BYTES).unwrap(),
    )
});

pub static JSON_SCHEMA_2020_12_FORMAT_ANNOTATION_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        AbsoluteUri::parse("https://json-schema.org/draft/2020-12/meta/format-annotation").unwrap(),
        serde_json::from_slice(JSON_SCHEMA_2020_12_FORMAT_ANNOTATION_BYTES).unwrap(),
    )
});

pub static JSON_SCHEMA_2020_12_FORMAT_ASSERTION_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        AbsoluteUri::parse("https://json-schema.org/draft/2020-12/meta/format-assertion").unwrap(),
        serde_json::from_slice(JSON_SCHEMA_2020_12_FORMAT_ASSERTION_BYTES).unwrap(),
    )
});

pub static JSON_SCHEMA_2020_12_META_DATA_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        AbsoluteUri::parse("https://json-schema.org/draft/2020-12/meta/meta-data").unwrap(),
        serde_json::from_slice(JSON_SCHEMA_2020_12_META_DATA_BYTES).unwrap(),
    )
});

pub static JSON_SCHEMA_2020_12_UNEVALUATED_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        AbsoluteUri::parse("https://json-schema.org/draft/2020-12/meta/unevaluated").unwrap(),
        serde_json::from_slice(JSON_SCHEMA_2020_12_UNEVALUATED_BYTES).unwrap(),
    )
});

pub static JSON_SCHEMA_2020_12_VALIDATION_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        AbsoluteUri::parse("https://json-schema.org/draft/2020-12/meta/validation").unwrap(),
        serde_json::from_slice(JSON_SCHEMA_2020_12_VALIDATION_BYTES).unwrap(),
    )
});

pub static JSON_SCHEMA_2020_12_APPLICATOR_METASCHEMA: Lazy<Metaschema> = Lazy::new(|| {
    Metaschema::new(
        AbsoluteUri::parse("https://json-schema.org/draft/2020-12/meta/applicator").unwrap(),
        serde_json::from_slice(JSON_SCHEMA_2020_12_APPLICATOR_BYTES).unwrap(),
    )
});

pub static JSON_SCHEMA_2020_12_DIALECT: Lazy<Dialect> = Lazy::new(|| {
    Dialect::new(
        json_schema_2020_12_absolute_uri().clone(),
        [
            Lazy::force(&JSON_SCHEMA_2020_12_METASCHEMA),
            Lazy::force(&JSON_SCHEMA_2020_12_CORE_METASCHEMA),
            Lazy::force(&JSON_SCHEMA_2020_12_FORMAT_ANNOTATION_METASCHEMA),
            Lazy::force(&JSON_SCHEMA_2020_12_FORMAT_ASSERTION_METASCHEMA),
            Lazy::force(&JSON_SCHEMA_2020_12_META_DATA_METASCHEMA),
            Lazy::force(&JSON_SCHEMA_2020_12_UNEVALUATED_METASCHEMA),
            Lazy::force(&JSON_SCHEMA_2020_12_VALIDATION_METASCHEMA),
            Lazy::force(&JSON_SCHEMA_2020_12_APPLICATOR_METASCHEMA),
        ],
        [super::draft_07::ConstHandler::new()], // TOOD: FIX
        is_json_schema_2020_12,
        identify_schema,
        locate_schemas,
    )
});

#[must_use]
pub fn is_json_schema_2020_12(v: &Value) -> bool {
    // bools are handled the same way across json schema dialects
    // so there's no need to cycle through the remaining schemas
    // just to ultimately end up with a default dialect
    if v.is_boolean() {
        return true;
    }

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
pub fn is_json_hyper_schema_2020_12_absolute_uri(uri: &AbsoluteUri) -> bool {
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

pub fn is_json_schema_2020_12_absolute_uri(uri: &AbsoluteUri) -> bool {
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
/// An implementation of [`LocateSchemas`](`crate::dialect::LocateSchemas`)
/// which recursively traverses a [`Value`] and returns a [`Vec`] of
/// [`LocatedSchema`]s for each identified (via `$id`) subschema and for each
/// schema with an`"$anchor"`.
///
pub fn locate_schemas<'v>(
    ptr: Pointer,
    value: &'v Value,
    dialects: Dialects,
    base_uri: &AbsoluteUri,
) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
    match value {
        Value::Array(arr) => locate_schemas_in_array(ptr, arr, dialects, base_uri),
        Value::Object(_) => locate_schemas_in_obj(ptr, value, dialects, base_uri),
        _ => Ok(Vec::new()),
    }
}

fn locate_schemas_in_obj<'v>(
    mut path: Pointer,
    value: &'v Value,
    mut dialects: Dialects,
    base_uri: &AbsoluteUri,
) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
    let mut results: Vec<LocatedSchema> = Vec::new();
    let default_dialect = dialects.default_dialect();
    let dialect_idx = dialects.dialect_index_for(value);
    dialects.set_default_dialect_index(dialect_idx);
    let dialect = dialects
        .get(dialect_idx)
        .expect("dialect index out of bounds");

    if default_dialect != dialect {
        return dialect.locate_schemas(path, value, dialects, base_uri);
    }
    if path.is_empty() || default_dialect.is_schema_property(&path, value) {
        results.push(identify_schema_location_by_path(&path, value, base_uri));
    }

    let mut base_uri = base_uri.clone();

    if let Some(located) =
        identify_schema_location_by_id(&path, value, &mut base_uri, &mut dialects)?
    {
        path = Pointer::default();
        results.push(located);
    }

    if let Some(anchored) = ident_schema_location_by_anchor(&path, value, &base_uri) {
        results.push(anchored);
    }

    if let Some(anchored) = ident_schema_location_by_dynamic_anchor(&path, value, &base_uri) {
        results.push(anchored);
    }

    for (key, value) in value.as_object().unwrap().iter() {
        if !dialects
            .get(dialects.dialect_index_for(value))
            .expect("dialect index out of bounds")
            .can_keyword_contain_schemas(Keyword(key))
        {
            let mut path = path.clone();
            path.push_back(key.into());
            if !dialect.is_schema_property(&path, value) {
                continue;
            }
        }
        let mut new_path = path.clone();
        new_path.push_back(key.into());
        let mut located_schemas = dialect.locate_schemas(new_path, value, dialects, &base_uri)?;
        results.append(&mut located_schemas);
    }
    Ok(results)
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
