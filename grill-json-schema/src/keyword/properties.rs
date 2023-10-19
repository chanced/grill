use crate::PROPERTIES;
use ahash::AHashMap;
use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError},
    keyword::{self, Compile, Context, Unimplemented},
    output::Output,
    Key, Schema,
};
use jsonptr::{Pointer, Token};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Properties {
    subschemas: AHashMap<String, Key>,
}

impl Properties {
    #[must_use]
    pub fn new() -> Self {
        Self {
            subschemas: AHashMap::new(),
        }
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}

impl keyword::Keyword for Properties {
    fn kind(&self) -> keyword::Kind {
        keyword::Kind::Single(PROPERTIES)
    }

    fn setup<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let j = serde_json::to_string_pretty(&schema).unwrap();
        println!("{j}");
        let Some(value) = schema.get(PROPERTIES) else {
            return Ok(false);
        };
        if !matches!(value, Value::Object(_)) {
            return Err(InvalidTypeError {
                expected: Expected::Object,
                actual: value.clone(),
            }
            .into());
        };
        for subschema in subschemas(&schema) {
            let keyword = subschema.last().unwrap().decoded().to_string();
            let key = compile.subschema(subschema)?;
            self.subschemas.insert(keyword, key);
        }
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v serde_json::Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let Some(obj) = value.as_object() else {
            return Ok(None);
        };
        let mut ptr = Pointer::new([PROPERTIES]);
        let mut output = ctx.annotate(PROPERTIES, None);
        for (prop, key) in &self.subschemas {
            if let Some(v) = obj.get(prop) {
                ptr.push_back(prop.into());
                output.push(ctx.evaluate(*key, Some(prop), &ptr, v)?);
                ptr.pop_back();
                if !output.is_valid() && ctx.should_short_circuit() {
                    break;
                }
            }
        }
        Ok(Some(output))
    }

    fn subschemas(&self, schema: &serde_json::Value) -> Result<Vec<Pointer>, Unimplemented> {
        Ok(subschemas(schema))
    }
}

fn subschemas(schema: &Value) -> Vec<Pointer> {
    let Some(Value::Object(props)) = schema.get("properties") else {
        return Vec::new();
    };
    let base = Pointer::new(["properties"]);
    props
        .keys()
        .map(|k| {
            let mut ptr = base.clone();
            ptr.push_back(Token::from(k));
            ptr
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use serde_json::json;

    use crate::{
        draft_2020_12::json_schema_2020_12_uri,
        keyword::{const_, id, schema},
        ID, SCHEMA,
    };
    use grill_core::{schema::Dialect, Interrogator, Structure};

    use super::*;

    async fn create_interrogator(properties: Value) -> Interrogator {
        let dialect = Dialect::build(json_schema_2020_12_uri().clone())
            .with_keyword(schema::Keyword::new(SCHEMA, false))
            .with_keyword(id::Keyword::new(ID, false))
            .with_keyword(const_::Const::new(None))
            .with_keyword(super::Properties::new())
            .with_metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
            .finish()
            .unwrap();
        Interrogator::builder()
            .dialect(dialect)
            .source_value(
                "https://example.com/with_properties",
                Cow::Owned(json!({
                    "$id": "https://example.com/with_properties",
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "properties": properties
                })),
            )
            .unwrap()
            .source_value(
                "https://example.com/without_properties",
                Cow::Owned(json!({
                    "$id": "https://example.com/without_properties",
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                })),
            )
            .unwrap()
            .finish()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_setup() {
        let mut interrogator = create_interrogator(json!({
            "foo": {
                "const": 34.34
            },
        }))
        .await;
        let key = interrogator
            .compile("https://example.com/with_properties")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();

        assert!(schema
            .keywords
            .iter()
            .any(|k| k.kind() == crate::PROPERTIES));
    }

    #[tokio::test]
    async fn test_evaluate() {
        let mut interrogator = create_interrogator(json!({
            "foo": {
                "const": 34.34
            },
        }))
        .await;
        let key = interrogator
            .compile("https://example.com/with_properties")
            .await
            .unwrap();
        let invalid = json!({
            "foo": 32
        });
        let output = interrogator
            .evaluate(key, Structure::Verbose, &invalid)
            .unwrap();
        println!("{output}");
    }
}
