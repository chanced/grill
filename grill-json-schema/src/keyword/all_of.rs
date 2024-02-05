//! `allOf` keyword.

use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError, UnexpectedValueError},
    keyword::{Compile, Context, Keyword, Kind, Unimplemented},
    output::Output,
    Key, Schema,
};
use jsonptr::Pointer;
use serde_json::Value;

use super::ALL_OF;

/// `allOf` [`Keyword`]
#[derive(Debug, Clone, Default)]
pub struct AllOf {
    /// List of subschemas
    pub keys: Vec<(Pointer, Key)>,
}

impl Keyword for AllOf {
    fn kind(&self) -> Kind {
        Kind::Keyword(ALL_OF)
    }

    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(all_of) = schema.get(ALL_OF) else {
            return Ok(false);
        };
        let Value::Array(all_of) = all_of else {
            return Err(InvalidTypeError {
                expected: Expected::Array,
                actual: Box::new(all_of.clone()),
                backtrace: snafu::Backtrace::capture(),
            }
            .into());
        };
        if all_of.is_empty() {
            return Err(UnexpectedValueError {
                expected: "a non-empty array",
                value: Box::new(schema.value().clone()),
            }
            .into());
        }
        self.keys = all_of
            .iter()
            .enumerate()
            .map(|(i, _)| jsonptr::Pointer::new([ALL_OF, &i.to_string()]))
            .map(|ptr| compile.subschema(&ptr).map(|key| (ptr, key)))
            .collect::<Result<_, _>>()?;
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v serde_json::Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let mut output = ctx.annotate(Some(ALL_OF), None);
        for (path, key) in &self.keys {
            let o = ctx.evaluate(*key, None, path, value)?;
            output.push(o);
            if output.is_invalid() && ctx.should_short_circuit() {
                return Ok(Some(output));
            }
        }
        Ok(Some(output))
    }

    fn subschemas(&self, schema: &Value) -> Result<Vec<jsonptr::Pointer>, Unimplemented> {
        let Some(Value::Array(all_of)) = schema.get(ALL_OF) else {
            return Ok(Vec::default());
        };
        Ok(all_of
            .iter()
            .enumerate()
            .map(|(i, _)| jsonptr::Pointer::new([ALL_OF, &i.to_string()]))
            .collect())
    }
}
