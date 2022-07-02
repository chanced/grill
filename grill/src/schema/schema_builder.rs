use serde_json::Value;
use uniresid::{AbsoluteUri, Uri};

use crate::{Error, Interrogator, Schema};

pub struct SchemaBuilder {
    pub(crate) source: Value,
    pub(crate) dialect: Option<Uri>,
    pub(crate) base_uri: Option<AbsoluteUri>,
}

impl SchemaBuilder {
    pub fn default_dialect(mut self, dialect: Uri) -> Self {
        self.dialect = Some(dialect);
        self
    }
    pub fn default_base_uri(mut self, base_uri: AbsoluteUri) -> Self {
        self.base_uri = Some(base_uri);
        self
    }

    pub fn build(self, interrogator: &Interrogator) -> Result<Schema, Error> {
        let schema = Schema::new(self.source, interrogator)?;
        if let Some(base_uri) = self.base_uri {
            if let Some(id) = schema.id() {
                if id.scheme().is_none() {
                    let id = base_uri.resolve(id);
                    schema.set_id(id);
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
