mod schema_builder;
pub use schema_builder::SchemaBuilder;

mod sub_schema;
pub use parking_lot::Mutex;
pub use sub_schema::SubSchema;

use crate::{
    applicator::{ExecutorFn, SetupFn},
    Error, Evaluation, Interrogator, Next, OutputFmt,
};
use arc_swap::{ArcSwap, ArcSwapOption};
use jsonptr::Pointer;
use serde_json::{Map, Value};
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    sync::Arc,
};
use uniresid::Uri;

#[derive(Clone)]
struct Functions {
    setup_fns: Arc<ArcSwap<Vec<Box<SetupFn>>>>,
    executor_fns: Arc<ArcSwap<Vec<Box<ExecutorFn>>>>,
    pending_setup: Arc<Mutex<Arc<Vec<Box<SetupFn>>>>>,
    pending_executor: Arc<Mutex<Arc<Vec<Box<ExecutorFn>>>>>,
}

impl Functions {
    fn new() -> Self {
        Self {
            setup_fns: Arc::new(ArcSwap::from_pointee(Vec::new())),
            executor_fns: Arc::new(ArcSwap::from_pointee(Vec::new())),

            // using 2 phase load
            pending_setup: Arc::new(Mutex::new(Arc::new(Vec::new()))),
            pending_executor: Arc::new(Mutex::new(Arc::new(Vec::new()))),
        }
    }

    fn store_executors(&self, fns: Vec<Box<ExecutorFn>>) {
        let mut f = self.pending_executor.lock();
        *f = Arc::new(fns);
    }

    fn store_setup(&self, fns: Vec<Box<SetupFn>>) {
        let mut f = self.pending_setup.lock();
        *f = Arc::new(fns);
    }
    fn rollback(&self) {
        let mut s = self.pending_setup.lock();
        let mut e = self.pending_executor.lock();
        *s = Arc::new(Vec::new());
        *e = Arc::new(Vec::new());
    }
    fn finalize(&self) {
        let mut pending_executor = self.pending_executor.lock();
        let mut pending_setup = self.pending_setup.lock();
        self.executor_fns.store(pending_executor.clone());
        self.setup_fns.store(pending_setup.clone());
        *pending_executor = Arc::new(Vec::new());
        *pending_setup = Arc::new(Vec::new());
    }

    fn executor_fns(&self) -> Arc<Vec<Box<ExecutorFn>>> {
        self.executor_fns.load().clone()
    }
    fn setup_fns(&self) -> Arc<Vec<Box<SetupFn>>> {
        self.setup_fns.load().clone()
    }
    fn pending_setup_fns(&self) -> Arc<Vec<Box<SetupFn>>> {
        let lock = self.pending_setup.lock();
        lock.clone()
    }
}

struct Inner {
    id: ArcSwapOption<Uri>,
    dialect: ArcSwapOption<Uri>,
    references: Arc<ArcSwap<HashSet<Uri>>>,
    source: Arc<Value>,
    sub_schemas: Arc<ArcSwap<HashMap<String, SubSchema>>>,
    functions: Functions,
}

/// Data structure representing a single [JSON Schema](https://json-schema.org/).
#[derive(Clone)]
pub struct Schema {
    inner: Arc<Inner>,
}

impl Schema {
    /// Creates and returns a new `Schema`.
    pub fn new(source: Value, interrogator: &Interrogator) -> Result<Self, Error> {
        let inner = Arc::new(Inner {
            id: ArcSwapOption::default(),
            dialect: ArcSwapOption::default(),
            source: Arc::new(source),
            references: Arc::new(ArcSwap::from_pointee(HashSet::new())),
            sub_schemas: Arc::new(ArcSwap::new(Arc::new(HashMap::new()))),
            functions: Functions::new(),
        });

        let schema = Self { inner };
        schema.initialize(interrogator)?;
        Ok(schema)
    }
    /// Returns a [`SchemaBuilder`](crate::schema::SchemaBuilder) which can be used to construct a [`Schema`]
    pub fn builder(source: Value) -> SchemaBuilder {
        SchemaBuilder {
            dialect: None,
            base_uri: None,
            source,
        }
    }
    fn executors(&self) -> Arc<Vec<Box<ExecutorFn>>> {
        self.inner.functions.executor_fns()
    }
    fn setup_fns(&self) -> Arc<Vec<Box<SetupFn>>> {
        self.inner.functions.pending_setup_fns()
    }
    /// Evaluates `value` against this `Schema`.
    pub fn evaluate(&self, value: &Value, output: OutputFmt) -> Result<Evaluation, Error> {
        let next = Next::new(self.executors());
        let eval = Evaluation::new(Pointer::default(), Pointer::default(), output);
        next.call(value, eval)
    }

