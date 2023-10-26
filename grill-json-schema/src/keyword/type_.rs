use std::{borrow::Cow, sync::Arc};

use bitflags::bitflags;
use grill_core::{
    define_translate,
    error::{CompileError, EvaluateError, Expected, InvalidTypeError, UnexpectedValueError},
    keyword::{self, Compile, Context, Keyword},
    output::{Annotation, Error, Output},
    Schema,
};
use once_cell::sync::Lazy;
use serde_json::{Number, Value};

use super::TYPE;

const EXPECTED: &str = r#"a string or an array of strings with any combination of the following values: ["null", "boolean", "object", "array", "number", "integer", "string"]"#;

bitflags! {
    /// The bitfield of types that are allowed
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Bitfield: u8 {
        /// `"null"`
        const NULL    = 0b00000001;
        /// `"boolean"`
        const BOOLEAN = 0b00000010;
        /// `"object`
        const OBJECT  = 0b00000100;
        /// `"array"`
        const ARRAY   = 0b00001000;
        /// `"number"`
        const NUMBER  = 0b00010000;
        /// `"integer"`
        const INTEGER = 0b00100000;
        /// `"string"`
        const STRING  = 0b01000000;
    }
}

fn null_str_value() -> &'static Value {
    static VALUE: Lazy<Value> = Lazy::new(|| Value::String("null".into()));
    &VALUE
}
fn boolean_str_value() -> &'static Value {
    static VALUE: Lazy<Value> = Lazy::new(|| Value::String("string".into()));
    &VALUE
}
fn object_str_value() -> &'static Value {
    static VALUE: Lazy<Value> = Lazy::new(|| Value::String("object".into()));
    &VALUE
}
fn array_str_value() -> &'static Value {
    static VALUE: Lazy<Value> = Lazy::new(|| Value::String("array".into()));
    &VALUE
}
fn number_str_value() -> &'static Value {
    static VALUE: Lazy<Value> = Lazy::new(|| Value::String("number".into()));
    &VALUE
}
fn integer_str_value() -> &'static Value {
    static VALUE: Lazy<Value> = Lazy::new(|| Value::String("integer".into()));
    &VALUE
}
fn string_str_value() -> &'static Value {
    static VALUE: Lazy<Value> = Lazy::new(|| Value::String("string".into()));
    &VALUE
}

/// [`Keyword`] implementation for `"type"`
#[derive(Clone, Debug)]
pub struct Type {
    /// The types that are allowed
    pub types: Types,
    /// The bitfield of the types that are allowed
    pub bitfield: Bitfield,
    /// The translation function for `TypeInvalid`
    pub translate: TranslateTypeInvalid,
}
impl Type {
    /// Construct a new `Type` keyword.
    #[must_use]
    pub fn new(translate: Option<TranslateTypeInvalid>) -> Self {
        Self {
            types: Types::String(""),
            bitfield: Bitfield::empty(),
            translate: translate.unwrap_or_default(),
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn matches<'v>(
        &self,
        ctx: &mut Context,
        bitfield: Bitfield,
        value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        if self.bitfield.contains(bitfield) {
            return Ok(Some(ctx.annotate(
                Some(TYPE),
                Some(Annotation::Ref(bitfield_to_value(bitfield))),
            )));
        }
        Ok(Some(ctx.error(
            Some(TYPE),
            Some(Box::new(TypeInvalid {
                value: Cow::Borrowed(value),
                value_type: bitfield_to_str(bitfield),
                expected: self.types.clone(),
                translate: self.translate.clone(),
            })),
        )))
    }
    fn matches_number<'v>(
        &self,
        ctx: &mut Context,
        number: &Number,
        value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let mut bitfield = Bitfield::NUMBER;
        if self.bitfield.contains(Bitfield::NUMBER) {
            return Ok(Some(ctx.annotate(Some(TYPE), Some(Annotation::Ref(value)))));
        }
        let number = ctx.number_ref(number)?;
        if self.bitfield.contains(Bitfield::INTEGER) {
            bitfield = Bitfield::NUMBER;
            if number.is_integer() {
                return Ok(Some(ctx.annotate(Some(TYPE), Some(Annotation::Ref(value)))));
            }
        }
        Ok(Some(ctx.error(
            Some(TYPE),
            Some(Box::new(TypeInvalid {
                value: Cow::Borrowed(value),
                value_type: bitfield_to_str(bitfield),
                expected: self.types.clone(),
                translate: self.translate.clone(),
            })),
        )))
    }
}

