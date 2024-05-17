//! # `$schema` keyword.
//!
//! - [Learn JSON Schema - `$schema`](https://www.learnjsonschema.com/2020-12/core/schema/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)

use std::{
    ops::{ControlFlow, Deref},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use snafu::IntoError;

use grill_core::{
    criterion::{Criterion, Keyword},
    error::{
        dialect_error, invalid_type_error, Actual, CompileError, DialectError, EvaluateError,
        Expectated,
    },
    Key,
};
use grill_uri::AbsoluteUri;
// use snafu::{Backtrace, ResultExt};

use crate::JsonSchema;

/// [`Keyword`] for `$schema`.
#[derive(Debug, Clone)]
pub struct Schema {
    /// the keyword to use (eg. `$schema`)
    pub keyword: &'static str,

    /// Indicates whether the schema is a boolean value or not
    pub bool: Option<bool>,

    /// The value of the `$schema` field parsed as an `AbsoluteUri`, if present
    pub uri: Option<Arc<AbsoluteUri>>,
}

impl Schema {
    /// Construct a new `Schema` keyword.
    #[must_use]
    pub fn new(keyword: &'static str) -> Self {
        Self {
            keyword,
            bool: None,
            uri: None,
        }
    }
}

impl Schema {}

impl<K> Keyword<JsonSchema, K> for Schema
where
    K: 'static + Key,
{
    fn compile<'i>(
        &mut self,
        _compile: &mut <JsonSchema as Criterion<K>>::Compile<'i>,
        schema: grill_core::Schema<'i, JsonSchema, K>,
    ) -> Result<ControlFlow<()>, CompileError<JsonSchema, K>> {
        match schema.value() {
            Value::Bool(bool) => {
                self.bool = Some(*bool);
                Ok(ControlFlow::Continue(()))
            }
            Value::Object(obj) => {
                let uri = parse_obj(self.keyword, &obj)?;
                self.uri = uri.map(Arc::new);
                Ok(ControlFlow::Continue(()))
            }
            _ => Ok(ControlFlow::Continue(())),
        }
    }
    fn dialect(
        &self,
        schema: &Value,
    ) -> ControlFlow<(), Result<Option<AbsoluteUri>, DialectError>> {
        let Value::Object(obj) = schema else {
            // if the schema is not an object or a bool, any relevant errors
            // should be caught by validation.
            return ControlFlow::Continue(Ok(None));
        };
        ControlFlow::Continue(parse_obj(self.keyword, obj))
    }

    fn evaluate<'i, 'c, 'v, 'r>(
        &'i self,
        ctx: &'c mut <JsonSchema as Criterion<K>>::Context<'i, 'v, 'r>,
        _value: &'v Value,
    ) -> Result<(), EvaluateError<K>> {
        // self.uri
        //     .clone()
        //     .map(Annotation)
        //     .map(|annotation| ctx.report.push_annotation(annotation.into()));
        // Ok(())
        todo!()
    }
}

fn parse_obj(
    keyword: &'static str,
    obj: &serde_json::Map<String, Value>,
) -> Result<Option<AbsoluteUri>, DialectError> {
    if let Some(value) = obj.get(keyword) {
        if let Some(s) = value.as_str() {
            return Ok(Some(AbsoluteUri::parse(s)?));
        }
        return Err(dialect_error::InvalidTypeSnafu { keyword }.into_error(
            invalid_type_error::InvalidTypeSnafu {
                actual: Actual::from_value(value),
                expected: Expectated::String,
                value: Box::new(value.clone()),
            }
            .build(),
        ));
    }
    Ok(None)
}
