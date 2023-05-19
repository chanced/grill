use serde_json::Value;

/// A source document containing one or more JSON Schemas.
pub struct Source {
    /// The URI of the source document.
    pub id: String,
    /// The JSON Schema contained in the source document.
    pub value: Value,
}

impl From<(String, Value)> for Source {
    fn from(value: (String, Value)) -> Self {
        let (id, value) = value;
        Self { id, value }
    }
}
