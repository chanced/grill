//! # `$comment` keyword.
//!
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core#section-8.3)

use serde_json::Value;

use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError},
    keyword::{Compile, Context, Keyword, Kind},
    output::Output,
    Schema,
};

use crate::keyword::COMMENT;

/// [`Keyword`] for `$comment`
#[derive(Debug, Default, Clone)]
pub struct Comment {
    /// the value of the `$comment` keyword
    pub comment: String,
}

impl Keyword for Comment {
    fn kind(&self) -> Kind {
        Kind::Single(COMMENT)
    }
    fn compile<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        if let Some(comment) = schema.get(COMMENT) {
            if let Value::String(comment) = comment {
                self.comment = comment.clone();
            } else {
                return Err(InvalidTypeError {
                    expected: Expected::String,
                    actual: Box::new(comment.clone()),
                }
                .into());
            }
            return Ok(true);
        }
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
