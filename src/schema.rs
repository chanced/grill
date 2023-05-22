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
pub use types::{Type, Types};

use crate::{
    output::{Annotation, Structure},
    Handler, Output, Scope,
};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

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
    pub absolute_location: Option<String>,
    handlers: Box<[Handler]>,
    schema: Schema,
}

impl CompiledSchema {
    #[allow(clippy::missing_panics_doc)]
    /// # Errors
    pub async fn evaluate<'v>(
        &self,
        value: &'v Value,
        structure: Structure,
    ) -> Result<Output<'v>, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// # Errors
    /// if a custom [`Handler`](`crate::Handler`) returns a [`Box<dyn Error`](`std::error::Error`)
    async fn annotate<'v, 's>(
        &self,
        instance_location: &'v str,
        keyword_location: &'s str,
        scope: &'s mut Scope,
        value: &'v Value,
        structure: Structure,
    ) -> Result<Annotation<'v>, Box<dyn std::error::Error>> {
        let annotation = Annotate {
            absolute_keyword_location: self.absolute_location.clone(),
            handlers: &self.handlers,
            instance_location,
            keyword_location,
            scope,
            structure,
            value,
        }
        .exec()
        .await?;
        Ok(annotation)
    }
}

struct Annotate<'v, 's, 'h> {
    instance_location: &'v str,
    keyword_location: &'s str,
    absolute_keyword_location: Option<String>,
    value: &'v Value,
    structure: Structure,
    scope: &'s mut Scope,
    handlers: &'h [Handler],
}

impl<'v, 's, 'h> Annotate<'v, 's, 'h> {
    async fn exec(self) -> Result<Annotation<'v>, Box<dyn std::error::Error>> {
        let Annotate {
            instance_location,
            keyword_location,
            absolute_keyword_location,
            value,
            structure,
            scope,
            handlers,
        } = self;

        let mut nested = scope.nested(
            instance_location,
            keyword_location,
            absolute_keyword_location,
        )?;

        let mut result = Annotation::new(nested.location().clone());
        for handler in handlers.iter() {
            let annotation = match handler {
                Handler::Sync(h) => h.evaluate(&mut nested, value, structure)?,
                Handler::Async(h) => h.evaluate(&mut nested, value, structure).await?,
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
    Reference(&'s str),
    Inline(Cow<'s, Schema>),
}

pub struct CompiledSubschema {
    keyword_location: String,
    schema: OnceCell<CompiledSchema>,
}

impl CompiledSubschema {
    pub fn absolute_location(&self) -> Option<&str> {
        self.schema().absolute_location.as_deref()
    }
    pub fn schema(&self) -> &CompiledSchema {
        self.schema
            .get()
            .expect("Schema not compiled: this is a bug")
    }

    pub fn keyword_location(&self) -> &str {
        &self.keyword_location
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