impl Keyword for Type {
    fn kind(&self) -> keyword::Kind {
        keyword::Kind::Single(TYPE)
    }

    fn setup<'i>(
        &mut self,
        _compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        let Some(type_) = schema.get(TYPE) else {
            return Ok(false);
        };

        let (types, bitfield) = Types::parse(type_)?;
        if types.is_empty() {
            return Ok(false);
        }
        self.types = types;
        self.bitfield = bitfield;
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v serde_json::Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        match value {
            Value::Null => self.matches(ctx, Bitfield::NULL, value),
            Value::Bool(_) => self.matches(ctx, Bitfield::BOOLEAN, value),
            Value::Number(n) => self.matches_number(ctx, n, value),
            Value::String(_) => self.matches(ctx, Bitfield::STRING, value),
            Value::Array(_) => self.matches(ctx, Bitfield::ARRAY, value),
            Value::Object(_) => self.matches(ctx, Bitfield::OBJECT, value),
        }
    }
}

/// [`Error`] for the [`Type`] (`"type"`) keyword
#[derive(Clone, Debug)]
pub struct TypeInvalid<'v> {
    /// The value of `"type"` that was invalid
    pub value: Cow<'v, Value>,
    /// The type of the value
    pub value_type: &'static str,
    /// The allowed types
    pub expected: Types,
    /// The [`TranslateTypeInvalid`] instance to use for this keyword.
    pub translate: TranslateTypeInvalid,
}

define_translate!(TypeInvalid, translate_type_invalid_en);

impl<'v> Error<'v> for TypeInvalid<'v> {
    fn into_owned(self: Box<Self>) -> Box<dyn Error<'static>> {
        Box::new(TypeInvalid {
            value: Cow::Owned(self.value.into_owned()),
            value_type: self.value_type,
            expected: self.expected,
            translate: self.translate,
        })
    }

    fn translate(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        translator: &grill_core::output::Translator,
    ) -> std::fmt::Result {
        if let Some(translate) = translator.get::<TranslateTypeInvalid>() {
            translate.run(f, self)
        } else {
            self.translate.run(f, self)
        }
    }

    fn set_translate(&mut self, translator: &grill_core::output::Translator) {
        if let Some(translate) = translator.get::<TranslateTypeInvalid>() {
            self.translate = translate.clone();
        }
    }
}

/// Default `TranslateTypeInvalid` implementation for English
pub fn translate_type_invalid_en(
    f: &mut ::std::fmt::Formatter,
    error: &TypeInvalid,
) -> std::fmt::Result {
    write!(f, "expected {}, found {}", error.expected, error.value_type)
}

