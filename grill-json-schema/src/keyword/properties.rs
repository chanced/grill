//! # `properties` keyword.
//!
//! - [Learn JSON Schema - `properties`](https://www.learnjsonschema.com/2020-12/applicator/properties/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.3.2.1)

use std::{borrow::Cow, collections::HashMap};

use super::PROPERTIES;

use grill_core::{
    define_translate,
    error::{CompileError, EvaluateError, Expected, InvalidTypeError},
    keyword::{self, paths_of_object, Compile, Context, Keyword, Unimplemented},
    output::{BoxedError, Error, Output},
    Key, Schema,
};
use jsonptr::Pointer;
use serde_json::Value;

define_translate!(PropertiesInvalid, translate_properties_invalid_en);

/// [`Keyword`] implementation for `"properties"`.
#[derive(Debug, Default, Clone)]
pub struct Properties {
    subschemas: HashMap<String, (Pointer, Key)>,
    translate: TranslatePropertiesInvalid,
}

impl Properties {
    /// Returns a new `Properties` [`Keyword`].
    #[must_use]
    pub fn new(translate: Option<TranslatePropertiesInvalid>) -> Self {
        Self {
            subschemas: HashMap::new(),
            translate: translate.unwrap_or_default(),
        }
    }
}

impl Keyword for Properties {
    fn kind(&self) -> keyword::Kind {
        keyword::Kind::Keyword(PROPERTIES)
    }

    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(value) = schema.get(PROPERTIES) else {
            return Ok(false);
        };
        if !matches!(value, Value::Object(_)) {
            return Err(InvalidTypeError {
                expected: Expected::Object,
                actual: Box::new(value.clone()),
            }
            .into());
        };
        for subschema in paths_of_object(PROPERTIES, &schema) {
            let keyword = subschema.last().unwrap().decoded().to_string();
            let key = compile.subschema(&subschema)?;
            self.subschemas.insert(keyword, (subschema, key));
        }
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
        let mut output = ctx.annotate(Some(PROPERTIES), None);
        let mut is_valid = true;
        let mut invalid = Vec::with_capacity(self.subschemas.len());
        for (prop, value) in obj {
            if let Some((ptr, key)) = self.subschemas.get(prop) {
                output.push(ctx.evaluate(*key, Some(prop), ptr, value)?);
                is_valid &= output.is_valid();
                if !is_valid && ctx.should_short_circuit() {
                    return Ok(Some(output));
                }
                invalid.push(Cow::Borrowed(prop.as_str()));
            }
        }
        if !is_valid {
            output.set_error(Some(Box::new(PropertiesInvalid {
                invalid,
                translate: self.translate.clone(),
            })));
        }
        Ok(Some(output))
    }

    fn subschemas(&self, schema: &serde_json::Value) -> Result<Vec<Pointer>, Unimplemented> {
        Ok(paths_of_object(PROPERTIES, schema))
    }
}

/// [`Error`] for `"properties"` [`Keyword`]
#[derive(Clone, Debug)]
pub struct PropertiesInvalid<'v> {
    /// the list of properties that failed to validate
    pub invalid: Vec<Cow<'v, str>>,
    /// the default [`TranslatePropertiesInvalid`] to use for translation
    pub translate: TranslatePropertiesInvalid,
}

/// default [`TranslatePropertiesInvalid`] instance
pub fn translate_properties_invalid_en(
    f: &mut std::fmt::Formatter<'_>,
    invalid: &PropertiesInvalid<'_>,
) -> std::fmt::Result {
    for (i, prop) in invalid.invalid.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "\"{prop}\"")?;
    }
    write!(f, " failed to validate")
}

impl<'v> Error<'v> for PropertiesInvalid<'v> {
    fn into_owned(self: Box<Self>) -> BoxedError<'static> {
        Box::new(PropertiesInvalid {
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
        if let Some(translate) = translator.get::<TranslatePropertiesInvalid>() {
            translate.run(f, self)
        } else {
            self.translate.run(f, self)
        }
    }

    fn set_translate(&mut self, translator: &grill_core::output::Translator) {
        if let Some(translate) = translator.get::<TranslatePropertiesInvalid>() {
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
        keyword::{const_, id, schema, ID, PROPERTIES, SCHEMA},
    };
    use grill_core::{schema::Dialect, Interrogator, Structure};

    use super::*;

    async fn create_interrogator(properties: Value) -> Interrogator {
        let dialect = Dialect::build(json_schema_2020_12_uri().clone())
            .add_keyword(schema::Schema::new(SCHEMA, false))
            .add_keyword(id::Id::new(ID, false))
            .add_keyword(const_::Const::new(None))
            .add_keyword(super::Properties::default())
            .add_metaschema(json_schema_2020_12_uri().clone(), Cow::Owned(json!({})))
            .finish()
            .unwrap();
        Interrogator::build()
            .dialect(dialect)
            .source_owned_value(
                "https://example.com/with_properties",
                json!({
                    "$id": "https://example.com/with_properties",
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "properties": properties
                }),
            )
            .source_owned_value(
                "https://example.com/without_properties",
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
        let mut interrogator = create_interrogator(json!({
            "foo": {
                "const": 34.34
            },
        }))
        .await;
        let key = interrogator
            .compile("https://example.com/with_properties")
            .await
            .unwrap();
        let schema = interrogator.schema(key).unwrap();

        assert!(schema.keywords.iter().any(|k| k.kind() == PROPERTIES));
    }

    #[tokio::test]
    async fn test_evaluate() {
        let mut interrogator = create_interrogator(json!({
            "foo": {
                "const": 34.34
            },
        }))
        .await;
        let key = interrogator
            .compile("https://example.com/with_properties")
            .await
            .unwrap();
        let invalid = json!({
            "foo": 32
        });
        let output = interrogator
            .evaluate(Structure::Verbose, key, &invalid)
            .unwrap();
        assert!(!output.is_annotation());
    }
}
