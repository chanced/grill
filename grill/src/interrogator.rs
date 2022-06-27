use crate::{Applicator, Error, Graph, Schema, UnidentifiedSchemaError};
use arc_swap::ArcSwap;
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug, sync::Arc};

pub struct Builder {}

/// Manages schemas and extensions.
#[derive(Clone)]
pub struct Interrogator {
    schemas: Arc<ArcSwap<HashMap<String, Schema>>>,
    graph: Arc<ArcSwap<Graph>>,
    applicators: Arc<ArcSwap<Vec<Box<dyn Applicator>>>>,
}

impl Debug for Interrogator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interrogator")
            .field("schemas", &self.schemas)
            .field("graph", &self.graph)
            .finish_non_exhaustive()
    }
}

// impl<R> Interrogator {}
impl Interrogator {
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(ArcSwap::from_pointee(HashMap::new())),
            graph: Arc::new(ArcSwap::from_pointee(Graph::new(&[]).unwrap())),
            applicators: Arc::new(ArcSwap::from_pointee(Vec::new())),
        }
    }
    pub fn builder() -> Builder {
        Builder {}
    }
    pub fn new_schema(&self, value: Value) -> Result<Schema, Error> {
        Schema::new(value, self)
    }

    pub(crate) fn applicators(&self) -> Arc<Vec<Box<dyn Applicator>>> {
        self.applicators.load().clone()
    }

    pub fn insert_schema(&self, schema: Schema) -> Result<Option<Schema>, Error> {
        if let Some(id) = schema.id() {
            let guard = self.schemas.load();
            let schemas = guard.clone();
            let old = guard.get(id.as_str());
            let mut data = HashMap::with_capacity(schemas.len());
            for (k, v) in schemas.iter() {
                data.insert(k.clone(), v.clone());
            }
            self.schemas.store(Arc::new(data));
            Ok(old.cloned())
        } else {
            Err(UnidentifiedSchemaError { schema }.into())
        }
    }
}

impl Default for Interrogator {
    fn default() -> Self {
        Self::new()
    }
}
