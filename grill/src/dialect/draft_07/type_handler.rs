use std::str::FromStr;

use itertools::Itertools;

use crate::{
    handler::SyncHandler,
    output::{Annotation, ValidationError},
    schema::{Type, Types},
    type_of,
};

/// The value of this keyword MUST be either a string or an array.  If it
/// is an array, elements of the array MUST be strings and MUST be
/// unique.
///
/// String values MUST be one of the six primitive types ("null",
/// "boolean", "object", "array", "number", or "string"), or "integer"
/// which matches any number with a zero fractional part.
///
/// An instance validates if and only if the instance is in any of the
/// sets listed for this keyword.
///
/// - [Schema Validation 07 # 6.1.1. `type`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-validation-01#section-6.1.1)
#[derive(Debug, Clone)]
pub struct TypeHandler {
    types: Option<Types>,
}

#[derive(Debug, Clone)]
pub struct TypeInvalid<'v> {
    expected: Types,
    value: &'v serde_json::Value,
    actual: Type,
}

impl std::fmt::Display for TypeInvalid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.expected {
            Types::Single(t) => write!(f, r#"expected type "{}", got "{}""#, t, self.actual),
            Types::Set(types) if types.len() == 1 => write!(
                f,
                r#"expected type "{}", got "{}""#,
                types.iter().next().unwrap(),
                self.actual
            ),
            Types::Set(types) => write!(
                f,
                r#"expected one of "{}", got "{}""#,
                types.iter().join(", "),
                self.actual
            ),
        }
    }
}

impl<'v> ValidationError<'v> for TypeInvalid<'v> {}

impl SyncHandler for TypeHandler {
    fn setup<'s>(
        &mut self,
        _compiler: &mut crate::Compiler<'s>,
        schema: &'s crate::Schema,
    ) -> Result<bool, crate::error::SetupError> {
        if let Some(obj) = schema.as_object() {
            if let Some(types) = obj.types.clone() {
                self.types = Some(types);
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn evaluate<'v>(
        &self,
        scope: &mut crate::Scope,
        value: &'v serde_json::Value,
        _output_structure: crate::Structure,
    ) -> Result<Option<Annotation<'v>>, Box<dyn snafu::Error>> {
        let types = self.types.as_ref().expect("types must be set in setup");
        let mut annotation = scope.annotate("type", value);
        if !types.includes(value) {
            annotation.error(TypeInvalid {
                actual: Type::from_str(type_of(value)).unwrap(),
                value,
                expected: types.clone(),
            });
        }
        Ok(Some(annotation))
    }
}
