//! # `$schema` keyword.
//!
//! - [Learn JSON Schema - `$schema`](https://www.learnjsonschema.com/2020-12/core/schema/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)

use std::{
    cell::OnceCell,
    ops::ControlFlow,
    sync::{Arc, OnceLock},
};

use serde_json::Value;

use grill_core::{
    criterion::{Criterion, Keyword, Kind},
    error::{CompileError, EvaluateError, Expected, IdentifyError, InvalidTypeError},
    uri::AbsoluteUri,
    Key,
};
use snafu::Backtrace;

/// [`Keyword`] for `$schema`.
#[derive(Debug, Clone)]
pub struct Schema {
    /// the keyword to use (eg. `$schema`)
    pub keyword: &'static str,

    /// Whether the [`Dialect`](grill_core::Dialect) allows for fragmented
    /// metaschema IDs
    pub allow_fragment: bool,

    /// Indicates whether the schema is a boolean value or not
    pub boolean: Option<bool>,

    /// The determined dialect. This may be inferred based on context or
    /// configuration if the schema does not contain a `$schema` field.
    pub dialect: OnceLock<AbsoluteUri>,

    /// The value of the `$schema` field, if present
    pub value: Option<Arc<Value>>,
}

impl Schema {
    /// Construct a new `Schema` keyword.
    #[must_use]
    pub fn new(keyword: &'static str, allow_fragment: bool) -> Self {
        Self {
            keyword,
            allow_fragment,
            boolean: None,
            dialect: OnceLock::new(),
            value: None,
        }
    }
}

impl<C, K> Keyword<C, K> for Schema
where
    C: Criterion<K>,
    K: Key,
{
    fn kind(&self) -> Kind {
        self.keyword.into()
    }
    fn compile<'i>(
        &mut self,
        compile: &mut C::Compile,
        schema: grill_core::Schema<'i, C, K>,
    ) -> Result<ControlFlow<()>, CompileError<C, K>> {
        match schema.value() {
            Value::Bool(bool) => {
                self.boolean = Some(*bool);
            }
            Value::Object(obj) => {}
            other => {
                // there should probably be a variant specifically for invalid schema type
                return Err(InvalidTypeError {
                    expected: Expected::AnyOf(&[Expected::Bool, Expected::Object]),
                    actual: Box::new(other.clone()),
                    backtrace: Backtrace::capture(),
                }
                .into());
            }
        }
        Ok(ControlFlow::Continue(()))
    }
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut C::Context,
        value: &'v Value,
    ) -> Result<Option<C::Report<'v>>, EvaluateError<K>> {
        // let Some(bool) = self.boolean else {
        //     return Ok(None);
        // };
        // if bool {
        //     Ok(Some(ctx.annotate(None, None)))
        // } else {
        //     Ok(Some(ctx.error(None, None)))
        // }
        todo!()
    }
    fn dialect(
        &self,
        schema: &Value,
    ) -> ControlFlow<(), Result<Option<AbsoluteUri>, IdentifyError>> {
        todo!()
        // let Some(schema) = schema.get(self.keyword) else {
        //     return Ok(Ok(None));
        // };
        // let schema = schema.as_str().ok_or(IdentifyError::NotAString {
        //     keyword: self.keyword,
        //     value: Box::new(schema.clone()),
        // });
        // if let Err(err) = schema {
        //     return Ok(Err(err));
        // }
        // let schema = schema.unwrap();
        // let uri = AbsoluteUri::parse(schema)
        //     .map(Some)
        //     .map_err(IdentifyError::InvalidUri);
        // if let Err(err) = uri {
        //     return Ok(Err(err));
        // }
        // let uri = uri.unwrap();
        // if uri.is_none() {
        //     return Ok(Ok(None));
        // }
        // let uri = uri.unwrap();
        // if !self.allow_fragment && !uri.is_fragment_empty_or_none() {
        //     return Ok(Err(IdentifyError::FragmentedId(uri.into())));
        // }
        // Ok(Ok(Some(uri)))
    }
}
