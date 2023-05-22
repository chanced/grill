use std::{error::Error, fmt::Display};

use itertools::Itertools;

use crate::{
    handler::SyncHandler,
    output::{Annotation, ValidationError},
    type_of, Schema,
};

#[derive(Debug, Clone)]
pub struct EnumInvalid<'v> {
    expected: Vec<serde_json::Value>,
    actual: &'v serde_json::Value,
}
impl Display for EnumInvalid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.expected.len() {
            0 => panic!("expected is empty"),
            1 => write!(
                f,
                "expected value to be a {}, found {}",
                self.expected[0],
                type_of(self.actual)
            ),
            _ => write!(
                f,
                "expected value to be one of [{:?}], found {}",
                self.expected.iter().join(", "),
                type_of(self.actual)
            ),
        }
    }
}
impl<'v> ValidationError<'v> for EnumInvalid<'v> {}
/// The value of this keyword MUST be an array.  This array SHOULD have
/// at least one element.  Elements in the array SHOULD be unique.
///
/// An instance validates successfully against this keyword if its value
/// is equal to one of the elements in this keyword's array value.

/// Elements in the array might be of any value, including null.
///
/// - [JSON Schema Validation 07 # 6.1.2. `enum`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-validation-01#section-6.1.2)
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
        _output_structure: crate::Structure,
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
