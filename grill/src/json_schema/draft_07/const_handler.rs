use std::borrow::Cow;

use crate::{
    handler::{Handler, Scope, SyncHandler},
    output,
};

/// [`Handler`](`crate::handler::Handler`) for the `const` keyword.
///
/// The value of this keyword MAY be of any type, including null.
///
/// An instance validates successfully against this keyword if its value is
/// equal to the value of the keyword.
#[derive(Default, Clone, Debug)]
pub struct ConstHandler {
    pub expected: Option<serde_json::Value>,
}

impl ConstHandler {
    #[must_use]
    pub fn new() -> ConstHandler {
        Self::default()
    }
}

impl SyncHandler for ConstHandler {
    fn compile<'i>(
        &'i mut self,
        compile: &mut crate::handler::Compile<'i>,
        schema: crate::Schema<'i>,
    ) -> Result<bool, crate::error::CompileError> {
        todo!()
    }

    fn evaluate<'i, 'v>(
        &'i self,
        scope: &'i mut Scope,
        value: &'v serde_json::Value,
        _structure: crate::Structure,
    ) -> Result<Option<output::Node<'v>>, crate::error::EvaluateError> {
        todo!()
    }
}
impl From<ConstHandler> for Handler {
    fn from(h: ConstHandler) -> Handler {
        Handler::Sync(Box::<ConstHandler>::default())
    }
}
// impl SyncHandler for ConstHandler {
//     fn compile<'s>(
//         &mut self,
//         _compile: &mut crate::handler::Compile<'s>,
//         _schema: &'s serde_json::Value,
//     ) -> Result<bool, crate::error::CompileError> {
//         todo!()
//     }

//     fn evaluate<'v>(
//         &self,
//         _scope: &mut Scope,
//         _value: &'v serde_json::Value,
//         _structure: crate::Structure,
//     ) -> Result<Option<output::Node<'v>>, crate::error::EvaluateError> {
//         todo!()
//     }
// }
// impl From<ConstHandler> for Handler {
//     fn from(value: ConstHandler) -> Self {
//         value.into_handler()
//     }
// }
// impl From<&ConstHandler> for Handler {
//     fn from(value: &ConstHandler) -> Self {
//         value.clone().into_handler()
//     }
// }
// impl SyncHandler for ConstHandler {
//     fn compile<'s>(
//         &mut self,
//         _compiler: &mut crate::Compiler<'s>,
//         schema: &'s Schema,
//     ) -> Result<bool, crate::error::SetupError> {
//         match schema {
//             Schema::Object(obj) if obj.constant.is_some() => {
//                 self.expected = obj.constant.clone();
//                 Ok(true)
//             }
//             _ => Ok(false),
//         }
//     }
//     fn evaluate<'v>(
//         &self,
//         scope: &mut crate::Scope,
//         schema: &CompiledSchema,
//         value: &'v serde_json::Value,
//         _structure: crate::Structure,
//     ) -> Result<Option<crate::output::Annotation<'v>>, Box<dyn snafu::Error>> {
//         let expected = self.expected.as_ref().unwrap();
//         let mut annotation = scope.annotate("const", value);
//         if value != expected {
//             annotation.error(ConstInvalid {
//                 actual: value,
//                 expected: expected.clone(),
//             });
//         }
//         Ok(Some(annotation))
//     }
// }

/// [`ValidationError`](`crate::error::ValidationError`) for the `enum` keyword, produced by [`ConstHandler`].
#[derive(Clone, Debug)]
pub struct ConstInvalid<'v> {
    pub expected: serde_json::Value,
    pub actual: Cow<'v, serde_json::Value>,
}
// impl Display for ConstInvalid<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "expected {}, found {}", self.actual, self.expected)
//     }
// }
// impl<'v> ValidationError<'v> for ConstInvalid<'v> {}

// #[cfg(test)]
// mod tests {
//     use serde_json::json;

//     use crate::{Location, State, Structure};

//     use super::*;

//     #[test]
//     fn test_const_setup() {
//         let mut compiler = crate::Compiler::default();
//         let schema: Schema = serde_json::from_value(json!({"const": 1})).unwrap();
//         let mut handler = ConstHandler::default();
//         assert!(handler.compile(&mut compiler, &schema).unwrap());
//         assert_eq!(handler.expected, Some(serde_json::json!(1)));

//         let schema: Schema = serde_json::from_value(json!({})).unwrap();
//         let mut handler = ConstHandler::default();
//         assert!(!handler.compile(&mut compiler, &schema).unwrap());
//     }

//     #[test]
//     fn test_const_evaluate() {
//         let mut compiler = crate::Compiler::default();
//         let schema: Schema = serde_json::from_value(json!({"const": 1})).unwrap();
//         let mut handler = ConstHandler::default();
//         handler.compile(&mut compiler, &schema).unwrap();
//         let mut state = State::new();
//         let mut scope = crate::Scope::new(Location::default(), &mut state);
//         let value = serde_json::json!(1);
//         let result = handler.evaluate(&mut scope, &value, Structure::Complete);
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(result.is_some());
//         let result = result.unwrap();
//         assert!(result.is_valid());
//         let value = serde_json::json!(2);
//         let result = handler.evaluate(&mut scope, &value, Structure::Complete);
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(result.is_some());
//         let result = result.unwrap();
//         assert!(result.is_invalid());
//     }
//     #[test]
//     fn test_const_obj() {
//         let mut compiler = crate::Compiler::default();
//         let schema: Schema =
//             serde_json::from_value(json!({"const": {"a": "a", "b": "b"}})).unwrap();
//         let mut handler = ConstHandler::default();
//         handler.compile(&mut compiler, &schema).unwrap();
//         let mut state = State::new();
//         let mut scope = crate::Scope::new(Location::default(), &mut state);

//         let value = serde_json::json!({"b": "b", "a":"a"});
//         let result = handler.evaluate(&mut scope, &value, Structure::Complete);
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(result.is_some());
//         let result = result.unwrap();
//         assert!(result.is_valid());
//     }
// }
