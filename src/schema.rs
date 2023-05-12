use std::{borrow::Cow, fmt::Display, str::FromStr};

use crate::{draft::is_schema_04_uri, Uri};
use heck::AsLowerCamelCase;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::Types;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaObject {
    /// The value of `$id` is a URI-reference without a fragment that resolves
    /// against the Retrieval URI. The resulting URI is the base URI for the
    /// schema.
    ///
    /// Note: In JSON Schema Draft 4, field was `id` rather than `$id`.
    ///
    /// -
    /// - [Understanding JSON Schema](https://json-schema.org/understanding-json-schema/structuring.html?highlight=id#id)
    #[serde(
        rename = "$id",
        alias = "id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<Uri>,

    /// The `$schema` keyword is both used as a JSON Schema dialect identifier
    /// and as the identifier of a resource which is itself a JSON Schema, which
    /// describes the set of valid schemas written for this particular dialect.
    ///
    /// - [Draft 2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)
    /// - [Draft 2019-09](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.1.1)
    /// - [Draft 7](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-01#section-7)
    /// - [Draft 4](https://datatracker.ietf.org/doc/html/draft-zyp-json-schema-04#section-6)
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<Uri>,

    /// Using JSON Pointer fragments requires knowledge of the structure of the
    /// schema. When writing schema documents with the intention to provide
    /// re-usable schemas, it may be preferable to use a plain name fragment
    /// that is not tied to any particular structural location. This allows a
    /// subschema to be relocated without requiring JSON Pointer references to
    /// be updated.
    ///
    /// The "$anchor" keyword is used to specify such a fragment. It is an
    /// identifier keyword that can only be used to create plain name fragments.
    ///
    /// Anchors must start with a letter followed by any number of letters,
    /// digits, `-`, `_`, `:`, or `.`.
    /// - [Draft 2020-12 - Defining location-independent identifiers](https://json-schema.org/draft/2020-12/json-schema-core.html#name-defining-location-independe)
    /// - [Draft 2019-09 - Defining location-independent identifiers with "$anchor"](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.2.3)
    /// - [Understanding JSON Schema](https://json-schema.org/understanding-json-schema/structuring.html?highlight=anchor#anchor)
    #[serde(rename = "$anchor", default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,

    /// - <https://json-schema.org/draft/2020-12/release-notes.html#dynamicref-and-dynamicanchor>
    #[serde(
        rename = "$dynamicAnchor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_anchor: Option<String>,

    types: Types,
    // #[serde(
    //     rename = "$vocabulary",
    //     default,
    //     skip_serializing_if = "Option::is_none"
    // )]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Bool(bool),
    Object(Box<SchemaObject>),
}

// impl RawSchema {
//     pub fn as_bool(&self) -> Option<bool> {
//         match self {
//             RawSchema::Bool(b) => Some(*b),
//             _ => None,
//         }
//     }
//     pub fn as_object(&self) -> Option<&Map<String, Value>> {
//         match self {
//             RawSchema::Object(o) => Some(o),
//             _ => None,
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Schema {
//     id: Uri,
//     meta_schema_id: Uri,
//     properties: Map<String, Value>,
//     always: Option<bool>,
// }

// impl Schema {
//     pub fn is_bool(&self) -> bool {
//         self.always.is_some()
//     }

//     pub fn as_bool(&self) -> Option<bool> {
//         self.always
//     }

//     pub fn property(&self, name: &str) -> Option<Cow<'_, Value>> {
//         if let Some(always) = self.always {
//             if !always {
//                 return None;
//             }
//         }
//         self.properties.get(name).map(Cow::Borrowed)
//     }
// }

// impl<'de> Deserialize<'de> for Schema {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         todo!()
//     }
// }
