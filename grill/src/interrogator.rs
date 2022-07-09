use crate::{
    draft::HYPER_SCHEMA_2020_12_URI,
    error::{MetaSchemaError, UnidentifiedSchemaError, UnknownMetaSchema},
    Error, Graph, MetaSchema, Schema, Vocabulary,
};
use dashmap::DashMap;
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
    meta_schemas: Arc<RwLock<MetaSchemas>>,
    graph: Arc<RwLock<Graph>>,
    base_uri: Arc<RwLock<Option<Arc<AbsoluteUri>>>>,
    vocabularies: Arc<DashMap<String, Vocabulary>>,
    lock: Arc<Mutex<()>>,
    default_meta_schema_uri: Arc<RwLock<Uri>>,
}

impl Debug for Interrogator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schemas = self.schemas.read();
        f.debug_struct("Interrogator")
            .field("schemas", &schemas.current)
            .field("graph", &self.graph)
            .finish_non_exhaustive()
    }
}

impl Interrogator {
    /// Creates
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(RwLock::new(Schemas::new())),
            meta_schemas: Arc::new(RwLock::new(MetaSchemas::new())),
            graph: Arc::new(RwLock::new(Graph::new(&[]).unwrap())),
            base_uri: Arc::new(RwLock::new(None)),
            lock: Arc::new(Mutex::new(())),
            vocabularies: Arc::new(DashMap::new()),
            default_meta_schema_uri: Arc::new(RwLock::new(HYPER_SCHEMA_2020_12_URI.clone())),
        }
    }

    /// Returns the `Schema` with the given `id` if it exists.
    pub fn schema(&self, id: &Uri) -> Option<Schema> {
        let r = self.schemas.read();
        r.get(id)
    }

    pub fn meta_schema(&self, id: &Uri) -> Option<MetaSchema> {
        let r = self.meta_schemas.read();
        r.get(id)
    }

    /// Returns the [`AbsoluteUri`](uniresid::AbsoluteUri) which .
    pub fn base_uri(&self) -> Option<Arc<AbsoluteUri>> {
        self.base_uri.read().clone()
    }
    /// Sets the base URI for all relative URIs found within each
    /// [`Schema`](Schema) attached to this [`Interrogator`](Interrogator).
    pub fn set_base_uri(&self, uri: AbsoluteUri) -> Option<Arc<AbsoluteUri>> {
        // todo: reinitialize and setup all schemas
        let mut guard = self.base_uri.write();
        guard.replace(Arc::new(uri))
    }
    /// Sets the default meta schema to use when no meta schema is specified,
    /// returning the previous default.
    pub fn set_default_meta_schema(&self, uri: Uri) -> Result<Uri, UnknownMetaSchema> {
        if self.meta_schema(&uri).is_none() {
            return Err(UnknownMetaSchema { uri });
        }
        let mut guard = self.default_meta_schema_uri.write();
        let old = guard.clone();
        *guard = uri;
        Ok(old)
    }
    /// Returns the default meta schema to use when no meta schema is specified.
    ///
    /// If not previously set, Draft 2020-12 will be the default.
    pub fn default_meta_schema(&self) -> MetaSchema {
        let r = self.default_meta_schema_uri.read();

        self.meta_schema(&*r).unwrap()
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
                let new_graph = {
                    match Graph::new(&values) {
                        Ok(g) => g,
                        Err(err) => {
                            let mut schemas = self.schemas.write();
                            schemas.rollback();
                            return Err(err.into());
                        }
                    }
                };
                let mut setup = HashSet::new();
                for s in values.iter().cloned() {
                    if s == schema {
                        continue;
                    }
                    if !setup.contains(&schema)
                        && !setup.contains(&s)
                        && new_graph.is_referenced(&s, &schema)
                    {
                        if !setup.contains(&schema) {
                            if let Err(err) = schema.setup(self) {
                                let mut schemas = self.schemas.write();
                                schemas.rollback();
                                return Err(err);
                            }
                            setup.insert(schema.clone());
                        }
                        if !setup.contains(&s) {
                            if let Err(err) = s.setup(self) {
                                let mut schemas = self.schemas.write();
                                schemas.rollback();
                                return Err(err);
                            }
                            setup.insert(s.clone());
                        }
                        continue;
                    }
                    if !setup.contains(&schema)
                        && !setup.contains(&s)
                        && new_graph.is_referenced(&schema, &s)
                    {
                        if !setup.contains(&schema) {
                            if let Err(err) = schema.setup(self) {
                                let mut schemas = self.schemas.write();
                                schemas.rollback();
                                return Err(err);
                            }
                        }
                        if !setup.contains(&s) {
                            if let Err(err) = s.setup(self) {
                                let mut schemas = self.schemas.write();
                                schemas.rollback();
                                return Err(err);
                            }
                        }
                    }
                }
                let mut schemas = self.schemas.write();
                schemas.commit();
                let mut graph = self.graph.write();
                graph.rebuild(&values).expect("Rebuilding the graph failed which is a bug. Please report this to https://github.com/chanced/grill/issues");
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
        schemas.commit();
        if existing.is_empty() {
            Ok(None)
        } else {
            Ok(Some(existing))
        }
    }

    //     pub fn add_applicator(&self, applicator: impl Applicator + 'static) -> Result<(), Error> {
    //         #[allow(unused_variables)]
    //         let lock = self.lock.lock();
    //         self.applicators.push(applicator);
    //         let schemas = {
    //             let s = self.schemas.read();
    //             s.values()
    //         };
    //         let mut initialized = Vec::with_capacity(schemas.len());
    //         for s in schemas {
    //             match s.duplicate(self) {
    //                 Ok(ds) => {
    //                     initialized.push(ds);
    //                 }
    //                 Err(err) => {
    //                     self.applicators.rollback();
    //                     return Err(err);
    //                 }
    //             }
    //         }
    //         let mut setup = Schemas::new();
    //         for schema in initialized.drain(..) {
    //             if let Err(err) = schema.setup(self) {
    //                 self.applicators.rollback();
    //                 return Err(err);
    //             }
    //             if let Err(err) = setup.insert(schema) {
    //                 self.applicators.rollback();
    //                 return Err(err.into());
    //             }
    //         }
    //         self.applicators.commit();

    //         for schema in setup.values() {
    //             todo!()
    //         }

    //         Ok(())
    //     }
    // }
}

