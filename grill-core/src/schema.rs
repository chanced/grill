use std::sync::Arc;

use dashmap::DashSet;
use serde_json::Value;
#[derive(Clone, Debug)]
pub struct Schema {
    pub id: Option<Arc<String>>,
    pub dialect: Option<Arc<String>>,
    pub references: Arc<DashSet<String>>,
    source: Arc<Value>,
    // validators: Arc<DashMap<String, Validator>>,
}

impl Schema {
    pub fn new(source: Value) -> Self {
        Self {
            id: None,
            dialect: None,
            source: Arc::new(source),
            references: Arc::new(DashSet::new()),
        }
    }
    pub fn references(&self) -> Arc<DashSet<String>> {
        self.references.clone()
    }
    /// Adds a single reference to the schema
    pub fn add_reference(&self, ref_id: impl ToString) -> bool {
        self.references.insert(ref_id.to_string())
    }
    /// Adds a slice of references to the schema
    pub fn add_references(&self, refs: &[impl ToString]) {
        for ref_id in refs {
            self.add_reference(ref_id.to_string());
        }
    }
    pub fn id(&self) -> Option<Arc<String>> {
        self.id.clone()
    }
    /// sets the schema's `id`
    pub fn set_id(&mut self, id: Option<impl ToString>) {
        self.id = id.map(|id| Arc::new(id.to_string()));
    }
    pub fn dialect(&self) -> Option<Arc<String>> {
        self.dialect.clone()
    }

    /// sets schema's `dialect`
    pub fn set_dialect(&mut self, dialect: Option<impl ToString>) {
        self.dialect = dialect.map(|dialect| Arc::new(dialect.to_string()));
    }
    pub fn source(&self) -> Arc<Value> {
        self.source.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Schema;

    #[test]
    fn test_add_references() {
        let v = serde_json::json!({
            "id": "foo",
        });
        let s = Schema::new(v);
        s.add_reference("bar");
        assert_eq!(s.references.len(), 1);
        s.add_references(&["baz", "qux"]);
        assert_eq!(s.references.len(), 3);
    }
}
