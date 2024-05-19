//! # `$ref` keyword.
//!
//! - [Learn JSON Schema - `$ref`](https://www.learnjsonschema.com/2020-12/core/ref/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core#section-8.2.3.1)
use std::{ops::ControlFlow, sync::Arc};

use grill_core::{
    language::{Language, Keyword},
    error::{CompileError, EvaluateError, Expectated, RefsError},
    Key,
};
use grill_uri::Uri;
use jsonptr::Pointer;
use serde_json::Value;

use crate::JsonSchema;

/// A reference to another schema.
#[derive(Debug, Clone, Default)]
pub struct Ref<K: 'static + Key> {
    /// The name of the keyword.
    pub keyword: &'static str,
    /// The pointer to the keyword in the schema.
    pub keyword_ptr: Pointer,
    /// The key of the referenced schema.
    pub ref_key: K,
    /// the value of the keyword as a [`Value`] in an `Arc`
    pub ref_uri_value: Arc<Value>,
    /// Determines whether this `Ref` must evaluate or merely annotate.
    ///
    /// Note: JSON Schema 07 and earlier do not evaluate refs.
    pub must_eval: bool,
}

impl<K> Ref<K>
where
    K: Key,
{
    /// Creates a new [`Keyword`] for handling direct references which may or
    /// may not evaluate, as determined by the `must_eval` parameter.
    #[must_use]
    pub fn new(keyword: &'static str, must_eval: bool) -> Self {
        Self {
            keyword,
            keyword_ptr: Pointer::new([keyword]),
            ref_key: K::default(),
            ref_uri_value: Arc::new(Value::Null),
            must_eval,
        }
    }

    // fn get_ref(
    //     &self,
    //     schema: &Value,
    // ) -> Result<Vec<grill_core::schema::Ref>, CompileError<JsonSchema, K>> {
    //     let Some(v) = schema.get(self.keyword) else {
    //         return Ok(Vec::default());
    //     };
    //     let Value::String(uri) = v else {
    //         return Err(CompileError::invalid_type(v.clone(), Expectated::String));
    //     };
    //     let uri = Uri::parse(uri)?;
    //     Ok(vec![grill_core::schema::Ref {
    //         uri,
    //         keyword: self.keyword,
    //     }])
    // }
}