    /// Creates and returns a new [`Schema`] that is nested within this [`Schema`].
    ///
    /// Adding the [`Schema`] as a
    /// sub-schema ensures that the dialect is set accordingly and  all
    /// referenced [`Schema`](Schema)s are resolved in the proper order.
    pub fn new_sub_schema(
        &self,
        key: &str,
        source: &Value,
        interrogator: &Interrogator,
    ) -> Result<SubSchema, Error> {
        let mut sub_schemas = HashMap::new();
        Ok(match source {
            Value::Array(arr) => {
                let base_uri = interrogator.base_uri().as_deref().cloned();
                let dialect = self.dialect();
                let mut subs = Vec::with_capacity(arr.len());
                for v in arr {
                    let b = SchemaBuilder {
                        source: v.clone(),
                        dialect: dialect.clone(),
                        base_uri: base_uri.clone(),
                    };
                    subs.push(b.build(interrogator)?);
                }
                SubSchema::Array(Arc::new(subs))
            }
            _ => {
                let b = SchemaBuilder {
                    source: source.clone(),
                    dialect: self.dialect(),
                    base_uri: interrogator.base_uri().as_deref().cloned(),
                };
                let new = b.build(interrogator)?;
                sub_schemas.insert(key.to_string(), new.clone());
                SubSchema::Single(new)
            }
        })
    }

    /// Returns the [`SubSchema`](crate::schema::SubSchema) associated with the given `field`.
    pub fn sub_schema(&self, field: &str) -> Option<SubSchema> {
        self.inner.sub_schemas.load().get(field).cloned()
    }
    /// Returns a [`HashMap`] of associated [`SubSchema`](crate::schema::SubSchema).
    pub fn sub_schemas(&self) -> Arc<HashMap<String, SubSchema>> {
        self.inner.sub_schemas.load().clone()
    }

    fn initialize(&self, interrogator: &Interrogator) -> Result<(), Error> {
        let applicators = interrogator.applicators();
        let mut fns = Vec::with_capacity(applicators.len());
        for app in applicators.iter() {
            if let Some(setup_fn) = app.init(interrogator.clone(), self.clone())? {
                fns.push(setup_fn)
            }
        }
        self.inner.functions.store_setup(fns);
        Ok(())
    }

    /// Prepares the schema for use calling `setup` of all [Applicators](crate::Applicator)
    /// attached to the [Interrogator]. Those which return an [ApplicatorFn] will be
    /// invoked upon calls to `evaluate`.
    ///
    pub(crate) fn setup(&self, interrogator: &Interrogator) -> Result<(), Error> {
        let setup_fns = self.setup_fns();
        let mut fns = Vec::with_capacity(setup_fns.len());
        for f in setup_fns.iter() {
            if let Some(exec) = f(interrogator.clone(), self.clone())? {
                fns.push(exec)
            }
        }
        self.inner.functions.store_executors(fns);
        for (_, sub) in self.sub_schemas().iter() {
            sub.setup(interrogator)?;
        }
        Ok(())
    }

    pub(crate) fn publish(&self) {
        self.inner.functions.finalize();
    }

