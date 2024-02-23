//! `anyOf` keyword.

use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError, UnexpectedValueError},
    keyword::{Compile, Context, Keyword, Kind, Unimplemented},
    output::Output,
    Key, Schema,
};
use jsonptr::Pointer;
use serde_json::Value;

use super::ANY_OF;

/// `anyOf` [`Keyword`]
#[derive(Debug, Clone, Default)]
pub struct AnyOf {
    /// List of subschemas
    pub keys: Vec<(Pointer, Key)>,
}

impl Keyword for AnyOf {
    fn kind(&self) -> Kind {
        Kind::Keyword(ANY_OF)
    }

    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(any_of) = schema.get(ANY_OF) else {
            return Ok(false);
        };
        let Value::Array(any_of) = any_of else {
            return Err(InvalidTypeError {
                expected: Expected::Array,
                actual: Box::new(any_of.clone()),
            }
            .into());
        };
        if any_of.is_empty() {
            return Err(UnexpectedValueError {
                expected: "a non-empty array",
                value: Box::new(schema.value().clone()),
            }
            .into());
        }
        self.keys = any_of
            .iter()
            .enumerate()
            .map(|(i, _)| jsonptr::Pointer::new([ANY_OF, &i.to_string()]))
            .map(|ptr| compile.subschema(&ptr).map(|key| (ptr, key)))
            .collect::<Result<_, _>>()?;
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v serde_json::Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let mut output = ctx.annotate(Some(ANY_OF), None);
        output.set_valid(false);
        for (path, key) in &self.keys {
            let o = ctx.evaluate(*key, None, path, value)?;
            let is_valid = output.is_valid() || o.is_valid();
            output.push(o);
            output.set_valid(is_valid);
            if is_valid && ctx.should_short_circuit() {
                return Ok(Some(output));
            }
        }
        Ok(Some(output))
    }

    fn subschemas(&self, schema: &Value) -> Result<Vec<jsonptr::Pointer>, Unimplemented> {
        let Some(Value::Array(any_of)) = schema.get(ANY_OF) else {
            return Ok(Vec::default());
        };
        Ok(any_of
            .iter()
            .enumerate()
            .map(|(i, _)| jsonptr::Pointer::new([ANY_OF, &i.to_string()]))
            .collect())
    }
}
