use crate::{Applicator, Error, Graph, Schema, UnidentifiedSchemaError};
use arc_swap::{ArcSwap, ArcSwapOption};
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use uniresid::{AbsoluteUri, Uri};

pub struct Builder {}

/// Manages schemas and extensions.
#[derive(Clone)]
pub struct Interrogator {
    schemas: Arc<ArcSwap<HashMap<Uri, Schema>>>,
    graph: Arc<ArcSwap<Graph>>,
    applicators: Arc<ArcSwap<Vec<Box<dyn Applicator>>>>,
    base_uri: Arc<ArcSwapOption<AbsoluteUri>>,
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
            base_uri: Arc::new(ArcSwapOption::default()),
        }
    }
    pub fn builder() -> Builder {
        Builder {}
    }

    pub(crate) fn applicators(&self) -> Arc<Vec<Box<dyn Applicator>>> {
        self.applicators.load().clone()
    }

    /// Returns the `Schema` with the given `id` if it exists.
    pub fn schema(&self, id: &Uri) -> Option<Schema> {
        self.schemas.load().get(id).cloned()
    }
    /// Returns the [`AbsoluteUri`](uniresid::AbsoluteUri) which .
    pub fn base_uri(&self) -> Option<Arc<AbsoluteUri>> {
        self.base_uri.load().clone()
    }
    /// Sets the base URI for all relative URIs found within each
    /// [`Schema`](Schema) attached to this [`Interrogator`](Interrogator).
    pub fn set_base_uri(&self, uri: AbsoluteUri) -> Option<Arc<AbsoluteUri>> {
        // todo: reinitialize and setup all schemas
        self.base_uri.swap(Some(Arc::new(uri)))
    }

    /// Adds a top-level `Schema` to the `Interrogator`, associated by its `id`.
    /// If the `Schema` already exists, it is overwritten and returned. `None`
    /// is returned otherwise.
    ///
    /// If the `id` of the `Schema` is not set, an `Error::UnidentifiedSchema`
    /// is returned and the `Schema` is not inserted.
    pub fn insert_schema(&self, schema: Schema) -> Result<Option<Schema>, Error> {
        if let Some(id) = schema.id() {
            let guard = self.schemas.load();
            let schemas = guard.clone();
            let old = schemas.get(&id);
            let mut data = HashMap::with_capacity(schemas.len());
            for (k, v) in schemas.iter() {
                data.insert(k.clone(), v.clone());
            }
            data.insert(id.as_ref().clone(), schema);
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
