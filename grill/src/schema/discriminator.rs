use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// When request bodies or response payloads may be one of a number of different
/// schemas, a discriminator object can be used to aid in serialization,
/// deserialization, and validation. The discriminator is a specific object in a
/// schema which is used to inform the consumer of the document of an
/// alternative schema based on the value associated with it.
///
/// - [OpenAPI 3.1 Specification # 4.8.25 Discriminator Object](https://spec.openapis.org/oas/v3.1.0#discriminator-object)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    /// The name of the property in the payload that will hold the discriminator
    /// value.
    property_name: String,
    /// An object to hold mappings between payload values and schema names or
    /// references.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    mapping: Option<HashMap<String, String>>,
    /// Additional keywords
    #[serde(flatten, default)]
    additional_keywords: HashMap<String, Value>,
}
