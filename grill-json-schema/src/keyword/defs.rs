//! # `$defs` keyword.
//!
//! - [Learn JSON Schema - $defs](https://www.learnjsonschema.com/2020-12/core/defs/)
use std::collections::HashMap;

use grill_core::{
    error::{CompileError, EvaluateError, Expected, InvalidTypeError},
    keyword::{self, paths_of_object, Compile, Keyword, Kind},
    keyword_fns, Eval, Key, Schema,
};
use serde_json::Value;

use super::DEFS;

/// [`Keyword`] for `$defs`
#[derive(Debug, Clone, Default)]
pub struct Defs {
    /// a map of `$defs` properties to their corresponding compiled schema
    /// [`Key`]
    pub defs: HashMap<String, Key>,
}
keyword_fns!(Defs);

impl Keyword for Defs {
    fn kind(&self) -> Kind {
        Kind::Keyword(DEFS)
    }
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(defs) = schema.get(DEFS) else {
            return Ok(false);
        };
        if !defs.is_object() {
            return Err(InvalidTypeError {
                expected: Expected::Object,
                actual: Box::new(defs.clone()),
            }
            .into());
        };

        for mut path in paths_of_object(DEFS, &schema) {
            let key = compile.subschema(&path)?;
            let name = path.pop_back().unwrap().decoded().to_string();
            self.defs.insert(name, key);
        }

        Ok(true)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<Eval<'v>>, EvaluateError> {
        Ok(None)
    }

    fn subschemas(&self, schema: &Value) -> Result<Vec<jsonptr::Pointer>, keyword::Unimplemented> {
        let subschemas = paths_of_object(DEFS, schema);
        Ok(subschemas)
    }
}
