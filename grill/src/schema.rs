mod schema_builder;
pub use schema_builder::SchemaBuilder;

mod sub_schema;
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

struct Inner {
    id: ArcSwapOption<Uri>,
    dialect: ArcSwapOption<Uri>,
    references: Arc<ArcSwap<HashSet<Uri>>>,
    source: Arc<Value>,
    setup_fns: Arc<ArcSwap<Vec<Box<SetupFn>>>>,
    executors: Arc<ArcSwap<Vec<Box<ExecutorFn>>>>,
    sub_schemas: Arc<ArcSwap<HashMap<String, SubSchema>>>,
}

#[derive(Clone)]
pub struct Schema {
    inner: Arc<Inner>,
}

impl Schema {
    pub fn new(source: Value, interrogator: &Interrogator) -> Result<Self, Error> {
        let inner = Arc::new(Inner {
            id: ArcSwapOption::default(),
            dialect: ArcSwapOption::default(),
            source: Arc::new(source),
            references: Arc::new(ArcSwap::from_pointee(HashSet::new())),
            setup_fns: Arc::new(ArcSwap::new(Arc::new(Vec::new()))),
            executors: Arc::new(ArcSwap::new(Arc::new(Vec::new()))),
            sub_schemas: Arc::new(ArcSwap::new(Arc::new(HashMap::new()))),
        });

        let schema = Self { inner };
        schema.initialize(interrogator)?;
        Ok(schema)
    }

    pub fn builder(source: Value) -> SchemaBuilder {
        SchemaBuilder {
            dialect: None,
            base_uri: None,
            source,
        }
    }

    fn executors(&self) -> Arc<Vec<Box<ExecutorFn>>> {
        self.inner.executors.load().clone()
    }
    fn setup_fns(&self) -> Arc<Vec<Box<SetupFn>>> {
        self.inner.setup_fns.load().clone()
    }
    /// Evaluates `value` against this `Schema`.
    pub fn evaluate(
        &self,
        value: &Value,
        evaluation: Evaluation,
        output: OutputFmt,
    ) -> Result<Evaluation, Error> {
        let next = Next::new(self.executors());
        let eval = Evaluation::new(Pointer::default(), Pointer::default(), output);
        next.call(value, evaluation)
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
    pub fn sub_schema(&self, field: &str) -> Option<SubSchema> {
        self.inner.sub_schemas.load().get(field).cloned()
    }

    pub fn sub_schemas(&self) -> Arc<HashMap<String, SubSchema>> {
        self.inner.sub_schemas.load().clone()
    }
    pub(crate) fn initialize(&self, interrogator: &Interrogator) -> Result<(), Error> {
        let applicators = interrogator.applicators();
        let mut fns = Vec::with_capacity(applicators.len());
        for app in applicators.iter() {
            if let Some(setup_fn) = app.init(interrogator.clone(), self.clone())? {
                fns.push(setup_fn)
            }
        }
        self.inner.setup_fns.store(Arc::new(fns));
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
        self.inner.executors.store(Arc::new(fns));
        for (_, sub) in self.sub_schemas().iter() {
            sub.setup(interrogator)?;
        }
        Ok(())
    }
    /// Returns the associated `&str` if the source [Value] is a
    /// `String`. Returns `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        self.inner.source.as_str()
    }
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        self.inner.source.as_array()
    }
    /// Returns the associated `serde_json::Map` if the source
    /// [Value] is an `Object`. Returns `None` otherwise.
    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        self.inner.source.as_object()
    }
    /// Returns the associated `bool` if the source [Value] is a
    /// `Boolean`. Returns `None` otherwise
    pub fn as_bool(&self) -> Option<bool> {
        self.inner.source.as_bool()
    }
    /// If the source [Value] is `Null`, returns `Some(())`. Returns
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

#[cfg(test)]
mod tests {}
