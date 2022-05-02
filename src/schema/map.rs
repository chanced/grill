use super::SchemaVisitor;
use crate::json;
use crate::{Dialect, Schema};
use serde::de::Visitor;
use serde::{Deserializer, Serializer};
use std::collections::HashMap;

struct SchemaMapVisitor {
    dialect: Dialect,
}
impl<'de> Visitor<'de> for SchemaMapVisitor {
    type Value = HashMap<String, Schema>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map of JSON Schema")
    }
    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
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
