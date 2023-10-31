//! # `$ref` keyword.
//!
//! - [Learn JSON Schema - `$ref`](https://www.learnjsonschema.com/2020-12/core/ref/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core#section-8.2.3.1)
use std::sync::Arc;

use keyword::Unimplemented;
use serde_json::Value;

use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError, RefError},
    keyword::{self, Compile, Context, Kind},
    static_pointer_fn, AbsoluteUri, Key, Output, Schema, Uri,
};

use super::{dynamic_anchor::DynamicAnchors, DYNAMIC_REF};

static_pointer_fn!(pub dynamic_ref "/$dynamicRef");

/// A dynmaic reference to another schema.
#[derive(Debug, Clone, Default)]
pub struct DynamicRef {
    /// The default key of the referenced schema.
    pub default_key: Key,
    /// the value of the keyword as a [`Value`] in an `Arc`
    pub value: Arc<Value>,
    /// The absolute URI of the referenced schema.
    ///
    /// If this `DynamicRef` is attached to a schema, this will be set.
    pub absolute_uri: Option<AbsoluteUri>,
}

impl DynamicRef {
    /// Creates a new [`Keyword`] for handling direct references which may or
    /// may not evaluate, as determined by the `must_eval` parameter.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl keyword::Keyword for DynamicRef {
    fn kind(&self) -> Kind {
        Kind::Keyword(DYNAMIC_REF)
    }
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(v) = schema.get(DYNAMIC_REF) else {
            return Ok(false);
        };
        self.value = compile.value(v);
        let Value::String(uri) = v else {
            return Err(InvalidTypeError {
                expected: Expected::String,
                actual: Box::new(v.clone()),
            }
            .into());
        };
        let ref_key = compile.schema(uri)?;
        self.default_key = ref_key;

        let uri = Uri::parse(uri)?;
        self.absolute_uri = Some(schema.absolute_uri().resolve(&uri)?);
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let key = ctx
            .eval_state()
            .get::<DynamicAnchors>()
            .and_then(|anchors| anchors.get(self.absolute_uri.as_ref().unwrap()))
            .unwrap_or(self.default_key);
        Some(ctx.evaluate(key, None, dynamic_ref_pointer(), value)).transpose()
    }

    /// Returns a list of [`Ref`]s to other
    /// schemas that `schema` depends on.
    fn refs(
        &self,
        schema: &Value,
    ) -> Result<Result<Vec<grill_core::schema::Ref>, RefError>, Unimplemented> {
        Ok(get_ref(schema))
    }
}

fn get_ref(schema: &Value) -> Result<Vec<grill_core::schema::Ref>, RefError> {
    let Some(v) = schema.get(DYNAMIC_REF) else {
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
        keyword: DYNAMIC_REF,
    }])
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
            .add_keyword(DynamicRef::new())
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
            .evaluate(key, Structure::Verbose, &value)
            .unwrap();
        println!("++ verbose:\n{output}");
        let basic_output = interrogator
            .evaluate(key, Structure::Basic, &value)
            .unwrap();
        println!("++ basic:\n{basic_output}");
    }
}
