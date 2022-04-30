use std::str::FromStr;

use serde::de::Visitor;
pub use serde::{Deserialize, Serialize};

pub const SIMPLE_TYPES: &[&str] = &["string", "boolean", "object", "array", "number", "null"];

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TypeOrTypes {
    Type(Type),
    Types(Vec<Type>),
}

#[derive(Clone, Debug, PartialEq, Eq, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum Type {
    String,
    Boolean,
    Object,
    Array,
    Number,
    Null,
}

impl Type {
    pub fn type_names() -> &'static [&'static str] {
        SIMPLE_TYPES
    }
}

pub(crate) struct TypeVisitor {}

impl<'de> Visitor<'de> for TypeVisitor {
    type Value = TypeOrTypes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a json schema type (string) or array of types (string[])")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Type::from_str(v)
            .map(TypeOrTypes::Type)
            .map_err(|_| serde::de::Error::unknown_variant(v, Type::type_names()))
    }
}
