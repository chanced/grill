mod anchor;
mod bool_or_number;
mod discriminator;
mod format;
mod items;
mod object;
mod types;

pub use anchor::Anchor;
pub use bool_or_number::{BoolOrNumber, CompiledBoolOrNumber};
pub use discriminator::Discriminator;
pub use format::Format;
pub use items::Items;
use num_rational::BigRational;
pub use object::Object;
pub use types::{Type, Types};

use crate::{
    output::{Annotation, Structure},
    Handler, Location, Output, Scope, State,
};
use jsonptr::Pointer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{borrow::Cow, collections::HashMap};

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
    /// # Errors
    /// Returns `self` if the schema is not [`Bool`].
    pub fn try_into_bool(self) -> Result<bool, Self> {
        if let Self::Bool(v) = self {
            Ok(v)
        } else {
            Err(self)
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
    /// # Errors
    /// Returns `self` if the schema is not [`Object`].
    pub fn try_into_object(self) -> Result<Box<Object>, Self> {
        if let Self::Object(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
impl Default for Schema {
    fn default() -> Self {
        Schema::Bool(true)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CompiledSchema {
    pub absolute_location: String,
    meta_schema: String,
    numbers: HashMap<&'static str, BigRational>,
    schemas: HashMap<&'static str, CompiledSchema>,
    handlers: Box<[Handler]>,
    schema: Schema,
}

impl CompiledSchema {
    /// # Errors
    #[allow(clippy::missing_panics_doc)]
    pub async fn evaluate<'v>(
        &self,
        value: &'v Value,
        structure: Structure,
    ) -> Result<Output<'v>, Box<dyn std::error::Error>> {
        let mut state = State::new();
        let location = Location {
            absolute_keyword_location: self.absolute_location.clone(),
            keyword_location: Pointer::default(),
            instance_location: Pointer::default(),
        };
        let mut scope = Scope::new(location, &mut state);
        let annotation = self.annotate("", "", &mut scope, value, structure).await?;
        Ok(Output::new(structure, annotation))
    }

    #[must_use]
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn number<'a>(&'a self, keyword: &str) -> Option<&'a BigRational> {
        self.numbers.get(keyword)
    }

    pub fn subschema<'a>(&'a self, keyword: &str) -> Option<CompiledSubschema<'a>> {
        self.schemas.get(keyword).map(|schema| CompiledSubschema {
            keyword,
            schema,
        })
    }

    /// # Errors
    /// if a custom [`Handler`](`crate::Handler`) returns a [`Box<dyn Error`](`std::error::Error`)
    async fn annotate<'v, 's, 'a>(
        &self,
        instance_location: &'v str,
        keyword_location: &'s str,
        scope: &'s mut Scope<'a>,
        value: &'v Value,
        structure: Structure,
    ) -> Result<Annotation<'v>, Box<dyn std::error::Error>> {
        let annotation = Annotate {
            absolute_keyword_location: &self.absolute_location,
            instance_location,
            keyword_location,
            scope,
            structure,
            value,
            schema: self,
        }
        .exec()
        .await?;
        Ok(annotation)
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

impl<'v, 's, 'h, 'a> Annotate<'v, 's, 'h, 'a> {
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
        for handler in schema.handlers.iter() {
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

#[derive(Debug, Clone)]
pub enum Subschema<'s> {
    Inline(Cow<'s, Schema>),
    Reference(&'s str),
}

#[derive(Debug, Clone)]
pub struct CompiledSubschema<'s> {
    keyword: &'s str,
    schema: &'s CompiledSchema,
}

impl CompiledSubschema<'_> {
    pub fn absolute_location(&self) -> &str {
        &self.schema.absolute_location
    }
    pub fn schema(&self) -> &Schema {
        &self.schema.schema
    }
    pub fn keyword_location(&self) -> &str {
        &self.keyword
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
