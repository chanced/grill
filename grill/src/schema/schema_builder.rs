use serde_json::Value;
use uniresid::{AbsoluteUri, Uri};

use crate::{Error, Interrogator, Schema};
/// Used to construct a [`Schema`] with default values.
pub struct SchemaBuilder {
    pub(crate) source: Value,
    pub(crate) dialect: Option<Uri>,
    pub(crate) base_uri: Option<AbsoluteUri>,
}

impl SchemaBuilder {
    /// Sets the default `dialect` for a [`Schema`]. If the [`Schema`] has a
    /// `dialect` defined, that will be used instead.
    pub fn default_dialect(mut self, dialect: Uri) -> Self {
        self.dialect = Some(dialect);
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
        if let Some(dialect) = self.dialect {
            if schema.dialect().is_none() {
                schema.set_dialect(dialect);
            }
        }
        Ok(schema)
    }
}
