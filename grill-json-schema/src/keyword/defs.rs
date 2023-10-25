use grill_core::{
    error::{CompileError, EvaluateError},
    keyword::{self, Compile, Kind},
    Output, Schema,
};
use serde_json::Value;
use std::sync::Arc;

use super::WRITE_ONLY;

#[derive(Debug, Clone, Default)]
pub struct Defs {
    pub value: Arc<Value>,
}
impl keyword::Keyword for Defs {
    fn kind(&self) -> Kind {
        Kind::Single(WRITE_ONLY)
    }
    fn setup<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        _schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        Ok(false)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(None)
    }
}
