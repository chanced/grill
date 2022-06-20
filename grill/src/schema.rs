use std::sync::Arc;

use dashmap::DashSet;
use serde_json::Value;
#[derive(Clone, Debug)]
pub struct Schema {
    pub id: Option<String>,
    pub dialect: Option<Arc<str>>,
    pub references: Arc<DashSet<String>>,
    pub source: Arc<Value>,
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
    /// Adds a single reference to the schema
    pub fn insert_reference(&self, ref_id: impl ToString) -> bool {
        self.references.insert(ref_id.to_string())
    }

    /// sets schema's `dialect`
    pub fn set_dialect(&mut self, dialect: &str) {
        self.dialect = dialect.map(|dialect| Arc::new(dialect.to_string()));
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
