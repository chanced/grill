use std::println;

use crate::{Schema, Uri};
use once_cell::sync::Lazy;

/// Returns [`&'static Uri`](`Uri`) for Schema Draft 2020-12.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn schema_2020_12_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_2020_12_URI).unwrap()
}

/// Returns the [`Uri`] of Hyper Schema Draft 2020-12.
///
/// # Returns
/// - A [`Uri`] equal to `"https://json-schema.org/draft/2020-12/hyper-schema"`
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn hyper_schema_2020_12_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_2020_12_URI).unwrap()
}

/// Returns `true` if the given [`Uri`] equals Schema Draft 2020-12.
///
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` equals `"https://json-schema.org/draft/2020-12/schema"`
/// - `false` otherwise
#[must_use]
pub fn is_schema_2020_12_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == schema_2020_12_uri()
}

/// Returns `true` if the given [`Uri`] equals Hyper Schema draft
/// 2020-12.
///
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` equals `"https://json-schema.org/draft/2020-12/hyper-schema`
#[must_use]
pub fn is_hyper_schema_2020_12_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == hyper_schema_2020_12_uri()
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
/// Returns the [`Uri`] of Schema Draft 2019-09.
///
/// # Returns
/// - [`Uri`] equal to `"https://json-schema.org/draft/2019-09/schema"`
pub fn schema_2019_09_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_2019_09_URI).unwrap()
}

/// Returns the [`Uri`] of Hyper Schema Draft 2019-09.
///
/// # Returns
/// - [`Uri`]` equal to "https://json-schema.org/draft/2019-09/hyper-schema"`.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn hyper_schema_2019_09_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_2019_09_URI).unwrap()
}

/// Returns the [`Uri`] of Schema Draft 07.
///
/// A [`Uri`] equal to `http://json-schema.org/draft-07/schema#` or
/// `https://json-schema.org/draft-07/schema#`.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn schema_07_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_07_URI).unwrap()
}

/// Returns the [`Uri`] of Schema Draft 04.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn schema_04_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_04_URI).unwrap()
}

/// Returns the [`Uri`] of Hyper Schema Draft 04.
///
/// # Returns
/// - A [`Uri`] equal to `"http://json-schema.org/draft-04/hyper-schema#"`
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn hyper_schema_04_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_04_URI).unwrap()
}

/// Returns `true` if the given [`Uri`] equals Schema Draft 04.
///
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` is equal to `"http://json-schema.org/draft-04/schema#"`
/// - `true` if the [`Uri`] `meta_schema_id` is equal to `"https://json-schema.org/draft-04/schema"`
/// - `false` otherwise
#[must_use]
pub fn is_schema_04_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == schema_04_uri()
}

/// Returns `true` if the given [`Uri`] equals Hyper Schema Draft 04.
///
/// # Returns
/// - `true` if [`Uri`] `meta_schema_id` equals `"http://json-schema.org/draft-04/hyper-schema#"`
/// - `true` if [`Uri`] `meta_schema_id` equals `"https://json-schema.org/draft-04/hyper-schema"`
/// - `false` otherwise
#[must_use]
pub fn is_hyper_schema_04_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == hyper_schema_04_uri()
}
// #[must_use]
// #[allow(clippy::missing_panics_doc)]
// /// Returns a slice of the JSON Schema Draft 04 meta-schemas.
// ///
// /// # Returns
// /// - `&[Value]` containing the JSON Schema Draft 04 meta-schemas
// pub fn schema_04() -> &'static [Value] {
//     Lazy::get(&SCHEMA_04).unwrap()
// }
// /// Returns a slice of the JSON Hyper Schema Draft 04 meta-schemas.
// ///
// /// # Returns
// /// - `&[Value]` containing the JSON Hyper Schema Draft 04 meta-schemas
// #[must_use]
// #[allow(clippy::missing_panics_doc)]
// pub fn hyper_schema_04() -> &'static [Value] {
//     Lazy::get(&HYPER_SCHEMA_04).unwrap()
// }

/// Returns `true` if the given [`Uri`] equals Schema Draft 07.
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` equals `"http://json-schema.org/draft-07/schema#"`
/// - `false` Otherwise
#[must_use]
pub fn is_schema_07_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == schema_07_uri()
}

