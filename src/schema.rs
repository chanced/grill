mod draft_202012;
use crate::{Error, Result};

use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use serde_json as json;
use std::{collections::HashMap, result::Result as StdResult, str::FromStr, sync::Arc};

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

struct SchemaMapVisitor {
    dialect: Dialect,
}
impl<'de> Visitor<'de> for SchemaMapVisitor {
    type Value = HashMap<String, Schema>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map of JSON Schema")
    }
    fn visit_map<A>(self, mut access: A) -> StdResult<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map: HashMap<String, Schema> =
            HashMap::with_capacity(access.size_hint().unwrap_or(0));
        while let Some((key, value)) = access.next_entry()? {
            let value: json::Value = value;
            let schema = value
                .deserialize_map(SchemaVisitor {
                    dialect: self.dialect,
                })
                .map_err(serde::de::Error::custom)?;
            map.insert(key, schema);
        }
        Ok(map)
    }
}

impl Draft202012 {
    pub fn new(dialect: Dialect) -> Self {
        Self {
            dialect,
            id: None,
            typ: None,
            properties: None,
        }
    }
}

pub enum SchemaOrSchemas {
    Schema(Schema),
    Schemas(Vec<Schema>),
}

#[derive(Debug)]
pub enum Schema {
    Draft202012(Draft202012),
}

/// SchemaVisitor is the first step in the deserialization of JSON Schema. This
/// Visitor parses the various representations of a JSON Schema and and ensures
/// the proper Draft is selected based on
struct SchemaVisitor {
    dialect: Dialect,
}

impl<'de> Visitor<'de> for SchemaVisitor {
    type Value = Schema;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a json schema")
    }
    fn visit_bool<E>(self, v: bool) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        println!("visit_bool: {}", v);
        todo!()
    }

    fn visit_str<E>(self, v: &str) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        println!("visit_str: {}", v);
        todo!()
    }

    fn visit_map<A>(self, mut access: A) -> StdResult<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map = read_access_map(&mut access)?;
        let dialect = get_dialect(&map, self.dialect)?;
        if !map.contains_key("$schema") {
            map.insert("$schema".to_string(), dialect.to_string().into());
        }

        json::Value::Object(map)
            .deserialize_map(DraftVisitor { dialect })
            .map_err(serde::de::Error::custom)
    }
}

struct TypeVisitor {}

impl<'de> Visitor<'de> for TypeVisitor {
    type Value = TypeOrTypes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a json schema type (string) or array of types (string[])")
    }

    fn visit_str<E>(self, v: &str) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Type::from_str(v)
            .map(TypeOrTypes::Type)
            .map_err(|_| serde::de::Error::unknown_variant(v, Type::type_names()))
    }
}

// deserializes the json schema data prepared by SchemaVisitor
struct DraftVisitor {
    dialect: Dialect,
}
impl<'de> Visitor<'de> for DraftVisitor {
    type Value = Schema;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("a json schema, version {}", self.dialect.name()))
    }

    fn visit_map<A>(self, mut map: A) -> StdResult<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = read_access_map(&mut map)?;
        let dialect = get_dialect(&map, self.dialect)?;
        match dialect {
            Dialect::Draft202012 => parse_202012(&map, dialect).map(Schema::Draft202012),
            Dialect::Draft201909 => todo!(),
            Dialect::Draft07 => todo!(),
            Dialect::Draft04 => todo!(),
        }
    }
}
fn parse_202012<E: serde::de::Error>(
    map: &json::Map<String, json::Value>,
    dialect: Dialect,
) -> StdResult<Draft202012, E> {
    let mut schema = Draft202012::new(dialect);

    for (k, v) in map {
        match k.as_str() {
            "$id" => {
                schema.id = v.as_str().map(|s| s.to_string());
            }
            "type" => {
                schema.type_ = Some(
                    v.deserialize_any(TypeVisitor {})
                        .map_err(|e| serde::de::Error::custom(e))?,
                )
            }
            _ => {}
        }
    }
    todo!()
}

struct DialectVisitor {}
impl<'de> Visitor<'de> for DialectVisitor {
    type Value = Dialect;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a jsons schema draft uri")
    }
    fn visit_str<E>(self, s: &str) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de::Error;
        Dialect::from_str(s).map_err(|_| Error::unknown_variant(s, Dialect::supported_drafts()))
    }
}

fn get_dialect<E: serde::de::Error>(
    map: &json::Map<String, json::Value>,
    default: Dialect,
) -> StdResult<Dialect, E> {
    map.get("$schema")
        .map_or(Ok(default), |d| d.deserialize_str(DialectVisitor {}))
        .map_err(serde::de::Error::custom)
}

fn read_access_map<'de, A>(access: &mut A) -> StdResult<json::Map<String, json::Value>, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    let mut map = json::Map::with_capacity(access.size_hint().unwrap_or(0));

    while let Some((key, value)) = access.next_entry()? {
        map.insert(key, value);
    }
    Ok(map)
}

pub fn deserialize_str(s: &str, dialect: Dialect) -> Result<Schema> {
    let mut de = serde_json::Deserializer::from_str(s);
    de.deserialize_any(SchemaVisitor { dialect })
        .map_err(Error::from)
}

//
// ═══════════════════════════════════════════════
// ═══════════════════════════════════════════════
// ═══════════════════════════════════════════════
//
#[cfg(test)]
mod test {
    use crate::Schema;

    use super::{deserialize_str, Dialect, SchemaVisitor, DEFAULT_VERSION};
    fn set_default_version() {
        DEFAULT_VERSION.set(Dialect::Draft202012).unwrap();
    }
    #[test]
    fn testing_schema_deserialization() {
        set_default_version();
        let s = r###"
        {
            `$id`: "https://example.com/tree",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$dynamicAnchor": "node",
            "type": "object",
            "properties": {
              "data": true,
              "children": {
                "type": "array",
                "items": { "$dynamicRef": "#node" }
              }
            }
          }
        "###;
        let d = serde_json::Deserializer::from_str(s);
        let s = deserialize_str(s, Dialect::Draft07).unwrap();
        dbg!(s);
    }
}

// let schema: serde_json::Value = serde_json::from_str(val)?;
// let version = schema["$schema"].as_str().ok_or(Error::UnsupportedSchema {
//     schema: val.to_string(),
// })?;
// let version = Version::try_from(version)?;
// Ok(match version {
//     Version::V202012 => Schema::V202012(serde_json::from_str(val)?),
// })

// struct SchemaData {
//     data: json::Map<String, json::Value>,
//     dialect: Dialect,
// }
// impl<'de> DeserializeSeed<'de> for SchemaData {
//     type Value = Schema;

//     fn deserialize<D>(self, de: D) -> StdResult<Self::Value, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let dialect = self
//             .data
//             .get("$schema")
//             .map_or(Ok(self.dialect), |v| {
//                 v.deserialize_string(DialectVisitor {})
//             })
//             .map_err(|e| serde::de::Error::custom(format!("{}", e)))?;
//         dbg!(dialect);
//         todo!()
//     }
// }
