use std::borrow::Cow;

use serde_json::Value;

use crate::{
    output::{Annotation, Structure},
    Schema, Scope,
};

use super::CompiledSchema;

#[derive(Debug, Clone)]
pub enum Subschema<'s> {
    Reference(&'s str),
    Inline(Cow<'s, Schema>),
}

pub struct CompiledSubschema {
    schema: Option<CompiledSchema>,
}

impl CompiledSubschema {
    /// # Errors
    /// if a custom [`Handler`](`crate::Handler`) returns a [`Box<dyn Error`](`std::error::Error`)
    pub async fn evaluate(
        &self,
        scope: &Scope,
        value: &Value,
        output_structure: Structure,
    ) -> Result<Option<Annotation>, Box<dyn std::error::Error>> {
        match self.schema {
            Some(ref schema) => {
                schema
                    .evaluate_internal(scope, value, output_structure)
                    .await
            }
            None => Ok(None),
        }
    }
}
