use std::{borrow::Cow, sync::Arc};

use grill_core::{
    big::{parse_rational, BigRational},
    error::CompileError,
    keyword::{define_translate, Kind},
};

use grill_core::output::Error as OutputError;
use serde_json::Value;

use grill_core::{
    error::EvaluateError,
    keyword::{self, Compile, Context},
    Output, Schema,
};

use crate::CONST;

define_translate!(ConstInvalid, translate_const_invalid_en);

/// [`Keyword`](`crate::keyword::Keyword`) for the `const` keyword.
///
/// The value of this keyword MAY be of any type, including null.
///
/// An instance validates successfully against this keyword if its value is
/// equal to the value of the keyword.
#[derive(Clone, Debug)]
pub struct Const {
    pub expected: Arc<Value>,
    pub expected_number: Option<Arc<BigRational>>,
    pub translate: TranslateConstInvalid,
}

impl Const {
    #[must_use]
    pub fn new(translate: Option<TranslateConstInvalid>) -> Const {
        Self {
            expected: Arc::new(Value::Null),
            expected_number: None,
            translate: translate
                .unwrap_or(TranslateConstInvalid::Pointer(translate_const_invalid_en)),
        }
    }
}
impl keyword::Keyword for Const {
    fn kind(&self) -> Kind {
        CONST.into()
    }
    fn setup<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        println!("setup const");
        println!("{}", schema.absolute_uri());
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());

        let Some(c) = schema.get(CONST) else {
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
                    return Ok(Some(ctx.annotate(Some(CONST), Some(value.into()))));
                }
                return Ok(Some(ctx.error(
                    Some(CONST),
                    Some(Box::new(ConstInvalid {
                        expected: self.expected.clone(),
                        actual: Cow::Borrowed(value),
                        translate: self.translate.clone(),
                    })),
                )));
            }
        }
        if self.expected.as_ref() == value {
            Ok(Some(ctx.annotate(Some(CONST), Some(value.into()))))
        } else {
            Ok(Some(ctx.error(
                Some(CONST),
                Some(Box::new(ConstInvalid {
                    expected: self.expected.clone(),
                    actual: Cow::Borrowed(value),
                    translate: self.translate.clone(),
                })),
            )))
        }
    }
}

pub fn translate_const_invalid_en(
    f: &mut std::fmt::Formatter<'_>,
    error: &ConstInvalid<'_>,
) -> std::fmt::Result {
    write!(f, "expected {}, found {}", error.expected, error.actual)
}

/// [`ValidationError`](`crate::error::ValidationError`) for the `enum` keyword, produced by [`ConstKeyword`].
#[derive(Clone, Debug)]
pub struct ConstInvalid<'v> {
    pub expected: Arc<Value>,
    pub actual: Cow<'v, Value>,
    pub translate: TranslateConstInvalid,
}

impl<'v> std::fmt::Display for ConstInvalid<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected {}, found {}", self.actual, self.expected)
    }
}

impl<'v> OutputError<'v> for ConstInvalid<'v> {
    fn make_owned(self: Box<Self>) -> Box<dyn OutputError<'static>> {
        match self.actual {
            Cow::Borrowed(actual) => Box::new(ConstInvalid {
                translate: self.translate,
                expected: self.expected,
                actual: Cow::Owned(actual.clone()),
            }) as Box<dyn OutputError<'static>>,
            Cow::Owned(actual) => Box::new(ConstInvalid {
                actual: Cow::Owned(actual),
                expected: self.expected,
                translate: self.translate,
            }) as Box<dyn OutputError<'static>>,
        }
    }

    fn translate(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        translator: &grill_core::output::Translator,
    ) -> std::fmt::Result {
        if let Some(translate) = translator.get::<TranslateConstInvalid>() {
            translate.run(f, self)
        } else {
            self.translate.run(f, self)
        }
    }

    fn set_translate(&mut self, translator: &grill_core::output::Translator) {
        if let Some(translate) = translator.get::<TranslateConstInvalid>() {
            self.translate = translate.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use grill_core::{schema::Dialect, Interrogator, Structure};
    use serde_json::json;

    use crate::{
        draft_2020_12::json_schema_2020_12_uri,
        keyword::{id, schema},
        ID, SCHEMA,
    };

    use super::{Const, *};
    async fn create_interrogator(const_value: Value) -> Interrogator {
        let dialect = Dialect::build(json_schema_2020_12_uri().clone())
            .with_keyword(schema::Schema::new(SCHEMA, false))
            .with_keyword(id::Id::new(ID, false))
            .with_keyword(Const::new(None))
            .with_metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
            .finish()
            .unwrap();
        Interrogator::build()
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
            .finish()
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
            .any(|k| k == crate::CONST));
        let key = interrogator
            .compile("https://example.com/without_const")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();
        assert!(!schema.keywords.iter().any(|k| k.kind() == crate::CONST));
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
