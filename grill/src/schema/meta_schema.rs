use arc_swap::ArcSwap;
use serde_json::Value;
use uniresid::Uri;

use crate::{Dialect, Schema};
use std::sync::Arc;

#[derive(Clone)]
pub struct MetaSchema {
    schema: Schema,
    dialect: Arc<ArcSwap<Dialect>>,
}

impl MetaSchema {
    pub fn id(&self) -> Option<Arc<Uri>> {
        self.schema.id()
    }
    pub fn source(&self) -> Arc<Value> {
        self.schema.source()
    }
}

impl std::fmt::Debug for MetaSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schema")
            // .field("id", &self.inner.id)
            // .field("dialect", &self.inner.meta_schema)
            // .field("references", &self.inner.references)
            // .field("source", &self.inner.source)
            .finish_non_exhaustive()
    }
}
impl std::cmp::Eq for MetaSchema {}
impl std::cmp::PartialEq for MetaSchema {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.source() == other.source()
    }
}
impl std::hash::Hash for MetaSchema {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
