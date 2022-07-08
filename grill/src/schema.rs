mod functions;
use functions::Functions;

mod meta_schema;

pub use meta_schema::MetaSchema;
mod schema_builder;
pub use schema_builder::SchemaBuilder;

mod sub_schema;
pub use parking_lot::Mutex;
pub use sub_schema::SubSchema;

struct Inner {
    id: ArcSwapOption<Uri>,
    meta_schema: ArcSwapOption<MetaSchema>,
    references: Arc<ArcSwap<HashSet<Uri>>>,
    source: Arc<ArcSwap<Value>>,
    sub_schemas: Arc<ArcSwap<HashMap<String, SubSchema>>>,
    functions: Functions,
    applicators: Applicators,
}

impl Inner {
    pub(crate) fn new(source: Value) -> Arc<Self> {
        Arc::new(Inner {
            id: ArcSwapOption::default(),
            meta_schema: ArcSwapOption::default(),
            source: Arc::new(ArcSwap::from_pointee(source)),
            references: Arc::new(ArcSwap::from_pointee(HashSet::new())),
            sub_schemas: Arc::new(ArcSwap::new(Arc::new(HashMap::new()))),
            functions: Functions::new(),
            applicators: Applicators::new(),
        })
    }
}

use crate::{
    applicator::{Applicators, ExecutorFn, SetupFn},
    error::{MetaSchemaError, UnknownMetaSchemaError},
    Error, Evaluation, Interrogator, Next, OutputFmt,
};
use arc_swap::{ArcSwap, ArcSwapOption};
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
    inner: Arc<Inner>,
}

impl Schema {
    /// Creates and returns a new `Schema`.
    pub fn new(source: Value, interrogator: &Interrogator) -> Result<Self, Error> {
        let inner = Inner::new(source);
        let schema = Self { inner };
        schema.initialize(interrogator)?;
        Ok(schema)
    }
    /// Returns a [`SchemaBuilder`](crate::schema::SchemaBuilder) which can be used to construct a [`Schema`]
    pub fn builder(source: Value) -> SchemaBuilder {
        SchemaBuilder::new(source)
    }
    fn executors(&self) -> Arc<Vec<Box<ExecutorFn>>> {
        self.inner.functions.executor_fns()
    }
    fn setup_fns(&self) -> Arc<Vec<Box<SetupFn>>> {
        self.inner.functions.setup_fns()
    }
    pub fn meta_schema(&self) -> Option<MetaSchema> {
        let meta = self.inner.meta_schema.load();
        meta.as_deref().cloned()
    }

    /// Evaluates `value` against this `Schema`.
    pub fn evaluate(&self, value: &Value, output: OutputFmt) -> Result<Evaluation, Error> {
        let next = Next::new(self.executors());
        let eval = Evaluation::new(Pointer::default(), Pointer::default(), output);
        next.call(value, eval)
    }

