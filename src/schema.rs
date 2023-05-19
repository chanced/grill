mod bool_or_number;
mod discriminator;
mod format;
mod items;
mod object;
mod subschema;
mod types;

pub use bool_or_number::BoolOrNumber;
pub use discriminator::Discriminator;
pub use format::Format;
pub use items::Items;
pub use object::Object;
use serde::{Deserialize, Serialize};
pub use subschema::{CompiledSubschema, Subschema};
pub use types::{Type, Types};

use crate::{output::Annotation, Handler, Output};

/// A JSON Schema document.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Bool(bool),
    Object(Object),
}
impl Default for Schema {
    fn default() -> Self {
        Schema::Bool(true)
    }
}

#[derive(Debug, Clone)]
pub struct CompiledSchema {
    /// The absolute, dereferenced  location of the validating keyword. The
    /// value MUST be expressed as a full URI using the canonical URI of the
    /// relevant schema resource with a JSON Pointer fragment, and it MUST NOT
    /// include by-reference applicators such as `"$ref"` or `"$dynamicRef"` as
    /// non-terminal path components. It MAY end in such keywords if the error
    /// or annotation is for that keyword, such as an unresolvable reference.
    ///
    /// Note that "absolute" here is in the sense of "absolute filesystem path"
    /// (meaning the complete location) rather than the `"absolute-URI"
    /// terminology from RFC 3986 (meaning with scheme but without fragment).
    /// Keyword absolute locations will have a fragment in order to identify the
    /// keyword.
    pub absolute_keyword_location: Option<String>,

    /// The relative location of the validating keyword that follows the
    /// validation path. The value MUST be expressed as a JSON Pointer, and it
    /// MUST include any by-reference applicators such as `"$ref"` or
    /// `"$dynamicRef"`.
    ///
    /// # Example
    /// ```plaintext
    /// /properties/width/$ref/minimum
    /// ```
    ///
    /// Note that this pointer may not be resolvable by the normal JSON Pointer
    /// process due to the inclusion of these by-reference applicator keywords.
    ///
    /// The JSON key for this information is `"keywordLocation"`.
    pub keyword_location: jsonptr::Pointer,

    field_handlers: Vec<(String, Handler)>,
}

impl CompiledSchema {
    #[allow(clippy::missing_panics_doc)]
    /// # Errors
    pub async fn evaluate(
        &self,
        value: &serde_json::Value,
        output_structure: crate::output::Structure,
    ) -> Result<Output, Box<dyn std::error::Error>> {
    }

    async fn evaluate_internal(
        &self,
        scope: &crate::Scope,
        value: &serde_json::Value,
        output_structure: crate::output::Structure,
    ) -> Result<Option<Annotation>, Box<dyn std::error::Error>> {
        // for (field, handler) in self.field_handlers {
        //     let scope = scope.nested(field);
        // }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_serde() {
        let schema = json!(
            {
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$id": "https://example.com/product.schema.json",
                    "title": "Product",
                    "description": "A product in the catalog",
                    "type": "object"
            }
        );
        let obj: Object = serde_json::from_str("{}").unwrap();
        let obj: Object = serde_json::from_value(schema.clone()).unwrap();

        let schema: Schema = serde_json::from_value(schema).unwrap();
    }
}