impl Default for Interrogator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Schemas {
    current: HashMap<Uri, Schema>,
    pending: HashMap<Uri, Schema>,
}

impl Schemas {
    fn new() -> Self {
        Self {
            current: HashMap::new(),
            pending: HashMap::new(),
        }
    }
    fn get(&self, id: &Uri) -> Option<Schema> {
        self.pending
            .get(id)
            .or_else(|| self.current.get(id))
            .cloned()
    }

    fn insert(&mut self, schema: Schema) -> Result<Option<Schema>, UnidentifiedSchemaError> {
        if let Some(id) = schema.id() {
            let prev = self.current.get(&id);
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
        for s in self.current.values() {
            set.insert(s.clone());
        }
        set.iter().cloned().collect()
    }
    fn commit(&mut self) -> Vec<Schema> {
        for (_, schema) in self.pending.drain() {
            let id = schema.id().expect("a top level schema was unidentified during finalization. This is a bug. Please report it to https://github.com/chanced/grill/issues.").as_ref().clone();

            // schema.commit();
            self.current.insert(id, schema);
        }
        self.current.values().cloned().collect()
    }

    fn rollback(&mut self) {
        for s in self.pending.values() {
            // TODO: LOOK INTO THIS
            // s.rollback()
        }
        self.pending.clear()
    }
}

struct MetaSchemas {
    current: HashMap<Uri, MetaSchema>,
    pending: HashMap<Uri, MetaSchema>,
}

impl MetaSchemas {
    fn new() -> Self {
        Self {
            current: HashMap::new(),
            pending: HashMap::new(),
        }
    }
    fn get(&self, id: &Uri) -> Option<MetaSchema> {
        self.pending
            .get(id)
            .or_else(|| self.current.get(id))
            .cloned()
    }

