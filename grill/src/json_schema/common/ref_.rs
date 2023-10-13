use keyword::Unimplemented;
use serde_json::Value;

use crate::{
    error::{CompileError, EvaluateError, Expected, RefError, UnexpectedTypeError},
    keyword::{self, Compile, Context, Kind},
    schema::Ref,
    Key, Output, Schema,
};

#[derive(Debug, Clone, Default)]
pub struct Keyword {
    pub keyword: &'static str,
    /// The key of the referenced schema.
    pub ref_key: Key,
}

impl keyword::Keyword for Keyword {
    fn kind(&self) -> Kind {
        Kind::Single(self.keyword)
    }
    fn setup<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(_v) = schema.get(self.keyword) else {
            return Ok(false);
        };
        todo!()
    }
    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        todo!()
    }
    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    fn refs(&self, schema: &Value) -> Result<Result<Vec<Ref>, RefError>, Unimplemented> {
        let Some(v) = schema.get(self.keyword) else {
            return Ok(Ok(Vec::default()));
        };
        let Value::String(_v) = v else {
            return Ok(Err(UnexpectedTypeError {
                expected: Expected::String,
                actual: v.clone(),
            }
            .into()));
        };
        todo!()
    }
}
