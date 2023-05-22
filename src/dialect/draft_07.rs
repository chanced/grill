pub mod handler {
    use itertools::Itertools;
    use std::str::FromStr;

    use crate::{
        handler::SyncHandler,
        output::ValidationError,
        schema::{Type, Types},
        type_of,
    };

    #[derive(Debug, Clone)]
    pub struct TypeHandler {
        types: Option<Types>,
    }

    #[derive(Debug, Clone)]
    pub struct TypeInvalid {
        expected: Types,
        actual: Type,
    }

    impl std::fmt::Display for TypeInvalid {
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

    impl ValidationError<'_> for TypeInvalid {}

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
        ) -> Result<Option<crate::output::Annotation<'v>>, Box<dyn snafu::Error>> {
            let types = self.types.as_ref().expect("types must be set in setup");
            let mut annotation = scope.annotate("type", value);
            if !types.includes(value) {
                annotation.error(TypeInvalid {
                    actual: Type::from_str(type_of(value)).unwrap(),
                    expected: types.clone(),
                });
            }
            Ok(Some(annotation))
        }
    }
}
