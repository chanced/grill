//! `oneOf` keyword.

use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError, UnexpectedValueError},
    keyword::{Compile, Context, Keyword, Kind, Unimplemented},
    output::Output,
    Key, Schema,
};
use jsonptr::Pointer;
use serde_json::Value;

use super::ONE_OF;

/// `oneOf` [`Keyword`]
#[derive(Debug, Clone, Default)]
pub struct OneOf {
    /// List of subschemas
    pub keys: Vec<(Pointer, Key)>,
}

impl Keyword for OneOf {
    fn kind(&self) -> Kind {
        Kind::Keyword(ONE_OF)
    }

    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(one_of) = schema.get(ONE_OF) else {
            return Ok(false);
        };
        let Value::Array(one_of) = one_of else {
            return Err(InvalidTypeError {
                expected: Expected::Array,
                actual: Box::new(one_of.clone()),
            }
            .into());
        };
        if one_of.is_empty() {
            return Err(UnexpectedValueError {
                expected: "a non-empty array",
                value: Box::new(schema.value().clone()),
            }
            .into());
        }
        self.keys = one_of
            .iter()
            .enumerate()
            .map(|(i, _)| jsonptr::Pointer::new([ONE_OF, &i.to_string()]))
            .map(|ptr| compile.subschema(&ptr).map(|key| (ptr, key)))
            .collect::<Result<_, _>>()?;
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v serde_json::Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let mut output = ctx.annotate(Some(ONE_OF), None);
        output.set_valid(false);

        for (path, key) in &self.keys {
            let o = ctx.evaluate(*key, None, path, value)?;
            let subschema_is_valid = o.is_valid();
            let output_is_valid = output.is_valid();
            output.push(o);
            if subschema_is_valid && output_is_valid {
                output.set_valid(false);
            } else if subschema_is_valid {
                output.set_valid(true);
            }
        }
        Ok(Some(output))
    }

    fn subschemas(&self, schema: &Value) -> Result<Vec<jsonptr::Pointer>, Unimplemented> {
        let Some(Value::Array(one_of)) = schema.get(ONE_OF) else {
            return Ok(Vec::default());
        };
        Ok(one_of
            .iter()
            .enumerate()
            .map(|(i, _)| jsonptr::Pointer::new([ONE_OF, &i.to_string()]))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use grill_core::Interrogator;
    use serde_json::json;

    use crate::JsonSchema;

    use super::*;

    async fn create_interrogator() -> Interrogator {
        Interrogator::build()
            .json_schema_2020_12()
            .source_owned_value(
                "https://example.com/with_oneOf",
                json!({
                    "$id": "https://example.com/with_oneOf",
                    "oneOf": [
                        {"type": "string"},
                        {"type": "number"}
                    ]
                }),
            )
            .source_owned_value(
                "https://example.com/without_oneOf",
                json!({
                    "$id": "https://example.com/without_oneOf",
                    "type": "string"
                }),
            )
            .finish()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_setup() {
        let mut interrogator = create_interrogator().await;
        let result = interrogator.compile("https://example.com/with_oneOf").await;
        let key = match result {
            Ok(key) => key,
            Err(err) => {
                panic!("Failed to compile schema\n{err}")
            }
        };
        let with_one_of = interrogator.schema(key).unwrap();
        assert!(with_one_of
            .keywords
            .iter()
            .any(|kw| kw.kind() == Kind::Keyword(ONE_OF)));
        let key = interrogator
            .compile("https://example.com/without_oneOf")
            .await
            .unwrap();
        let without_one_of = interrogator.schema(key).unwrap();
        assert!(!without_one_of
            .keywords
            .iter()
            .any(|kw| kw.kind() == Kind::Keyword(ONE_OF)));
    }
}
