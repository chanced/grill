mod functions;
use functions::Functions;

mod meta_schema;

pub use meta_schema::MetaSchema;
mod schema_builder;
use parking_lot::RwLock;
pub use schema_builder::SchemaBuilder;

mod sub_schema;
pub use parking_lot::Mutex;
pub use sub_schema::SubSchema;

use crate::{
    applicator::{Applicators, ExecutorFn, SetupFn},
    error::{MetaSchemaError, UnknownMetaSchema},
    Error, Evaluation, Interrogator, Next, OutputFmt,
};
use jsonptr::Pointer;
use serde_json::{Map, Value};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use uniresid::Uri;

/// Data structure representing a single [JSON Schema](https://json-schema.org/).
#[derive(Clone)]
pub struct Schema {
    id: Arc<RwLock<Option<Arc<Uri>>>>,
    meta_schema_id: Arc<RwLock<Option<Arc<Uri>>>>,
    references: Arc<RwLock<Arc<HashSet<Uri>>>>,
    source: Arc<RwLock<Arc<Value>>>,
    sub_schemas: Arc<RwLock<HashMap<String, SubSchema>>>,
    functions: Functions,
    applicators: Applicators,
}

impl Schema {
    /// Creates and returns a new `Schema`.
    pub fn new(source: Value, interrogator: &Interrogator) -> Result<Self, Error> {
        let schema = Schema {
            id: Arc::new(RwLock::new(None)),
            meta_schema_id: Arc::new(RwLock::new(None)),
            references: Arc::new(RwLock::new(Arc::new(HashSet::new()))),
            source: Arc::new(RwLock::new(Arc::new(source))),
            sub_schemas: Arc::new(RwLock::new(HashMap::new())),
            functions: Functions::new(),
            applicators: Applicators::new(),
        };
        schema.initialize(interrogator)?;
        Ok(schema)
    }
    /// Returns a [`SchemaBuilder`](crate::schema::SchemaBuilder) which can be used to construct a [`Schema`]
    pub fn builder(source: Value) -> SchemaBuilder {
        SchemaBuilder::new(source)
    }
    fn exec_fns(&self) -> Vec<Box<ExecutorFn>> {
        self.functions.executor_fns()
    }
    fn setup_fns(&self) -> Vec<Box<SetupFn>> {
        self.functions.setup_fns()
    }
    pub fn meta_schema_id(&self) -> Option<Arc<Uri>> {
        let id = self.meta_schema_id.read();
        id.clone()
    }

    /// Evaluates `value` against this `Schema`.
    pub fn evaluate(&self, value: &Value, output: OutputFmt) -> Result<Evaluation, Error> {
        let next = Next::new(self.exec_fns());
        let eval = Evaluation::new(Pointer::default(), Pointer::default(), output);
        next.call(value, eval)
    }

    /// Creates and returns a new [`SubSchema`] that is nested within this `Schema`.
    pub fn add_sub_schema(
        &self,
        key: &str,
        source: Value,
        interrogator: &Interrogator,
    ) -> Result<SubSchema, Error> {
        let meta_schema_id = self.meta_schema_id();
        let meta_schema = match meta_schema_id {
            Some(ref id) => interrogator.meta_schema(id),
            None => None,
        };
        let base_uri = interrogator.base_uri().as_deref().cloned();

        let ss = match source {
            Value::Array(arr) => {
                let mut subs = Vec::with_capacity(arr.len());
                for source in arr {
                    let b = SchemaBuilder {
                        id: None,
                        source,
                        meta_schema: meta_schema.clone(),
                        base_uri: base_uri.clone(),
                    };
                    subs.push(b.build(interrogator)?);
                }
                SubSchema::Array(Arc::new(subs))
            }
            _ => {
                let b = SchemaBuilder {
                    source,
                    base_uri,
                    meta_schema,
                    id: None,
                };
                SubSchema::Single(b.build(interrogator)?)
            }
        };
        let res = ss.clone();
        let mut sub_schemas = self.sub_schemas.write();
        sub_schemas.insert(key.to_string(), ss);
        Ok(res)
    }
    pub fn meta_schema(&self, interrogator: &Interrogator) -> Option<MetaSchema> {
        let id = self.meta_schema_id.read();
        match id.as_ref() {
            Some(id) => interrogator.meta_schema(id),
            None => None,
        }
    }

