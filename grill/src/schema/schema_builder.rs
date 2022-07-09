use serde_json::Value;
use uniresid::{AbsoluteUri, Uri};

use crate::{Error, Interrogator, MetaSchema, Schema};
/// Used to construct a [`Schema`] with default values.
pub struct SchemaBuilder {
    pub(crate) source: Value,
    pub(crate) meta_schema: Option<MetaSchema>,
    pub(crate) base_uri: Option<AbsoluteUri>,
    pub(crate) id: Option<Uri>,
}

impl SchemaBuilder {
    pub fn new(source: Value) -> Self {
        Self {
            base_uri: None,
            source,
            id: None,
            meta_schema: None,
        }
    }
    #[must_use]
    pub fn default_meta_schema(mut self, meta_schema: MetaSchema) -> Self {
        self.meta_schema = Some(meta_schema);
        self
    }

    #[must_use]
    pub fn default_id(mut self, id: Uri) -> Self {
        self.id = Some(id);
        self
    }
    /// Sets the base URI of of the [`Schema`]. This enables for a [`Schema`]
    /// with a relative URI to be rewritten with an absolute URI.
    ///
    /// If the [`Schema`] has an absolute URI as an `id`, it will be used. If
    /// the `id` is relative, the id will be resolved from `base_uri`.
    ///
    /// If the [`Schema`] does not have an `id`, the `id` field will be left as
    /// `None`.
    #[must_use]
    pub fn default_base_uri(mut self, base_uri: AbsoluteUri) -> Self {
        self.base_uri = Some(base_uri);
        self
    }
    /// Builds the [`Schema`] with the provided defaults.
    pub fn build(self, interrogator: &Interrogator) -> Result<Schema, Error> {
        let schema = Schema::new(self.source, interrogator)?;
        if let Some(base_uri) = self.base_uri {
            if let Some(id) = schema.id() {
                if id.scheme().is_none() {
                    let id = base_uri.resolve(id);
                    schema.set_id(id.into());
                }
            }
        }
        if let Some(ref meta_schema) = self.meta_schema {
            if schema.meta_schema(interrogator).is_none() {
                schema.set_meta_schema(meta_schema);
            }
        }
        Ok(schema)
    }
}
