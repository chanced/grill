use arc_swap::{ArcSwap, ArcSwapOption};
use dashmap::DashSet;
use serde_json::Value;

use std::sync::Arc;

use crate::{ApplicatorFn, Error, Interrogator};

struct Inner {
    id: ArcSwapOption<String>,
    dialect: ArcSwapOption<String>,
    references: Arc<DashSet<String>>,
    source: Arc<Value>,
    applicator_fns: ArcSwap<Vec<Box<ApplicatorFn>>>,
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
            references: Arc::new(DashSet::new()),
            applicator_fns: ArcSwap::new(Arc::new(Vec::new())),
        });
        let mut schema = Self { inner };
        schema.initialize(interrogator)?;
        Ok(schema)
    }

    fn initialize(&mut self, interrogator: &Interrogator) -> Result<(), Error> {
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

    pub fn id(&self) -> Option<String> {
        self.inner.id.load().as_ref().map(|v| v.to_string())
    }

    /// Sets the id of the schema, returning the previous value if it exists.
    pub fn set_id(&self, val: String) -> Option<String> {
        self.inner
            .id
            .swap(Some(Arc::new(val)))
            .map(|v| v.as_ref().clone())
    }
    /// Adds a single reference to the schema. Returns `true` if the reference
    /// was not already present.
    pub fn add_reference(&self, ref_id: impl ToString) -> bool {
        self.inner.references.insert(ref_id.to_string())
    }
    pub fn references(&self) -> Vec<String> {
        self.inner
            .references
            .iter()
            .map(|v| v.to_string())
            .collect()
    }
    pub fn dialect(&self) -> Option<String> {
        self.inner.dialect.load().as_ref().map(|v| v.to_string())
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
