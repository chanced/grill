//! # `writeOnly` keyword.
//!
//! - [Learn JSON Schema - `writeOnly`](https://www.learnjsonschema.com/2020-12/meta-data/writeonly/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-validation#section-9.4)
//!
use super::WRITE_ONLY;
use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError},
    keyword::{self, boolean, Compile},
    output::Annotation,
    Eval, Schema,
};
use serde_json::Value;

/// [`Keyword`] for `"writeOnly"`
#[derive(Debug, Clone, Default)]
pub struct WriteOnly {
    /// value of the `"writeOnly"` keyword
    pub value: bool,
}

impl keyword::Keyword for WriteOnly {
    fn kind(&self) -> keyword::Kind {
        keyword::Kind::Keyword(WRITE_ONLY)
    }
    fn compile<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(value) = schema.get(WRITE_ONLY) else {
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
    ) -> Result<Option<Eval<'v>>, EvaluateError> {
        Ok(Some(ctx.annotate(
            Some(WRITE_ONLY),
            Some(Annotation::StaticRef(boolean(self.value))),
        )))
    }
}
