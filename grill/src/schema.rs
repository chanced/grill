use slotmap::new_key_type;

use crate::{AbsoluteUri, Handler};

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