/// Returns the [`Uri`] of Hyper Schema Draft 07
///
/// # Returns
/// A [`Uri`] equal to `"http://json-schema.org/draft-07/hyper-schema#"`.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn hyper_schema_07_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_07_URI).unwrap()
}

/// [`Uri`] of Hyper Schema Draft 07.
///
/// # Returns
/// - A [`Uri`] equal to `"http://json-schema.org/draft-07/hyper-schema#"`
static HYPER_SCHEMA_07_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("http://json-schema.org/draft-07/hyper-schema#").unwrap());

/// Returns `true` if the given [`Uri`] equals Hyper Schema Draft 07.
///
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` equals `"http://json-schema.org/draft-07/hyper-schema#"`
/// - `true` if the [`Uri`] `meta_schema_id` equals `"https://json-schema.org/draft-07/hyper-schema#"`
/// - `false` Otherwise
#[must_use]
pub fn is_hyper_schema_07_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == hyper_schema_07_uri()
}

// /// Returns Meta Schemas for Schema Draft 07.
// /// # Returns
// /// - `&[Value]`, containing each meta-schema in Schema Draft 07.
// #[must_use]
// #[allow(clippy::missing_panics_doc)]
// pub fn schema_07() -> &'static [Value] {
//     Lazy::get(&SCHEMA_07).unwrap()
// }

// /// Returns Meta Schemas for Hyper Schema Draft 07.
// ///
// /// # Returns
// /// - `&[Value]`, containing each meta-schema in Hyper Schema Draft 07.
// #[must_use]
// #[allow(clippy::missing_panics_doc)]
// pub fn hyper_schema_07() -> &'static [Value] {
//     Lazy::get(&HYPER_SCHEMA_07).unwrap()
// }

/// [`Uri`] of Schema Draft 2020-12: <https://json-schema.org/draft/2020-12/schema>
static SCHEMA_2020_12_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2020-12/schema").unwrap());

/// [`Uri`] of Hyper Schema Draft 2020-12.
///
/// <https://json-schema.org/draft/2020-12/hyper-schema>
static HYPER_SCHEMA_2020_12_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2020-12/hyper-schema").unwrap());

/// [`Uri`] of Schema Draft 07.
/// # Returns
/// - A [`Uri`] equal to `"http://json-schema.org/draft-07/schema#"`
static SCHEMA_07_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("http://json-schema.org/draft-07/schema#").unwrap());

/// [`Uri`] of Schema Draft 04.
static SCHEMA_04_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("http://json-schema.org/draft-04/schema#").unwrap());

/// [`Uri`] of Hyper Schema Draft 04.
static HYPER_SCHEMA_04_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("http://json-schema.org/draft-04/hyper-schema#").unwrap());

/// [`Uri`] of Schema Draft 2019-09.
static SCHEMA_2019_09_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2019-09/schema").unwrap());

/// [`Uri`] of Hyper Schema Draft 2019-09.
static HYPER_SCHEMA_2019_09_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2019-09/hyper-schema").unwrap());

static SCHEMA_04: Lazy<Vec<Schema>> =
    Lazy::new(|| vec![serde_json::from_slice(include_bytes!("../spec/04/schema.json")).unwrap()]);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn schema_04() -> &'static [Schema] {
    Lazy::force(&SCHEMA_04);
    Lazy::get(&SCHEMA_04).unwrap()
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::*;

    #[test]
    fn test_name() {
        println!("{}", serde_json::to_string_pretty(schema_04()).unwrap());
    }
}
