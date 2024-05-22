//! # `if`, `then`, `else` keywords.
//!
//! - [Learn JSON Schema - if](https://www.learnjsonschema.com/2020-12/applicator/if/)
//! - [Learn JSON Schema - then](https://www.learnjsonschema.com/2020-12/applicator/then/)
//! - [Learn JSON Schema - else](https://www.learnjsonschema.com/2020-12/applicator/else/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core#section-10.2.2.1)
use grill_core::{
    error::{CompileError, EvaluateError},
    language::{static_pointer_fn, Compile, Context, Keyword, Kind},
    Key, Schema,
};
use serde_json::Value;

use super::{ELSE, IF, THEN};

static_pointer_fn!(pub if "/if");
static_pointer_fn!(pub then "/then");
static_pointer_fn!(pub else "/else");

/// [`Keyword`] for the `if`, `then`, and `else` keywords.
#[derive(Debug, Clone, Default)]
pub struct IfThenElse<K: 'static + Key> {
    /// The key of the subschema for the `if` keyword.
    pub if_key: K,
    /// The key of the subschema for the `then` keyword.
    pub then_key: Option<K>,
    /// The key of the subschema for the `else` keyword.
    pub else_key: Option<K>,
}

impl Keyword for IfThenElse {
    fn kind(&self) -> Kind {
        Kind::Composite(&[IF, THEN, ELSE])
    }

    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        if schema.get(IF).is_none() {
            return Ok(false);
        };
        self.if_key = compile.subschema(if_pointer())?;
        if schema.get(THEN).is_some() {
            self.then_key = Some(compile.subschema(then_pointer())?);
        }
        if schema.get(ELSE).is_some() {
            self.else_key = Some(compile.subschema(else_pointer())?);
        }
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let if_output = ctx.probe(self.if_key, None, if_pointer(), value)?;
        let mut outputs = vec![if_output];
        let mut is_valid = true;
        if outputs[0].is_annotation() && self.then_key.is_some() {
            let then_key = self.then_key.unwrap();
            let output = ctx.evaluate(then_key, None, then_pointer(), value)?;
            is_valid = output.is_annotation();
            outputs.push(output);
        } else if outputs[0].is_error() && self.else_key.is_some() {
            let else_key = self.else_key.unwrap();
            let output = ctx.evaluate(else_key, None, else_pointer(), value)?;
            is_valid = output.is_annotation();
            outputs.push(output);
        }
        Ok(Some(ctx.transient(is_valid, outputs)))
    }

    fn subschemas(
        &self,
        schema: &Value,
    ) -> Result<Vec<jsonptr::Pointer>, grill_core::keyword::Unimplemented> {
        let mut subschemas = Vec::new();
        if schema.get(IF).is_some() {
            subschemas.push(if_pointer().clone());
        } else {
            return Ok(Vec::default());
        }
        if schema.get(THEN).is_some() {
            subschemas.push(then_pointer().clone());
        }
        if schema.get(ELSE).is_some() {
            subschemas.push(else_pointer().clone());
        }
        Ok(subschemas)
    }
}

#[cfg(test)]
mod tests {

    use crate::JsonSchema;

    use super::*;
    use grill_core::{Interrogator, Output};
    use serde_json::json;

    #[tokio::test]
    async fn teset_if_then_else_setup() {
        let schema = json!({"if": {} });
        let mut interrogator = Interrogator::build()
            .json_schema_2020_12()
            .source_owned_value("https://example.com/schema", schema)
            .finish()
            .await
            .unwrap();
        let key = interrogator
            .compile("https://example.com/schema")
            .await
            .unwrap();
        assert!(interrogator
            .schema(key)
            .unwrap()
            .keywords
            .iter()
            .any(|k| k.kind() == Kind::Composite(&["if", "then", "else"])));

        let schema = json!({"else": {}, "then": {}});
        let mut interrogator = Interrogator::build()
            .json_schema_2020_12()
            .source_owned_value("https://example.com/schema", schema)
            .finish()
            .await
            .unwrap();
        let key = interrogator
            .compile("https://example.com/schema")
            .await
            .unwrap();
        assert!(!interrogator
            .schema(key)
            .unwrap()
            .keywords
            .iter()
            .any(|k| k.kind() == Kind::Composite(&["if", "then", "else"])));
    }

    #[tokio::test]
    async fn teset_if_then_else_evaluate() {
        let schema = json!({
            "if": true,
            "then": {
                "const": 34.34
            },
            "else": {
                "const": 34
            }
        });
        let mut interrogator = Interrogator::build()
            .json_schema_2020_12()
            .source_owned_value("https://example.com/schema", schema)
            .finish()
            .await
            .unwrap();
        let key = interrogator
            .compile("https://example.com/schema")
            .await
            .unwrap();

        let value = json!(34);
        let o = interrogator.evaluate(Output::Flag, key, &value).unwrap();
        assert!(!o.is_annotation());
        let value = json!(34.34);
        let o = interrogator.evaluate(Output::Flag, key, &value).unwrap();
        assert!(o.is_annotation());

        let schema = json!({
            "if": false,
            "then": {
                "const": 34.34
            },
            "else": {
                "const": 34
            }
        });
        let mut interrogator = Interrogator::build()
            .json_schema_2020_12()
            .source_owned_value("https://example.com/schema", schema)
            .finish()
            .await
            .unwrap();
        let key = interrogator
            .compile("https://example.com/schema")
            .await
            .unwrap();

        let value = json!(34.34);
        let o = interrogator.evaluate(Output::Verbose, key, &value).unwrap();
        assert!(!o.is_annotation());
        let value = json!(34);
        let o = interrogator.evaluate(Output::Verbose, key, &value).unwrap();
        assert!(o.is_annotation());
    }
}
