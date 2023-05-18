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

use crate::{output::Annotation, Output};

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
pub struct CompiledSchema {}
impl CompiledSchema {
    #[allow(clippy::missing_panics_doc)]
    /// # Errors
    pub async fn evaluate(
        &self,
        value: &serde_json::Value,
        output_structure: crate::output::Structure,
    ) -> Result<Output, Box<dyn std::error::Error>> {
        todo!()
    }
    async fn evaluate_internal(
        &self,
        scope: &crate::Scope,
        value: &serde_json::Value,
        output_structure: crate::output::Structure,
    ) -> Result<Option<Annotation>, Box<dyn std::error::Error>> {
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
