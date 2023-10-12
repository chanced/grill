use crate::{
    error::{CompileError, EvaluateError, Expected, UnexpectedTypeError},
    json_schema,
    keyword::{self, Compile, Kind},
    Output, Schema,
};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct Keyword {
    pub value: Arc<Value>,
}

impl keyword::Keyword for Keyword {
    fn kind(&self) -> Kind {
        Kind::Single(json_schema::READ_ONLY)
    }
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(value) = schema.get(json_schema::READ_ONLY) else {
            return Ok(false);
        };
        if !matches!(value, Value::Bool(_)) {
            return Err(UnexpectedTypeError {
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
        Ok(Some(ctx.annotate(Some(self.value.clone().into()))))
    }
}
