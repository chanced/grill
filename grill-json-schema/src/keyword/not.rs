//! # `not` keyword.
//!
//! - [Learn JSON Schema - `not`](https://www.learnjsonschema.com/2020-12/applicator/not/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.2.1.4)
//!
use super::NOT;
use grill_core::{
    error::{CompileError, EvaluateError},
    keyword::{self, Compile, Keyword},
    Key, Output, Schema,
};
use jsonptr::Pointer;
use once_cell::sync::Lazy;
use serde_json::Value;

/// [`Keyword`] for `"not"`
#[derive(Debug, Clone, Default)]
pub struct Not {
    /// `Key` of the schema
    pub key: Key,
}

impl Keyword for Not {
    fn kind(&self) -> keyword::Kind {
        keyword::Kind::Keyword(NOT)
    }
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        if schema.get(NOT).is_none() {
            return Ok(false);
        };
        self.key = compile.subschema(not_path())?;
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut keyword::Context,
        value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let mut output = ctx.evaluate(self.key, None, not_path(), value)?;
        output.set_valid(output.is_error());
        Ok(Some(output))
    }

    fn subschemas(&self, schema: &Value) -> Result<Vec<jsonptr::Pointer>, keyword::Unimplemented> {
        if schema.get(NOT).is_some() {
            return Ok(vec![not_path().clone()]);
        };
        Ok(Vec::default())
    }
}

fn not_path() -> &'static Pointer {
    static NOT: Lazy<Pointer> = Lazy::new(|| Pointer::new(["/not"]));
    &NOT
}
