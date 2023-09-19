use std::borrow::Cow;

use either::Either;
use serde_json::Value;

use crate::{
    keyword,
    keyword::{Compile, Context, Keyword, RationalKey, ValueKey},
    output::{self},
    Schema,
};

/// [`Keyword`](`crate::keyword::Keyword`) for the `const` keyword.
///
/// The value of this keyword MAY be of any type, including null.
///
/// An instance validates successfully against this keyword if its value is
/// equal to the value of the keyword.
#[derive(Default, Clone, Debug)]
pub struct ConstKeyword {
    pub expected_key: Option<Either<ValueKey, RationalKey>>,
}

impl ConstKeyword {
    #[must_use]
    pub fn new() -> ConstKeyword {
        Self::default()
    }
}

impl Keyword for ConstKeyword {
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, crate::error::CompileError> {
        let Some(c) = schema.get(keyword::CONST) else { return Ok(false) };
        if let Value::Number(n) = c {
            let rat = compile.rational(n)?;
            self.expected_key = Some(Either::Right(rat));
        } else {
            let val = compile.value(c);
            self.expected_key = Some(Either::Left(val));
        }
        Ok(true)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v Value,
        _structure: crate::Structure,
    ) -> Result<Option<output::Node<'v>>, crate::error::EvaluateError> {
        todo!()
    }
}

// impl SyncKeyword for ConstKeyword {
//     fn compile<'s>(
//         &mut self,
//         _compile: &mut crate::keyword::Compile<'s>,
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
// impl From<ConstKeyword> for Keyword {
//     fn from(value: ConstKeyword) -> Self {
//         value.into_keyword()
//     }
// }
// impl From<&ConstKeyword> for Keyword {
//     fn from(value: &ConstKeyword) -> Self {
//         value.clone().into_keyword()
//     }
// }
// impl SyncKeyword for ConstKeyword {
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

/// [`ValidationError`](`crate::error::ValidationError`) for the `enum` keyword, produced by [`ConstKeyword`].
#[derive(Clone, Debug)]
pub struct ConstInvalid<'v> {
    pub expected: Value,
    pub actual: Cow<'v, Value>,
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
//         let mut keyword = ConstKeyword::default();
//         assert!(keyword.compile(&mut compiler, &schema).unwrap());
//         assert_eq!(keyword.expected, Some(serde_json::json!(1)));

//         let schema: Schema = serde_json::from_value(json!({})).unwrap();
//         let mut keyword = ConstKeyword::default();
//         assert!(!keyword.compile(&mut compiler, &schema).unwrap());
//     }

//     #[test]
//     fn test_const_evaluate() {
//         let mut compiler = crate::Compiler::default();
//         let schema: Schema = serde_json::from_value(json!({"const": 1})).unwrap();
//         let mut keyword = ConstKeyword::default();
//         keyword.compile(&mut compiler, &schema).unwrap();
//         let mut state = State::new();
//         let mut scope = crate::Scope::new(Location::default(), &mut state);
//         let value = serde_json::json!(1);
//         let result = keyword.evaluate(&mut scope, &value, Structure::Complete);
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(result.is_some());
//         let result = result.unwrap();
//         assert!(result.is_valid());
//         let value = serde_json::json!(2);
//         let result = keyword.evaluate(&mut scope, &value, Structure::Complete);
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
//         let mut keyword = ConstKeyword::default();
//         keyword.compile(&mut compiler, &schema).unwrap();
//         let mut state = State::new();
//         let mut scope = crate::Scope::new(Location::default(), &mut state);

//         let value = serde_json::json!({"b": "b", "a":"a"});
//         let result = keyword.evaluate(&mut scope, &value, Structure::Complete);
//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert!(result.is_some());
//         let result = result.unwrap();
//         assert!(result.is_valid());
//     }
// }
