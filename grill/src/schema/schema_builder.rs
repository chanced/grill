use std::sync::Arc;

use serde_json::Value;
use uniresid::{AbsoluteUri, Uri};

use crate::{
    applicator::{InitFn, SetupFn},
    Error, Interrogator, MetaSchema, Schema,
};
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
    /// Sets the default [`MetaSchema`] for the [`Schema`]. If the [`Schema`] has a
    /// `$schema` field defined, that will be used instead.
    pub fn default_meta_schema(mut self, meta_schema: MetaSchema) -> Self {
        self.meta_schema = Some(meta_schema);
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
        if let Some(meta_schema) = self.meta_schema {
            if schema.meta_schema().is_none() {
                schema.set_meta_schema(meta_schema);
            }
        }
        Ok(schema)
    }
}

fn assign_default_id(id: Uri) -> Box<InitFn> {
    // let id = Arc::new(id);
    Box::new(move |_, schema| {
        if schema.id().is_none() {
            schema.set_id(id.clone());
        }
        Ok(Some(Box::new(move |_, _| {
            Ok(Box::new(move |value, eval, next| next.call(value, eval)))
        })))
    })
}

fn assign_default_meta_schema(meta_schema: MetaSchema) -> Box<InitFn> {
    Box::new(move |_, schema| {
        if schema.id().is_none() {
            schema.set_meta_schema(meta_schema.clone());
        }
        Ok(Some(Box::new(move |i: Interrogator, s: Schema| {
            Ok(Box::new(move |value, eval, next| next.call(value, eval)))
        })))
    })
}
