use std::{borrow::Cow, sync::Arc};

use crate::big::parse_rational;
use crate::json_schema;
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
        json_schema::CONST.into()
    }
    fn setup<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, crate::error::CompileError> {
        let Some(c) = schema.get(json_schema::CONST) else {
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
                    return Ok(Some(ctx.annotate(Some(value.into()))));
                }
                return Ok(Some(ctx.error(Error {
                    expected: self.expected.clone(),
                    actual: Cow::Borrowed(value),
                    translate: self.translate.clone(),
                })));
            }
        }
        if self.expected.as_ref() == value {
            Ok(Some(ctx.annotate(Some(value.into()))))
        } else {
            Ok(Some(ctx.error(Error {
                expected: self.expected.clone(),
                actual: Cow::Borrowed(value),
                translate: self.translate.clone(),
            })))
        }
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
        translator: &crate::output::Translator,
    ) -> std::fmt::Result {
        if let Some(translate) = translator.get::<Translate>() {
            translate.run(f, self)
        } else {
            self.translate.run(f, self)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        json_schema::{
            common::{id, schema},
            draft_2020_12::json_schema_2020_12_uri,
        },
        schema::Dialect,
        Interrogator, Structure,
    };

    use super::{Keyword, *};
    async fn create_interrogator(const_value: Value) -> Interrogator {
        let dialect = Dialect::builder(json_schema_2020_12_uri().clone())
            .keyword(schema::Keyword::new(json_schema::SCHEMA, false))
            .keyword(id::Keyword::new(json_schema::ID, false))
            .keyword(Keyword::new(None))
            .metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
            .build()
            .unwrap();
        Interrogator::builder()
            .dialect(dialect)
            .source_value(
                "https://example.com/with_const",
                Cow::Owned(json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "const": const_value
                })),
            )
            .unwrap()
            .source_value(
                "https://example.com/without_const",
                Cow::Owned(json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                })),
            )
            .unwrap()
            .build()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_setup() {
        let mut interrogator = create_interrogator(json!(34.34)).await;
        let key = interrogator
            .compile("https://example.com/with_const")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();
        assert!(schema
            .keywords
            .iter()
            .map(|kw| kw.kind())
            .any(|k| k == json_schema::CONST));
        let key = interrogator
            .compile("https://example.com/without_const")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();
        assert!(!schema
            .keywords
            .iter()
            .map(|kw| kw.kind())
            .any(|k| k == json_schema::CONST));
    }

    #[tokio::test]
    async fn test_const_evaluate() {
        let mut interrogator = create_interrogator(json!(34.34)).await;
        let key = interrogator
            .compile("https://example.com/with_const")
            .await
            .unwrap();
        let value = json!(34.34);
        let output = interrogator
            .evaluate(key, Structure::Verbose, &value)
            .unwrap();
        assert!(output.is_valid());
        let value = json!(34.3434);
        let output = interrogator
            .evaluate(key, Structure::Verbose, &value)
            .unwrap();
        assert!(!output.is_valid());
    }
    #[test]
    fn test_const_obj() {}
}
