use std::{borrow::Cow, fmt::Display};

use crate::{
    error::CompileError,
    handler::SyncHandler,
    json_schema::Keyword,
    output::{self, ValidationError},
    Compile, Handler,
};
use serde_json::{Number, Value};

#[derive(Debug, Clone, Default)]
pub struct MultipleOfHandler {}
impl From<MultipleOfHandler> for Handler {
    fn from(handler: MultipleOfHandler) -> Self {
        Self::Sync(Box::new(handler))
    }
}
impl SyncHandler for MultipleOfHandler {
    fn compile<'s>(
        &mut self,
        compile: &mut Compile<'s>,
        schema: &'s Value,
    ) -> Result<bool, CompileError> {
        todo!()
    }

    fn evaluate<'v>(
        &self,
        scope: &mut crate::Scope,
        value: &'v Value,
        _structure: crate::Structure,
    ) -> Result<Option<output::Node<'v>>, crate::error::EvaluateError> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct MultipleOfInvalid<'v> {
    pub expected_multiple_of: Number,
    pub actual: Cow<'v, Number>,
}

impl Display for MultipleOfInvalid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "expected multiple of {}, found {}",
            self.expected_multiple_of, self.actual
        )
    }
}

impl<'v> ValidationError<'v> for MultipleOfInvalid<'v> {
    fn into_owned(self) -> Box<dyn ValidationError<'static>> {
        let MultipleOfInvalid {
            actual,
            expected_multiple_of,
        } = self;

        Box::new(MultipleOfInvalid {
            expected_multiple_of,
            actual: Cow::Owned(actual.into_owned()),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num::{FromPrimitive, Zero};
    use num_rational::BigRational;
    use serde_json::{json, Value};

    use crate::{test, Compile, Handler, Location, Scope, State, Structure};

    use super::*;

    #[test]
    fn test_multiple_of_setup() {
        // let mut handler = MultipleOfHandler::default();
        // let mut compiler = Compiler::default();
        // let schema = serde_json::json!({"multipleOf": 3});
        // let schema: Schema = serde_json::from_value(schema).unwrap();
        // let result = handler.setup(&mut compiler, &schema);
        // assert!(result.is_ok());
        // assert!(result.unwrap());
        // assert_eq!(handler.expected_multiple_of, Some(3.into()));

        // let mut handler = MultipleOfHandler::default();
        // let mut compiler = Compiler::default();
        // let schema = serde_json::json!({});
        // let schema: Schema = serde_json::from_value(schema).unwrap();
        // let result = handler.setup(&mut compiler, &schema);
        // assert!(result.is_ok());
        // assert!(!result.unwrap());
        // assert_eq!(handler.expected_multiple_of, None);
    }

    #[test]
    fn test_multiple_of_evaluate() {
        // let mut handler = MultipleOfHandler::default();
        // let mut compiler = Compiler::default();
        // let schema = serde_json::json!({"multipleOf": 3});
        // let schema: Schema = serde_json::from_value(schema).unwrap();
        // let result = handler.setup(&mut compiler, &schema);
        // assert!(result.is_ok());
        // assert_eq!(handler.expected_multiple_of, Some(3.into()));
        // let mut state = State::default();
        // let mut scope = Scope::new(Location::default(), &mut state);
        // let value = serde_json::json!(3);
        // let result = handler.evaluate(&mut scope, &value, Structure::Complete);
        // assert!(result.is_ok());
        // let result = result.unwrap();
        // assert!(result.is_some());
        // let result = result.unwrap();
        // assert!(result.is_valid());

        // let value = serde_json::json!(21);
        // let result = handler.evaluate(&mut scope, &value, Structure::Complete);
        // assert!(result.is_ok());
        // let result = result.unwrap();
        // assert!(result.is_some());
        // let result = result.unwrap();
        // assert!(result.is_valid());

        // let value = serde_json::json!(34);
        // let result = handler.evaluate(&mut scope, &value, Structure::Complete);
        // assert!(result.is_ok());
        // let result = result.unwrap();
        // assert!(result.is_some());
        // let result = result.unwrap();
        // assert!(result.is_invalid());
    }

    #[test]
    fn test_multiple_of_evaluate_float() {}

    #[test]
    fn spike() {
        // println!("v: {} / y: {}; v % y = {}", &v, &y, &v % &y);

        // let value = BigRational::from_f64(9.9).unwrap();
        // let multiple_of = BigRational::from_f64(3.3).unwrap();
        // // println!("{value} {multiple_of}");
        // let r = &value % &multiple_of;
        // println!("{} % {} = {}", &value, &multiple_of, r);
        // println!("is zero: {}", r.is_zero());
        // println!("is zero: {}", r.to_integer().is_zero());
    }
}
