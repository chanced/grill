use std::sync::Arc;

use keyword::Unimplemented;
use serde_json::Value;

use crate::{
    error::{CompileError, EvaluateError, Expected, RefError, UnexpectedTypeError},
    keyword::{self, Compile, Context, Kind},
    schema::Ref,
    Key, Output, Schema, Uri,
};

#[derive(Debug, Clone, Default)]
pub struct Keyword {
    pub keyword: &'static str,
    /// The key of the referenced schema.
    pub ref_key: Key,
    pub ref_uri_value: Arc<Value>,
    pub must_eval: bool,
}

impl Keyword {
    /// Creates a new [`Keyword`] for handling direct references which may or
    /// may not evaluate, as determined by the `must_eval` parameter.
    #[must_use]
    pub fn new(keyword: &'static str, must_eval: bool) -> Self {
        Self {
            keyword,
            ref_key: Key::default(),
            ref_uri_value: Arc::new(Value::Null),
            must_eval,
        }
    }
    fn get_ref(&self, schema: &Value) -> Result<Vec<Ref>, RefError> {
        let Some(v) = schema.get(self.keyword) else {
            return Ok(Vec::default());
        };
        let Value::String(uri) = v else {
            return Err(UnexpectedTypeError {
                expected: Expected::String,
                actual: v.clone(),
            }
            .into());
        };
        let uri = Uri::parse(uri)?;
        Ok(vec![Ref {
            uri,
            keyword: self.keyword,
        }])
    }
}

impl keyword::Keyword for Keyword {
    fn kind(&self) -> Kind {
        Kind::Single(self.keyword)
    }

    fn setup<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(v) = schema.get(self.keyword) else {
            return Ok(false);
        };
        self.ref_uri_value = compile.value(v);
        let Value::String(uri) = v else {
            return Err(UnexpectedTypeError {
                expected: Expected::String,
                actual: v.clone(),
            }
            .into());
        };
        let ref_key = compile.schema(uri)?;
        self.ref_key = ref_key;
        Ok(true)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        if !self.must_eval {
            return ctx
                .annotate(self.keyword, Some(self.ref_uri_value.clone().into()))
                .into();
        }
        ctx.evalute(self.ref_key, None, self.keyword, value)?.into()
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    fn refs(&self, schema: &Value) -> Result<Result<Vec<Ref>, RefError>, Unimplemented> {
        Ok(self.get_ref(schema))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use serde_json::json;

    use crate::{
        json_schema::{
            self,
            common::{const_, id, schema},
            draft_2020_12::json_schema_2020_12_uri,
        },
        schema::Dialect,
        Interrogator, Structure,
    };
    async fn create_interrogator(ref_value: impl ToString) -> Interrogator {
        let dialect = Dialect::builder(json_schema_2020_12_uri().clone())
            .keyword(schema::Keyword::new(json_schema::SCHEMA, false))
            .keyword(const_::Keyword::new(None))
            .keyword(id::Keyword::new(json_schema::ID, false))
            .keyword(Keyword::new(json_schema::REF, true))
            .metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
            .build()
            .unwrap();
        Interrogator::builder()
            .dialect(dialect)
            .source_value(
                "https://example.com/referenced",
                Cow::Owned(json!({
                    "const": "value"
                })),
            )
            .unwrap()
            .source_value(
                "https://example.com/with_$ref",
                Cow::Owned(json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$id": "https://example.com/with_$ref",
                    "$ref": Value::String(ref_value.to_string())
                })),
            )
            .unwrap()
            .source_value(
                "https://example.com/without_$ref",
                Cow::Owned(json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$id": "https://example.com/without_$ref",
                })),
            )
            .unwrap()
            .finish()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_setup() {
        let mut interrogator = create_interrogator("https://example.com/referenced").await;
        let key = interrogator
            .compile("https://example.com/with_$ref")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();
        assert!(schema
            .keywords
            .iter()
            .map(|kw| kw.kind())
            .any(|k| k == json_schema::REF));
        let key = interrogator
            .compile("https://example.com/without_$ref")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();
        assert!(!schema
            .keywords
            .iter()
            .map(|kw| kw.kind())
            .any(|k| k == json_schema::REF));
        dbg!(interrogator.schemas);
    }
    #[tokio::test]
    async fn test_evaluate() {
        let mut interrogator = create_interrogator("https://example.com/referenced").await;
        let key = interrogator
            .compile("https://example.com/with_$ref")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();
        assert!(schema
            .keywords
            .iter()
            .map(|kw| kw.kind())
            .any(|k| k == json_schema::REF));
        let _ = interrogator
            .compile("https://example.com/without_$ref")
            .await
            .unwrap();
        let value = json!(34.34);
        let output = interrogator
            .evaluate(key, Structure::Verbose, &value)
            .unwrap();
        println!("++ verbose:\n{output}");
        let basic_output = interrogator
            .evaluate(key, Structure::Basic, &value)
            .unwrap();
        println!("++ basic:\n{basic_output}");
    }
}
