use serde::{Deserialize, Serialize};

use crate::Schema;

use super::SchemaRef;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Items {
    Schema(Box<Schema>),
    Array(Vec<Schema>),
}

#[derive(Clone, Debug)]
pub enum CompiledItems {
    Schema(SchemaRef),
    Array(Vec<SchemaRef>),
}
