//! # `"enum"` keyword.
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-validation#section-6.1.2)
//! - [Draft 2019-09 Specification](https://json-schema.org/draft/2019-09/json-schema-validation#rfc.section.6.1.2)

use grill_core::{
    define_translate,
    error::InvalidTypeError,
    keyword::{Compile, Keyword, Kind},
    output::Error,
};
use serde_json::Value;
use std::{borrow::Cow, string::ToString, sync::Arc};

use super::ENUM;

/// [`Keyword`] for`enum`.
#[derive(Debug, Clone)]
pub struct Enum {
    /// The expected values
    pub expected: Arc<[Value]>,
    /// The instance of [`TranslateEnumInvalid`] to use
    pub translate: TranslateEnumInvalid,
}
impl Enum {
    /// Construct a new `Enum` keyword.
    ///
    /// `translate` allows for overriding of the default [`TranslateEnumInvalid`]
    /// instance.
    #[must_use]
    pub fn new(translate: Option<TranslateEnumInvalid>) -> Enum {
        Self {
            expected: Arc::new([]),
            translate: translate.unwrap_or_default(),
        }
    }
}
impl Keyword for Enum {
    fn kind(&self) -> Kind {
        Kind::Single(ENUM)
    }
    fn setup<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        schema: grill_core::Schema<'i>,
    ) -> Result<bool, grill_core::error::CompileError> {
        let Some(enum_) = schema.get(ENUM) else {
            return Ok(false);
        };
        let Value::Array(enum_) = enum_ else {
            return Err(InvalidTypeError {
                expected: grill_core::error::Expected::Array,
                actual: Box::new(enum_.clone()),
            }
            .into());
        };
        if enum_.is_empty() {
            return Ok(false);
        }
        self.expected = Arc::from(enum_.as_slice());
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut grill_core::keyword::Context,
        value: &'v Value,
    ) -> Result<Option<grill_core::output::Output<'v>>, grill_core::error::EvaluateError> {
        for expected in &*self.expected {
            if expected == value {
                return Ok(Some(ctx.annotate(Some(ENUM), Some(value.into()))));
            }
        }
        Ok(Some(ctx.error(
            Some(ENUM),
            Some(Box::new(EnumInvalid {
                actual: Cow::Borrowed(value),
                expected: self.expected.clone(),
                translate: self.translate.clone(),
            })),
        )))
    }
}

/// [`Error`] for the `enum` keyword, produced by [`EnumKeyword`].
#[derive(Debug, Clone)]
pub struct EnumInvalid<'v> {
    /// The expected values.
    pub expected: Arc<[Value]>,
    /// The value received.
    pub actual: Cow<'v, Value>,
    /// The instance of [`TranslateEnumInvalid`] to use
    pub translate: TranslateEnumInvalid,
}

define_translate!(EnumInvalid, translate_enum_invalid_en);

impl<'v> Error<'v> for EnumInvalid<'v> {
    fn into_owned(self: Box<Self>) -> Box<dyn Error<'static>> {
        Box::new(EnumInvalid {
            expected: self.expected,
            actual: Cow::Owned(self.actual.into_owned()),
            translate: self.translate,
        })
    }

    fn translate(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        translator: &grill_core::output::Translator,
    ) -> std::fmt::Result {
        if let Some(translate) = translator.get::<TranslateEnumInvalid>() {
            translate.run(f, self)
        } else {
            self.translate.run(f, self)
        }
    }

    fn set_translate(&mut self, translator: &grill_core::output::Translator) {
        if let Some(translate) = translator.get::<TranslateEnumInvalid>() {
            self.translate = translate.clone();
        }
    }
}

/// Default [`TranslateEnumInvalid`].
pub fn translate_enum_invalid_en(
    f: &mut ::std::fmt::Formatter,
    error: &EnumInvalid,
) -> std::fmt::Result {
    let expected = error
        .expected
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    write!(
        f,
        "expected one of {}, found {}",
        expected.join(", "),
        error.actual
    )
}
