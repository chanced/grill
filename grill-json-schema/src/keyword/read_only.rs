use std::borrow::Cow;

use super::{FALSE, READ_ONLY, TRUE};
use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError},
    keyword::{self, Compile},
    output::Annotation,
    Output, Schema,
};
use serde_json::Value;

/// [`Keyword`] for `"readOnly"`
#[derive(Debug, Clone, Default)]
pub struct ReadOnly {
    /// value of the `"readOnly"` keyword
    pub value: bool,
}

impl keyword::Keyword for ReadOnly {
    fn kind(&self) -> keyword::Kind {
        keyword::Kind::Single(READ_ONLY)
    }
    fn setup<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(value) = schema.get(READ_ONLY) else {
            return Ok(false);
        };
        if !matches!(value, Value::Bool(_)) {
            return Err(InvalidTypeError {
                expected: Expected::Bool,
                actual: Box::new(value.clone()),
            }
            .into());
        }
        Ok(true)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(Some(ctx.annotate(
            Some(READ_ONLY),
            Some(Annotation::Cow(Cow::Borrowed(if self.value {
                TRUE
            } else {
                FALSE
            }))),
        )))
    }
}
