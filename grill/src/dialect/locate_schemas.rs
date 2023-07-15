use std::collections::HashMap;

use crate::{error::LocateSchemasError, keyword::Keyword, AbsoluteUri};
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;

use super::Dialects;

/// Locates identified or anchored schemas within a [`Value`].
///
/// Not all URIs may be identified for each schema. For example, the schema:
/// ```json
/// {
///     "$schema": "https://json-schema.org/draft/2020-12/schema",
///     "$id": "https://example.com/example.json",
///     "$defs": {
///         "foo": {
///             "$id": "https://example.com/foo.json",
///             "$defs": {
///                 "bar": {
///                     "$id": "https://example.com/bar.json",
///                     "$defs": {
///                         "$anchor": : "anchor",
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
/// may only identify following URIs:
/// - `https://example.com/example.json`
/// - `https://example.com/foo.json`
/// - `https://example.com/bar.json`
/// - `https://example.com/bar.json#anchor`
///
/// The remainder are resolved upon use.
///
/// # Examples
/// ## [JSON Schema 2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html#name-schema-identification-examp)
///
/// ```json
/// {
///     "$id": "https://example.com/root.json",
///     "$defs": {
///         "A": { "$anchor": "foo" },
///         "B": {
///             "$id": "other.json",
///             "$defs": {
///                 "X": { "$anchor": "bar" },
///                 "Y": {
///                     "$id": "t/inner.json",
///                     "$anchor": "bar"
///                 }
///             }
///         },
///         "C": {
///             "$id": "urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f"
///         }
///     }
/// }
/// ```
///
/// The schemas at the following URI-encoded JSON Pointers (relative to the root
/// schema) have the following base URIs, and are identifiable by any listed
/// URI:
///
/// - `#` (document root) canonical (and base) URI
///   `https://example.com/root.json`
///     - canonical resource URI plus pointer fragment
///       `https://example.com/root.json#`
/// - `#/$defs/A` base URI `https://example.com/root.json`
///     - canonical resource URI plus plain fragment
///       `https://example.com/root.json#foo`
///     - canonical resource URI plus pointer fragment
///       `https://example.com/root.json#/$defs/A`
/// - `#/$defs/B` canonical (and base) URI `https://example.com/other.json`
///     - canonical resource URI plus pointer fragment
///       `https://example.com/other.json#`
///     - base URI of enclosing (root.json) resource plus fragment
///       `https://example.com/root.json#/$defs/B`
/// - `#/$defs/B/$defs/X` base URI `https://example.com/other.json`
///     - canonical resource URI plus plain fragment
///       `https://example.com/other.json#bar`
///     - canonical resource URI plus pointer fragment
///       `https://example.com/other.json#/$defs/X`
///     - base URI of enclosing (root.json) resource plus fragment
///       `https://example.com/root.json#/$defs/B/$defs/X`
/// - `#/$defs/B/$defs/Y` canonical (and base) URI
///   `https://example.com/t/inner.json`
///     - canonical URI plus plain fragment
///       `https://example.com/t/inner.json#bar`
///     - canonical URI plus pointer fragment
///       `https://example.com/t/inner.json#`
///     - base URI of enclosing (other.json) resource plus fragment
///       `https://example.com/other.json#/$defs/Y`
///     - base URI of enclosing (root.json) resource plus fragment
///       `https://example.com/root.json#/$defs/B/$defs/Y`
/// - `#/$defs/C`
///     - canonical (and base) URI
///       `urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f`
///     - canonical URI plus pointer fragment
///       `urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f#`
///     - base URI of enclosing (root.json) resource plus fragment
///       `https://example.com/root.json#/$defs/C`
///
/// ## [JSON Schema 07](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-01#section-8.2.4)
/// ```json
/// {
///     "$id": "http://example.com/root.json",
///     "definitions": {
///         "A": { "$id": "#foo" },
///         "B": {
///             "$id": "other.json",
///             "definitions": {
///                 "X": { "$id": "#bar" },
///                 "Y": { "$id": "t/inner.json" }
///             }
///         },
///         "C": {
///             "$id": "urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f"
///         }
///     }
/// }
/// ```
/// -   `#` (document root)
///     -   `http://example.com/root.json`
///     -   `http://example.com/root.json#`
/// -   `#/definitions/A`
///     -   `http://example.com/root.json#foo`
///     -   `http://example.com/root.json#/definitions/A`
/// -   `#/definitions/B`
///     -   `http://example.com/other.json`
///         -   `http://example.com/other.json#`
///     -   `http://example.com/root.json#/definitions/B`
/// -   `#/definitions/B/definitions/X`
///     -   `http://example.com/other.json#bar`
///         -   `http://example.com/other.json#/definitions/X`
///         -   `http://example.com/root.json#/definitions/B/definitions/X`
/// -   `#/definitions/B/definitions/Y`
///     -   `http://example.com/t/inner.json`
///     -   `http://example.com/t/inner.json#`
///     -   `http://example.com/other.json#/definitions/Y`
///     -   `http://example.com/root.json#/definitions/B/definitions/Y`
/// -   `#/definitions/C`
///     -   `urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f`
///     -   `urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f#`
///     -   `http://example.com/root.json#/definitions/C`
///
pub trait LocateSchemas: Send + Sync + DynClone {
    fn locate_schemas<'v>(
        &self,
        path: Pointer,
        value: &'v Value,
        dialects: Dialects,
        base_uri: &AbsoluteUri,
    ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError>;
}
clone_trait_object!(LocateSchemas);

impl<F> LocateSchemas for F
where
    F: Send
        + 'static
        + Sync
        + Clone
        + for<'v, 'd> Fn(
            Pointer,
            &'v Value,
            Dialects<'d>,
            &AbsoluteUri,
        ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError>,
{
    fn locate_schemas<'v>(
        &self,
        path: Pointer,
        value: &'v Value,
        dialects: Dialects,
        base_uri: &AbsoluteUri,
    ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
        (self)(path, value, dialects, base_uri)
    }
}

#[derive(Debug, Clone)]
pub struct LocatedSchema<'v> {
    /// The Uri of the Schema
    pub uri: AbsoluteUri,
    pub value: &'v Value,
    pub path: Pointer,
    pub keyword: Option<Keyword<'static>>,
}

impl<'v> LocatedSchema<'v> {
    #[must_use]
    pub fn new(
        uri: AbsoluteUri,
        value: &'v Value,
        path: Pointer,
        keyword: Option<Keyword<'static>>,
    ) -> Self {
        Self {
            uri,
            value,
            path,
            keyword,
        }
    }
}

pub(crate) struct LocatedSchemas<'v> {
    pub(crate) schemas: HashMap<Pointer, Vec<LocatedSchema<'v>>>,
}
impl<'v> From<Vec<LocatedSchema<'v>>> for LocatedSchemas<'v> {
    fn from(schemas: Vec<LocatedSchema<'v>>) -> Self {
        let mut this = Self {
            schemas: HashMap::default(),
        };
        for schema in schemas {
            this.schemas
                .entry(schema.path.clone())
                .or_default()
                .push(schema);
        }
        this
    }
}
