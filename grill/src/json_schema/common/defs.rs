use crate::{
    error::{CompileError, EvaluateError},
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
        _compile: &mut Compile<'i>,
        _schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        Ok(false)
    }
    async fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(None)
    }
}
