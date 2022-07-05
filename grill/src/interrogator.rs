use crate::{Applicator, Error, Graph, Schema, UnidentifiedSchemaError};
use arc_swap::ArcSwapOption;
use parking_lot::{Mutex, RwLock};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
};
use uniresid::{AbsoluteUri, Uri};

type Applicators = Arc<RwLock<Arc<Vec<Arc<dyn Applicator>>>>>;

/// Centeral hub to manage [`Schema`] and [`Applicator`] instances.
#[derive(Clone)]
pub struct Interrogator {
    schemas: Arc<RwLock<Schemas>>,
    graph: Arc<RwLock<Graph>>,
    applicators: Applicators,
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

    pub(crate) fn applicators(&self) -> Arc<Vec<Arc<dyn Applicator>>> {
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
            let mut schemas = self.schemas.write();
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
                schemas.rollback();
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
                            schemas.rollback();
                            return Err(err);
                        }
                        if let Err(err) = s.setup(self) {
                            let mut schemas = self.schemas.write();
                            schemas.rollback();
                            return Err(err);
                        }
                        continue;
                    }
                    if new_graph.is_referenced(&schema, &s) {
                        if let Err(err) = schema.setup(self) {
                            let mut schemas = self.schemas.write();
                            schemas.rollback();
                            return Err(err);
                        }
                        if let Err(err) = s.setup(self) {
                            let mut schemas = self.schemas.write();
                            schemas.rollback();
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
        // this mutex lock ensures that only one process can modify the schemas at a time.
        // this is necessary because the RwLock guarding schemas cannot be held for
        // the duration of the `insert_schema` call as it would cause a deadlock.
        #[allow(unused_variables)]
        let g = self.lock.lock();

        let mut schemas = self.schemas.write();
        let mut existing = Vec::new();
        for s in schemas_to_add {
            match schemas.insert(s.clone()) {
                Ok(Some(old)) => existing.push(old),
                Err(e) => {
                    schemas.rollback();
                    return Err(e.into());
                }
                _ => {}
            }
        }

        let all_schemas = schemas.values();
        let temp_graph = Graph::new(&all_schemas).expect("Encountered unidentified schema. This is a bug. Please report it to https://github.com/chanced/grill/issues.");
        let mut schemas_to_update = HashSet::with_capacity(schemas_to_add.len());
        // releasing the lock
        drop(schemas);

        for s in schemas_to_add {
            schemas_to_update.insert(s.clone());
        }

        for s in all_schemas {
            for ns in schemas_to_add {
                if s == ns.clone() {
                    continue;
                }
                if temp_graph.is_referenced(ns, &s) {
                    schemas_to_update.insert(s.clone());
                }
            }
        }
        for s in schemas_to_update {
            if let Err(err) = s.setup(self) {
                let mut schemas = self.schemas.write();
                schemas.rollback();
                return Err(err);
            }
        }
        let mut schemas = self.schemas.write();
        schemas.publish();
        if existing.is_empty() {
            Ok(None)
        } else {
            Ok(Some(existing))
        }
    }

    pub fn add_applicator(&self, applicator: impl Applicator + 'static) -> Result<(), Error> {
        let mut applicators = self.applicators.write();
        let previous = applicators.clone();
        let mut temp = applicators.iter().cloned().collect::<Vec<_>>();
        temp.push(Arc::new(applicator));
        *applicators = Arc::new(temp);
        // dropping applicators so that each Schema can access the list of applicators.
        drop(applicators);
        let schemas = {
            let guard = self.schemas.read();
            guard.values()
        };
        let mut updated = Vec::with_capacity(schemas.len());
        for s in schemas {
            match s.duplicate(self) {
                Ok(s) => updated.push(s),
                Err(e) => {
                    let mut guard = self.schemas.write();
                    guard.rollback();
                    return Err(e);
                }
            }
        }

        for s in updated {
            if let Err(err) = s.setup(self) {
                let mut guard = self.schemas.write();
                guard.rollback();
                return Err(err);
            }
        }

        let mut schemas = self.schemas.write();
        println!("#4");
        schemas.publish();
        println!("#5");
        Ok(())
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
        self.pending
            .get(id)
            .or_else(|| self.schemas.get(id))
            .cloned()
    }

    fn insert(&mut self, schema: Schema) -> Result<Option<Schema>, UnidentifiedSchemaError> {
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
            schema.publish();
            self.schemas.insert(id, schema);
        }
        self.schemas.values().cloned().collect()
    }

    fn rollback(&mut self) {
        for s in self.pending.values() {
            s.rollback()
        }
        self.pending.clear()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        applicator::{ExecutorFn, SetupFn},
        Error, Evaluation, Interrogator, Next, Schema,
    };
    use serde_json::{json, Value};

    fn spike(interrogator: Interrogator, schema: Schema) -> Result<Option<Box<SetupFn>>, Error> {
        println!("inside init");
        Ok(Some(Box::new(|interrogator, schema| {
            println!("inside setup");
            Ok(Some(Box::new(|value, evaluation, next| {
                println!("inside execute");
                next.call(value, evaluation)
            })))
        })))
    }

    #[test]
    fn it_works() {
        let i = Interrogator::new();
        println!("before add_applicator");
        i.add_applicator(spike).unwrap();
        println!("after add_applicator");
        let s1 = Schema::new(json! {{}}, &i).unwrap();
        s1.set_id("http://example.com/1".try_into().unwrap());
        i.insert_schema(s1.clone()).unwrap();
        s1.evaluate(&json!({}), crate::OutputFmt::Basic).unwrap();

        panic!("...")
    }
}
