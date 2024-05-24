use std::sync::Arc;

use grill_uri::AbsoluteUri;
use jsonptr::Pointer;
use serde_json::Value;

pub trait EmbeddedSchemaPaths {
    fn embedded_schemas_paths(&self, base_path: &Pointer) -> Vec<Pointer>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dialect<W, E> {
    pub uri: AbsoluteUri,
    pub keywords: Vec<W>,
    pub sources: Vec<(AbsoluteUri, Arc<Value>)>,
    pub embedded_schema_paths: Vec<E>,
}
