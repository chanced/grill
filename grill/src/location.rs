use jsonptr::Pointer;
use serde::{Deserialize, Serialize};

use crate::{AbsoluteUri, Uri};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    /// The relative location of the validating keyword that follows the validation
    /// path. The value MUST be expressed as a JSON Pointer, and it MUST include any
    /// by-reference applicators such as "$ref" or "$dynamicRef".
    /// ```plaintext
    /// properties/width/$ref/minimum
    /// ```
    ///
    /// Note that this pointer may not be resolvable by the normal JSON Pointer
    /// process due to the inclusion of these by-reference applicator keywords.
    ///
    /// The JSON key for this information is "keywordLocation".
    ///
    /// - [JSON Schema Core 2020-12 #12.3.1. Keyword Relative Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-relative-location)
    #[serde(rename = "keywordLocation")]
    keyword_location: Pointer,

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
    /// - [JSON Schema Core 2020-12 # 12.3.2. Keyword Absolute Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-absolute-location)
    #[serde(rename = "absoluteKeywordLocation")]
    absolute_keyword_location: (Uri, Pointer),

    /// The location of the JSON value within the instance being validated. The
    /// value MUST be expressed as a JSON Pointer.
    ///
    /// - [JSON Schema Core 2020-12 # 12.3.3. Instance Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-instance-location)
    #[serde(rename = "instanceLocation")]
    instance_location: Pointer,
}

impl Location {
    pub fn absolute_keyword_location(&self) -> Uri {
        let (uri, ptr) = &self.absolute_keyword_location;
        let mut uri = uri.clone();
        uri.set_fragment(Some(&ptr.to_string()));
        uri
    }

    pub fn push_keyword_location(&mut self, keyword: &str) {
        let tok: jsonptr::Token = keyword.into();
        self.keyword_location.push_back(tok.clone());
        let (_, mut ptr) = &mut self.absolute_keyword_location;
        ptr.push_back(tok);
    }

    pub fn push_instance_location(&mut self, instance: &str) {
        let tok: jsonptr::Token = instance.into();
        self.instance_location.push_back(tok);
    }

    pub fn keyword_location(&self) -> &Pointer {
        &self.keyword_location
    }

    pub fn instance_location(&self) -> &Pointer {
        &self.instance_location
    }
}
