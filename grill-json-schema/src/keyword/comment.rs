use serde_json::Value;

use grill_core::{
    error::{CompileError, EvaluateError},
    keyword::{self, Compile, Context, Kind},
    output::Output,
    Schema,
};

use crate::keyword::COMMENT;

#[derive(Debug, Default, Clone)]
pub struct Comment;

impl keyword::Keyword for Comment {
    fn kind(&self) -> Kind {
        Kind::Single(COMMENT)
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
        _ctx: &'i mut Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(None)
    }
}
