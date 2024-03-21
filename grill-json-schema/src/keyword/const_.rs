//! # `const` keyword.
//!
//! - [Learn JSON Schema - `const`](https://www.learnjsonschema.com/2020-12/validation/const/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-validation#section-6.1.3)
use std::{borrow::Cow, sync::Arc};

use grill_core::{
    big::BigRational,
    error::CompileError,
    keyword::{define_translate, Keyword, Kind},
    output::{BoxedError, Error},
};

use serde_json::Value;

use grill_core::{
    error::EvaluateError,
    keyword::{Compile, Context},
    Output, Schema,
};

use crate::keyword::CONST;

/// [`Keyword`] handler for `"const"`.
///
/// The value of this keyword MAY be of any type, including null.
///
/// An instance validates successfully against this keyword if its value is
/// equal to the value of the keyword.
#[derive(Clone, Debug)]
pub struct Const {
    /// The value of the keyword.
    pub expected: Arc<Value>,
    /// The `BigRational` representation of a `Number` value.
    pub expected_number: Option<Arc<BigRational>>,
    /// The [`TranslateConstInvalid`] instance to use for this keyword.
    pub translate: TranslateConstInvalid,
}

impl Const {
    /// Construct a new `Const` keyword.
    ///
    /// `translate` allows for overriding of the default [`TranslateConstInvalid`]
    /// instance.
    #[must_use]
    pub fn new(translate: Option<TranslateConstInvalid>) -> Const {
        Self {
            expected: Arc::new(Value::Null),
            expected_number: None,
            translate: translate.unwrap_or_default(),
        }
    }
}
impl Keyword for Const {
    fn kind(&self) -> Kind {
        CONST.into()
    }
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
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
                let actual_number = ctx.number_ref(n)?;
                if actual_number == expected_number {
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
/// The default [`TranslateConstInvalid`] instance.
///
/// ```plaintext
/// "expected {expected}, found {actual}"
/// ```
pub fn translate_const_invalid_en(
    f: &mut std::fmt::Formatter<'_>,
    error: &ConstInvalid<'_>,
) -> std::fmt::Result {
    write!(f, "expected {}, found {}", error.expected, error.actual)
}

/// [`Error`] for the `enum` keyword, produced by [`Const`].
#[derive(Clone, Debug)]
pub struct ConstInvalid<'v> {
    /// The expected value.
    pub expected: Arc<Value>,
    /// The actual value.
    pub actual: Cow<'v, Value>,
    /// The [`TranslateConstInvalid`] instance to use for this error.
    pub translate: TranslateConstInvalid,
}

define_translate!(ConstInvalid, translate_const_invalid_en);

impl<'v> std::fmt::Display for ConstInvalid<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected {}, found {}", self.actual, self.expected)
    }
}

impl<'v> Error<'v> for ConstInvalid<'v> {
    fn into_owned(self: Box<Self>) -> BoxedError<'static> {
        match self.actual {
            Cow::Borrowed(actual) => Box::new(ConstInvalid {
                translate: self.translate,
                expected: self.expected,
                actual: Cow::Owned(actual.clone()),
            }),
            Cow::Owned(actual) => Box::new(ConstInvalid {
                actual: Cow::Owned(actual),
                expected: self.expected,
                translate: self.translate,
            }),
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
    use grill_core::{schema::Dialect, Interrogator, Output};
    use serde_json::json;

    use super::{Const, *};
    use crate::{
        draft_2020_12::json_schema_2020_12_uri,
        keyword::{id, schema, ID, SCHEMA},
    };
    async fn create_interrogator(const_value: Value) -> Interrogator {
        let dialect = Dialect::build(json_schema_2020_12_uri().clone())
            .add_keyword(schema::Schema::new(SCHEMA, false))
            .add_keyword(id::Id::new(ID, false))
            .add_keyword(Const::new(None))
            .add_metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
            .finish()
            .unwrap();
        Interrogator::build()
            .dialect(dialect)
            .source_owned_value(
                "https://example.com/with_const",
                json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "const": const_value
                }),
            )
            .source_owned_value(
                "https://example.com/without_const",
                json!({ "$schema": "https://json-schema.org/draft/2020-12/schema" }),
            )
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
            .any(|k| k == CONST));
        let key = interrogator
            .compile("https://example.com/without_const")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();
        assert!(!schema.keywords.iter().any(|k| k.kind() == CONST));
    }

    #[tokio::test]
    async fn test_const_evaluate() {
        let mut interrogator = create_interrogator(json!(34.34)).await;
        let key = interrogator
            .compile("https://example.com/with_const")
            .await
            .unwrap();
        let value = json!(34.34);
        let output = interrogator.evaluate(Output::Verbose, key, &value).unwrap();
        assert!(output.is_annotation());
        let value = json!(34.3434);
        let output = interrogator.evaluate(Output::Verbose, key, &value).unwrap();
        assert!(!output.is_annotation());
    }
    #[test]
    fn test_const_obj() {}
}
