use crate::{
    error::{CompileError, EvaluateError, Expected, UnexpectedTypeError},
    keyword::{self, Compile},
    Output, Schema,
};
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct Keyword {
    pub value: Arc<Value>,
}
#[async_trait]
impl keyword::Keyword for Keyword {
    fn kind(&self) -> keyword::Kind {
        keyword::Kind::Single(keyword::WRITE_ONLY)
    }
    async fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(value) = schema.get(keyword::WRITE_ONLY) else {
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
    async fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(Some(ctx.annotate(Some(self.value.clone().into()))))
    }
}
