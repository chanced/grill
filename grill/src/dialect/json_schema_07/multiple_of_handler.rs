use std::fmt::Display;

use crate::{handler::SyncHandler, output::ValidationError, schema::CompiledSchema, Schema};
use num::{FromPrimitive, Zero};
use num_rational::BigRational;
use serde_json::Number;

#[derive(Debug, Clone, Default)]
pub struct MultipleOfHandler {}

impl SyncHandler for MultipleOfHandler {
    fn compile<'s>(
        &mut self,
        _compiler: &mut crate::Compiler<'s>,
        schema: &'s Schema,
    ) -> Result<bool, crate::error::SetupError> {
        match schema {
            Schema::Object(obj) if obj.multiple_of.is_some() => {
                // self.expected_multiple_of = obj.multiple_of.clone();
                let multiple_of = obj.multiple_of.as_ref().unwrap();
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn evaluate<'v>(
        &self,
        scope: &mut crate::Scope,
        schema: &CompiledSchema,
        value: &'v serde_json::Value,
        _structure: crate::Structure,
    ) -> Result<Option<crate::output::Annotation<'v>>, Box<dyn snafu::Error>> {
        // match value {
        //     serde_json::Value::Number(number) => {
        //         let expected_multiple_of = self.expected_multiple_of.clone().unwrap();
        //         let mut annotation = scope.annotate("multipleOf", value);

        //         if let Some(multiple_of) = self.expected_multiple_of.as_ref() {
        //             if let Some(instance) = number.as_u64() {
        //             } else if let Some(instance) = number.as_i64() {
        //                 if instance.unsigned_abs() {
        //                     return Ok(Some(annotation));
        //                 }
        //             } else if let Some(instance) = number.as_f64() {
        //                 if let Some(instance_fraction) = BigRational::from_f64(instance) {
        //                     if instance_fraction % multiple_of == BigRational::zero() {
        //                         return Ok(Some(annotation));
        //                     }
        //                 }
        //             }
        //             annotation.error(MultipleOfInvalid {
        //                 expected_multiple_of,
        //                 actual: number,
        //             });
        //             Ok(Some(annotation))
        //         } else if let Some(multiple_of) = expected_multiple_of.as_u64() {
        //             if let Some(instance) = number.as_u64() {
        //                 if instance % multiple_of == 0 {
        //                     return Ok(Some(annotation));
        //                 }
        //             } else if let Some(instance) = number.as_i64() {
        //                 if instance.unsigned_abs() % multiple_of == 0 {
        //                     return Ok(Some(annotation));
        //                 }
        //             }
        //             annotation.error(MultipleOfInvalid {
        //                 expected_multiple_of,
        //                 actual: number,
        //             });
        //             Ok(Some(annotation))
        //         } else if let Some(multiple_of) = expected_multiple_of.as_i64() {
        //             if let Some(instance) = number.as_i64() {
        //                 if instance % multiple_of == 0 {
        //                     return Ok(Some(annotation));
        //                 }
        //             } else if let Some(instance) = number.as_u64() {
        //                 if instance % multiple_of.unsigned_abs() == 0 {
        //                     return Ok(Some(annotation));
        //                 }
        //             }
        //             annotation.error(MultipleOfInvalid {
        //                 expected_multiple_of,
        //                 actual: number,
        //             });
        //             return Ok(Some(annotation));
        //         } else {
        //             Ok(None)
        //         }
        //     }
        //     _ => Ok(None),
        // }
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct MultipleOfInvalid<'v> {
    pub expected_multiple_of: Number,
    pub actual: &'v Number,
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

impl<'v> ValidationError<'v> for MultipleOfInvalid<'v> {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num::{FromPrimitive, Zero};
    use num_rational::BigRational;
    use serde_json::{json, Value};

    use crate::{Compiler, Location, Scope, State, Structure};

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
    fn test_multiple_of_evaluate_float() {
        // let mut handler = MultipleOfHandler::default();
        // let mut compiler = Compiler::default();
        // let mut state = State::default();
        // let mut scope = Scope::new(Location::default(), &mut state);

        // let schema = serde_json::json!({"multipleOf": 3.3});
        // let schema: Schema = serde_json::from_value(schema).unwrap();
        // let result = handler.setup(&mut compiler, &schema);
        // assert!(result.is_ok());
        // assert_eq!(
        //     handler.expected_multiple_of,
        //     serde_json::Number::from_f64(3.3)
        // );
        // let value = serde_json::json!(9.9);
        // let result = handler.evaluate(&mut scope, &value, Structure::Complete);
        // assert!(result.is_ok());
        // let result = result.unwrap();
        // assert!(result.is_some());
        // let result = result.unwrap();
        // println!(
        //     "RESULT:{}",
        //     result.as_invalid().unwrap().error.as_ref().unwrap()
        // );
        // assert!(result.is_valid());
    }

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
