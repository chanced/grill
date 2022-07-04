use crate::{Applicator, Error, Graph, Schema, UnidentifiedSchemaError};
use arc_swap::ArcSwapOption;
use parking_lot::{Mutex, RwLock};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
};
use uniresid::{AbsoluteUri, Uri};

/// Centeral hub to manage [`Schema`] and [`Applicator`] instances.
#[derive(Clone)]
pub struct Interrogator {
    schemas: Arc<RwLock<Schemas>>,
    graph: Arc<RwLock<Graph>>,
    applicators: Arc<RwLock<Arc<Vec<Box<dyn Applicator>>>>>,
    base_uri: Arc<ArcSwapOption<AbsoluteUri>>,
    lock: Arc<Mutex<()>>,
}

impl Debug for Interrogator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schemas = self.schemas.read();
        f.debug_struct("Interrogator")
            .field("schemas", &schemas.schemas)
            .field("graph", &self.graph)
            .finish_non_exhaustive()
    }
}

impl Interrogator {
    /// Creates
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(RwLock::new(Schemas::new())),
            graph: Arc::new(RwLock::new(Graph::new(&[]).unwrap())),
            applicators: Arc::new(RwLock::new(Arc::new(Vec::new()))),
            base_uri: Arc::new(ArcSwapOption::default()),
            lock: Arc::new(Mutex::new(())),
        }
    }

    pub(crate) fn applicators(&self) -> Arc<Vec<Box<dyn Applicator>>> {
        let r = self.applicators.read();
        r.clone()
    }

    /// Returns the `Schema` with the given `id` if it exists.
    pub fn schema(&self, id: &Uri) -> Option<Schema> {
        let r = self.schemas.read();
        r.get(id)
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
        // this mutex lock ensures that only one process can modify the schemas at a time.
        // this is necessary because the RwLock guarding schemas cannot be held for
        // the duration of the `insert_schema` call as it would cause a deadlock.
        #[allow(unused_variables)]
        let g = self.lock.lock();

        match {
            let schemas = self.schemas.write();
            match schemas.insert(schema.clone()) {
                Ok(old) => match schema.setup(self) {
                    Ok(_) => Ok(old),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e.into()),
            }
        } {
            Err(err) => {
                let mut schemas = self.schemas.write();
                schemas.clear_pending();
                Err(err)
            }
            Ok(old) => {
                let values = {
                    let s = self.schemas.read();
                    s.values()
                };
                // this is safe as all schemas should be identified
                let new_graph = Graph::new(&values).expect("Encountered unidentified schema. This is a bug. Please report it to https://github.com/chanced/grill/issues.");
                for s in values.iter().cloned() {
                    if s == schema {
                        continue;
                    }
                    if new_graph.is_referenced(&s, &schema) {
                        if let Err(err) = schema.setup(self) {
                            let mut schemas = self.schemas.write();
                            schemas.clear_pending();
                            return Err(err);
                        }
                        if let Err(err) = s.setup(self) {
                            let mut schemas = self.schemas.write();
                            schemas.clear_pending();
                            return Err(err);
                        }
                        continue;
                    }
                    if new_graph.is_referenced(&schema, &s) {
                        if let Err(err) = schema.setup(self) {
                            let mut schemas = self.schemas.write();
                            schemas.clear_pending();
                            return Err(err);
                        }
                        if let Err(err) = s.setup(self) {
                            let mut schemas = self.schemas.write();
                            schemas.clear_pending();
                            return Err(err);
                        }
                    }
                }
                let mut schemas = self.schemas.write();
                schemas.publish();
                let mut graph = self.graph.write();
                graph.rebuild(&values).expect("Rebuilding the graph failed which is a bug. Please report it to https://github.com/chanced/grill/issues");
                Ok(old)
            }
        }
    }
    /// Adds a slice of top-level [`Schema`] to the `Interrogator`, each associated
    /// by its `id`. [`Schema`] which already exist are overwritten and
    /// returned. `None` is returned otherwise.
    ///
    /// If an `id` of a [`Schema`] is not set, [`Error::UnidentifiedSchema`]
    /// is returned and none of the [`Schema`] are inserted.
    pub fn insert_schemas(&self, schemas_to_add: &[Schema]) -> Result<Option<Vec<Schema>>, Error> {
        todo!()
    }
}

impl Default for Interrogator {
    fn default() -> Self {
        Self::new()
    }
}

struct Schemas {
    schemas: HashMap<Uri, Schema>,
    pending: HashMap<Uri, Schema>,
}

impl Schemas {
    fn new() -> Self {
        Self {
            schemas: HashMap::new(),
            pending: HashMap::new(),
        }
    }
    fn get(&self, id: &Uri) -> Option<Schema> {
        self.schemas.get(id).or(self.pending.get(id)).cloned()
    }
    fn len(&self) -> usize {
        self.schemas.len()
    }

    fn insert(&self, schema: Schema) -> Result<Option<Schema>, UnidentifiedSchemaError> {
        if let Some(id) = schema.id() {
            let prev = self.schemas.get(&id);
            self.pending.insert(id.as_ref().clone(), schema);
            Ok(prev.cloned())
        } else {
            Err(UnidentifiedSchemaError { schema })
        }
    }
    fn values(&self) -> Vec<Schema> {
        let mut set = HashSet::new();
        for s in self.pending.values() {
            set.insert(s.clone());
        }
        for s in self.schemas.values() {
            set.insert(s.clone());
        }
        set.iter().cloned().collect()
    }
    fn publish(&mut self) -> Vec<Schema> {
        for (_, schema) in self.pending.drain() {
            let id = schema.id().expect("a top level schema was unidentified during finalization. This is a bug. Please report it to https://github.com/chanced/grill/issues.").as_ref().clone();
            self.schemas.insert(id, schema.clone());
        }
        self.schemas.values().cloned().collect()
    }

    fn clear_pending(&mut self) {
        self.pending.clear()
    }
}