    /// Creates and returns a new [`SubSchema`] that is nested within this `Schema`.
    pub fn new_sub_schema(
        &self,
        key: &str,
        source: Value,
        interrogator: &Interrogator,
    ) -> Result<SubSchema, Error> {
        let mut sub_schemas = HashMap::new();
        Ok(match source {
            Value::Array(arr) => {
                let base_uri = interrogator.base_uri().as_deref().cloned();
                let dialect = self.meta_schema();
                let mut subs = Vec::with_capacity(arr.len());
                for v in arr {
                    let b = SchemaBuilder {
                        id: None,
                        source: v.clone(),
                        meta_schema: self.meta_schema(),
                        base_uri: base_uri.clone(),
                    };
                    subs.push(b.build(interrogator)?);
                }
                SubSchema::Array(Arc::new(subs))
            }
            _ => {
                let b = SchemaBuilder {
                    source: source.clone(),
                    base_uri: interrogator.base_uri().as_deref().cloned(),
                    meta_schema: self.meta_schema(),
                    id: None,
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

    fn initialize(&self, interrogator: Interrogator) -> Result<(), Error> {
        todo!()
    }

    fn load_dialect(&self, interrogator: Interrogator) -> Result<MetaSchema, Error> {
        if let Some(meta) = self.meta_schema() {
            return Ok(meta);
        }
        let source = self.inner.source.load();
        if let Some(obj) = source.as_object() {
            if let Some(uri) = obj.get("$schema") {
                if let Some(uri) = uri.as_str() {
                    match Uri::parse(uri) {
                        Ok(uri) => interrogator
                            .meta_schema(&uri)
                            .ok_or(UnknownMetaSchemaError { meta_schema: uri }.into()),
                        Err(err) => Err(MetaSchemaError::InvalidUri(err).into()),
                    }
                } else {
                    interrogator.default_meta_schema()
                }
            }
        }
    }

    fn store_setup(&self, fns: Vec<Box<SetupFn>>) {
        self.inner.functions.store_setup(fns);
    }
    fn store_executors(&self, fns: Vec<Box<ExecutorFn>>) {
        self.inner.functions.store_executors(fns);
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
        self.inner.functions.store_executors(fns);
        for (_, sub) in self.sub_schemas().iter() {
            sub.setup(interrogator)?;
        }
        Ok(())
    }

    pub(crate) fn publish(&self, src: Schema) {
        self.inner.id.swap(src.id());
        self.inner.functions.publish_from(&src.inner.functions)
    }

    pub fn source(&self) -> Arc<Value> {
        let source = self.inner.source.load();
        source.clone()
    }

    /// Returns the associated `&str` if the source [Value] is a
    /// `String`. Returns `None` otherwise.
    pub fn as_str(&self) -> Option<String> {
        let source = self.inner.source.load();
        source.as_str().map(|s| s.to_string())
    }

    /// Returns the associated `` if the source
    /// [Value] is an `Object`. Returns `None` otherwise.
    pub fn as_array(&self) -> Option<Vec<Value>> {
        self.source().as_array().cloned()
    }
    /// Returns the associated [`Map`](serde_json::Map) if the source
    /// [Value] is an `Object`. Returns `None` otherwise.
    pub fn as_object(&self) -> Option<Map<String, Value>> {
        self.source().as_object().cloned()
    }
    /// Returns the associated `bool` if the source [Value] is a
    /// `Boolean`. Returns `None` otherwise
    pub fn as_bool(&self) -> Option<bool> {
        self.source().as_bool()
    }
    /// If the source [`Value`] is [`Null`](serde_json::Value::Null), returns `Some(())`. Returns
    /// `None` otherwise.
    pub fn as_null(&self) -> Option<()> {
        self.source().as_null()
    }
    /// If the source [Value] is a number, represent it as an `i64` if possible.
    /// Returns `None` otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        self.source().as_i64()
    }
    /// If the source [Value] is a number, represent it as an `f64` if possible.
    /// Returns `None` otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        self.source().as_f64()
    }
    /// If the source [Value] is a number, represent it as an `u64`
    /// if possible. Returns `None` otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        self.source().as_u64()
    }
    /// Returns `true` if the source [Value] is a `Number`. Returns
    /// `false` otherwise.
    pub fn is_number(&self) -> bool {
        self.source().is_number()
    }
    /// Returns `true` if the source [Value] is an `Object`. Returns
    /// `None` otherwise.
    pub fn is_object(&self) -> bool {
        self.source().is_object()
    }

    /// Returns `true` if the source [Value] is a `Boolean`. Returns
    /// `None` otherwise.
    pub fn is_boolean(&self) -> bool {
        self.source().is_boolean()
    }

    /// Returns `true` if the source [Value] is an integer between
    /// `i64::MIN` and `i64::MAX`.
    pub fn is_i64(&self) -> bool {
        self.source().is_i64()
    }
    /// Returns `true` if the source [Value] can be represented as
    /// an `f64`.
    pub fn is_f64(&self) -> bool {
        self.source().is_f64()
    }
    /// Returns `true` if the source [Value] can be represented as an `u64`.
    /// Returns `false` otherwise.
    pub fn is_u64(&self) -> bool {
        self.source().is_u64()
    }

    /// Returns `true` if the source [Value] is a `Null`. Returns
    /// `false` otherwise.
    pub fn is_null(&self) -> bool {
        self.source().is_null()
    }

    /// Returns the associated id if set. Otherwise returns `None`.
    pub fn id(&self) -> Option<Arc<Uri>> {
        self.inner.id.load().as_ref().cloned()
    }

    /// Sets the id of the schema, returning the previous value if it exists.
    pub fn set_id(&self, val: Uri) -> Option<Uri> {
        self.inner
            .id
            .swap(Some(Arc::new(val)))
            .map(|v| v.as_ref().clone())
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
    pub fn set_meta_schema(&self, meta: MetaSchema) -> Option<MetaSchema> {
        self.inner
            .meta_schema
            .swap(Some(Arc::new(meta)))
            .map(|v| v.as_ref().clone())
    }

    /// Clones the value and nothing else
    pub(crate) fn duplicate(&self, interrogator: &Interrogator) -> Result<Schema, Error> {
        let inner = Inner::new(self.source().as_ref().clone());
        let schema = Self { inner };
        schema.initialize(interrogator)?;
        Ok(schema)
    }

    pub(crate) fn update(&self, from: Schema) {
        self.inner.id.swap(from.id());
        self.inner.functions.update(from.inner.functions.clone());
        self.inner
            .references
            .swap(from.inner.references.load_full());
        self.inner
            .meta_schema
            .swap(from.inner.meta_schema.load_full());
        self.inner
            .sub_schemas
            .swap(from.inner.sub_schemas.load_full());
    }
}

impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schema")
            .field("id", &self.inner.id)
            .field("dialect", &self.inner.meta_schema)
            .field("references", &self.inner.references)
            .field("source", &self.inner.source)
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