    /// Returns the [`SubSchema`](crate::schema::SubSchema) associated with the given `field`.
    pub fn sub_schema(&self, field: &str) -> Option<SubSchema> {
        let sub_schemas = self.sub_schemas.read();
        sub_schemas.get(field).cloned()
    }
    /// Returns a [`HashMap`] of associated [`SubSchema`](crate::schema::SubSchema).
    pub fn sub_schemas(&self) -> HashMap<String, SubSchema> {
        let guard = self.sub_schemas.read();
        guard.clone()
    }

    fn initialize(&self, interrogator: &Interrogator) -> Result<(), Error> {
        let meta_schema = self.load_meta_schema(interrogator);
        todo!()
    }

    fn load_meta_schema(&self, interrogator: &Interrogator) -> Result<MetaSchema, Error> {
        if let Some(meta) = self.meta_schema(&interrogator) {
            return Ok(meta);
        }
        let source = self.source();
        if let Some(obj) = source.as_object() {
            if let Some(uri) = obj.get("$schema") {
                if let Some(uri) = uri.as_str() {
                    return match Uri::parse(uri) {
                        Ok(uri) => interrogator
                            .meta_schema(&uri)
                            .ok_or(UnknownMetaSchema { uri }.into()),
                        Err(err) => Err(MetaSchemaError::InvalidUri(err).into()),
                    };
                }
            }
        }
        Ok(interrogator.default_meta_schema())
    }

    fn set_setup(&self, fns: Vec<Box<SetupFn>>) {
        self.functions.set_setup(fns);
    }
    fn set_executors(&self, fns: Vec<Box<ExecutorFn>>) {
        self.functions.set_executors(fns);
    }
    /// Prepares the schema for use calling `setup` of all [Applicators](crate::Applicator)
    /// attached to the [Interrogator]. Those which return an [ApplicatorFn] will be
    /// invoked upon calls to `evaluate`.
    ///
    pub(crate) fn setup(&self, interrogator: &Interrogator) -> Result<(), Error> {
        let setup_fns = self.setup_fns();
        let mut fns = Vec::with_capacity(setup_fns.len());
        for f in setup_fns.iter() {
            fns.push(f(interrogator.clone(), self.clone())?)
        }
        self.set_executors(fns);
        for (_, sub) in self.sub_schemas().iter() {
            sub.setup(interrogator)?;
        }
        Ok(())
    }

    pub fn source(&self) -> Arc<Value> {
        let source = self.source.read();
        source.clone()
    }

    /// Creates and returns an `Arc<str>` if the `source` [`Value`] is a
    /// [`String`](serde_json::Value). Returns `None` otherwise.
    pub fn as_str(&self) -> Option<Arc<str>> {
        let source = self.source();
        source.as_str().map(|s| Arc::from(s))
    }