impl<K> Keyword<JsonSchema, K> for Ref<K>
where
    K: 'static + Key + Send + Sync,
{
    fn compile<'i>(
        &mut self,
        compile: &mut <JsonSchema as Language<K>>::Compile<'i>,
        schema: grill_core::Schema<'i, JsonSchema, K>,
    ) -> Result<ControlFlow<()>, CompileError<JsonSchema, K>> {
        let Some(v) = schema.get(self.keyword) else {
            return Ok(ControlFlow::Break(()));
        };
        self.ref_uri_value = compile.values.get_or_insert(v);

        let Value::String(uri) = v else {
            return Err(CompileError::invalid_type(v.clone(), Expectated::String));
        };
        let uri = Uri::parse(uri)?;
        let uri = compile.schema_uri.with_fragment(None)?.resolve(&uri)?;
        self.ref_key = compile
            .schemas
            .get_key(&uri)
            .ok_or(CompileError::schema_not_found(uri))?;
        Ok(ControlFlow::Continue(()))
    }
    fn evaluate<'i, 'c, 'v, 'r>(
        &'i self,
        ctx: &'c mut <JsonSchema as Language<K>>::Context<'i, 'v, 'r>,
        value: &'v Value,
    ) -> Result<(), EvaluateError<K>> {
        if !self.must_eval {
            // return ctx
            //     .annotate(Some(self.keyword), Some(self.ref_uri_value.clone().into()))
            //     .into();
            todo!()
        }
        // ctx.evaluate(self.ref_key, None, &self.keyword_ptr, value)?

        //     .into()
        todo!()
    }

    /// Returns a list of [`Ref`]s to other
    /// schemas that `schema` depends on.
    fn refs(
        &self,
        schema: &Value,
    ) -> ControlFlow<(), Result<Vec<grill_core::language::Ref>, RefsError>> {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use std::borrow::Cow;

//     use super::*;
//     use serde_json::json;

//     use crate::{
//         draft_2020_12::json_schema_2020_12_uri,
//         keyword::{const_, id, schema, ID, REF, SCHEMA},
//     };
//     use grill_core::{schema::Dialect, AbsoluteUri, Interrogator, Output};

//     async fn create_interrogator(ref_value: impl ToString) -> Interrogator {
//         let dialect = Dialect::build(json_schema_2020_12_uri().clone())
//             .add_keyword(schema::Schema::new(SCHEMA, false))
//             .add_keyword(const_::Const::new(None))
//             .add_keyword(id::Id::new(ID, false))
//             .add_keyword(Ref::new(REF, true))
//             .add_metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
//             .finish()
//             .unwrap();
//         Interrogator::build()
//             .dialect(dialect)
//             .source_owned_value(
//                 "https://example.com/referenced",
//                 json!({
//                     "const": "value"
//                 }),
//             )
//             .source_owned_value(
//                 "https://example.com/with_$ref",
//                 json!({
//                     "$schema": "https://json-schema.org/draft/2020-12/schema",
//                     "$id": "https://example.com/with_$ref",
//                     "$ref": Value::String(ref_value.to_string())
//                 }),
//             )
//             .source_owned_value(
//                 "https://example.com/without_$ref",
//                 json!({
//                     "$schema": "https://json-schema.org/draft/2020-12/schema",
//                     "$id": "https://example.com/without_$ref",
//                 }),
//             )
//             .finish()
//             .await
//             .unwrap()
//     }

//     #[tokio::test]
//     async fn test_setup() {
//         let mut interrogator = create_interrogator("https://example.com/referenced").await;
//         let key = interrogator
//             .compile("https://example.com/with_$ref")
//             .await
//             .unwrap();
//         let schema = interrogator.schema(key).unwrap();
//         assert!(schema.keywords.iter().map(|kw| kw.kind()).any(|k| k == REF));
//         let key = interrogator
//             .compile("https://example.com/without_$ref")
//             .await
//             .unwrap();
//         let schema = interrogator.schema(key).unwrap();
//         assert!(!schema.keywords.iter().map(|kw| kw.kind()).any(|k| k == REF));
//     }
//     #[tokio::test]
//     async fn test_evaluate() {
//         let mut interrogator = create_interrogator("https://example.com/referenced").await;
//         let key = interrogator
//             .compile("https://example.com/with_$ref")
//             .await
//             .unwrap();
//         let schema = interrogator.schema(key).unwrap();
//         assert!(schema.keywords.iter().map(|kw| kw.kind()).any(|k| k == REF));
//         let _ = interrogator
//             .compile("https://example.com/without_$ref")
//             .await
//             .unwrap();
//         let value = json!(34.34);
//         let output = interrogator.evaluate(Output::Verbose, key, &value).unwrap();
//         println!("++ verbose:\n{output}");
//         let basic_output = interrogator.evaluate(Output::Basic, key, &value).unwrap();
//         println!("++ basic:\n{basic_output}");
//     }

//     #[tokio::test]
//     async fn test_recursive() {
//         println!("-----------");
//         let schema = json!({
//             "$schema": "https://json-schema.org/draft/2020-12/schema",
//             "properties": {
//                 "foo": {"$ref": "#"}
//             },
//             "additionalProperties": false
//         });
//         let dialect = Dialect::build(
//             "https://json-schema.org/draft/2020-12/schema"
//                 .try_into()
//                 .unwrap(),
//         )
//         .add_keyword(schema::Schema::new(SCHEMA, false))
//         .add_keyword(const_::Const::new(None))
//         .add_keyword(id::Id::new(ID, false))
//         .add_keyword(Ref::new(REF, true))
//         .add_keyword(crate::keyword::properties::Properties::default())
//         .add_metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
//         .finish()
//         .unwrap();

//         let mut interrogator = Interrogator::build()
//             .dialect(dialect)
//             .source_owned_value("https://example.com/recursive", schema)
//             .await
//             .unwrap();
//         let key = interrogator
//             .compile("https://example.com/recursive")
//             .await
//             .unwrap();
//         dbg!(&interrogator);
//         dbg!(key);
//         let uri = AbsoluteUri::parse("https://example.com/recursive#/properties/foo").unwrap();
//         let _schema = interrogator.schema_by_uri(&uri).unwrap();
//         // dbg!(schema);
//         let value = json!({"foo": {"bar": false}});
//         let _output = interrogator.evaluate(Output::Verbose, key, &value).unwrap();
//     }
// }
