use crate::{Applicator, Error, Graph, Schema};
use arc_swap::ArcSwap;
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug, sync::Arc};
struct Layer<T: Clone + Send + Sync + 'static>(T);

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
}

impl Default for Interrogator {
    fn default() -> Self {
        Self::new()
    }
}
