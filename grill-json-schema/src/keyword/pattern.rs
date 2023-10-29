use std::{borrow::Cow, sync::Arc};

use grill_core::{
    define_translate,
    error::Expected,
    keyword::{Compile, Keyword, Kind},
    output::{Annotation, BoxedError, Error},
};
use regex::Regex;
use serde_json::Value;

use super::PATTERN;

/// [`Keyword`] implementation for `pattern`.
#[derive(Clone, Default, Debug)]
pub struct Pattern {
    value: Arc<Value>,
    regex: Option<regex::Regex>,
    pattern: Option<String>,
    translate: TranslatePatternInvalid,
}

impl Keyword for Pattern {
    fn kind(&self) -> Kind {
        Kind::Keyword(PATTERN)
    }

    fn compile<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        schema: grill_core::Schema<'i>,
    ) -> Result<bool, grill_core::error::CompileError> {
        let Some(value) = schema.get(PATTERN) else {
            return Ok(false);
        };
        let Value::String(regex) = value else {
            return Err(grill_core::error::InvalidTypeError {
                expected: Expected::String,
                actual: Box::new(value.clone()),
            }
            .into());
        };
        self.pattern = Some(regex.clone());
        let regex = Regex::new(regex)?;
        self.regex = Some(regex);
        self.value = Arc::new(value.clone());
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut grill_core::keyword::Context,
        value: &'v serde_json::Value,
    ) -> Result<Option<grill_core::output::Output<'v>>, grill_core::error::EvaluateError> {
        let Value::String(haystack) = value else {
            return Ok(None);
        };
        let regex = self.regex.as_ref().unwrap();
        if regex.is_match(haystack) {
            Ok(Some(ctx.annotate(
                Some(PATTERN),
                Some(Annotation::Arc(self.value.clone())),
            )))
        } else {
            Ok(Some(ctx.error(
                Some(PATTERN),
                Some(Box::new(PatternInvalid {
                    actual: Cow::Borrowed(value),
                    pattern: self.pattern.clone().unwrap(),
                    translate: self.translate.clone(),
                })),
            )))
        }
    }
}

/// [`Error`] for the `pattern` keyword, produced by [`Pattern`].
#[derive(Debug, Clone)]
pub struct PatternInvalid<'v> {
    /// The expected values.
    pub pattern: String,
    /// The value received.
    pub actual: Cow<'v, Value>,
    /// The instance of [`TranslateEnumInvalid`] to use
    pub translate: TranslatePatternInvalid,
}

define_translate!(PatternInvalid, translate_pattern_invalid_en);

impl<'v> Error<'v> for PatternInvalid<'v> {
    fn into_owned(self: Box<Self>) -> BoxedError<'static> {
        Box::new(PatternInvalid {
            pattern: self.pattern,
            actual: Cow::Owned(self.actual.into_owned()),
            translate: self.translate,
        })
    }

    fn translate(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        translator: &grill_core::output::Translator,
    ) -> std::fmt::Result {
        if let Some(translate) = translator.get::<TranslatePatternInvalid>() {
            translate.run(f, self)
        } else {
            self.translate.run(f, self)
        }
    }

    fn set_translate(&mut self, translator: &grill_core::output::Translator) {
        if let Some(translate) = translator.get::<TranslatePatternInvalid>() {
            self.translate = translate.clone();
        }
    }
}

/// Default fn for [`TranslatePatternInvalid`].
pub fn translate_pattern_invalid_en(
    f: &mut std::fmt::Formatter<'_>,
    error: &PatternInvalid<'_>,
) -> std::fmt::Result {
    write!(
        f,
        "expected value matching pattern \"{}\", got {}",
        error.pattern, error.actual
    )
}
