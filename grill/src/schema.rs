use serde_json::Value;
use slotmap::new_key_type;

use crate::{AbsoluteUri, Handler, keyword::Keyword};

new_key_type! {
    pub struct SchemaKey;
}

#[derive(Clone)]
pub struct Schema {
    /// The URI of the schema.
    pub id: AbsoluteUri,
    /// The URI of the schema's `Metaschema`.
    pub meta_schema: AbsoluteUri,
    /// The Handlers associated with the schema.
    pub handlers: Box<[Handler]>,
}
