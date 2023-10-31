//! # `additioanalProperties` keyword.
//!
//! - [Learn JSON Schema : `additionalProperties`](https://www.learnjsonschema.com/2020-12/applicator/additionalproperties/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core#section-10.3.2.3)

use std::borrow::Cow;

use super::ADDITIONAL_PROPERTIES;
use grill_core::{
    define_translate,
    error::{CompileError, EvaluateError},
    keyword::{Compile, Context, Keyword, Kind},
    output::{Error, Output, Translator},
    static_pointer_fn, Key,
};
use serde_json::Value;

/// [`Keyword`] implementation for `additionalProperties`.
#[derive(Clone, Debug, Default)]
pub struct AdditionalProperties {
    key: Key,
    translate: TranslateAdditionalPropertiesInvalid,
}

impl Keyword for AdditionalProperties {
    fn kind(&self) -> Kind {
        Kind::Keyword(ADDITIONAL_PROPERTIES)
    }

    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: grill_core::Schema<'i>,
    ) -> Result<bool, CompileError> {
        if schema.get(ADDITIONAL_PROPERTIES).is_none() {
            return Ok(false);
        };

        self.key = compile.subschema(additional_properties_pointer())?;
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let Some(obj) = value.as_object() else {
            return Ok(None);
        };

        let mut output = ctx.annotate(Some(ADDITIONAL_PROPERTIES), None);
        let mut invalid = Vec::new();

        for (prop, value) in obj {
            if !ctx.has_evaluated(prop) {
                let ptr = jsonptr::Pointer::new([ADDITIONAL_PROPERTIES, prop]);
                output.push(ctx.evaluate(self.key, Some(prop), &ptr, value)?);
                if output.is_invalid() {
                    invalid.push(Cow::Borrowed(prop.as_str()));
                    if ctx.should_short_circuit() {
                        break;
                    }
                }
            }
        }

        if output.is_invalid() {
            output.set_error(Some(Box::new(AdditionalPropertiesInvalid {
                invalid,
                translate: self.translate.clone(),
            })));
        }

        Ok(Some(output))
    }

    fn subschemas(
        &self,
        schema: &Value,
    ) -> Result<Vec<jsonptr::Pointer>, grill_core::keyword::Unimplemented> {
        let mut result = Vec::new();
        if schema.get(ADDITIONAL_PROPERTIES).is_some() {
            result.push(additional_properties_pointer().clone());
        }
        Ok(result)
    }
}

/// [`Error`] for `"additionalProperties"` [`Keyword`]
#[derive(Debug, Clone)]
pub struct AdditionalPropertiesInvalid<'v> {
    /// List of invalid properties
    pub invalid: Vec<Cow<'v, str>>,
    /// [`TranslateAdditionalPropertiesInvalid`] instance
    pub translate: TranslateAdditionalPropertiesInvalid,
}

define_translate!(
    AdditionalPropertiesInvalid,
    translate_additional_properties_invalid_en
);

impl<'v> Error<'v> for AdditionalPropertiesInvalid<'v> {
    fn into_owned(self: Box<Self>) -> grill_core::output::BoxedError<'static> {
        Box::new(AdditionalPropertiesInvalid {
            invalid: self
                .invalid
                .into_iter()
                .map(|s| Cow::Owned(s.into_owned()))
                .collect(),
            translate: self.translate,
        })
    }

    fn translate(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        translator: &Translator,
    ) -> std::fmt::Result {
        if let Some(translate) = translator.get::<TranslateAdditionalPropertiesInvalid>() {
            translate.run(f, self)
        } else {
            self.translate.run(f, self)
        }
    }

    fn set_translate(&mut self, translator: &Translator) {
        if let Some(translate) = translator.get::<TranslateAdditionalPropertiesInvalid>() {
            self.translate = translate.clone();
        }
    }
}

/// default [`TranslatePropertiesInvalid`] instance
pub fn translate_additional_properties_invalid_en(
    f: &mut std::fmt::Formatter<'_>,
    invalid: &AdditionalPropertiesInvalid<'_>,
) -> std::fmt::Result {
    for (i, prop) in invalid.invalid.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "\"{prop}\"")?;
    }
    write!(f, " failed to validate")?;
    Ok(())
}
static_pointer_fn!(pub additional_properties "/additionalProperties");
