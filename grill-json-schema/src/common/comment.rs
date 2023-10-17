use serde_json::Value;

use crate::{
    error::EvaluateError,
    json_schema,
    keyword::{self, Context, Kind},
    output::Output,
};

#[derive(Debug, Default, Clone)]
pub struct Keyword;

impl keyword::Keyword for Keyword {
    fn kind(&self) -> Kind {
        Kind::Single(json_schema::COMMENT)
    }
    fn setup<'i>(
        &mut self,
        _compile: &mut keyword::Compile<'i>,
        _schema: crate::Schema<'i>,
    ) -> Result<bool, crate::error::CompileError> {
        Ok(false)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(None)
    }
}
