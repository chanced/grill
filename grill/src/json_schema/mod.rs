//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.

pub mod draft_04;
pub mod draft_07;
pub mod draft_2019_09;
pub mod draft_2020_12;

// pub use draft_04::{
//     is_json_hyper_schema_04_absolute_uri, is_json_hyper_schema_04_uri, is_json_schema_04,
//     is_json_schema_04_absolute_uri, is_json_schema_04_uri, JSON_HYPER_SCHEMA_04_ABSOLUTE_URI,
//     JSON_HYPER_SCHEMA_04_BYTES, JSON_HYPER_SCHEMA_04_LINKS_BYTES, JSON_HYPER_SCHEMA_04_URI,
//     JSON_HYPER_SCHEMA_04_URI_STR, JSON_HYPER_SCHEMA_04_URL, JSON_SCHEMA_04,
//     JSON_SCHEMA_04_ABSOLUTE_URI, JSON_SCHEMA_04_BYTES, JSON_SCHEMA_04_URI, JSON_SCHEMA_04_URI_STR,
//     JSON_SCHEMA_04_URL, JSON_SCHEMA_04_VALUE,
// };

// pub use draft_07::{
//     is_json_hyper_schema_07_absolute_uri, is_json_hyper_schema_07_uri, is_json_schema_07,
//     is_json_schema_07_absolute_uri, is_json_schema_07_uri, JSON_HYPER_SCHEMA_07_ABSOLUTE_URI,
//     JSON_HYPER_SCHEMA_07_BYTES, JSON_HYPER_SCHEMA_07_LINKS_BYTES,
//     JSON_HYPER_SCHEMA_07_OUTPUT_BYTES, JSON_HYPER_SCHEMA_07_URI, JSON_HYPER_SCHEMA_07_URI_STR,
//     JSON_HYPER_SCHEMA_07_URL, JSON_SCHEMA_07, JSON_SCHEMA_07_ABSOLUTE_URI, JSON_SCHEMA_07_BYTES,
//     JSON_SCHEMA_07_URI, JSON_SCHEMA_07_URI_STR, JSON_SCHEMA_07_URL, JSON_SCHEMA_07_VALUE,
// };
// pub use draft_2019_09::{
//     is_json_hyper_schema_2019_09_absolute_uri, is_json_schema_2019_09,
//     is_json_schema_2019_09_absolute_uri, is_json_schema_2019_09_uri,
//     JSON_HYPER_SCHEMA_2019_09_ABSOLUTE_URI, JSON_HYPER_SCHEMA_2019_09_BYTES,
//     JSON_HYPER_SCHEMA_2019_09_LINKS_BYTES, JSON_HYPER_SCHEMA_2019_09_LINKS_METASCHEMA,
//     JSON_HYPER_SCHEMA_2019_09_METASCHEMA, JSON_HYPER_SCHEMA_2019_09_OUTPUT_BYTES,
//     JSON_HYPER_SCHEMA_2019_09_OUTPUT_VALUE, JSON_HYPER_SCHEMA_2019_09_URI,
//     JSON_HYPER_SCHEMA_2019_09_URI_STR, JSON_HYPER_SCHEMA_2019_09_URL, JSON_SCHEMA_2019_09,
//     JSON_SCHEMA_2019_09_ABSOLUTE_URI, JSON_SCHEMA_2019_09_APPLICATOR_BYTES,
//     JSON_SCHEMA_2019_09_APPLICATOR_METASCHEMA, JSON_SCHEMA_2019_09_BYTES,
//     JSON_SCHEMA_2019_09_CONTENT_BYTES, JSON_SCHEMA_2019_09_CONTENT_METASCHEMA,
//     JSON_SCHEMA_2019_09_CORE_BYTES, JSON_SCHEMA_2019_09_CORE_METASCHEMA,
//     JSON_SCHEMA_2019_09_FORMAT_BYTES, JSON_SCHEMA_2019_09_FORMAT_METASCHEMA,
//     JSON_SCHEMA_2019_09_METASCHEMA, JSON_SCHEMA_2019_09_META_DATA_BYTES,
//     JSON_SCHEMA_2019_09_META_DATA_METASCHEMA, JSON_SCHEMA_2019_09_OUTPUT_BYTES,
//     JSON_SCHEMA_2019_09_OUTPUT_VALUE, JSON_SCHEMA_2019_09_URI, JSON_SCHEMA_2019_09_URI_STR,
//     JSON_SCHEMA_2019_09_URL, JSON_SCHEMA_2019_09_VALIDATION_BYTES,
//     JSON_SCHEMA_2019_09_VALIDATION_METASCHEMA,
// };

// pub use draft_2020_12::{
//     is_json_hyper_schema_2020_12_absolute_uri, is_json_hyper_schema_2020_12_uri,
//     is_json_schema_2020_12, is_json_schema_2020_12_absolute_uri, is_json_schema_2020_12_uri,
// };

use crate::uri::{Url, AsUriRef};

fn is_uri_for<'a>(target: &Url, other: impl AsUriRef) -> bool {
    let Some(u) = other.as_uri_ref().as_url()  else { return false };
    let scheme = u.scheme();
    (scheme == "https" || scheme == "http")
        && u.domain() == target.domain()
        && u.path() == target.path()
        && u.fragment().unwrap_or_default().is_empty()
}

#[cfg(test)]
mod tests {}
