//! # `$defs` keyword.
//!
//! - [Learn JSON Schema - $defs](https://www.learnjsonschema.com/2020-12/core/defs/)
use grill_core::{
    error::{CompileError, EvaluateError},
    keyword::{self, paths_of_object, Compile, Kind},
    Output, Schema,
};
use serde_json::Value;

use super::DEFS;

/// [`Keyword`] for `$defs`
#[derive(Debug, Clone, Default)]
pub struct Defs;

impl keyword::Keyword for Defs {
    fn kind(&self) -> Kind {
        Kind::Keyword(DEFS)
    }
    fn compile<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        _schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        Ok(false)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(None)
    }
    fn subschemas(&self, schema: &Value) -> Result<Vec<jsonptr::Pointer>, keyword::Unimplemented> {
        let subschemas = paths_of_object(DEFS, schema);
        println!("subschemas: {subschemas:?}");
        Ok(subschemas)
    }
}
