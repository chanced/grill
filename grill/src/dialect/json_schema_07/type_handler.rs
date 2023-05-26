use itertools::Itertools;

use crate::{
    handler::SyncHandler,
    output::{Annotation, ValidationError},
    schema::Types,
};
/// [`Handler`](`crate::handler::Handler`) for the `type` keyword.
///
/// `type` is fundamental to JSON Schema. It specifies the data type for a
/// schema.
///
/// The value of this keyword MUST be either a string or an array.  If it is an
/// array, elements of the array MUST be strings and MUST be unique.
///
/// String values MUST be one of the six primitive types ("null", "boolean",
/// "object", "array", "number", or "string"), or "integer" which matches any
/// number with a zero fractional part.
///
/// An instance validates if and only if the instance is in any of the sets
/// listed for this keyword.
///
/// - [Schema Validation 07 # 6.1.1.
///   `type`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-validation-01#section-6.1.1)
#[derive(Clone)]
pub struct TypeHandler<F> {
    get_types: F,
    expected: Option<Types>,
}

impl Default for TypeHandler<fn(&serde_json::Value) -> Types> {
    fn default() -> Self {
        Self {
            get_types: Types::of_value,
            expected: None,
        }
    }
}

impl<F> std::fmt::Debug for TypeHandler<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeHandler")
            .field("types", &self.expected)
            .finish_non_exhaustive()
    }
}

/// [`ValidationError`] for the `type` keyword, produced by [`TypeHandler`].
#[derive(Debug, Clone)]
pub struct TypeInvalid<'v> {
    pub expected: Types,
    pub actual: Types,
    pub value: &'v serde_json::Value,
}

impl std::fmt::Display for TypeInvalid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.expected {
            Types::Single(t) => write!(f, r#"expected type "{}", got "{}""#, t, self.actual),
            Types::Set(types) if types.len() == 1 => write!(
                f,
                r#"expected type "{}", found {}"#,
                types.iter().next().unwrap(),
                self.actual.iter().map(|t| format!("{t}")).join(", ")
            ),
            Types::Set(types) => write!(
                f,
                r#"expected one of {} found {}"#,
                types.iter().map(|t| format!("{t}")).join(", "),
                self.actual.iter().map(|t| format!("{t}")).join(", ")
            ),
        }
    }
}
impl<'v> ValidationError<'v> for TypeInvalid<'v> {}

impl<F> SyncHandler for TypeHandler<F>
where
    F: 'static + Send + Sync + Clone + Fn(&serde_json::Value) -> Types,
{
    fn compile<'s>(
        &mut self,
        _compiler: &mut crate::Compiler<'s>,
        schema: &'s crate::Schema,
    ) -> Result<bool, crate::error::SetupError> {
        if let Some(obj) = schema.as_object() {
            if let Some(types) = obj.types.as_ref() {
                if types.is_empty() {
                    return Ok(false);
                }
                self.expected = Some(types.clone());
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn evaluate<'v>(
        &self,
        scope: &mut crate::Scope,
        value: &'v serde_json::Value,
        _structure: crate::Structure,
    ) -> Result<Option<Annotation<'v>>, Box<dyn snafu::Error>> {
        let expected = self.expected.as_ref().expect("types must be set in setup");
        let mut annotation = scope.annotate("type", value);
        let actual = (self.get_types)(value);
        if !expected.contains_any(&actual) {
            annotation.error(TypeInvalid {
                actual,
                value,
                expected: expected.clone(),
            });
        }
        Ok(Some(annotation))
    }
}

#[cfg(test)]
mod tests {

    use crate::{Schema, Scope, State};

    use super::*;

    #[test]
    fn test_setup_succeeds() {
        let mut handler = TypeHandler::default();
        let mut compiler = crate::Compiler::default();
        let schema = serde_json::json!({"type": ["string", "number"]});
        let schema: Schema = serde_json::from_value(schema).unwrap();
        let result = handler.compile(&mut compiler, &schema);
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert_eq!(
            handler.expected,
            Some(Types::from(vec!["string", "number"]))
        );
    }

    #[test]
    fn test_successful_evaluate() {
        let mut handler = TypeHandler::default();
        let mut compiler = crate::Compiler::default();
        let schema = serde_json::json!({"type": ["null", "number"]});
        let schema: Schema = serde_json::from_value(schema).unwrap();
        handler.compile(&mut compiler, &schema).unwrap();
        let mut state = State::new();
        let mut scope = Scope::new(crate::Location::default(), &mut state);
        let one = serde_json::json!(1.1);
        let result = handler.evaluate(&mut scope, &one, crate::Structure::Complete);
        assert!(result.is_ok());
        let annotation = result.unwrap();
        assert!(annotation.is_some());
        let annotation = annotation.unwrap();
        assert!(annotation.nested_errors().is_empty());
        assert!(annotation.is_valid());
    }

    #[test]
    fn test_failed_evaluate() {
        let mut handler = TypeHandler::default();
        let mut compiler = crate::Compiler::default();
        let schema = serde_json::json!({"type": ["null", "string"]});
        let schema: Schema = serde_json::from_value(schema).unwrap();
        handler.compile(&mut compiler, &schema).unwrap();
        let mut state = State::new();
        let mut scope = Scope::new(crate::Location::default(), &mut state);
        let one = serde_json::json!(1.1);
        let result = handler.evaluate(&mut scope, &one, crate::Structure::Complete);
        assert!(result.is_ok());
        let annotation = result.unwrap();
        assert!(annotation.is_some());
        let annotation = annotation.unwrap();
        assert!(annotation.nested_errors().is_empty());
        assert!(annotation.is_invalid());
    }
}
