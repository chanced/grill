use crate::{ApplicatorFn, Error, Evaluation, Interrogator, Next, Output};
use arc_swap::{ArcSwap, ArcSwapOption};
use jsonptr::Pointer;
use serde_json::{Map, Value};
use std::{borrow::Borrow, collections::HashSet, sync::Arc};
use uniresid::{AbsoluteUri, Uri};

struct Inner {
    id: ArcSwapOption<Uri>,
    dialect: ArcSwapOption<String>,
    references: Arc<ArcSwap<HashSet<Uri>>>,
    source: Arc<Value>,
    applicator_fns: Arc<ArcSwap<Vec<Box<ApplicatorFn>>>>,
}
#[derive(Clone)]
pub struct Schema {
    inner: Arc<Inner>,
}

pub struct Builder {
    source: Value,
    dialect: Option<AbsoluteUri>,
    base_uri: Option<AbsoluteUri>,
}

impl Builder {
    pub fn default_dialect(mut self, dialect: AbsoluteUri) -> Self {
        self.dialect = Some(dialect);
        self
    }
    pub fn default_base_uri(mut self, base_uri: AbsoluteUri) -> Self {
        self.base_uri = Some(base_uri);
        self
    }

    pub fn build(self, interrogator: &Interrogator) -> Result<Schema, Error> {
        let schema = Schema::new(self.source, interrogator)?;
        if let Some(base_uri) = self.base_uri {
            if let Some(id) = schema.id() {
                if id.scheme().is_none() {
                    let id = base_uri.resolve(id);
                    schema.set_id(id);
                }
            }
        }
        if let Some(dialect) = self.dialect {
            if schema.dialect().is_none() {
                schema.set_dialect(dialect);
            }
        }
        Ok(schema)
    }
}

impl Schema {
    pub fn new(source: Value, interrogator: &Interrogator) -> Result<Self, Error> {
        let inner = Arc::new(Inner {
            id: ArcSwapOption::default(),
            dialect: ArcSwapOption::default(),
            source: Arc::new(source),
            references: Arc::new(ArcSwap::from_pointee(HashSet::new())),
            applicator_fns: Arc::new(ArcSwap::new(Arc::new(Vec::new()))),
        });

        let schema = Self { inner };
        schema.initialize(interrogator)?;
        Ok(schema)
    }

    pub fn builder(source: Value) -> Builder {
        Builder {
            dialect: None,
            base_uri: None,
            source,
        }
    }

    pub fn evaluate(&self, value: Value, output: Output) -> Result<Evaluation, Error> {
        let next = Next::new(self.applicator_fns().clone());
        let eval = Evaluation::new(Pointer::default(), Pointer::default(), value, output);
        next.call(eval)
    }

    pub(crate) fn initialize(&self, interrogator: &Interrogator) -> Result<(), Error> {
        for initializer in interrogator.initializers().iter() {
            initializer.call(interrogator.clone(), self.clone())?;
        }
        let applicators = interrogator.applicators();
        let mut fns = Vec::with_capacity(applicators.len());

        for applicator in applicators.iter() {
            if let Some(f) = applicator.setup(interrogator.clone(), self.clone())? {
                fns.push(f);
            }
        }
        self.inner.applicator_fns.store(Arc::new(fns));
        Ok(())
    }

    /// Returns the associated `&str` if the source `serde_json::Value` is a
    /// `String`. Returns `None` otherwise.
    pub fn as_str(&self) -> Option<&str> {
        self.inner.source.as_str()
    }
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        self.inner.source.as_array()
    }
    /// Returns the associated `serde_json::Map` if the source
    /// `serde_json::Value` is an `Object`. Returns `None` otherwise.
    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        self.inner.source.as_object()
    }
    /// Returns the associated `bool` if the source `serde_json::Value` is a
    /// `Boolean`. Returns `None` otherwise
    pub fn as_bool(&self) -> Option<bool> {
        self.inner.source.as_bool()
    }
    /// If the source `serde_json::Value` is `Null`, returns `Some(())`. Returns
    /// `None` otherwise.
    pub fn as_null(&self) -> Option<()> {
        self.inner.source.as_null()
    }
    /// If the source `serde_json::Value` is a number, represent it as an `i64` if possible.
    /// Returns `None` otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        self.inner.source.as_i64()
    }
    /// If the source `serde_json::Value` is a number, represent it as an `f64` if possible.
    /// Returns `None` otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        self.inner.source.as_f64()
    }
    /// If the source `serde_json::Value` is a number, represent it as an `u64`
    /// if possible. Returns `None` otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        self.inner.source.as_u64()
    }
    /// Returns `true` if the source `serde_json::Value` is a `Number`. Returns
    /// `false` otherwise.
    pub fn is_number(&self) -> bool {
        self.inner.source.is_number()
    }
    /// Returns `true` if the source `serde_json::Value` is an `Object`. Returns
    /// `None` otherwise.
    pub fn is_object(&self) -> bool {
        self.inner.source.is_object()
    }

    /// Returns `true` if the source `serde_json::Value` is a `Boolean`. Returns
    /// `None` otherwise.
    pub fn is_boolean(&self) -> bool {
        self.inner.source.is_boolean()
    }

    /// Returns `true` if the source `serde_json::Value` is an integer between
    /// `i64::MIN` and `i64::MAX`.
    pub fn is_i64(&self) -> bool {
        self.inner.source.is_i64()
    }
    /// Returns `true` if the source `serde_json::Value` can be represented as
    /// an `f64`.
    pub fn is_f64(&self) -> bool {
        self.inner.source.is_f64()
    }
    /// Returns `true` if the source `serde_json::Value` can be represented as an `u64`.
    /// Returns `false` otherwise.
    pub fn is_u64(&self) -> bool {
        self.inner.source.is_u64()
    }

    /// Returns `true` if the source `serde_json::Value` is a `Null`. Returns
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
    pub fn dialect(&self) -> Option<String> {
        self.inner.dialect.load().as_ref().map(|s| s.to_string())
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
    pub fn set_dialect(&self, dialect: impl ToString) -> Option<String> {
        self.inner
            .dialect
            .swap(Some(Arc::new(dialect.to_string())))
            .map(|v| v.as_ref().clone())
    }

    pub(crate) fn applicator_fns(&self) -> Arc<Vec<Box<ApplicatorFn>>> {
        self.inner.applicator_fns.load().clone()
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
