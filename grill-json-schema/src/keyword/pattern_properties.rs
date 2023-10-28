//! # `patternProperties` keyword.
//!
//! - [Learn JSON Schema - const](https://www.learnjsonschema.com/2020-12/applicator/patternproperties/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.3.2.2)
use std::borrow::Cow;

use super::PATTERN_PROPERTIES;

use grill_core::{
    define_translate,
    error::{CompileError, EvaluateError, Expected, InvalidTypeError},
    keyword::{self, paths_of_object, Compile, Context, Keyword, Unimplemented},
    output::{Error, Output},
    Key, Schema,
};
use jsonptr::Pointer;
use serde_json::Value;

define_translate!(
    PatternPropertiesInvalid,
    translate_pattern_properties_invalid_en
);

/// [`Keyword`] implementation for `"patternProperties"`.
///
/// This keyword currently uses the [`regex`](https://crates.io/crates/regex)
/// crate for pattern matching.
///
#[derive(Debug, Default, Clone)]
pub struct PatternProperties {
    patterns: Vec<(String, Pointer, Key)>,
    translate: TranslatePatternPropertiesInvalid,
    regexes: regex::RegexSet,
}

impl PatternProperties {
    /// Returns a new `PatternProperties` keyword handler.
    ///
    /// The `translate` parameter can be used to override the default
    /// translation of the error message.
    #[must_use]
    pub fn new(translate: Option<TranslatePatternPropertiesInvalid>) -> Self {
        Self {
            patterns: Vec::new(),
            translate: translate.unwrap_or_default(),
            regexes: regex::RegexSet::empty(),
        }
    }
}

impl Keyword for PatternProperties {
    fn kind(&self) -> keyword::Kind {
        keyword::Kind::Keyword(PATTERN_PROPERTIES)
    }
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(value) = schema.get(PATTERN_PROPERTIES) else {
            return Ok(false);
        };
        if !matches!(value, Value::Object(_)) {
            return Err(InvalidTypeError {
                expected: Expected::Object,
                actual: Box::new(value.clone()),
            }
            .into());
        };
        let paths = paths_of_object(PATTERN_PROPERTIES, &schema);

        let mut patterns = Vec::with_capacity(paths.len());

        for path in paths {
            let keyword = path.last().unwrap().decoded().to_string();
            let _ = regex::Regex::new(&keyword)?;
            patterns.push(keyword.clone());
            let key = compile.subschema(&path)?;
            self.patterns.push((keyword, path, key));
        }
        self.regexes = regex::RegexSet::new(patterns)?;
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v serde_json::Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let Some(obj) = value.as_object() else {
            return Ok(None);
        };

        let mut output = ctx.annotate(Some(PATTERN_PROPERTIES), None);
        let mut is_valid = true;
        let mut invalid = Vec::with_capacity(self.patterns.len());
        for (prop, value) in obj {
            for matched in self.regexes.matches(prop) {
                let (_, ptr, key) = &self.patterns[matched];
                output.push(ctx.evaluate(*key, Some(prop), ptr, value)?);
                is_valid &= output.is_annotation();
                if !is_valid && ctx.should_short_circuit() {
                    return Ok(Some(output));
                }
                if !output.is_annotation() {
                    invalid.push(Cow::Borrowed(prop.as_str()));
                }
            }
        }
        if !is_valid {
            output.set_error(Some(Box::new(PatternPropertiesInvalid {
                invalid,
                translate: self.translate.clone(),
            })));
        }
        Ok(Some(output))
    }

    fn subschemas(&self, schema: &serde_json::Value) -> Result<Vec<Pointer>, Unimplemented> {
        Ok(paths_of_object(PATTERN_PROPERTIES, schema))
    }
}

/// `"patternProperties"` failed to validate.
#[derive(Clone, Debug)]
pub struct PatternPropertiesInvalid<'v> {
    /// the properties which failed to validate
    pub invalid: Vec<Cow<'v, str>>,
    /// the translation to use for the error message
    pub translate: TranslatePatternPropertiesInvalid,
}

/// Default [`TranslatePatternPropertiesInvalid`] implementation.
pub fn translate_pattern_properties_invalid_en(
    f: &mut std::fmt::Formatter<'_>,
    invalid: &PatternPropertiesInvalid<'_>,
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

impl<'v> Error<'v> for PatternPropertiesInvalid<'v> {
    fn into_owned(self: Box<Self>) -> Box<dyn Error<'static>> {
        Box::new(PatternPropertiesInvalid {
            invalid: self
                .invalid
                .into_iter()
                .map(|s| Cow::Owned(s.to_string()))
                .collect(),
            translate: self.translate,
        })
    }

    fn translate(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        translator: &grill_core::output::Translator,
    ) -> std::fmt::Result {
        if let Some(translate) = translator.get::<TranslatePatternPropertiesInvalid>() {
            translate.run(f, self)
        } else {
            self.translate.run(f, self)
        }
    }

    fn set_translate(&mut self, translator: &grill_core::output::Translator) {
        if let Some(translate) = translator.get::<TranslatePatternPropertiesInvalid>() {
            self.translate = translate.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use serde_json::json;

    use crate::{
        draft_2020_12::json_schema_2020_12_uri,
        keyword::{id, schema, type_, ID, SCHEMA},
    };
    use grill_core::{schema::Dialect, Interrogator, Structure};

    use super::*;

    async fn create_interrogator(properties: Value) -> Interrogator {
        let dialect = Dialect::build(json_schema_2020_12_uri().clone())
            .add_keyword(schema::Schema::new(SCHEMA, false))
            .add_keyword(id::Id::new(ID, false))
            .add_keyword(type_::Type::new(None))
            .add_keyword(super::PatternProperties::default())
            .add_metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
            .finish()
            .unwrap();
        Interrogator::build()
            .dialect(dialect)
            .source_owned_value("https://example.com/with_pattern_properties", properties)
            .source_owned_value(
                "https://example.com/without_pattern_properties",
                json!({
                    "$id": "https://example.com/without_properties",
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                }),
            )
            .finish()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_setup() {
        let mut interrogator = create_interrogator(json!(
            {
                "type": "object",
                "patternProperties": {
                    "^S_": { "type": "string" },
                    "^I_": { "type": "integer" }
                }
            }
        ))
        .await;
        let key = interrogator
            .compile("https://example.com/with_pattern_properties")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();

        assert!(schema
            .keywords
            .iter()
            .any(|k| k.kind() == PATTERN_PROPERTIES));
    }

    #[tokio::test]
    async fn test_evaluate() {
        let mut interrogator = create_interrogator(json!(
            {
                "type": "object",
                "patternProperties": {
                    "^S_": { "type": "string" },
                    "^I_": { "type": "integer" }
                }
            }
        ))
        .await;
        let key = interrogator
            .compile("https://example.com/with_pattern_properties")
            .await
            .unwrap();

        for (data, expected_valid) in [
            (json!({ "S_25": "This is a string" }), true),
            (json!({ "I_0": 34 }), true),
            (json!({ "S_25": 34 }), false),
            (json!({ "I_0": "This is a string" }), false),
            (json!({ "S_25": "This is a string", "I_0": 34 }), true),
        ] {
            let output = interrogator
                .evaluate(key, Structure::Verbose, &data)
                .unwrap();
            assert_eq!(
                output.is_annotation(),
                expected_valid,
                "expected {data} to be {expected_valid}"
            );
        }
    }
}
