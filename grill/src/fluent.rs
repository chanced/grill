use serde_json::Map;
// use serde_json::Value;
// pub fn object() -> Object {
//     Object(Map::new())
// }
// pub struct Object(pub Map<String, Value>);

// impl Object {
//     pub fn any_of<T: Into<Schema>>(mut self, schema: T) -> Object {
//         let mut m = self.0;
//         let s: Schema = schema.into();
//         let v: Value = s.into();
//         m.insert("anyOf".to_string(), v);
//         self
//     }
// }