    /// Returns the associated `&str` if the source [Value] is a
    /// `String`. Returns `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        self.inner.source.as_str()
    }

    /// Returns the associated `` if the source
    /// [Value] is an `Object`. Returns `None` otherwise.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        self.inner.source.as_array()
    }
    /// Returns the associated [`Map`](serde_json::Map) if the source
    /// [Value] is an `Object`. Returns `None` otherwise.
    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        self.inner.source.as_object()
    }
    /// Returns the associated `bool` if the source [Value] is a
    /// `Boolean`. Returns `None` otherwise
    pub fn as_bool(&self) -> Option<bool> {
        self.inner.source.as_bool()
    }
    /// If the source [`Value`] is [`Null`](serde_json::Value::Null), returns `Some(())`. Returns
    /// `None` otherwise.
    pub fn as_null(&self) -> Option<()> {
        self.inner.source.as_null()
    }
    /// If the source [Value] is a number, represent it as an `i64` if possible.
    /// Returns `None` otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        self.inner.source.as_i64()
    }
    /// If the source [Value] is a number, represent it as an `f64` if possible.
    /// Returns `None` otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        self.inner.source.as_f64()
    }
    /// If the source [Value] is a number, represent it as an `u64`
    /// if possible. Returns `None` otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        self.inner.source.as_u64()
    }
    /// Returns `true` if the source [Value] is a `Number`. Returns
    /// `false` otherwise.
    pub fn is_number(&self) -> bool {
        self.inner.source.is_number()
    }
    /// Returns `true` if the source [Value] is an `Object`. Returns
    /// `None` otherwise.
    pub fn is_object(&self) -> bool {
        self.inner.source.is_object()
    }

    /// Returns `true` if the source [Value] is a `Boolean`. Returns
    /// `None` otherwise.
    pub fn is_boolean(&self) -> bool {
        self.inner.source.is_boolean()
    }

    /// Returns `true` if the source [Value] is an integer between
    /// `i64::MIN` and `i64::MAX`.
    pub fn is_i64(&self) -> bool {
        self.inner.source.is_i64()
    }
    /// Returns `true` if the source [Value] can be represented as
    /// an `f64`.
    pub fn is_f64(&self) -> bool {
        self.inner.source.is_f64()
    }
    /// Returns `true` if the source [Value] can be represented as an `u64`.
    /// Returns `false` otherwise.
    pub fn is_u64(&self) -> bool {
        self.inner.source.is_u64()
    }

    /// Returns `true` if the source [Value] is a `Null`. Returns
    /// `false` otherwise.
    pub fn is_null(&self) -> bool {
        self.inner.source.is_null()
    }

    /// Returns the associated id if set. Otherwise returns `None`.
    pub fn id(&self) -> Option<Arc<Uri>> {
        self.inner.id.load().as_ref().cloned()
    }

    /// Sets the id of the schema, returning the previous value if it exists.
    pub fn set_id(&self, val: impl Borrow<Uri>) -> Option<Uri> {
        self.inner
            .id
            .swap(Some(Arc::new(val.borrow().clone())))
            .map(|v| v.as_ref().clone())
    }

    /// Returns the associated dialect if set. Otherwise returns `None`.
    pub fn dialect(&self) -> Option<Uri> {
        self.inner.dialect.load().as_deref().cloned()
    }

    /// Adds a reference to the schema. Returns `true` if the reference was not
    /// already present.
    pub fn add_reference(&self, reference: Uri) -> bool {
        let g = self.inner.references.load();
        if g.contains(&reference) {
            return false;
        }
        let mut set = HashSet::new();
        for u in g.iter() {
            set.insert(u.clone());
        }
        self.inner.references.store(Arc::new(set));
        true
    }

    /// Returns the associated `
    pub fn references(&self) -> Arc<HashSet<Uri>> {
        self.inner.references.load().clone()
    }

    /// sets schema's `dialect`, returning the previous value if it exists.
    pub fn set_dialect(&self, dialect: Uri) -> Option<Uri> {
        self.inner
            .dialect
            .swap(Some(Arc::new(dialect)))
            .map(|v| v.as_ref().clone())
    }

    pub(crate) fn rollback(&self) {
        self.inner.functions.rollback();
    }
}

impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schema")
            .field("id", &self.inner.id)
            .field("dialect", &self.inner.dialect)
            .field("references", &self.inner.references)
            .field("source", &self.inner.source)
            .finish_non_exhaustive()
    }
}
impl std::cmp::Eq for Schema {}
impl std::cmp::PartialEq for Schema {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.inner.source == other.inner.source
    }
}
impl std::hash::Hash for Schema {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

#[cfg(test)]
mod tests {}
