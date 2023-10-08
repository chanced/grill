use std::{borrow::Cow, sync::Arc};

use crate::big::parse_rational;
use crate::keyword::{define_translate, Kind};
use num_rational::BigRational;
use serde_json::Value;

use crate::output::Error as OutputError;

use crate::{
    error::EvaluateError,
    keyword::{self, Compile, Context},
    Output, Schema,
};

/// [`Keyword`](`crate::keyword::Keyword`) for the `const` keyword.
///
/// The value of this keyword MAY be of any type, including null.
///
/// An instance validates successfully against this keyword if its value is
/// equal to the value of the keyword.
#[derive(Clone, Debug)]
pub struct Keyword {
    pub expected: Arc<Value>,
    pub expected_number: Option<Arc<BigRational>>,
    pub translate: Translate,
}

impl Keyword {
    #[must_use]
    pub fn new(translate: Option<Translate>) -> Keyword {
        Self {
            expected: Arc::new(Value::Null),
            expected_number: None,
            translate: translate.unwrap_or(Translate::Pointer(translate_en)),
        }
    }
}
impl keyword::Keyword for Keyword {
    fn kind(&self) -> Kind {
        keyword::CONST.into()
    }
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, crate::error::CompileError> {
        let Some(c) = schema.get(keyword::CONST) else {
            return Ok(false);
        };
        let expected = compile.value(c);
        self.expected = expected;
        if let Value::Number(n) = c {
            let number = compile.number(n)?;
            self.expected_number = Some(number);
        }
        Ok(true)
    }
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        if let Value::Number(n) = value {
            if let Some(expected_number) = self.expected_number.as_deref() {
                let actual_number = parse_rational(n.as_str())?;
                if &actual_number == expected_number {
                    return Ok(Some(ctx.annotate(Cow::Borrowed(value))));
                }
                return Ok(Some(ctx.error(Error {
                    expected: self.expected.clone(),
                    actual: Cow::Borrowed(value),
                    translate: self.translate.clone(),
                })));
            }
        }
        todo!()
    }
}

define_translate!(Error);

pub fn translate_en(f: &mut std::fmt::Formatter<'_>, error: &Error<'_>) -> std::fmt::Result {
    write!(f, "expected {}, found {}", error.expected, error.actual)
}

/// [`ValidationError`](`crate::error::ValidationError`) for the `enum` keyword, produced by [`ConstKeyword`].
#[derive(Clone, Debug)]
pub struct Error<'v> {
    pub expected: Arc<Value>,
    pub actual: Cow<'v, Value>,
    pub translate: Translate,
}

impl<'v> std::fmt::Display for Error<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected {}, found {}", self.actual, self.expected)
    }
}

impl<'v> OutputError<'v> for Error<'v> {
    fn make_owned(self: Box<Self>) -> Box<dyn OutputError<'static>> {
        match self.actual {
            Cow::Borrowed(actual) => Box::new(Error {
                translate: self.translate,
                expected: self.expected,
                actual: Cow::Owned(actual.clone()),
            }) as Box<dyn OutputError<'static>>,
            Cow::Owned(actual) => Box::new(Error {
                actual: Cow::Owned(actual),
                expected: self.expected,
                translate: self.translate,
            }) as Box<dyn OutputError<'static>>,
        }
    }

    fn translate_error(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        lang: &crate::output::Translations,
    ) -> std::fmt::Result {
        if let Some(translate) = lang.get::<Translate>() {
            translate.run(f, self)
        } else {
            self.translate.run(f, self)
        }
    }
}

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
