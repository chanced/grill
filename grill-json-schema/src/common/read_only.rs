use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError},
    keyword::{self, Compile, Kind},
    Output, Schema,
};
use serde_json::Value;
use std::sync::Arc;

use crate::READ_ONLY;

#[derive(Debug, Clone, Default)]
pub struct Keyword {
    pub value: Arc<Value>,
}

impl keyword::Keyword for Keyword {
    fn kind(&self) -> Kind {
        Kind::Single(READ_ONLY)
    }
    fn setup<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(value) = schema.get(READ_ONLY) else {
            return Ok(false);
        };
        if !matches!(value, Value::Bool(_)) {
            return Err(InvalidTypeError {
                expected: Expected::Bool,
                actual: value.clone(),
            }
            .into());
        }
        self.value = compile.value(value);
        Ok(true)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(Some(
            ctx.annotate(READ_ONLY, Some(self.value.clone().into())),
        ))
    }
}