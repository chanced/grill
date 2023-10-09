use async_trait::async_trait;
use serde_json::Value;

use crate::{
    error::EvaluateError,
    keyword::{self, Context, Kind},
    output::Output,
};

#[derive(Debug, Default, Clone)]
pub struct Keyword;

#[async_trait]
impl keyword::Keyword for Keyword {
    fn kind(&self) -> Kind {
        Kind::Single(keyword::COMMENT)
    }
    async fn compile<'i>(
        &mut self,
        _compile: &mut keyword::Compile<'i>,
        _schema: crate::Schema<'i>,
    ) -> Result<bool, crate::error::CompileError> {
        Ok(false)
    }
    async fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(None)
    }
}