/// An error occurred while parsing the value of `"type"`
#[derive(thiserror::Error, Debug)]
pub enum TypesError {
    #[error(transparent)]
    /// The value type of `"type"` was invalid
    InvalidTypeError(#[from] InvalidTypeError),
    /// The value of `"type"` was unexpected
    #[error(transparent)]
    UnexpectedValue(#[from] UnexpectedValueError),
}
impl From<TypesError> for CompileError {
    fn from(e: TypesError) -> Self {
        match e {
            TypesError::InvalidTypeError(err) => err.into(),
            TypesError::UnexpectedValue(err) => err.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The proccessed value of a `"type"` keyword
pub enum Types {
    /// A single type
    String(&'static str),
    /// An array of types
    Array(Arc<[&'static str]>),
}

impl std::fmt::Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::String(s) => write!(f, "{s}"),
            Types::Array(arr) => write!(f, "{}", arr.join(", ")),
        }
    }
}

impl Types {
    /// Parses and returns the [`Types`] and [`Bitfield`] of the `value` if valid.
    ///
    /// # Errors
    /// Returns [`InvalidTypeError`] if the `value` contains invalid types
    pub fn parse(value: &Value) -> Result<(Self, Bitfield), TypesError> {
        match value {
            Value::String(s) => Self::try_from_str(value, s),
            Value::Array(a) => Self::try_from_slice(value, a),
            _ => Err(InvalidTypeError {
                expected: Expected::AnyOf(&[Expected::String, Expected::Array]),
                actual: Box::new(value.clone()),
            }
            .into()),
        }
    }

    /// Returns `true` if `s` is present
    #[must_use]
    pub fn contains(&self, s: &str) -> bool {
        match self {
            Types::String(v) => *v == s,
            Types::Array(v) => v.contains(&s),
        }
    }

    fn parse_str(value: &Value, s: &str) -> Result<(&'static str, Bitfield), TypesError> {
        let b = determine_bitfield(s).map_err(|()| UnexpectedValueError {
            expected: EXPECTED,
            value: Box::new(value.clone()),
        })?;
        Ok((bitfield_to_str(b), b))
    }

    /// Returrns `true` if empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Types::String(s) => s.is_empty(),
            Types::Array(a) => a.is_empty(),
        }
    }
    fn try_from_str(value: &Value, s: &str) -> Result<(Self, Bitfield), TypesError> {
        let (value, bitfield) = Self::parse_str(value, s)?;
        Ok((Self::String(value), bitfield))
    }

    fn try_from_slice(value: &Value, array: &[Value]) -> Result<(Self, Bitfield), TypesError> {
        let mut types = Vec::with_capacity(array.len());
        let mut bitfield = Bitfield::empty();
        for entry in array {
            match entry {
                Value::String(s) => {
                    let (s, bf) = Self::parse_str(value, s)?;
                    if !types.contains(&s) {
                        bitfield |= bf;
                        types.push(s);
                    }
                }
                _ => {
                    return Err(InvalidTypeError {
                        expected: Expected::String,
                        actual: Box::new(entry.clone()),
                    }
                    .into())
                }
            }
        }
        Ok((Self::Array(Arc::from(types)), bitfield))
    }
}

fn bitfield_to_str(bf: Bitfield) -> &'static str {
    match bf {
        Bitfield::NULL => "null",
        Bitfield::BOOLEAN => "boolean",
        Bitfield::OBJECT => "object",
        Bitfield::ARRAY => "array",
        Bitfield::NUMBER => "number",
        Bitfield::INTEGER => "integer",
        Bitfield::STRING => "string",
        _ => unreachable!(),
    }
}
fn bitfield_to_value(bf: Bitfield) -> &'static Value {
    match bf {
        Bitfield::NULL => null_str_value(),
        Bitfield::BOOLEAN => boolean_str_value(),
        Bitfield::OBJECT => object_str_value(),
        Bitfield::ARRAY => array_str_value(),
        Bitfield::NUMBER => number_str_value(),
        Bitfield::INTEGER => integer_str_value(),
        Bitfield::STRING => string_str_value(),
        _ => unreachable!(),
    }
}

fn determine_bitfield(s: &str) -> Result<Bitfield, ()> {
    match s {
        "null" => Ok(Bitfield::NULL),
        "boolean" => Ok(Bitfield::BOOLEAN),
        "object" => Ok(Bitfield::OBJECT),
        "array" => Ok(Bitfield::ARRAY),
        "number" => Ok(Bitfield::NUMBER),
        "integer" => Ok(Bitfield::INTEGER),
        "string" => Ok(Bitfield::STRING),
        other => {
            let lower = other.to_lowercase();
            if other != lower {
                let result = determine_bitfield(&lower);
                if let Ok(bf) = result {
                    return Ok(bf);
                }
            }
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::JsonSchema as _;
    use grill_core::{Build, Interrogator, Structure};
    use serde_json::json;

    use super::*;

    fn build_interrogator(uri: &str, source: Value) -> Build {
        Interrogator::build()
            .json_schema_2020_12()
            .source_value(uri, Cow::Owned(source))
    }

    #[tokio::test]
    async fn test_setup() {
        let with_string_type = json!({
            "type": "string"
        });
        let with_array_type = json!({
            "type": ["string", "number"]
        });

        let without_type = json!({});

        let mut interrogator =
            build_interrogator("https://example.com/with_string_type", with_string_type)
                .source_owned_value("https://example.com/without_type", without_type)
                .source_owned_value("https://example.com/with_array_type", with_array_type)
                .finish()
                .await
                .unwrap();

        let with_type_key = interrogator
            .compile("https://example.com/with_string_type")
            .await
            .unwrap();

        let schema = interrogator.schema(with_type_key).unwrap();
        assert!(schema.keywords.iter().any(|kw| kw.kind() == TYPE));

        let without_type_key = interrogator
            .compile("https://example.com/without_type")
            .await
            .unwrap();

        let schema = interrogator.schema(without_type_key).unwrap();

        assert!(!schema.keywords.iter().any(|kw| kw.kind() == TYPE));
    }

    #[tokio::test]
    async fn test_evaluate() {
        let with_string_type = json!({
            "type": "string"
        });
        let with_array_of_types = json!({
            "type": ["string", "number"]
        });
        let with_integer_type = json!({
            "type": "integer"
        });
        let with_number_type = json!({
            "type": "number"
        });

        let without_type = json!({});

        let mut interrogator =
            build_interrogator("https://example.com/with_string_type", with_string_type)
                .source_owned_value("https://example.com/without_type", without_type)
                .source_owned_value(
                    "https://example.com/with_array_of_types",
                    with_array_of_types,
                )
                .source_owned_value("https://example.com/with_number_type", with_number_type)
                .source_owned_value("https://example.com/with_integer_type", with_integer_type)
                .finish()
                .await
                .unwrap();

        let key = interrogator
            .compile("https://example.com/with_string_type")
            .await
            .unwrap();

        let valid_value = json!("test");

        let output = interrogator
            .evaluate(key, Structure::Verbose, &valid_value)
            .unwrap();
        assert!(output.is_valid());

        let invalid_value = json!(["invalid"]);
        let output = interrogator
            .evaluate(key, Structure::Verbose, &invalid_value)
            .unwrap();
        assert!(output.is_invalid());
        let key = interrogator
            .compile("https://example.com/with_integer_type")
            .await
            .unwrap();

        let valid_value = json!(34);
        let output = interrogator
            .evaluate(key, Structure::Verbose, &valid_value)
            .unwrap();
        assert!(output.is_valid());

        let invalid_value = json!(34.34);
        let output = interrogator
            .evaluate(key, Structure::Verbose, &invalid_value)
            .unwrap();
        assert!(output.is_invalid());

        let key = interrogator
            .compile("https://example.com/with_array_of_types")
            .await
            .unwrap();

        let valid_value = json!(34);
        let output = interrogator
            .evaluate(key, Structure::Verbose, &valid_value)
            .unwrap();
        assert!(output.is_valid());

        let valid_value = json!("\"34\"");
        let output = interrogator
            .evaluate(key, Structure::Verbose, &valid_value)
            .unwrap();
        assert!(output.is_valid());

        let invalid_value = json!(true);
        let output = interrogator
            .evaluate(key, Structure::Verbose, &invalid_value)
            .unwrap();
        assert!(output.is_invalid());

        let invalid_value = json!({});
        let output = interrogator
            .evaluate(key, Structure::Verbose, &invalid_value)
            .unwrap();
        assert!(output.is_invalid());

        let invalid_value = json!([34]);
        let output = interrogator
            .evaluate(key, Structure::Verbose, &invalid_value)
            .unwrap();
        assert!(output.is_invalid());
    }
}
