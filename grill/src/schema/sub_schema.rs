use serde_json::Value;

use crate::{Error, Schema, interrogator, Interrogator};

#[derive(Clone)]
pub enum SubSchema {
    Single(Schema),
    Array(Vec<Schema>),
}

impl SubSchema {
    pub fn new(parent: Schema, source: &Value, interrogator: &Interrogator) -> Result<SubSchema, Error> {
        parent.new_sub_schema(source, interrogator)
    }
}
