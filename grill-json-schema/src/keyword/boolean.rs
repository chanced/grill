//! Handles schemas which are boolean values.

use grill_core::keyword::{Keyword, Kind};

/// Handles schemas which are boolean values.
#[derive(Debug, Clone, Default)]
pub struct Boolean {
    bool: bool,
}
impl Keyword for Boolean {
    fn kind(&self) -> Kind {
        Kind::BooleanSchema(self.bool)
    }

    fn compile<'i>(
        &mut self,
        _compile: &mut grill_core::keyword::Compile<'i>,
        schema: grill_core::Schema<'i>,
    ) -> Result<bool, grill_core::error::CompileError> {
        if let Some(b) = schema.as_bool() {
            self.bool = b;
        }
        Ok(schema.is_boolean())
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut grill_core::keyword::Context,
        _value: &'v serde_json::Value,
    ) -> Result<Option<grill_core::output::Output<'v>>, grill_core::error::EvaluateError> {
        if self.bool {
            Ok(Some(ctx.annotate(None, None)))
        } else {
            Ok(Some(ctx.error(None, None)))
        }
    }
}
