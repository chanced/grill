use serde_json::Value;
use slotmap::new_key_type;

use crate::{keyword::Keyword, AbsoluteUri, Handler};

new_key_type! {
    pub struct SchemaKey;
}

#[derive(Clone, Debug)]
pub struct Schema {
    /// The URI of the schema.
    pub id: AbsoluteUri,
    /// The URI of the schema's `Metaschema`.
    pub meta_schema: AbsoluteUri,
    /// The Handlers associated with the schema.
    pub handlers: Box<[Handler]>,
}
impl PartialEq for Schema {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.meta_schema == other.meta_schema
    }
}
impl Eq for Schema {}
