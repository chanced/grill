use arc_swap::ArcSwapOption;
use dashmap::DashSet;
use parking_lot::RwLock;
use serde_json::Value;

use std::sync::Arc;

use crate::Applicator;
pub struct Schema {
    id: ArcSwapOption<String>,
    dialect: ArcSwapOption<String>,
    references: Arc<DashSet<String>>,
    source: Arc<Value>,
    // this may need to be Arc<RwLock<Vec<Arc<dyn Applicator>>>>
    applicators: Arc<RwLock<Vec<Box<dyn Applicator>>>>,
    // list of fields in the schema which may or may not have applicators
    //
    // this is used to load applicators upon re-initialization or when a new
    // applicator is added
    //
    // todo: maybe removee this?
    keywords: Arc<DashSet<String>>,
}

impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schema")
            .field("id", &self.id)
            .field("dialect", &self.dialect)
            .field("references", &self.references)
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}

impl Schema {
    pub fn new(source: Value) -> Self {
        Self {
            id: ArcSwapOption::default(),
            dialect: ArcSwapOption::default(),
            source: Arc::new(source),
            references: Arc::new(DashSet::new()),
            applicators: Arc::new(RwLock::new(Vec::new())),
            keywords: Arc::new(DashSet::new()),
        }
    }
    pub fn id(&self) -> Option<String> {
        self.id.load().as_ref().map(|v| v.to_string())
    }
    /// Sets the id of the schema, returning the previous value if it exists.
    pub fn set_id(&self, val: String) -> Option<String> {
        self.id
            .swap(Some(Arc::new(val)))
            .map(|v| v.as_ref().clone())
    }
    /// Adds a single reference to the schema. Returns `true` if the reference
    /// was not already present.
    pub fn add_reference(&self, ref_id: impl ToString) -> bool {
        self.references.insert(ref_id.to_string())
    }
    pub fn references(&self) -> Vec<String> {
        self.references.iter().map(|v| v.to_string()).collect()
    }
    pub fn dialect(&self) -> Option<String> {
        self.dialect.load().as_ref().map(|v| v.to_string())
    }
    /// sets schema's `dialect`, returning the previous value if it exists.
    pub fn set_dialect(&self, dialect: impl ToString) -> Option<String> {
        self.dialect
            .swap(Some(Arc::new(dialect.to_string())))
            .map(|v| v.as_ref().clone())
    }

    pub(crate) fn push_applicator(&self, applicator: impl Applicator + 'static) {
        self.applicators.write().push(Box::new(applicator));
    }
}

#[cfg(test)]
mod tests {}
