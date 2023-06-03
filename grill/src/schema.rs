mod anchor;
mod bool_or_number;
mod compiled_schema;
mod discriminator;
mod format;
mod items;
mod object;
mod subschema;
mod types;

pub use anchor::Anchor;
pub use bool_or_number::{BoolOrNumber, CompiledBoolOrNumber};
pub use compiled_schema::CompiledSchema;
pub use discriminator::Discriminator;
pub use format::Format;
pub use items::Items;
pub use object::Object;
use slotmap::new_key_type;
pub use subschema::Subschema;
pub use types::{Type, Types};

use crate::{
    output::{Annotation, Structure},
    Handler, Scope,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

new_key_type! {
    /// Reference to a [`CompiledSchema`]
    pub struct SchemaRef;
}

/// A JSON Schema document.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Bool(bool),
    Object(Box<Object>),
}

impl Schema {
    /// Returns `true` if the schema is [`Bool`].
    ///
    /// [`Bool`]: Schema::Bool
    #[must_use]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    #[must_use]
    pub fn as_bool(&self) -> Option<&bool> {
        if let Self::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the schema is [`Object`].
    ///
    /// [`Object`]: Schema::Object
    #[must_use]
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(..))
    }
    #[must_use]
    pub fn as_object(&self) -> Option<&Object> {
        if let Self::Object(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
impl Default for Schema {
    fn default() -> Self {
        Schema::Bool(true)
    }
}

struct Annotate<'v, 's, 'c, 'a> {
    instance_location: &'v str,
    keyword_location: &'s str,
    absolute_keyword_location: &'s str,
    value: &'v Value,
    structure: Structure,
    scope: &'s mut Scope<'a>,
    schema: &'c CompiledSchema,
}

impl<'v, 's, 'c, 'a> Annotate<'v, 's, 'c, 'a> {
    async fn exec(self) -> Result<Annotation<'v>, Box<dyn std::error::Error>> {
        let Annotate {
            instance_location,
            keyword_location,
            absolute_keyword_location,
            value,
            structure,
            scope,
            schema,
        } = self;

        let mut nested = scope.nested(
            instance_location,
            keyword_location,
            Some(absolute_keyword_location.to_string()),
        )?;

        let mut result = Annotation::new(nested.location().clone(), value);
        for handler in schema.handlers().iter() {
            let annotation = match handler {
                Handler::Sync(h) => h.evaluate(&mut nested, schema, value, structure)?,
                Handler::Async(h) => h.evaluate(&mut nested, schema, value, structure).await?,
            };
            if let Some(annotation) = annotation {
                result.add(annotation);
                // return early if the annotation is invalid and the output
                // structure is Flag
                if structure.is_flag() && result.is_invalid() {
                    return Ok(result);
                }
            }
        }
        Ok(result)
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
        let obj: Object = serde_json::from_value(schema.clone()).unwrap();
        assert_eq!(
            obj.description,
            Some("A product in the catalog".to_string())
        );
        assert_eq!(obj.title, Some("Product".to_string()));
        assert_eq!(
            obj.id,
            Some("https://example.com/product.schema.json".to_string())
        );
        assert_eq!(
            obj.schema,
            Some("https://json-schema.org/draft/2020-12/schema".to_string())
        );

        let schema: Schema = serde_json::from_value(schema).unwrap();
        assert!(matches!(schema, Schema::Object(..)));
    }
}
