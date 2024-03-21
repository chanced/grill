//! # `readOnly` keyword.
//!
//! - [Learn JSON Schema - `readOnly`](https://www.learnjsonschema.com/2020-12/meta-data/readonly/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-validation#section-9.4)
use super::READ_ONLY;
use grill_core::error::invalid_type_error::InvalidTypeSnafu;
use grill_core::{
    error::{EvaluateError, Expected, InvalidTypeError},
    keyword::{self, boolean, Compile},
    output::Annotation,
    Output, Schema,
};
use serde_json::Value;
use snafu::ensure;

/// [`Keyword`] for `"readOnly"`
#[derive(Debug, Clone, Default)]
pub struct ReadOnly {
    /// value of the `"readOnly"` keyword
    pub value: bool,
}

impl keyword::Keyword for ReadOnly {
    fn kind(&self) -> keyword::Kind {
        keyword::Kind::Keyword(READ_ONLY)
    }
    fn compile<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(value) = schema.get(READ_ONLY) else {
            return Ok(false);
        };
        ensure!(
            matches!(value, Value::Bool(_)),
            InvalidTypeSnafu {
                expected: Expected::Bool,
                actual: Box::new(value.clone())
            }
        );
        Ok(true)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(Some(ctx.annotate(
            Some(READ_ONLY),
            Some(Annotation::StaticRef(boolean(self.value))),
        )))
    }
}