    fn insert(
        &mut self,
        meta_schema: MetaSchema,
    ) -> Result<Option<MetaSchema>, UnidentifiedSchemaError> {
        if let Some(id) = meta_schema.id() {
            let prev = self.current.get(&id);
            self.pending.insert(id.as_ref().clone(), meta_schema);
            Ok(prev.cloned())
        } else {
            Err(UnidentifiedSchemaError {
                schema: meta_schema.as_schema(),
            })
        }
    }
    fn values(&self) -> Vec<MetaSchema> {
        let mut set = HashSet::new();
        for s in self.pending.values() {
            set.insert(s.clone());
        }
        for s in self.current.values() {
            set.insert(s.clone());
        }
        set.iter().cloned().collect()
    }

    fn commit(&mut self) -> Vec<MetaSchema> {
        for (_, schema) in self.pending.drain() {
            let id = schema.id().expect("a top level schema was unidentified during finalization. This is a bug. Please report it to https://github.com/chanced/grill/issues.").as_ref().clone();

            // schema.commit();
            self.current.insert(id, schema);
        }
        self.current.values().cloned().collect()
    }

    fn rollback(&mut self) {
        for s in self.pending.values() {
            // TODO: LOOK INTO THIS
            // s.rollback()
        }
        self.pending.clear()
    }
}
// #[cfg(test)]
// mod test {
//     use crate::{applicator::SetupFn, Error, Interrogator, OutputFmt, Schema};
//     use serde_json::json;
//     use uniresid::Uri;

//     fn spike(_int: Interrogator, schema: Schema) -> Result<Option<Box<SetupFn>>, Error> {
//         let id_str = schema
//             .as_object()
//             .unwrap()
//             .get("id")
//             .unwrap()
//             .as_str()
//             .unwrap()
//             .to_string();
//         let id = Uri::parse(id_str).unwrap();
//         schema.set_id(id);
//         Ok(Some(Box::new(move |_int, _schema: Schema| {
//             Ok(Box::new(move |value, evaluation, next| {
//                 println!("inside execute");
//                 next.call(value, evaluation)
//             }))
//         })))
//     }

//     fn spike2(_int: Interrogator, _schema: Schema) -> Result<Option<Box<SetupFn>>, Error> {
//         Ok(Some(Box::new(move |_int, _schema: Schema| {
//             println!("inside setup 2");
//             Ok(Box::new(move |value, evaluation, next| {
//                 println!("inside execute 2");
//                 next.call(value, evaluation)
//             }))
//         })))
//     }

//     #[test]
//     fn it_works() {
//         let i = Interrogator::new();
//         println!("before add_applicator");
//         i.add_applicator(spike).unwrap();
//         println!("after add_applicator");
//         println!("---");

//         println!("creating s1");
//         let s1 = Schema::new(json!({"id":"/s1"}), &i).unwrap();
//         println!("---");
//         println!("adding s1");
//         i.insert_schema(s1.clone()).unwrap();
//         println!("---");

//         println!("evaluating with s1");
//         s1.evaluate(&json!({}), OutputFmt::Basic).unwrap();
//         println!("---");

//         println!("adding spike2");
//         i.add_applicator(spike2).unwrap();
//         println!("---");

//         println!("evaluating with s1");
//         s1.evaluate(&json!({}), OutputFmt::Basic).unwrap();
//         println!("---");

//         println!("creating s2");
//         let s2 = Schema::new(json!({"id":"/s2"}), &i).unwrap();
//         println!("---");

//         println!("adding s2");
//         i.insert_schema(s2.clone()).unwrap();
//         println!("---");

//         println!("evaluating with s2");
//         s2.evaluate(&json!({}), OutputFmt::Basic).unwrap();
//         println!("---");
//     }
// }
