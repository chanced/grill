//! # `$ref` keyword.
//!
//! - [Learn JSON Schema - `$ref`](https://www.learnjsonschema.com/2020-12/core/ref/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core#section-8.2.3.1)
use std::sync::Arc;

use jsonptr::Pointer;
use keyword::Unimplemented;
use serde_json::Value;

use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError, RefError},
    keyword::{self, Compile, Context, Kind},
    Key, Output, Schema, Uri,
};

/// A reference to another schema.
#[derive(Debug, Clone, Default)]
pub struct Ref {
    /// The name of the keyword.
    pub keyword: &'static str,
    /// The pointer to the keyword in the schema.
    pub keyword_ptr: Pointer,
    /// The key of the referenced schema.
    pub ref_key: Key,
    /// the value of the keyword as a [`Value`] in an `Arc`
    pub ref_uri_value: Arc<Value>,
    /// Determines whether this `Ref` must evaluate or merely annotate.
    ///
    /// Note: JSON Schema 07 and earlier do not evaluate refs.
    pub must_eval: bool,
}

impl Ref {
    /// Creates a new [`Keyword`] for handling direct references which may or
    /// may not evaluate, as determined by the `must_eval` parameter.
    #[must_use]
    pub fn new(keyword: &'static str, must_eval: bool) -> Self {
        Self {
            keyword,
            keyword_ptr: Pointer::new([keyword]),
            ref_key: Key::default(),
            ref_uri_value: Arc::new(Value::Null),
            must_eval,
        }
    }
    fn get_ref(&self, schema: &Value) -> Result<Vec<grill_core::schema::Ref>, RefError> {
        let Some(v) = schema.get(self.keyword) else {
            return Ok(Vec::default());
        };
        let Value::String(uri) = v else {
            return Err(InvalidTypeError {
                expected: Expected::String,
                actual: Box::new(v.clone()),
            }
            .into());
        };
        let uri = Uri::parse(uri)?;
        Ok(vec![grill_core::schema::Ref {
            uri,
            keyword: self.keyword,
        }])
    }
}

impl keyword::Keyword for Ref {
    fn kind(&self) -> Kind {
        Kind::Keyword(self.keyword)
    }

    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(v) = schema.get(self.keyword) else {
            return Ok(false);
        };
        self.ref_uri_value = compile.value(v);
        let Value::String(uri) = v else {
            return Err(InvalidTypeError {
                expected: Expected::String,
                actual: Box::new(v.clone()),
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
                .annotate(Some(self.keyword), Some(self.ref_uri_value.clone().into()))
                .into();
        }
        ctx.evaluate(self.ref_key, None, &self.keyword_ptr, value)?
            .into()
    }

    /// Returns a list of [`Ref`]s to other
    /// schemas that `schema` depends on.
    fn refs(
        &self,
        schema: &Value,
    ) -> Result<Result<Vec<grill_core::schema::Ref>, RefError>, Unimplemented> {
        Ok(self.get_ref(schema))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use serde_json::json;

    use crate::{
        draft_2020_12::json_schema_2020_12_uri,
        keyword::{const_, id, schema, ID, REF, SCHEMA},
    };
    use grill_core::{schema::Dialect, Interrogator, Structure};

    async fn create_interrogator(ref_value: impl ToString) -> Interrogator {
        let dialect = Dialect::build(json_schema_2020_12_uri().clone())
            .add_keyword(schema::Schema::new(SCHEMA, false))
            .add_keyword(const_::Const::new(None))
            .add_keyword(id::Id::new(ID, false))
            .add_keyword(Ref::new(REF, true))
            .add_metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
            .finish()
            .unwrap();
        Interrogator::build()
            .dialect(dialect)
            .source_owned_value(
                "https://example.com/referenced",
                json!({
                    "const": "value"
                }),
            )
            .source_owned_value(
                "https://example.com/with_$ref",
                json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$id": "https://example.com/with_$ref",
                    "$ref": Value::String(ref_value.to_string())
                }),
            )
            .source_owned_value(
                "https://example.com/without_$ref",
                json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$id": "https://example.com/without_$ref",
                }),
            )
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
        assert!(schema.keywords.iter().map(|kw| kw.kind()).any(|k| k == REF));
        let key = interrogator
            .compile("https://example.com/without_$ref")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();
        assert!(!schema.keywords.iter().map(|kw| kw.kind()).any(|k| k == REF));
    }
    #[tokio::test]
    async fn test_evaluate() {
        let mut interrogator = create_interrogator("https://example.com/referenced").await;
        let key = interrogator
            .compile("https://example.com/with_$ref")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();
        assert!(schema.keywords.iter().map(|kw| kw.kind()).any(|k| k == REF));
        let _ = interrogator
            .compile("https://example.com/without_$ref")
            .await
            .unwrap();
        let value = json!(34.34);
        let output = interrogator
            .evaluate(Structure::Verbose, key, &value)
            .unwrap();
        println!("++ verbose:\n{output}");
        let basic_output = interrogator
            .evaluate(Structure::Basic, key, &value)
            .unwrap();
        println!("++ basic:\n{basic_output}");
    }
}
