use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    ///
    /// - [JSON Schema Core 2020-12 #12.3.1. Keyword Relative Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-relative-location)
    #[serde(rename = "keywordLocation")]
    pub keyword_location: jsonptr::Pointer,

    /// The absolute, dereferenced location of the validating keyword. The value
    /// MUST be expressed as a full URI using the canonical URI of the relevant
    /// schema resource with a JSON Pointer fragment, and it MUST NOT include
    /// by-reference applicators such as `"$ref"` or `"$dynamicRef"` as
    /// non-terminal path components. It MAY end in such keywords if the error
    /// or annotation is for that keyword, such as an unresolvable reference.
    /// Note that "absolute" here is in the sense of "absolute filesystem path"
    /// (meaning the complete location) rather than the `"absolute-URI"
    /// terminology from RFC 3986 (meaning with scheme but without fragment).
    /// Keyword absolute locations will have a fragment in order to identify the
    /// keyword.
    ///
    /// # Example
    /// ```plaintext
    /// https://example.com/schemas/common#/$defs/count/minimum
    /// ```
    /// This information MAY be omitted only if either the dynamic scope did not
    /// pass over a reference or if the schema does not declare an absolute URI
    /// as its "$id".
    ///
    /// - [12.3.2. Keyword Absolute
    ///   Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-absolute-location)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_keyword_location: Option<String>,

    /// The location of the JSON value within the instance being validated. The
    /// value MUST be expressed as a JSON Pointer.
    ///
    /// - [JSON Schema Core 2020-12 # 12.3.3. Instance Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-instance-location)
    #[serde(rename = "instanceLocation")]
    pub instance_location: jsonptr::Pointer,
}
