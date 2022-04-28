use crate::error::Error;
use once_cell::sync::OnceCell;
use serde::{de::Visitor, Deserialize, Serialize};
use std::{result::Result as StdResult, str::FromStr};

pub const SUPPORTED_DRAFTS: &[&str] = &[
    "https://json-schema.org/draft/2020-12/schema",
    "https://json-schema.org/draft/2019-09/schema",
    "https://json-schema.org/draft-07/schema",
    "https://json-schema.org/draft-04/schema",
];

/// The `$schema` keyword is used to declare which dialect of JSON Schema the
/// schema was written for. The value of the `$schema` keyword is also the
/// identifier for a schema that can be used to verify that the schema is valid
/// according to the dialect $schema identifies. A schema that describes another
/// schema is called a “meta-schema”.
///
/// `$schema` applies to the entire document and must be at the root level. It
/// does not apply to externally referenced (`$ref`, `$dynamicRef`) documents. Those
/// schemas need to declare their own $schema.
///
/// If `$schema` is not used, an implementation might allow you to specify a
/// value externally or it might make assumptions about which specification
/// version should be used to evaluate the schema. It’s recommended that all
/// JSON Schemas have a `$schema` keyword to communicate to readers and tooling
/// which specification version is intended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum Dialect {
    Draft202012,
    Draft201909,
    Draft07,
    Draft04,
}

impl<'de> Dialect {
    pub fn name(&self) -> &str {
        match self {
            Dialect::Draft202012 => "2020-12",
            Dialect::Draft201909 => "2019-09",
            Dialect::Draft07 => "07",
            Dialect::Draft04 => "04",
        }
    }
}
impl ToString for Dialect {
    fn to_string(&self) -> String {
        match self {
            Dialect::Draft202012 => "https://json-schema.org/draft/2020-12/schema",
            Dialect::Draft201909 => "https://json-schema.org/draft/2019-09/schema",
            Dialect::Draft07 => "https://json-schema.org/draft-07/schema",
            Dialect::Draft04 => "https://json-schema.org/draft-04/schema",
        }
        .to_string()
    }
}
impl<'de> Deserialize<'de> for Dialect {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Dialect::from_str(&s).map_err(|_| serde::de::Error::unknown_variant(&s, SUPPORTED_DRAFTS))
    }
}
impl Dialect {
    pub fn supported_drafts() -> &'static [&'static str] {
        SUPPORTED_DRAFTS
    }
}

impl FromStr for Dialect {
    type Err = Error;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s.trim().trim_end_matches(|c| c == '#' || c == '/') {
            "https://json-schema.org/draft/2020-12/schema" => Ok(Self::Draft202012),
            "https://json-schema.org/draft/2019-09/schema" => Ok(Self::Draft201909),
            "https://json-schema.org/draft-07/schema" => Ok(Self::Draft07),
            "https://json-schema.org/draft-04/schema" => Ok(Self::Draft04),
            _ => Err(Error::UnsupportedSchema {
                schema: s.to_string(),
            }),
        }
    }
}

struct DialectVisitor {}
impl<'de> Visitor<'de> for DialectVisitor {
    type Value = Dialect;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON Schema dialect identifier")
    }
    fn visit_str<E>(self, v: &str) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Dialect::from_str(v).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}
