use crate::ISSUES_URL;
use itertools::Itertools;
use std::{error::Error, fmt::Display};

use crate::{
    handler::SyncHandler,
    output::{Annotation, ValidationError},
    schema::Types,
    Schema,
};

impl<'v> ValidationError<'v> for EnumInvalid<'v> {}
/// [`Handler`](`crate::handler::Handler`) for the `enum` keyword.
///
/// An instance validates successfully against this keyword if its value is
/// equal to one of the elements in this keyword's array value.
///
/// The value of this keyword MUST be an array.  This array SHOULD have at least
/// one element.  Elements in the array SHOULD be unique.
///
/// An instance validates successfully against this keyword if its value is
/// equal to one of the elements in this keyword's array value.
///
/// Elements in the array might be of any value, including null.
///
/// - [JSON Schema Validation 07 # 6.1.2.
///   `enum`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-validation-01#section-6.1.2)
#[derive(Debug, Clone, Default)]
pub struct EnumHandler {
    expected: Vec<serde_json::Value>,
}

impl SyncHandler for EnumHandler {
    fn setup<'s>(
        &mut self,
        _compiler: &mut crate::Compiler<'s>,
        schema: &'s Schema,
    ) -> Result<bool, crate::error::SetupError> {
        match schema {
            Schema::Bool(_) => Ok(false),
            Schema::Object(obj) if obj.enumeration.is_empty() => Ok(false),
            Schema::Object(obj) => {
                self.expected = obj.enumeration.clone();
                Ok(true)
            }
        }
    }

    fn evaluate<'v>(
        &self,
        scope: &mut crate::Scope,
        value: &'v serde_json::Value,
        _structure: crate::Structure,
    ) -> Result<Option<Annotation<'v>>, Box<dyn Error>> {
        let mut annotation = scope.annotate("enum", value);
        if !self.expected.contains(value) {
            annotation.error(EnumInvalid {
                actual: value,
                expected: self.expected.clone(),
            });
        }

        Ok(Some(annotation))
    }
}

/// [`ValidationError`] for the `enum` keyword, produced by [`EnumHandler`].
#[derive(Debug, Clone)]
pub struct EnumInvalid<'v> {
    expected: Vec<serde_json::Value>,
    actual: &'v serde_json::Value,
}

impl Display for EnumInvalid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.expected.len() {
            0 => panic!(
                "EnumInvalid: expected is empty. This is a bug. Please report it to {ISSUES_URL}"
            ),
            1 => write!(
                f,
                "expected value to be a {}, found {}",
                self.expected[0],
                Types::of_value(self.actual)
            ),
            _ => write!(
                f,
                "expected value to be one of [{:?}], found {}",
                self.expected.iter().join(", "),
                Types::of_value(self.actual)
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{Scope, State};

    use super::*;

    #[test]
    fn test_setup_succeeds() {
        let mut handler = EnumHandler::default();
        let mut compiler = crate::Compiler::default();
        let schema = serde_json::json!({"enum": [1, 2, 3]});
        let schema: Schema = serde_json::from_value(schema).unwrap();
        let result = handler.setup(&mut compiler, &schema);
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert_eq!(handler.expected, vec![1, 2, 3]);
    }
    #[test]
    fn test_evaluate() {
        let mut handler = EnumHandler::default();
        let mut compiler = crate::Compiler::default();
        let schema = serde_json::json!({"enum": [1, 2, 3]});
        let schema: Schema = serde_json::from_value(schema).unwrap();
        handler.setup(&mut compiler, &schema).unwrap();
        let mut state = State::new();
        let mut scope = Scope::new(crate::Location::default(), &mut state);
        let one = serde_json::json!(1);
        let result = handler.evaluate(&mut scope, &one, crate::Structure::Complete);
        assert!(result.is_ok());
        let annotation = result.unwrap();
        assert!(annotation.is_some());
        let annotation = annotation.unwrap();
        assert!(annotation.nested_errors().is_empty());
    }
}
