use grill_core::{
    error::{CompileError, EvaluateError},
    keyword::{static_pointer_fn, Compile, Context, Keyword, Kind},
    output::Output,
    Key, Schema,
};
use serde_json::Value;

use crate::{ELSE, IF, THEN};

static_pointer_fn!(pub if "/if");
static_pointer_fn!(pub then "/then");
static_pointer_fn!(pub else "/else");

#[derive(Debug, Clone, Default)]
pub struct IfThenElse {
    pub if_key: Key,
    pub then_key: Option<Key>,
    pub else_key: Option<Key>,
}

impl Keyword for IfThenElse {
    fn kind(&self) -> Kind {
        Kind::Composite(&[IF, THEN, ELSE])
    }

    fn setup<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        if schema.get(IF).is_none() {
            return Ok(false);
        };
        self.if_key = compile.subschema(if_pointer())?;
        if schema.get(THEN).is_some() {
            self.then_key = Some(compile.subschema(then_pointer())?);
        }
        if schema.get(ELSE).is_some() {
            self.else_key = Some(compile.subschema(else_pointer())?);
        }
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        todo!()
    }
}

impl IfThenElse {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grill_core::schema;
    use grill_core::test;
    use grill_core::Interrogator;
    use serde_json::json;

    #[test]
    fn teset_setup() {
        test::build_dialect();
        let schema = json!({"if": {} });
        let interrogator = test::build_interrogator(
            [("https://example.com/schema", schema)],
            [IfThenElse::default()],
        );
    }
}