    /// Creates and returns an `Arc<Vec<Value>>` if the `source` [`Value`] is an
    /// [`Array`](serde_json::Value). Returns `None` otherwise.
    pub fn as_array(&self) -> Option<Arc<Vec<Value>>> {
        self.source().as_array().cloned().map(Arc::new)
    }
    /// Returns the associated [`Map`](serde_json::Map) if the source
    /// [`Value`] is an `Object`. Returns `None` otherwise.
    pub fn as_object(&self) -> Option<Arc<Map<String, Value>>> {
        self.source().as_object().cloned().map(Arc::new)
    }
    /// Returns the associated `bool` if the source [`Value`] is a
    /// `Boolean`. Returns `None` otherwise
    pub fn as_bool(&self) -> Option<bool> {
        self.source().as_bool()
    }
    /// If the source [`Value`] is [`Null`](serde_json::Value::Null), returns `Some(())`. Returns
    /// `None` otherwise.
    pub fn as_null(&self) -> Option<()> {
        self.source().as_null()
    }
    /// If the source [`Value`] is a number, represent it as an `i64` if possible.
    /// Returns `None` otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        self.source().as_i64()
    }
    /// If the source [`Value`] is a number, represent it as an `f64` if possible.
    /// Returns `None` otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        self.source().as_f64()
    }
    /// If the source [`Value`] is a number, represent it as an `u64`
    /// if possible. Returns `None` otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        self.source().as_u64()
    }
    /// Returns `true` if the source [`Value`] is a `Number`. Returns
    /// `false` otherwise.
    pub fn is_number(&self) -> bool {
        self.source().is_number()
    }
    /// Returns `true` if the source [`Value`] is an `Object`. Returns
    /// `None` otherwise.
    pub fn is_object(&self) -> bool {
        self.source().is_object()
    }

    /// Returns `true` if the source [`Value`] is a `Boolean`. Returns
    /// `None` otherwise.
    pub fn is_boolean(&self) -> bool {
        self.source().is_boolean()
    }

    /// Returns `true` if the source [`Value`] is an integer between
    /// `i64::MIN` and `i64::MAX`.
    pub fn is_i64(&self) -> bool {
        self.source().is_i64()
    }
    /// Returns `true` if the source [`Value`] can be represented as
    /// an `f64`.
    pub fn is_f64(&self) -> bool {
        self.source().is_f64()
    }
    /// Returns `true` if the source [`Value`] can be represented as an `u64`.
    /// Returns `false` otherwise.
    pub fn is_u64(&self) -> bool {
        self.source().is_u64()
    }

    /// Returns `true` if the source [`Value`] is a `Null`. Returns
    /// `false` otherwise.
    pub fn is_null(&self) -> bool {
        self.source().is_null()
    }

    /// Returns the associated id if set. Otherwise returns `None`.
    pub fn id(&self) -> Option<Arc<Uri>> {
        let guard = self.id.read();
        guard.clone()
    }

    /// Sets the id of the schema, returning the previous value if it exists.
    pub fn set_id(&self, id: Uri) -> Option<Arc<Uri>> {
        let mut guard = self.id.write();
        let old = guard.clone();
        *guard = Some(Arc::new(id));
        old
    }

    /// Adds a reference to the schema. Returns `true` if the reference was not
    /// already present.
    pub fn add_reference(&self, reference: Uri) -> bool {
        let mut guard = self.references.write();
        let mut references = guard.as_ref().clone();
        if references.contains(&reference) {
            false
        } else {
            references.insert(reference);
            *guard = Arc::new(references);
            true
        }
    }

    /// Returns the associated `
    pub fn references(&self) -> Arc<HashSet<Uri>> {
        let references = self.references.read();
        references.clone()
    }

    /// sets schema's `dialect`, returning the previous value if it exists.
    pub fn set_meta_schema(&self, meta: MetaSchema) -> Option<Arc<Uri>> {
        let id = meta.id();
        let mut meta_schema_id = self.meta_schema_id.write();
        let old = meta_schema_id.clone();
        *meta_schema_id = id;
        old
    }

    pub(crate) fn update(&self, from: Schema) {
        let new_setup_fns = from.setup_fns();
        let new_exec_fns = from.exec_fns();
        let new_id = from.id();
        let new_meta_schema_id = from.meta_schema_id();
        let new_references = from.references();
        let new_source = from.source();
        let (new_current, new_pending) = from.applicators.clone_functions();

        let mut functions = self.functions.write();
        let mut id = self.id.write();
        let mut meta_schema_id = self.meta_schema_id.write();
        let mut references = self.references.write();
        let mut source = self.source.write();
        let mut applicators = self.applicators.lock();

        *id = new_id;
        *meta_schema_id = new_meta_schema_id;
        *references = new_references;
        *source = new_source;
        applicators.update(new_current, new_pending);
        functions.update(new_setup_fns, new_exec_fns);
    }
}

impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schema")
            .field("id", &self.id())
            .field("meta_schema_id", &self.meta_schema_id())
            .field("references", &self.references())
            .field("source", &self.source())
            .finish_non_exhaustive()
    }
}
impl std::cmp::Eq for Schema {}
impl std::cmp::PartialEq for Schema {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.source() == other.source()
    }
}
impl std::hash::Hash for Schema {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

#[cfg(test)]
mod tests {}
